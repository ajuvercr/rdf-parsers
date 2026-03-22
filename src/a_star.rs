use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

use crate::{Error, FatToken, IncrementalBias, Step, TokenTrait, list::List};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Fingerprint(u128);

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

pub struct AStar<'a, R: ParserTrait> {
    /// Maps each state key to the best score seen so far (queued or processed).
    /// Used for lazy-deletion dedup: elements with worse scores are rejected
    /// early in `add_element`; stale heap entries are skipped in `consume`.
    done: HashMap<(Fingerprint, usize, usize), isize>,
    tokens: &'a [FatToken<R::Kind>],
    todo: BinaryHeap<Element<R>>,
    pub bias: IncrementalBias,
}

impl<'a, R: ParserTrait> AStar<'a, R> {
    fn new(tokens: &'a [FatToken<R::Kind>], bias: IncrementalBias) -> Self {
        Self {
            tokens,
            done: HashMap::new(),
            todo: BinaryHeap::new(),
            bias,
        }
    }

    pub fn consume(&mut self) -> Option<List<Step<R::Kind>>> {
        loop {
            let e = self.todo.pop()?;

            if let Some((head, _, _)) = e.parent.head() {
                let state = (e.state.0, e.state.1, head.at());

                // Skip stale entries: a better-scoring element for this state
                // was enqueued after this one.
                match self.done.get(&state) {
                    Some(&best) if best > e.score => continue,
                    _ => {}
                }

                if e.state.1 == self.tokens.len() && e.parent.len() == 1 {
                    return Some(e.list);
                }

                head.step(&e, self);
            }
        }
    }

    pub fn add_element(&mut self, element: Element<R>) -> bool {
        let at = element.parent.head().map(|x| x.0.at()).unwrap_or(0);
        let state = (element.state.0, element.state.1, at);

        match self.done.entry(state) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                if element.score <= *e.get() {
                    return false; // reject: equal-or-worse duplicate
                }
                *e.get_mut() = element.score; // better path found
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(element.score);
            }
        }

        self.todo.push(element);
        true
    }

    pub fn expect_as(
        &mut self,
        element: &Element<R>,
        token: R::Kind,
        error_value: isize,
    ) -> Element<R> {
        let mut idx = element.state.1;
        if let Some(found) = self.tokens.get(idx) {
            if found.kind == token {
                let list = element.list.prepend(Step::bump());
                idx += 1;
                while self
                    .tokens
                    .get(idx)
                    .map(|x| x.kind.skips())
                    .unwrap_or(false)
                {
                    idx += 1;
                }

                // Compute incremental bias using the cached role.
                let bias = match (found.old_kind(), element.cached_role.as_ref()) {
                    (Some(old), Some(cur)) if old == cur => self.bias.match_bonus,
                    (Some(_), Some(_)) => self.bias.conflict_penalty,
                    _ => 0,
                };

                if bias == self.bias.conflict_penalty && self.bias.conflict_penalty < 0 {
                    let fallback = Element {
                        list: element.list.prepend(Step::error(Error::Expected(token.clone()))),
                        parent: element.parent.clone(),
                        score: element.score + self.bias.match_bonus,
                        state: (element.state.0, element.state.1),
                        cached_role: element.cached_role.clone(),
                    };
                    if let Some(popped) = fallback.pop() {
                        self.add_element(popped);
                    }
                }

                return Element {
                    list,
                    parent: element.parent.clone(),
                    score: element.score + 2 + error_value + bias,
                    state: (element.state.0, idx),
                    cached_role: element.cached_role.clone(),
                };
            }
        }

        let list = element.list.prepend(Step::error(Error::Expected(token)));

        Element {
            list,
            parent: element.parent.clone(),
            score: element.score - error_value,
            state: (element.state.0, idx),
            cached_role: element.cached_role.clone(),
        }
    }

    /// Match a terminal token and wrap it in CST Start/End nodes inline,
    /// without pushing a separate rule onto the parent stack.
    ///
    /// This replaces the push_rule + terminal step + pop cycle for terminals,
    /// eliminating ~3 Rc allocations, 2 BinaryHeap operations, and 1
    /// fingerprint computation per terminal token.
    ///
    /// Returns `(main, fallback)`. When a conflict_penalty bias applies,
    /// `fallback` is `Some(element)` representing the error/no-consume path.
    /// The caller must apply `pop_push(next_rule)` to both before enqueuing:
    /// this gives the fallback a different `done` key (head.at() = NEXT ≠ K).
    pub fn expect_as_inline(
        &mut self,
        element: &Element<R>,
        token: R::Kind,
        error_value: isize,
    ) -> (Element<R>, Option<Element<R>>) {
        let mut idx = element.state.1;
        if let Some(found) = self.tokens.get(idx) {
            if found.kind == token {
                idx += 1;
                while self
                    .tokens
                    .get(idx)
                    .map(|x| x.kind.skips())
                    .unwrap_or(false)
                {
                    idx += 1;
                }

                // Compute incremental bias using the cached role.
                let bias = match (found.old_kind(), element.cached_role.as_ref()) {
                    (Some(old), Some(cur)) if old == cur => self.bias.match_bonus,
                    (Some(_), Some(_)) => self.bias.conflict_penalty,
                    _ => 0,
                };

                // When conflict_penalty applies, produce a fallback that records
                // an error without consuming the token. The caller applies
                // pop_push(NEXT) to both, so the fallback key becomes
                // (fp, same_pos, NEXT) which differs from current key (fp, pos, K).
                let fallback = if bias == self.bias.conflict_penalty && self.bias.conflict_penalty < 0 {
                    let fb_list = element.list
                        .prepend(Step::start(token.clone()))
                        .prepend(Step::error(Error::Expected(token.clone())))
                        .prepend(Step::end());
                    Some(Element {
                        list: fb_list,
                        parent: element.parent.clone(),
                        score: element.score + self.bias.match_bonus,
                        state: element.state,
                        cached_role: element.cached_role.clone(),
                    })
                } else {
                    None
                };

                // Build list: prepend Start, Bump, End (End at head).
                // After reversal in from_steps: Start, Bump, End — correct CST.
                let list = element.list
                    .prepend(Step::start(token))
                    .prepend(Step::bump())
                    .prepend(Step::end());

                return (
                    Element {
                        list,
                        parent: element.parent.clone(),
                        score: element.score + 2 + error_value + bias,
                        state: (element.state.0, idx),
                        cached_role: element.cached_role.clone(),
                    },
                    fallback,
                );
            }
        }

        // Error: token not present or wrong kind.
        let list = element.list
            .prepend(Step::start(token.clone()))
            .prepend(Step::error(Error::Expected(token)))
            .prepend(Step::end());

        (
            Element {
                list,
                parent: element.parent.clone(),
                score: element.score - error_value,
                state: (element.state.0, idx),
                cached_role: element.cached_role.clone(),
            },
            None,
        )
    }
}

#[derive(Debug)]
pub struct Element<R: ParserTrait> {
    list: List<Step<R::Kind>>,
    parent: List<(R, Fingerprint, Option<R::Kind>)>,
    score: isize,
    state: (Fingerprint, usize),
    /// Cached role: the innermost significant ancestor rule kind, maintained
    /// through push/pop/pop_push to avoid an O(depth) parent-chain walk per token.
    cached_role: Option<R::Kind>,
}
impl<R: ParserTrait> PartialEq for Element<R> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.parent.len() == other.parent.len()
    }
}
impl<R: ParserTrait> Eq for Element<R> {}
impl<R: ParserTrait> Element<R> {
    fn new(current: R) -> Self {
        let parent = List::default();
        let head = Fingerprint(0);
        let kind = current.element_kind();
        let cached_role = if kind.is_significant() { Some(kind) } else { None };
        Self {
            list: List::default(),
            parent: parent.prepend((current, head, None)),
            score: 0,
            state: (Fingerprint(1), 0),
            cached_role,
        }
    }

    pub fn pop_push(&self, rule: R) -> Self {
        let ((_, f, old_tt), tail) = self.parent.slice().unwrap();
        let kind = rule.element_kind();
        let cached_role = if kind.is_significant() { Some(kind) } else { old_tt.clone() };
        let parent = tail.prepend((rule, *f, old_tt.clone()));
        Self {
            parent,
            list: self.list.clone(),
            score: self.score,
            state: self.state.clone(),
            cached_role,
        }
    }

    pub fn push(&self, rule: R) -> Self {
        let kind = rule.element_kind();
        let (s, a) = self.state;
        let cached_role = if kind.is_significant() { Some(kind.clone()) } else { self.cached_role.clone() };
        let parent = self.parent.prepend((rule, s, self.cached_role.clone()));
        let list = self.list.prepend(Step::start(kind.clone()));
        let s = descend(s, kind.branch());
        Self {
            parent,
            list,
            score: self.score,
            state: (s, a),
            cached_role,
        }
    }

    pub fn pop(&self) -> Option<Self> {
        let ((_, f, old_tt), parent) = self.parent.slice()?;
        let list = self.list.prepend(Step::end());
        let (_, a) = self.state;
        Some(Self {
            parent: parent.clone(),
            list,
            score: self.score,
            state: (*f, a),
            cached_role: old_tt.clone(),
        })
    }
}

impl<R: ParserTrait> Ord for Element<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score
            .cmp(&other.score)
            .then(self.parent.len().cmp(&other.parent.len()))
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
) -> List<Step<R::Kind>> {
    let mut state = AStar::new(tokens, bias);
    state.todo.push(Element::new(root));
    state.consume().unwrap_or_default()
}
