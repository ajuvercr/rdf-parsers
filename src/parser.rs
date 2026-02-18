#[allow(unused)]
#[derive(Debug)]
pub struct FatToken {
    kind: Token,
    text: String,
    old_kind: Option<TermType>,
}
impl FatToken {
    pub fn new(kind: Token, text: String) -> Self {
        FatToken {
            kind,
            text,
            old_kind: None,
        }
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
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<Lang>;

#[allow(unused)]
pub type SyntaxToken = rowan::SyntaxToken<Lang>;

#[allow(unused)]
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

use rowan::{GreenNode, GreenNodeBuilder};

use crate::{Context, Lang, ParserTrait, Token, list::List, testing};

#[derive(Clone, Debug)]
pub struct Parser {
    /// input tokens, including whitespace,
    /// in *reverse* order.
    pub tokens: List<FatToken>,
    pub done: List<testing::SyntaxKind>,
    pub res: ParseRes,
    /// the list of syntax errors we've accumulated
    /// so far.
    errors: List<String>,
}

impl Parser {
    pub fn new(tokens: List<FatToken>) -> Self {
        Parser {
            tokens,
            res: ParseRes::default(),
            errors: List::default(),
            done: List::default(),
        }
    }
}
#[derive(Clone, Debug)]
pub enum Error {
    Expected(testing::SyntaxKind),
}

#[derive(Clone, Debug)]
pub enum Step {
    Start(rowan::SyntaxKind),
    Error(Error),
    End,
    Bump,
}
impl Step {
    pub fn start(kind: impl Into<rowan::SyntaxKind>) -> Self {
        Step::Start(kind.into())
    }
    pub fn error(error: Error) -> Self {
        Self::Error(error)
    }

    pub fn end() -> Self {
        Step::End
    }

    pub fn bump() -> Self {
        Step::Bump
    }

    pub fn apply(&self, parser: &mut Parser, builder: &mut GreenNodeBuilder<'_>) {
        match self {
            Step::Start(syntax_kind) => {
                parser.skip_white_with_builder(builder);
                builder.start_node(*syntax_kind);
            }
            Step::End => builder.finish_node(),
            Step::Error(e) => {
                builder.start_node(testing::SyntaxKind::Error.into());
                parser.errors = parser.errors.prepend(format!("{:?}", e));
                builder.finish_node();
            }
            Step::Bump => {
                parser.skip_white_with_builder(builder);
                if let Some((i, r)) = parser.tokens.slice() {
                    if let Ok(r) = testing::SyntaxKind::try_from(i.kind) {
                        builder.token(r.into(), &i.text);
                    } else {
                        builder.token(testing::SyntaxKind::Error.into(), &i.text);
                    }
                    parser.tokens = r.clone();
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParseRes {
    pub steps: List<Step>,
    pub error_value: isize,
}

impl ParseRes {
    pub fn combine(&mut self, other: &ParseRes) {
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

impl Parser {
    fn skip_white_with_builder(&mut self, builder: &mut GreenNodeBuilder<'_>) {
        while let Some((i, r)) = self.tokens.slice()
            && self.skip_token(i)
        {
            if let Ok(r) = testing::SyntaxKind::try_from(i.kind) {
                builder.token(r.into(), &i.text);
            } else {
                builder.token(testing::SyntaxKind::Error.into(), &i.text);
            }
            self.tokens = r.clone();
        }
    }

    pub fn parse_item<T: ParserTrait>(mut self) -> Parse {
        let tokens = self.tokens.clone();
        T::parse(&mut self, &mut Context {});
        self.eat_skips();

        self.tokens = tokens;
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(testing::SyntaxKind::ROOT.into());

        let steps: Vec<_> = self.res.steps.iter().cloned().collect();
        for step in steps.into_iter().rev() {
            step.apply(&mut self, &mut builder);
        }

        self.skip_white_with_builder(&mut builder);

        builder.finish_node();
        Parse {
            green_node: builder.finish(),
            errors: self.errors,
        }
    }

    pub fn producing_rule<P: ParserTrait>(&mut self, imp: impl FnOnce(&mut Parser) -> ()) {
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

    pub fn star(&mut self, mut imp: impl FnMut(&mut Parser) -> ()) {
        let mut checkpoint = self.clone();

        while {
            imp(self);
            self.consumed_since(&checkpoint)
        } {
            checkpoint = self.clone();
        }
        self.reset(&checkpoint);
    }

    pub fn plus(&mut self, mut imp: impl FnMut(&mut Parser) -> ()) {
        imp(self);
        self.star(imp);
    }
    pub fn option(&mut self, imp: impl FnOnce(&mut Parser) -> ()) {
        let check = self.clone();
        imp(self);

        if self.res.error_value > check.res.error_value {
            *self = check;
        }
    }

    pub fn peek(&self, kind: Token) -> bool {
        if let Some(c) = self.current() {
            if c.kind == kind {
                return true;
            }
        }
        return false;
    }

    pub fn peek_as<T: TryFrom<Token> + PartialEq>(&self, kind: T) -> bool {
        if let Some(c) = self.current() {
            if let Ok(c) = T::try_from(c.kind) {
                if c == kind {
                    return true;
                }
            }
        }
        false
    }

    pub fn expect(&mut self, kind: Token) {
        if let Some(c) = self.current() {
            if c.kind == kind {
                self.bump();
                return;
            }
        }

        self.start_node(testing::SyntaxKind::Error);
        self.errors = self.errors.prepend(format!("Expected {:?}", kind));
        self.finish_node();
    }

    pub fn starting<T: ParserTrait>(&mut self) {
        self.done = self.done.prepend(T::KIND);
    }

    pub fn reset(&mut self, other: &Self) {
        *self = other.clone();
    }

    pub fn already_done<T: ParserTrait>(&self) -> bool {
        let k = T::KIND;
        self.done.iter().any(|x| x == &k)
    }

    pub fn expect_as<T: ParserTrait + std::fmt::Debug>(&mut self, error: isize) {
        let e = if let Some(c) = self.current() {
            if let Ok(c) = testing::SyntaxKind::try_from(c.kind) {
                if c == T::KIND {
                    self.bump();
                    self.done = List::default();
                    self.res.error_value -= error / 2;
                    return;
                }
            }

            self.res.steps = self
                .res
                .steps
                .prepend(Step::Error(Error::Expected(T::KIND)));
            self.res.error_value += error;
        } else {
            self.res.steps = self
                .res
                .steps
                .prepend(Step::Error(Error::Expected(T::KIND)));
            self.res.error_value += error;
        };

        self.done = self.done.prepend(T::KIND);
        e
    }

    pub fn start_node<K: Into<rowan::SyntaxKind>>(&mut self, kind: K) {
        self.res.start_node(kind.into());
    }
    pub fn finish_node(&mut self) {
        self.res.finish_node();
    }

    pub fn skip_token(&self, token: &FatToken) -> bool {
        match token.kind {
            Token::WHITESPACE => true,
            Token::COMMENT => true,
            _ => false,
        }
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
    pub fn current(&self) -> Option<&FatToken> {
        self.tokens.iter().skip_while(|t| self.skip_token(t)).next()
    }

    pub fn consumed_since(&self, checkpoint: &Self) -> bool {
        !self.tokens.same(&checkpoint.tokens)
    }

    pub fn eat_skips(&mut self) {
        while let Some(token) = self.tokens.head()
            && self.skip_token(token)
        {
            // self.res.steps = self.res.steps.prepend(Step::bump());
            self.tokens = self.tokens.tail().unwrap().clone();
        }
    }
}

pub(crate) struct Checker {
    checkpoint: Parser,
    error_value: isize,
    out: Option<Parser>,
}

impl Checker {
    pub fn new(parser: &Parser) -> Self {
        Self {
            checkpoint: parser.clone(),
            error_value: parser.res.error_value,
            out: None,
        }
    }

    pub fn update(&mut self, parser: &mut Parser) {
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

    pub fn get(self) -> Parser {
        self.out.unwrap_or(self.checkpoint)
    }
}
