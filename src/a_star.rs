use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
};

use crate::{Error, FatToken, Step, TokenTrait, list::List};

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
    done: HashSet<(Fingerprint, usize, usize)>,
    tokens: &'a [FatToken<R::Kind>],
    todo: BinaryHeap<Element<R>>,
}

impl<'a, R: ParserTrait> AStar<'a, R> {
    fn new(tokens: &'a [FatToken<R::Kind>]) -> Self {
        Self {
            tokens,
            done: HashSet::new(),
            todo: BinaryHeap::new(),
        }
    }

    pub fn consume(&mut self) -> Option<List<Step<R::Kind>>> {
        loop {
            let e = self.todo.pop()?;

            if e.state.1 == self.tokens.len() && e.parent.len() == 1 {
                return Some(e.list);
            }

            if let Some((head, _)) = e.parent.head() {
                let state = (e.state.0, e.state.1, head.at());
                if self.done.contains(&state) {
                    continue;
                }

                self.done.insert(state);
                head.step(&e, self);
            }
        }
    }

    pub fn add_element(&mut self, element: Element<R>) -> bool {
        let at = element.parent.head().map(|x| x.0.at()).unwrap_or(0);
        let state = (element.state.0, element.state.1, at);

        if self.done.contains(&state) {
            return false;
        }


        self.todo.push(element);
        true
    }

    pub fn expect_as(
        &self,
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
                return Element {
                    list,
                    parent: element.parent.clone(),
                    score: element.score + 2 + error_value,
                    state: (element.state.0, idx),
                };
            }
        }

        let list = element.list.prepend(Step::error(Error::Expected(token)));

        Element {
            list,
            parent: element.parent.clone(),
            score: element.score - error_value,
            state: (element.state.0, idx),
        }
    }
}

#[derive(Debug)]
pub struct Element<R: ParserTrait> {
    list: List<Step<R::Kind>>,
    parent: List<(R, Fingerprint)>,
    score: isize,
    state: (Fingerprint, usize),
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
        Self {
            list: List::default(),
            parent: parent.prepend((current, head)),
            score: 0,
            state: (Fingerprint(1), 0),
        }
    }

    pub fn pop_push(&self, rule: R) -> Self {
        let ((_, f), tail) = self.parent.slice().unwrap();
        let parent = tail.prepend((rule, *f));
        Self {
            parent,
            list: self.list.clone(),
            score: self.score,
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
            score: self.score,
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
            score: self.score,
            state: (*f, a),
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

pub fn a_star<R: ParserTrait>(root: R, tokens: &[FatToken<R::Kind>]) -> List<Step<R::Kind>> {
    let mut state = AStar::new(tokens);
    state.todo.push(Element::new(root));
    state.consume().unwrap_or_default()
}
