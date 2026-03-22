#[allow(unused)]
#[derive(Debug)]
pub struct FatToken<T: TokenTrait> {
    pub kind: T,
    text: String,
    pub range: Range<usize>,
    old_kind: Option<T>,
}
impl<T: TokenTrait> FatToken<T> {
    pub fn new(kind: T, range: Range<usize>, text: String) -> Self {
        FatToken {
            kind,
            range,
            text,
            old_kind: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn old_kind(&self) -> Option<&T> {
        self.old_kind.as_ref()
    }

    pub fn set_old_kind(&mut self, kind: Option<T>) {
        self.old_kind = kind;
    }
}

/// Collapse rule nodes that consumed no tokens into a single `Expected(RuleName)` error.
///
/// Only the outermost failing rule under a successfully-parsed ancestor is collapsed:
/// if a parent rule also has no bumps, it defers to the grandparent, and so on.
/// This prevents showing dozens of low-level "Expected(Iriref)" errors when an
/// entire `PredicateObjectList` is simply absent.
fn coalesce_empty_rules<T: crate::TokenTrait>(steps: Vec<Step<T>>) -> Vec<Step<T>> {
    let mut out: Vec<Step<T>> = Vec::with_capacity(steps.len());
    // Stack entry: (kind, start_idx_in_out, has_bump, has_error)
    let mut stack: Vec<(T, usize, bool, bool)> = Vec::new();

    for step in steps {
        match step {
            Step::Start(ref kind) => {
                stack.push((kind.clone(), out.len(), false, false));
                out.push(step);
            }
            Step::Bump => {
                for entry in &mut stack {
                    entry.2 = true;
                }
                out.push(step);
            }
            Step::Error(_) => {
                for entry in &mut stack {
                    entry.3 = true;
                }
                out.push(step);
            }
            Step::End => {
                let (kind, start_pos, has_bump, has_error) = stack.pop().expect("unmatched End");
                // Coalesce only when:
                // - this rule produced no real tokens (has_bump = false)
                // - it contains at least one error (has_error = true)
                // - its parent has real tokens (parent.has_bump = true), meaning
                //   the parent will NOT itself be coalesced at a higher level.
                let parent_has_bump = stack.last().map(|e| e.2).unwrap_or(false);
                if !has_bump && has_error && parent_has_bump {
                    out.truncate(start_pos);
                    out.push(Step::Start(kind.clone()));
                    out.push(Step::Error(Error::Expected(kind)));
                    out.push(Step::End);
                } else {
                    out.push(Step::End);
                }
            }
        }
    }
    out
}

pub struct Parse {
    pub green_node: GreenNode,
    pub errors: List<String>,
    pub suggestions: HashSet<(String, Range<usize>)>,
}
impl Parse {
    pub fn expected_at<L>(
        &self,
        byte_offset: usize,
        first_tokens: fn(L::Kind) -> &'static [L::Kind],
    ) -> Vec<L::Kind>
    where
        L: rowan::Language,
        L::Kind: Eq + std::hash::Hash + Copy,
    {
        use rowan::{TextSize, TokenAtOffset};
        let root = self.syntax::<L>();
        let offset = TextSize::new(byte_offset as u32);

        let (current_kind, start_node) = match root.token_at_offset(offset) {
            TokenAtOffset::None => (None, None),
            TokenAtOffset::Single(t) => (Some(t.kind()), t.parent()),
            TokenAtOffset::Between(_, right) => (Some(right.kind()), right.parent()),
        };

        let mut result = HashSet::new();
        let mut node = start_node;
        while let Some(n) = node {
            result.extend(first_tokens(n.kind()).iter().copied());
            node = n.parent();
        }

        if let Some(k) = current_kind {
            result.remove(&k);
        }

        result.into_iter().collect()
    }

    pub fn from_steps<T: crate::TokenTrait>(tokens: &[FatToken<T>], steps: List<Step<T>>) -> Self {
        let mut at = 0;
        let skip_white_with_builder = |builder: &mut GreenNodeBuilder<'_>, at: &mut usize| {
            while let Some(t) = tokens.get(*at)
                && t.kind.skips()
            {
                builder.token(t.kind.clone().into(), &t.text);
                *at += 1;
            }
        };

        let mut errors = List::default();
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(T::ROOT.into());
        let step_count = steps.len();
        let steps: Vec<_> = {
            let mut v = Vec::with_capacity(step_count);
            v.extend(steps.iter().cloned());
            v
        };
        let steps = coalesce_empty_rules(steps.into_iter().rev().collect());
        for step in steps.into_iter() {
            match step {
                Step::Start(kind) => {
                    skip_white_with_builder(&mut builder, &mut at);
                    builder.start_node(kind.into());
                }
                Step::End => builder.finish_node(),
                Step::Error(e) => {
                    builder.start_node(T::ERROR.into());
                    errors = errors.prepend(format!("{:?}", e));
                    builder.finish_node();
                }
                Step::Bump => {
                    skip_white_with_builder(&mut builder, &mut at);
                    if let Some(i) = tokens.get(at) {
                        builder.token(i.kind.clone().into(), &i.text);
                        at += 1;
                    }
                }
            }
        }
        skip_white_with_builder(&mut builder, &mut at);

        builder.finish_node();

        Parse {
            green_node: builder.finish(),
            errors: errors,
            suggestions: HashSet::new(),
        }
    }
}

impl Parse {
    pub fn syntax<L: Language>(&self) -> rowan::SyntaxNode<L> {
        rowan::SyntaxNode::new_root(self.green_node.clone())
    }
}

use std::{collections::HashSet, ops::Range};

use rowan::{GreenNode, GreenNodeBuilder, Language};

use crate::{Context, ParserTrait, TokenTrait, list::List};

/// Walk a rowan `SyntaxNode` tree and extract the grammar role for each token
/// by finding the innermost ancestor whose kind is "significant" (i.e.
/// `is_significant()` returns true).
///
/// Returns `Vec<Option<L::Kind>>` indexed by token position, aligned with
/// the token-vec produced by `tokenize`.
pub fn extract_prev_roles<L: rowan::Language>(
    root: &rowan::SyntaxNode<L>,
) -> Vec<Option<L::Kind>>
where
    L::Kind: crate::TokenTrait,
{
    let mut result = Vec::new();
    for token in root.descendants_with_tokens() {
        let rowan::NodeOrToken::Token(tok) = token else {
            continue;
        };
        // Walk up ancestors to find the innermost significant one.
        let role = tok
            .parent_ancestors()
            .find_map(|ancestor| {
                let k = ancestor.kind();
                if k.is_significant() { Some(k) } else { None }
            });
        result.push(role);
    }
    result
}

#[derive(Clone, Debug)]
pub struct Parser<T: TokenTrait> {
    /// input tokens, including whitespace,
    /// in *reverse* order.
    pub tokens: List<FatToken<T>>,
    pub done: List<T>,
    pub res: ParseRes<T>,
    suggesting: bool,
    /// the list of syntax errors we've accumulated
    /// so far.
    errors: List<String>,
}

impl<T: TokenTrait> Parser<T> {
    pub fn new(tokens: List<FatToken<T>>) -> Self {
        Parser {
            tokens,
            suggesting: true,
            res: ParseRes::<T>::default(),
            errors: List::default(),
            done: List::default(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error<T> {
    Expected(T),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step<T> {
    Start(T),
    Error(Error<T>),
    End,
    Bump,
}
impl<T: TokenTrait> Step<T> {
    pub fn start(kind: T) -> Self {
        Step::Start(kind)
    }
    pub fn error(error: Error<T>) -> Self {
        Self::Error(error)
    }

    pub fn end() -> Self {
        Step::End
    }

    pub fn bump() -> Self {
        Step::Bump
    }

    pub fn apply(&self, parser: &mut Parser<T>, builder: &mut GreenNodeBuilder<'_>) {
        match self {
            Step::Start(kind) => {
                parser.skip_white_with_builder(builder);
                builder.start_node(kind.clone().into());
            }
            Step::End => builder.finish_node(),
            Step::Error(e) => {
                builder.start_node(T::ERROR.into());
                parser.errors = parser.errors.prepend(format!("{:?}", e));
                builder.finish_node();
            }
            Step::Bump => {
                parser.skip_white_with_builder(builder);
                if let Some((i, r)) = parser.tokens.slice() {
                    builder.token(i.kind.clone().into(), &i.text);
                    parser.tokens = r.clone();
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParseRes<T: TokenTrait> {
    pub steps: List<Step<T>>,
    pub error_value: isize,
}

impl<T: TokenTrait> Default for ParseRes<T> {
    fn default() -> Self {
        Self {
            steps: Default::default(),
            error_value: Default::default(),
        }
    }
}

impl<T: TokenTrait> ParseRes<T> {
    pub fn combine(&mut self, other: &Self) {
        self.error_value += other.error_value;
        for x in other.steps.iter() {
            self.steps.prepend(x.clone());
        }
    }

    pub fn finish_node(&mut self) {
        self.steps = self.steps.prepend(Step::end());
    }

    pub fn start_node(&mut self, kind: T) {
        self.steps = self.steps.prepend(Step::start(kind));
    }
}

impl<T: TokenTrait> Parser<T> {
    fn skip_white_with_builder(&mut self, builder: &mut GreenNodeBuilder<'_>) {
        while let Some((i, r)) = self.tokens.slice()
            && self.skip_token(i)
        {
            builder.token(i.kind.clone().into(), &i.text);
            self.tokens = r.clone();
        }
    }

    pub fn parse_item<O: ParserTrait<Kind = T>>(mut self) -> Parse {
        let tokens = self.tokens.clone();
        let mut ctx = Context {
            suggestions: HashSet::new(),
        };
        O::parse(&mut self, &mut ctx);
        self.eat_skips();

        self.tokens = tokens;
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(T::ROOT.into());

        let steps: Vec<_> = self.res.steps.iter().cloned().collect();
        for step in steps.into_iter().rev() {
            step.apply(&mut self, &mut builder);
        }

        self.skip_white_with_builder(&mut builder);

        builder.finish_node();
        Parse {
            green_node: builder.finish(),
            errors: self.errors,
            suggestions: ctx.suggestions,
        }
    }

    pub fn producing_rule<P: ParserTrait<Kind = T>>(&mut self, imp: impl FnOnce(&mut Self) -> ()) {
        if self.already_done::<P>() {
            self.res.error_value += 10;
            return;
        }

        self.starting::<P>();

        self.res.start_node(P::KIND);
        let old_parser = self.clone();
        let on = old_parser.tokens.len();

        imp(self);

        if self.tokens.len() == on {
            let ev = self.res.error_value;
            *self = old_parser;
            self.res.steps = self
                .res
                .steps
                .prepend(crate::Step::Error(crate::Error::Expected(P::KIND)));
            self.res.error_value = ev;
        }
        self.res.finish_node();
    }

    pub fn star(&mut self, mut imp: impl FnMut(&mut Self) -> ()) {
        let mut checkpoint = self.clone();

        while {
            imp(self);
            self.consumed_since(&checkpoint)
        } {
            checkpoint = self.clone();
        }

        self.reset(&checkpoint);
    }

    pub fn plus(&mut self, mut imp: impl FnMut(&mut Self) -> ()) {
        imp(self);
        self.star(imp);
    }
    pub fn option(&mut self, imp: impl FnOnce(&mut Self) -> ()) {
        let check = self.clone();
        imp(self);

        if self.res.error_value > check.res.error_value {
            *self = check;
        }
    }

    pub fn starting<O: ParserTrait<Kind = T>>(&mut self) {
        self.done = self.done.prepend(O::KIND);
    }

    pub fn reset(&mut self, other: &Self) {
        *self = other.clone();
    }

    pub fn already_done<O: ParserTrait<Kind = T>>(&self) -> bool {
        let k = O::KIND;
        self.done.iter().any(|x| x == &k)
    }

    pub fn expect_as<O: ParserTrait<Kind = T> + std::fmt::Debug>(
        &mut self,
        error: isize,
        context: &mut Context,
    ) {
        let e = if let Some(c) = self.current() {
            if c.kind == O::KIND {
                self.bump();
                self.done = List::default();
                self.res.error_value -= error / 2 + 1;
                return;
            }

            if self.suggesting && self.res.error_value <= 0 {
                context
                    .suggestions
                    .insert((format!("{:?}", O::KIND), c.range.clone()));
            }
            self.suggesting = false;

            self.res.steps = self
                .res
                .steps
                .prepend(Step::Error(Error::Expected(O::KIND)));
            self.res.error_value += error;
        } else {
            self.res.steps = self
                .res
                .steps
                .prepend(Step::Error(Error::Expected(O::KIND)));
            self.res.error_value += error;
        };

        self.done = self.done.prepend(O::KIND);
        e
    }

    pub fn start_node(&mut self, kind: T) {
        self.res.start_node(kind);
    }
    pub fn finish_node(&mut self) {
        self.res.finish_node();
    }

    pub fn skip_token(&self, token: &FatToken<T>) -> bool {
        token.kind.skips()
    }

    /// Advance one token, adding it to the current branch of the tree builder.
    pub fn bump(&mut self) {
        self.eat_skips();
        self.res.steps = self.res.steps.prepend(Step::bump());
        self.tokens = self.tokens.tail().unwrap().clone();
    }
    pub fn is_empty(&self) -> bool {
        self.tokens.head().is_none()
    }

    /// Peek at the first unprocessed token that is relevant
    pub fn current(&self) -> Option<&FatToken<T>> {
        self.tokens.iter().skip_while(|t| self.skip_token(t)).next()
    }

    pub fn consumed_since(&self, checkpoint: &Self) -> bool {
        !self.tokens.same(&checkpoint.tokens)
    }

    pub fn eat_skips(&mut self) {
        while let Some(token) = self.tokens.head()
            && self.skip_token(token)
        {
            self.tokens = self.tokens.tail().unwrap().clone();
        }
    }
}

pub(crate) struct Checker<T: TokenTrait> {
    checkpoint: Parser<T>,
    error_value: isize,
    out: Option<Parser<T>>,
}

impl<T: TokenTrait> Checker<T> {
    pub fn new(parser: &Parser<T>) -> Self {
        Self {
            checkpoint: parser.clone(),
            error_value: parser.res.error_value,
            out: None,
        }
    }

    pub fn update(&mut self, parser: &mut Parser<T>) {
        if self.out.is_none() {
            self.out = Some(parser.clone());
            self.error_value = parser.res.error_value;
        } else {
            let o_error_value = parser.res.error_value;
            if parser.res.error_value < self.error_value {
                self.out = Some(parser.clone());
                self.error_value = o_error_value;
            }
        }
        parser.reset(&self.checkpoint);
    }

    pub fn get(self) -> Parser<T> {
        self.out.unwrap_or(self.checkpoint)
    }
}
