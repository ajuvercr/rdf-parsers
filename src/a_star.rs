use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
};

use crate::{Error, FatToken, Parse, Step, TokenTrait, list::List};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Fingerprint(u128);

fn descend(fp: Fingerprint, branch_id: u32) -> Fingerprint {
    const M: u128 = 0x9E3779B97F4A7C15_6A09E667F3BCC909;
    Fingerprint(fp.0.wrapping_mul(M) ^ (branch_id as u128).wrapping_add(0xD6E8FEB86659FD93))
}

pub trait ParserTrait: Debug + 'static {
    type Kind: 'static + TokenTrait;

    fn step(&self, el: &Element<Self::Kind>, state: &mut AStar<Self::Kind>);
    fn at(&self) -> usize;
}

pub trait ParserTraitConsts: ParserTrait {
    const ELEMENT: Self::Kind;

    fn new() -> Self;
}

pub struct AStar<'a, T: TokenTrait> {
    done: HashSet<(Fingerprint, usize, usize)>,
    tokens: &'a [FatToken<T>],
    todo: BinaryHeap<Element<T>>,
}

impl<'a, T: TokenTrait> AStar<'a, T> {
    fn new(tokens: &'a [FatToken<T>]) -> Self {
        Self {
            tokens,
            done: HashSet::new(),
            todo: BinaryHeap::new(),
        }
    }

    pub fn consume(&mut self) -> Option<List<Step<T>>> {
        if let Some(e) = self.todo.pop() {
            if e.state.1 == self.tokens.len() && e.parent.len() == 1 {
                return Some(e.list);
            }

            if let Some((head, _)) = e.parent.head() {
                println!(
                    "consuming {:?} {} {}/{}",
                    head,
                    e.score,
                    e.state.1,
                    self.tokens.len()
                );
                let state = (e.state.0, e.state.1, head.at());
                if self.done.contains(&state) {
                    return self.consume();
                }

                self.done.insert(state);
                head.step(&e, self);
            }

            None
        } else {
            panic!("Expected more")
        }
    }

    pub fn add_element(&mut self, element: Element<T>) -> bool {
        let at = element.parent.head().map(|x| x.0.at()).unwrap_or(0);
        let state = (element.state.0, element.state.1, at);

        if self.done.contains(&state) {
            return false;
        }

        self.todo.push(element);
        true
    }

    pub fn expect_as(&self, element: &Element<T>, token: T, error_value: isize) -> Element<T> {
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

type Rule<T> = Box<dyn ParserTrait<Kind = T>>;

#[derive(Debug)]
pub struct Element<T: TokenTrait> {
    list: List<Step<T>>,
    parent: List<(Rule<T>, Fingerprint)>,
    score: isize,
    state: (Fingerprint, usize),
}
impl<T: TokenTrait> PartialEq for Element<T> {
    fn eq(&self, other: &Self) -> bool {
        self.list == other.list && self.score == other.score && self.state == other.state
    }
}
impl<T: TokenTrait> Eq for Element<T> {}
impl<T: TokenTrait> Element<T> {
    fn new(current: Rule<T>) -> Self {
        let parent = List::default();
        let head = Fingerprint(0);
        Self {
            list: List::default(),
            parent: parent.prepend((current, head)),
            score: 0,
            state: (Fingerprint(1), 0),
        }
    }

    pub fn pop_push<P: ParserTraitConsts<Kind = T> + 'static>(&self, rule: P) -> Self {
        let ((_, f), tail) = self.parent.slice().unwrap();
        let parent = tail.prepend((Box::new(rule), *f));
        Self {
            parent,
            list: self.list.clone(),
            score: self.score,
            state: self.state.clone(),
        }
    }

    pub fn push<P: ParserTraitConsts<Kind = T> + 'static>(&self, rule: P) -> Self {
        let (s, a) = self.state;
        let parent = self.parent.prepend((Box::new(rule), s));
        let list = self.list.prepend(Step::start(P::ELEMENT));
        let s = descend(s, P::ELEMENT.branch());
        Self {
            parent,
            list,
            score: self.score,
            state: (s, a),
        }
    }

    pub fn pop(&self) -> Option<Self> {
        let ((_, f), parent) = self.parent.slice()?.clone();
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

impl<T: TokenTrait> Ord for Element<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score
            .cmp(&other.score)
            .then(self.parent.len().cmp(&other.parent.len()))
    }
}
impl<T: TokenTrait> PartialOrd for Element<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star<P: ParserTrait + 'static>(
    root: P,
    tokens: &[FatToken<P::Kind>],
) -> List<Step<P::Kind>> {
    let mut state = AStar::new(tokens);
    state.todo.push(Element::new(Box::new(root)));

    loop {
        if let Some(o) = state.consume() {
            return o;
        }
    }
}
