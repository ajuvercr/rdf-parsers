#[allow(unused)]
#[derive(Debug)]
pub struct FatToken<T: TokenTrait> {
    pub kind: T,
    text: String,
    pub range: Range<usize>,
    old_kind: Option<TermType>,
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

    pub fn old_kind(&self) -> Option<TermType> {
        self.old_kind
    }

    pub fn set_old_kind(&mut self, kind: Option<TermType>) {
        self.old_kind = kind;
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TermType {
    Subject,
    Predicate,
    Object,
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
        for step in steps.into_iter().rev() {
            match step {
                Step::Start(syntax_kind) => {
                    skip_white_with_builder(&mut builder, &mut at);
                    builder.start_node(syntax_kind);
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

/// Walk a rowan `SyntaxNode` tree and extract the `TermType` for each
/// non-whitespace/non-error token by finding the innermost ancestor whose
/// kind maps to a `TermType` via `term_type_of`.
///
/// Returns `Vec<Option<TermType>>` indexed by token position (skipping
/// whitespace/error tokens to match the token-vec indices used by the A*
/// parser).
pub fn extract_term_types<L: rowan::Language>(
    root: &rowan::SyntaxNode<L>,
    term_type_of: impl Fn(L::Kind) -> Option<TermType>,
) -> Vec<Option<TermType>>
where
    L::Kind: Into<rowan::SyntaxKind>,
{
    let mut result = Vec::new();
    for token in root.descendants_with_tokens() {
        let rowan::NodeOrToken::Token(tok) = token else {
            continue;
        };
        // Walk up ancestors to find the innermost one with a TermType.
        let tt = tok
            .parent_ancestors()
            .find_map(|ancestor| term_type_of(ancestor.kind()));
        result.push(tt);
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
    Start(rowan::SyntaxKind),
    Error(Error<T>),
    End,
    Bump,
}
impl<T: TokenTrait> Step<T> {
    pub fn start(kind: impl Into<rowan::SyntaxKind>) -> Self {
        Step::Start(kind.into())
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
            Step::Start(syntax_kind) => {
                parser.skip_white_with_builder(builder);
                builder.start_node(*syntax_kind);
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

    pub fn start_node(&mut self, kind: rowan::SyntaxKind) {
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

        self.res.start_node(P::KIND.into());
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

    pub fn start_node<K: Into<rowan::SyntaxKind>>(&mut self, kind: K) {
        self.res.start_node(kind.into());
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
