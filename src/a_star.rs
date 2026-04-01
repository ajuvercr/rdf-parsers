use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

use crate::{Error, FatToken, IncrementalBias, Step, TokenTrait, list::List};

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
    heuristic: Vec<isize>,
    /// Number of elements popped from the heap so far.
    iterations: usize,
    /// Maximum number of heap pops before giving up.  See `DEFAULT_MAX_ITERATIONS`.
    max_iterations: usize,
    /// Best element that reached end-of-input during search, tracked as
    /// `(cost, step_list, parent_depth)`.  Used as a fallback when the search
    /// times out before finding a fully-unwound parse (depth == 1).
    best_at_eof: Option<(isize, List<Step<R::Kind>>, usize)>,
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
        }
    }

    pub fn consume(&mut self, root: R) -> Option<List<Step<R::Kind>>> {
        let start_idx = self.get_actual_index(0);
        let h = if let Some(heurisitic) = self.heuristic.get(start_idx) {
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
                    if self
                        .best_at_eof
                        .as_ref()
                        .map_or(true, |&(c, _, _)| e.cost < c)
                    {
                        self.best_at_eof = Some((e.cost, e.list.clone(), depth));
                    }
                    if depth == 1 {
                        return Some(e.list);
                    }
                }

                // Enforce the iteration budget before expanding this element.
                if self.iterations >= self.max_iterations {
                    println!("Max iterations reached");
                    break;
                }

                head.step(&e, self);
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

    pub fn add_element(&mut self, element: Element<R>) -> bool {
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
                let is_expected_element = self.is_expected_element(element, found);
                let fallback = match is_expected_element {
                    IsExpectedElement::False => Some(Element {
                        list: create_list(Step::error(Error::Expected(token.clone()))),
                        parent: element.parent.clone(),
                        // So we just assume this is an error, so we add the error value
                        cost: element.cost + token.max_error_value(),
                        h: element.h,
                        state: element.state,
                    }),
                    _ => None,
                };

                let bias = match is_expected_element {
                    IsExpectedElement::True => 0,
                    // When we have to insert morethen 4 token, we
                    // should just assume it changed
                    IsExpectedElement::False => 40,
                    IsExpectedElement::Unkown => 0,
                };

                let matched = Element {
                    list: create_list(Step::Bump(element.state.0)),
                    parent: element.parent.clone(),
                    cost: element.cost + token.max_error_value() + bias,
                    h: self.heuristic[next],
                    state: (element.state.0, next),
                };

                return (matched, fallback);
            }
        }

        // The thing didn't match, so we just add that we expected the thing
        let error = Element {
            list: create_list(Step::error(Error::Expected(token.clone()))),
            parent: element.parent.clone(),
            cost: element.cost + token.max_error_value(),
            h: self.heuristic[idx],
            state: (element.state.0, idx),
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
