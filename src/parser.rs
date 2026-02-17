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
    tokens: List<FatToken>,
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
    Start(rowan::SyntaxKind, Option<rowan::Checkpoint>),
    Error(Error),
    End,
    Bump,
}
impl Step {
    pub fn start(kind: impl Into<rowan::SyntaxKind>) -> Self {
        Step::Start(kind.into(), None)
    }
    pub fn error(error: Error) -> Self {
        Self::Error(error)
    }
    pub fn start_at(kind: impl Into<rowan::SyntaxKind>, checkpoint: rowan::Checkpoint) -> Self {
        Step::Start(kind.into(), Some(checkpoint))
    }

    pub fn end() -> Self {
        Step::End
    }

    pub fn bump() -> Self {
        Step::Bump
    }

    pub fn apply(&self, parser: &mut Parser, builder: &mut GreenNodeBuilder<'_>) {
        match self {
            Step::Start(syntax_kind, None) => builder.start_node(*syntax_kind),
            Step::Start(syntax_kind, Some(x)) => builder.start_node_at(x.clone(), *syntax_kind),
            Step::End => builder.finish_node(),
            Step::Error(e) => {
                builder.start_node(testing::SyntaxKind::Error.into());
                parser.errors = parser.errors.prepend(format!("{:?}", e));
                builder.finish_node();
            }
            Step::Bump => {
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
    pub error_value: usize,
}

impl ParseRes {
    pub fn combine(&mut self, other: &ParseRes) {
        self.error_value += other.error_value;
        for x in other.steps.iter() {
            self.steps.prepend(x.clone());
        }
    }

    pub fn start_node_at(&mut self, checkpoint: rowan::Checkpoint, kind: rowan::SyntaxKind) {
        self.steps = self.steps.prepend(Step::start_at(kind, checkpoint));
    }

    pub fn finish_node(&mut self) {
        self.steps = self.steps.prepend(Step::end());
    }

    pub fn start_node(&mut self, kind: rowan::SyntaxKind) {
        self.steps = self.steps.prepend(Step::start(kind));
    }
}

impl Parser {
    pub fn parse_item<T: ParserTrait>(mut self) -> Parse {
        let tokens = self.tokens.clone();
        T::parse(&mut self, &mut Context {});
        println!("result {:#?}", self);
        self.eat_skips();

        self.tokens = tokens;
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(testing::SyntaxKind::ROOT.into());

        let steps: Vec<_> = self.res.steps.iter().cloned().collect();
        for step in steps.into_iter().rev() {
            println!("apply {:?}", step);
            step.apply(&mut self, &mut builder);
        }

        builder.finish_node();
        Parse {
            green_node: builder.finish(),
            errors: self.errors,
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

    pub fn expect_as<T: ParserTrait + std::fmt::Debug>(&mut self, kind: T) {
        let e = if let Some(c) = self.current() {
            println!(
                "{:?} {:?} !== {:?}",
                c,
                testing::SyntaxKind::try_from(c.kind),
                kind
            );
            if let Ok(c) = testing::SyntaxKind::try_from(c.kind) {
                if c == T::KIND {
                    println!("  Equals");
                    self.bump();
                    self.done = List::default();
                    return;
                }
            }

            self.res.steps = self
                .res
                .steps
                .prepend(Step::Error(Error::Expected(T::KIND)));
            self.res.error_value += 1;
        } else {
            self.res.error_value += 1;
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
            println!("Eat skips {:?}", token);
            self.res.steps = self.res.steps.prepend(Step::bump());
            self.tokens = self.tokens.tail().unwrap().clone();
        }
    }
}
