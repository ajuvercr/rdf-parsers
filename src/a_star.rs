use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

use crate::{
    Error, FatToken, IncrementalBias, Step, TokenTrait,
    incremental::{Checkpoint, Memo, RawStep, Reuse},
    list::List,
};

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

/// Multiply-rotate hasher (as in rustc's FxHash) for the search's state
/// keys, which are already well-mixed fingerprints and small integers: the
/// standard SipHash is a needless cost on the hottest map in the parser.
#[derive(Default, Clone, Copy)]
struct FxHasher(u64);

impl std::hash::Hasher for FxHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(8) {
            let mut word = [0u8; 8];
            word[..chunk.len()].copy_from_slice(chunk);
            self.write_u64(u64::from_le_bytes(word));
        }
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = (self.0.rotate_left(5) ^ n).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
    fn write_u128(&mut self, n: u128) {
        self.write_u64(n as u64);
        self.write_u64((n >> 64) as u64);
    }
    fn write_usize(&mut self, n: usize) {
        self.write_u64(n as u64);
    }
}

type FxBuild = std::hash::BuildHasherDefault<FxHasher>;

/// One entry of the rule stack.
#[derive(Debug)]
struct Frame<R> {
    rule: R,
    /// The state fingerprint of the parent, restored when this rule pops.
    saved: Fingerprint,
    /// Role fingerprint of tokens consumed directly by this rule.
    ///
    /// The state fingerprint hashes the whole rule path, so it changes for
    /// every token after an edit that adds or removes a level of recursion
    /// (SPARQL's `TriplesBlock ::= TriplesSameSubjectPath ("." TriplesBlock?)?`
    /// nests once per triple).  The role hashes the path with cycles
    /// collapsed: re-entering a rule already on the stack takes that rule's
    /// role.  So a triple's subject has the same role at any nesting, and
    /// tokens keep their role from the previous parse across such edits.
    role: Fingerprint,
}

fn descend(fp: Fingerprint, branch_id: u32) -> Fingerprint {
    const M: u128 = 0x9E3779B97F4A7C15_6A09E667F3BCC909;
    Fingerprint(fp.0.wrapping_mul(M) ^ (branch_id as u128).wrapping_add(0xD6E8FEB86659FD93))
}

pub trait ParserTrait: Debug + Sized + 'static {
    type Kind: 'static + TokenTrait;

    fn step(&self, el: &Element<Self>, state: &mut AStar<Self>);
    fn at(&self) -> usize;
    fn element_kind(&self) -> Self::Kind;
    /// Rebuilds the rule in state `at` (see `at`), e.g. from a checkpoint.
    fn from_parts(kind: Self::Kind, at: usize) -> Self;

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
pub const DEFAULT_MAX_REPAIR_SPAN: u16 = 10;

/// How many tokens a cheaper path must parse cleanly past a costlier one
/// before the costlier one is dropped.  See `AStar::dominated`.
pub const DEFAULT_DOMINANCE_WINDOW: usize = 12;

/// Maximum heap pops at a single token position in fault-tolerant mode.
/// Elements are popped cheapest-first, so once a position has used its budget
/// the remaining elements there are the least promising and are parked.  This
/// bounds the search at O(tokens × budget) whatever the grammar does.
pub const DEFAULT_POPS_PER_POSITION: u32 = 4096;

/// Number of consecutive error operations that triggers sync-point scanning.
/// The paper's (R3S-n) is always available; we gate it at 2 to avoid
/// scanning overhead on isolated single-token errors while still activating
/// early enough to prevent dead-end branch accumulation.
const SYNC_THRESHOLD: u16 = 2;

/// Maximum number of tokens to scan ahead when looking for a sync point.
const MAX_SYNC_SCAN: usize = 50;

pub struct AStar<'a, R: ParserTrait> {
    /// Maps each state key to the best (lowest) cost seen so far (queued or
    /// processed).  Used for lazy-deletion dedup: elements with worse (higher)
    /// costs are rejected early in `add_element`; stale heap entries are
    /// skipped in `consume`.
    done: HashMap<(Fingerprint, usize, usize), isize, FxBuild>,
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
    best_at_eof: Option<(isize, List<Step<R::Kind>>, usize, List<Mark<R>>)>,
    /// Whether to perform fault-tolerant error recovery.
    mode: ParseMode,
    /// Maximum consecutive error operations before an element is dropped.
    max_repair_span: u16,
    /// Frontier-dominance pruning, indexed by excess cost (cost above a
    /// perfect match of the tokens consumed); see `dominated`.
    ///
    /// Without this, an error raises the optimal cost and every earlier
    /// position gets enough slack to explore spurious repairs, so the search
    /// spends most of its time before the error.  With it, repairs only happen
    /// near where the cheaper parse got stuck.
    frontier: Vec<usize>,
    dominance_window: usize,
    /// Token position the search started at.
    start: usize,
    /// Heap pops per token position; see `DEFAULT_POPS_PER_POSITION`.
    pops_at: Vec<u32>,
    pops_per_position: u32,
    /// Elements dropped by the pruning above.  Pruning bets that the cheaper
    /// paths ahead will finish; when they all dead-end instead, these are put
    /// back with the limits loosened, so pruning never loses the only parse.
    parked: Vec<Element<R>>,
    unpark_rounds: u32,
    /// What the previous parse left behind, for incremental parsing.
    reuse: Option<&'a Reuse<'a>>,
    /// Cheapest path completed with a previous parse's steps so far.
    spliced: Option<Spliced<R>>,
    /// Elements reached from the one being expanded without consuming a
    /// token or adding cost: entering or leaving a rule, moving to its next
    /// state.  They would be popped next anyway, so they skip the heap and
    /// are expanded straight away (last in, first out).  Rule chains like
    /// SPARQL's ten expression levels then cost a few cheap expansions per
    /// token instead of dozens of heap operations, and don't count against
    /// `pops_per_position`.
    eager: Vec<Element<R>>,
    /// `(f, token position)` of the element being expanded.
    expanding: Option<(isize, usize)>,
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
            done: HashMap::default(),
            todo: BinaryHeap::new(),
            bias,
            heuristic,
            iterations: 0,
            max_iterations,
            best_at_eof: None,
            mode: ParseMode::FaultTolerant,
            max_repair_span: DEFAULT_MAX_REPAIR_SPAN,
            frontier: Vec::new(),
            dominance_window: DEFAULT_DOMINANCE_WINDOW,
            start: 0,
            pops_at: vec![0; tokens.len() + 1],
            parked: Vec::new(),
            unpark_rounds: 0,
            pops_per_position: DEFAULT_POPS_PER_POSITION,
            reuse: None,
            spliced: None,
            eager: Vec::new(),
            expanding: None,
        }
    }

    /// Creates an `AStar` configured for fast, non-fault-tolerant parsing.
    /// Skips heuristic precomputation, incremental bias, and best-at-EOF
    /// tracking.  Returns `None` from `consume` if the document contains any
    /// error.
    fn new_fast(tokens: &'a [FatToken<R::Kind>]) -> Self {
        Self {
            tokens,
            done: HashMap::default(),
            todo: BinaryHeap::new(),
            bias: IncrementalBias { strength: 0 },
            heuristic: Vec::new(),
            iterations: 0,
            max_iterations: usize::MAX,
            best_at_eof: None,
            mode: ParseMode::Fast,
            max_repair_span: u16::MAX,
            frontier: Vec::new(),
            dominance_window: usize::MAX,
            start: 0,
            pops_at: Vec::new(),
            parked: Vec::new(),
            unpark_rounds: 0,
            pops_per_position: u32::MAX,
            reuse: None,
            spliced: None,
            eager: Vec::new(),
            expanding: None,
        }
    }

    fn consume(&mut self, root: R) -> Option<Found<R>> {
        let resume = self
            .reuse
            .and_then(|r| r.resume.map(|i| (&r.memo.checkpoints[i], r.prefix(i))));
        let start_idx = match &resume {
            Some((cp, _)) => cp.token,
            None => self.get_actual_index(0),
        };
        let h = if self.mode == ParseMode::Fast {
            0
        } else if let Some(heurisitic) = self.heuristic.get(start_idx) {
            *heurisitic
        } else {
            return Some(Found {
                list: List::default(),
                marks: List::default(),
                cost: 0,
                spliced: None,
            });
        };
        self.start = start_idx;
        self.todo.push(match resume {
            Some((cp, prefix)) => Element::resume(cp, prefix, h),
            None => Element::new_at(root, h, start_idx),
        });

        loop {
            // A path completed with old steps wins once nothing left in the
            // queue can beat it.
            if let Some(s) = &self.spliced
                && self.eager.is_empty()
                && self.todo.peek().is_none_or(|t| t.cost + t.h >= s.total)
            {
                let s = self.spliced.take().unwrap();
                return Some(Found {
                    list: s.list,
                    marks: s.marks,
                    cost: s.total,
                    spliced: Some(s.checkpoint),
                });
            }

            let mut e = match self.eager.pop() {
                Some(e) => e,
                None => match self.todo.pop() {
                    Some(e) => e,
                    None if !self.parked.is_empty() => {
                        self.unpark();
                        continue;
                    }
                    None => break,
                },
            };
            self.iterations += 1;

            if let Some(at) = e.parent.head().map(|f| f.rule.at()) {
                let state = (e.state.0, e.state.1, at);

                // Skip stale entries: a better-scoring element for this state
                // was enqueued after this one.
                match self.done.get(&state) {
                    Some(&best) if best < e.cost => continue,
                    _ => {}
                }

                let pos = e.state.1;
                if pos >= e.marks.head().map_or(self.start, |m| m.token) + MARK_INTERVAL {
                    e.marks = e.marks.prepend(Mark {
                        token: pos,
                        stack: e.parent.clone(),
                        fp: e.state.0,
                        depth: e.current_depth,
                        steps: e.list.len(),
                        cost: e.cost,
                    });
                }
                if self.try_splice(&e) {
                    continue;
                }

                if e.state.1 == self.tokens.len() {
                    let depth = e.parent.len();
                    if self.mode == ParseMode::FaultTolerant
                        && self
                            .best_at_eof
                            .as_ref()
                            .map_or(true, |(c, ..)| e.cost < *c)
                    {
                        self.best_at_eof =
                            Some((e.cost, e.list.clone(), depth, e.marks.clone()));
                    }
                    if depth == 1 {
                        // In Fast mode, reject any parse that contains an error step.
                        if self.mode == ParseMode::Fast && e.has_error {
                            continue;
                        }
                        return Some(Found {
                            list: e.list,
                            marks: e.marks,
                            cost: e.cost,
                            spliced: None,
                        });
                    }
                }

                if self.mode == ParseMode::FaultTolerant {
                    let pops = &mut self.pops_at[e.state.1];
                    if *pops >= self.pops_per_position {
                        self.parked.push(e);
                        continue;
                    }
                    *pops += 1;
                    if self.dominated(&mut e) {
                        self.parked.push(e);
                        continue;
                    }
                }

                // Enforce the iteration budget before expanding this element.
                // In Fast mode max_iterations is usize::MAX, so this never fires.
                if self.iterations >= self.max_iterations {
                    eprintln!("Max iterations reached");
                    break;
                }

                self.expanding = Some((e.cost + e.h, e.state.1));
                let eager_from = self.eager.len();
                if let Some(frame) = e.parent.head() {
                    frame.rule.step(&e, self);
                }
                // Expand this step's successors in grammar order, so ties go to
                // the first alternative (an IRI over `false` for a missing
                // object), as they do through the heap.
                self.eager[eager_from..].reverse();
                self.expanding = None;

                // Token deletion and sync-ahead: in fault-tolerant mode, offer
                // to skip the current token at a deletion cost.
                //
                // Gate on shifts_since_pop: the paper (Kim & Yi, §3.3) proves
                // that when t > 1 (multiple shifts since last reduction), the
                // only useful operations are insertions via shift/goto paths.
                // Offering deletions at this point generates redundant configs
                // that the A* will eventually discard — but they bloat the heap.
                if self.mode == ParseMode::FaultTolerant && e.shifts_since_pop <= 1 {
                    self.try_delete_token(&e);
                    self.try_sync_ahead(&e);
                }
            }
        }

        // Timeout (or empty heap with no complete parse).  Return the best
        // element that reached end-of-input, closing any still-open grammar
        // rules with synthetic End steps.
        if let Some(s) = self.spliced.take() {
            return Some(Found {
                list: s.list,
                marks: s.marks,
                cost: s.total,
                spliced: Some(s.checkpoint),
            });
        }
        self.best_at_eof.take().map(|(cost, list, depth, marks)| {
            let mut list = list;
            for _ in 0..depth.saturating_sub(1) {
                list = list.prepend(Step::end());
            }
            Found {
                list,
                marks,
                cost,
                spliced: None,
            }
        })
    }

    /// If `e` is in the state a previous parse was in at this token, records
    /// it as completed by that parse's remaining steps and returns true: the
    /// tokens ahead are unchanged, so the old steps are the best way on.
    fn try_splice(&mut self, e: &Element<R>) -> bool {
        let Some(reuse) = self.reuse else {
            return false;
        };
        let Some(candidates) = reuse.splice.get(&e.state.1) else {
            return false;
        };
        for &i in candidates {
            let cp = &reuse.memo.checkpoints[i];
            if cp.fp != e.state.0
                || (self.mode == ParseMode::Fast && !cp.clean_after)
                || !same_stack(&e.parent, &cp.stack)
            {
                continue;
            }
            let total = e.cost + reuse.memo.cost - cp.cost;
            if self.spliced.as_ref().is_none_or(|s| total < s.total) {
                self.spliced = Some(Spliced {
                    total,
                    list: e.list.clone(),
                    marks: e.marks.clone(),
                    checkpoint: i,
                });
            }
            return true;
        }
        false
    }

    /// Every live path dead-ended: resume pruned ones, nearest to where the
    /// search got stuck first.  Each round reaches twice as far back and
    /// loosens the limits, so in the worst case everything is restored.
    fn unpark(&mut self) {
        self.unpark_rounds += 1;
        let hi = self.parked.iter().map(|e| e.state.1).max().unwrap_or(0);
        let reach = DEFAULT_DOMINANCE_WINDOW << self.unpark_rounds.min(16);
        let cut = hi.saturating_sub(reach);
        let (resume, keep): (Vec<_>, Vec<_>) = std::mem::take(&mut self.parked)
            .into_iter()
            .partition(|e| e.state.1 >= cut);
        self.parked = keep;
        self.frontier.clear();
        self.dominance_window = self.dominance_window.saturating_mul(2);
        self.pops_per_position = self.pops_per_position.saturating_mul(2);
        self.todo.extend(resume);
    }

    /// Frontier-dominance check for a freshly popped element.
    ///
    /// A path is dropped when a path with strictly lower excess cost is at
    /// least `dominance_window` tokens ahead of it and parsed those last
    /// tokens cleanly (without adding cost): that path has recovered from
    /// whatever went wrong and confirmed it on real input, so this costlier
    /// alternative is not worth exploring.  Requiring the clean run matters:
    /// a path that is merely cheaper *so far* may be racking up small repairs
    /// that end up costing more than one bigger fix.
    fn dominated(&mut self, e: &mut Element<R>) -> bool {
        let pos = e.state.1;
        let matched = self.heuristic[self.start] - self.heuristic[pos];
        let excess = e.cost - matched;
        if excess != e.clean.0 {
            e.clean = (excess, pos);
        }
        let level = excess.max(0) as usize;
        if let Some(below) = level.min(self.frontier.len()).checked_sub(1)
            && self.frontier[below] > pos
        {
            return true;
        }
        if pos - e.clean.1 >= self.dominance_window {
            // `frontier[x]` is 1 + the furthest position that some path with
            // excess at most `x` has a clean window of tokens beyond (0 =
            // none).  Taking the max over excess `<= x` keeps it
            // non-decreasing.
            let run_start = pos - self.dominance_window + 1;
            if self.frontier.len() <= level {
                let last = self.frontier.last().copied().unwrap_or(0);
                self.frontier.resize(level + 1, last);
            }
            for reach in &mut self.frontier[level..] {
                if *reach >= run_start {
                    break;
                }
                *reach = run_start;
            }
        }
        false
    }

    /// Like `add_element` but pre-charges the element with the minimum error
    /// cost implied by the current token before adding it when in Fast mode.
    /// In FaultTolerant mode, explores the branch without pre-charging to allow
    /// lower-cost alternatives to be found.  Called by generated `push_rule`
    /// code at sub-rule entry.
    ///
    /// `pushed_kind.min_error_for_token(tok)` returns 0 when the token is
    /// reachable inside the sub-rule (or the rule is nullable), and a positive
    /// value otherwise.  Pre-charging in Fast mode lets early pruning happen
    /// automatically via the `has_error` guard in `add_element`.
    pub fn add_element_checked(&mut self, element: Element<R>, pushed_kind: R::Kind) -> bool {
        if self.mode == ParseMode::Fast {
            let mut pos = element.state.1;
            while self.tokens.get(pos).map_or(false, |t| t.kind.skips()) {
                pos += 1;
            }
            let element = if let Some(tok) = self.tokens.get(pos) {
                let min_err = pushed_kind.min_error_for_token(&tok.kind);
                if min_err > 0 {
                    Element {
                        cost: element.cost + min_err,
                        has_error: true,
                        ..element
                    }
                } else {
                    element
                }
            } else {
                element
            };
            return self.add_element(element);
        }
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

        // dist(q, a) heuristic boost: the top-of-stack rule's minimum
        // insertion cost to reach the next input token.  This is additive with
        // the suffix-sum heuristic (which counts matching costs for remaining
        // tokens) because they measure different, non-overlapping costs:
        //   h_suffix = lower bound on cost of matching all remaining tokens
        //   dist     = lower bound on insertion cost before next token can match
        if self.mode == ParseMode::FaultTolerant {
            if let Some(Frame { rule: head, .. }) = element.parent.head() {
                let mut pos = element.state.1;
                while self.tokens.get(pos).map_or(false, |t| t.kind.skips()) {
                    pos += 1;
                }
                let new_d = self
                    .tokens
                    .get(pos)
                    .map(|tok| head.state_dist(&tok.kind).max(0))
                    .unwrap_or(0);
                element.h = element.h - element.dist + new_d;
                element.dist = new_d;
            }
        }

        let at = element.parent.head().map(|f| f.rule.at()).unwrap_or(0);
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

        match self.expanding {
            Some((f, pos)) if element.state.1 == pos && element.cost + element.h <= f => {
                self.eager.push(element)
            }
            _ => self.todo.push(element),
        }
        true
    }

    fn is_expected_element(
        &self,
        element: &Element<R>,
        found: &FatToken<R::Kind>,
    ) -> IsExpectedElement {
        match found.old_kind() {
            // Roles ignore unbracketed recursion (see `Frame`), so nesting
            // only counts through brackets: the token must also be at the
            // bracket depth it had before.  Otherwise it goes through the
            // depth-shift handling in `expect_with_fallback`.
            Some(old_fp)
                if old_fp == element.role()
                    && found
                        .old_depth()
                        .is_none_or(|d| d as i16 == element.current_depth as i16) =>
            {
                IsExpectedElement::True
            }
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
                // Update bracket depth after consuming this token.  Inserted
                // (missing) brackets count too: the parse continues as if
                // they were there.
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
                                    current_depth: element.current_depth + token.bracket_delta(),
                                    consecutive_errors: element
                                        .consecutive_errors
                                        .saturating_add(1),
                                    shifts_since_pop: element.shifts_since_pop,
                                    dist: element.dist,
                                    clean: element.clean,
                                    marks: element.marks.clone(),
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
                                    current_depth: element.current_depth + token.bracket_delta(),
                                    consecutive_errors: element
                                        .consecutive_errors
                                        .saturating_add(1),
                                    shifts_since_pop: element.shifts_since_pop,
                                    dist: element.dist,
                                    clean: element.clean,
                                    marks: element.marks.clone(),
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
                    list: create_list(Step::Bump(element.role())),
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
                    shifts_since_pop: element.shifts_since_pop.saturating_add(1),
                    dist: 0, // h is a fresh suffix-sum; no dist folded in yet
                    clean: element.clean,
                    marks: element.marks.clone(),
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
            current_depth: element.current_depth + token.bracket_delta(),
            consecutive_errors: element.consecutive_errors.saturating_add(1),
            shifts_since_pop: element.shifts_since_pop,
            dist: 0, // h is a fresh suffix-sum
            clean: element.clean,
            marks: element.marks.clone(),
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
            shifts_since_pop: element.shifts_since_pop,
            dist: 0, // h is a fresh suffix-sum
            clean: element.clean,
            marks: element.marks.clone(),
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
                let Some((Frame { rule, saved: saved_fp, .. }, tail)) = parent.slice() else {
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
                        shifts_since_pop: 0, // sync resets — we popped to ancestor
                        dist: 0,             // h is a fresh suffix-sum
                        clean: element.clean,
                        marks: element.marks.clone(),
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
    parent: List<Frame<R>>,
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
    /// Consecutive shifts (bumps) since the last reduction (pop).  Paper's `t`
    /// counter.  When > 0, the parser is at a known intra-rule state, so
    /// `dist(q, a)` is exact (not diluted by the conservative pop = 0 fallback).
    /// This enables hard pruning of unreachable branches.
    shifts_since_pop: u16,
    /// The `state_dist(next_token)` value already folded into `h`.  Maintained
    /// so that `add_element` can replace it exactly (`h = h - dist + new_d`)
    /// rather than accumulating it across push/pop expansions at the same token
    /// position.  Invariant: `h = h_suffix + dist`.  Reset to 0 whenever `h`
    /// is assigned a fresh suffix-sum value.
    dist: isize,
    /// `(excess, pos)`: the excess cost of this path and the token position
    /// from which it has stayed at that excess.  See `AStar::dominated`.
    clean: (isize, usize),
    /// Parser states along this path, newest first; see `Mark`.
    marks: List<Mark<R>>,
}

/// Minimum distance in token indices (trivia included) between two marks.
const MARK_INTERVAL: usize = 32;

/// A parser state on a path, recorded every `MARK_INTERVAL` tokens.  The
/// winning path's marks become the `Checkpoint`s the next incremental parse
/// resumes from or splices at.
#[derive(Debug)]
struct Mark<R: ParserTrait> {
    token: usize,
    stack: List<Frame<R>>,
    fp: Fingerprint,
    depth: i8,
    steps: usize,
    cost: isize,
}

/// A path that reached the state of a previous parse's checkpoint, after
/// the edit: it completes with that parse's remaining steps.
struct Spliced<R: ParserTrait> {
    /// Cost of the completed parse.
    total: isize,
    list: List<Step<R::Kind>>,
    marks: List<Mark<R>>,
    checkpoint: usize,
}

/// The parse a search settled on.
pub(crate) struct Found<R: ParserTrait> {
    list: List<Step<R::Kind>>,
    marks: List<Mark<R>>,
    /// Cost of `list` (from the resume point, if any).
    cost: isize,
    /// Checkpoint whose old steps complete `list`.
    spliced: Option<usize>,
}
impl<R: ParserTrait> PartialEq for Element<R> {
    fn eq(&self, other: &Self) -> bool {
        (self.cost + self.h) == (other.cost + other.h) && self.parent.len() == other.parent.len()
    }
}
impl<R: ParserTrait> Eq for Element<R> {}
impl<R: ParserTrait> Element<R> {
    fn new_at(current: R, h: isize, at: usize) -> Self {
        let parent = List::default().prepend(Frame {
            rule: current,
            saved: Fingerprint(0),
            role: Fingerprint(1),
        });
        Self {
            list: List::default(),
            parent,
            cost: 0,
            h,
            state: (Fingerprint(1), at),
            has_error: false,
            assumed_depth_delta: 0,
            current_depth: 0,
            consecutive_errors: 0,
            shifts_since_pop: 0,
            dist: 0,
            clean: (0, at),
            marks: List::default(),
        }
    }

    /// An element in the state of checkpoint `cp`, with `prefix` as the steps
    /// taken so far.  Its cost counts from here.
    fn resume(cp: &Checkpoint, prefix: List<Step<R::Kind>>, h: isize) -> Self {
        let parent = cp.stack.iter().rev().fold(List::default(), |list, &(kind, at, saved, role)| {
            list.prepend(Frame {
                rule: R::from_parts(kind.into(), at as usize),
                saved,
                role,
            })
        });
        Self {
            list: prefix,
            parent,
            cost: 0,
            h,
            state: (cp.fp, cp.token),
            has_error: false,
            assumed_depth_delta: 0,
            current_depth: cp.depth,
            consecutive_errors: 0,
            shifts_since_pop: 0,
            dist: 0,
            clean: (0, cp.token),
            marks: List::default(),
        }
    }

    /// Role fingerprint of the token this element would consume; see `Frame`.
    fn role(&self) -> Fingerprint {
        self.parent.head().map_or(Fingerprint(1), |f| f.role)
    }

    pub fn pop_push(&self, rule: R) -> Self {
        let (frame, tail) = self.parent.slice().unwrap();
        let parent = tail.prepend(Frame {
            rule,
            saved: frame.saved,
            role: frame.role,
        });
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
            shifts_since_pop: self.shifts_since_pop,
            dist: self.dist,
            clean: self.clean,
            marks: self.marks.clone(),
        }
    }

    pub fn push(&self, rule: R) -> Self {
        let kind = rule.element_kind();
        let (s, a) = self.state;
        let role = self
            .parent
            .iter()
            .find(|f| f.rule.element_kind() == kind)
            .map_or_else(|| descend(self.role(), kind.branch()), |f| f.role);
        let parent = self.parent.prepend(Frame {
            rule,
            saved: s,
            role,
        });
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
            shifts_since_pop: self.shifts_since_pop,
            dist: self.dist,
            clean: self.clean,
            marks: self.marks.clone(),
        }
    }

    pub fn pop(&self) -> Option<Self> {
        let (frame, parent) = self.parent.slice()?;
        let list = self.list.prepend(Step::end());
        let (_, a) = self.state;
        Some(Self {
            parent: parent.clone(),
            list,
            cost: self.cost,
            h: self.h,
            state: (frame.saved, a),
            has_error: self.has_error,
            assumed_depth_delta: self.assumed_depth_delta,
            current_depth: self.current_depth,
            consecutive_errors: self.consecutive_errors,
            shifts_since_pop: 0, // reduction resets t counter
            dist: self.dist,
            clean: self.clean,
            marks: self.marks.clone(),
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

thread_local! {
    static LAST_ITERATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Heap pops performed by the most recent search on this thread.  Diagnostic
/// only, for benchmarks.
#[doc(hidden)]
pub fn last_iterations() -> usize {
    LAST_ITERATIONS.with(|c| c.get())
}

fn same_stack<R: ParserTrait>(
    stack: &List<Frame<R>>,
    plain: &[(rowan::SyntaxKind, u32, Fingerprint, Fingerprint)],
) -> bool {
    stack.len() == plain.len()
        && stack.iter().zip(plain).all(|(f, &(kind, at, saved, _))| {
            f.saved == saved && f.rule.at() == at as usize && f.rule.element_kind().into() == kind
        })
}

pub fn a_star<R: ParserTrait>(
    root: R,
    tokens: &[FatToken<R::Kind>],
    bias: IncrementalBias,
    max_iterations: usize,
) -> List<Step<R::Kind>> {
    search(root, tokens, ParseMode::FaultTolerant, bias, max_iterations, None)
        .map_or_else(List::default, |f| f.list)
}

/// Like `a_star` but runs in non-fault-tolerant fast mode.  Returns `None` if
/// the document contains any error; `Some(steps)` on a clean parse.
pub fn a_star_fast<R: ParserTrait>(
    root: R,
    tokens: &[FatToken<R::Kind>],
) -> Option<List<Step<R::Kind>>> {
    search(root, tokens, ParseMode::Fast, IncrementalBias { strength: 0 }, usize::MAX, None)
        .map(|f| f.list)
}

pub(crate) fn search<'a, R: ParserTrait>(
    root: R,
    tokens: &'a [FatToken<R::Kind>],
    mode: ParseMode,
    bias: IncrementalBias,
    max_iterations: usize,
    reuse: Option<&'a Reuse<'a>>,
) -> Option<Found<R>> {
    let mut state = match mode {
        ParseMode::FaultTolerant => AStar::new(tokens, bias, max_iterations),
        ParseMode::Fast => AStar::new_fast(tokens),
    };
    state.reuse = reuse;
    let out = state.consume(root);
    LAST_ITERATIONS.with(|c| c.set(state.iterations));
    out
}

impl<R: ParserTrait> Found<R> {
    /// The complete step list, plus the memo the next incremental parse
    /// reuses.  `reuse` must be what the search ran with.
    pub(crate) fn finish(self, reuse: Option<&Reuse>) -> (List<Step<R::Kind>>, Memo) {
        let resumed = reuse.and_then(|r| r.resume.map(|i| (r, i)));
        // Costs in this search count from the resume point.
        let base = resumed.map_or(0, |(r, i)| r.memo.checkpoints[i].cost);

        // Checkpoints before the resume point are unchanged.
        let mut checkpoints: Vec<Checkpoint> = resumed
            .map(|(r, i)| r.memo.checkpoints[..=i].to_vec())
            .unwrap_or_default();
        let mut marks: Vec<&Mark<R>> = self.marks.iter().collect();
        marks.reverse();
        checkpoints.extend(marks.into_iter().map(|m| Checkpoint {
            token: m.token,
            steps: m.steps,
            cost: base + m.cost,
            fp: m.fp,
            depth: m.depth,
            stack: m
                .stack
                .iter()
                .map(|f| (f.rule.element_kind().into(), f.rule.at() as u32, f.saved, f.role))
                .collect(),
            clean_before: false,
            clean_after: false,
        }));

        let mut list = self.list;
        if let (Some(i), Some(r)) = (self.spliced, reuse) {
            let memo = r.memo;
            let at = &memo.checkpoints[i];
            let (steps, cost) = (list.len(), base + self.cost - memo.cost + at.cost);
            // Checkpoints from the splice point on, shifted to their new place.
            checkpoints.extend(memo.checkpoints[i..].iter().map(|cp| Checkpoint {
                token: (cp.token as isize + r.delta) as usize,
                steps: cp.steps - at.steps + steps,
                cost: cp.cost - at.cost + cost,
                ..cp.clone()
            }));
            for s in &memo.steps[at.steps..] {
                list = list.prepend(s.to_step());
            }
        }

        let mut steps: Vec<RawStep> = list.iter().map(RawStep::from_step).collect();
        steps.reverse();
        (list, Memo::new(steps, checkpoints, base + self.cost))
    }
}
