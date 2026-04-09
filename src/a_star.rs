use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

use crate::{Error, FatToken, IncrementalBias, Step, TokenTrait, list::List};

/// Controls whether the A* search performs fault-tolerant error recovery.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseMode {
    /// Full fault-tolerant mode: explores error-recovery branches, tracks
    /// role-agreement fingerprints, maintains a best-at-EOF fallback, and
    /// applies an incremental heuristic.  Use when the document may contain
    /// errors and a best-effort CST is required.
    FaultTolerant,
    /// Fast mode: no error-recovery branches, no prevInfo fingerprints, no
    /// heuristic precomputation, no best-at-EOF fallback.  Returns `None`
    /// from `consume` if the document contains any error.  Use when the
    /// document is known to be correct and maximum throughput is desired.
    Fast,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Fingerprint(pub u128);

fn descend(fp: Fingerprint, branch_id: u32) -> Fingerprint {
    const M: u128 = 0x9E3779B97F4A7C15_6A09E667F3BCC909;
    Fingerprint(fp.0.wrapping_mul(M) ^ (branch_id as u128).wrapping_add(0xD6E8FEB86659FD93))
}

pub trait ParserTrait: Debug + Sized + 'static {
    type Kind: 'static + TokenTrait;

    fn step(&self, el: &Element<Self>, state: &mut AStar<Self>);
    fn at(&self) -> usize;
    fn element_kind(&self) -> Self::Kind;

    /// Precomputed dist(q, a): minimum insertion cost to reach a point where
    /// terminal `terminal` can be matched, starting from this parser state
    /// (kind, state_number).  Default: 0 (always admissible).
    fn state_dist(&self, _terminal: &Self::Kind) -> isize {
        0
    }
}

enum IsExpectedElement {
    True,
    False,
    Unkown,
}

/// Default maximum number of heap pops before the search gives up and returns
/// the best partial result found so far.  Prevents pathological inputs from
/// blocking indefinitely.
pub const DEFAULT_MAX_ITERATIONS: usize = 1_000_000;

/// Maximum number of consecutive error operations (insertions + deletions)
/// before an element is dropped.  Prevents unbounded error cascades on
/// deeply broken input.
pub const DEFAULT_MAX_REPAIR_SPAN: u16 = 15;

/// Number of consecutive error operations that triggers sync-point scanning.
const SYNC_THRESHOLD: u16 = 5;

/// Maximum number of tokens to scan ahead when looking for a sync point.
const MAX_SYNC_SCAN: usize = 50;

pub struct AStar<'a, R: ParserTrait> {
    /// Maps each state key to the best (lowest) cost seen so far (queued or
    /// processed).  Used for lazy-deletion dedup: elements with worse (higher)
    /// costs are rejected early in `add_element`; stale heap entries are
    /// skipped in `consume`.
    done: HashMap<(Fingerprint, usize, usize), isize>,
    tokens: &'a [FatToken<R::Kind>],
    todo: BinaryHeap<Element<R>>,
    pub bias: IncrementalBias,
    /// Admissible heuristic: `heuristic[pos]` is a non-positive lower bound on
    /// the minimum additional cost from token position `pos` to end of input.
    /// Computed as suffix sum of potential agreement bonuses: each remaining
    /// token with `old_kind` could yield a `-strength` discount.  This is
    /// optimistic (assumes every token lands in its previous role) and
    /// therefore admissible.
    /// In `Fast` mode this is always empty; 0 is used directly.
    heuristic: Vec<isize>,
    /// Number of elements popped from the heap so far.
    iterations: usize,
    /// Maximum number of heap pops before giving up.  See `DEFAULT_MAX_ITERATIONS`.
    max_iterations: usize,
    /// Best element that reached end-of-input during search, tracked as
    /// `(cost, step_list, parent_depth)`.  Used as a fallback when the search
    /// times out before finding a fully-unwound parse (depth == 1).
    /// Never populated in `Fast` mode.
    best_at_eof: Option<(isize, List<Step<R::Kind>>, usize)>,
    /// Whether to perform fault-tolerant error recovery.
    mode: ParseMode,
    /// Maximum consecutive error operations before an element is dropped.
    max_repair_span: u16,
}

impl<'a, R: ParserTrait> AStar<'a, R> {
    fn new(tokens: &'a [FatToken<R::Kind>], bias: IncrementalBias, max_iterations: usize) -> Self {
        let mut heuristic = vec![0isize; tokens.len() + 1];
        for i in (0..tokens.len()).rev() {
            let offset = if tokens[i].kind.skips() {
                0
            } else {
                tokens[i].kind.max_error_value()
            };
            // Maybe off by one
            heuristic[i] = heuristic[i + 1] + offset;
        }
        Self {
            tokens,
            done: HashMap::new(),
            todo: BinaryHeap::new(),
            bias,
            heuristic,
            iterations: 0,
            max_iterations,
            best_at_eof: None,
            mode: ParseMode::FaultTolerant,
            max_repair_span: DEFAULT_MAX_REPAIR_SPAN,
        }
    }

    /// Creates an `AStar` configured for fast, non-fault-tolerant parsing.
    /// Skips heuristic precomputation, incremental bias, and best-at-EOF
    /// tracking.  Returns `None` from `consume` if the document contains any
    /// error.
    fn new_fast(tokens: &'a [FatToken<R::Kind>]) -> Self {
        Self {
            tokens,
            done: HashMap::new(),
            todo: BinaryHeap::new(),
            bias: IncrementalBias { strength: 0 },
            heuristic: Vec::new(),
            iterations: 0,
            max_iterations: usize::MAX,
            best_at_eof: None,
            mode: ParseMode::Fast,
            max_repair_span: u16::MAX,
        }
    }

    pub fn consume(&mut self, root: R) -> Option<List<Step<R::Kind>>> {
        let start_idx = self.get_actual_index(0);
        let h = if self.mode == ParseMode::Fast {
            0
        } else if let Some(heurisitic) = self.heuristic.get(start_idx) {
            *heurisitic
        } else {
            return Some(List::default());
        };
        self.todo.push(Element::new_at(root, h, start_idx));

        loop {
            let e = self.todo.pop()?;
            self.iterations += 1;

            if let Some((head, _)) = e.parent.head() {
                let state = (e.state.0, e.state.1, head.at());

                // Skip stale entries: a better-scoring element for this state
                // was enqueued after this one.
                match self.done.get(&state) {
                    Some(&best) if best < e.cost => continue,
                    _ => {}
                }

                if e.state.1 == self.tokens.len() {
                    let depth = e.parent.len();
                    if self.mode == ParseMode::FaultTolerant
                        && self
                            .best_at_eof
                            .as_ref()
                            .map_or(true, |&(c, _, _)| e.cost < c)
                    {
                        self.best_at_eof = Some((e.cost, e.list.clone(), depth));
                    }
                    if depth == 1 {
                        // In Fast mode, reject any parse that contains an error step.
                        if self.mode == ParseMode::Fast && e.has_error {
                            continue;
                        }
                        return Some(e.list);
                    }
                }

                // Enforce the iteration budget before expanding this element.
                // In Fast mode max_iterations is usize::MAX, so this never fires.
                if self.iterations >= self.max_iterations {
                    println!("Max iterations reached");
                    break;
                }

                head.step(&e, self);

                // Token deletion: in fault-tolerant mode, offer to skip the
                // current token at a deletion cost.  The A* cost model ensures
                // this branch is only selected when it's cheaper than cascading
                // error insertions.
                if self.mode == ParseMode::FaultTolerant {
                    self.try_delete_token(&e);
                    self.try_sync_ahead(&e);
                }
            }
        }

        // Timeout (or empty heap with no complete parse).  Return the best
        // element that reached end-of-input, closing any still-open grammar
        // rules with synthetic End steps.
        self.best_at_eof.take().map(|(_, list, depth)| {
            let mut list = list;
            for _ in 0..depth.saturating_sub(1) {
                list = list.prepend(Step::end());
            }
            list
        })
    }

    /// Like `add_element` but pre-charges the element with the minimum error
    /// cost implied by the current token before adding it.  Called by
    /// generated `push_rule` code at sub-rule entry.
    ///
    /// `pushed_kind.min_error_for_token(tok)` returns 0 when the token is
    /// reachable inside the sub-rule (or the rule is nullable), and a positive
    /// value otherwise.  Pre-charging lets FaultTolerant mode still explore
    /// the branch (with a higher cost), while Fast mode drops it automatically
    /// via the `has_error` guard in `add_element`.
    ///
    /// When the token is unreachable, we also boost the heuristic (`h`) by
    /// `min_completion_cost` — the cheapest insertion cost to complete the
    /// entire pushed rule.  This gives the A* a tighter lower bound on
    /// remaining work, deprioritising dead-end branches without inflating
    /// the actual cost.  Child elements produced by expanding this one will
    /// receive fresh `h` values from the heuristic table, so the boost is
    /// naturally shed as progress is made.
    pub fn add_element_checked(&mut self, element: Element<R>, pushed_kind: R::Kind) -> bool {
        let mut pos = element.state.1;
        while self.tokens.get(pos).map_or(false, |t| t.kind.skips()) {
            pos += 1;
        }
        let element = if let Some(tok) = self.tokens.get(pos) {
            let min_err = pushed_kind.min_error_for_token(&tok.kind);
            if min_err > 0 {
                // Boost h (heuristic) by the rule's minimum completion cost.
                // This is admissible: it's a lower bound on the insertion cost
                // to complete the rule when the current token can't start it.
                let h_boost = pushed_kind.min_completion_cost();
                Element {
                    cost: element.cost + min_err,
                    h: element.h + h_boost,
                    has_error: true,
                    ..element
                }
            } else {
                element
            }
        } else {
            element
        };
        self.add_element(element)
    }

    pub fn add_element(&mut self, mut element: Element<R>) -> bool {
        // In Fast mode, elements that already contain an error can never
        // contribute to a valid parse.  Dropping them here prevents the heap
        // from filling with dead branches and stops infinite-loop searches on
        // invalid documents.
        if self.mode == ParseMode::Fast && element.has_error {
            return false;
        }

        // Drop elements that have exceeded the maximum consecutive error span.
        if element.consecutive_errors > self.max_repair_span {
            return false;
        }

        // dist(q, a) heuristic boost: if the top-of-stack rule has a known
        // minimum insertion cost to reach the next input token, add it to h.
        // This is the recursive-descent analog of the paper's dist(q, a_p).
        if self.mode == ParseMode::FaultTolerant {
            if let Some((head, _)) = element.parent.head() {
                let mut pos = element.state.1;
                while self.tokens.get(pos).map_or(false, |t| t.kind.skips()) {
                    pos += 1;
                }
                if let Some(tok) = self.tokens.get(pos) {
                    let d = head.state_dist(&tok.kind);
                    if d > 0 {
                        element.h = element.h.max(d);
                    }
                }
            }
        }

        let at = element.parent.head().map(|x| x.0.at()).unwrap_or(0);
        let state = (element.state.0, element.state.1, at);

        match self.done.entry(state) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                if element.cost >= *e.get() {
                    return false; // reject: equal-or-worse (higher cost) duplicate
                }
                *e.get_mut() = element.cost; // better (lower cost) path found
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(element.cost);
            }
        }

        self.todo.push(element);
        true
    }

    fn is_expected_element(
        &self,
        element: &Element<R>,
        found: &FatToken<R::Kind>,
    ) -> IsExpectedElement {
        match found.old_kind() {
            Some(old_fp) if old_fp == element.state.0 => IsExpectedElement::True,
            Some(_) => IsExpectedElement::False,
            None => IsExpectedElement::Unkown,
        }
    }

    pub fn expect_as(&mut self, element: &Element<R>, token: R::Kind) -> Element<R> {
        let (out, fallback) = self.expect_with_fallback(element, token, false);
        if let Some(fb) = fallback {
            if let Some(popped) = fb.pop() {
                self.add_element(popped);
            }
        }
        out
    }

    fn get_actual_index(&mut self, mut idx: usize) -> usize {
        while self
            .tokens
            .get(idx)
            .map(|x| x.kind.skips())
            .unwrap_or(false)
        {
            idx += 1;
        }

        idx
    }

    fn expect_with_fallback(
        &mut self,
        element: &Element<R>,
        token: R::Kind,
        wrapped: bool,
    ) -> (Element<R>, Option<Element<R>>) {
        let create_list = |step: Step<R::Kind>| {
            if wrapped {
                element
                    .list
                    .prepend(Step::start(token.clone()))
                    .prepend(step)
                    .prepend(Step::end())
            } else {
                element.list.prepend(step)
            }
        };

        let idx = element.state.1;

        // This thing actually matches
        if let Some(found) = self.tokens.get(idx) {
            if found.kind == token {
                let next = self.get_actual_index(idx + 1);
                // Update bracket depth after consuming this token.
                let new_depth = element.current_depth + token.bracket_delta();

                let (fallback, bias, h) = if self.mode == ParseMode::Fast {
                    // Fast mode: no role-conflict check, no fallback branch, h = 0.
                    (None, 0, 0)
                } else {
                    let is_expected_element = self.is_expected_element(element, found);
                    let (fallback, bias) = match is_expected_element {
                        IsExpectedElement::True | IsExpectedElement::Unkown => (None, 0),
                        IsExpectedElement::False => {
                            // Compute the depth delta this token carries from the
                            // previous parse and how it relates to the current depth.
                            let depth_delta = found
                                .old_depth()
                                .map(|old| element.current_depth as i16 - old as i16);
                            let committed_delta = element.assumed_depth_delta as i16;
                            if depth_delta == Some(committed_delta) && committed_delta != 0 {
                                // This conflict is fully explained by the depth shift
                                // already committed to — treat it as a free agree.
                                (None, 0)
                            } else if depth_delta == Some(committed_delta) {
                                // Same depth, fingerprint wrong: genuine role conflict.
                                // Restore the original +50 bias so the A* prefers a
                                // correct parse over reusing a token in the wrong role.
                                let insert_error = Some(Element {
                                    list: create_list(Step::error(Error::Expected(
                                        token.clone().into(),
                                    ))),
                                    parent: element.parent.clone(),
                                    cost: element.cost + token.max_error_value(),
                                    h: element.h,
                                    state: element.state,
                                    has_error: true,
                                    assumed_depth_delta: element.assumed_depth_delta,
                                    current_depth: element.current_depth,
                                    consecutive_errors: element.consecutive_errors.saturating_add(1),
                                });
                                (insert_error, 50)
                            } else {
                                // New or different depth conflict. Create two branches:
                                // (a) adopt this depth delta (one-time cost), and
                                // (b) insert an error token instead (don't consume).
                                let insert_error = Some(Element {
                                    list: create_list(Step::error(Error::Expected(
                                        token.clone().into(),
                                    ))),
                                    parent: element.parent.clone(),
                                    cost: element.cost + token.max_error_value(),
                                    h: element.h,
                                    state: element.state,
                                    has_error: true,
                                    assumed_depth_delta: element.assumed_depth_delta,
                                    current_depth: element.current_depth,
                                    consecutive_errors: element.consecutive_errors.saturating_add(1),
                                });
                                // One-time delta-adoption cost: proportional to how much
                                // the assumed delta changes.  When depth info is unavailable
                                // (None — because the previous parse had errors and the token
                                // was not at depth 0), treat as a free adoption: we cannot
                                // distinguish a genuine role conflict from a legitimate depth
                                // shift, so we do not penalise either.
                                let new_delta = depth_delta
                                    .map(|d| d.clamp(i8::MIN as i16, i8::MAX as i16) as i8)
                                    .unwrap_or(element.assumed_depth_delta);
                                let delta_change =
                                    (new_delta as i16 - element.assumed_depth_delta as i16).abs();
                                let adoption_cost = delta_change as isize * token.max_error_value();
                                (insert_error, adoption_cost)
                            }
                        }
                    };
                    (fallback, bias, self.heuristic[next])
                };

                let matched = Element {
                    list: create_list(Step::Bump(element.state.0)),
                    parent: element.parent.clone(),
                    cost: element.cost + token.max_error_value() + bias,
                    h,
                    state: (element.state.0, next),
                    has_error: element.has_error,
                    assumed_depth_delta: if bias > 0 {
                        // We adopted a new delta; compute it from the token's depth.
                        found
                            .old_depth()
                            .map(|old| {
                                (element.current_depth as i16 - old as i16)
                                    .clamp(i8::MIN as i16, i8::MAX as i16)
                                    as i8
                            })
                            .unwrap_or(element.assumed_depth_delta)
                    } else {
                        element.assumed_depth_delta
                    },
                    current_depth: new_depth,
                    consecutive_errors: 0, // successful match resets counter
                };

                return (matched, fallback);
            }
        }

        let h = if self.mode == ParseMode::Fast {
            0
        } else {
            self.heuristic[idx]
        };

        // The thing didn't match, so we just add that we expected the thing
        let error = Element {
            list: create_list(Step::error(Error::Expected(token.clone().into()))),
            parent: element.parent.clone(),
            cost: element.cost + token.max_error_value(),
            h,
            state: (element.state.0, idx),
            has_error: true,
            assumed_depth_delta: element.assumed_depth_delta,
            current_depth: element.current_depth,
            consecutive_errors: element.consecutive_errors.saturating_add(1),
        };

        (error, None)
    }

    /// Like `expect_as` but wraps the token in inline CST Start/End nodes,
    /// avoiding a push_rule + pop cycle (~3 Rc allocs, 2 heap ops, 1 fingerprint).
    ///
    /// Returns `(main, fallback)`.  On a role conflict the caller must apply
    /// `pop_push(next_rule)` to both before enqueuing so they get distinct
    /// `done` keys.
    pub fn expect_as_inline(
        &mut self,
        element: &Element<R>,
        token: R::Kind,
    ) -> (Element<R>, Option<Element<R>>) {
        self.expect_with_fallback(element, token, true)
    }

    /// Offer to skip (delete) the current token at a deletion cost.
    ///
    /// This is the recursive-descent analog of the (R3D) delete rule in
    /// Kim & Yi.  The token is not consumed by any grammar rule — it will
    /// appear in the CST wrapped in an Error node.  The grammar state and
    /// parent stack are unchanged; only the token position advances.
    fn try_delete_token(&mut self, element: &Element<R>) {
        let pos = element.state.1;
        let Some(tok) = self.tokens.get(pos) else {
            return;
        };
        // Don't delete whitespace / trivia (positions always point past them).
        if tok.kind.skips() {
            return;
        }

        let del_cost = tok.kind.deletion_cost();
        let next = self.get_actual_index(pos + 1);
        let h = if let Some(&hv) = self.heuristic.get(next) {
            hv
        } else {
            0
        };

        // History penalty: deleting a token that had a role in the previous
        // parse is discouraged — it's more likely the user intended to keep it.
        let history_penalty = if tok.old_kind().is_some() {
            self.bias.strength
        } else {
            0
        };

        let del_element = Element {
            list: element.list.prepend(Step::Delete),
            parent: element.parent.clone(),
            cost: element.cost + del_cost + history_penalty,
            h,
            state: (element.state.0, next),
            has_error: true,
            assumed_depth_delta: element.assumed_depth_delta,
            current_depth: element.current_depth,
            consecutive_errors: element.consecutive_errors.saturating_add(1),
        };
        self.add_element(del_element);
    }

    /// Synchronisation-point skip-ahead: when consecutive errors exceed a
    /// threshold, scan ahead for a token that an ancestor rule can accept.
    /// If found, create a single recovery element that:
    ///   1. Pops rules to the accepting ancestor
    ///   2. Wraps all skipped tokens in a single Error node (via Delete steps)
    ///   3. Resets consecutive_errors to 0
    ///
    /// This is the recursive-descent analog of Kim & Yi's (R3S-n) unrestricted
    /// shift — instead of shifting through LR states we pop grammar rules.
    fn try_sync_ahead(&mut self, element: &Element<R>) {
        if element.consecutive_errors < SYNC_THRESHOLD {
            return;
        }
        let pos = element.state.1;
        let max_scan = (pos + MAX_SYNC_SCAN).min(self.tokens.len());

        // Track the best sync candidate (lowest cost).
        let mut best: Option<Element<R>> = None;

        for scan_pos in (pos + 1)..max_scan {
            let tok = &self.tokens[scan_pos];
            if tok.kind.skips() {
                continue;
            }

            // Walk the parent stack looking for an ancestor that can accept
            // this token (min_error_for_token == 0).
            let mut parent = element.parent.clone();
            let mut list = element.list.clone();

            loop {
                let Some(((rule, saved_fp), tail)) = parent.slice().map(|((r, fp), t)| ((r, fp), t)) else {
                    break;
                };

                let kind = rule.element_kind();
                if kind.min_error_for_token(&tok.kind) == 0 {
                    // This ancestor can accept the sync token.
                    // Cost: deletion cost for each non-trivia token we skip.
                    let skip_cost: isize = (pos..scan_pos)
                        .filter_map(|p| self.tokens.get(p))
                        .filter(|t| !t.kind.skips())
                        .map(|t| t.kind.deletion_cost())
                        .sum();

                    let next = self.get_actual_index(scan_pos);
                    let h = self.heuristic.get(next).copied().unwrap_or(0);

                    let candidate = Element {
                        list,
                        parent: parent.clone(),
                        cost: element.cost + skip_cost,
                        h,
                        state: (*saved_fp, next),
                        has_error: true,
                        assumed_depth_delta: element.assumed_depth_delta,
                        current_depth: element.current_depth,
                        consecutive_errors: 0,
                    };

                    let f = candidate.cost + candidate.h;
                    if best.as_ref().map_or(true, |b| f < b.cost + b.h) {
                        best = Some(candidate);
                    }
                    break;
                }

                // Pop this rule: append an End step.
                list = list.prepend(Step::end());
                parent = tail.clone();

                // Don't pop the root rule.
                if parent.len() == 0 {
                    break;
                }
            }
        }

        if let Some(sync_element) = best {
            self.add_element(sync_element);
        }
    }
}

#[derive(Debug)]
pub struct Element<R: ParserTrait> {
    list: List<Step<R::Kind>>,
    parent: List<(R, Fingerprint)>,
    /// Accumulated error cost (lower = better).  Starts at 0; increases on
    /// error insertions, unchanged on successful matches.  Bias adjustments
    /// may slightly decrease cost (agreement) or increase it (conflict).
    cost: isize,
    /// Admissible heuristic: non-negative lower bound on the minimum
    /// additional cost from the current token position to end of input.
    /// Currently always 0.  `f = cost + h` is the A* priority.
    h: isize,
    state: (Fingerprint, usize),
    /// Set to `true` the moment any `Step::Error` is prepended to this
    /// element's list.  In `Fast` mode, elements with `has_error = true` are
    /// never returned as a successful parse result.
    has_error: bool,
    /// The bracket nesting depth shift this path has committed to.  When a
    /// block of tokens moves to a different nesting level, the first conflict
    /// adopts the delta (paying a one-time cost); subsequent tokens in the
    /// same block that conflict by the same delta are free.
    assumed_depth_delta: i8,
    /// The current bracket nesting depth of the parser at this element's token
    /// position.  Incremented when an opener is bumped, decremented for closers.
    current_depth: i8,
    /// Number of consecutive error operations (Error insertions + Delete steps)
    /// since the last successful token match (Bump).  Used by the bounded-repair
    /// mechanism: elements exceeding `max_repair_span` are dropped.
    consecutive_errors: u16,
}
impl<R: ParserTrait> PartialEq for Element<R> {
    fn eq(&self, other: &Self) -> bool {
        (self.cost + self.h) == (other.cost + other.h) && self.parent.len() == other.parent.len()
    }
}
impl<R: ParserTrait> Eq for Element<R> {}
impl<R: ParserTrait> Element<R> {
    fn new_at(current: R, h: isize, at: usize) -> Self {
        let parent = List::default();
        let head = Fingerprint(0);
        Self {
            list: List::default(),
            parent: parent.prepend((current, head)),
            cost: 0,
            h,
            state: (Fingerprint(1), at),
            has_error: false,
            assumed_depth_delta: 0,
            current_depth: 0,
            consecutive_errors: 0,
        }
    }

    pub fn pop_push(&self, rule: R) -> Self {
        let ((_, f), tail) = self.parent.slice().unwrap();
        let parent = tail.prepend((rule, *f));
        Self {
            parent,
            list: self.list.clone(),
            cost: self.cost,
            h: self.h,
            state: self.state.clone(),
            has_error: self.has_error,
            assumed_depth_delta: self.assumed_depth_delta,
            current_depth: self.current_depth,
            consecutive_errors: self.consecutive_errors,
        }
    }

    pub fn push(&self, rule: R) -> Self {
        let kind = rule.element_kind();
        let (s, a) = self.state;
        let parent = self.parent.prepend((rule, s));
        let list = self.list.prepend(Step::start(kind.clone()));
        let s = descend(s, kind.branch());
        Self {
            parent,
            list,
            cost: self.cost,
            h: self.h,
            state: (s, a),
            has_error: self.has_error,
            assumed_depth_delta: self.assumed_depth_delta,
            current_depth: self.current_depth,
            consecutive_errors: self.consecutive_errors,
        }
    }

    pub fn pop(&self) -> Option<Self> {
        let ((_, f), parent) = self.parent.slice()?;
        let list = self.list.prepend(Step::end());
        let (_, a) = self.state;
        Some(Self {
            parent: parent.clone(),
            list,
            cost: self.cost,
            h: self.h,
            state: (*f, a),
            has_error: self.has_error,
            assumed_depth_delta: self.assumed_depth_delta,
            current_depth: self.current_depth,
            consecutive_errors: self.consecutive_errors,
        })
    }
}

impl<R: ParserTrait> Ord for Element<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Min-heap on f = cost + h (lower f = fewer errors = higher priority).
        // Tiebreak: prefer shallower parent stacks (fewer pending rules).
        let f_self = self.cost + self.h;
        let f_other = other.cost + other.h;
        f_other
            .cmp(&f_self)
            .then(other.parent.len().cmp(&self.parent.len()))
    }
}
impl<R: ParserTrait> PartialOrd for Element<R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star<R: ParserTrait>(
    root: R,
    tokens: &[FatToken<R::Kind>],
    bias: IncrementalBias,
    max_iterations: usize,
) -> List<Step<R::Kind>> {
    let mut state = AStar::new(tokens, bias, max_iterations);
    // let h0 = state.heuristic[0];
    // state.todo.push(Element::new(root, h0));
    // Lets update the element to point to something that isn't whitespace
    state.consume(root).unwrap_or_default()
}

/// Like `a_star` but runs in non-fault-tolerant fast mode.  Returns `None` if
/// the document contains any error; `Some(steps)` on a clean parse.
pub fn a_star_fast<R: ParserTrait>(
    root: R,
    tokens: &[FatToken<R::Kind>],
) -> Option<List<Step<R::Kind>>> {
    let mut state = AStar::new_fast(tokens);
    state.consume(root)
}
