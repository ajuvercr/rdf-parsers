use rowan::GreenNode;
use rowan::GreenNodeBuilder;

mod impls;
mod list;
pub trait ParserTrait {
    const KIND: testing::SyntaxKind;
    fn parse(parser: &mut crate::Parser, context: &mut Context) -> crate::ParseRes;
}
pub struct Context {}

pub mod testing {
    use xtask::include_path_code;

    include_path_code!("./xtask/turtle.txt");

    impl From<SyntaxKind> for rowan::SyntaxKind {
        fn from(kind: SyntaxKind) -> Self {
            Self(kind as u16)
        }
    }

    impl From<rowan::SyntaxKind> for SyntaxKind {
        fn from(value: rowan::SyntaxKind) -> Self {
            assert!(value.0 <= SyntaxKind::ROOT as u16);
            unsafe { std::mem::transmute::<u16, SyntaxKind>(value.0) }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum Token {
    L_SQ = 0,   // '['
    R_SQ,       // ']'
    STOP,       // '.'
    SEMI,       // ';'
    COMMA,      // ','
    TERM,       // '+', '15'
    WHITESPACE, // whitespaces is explicit
    COMMENT,    // comment is explicit
    ERROR,      // as well as errors
}
use Token::*;

impl TryFrom<Token> for testing::SyntaxKind {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, ()> {
        let o = match value {
            L_SQ => testing::SyntaxKind::SqOpen,
            R_SQ => testing::SyntaxKind::SqClose,
            STOP => testing::SyntaxKind::Stop,
            SEMI => testing::SyntaxKind::Colon,
            COMMA => testing::SyntaxKind::Comma,
            TERM => testing::SyntaxKind::Iriref,
            COMMENT => testing::SyntaxKind::Comment,
            WHITESPACE => testing::SyntaxKind::WhiteSpace,
            _ => return Err(()),
        };
        Ok(o)
    }
}

pub struct Parse {
    pub green_node: GreenNode,
    pub errors: List<String>,
}

pub type SyntaxNode = rowan::SyntaxNode<Lang>;

#[allow(unused)]
pub type SyntaxToken = rowan::SyntaxToken<Lang>;

#[allow(unused)]
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

/// Second, implementing the `Language` trait teaches rowan to convert between
/// these two SyntaxKind types, allowing for a nicer SyntaxNode API where
/// "kinds" are values from our `enum SyntaxKind`, instead of plain u16 values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}
impl rowan::Language for Lang {
    type Kind = testing::SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= testing::SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, testing::SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

#[derive(Debug)]
pub struct FatToken {
    kind: Token,
    text: String,
    old_kind: Option<TermType>,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TermType {
    Subject,
    Predicate,
    Object,
}
// impl From<TermType> for rowan::SyntaxKind {
//     fn from(value: TermType) -> Self {
//         match value {
//             TermType::Subject => SUBJECT.into(),
//             TermType::Predicate => PREDICATE.into(),
//             TermType::Object => OBJECT.into(),
//         }
//     }
// }

mod parser_impl {
    use crate::list::List;

    use super::*;

    pub struct Checkpoint {
        tokens: List<FatToken>,
        errors: List<String>,
        done: List<testing::SyntaxKind>,
    }

    pub struct Parser {
        /// input tokens, including whitespace,
        /// in *reverse* order.
        tokens: List<FatToken>,
        pub done: List<testing::SyntaxKind>,
        /// the in-progress tree.
        pub builder: GreenNodeBuilder<'static>,
        /// the list of syntax errors we've accumulated
        /// so far.
        errors: List<String>,
    }

    impl Parser {
        pub fn checkpoint(&self) -> Checkpoint {
            Checkpoint {
                tokens: self.tokens.clone(),
                errors: self.errors.clone(),
                done: self.done.clone(),
            }
        }

        pub fn reset(&mut self, checkout: &Checkpoint) {
            self.errors = checkout.errors.clone();
            self.tokens = checkout.tokens.clone();
            self.done = checkout.done.clone();
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum ParseRes {
        Ok,
        IncorrectTermType(TermType),
        Unexpected(Token),
        Loop,
        Eof,
    }
    impl ParseRes {
        pub fn combine_or(self, other: ParseRes) -> ParseRes {
            match (self, other) {
                (ParseRes::Ok, _) => ParseRes::Ok,
                (_, ParseRes::Ok) => ParseRes::Ok,
                (ParseRes::Eof, t) => t,
                (x, ParseRes::Eof) => x,
                (ParseRes::IncorrectTermType(t), _) => ParseRes::IncorrectTermType(t),
                (_, ParseRes::IncorrectTermType(term_type)) => {
                    ParseRes::IncorrectTermType(term_type)
                }
                (ParseRes::Unexpected(syntax_kind), _) => ParseRes::Unexpected(syntax_kind),
                (x, _) => x,
            }
        }
        pub fn combine_and(self, other: ParseRes) -> ParseRes {
            match (self, other) {
                (x, ParseRes::Ok) => x,
                (_, ParseRes::Eof) => ParseRes::Eof,
                (_, x) => x,
            }
        }
        pub fn is_unexpected(&self) -> bool {
            match self {
                Self::Unexpected(_) => true,
                _ => false,
            }
        }
    }

    impl Parser {
        pub fn parse_item<T: ParserTrait>(mut self) -> Parse {
            self.builder.start_node(testing::SyntaxKind::ROOT.into());
            let o = T::parse(&mut self, &mut Context {});
            println!("result {:?}", o);
            self.eat_skips();
            self.builder.finish_node();
            Parse {
                green_node: self.builder.finish(),
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

        pub fn already_done<T: ParserTrait>(&self) -> bool {
            let k = T::KIND;
            self.done.iter().any(|x| x == &k)
        }

        pub fn expect_as<T: ParserTrait + std::fmt::Debug>(&mut self, kind: T) -> ParseRes {
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
                        return ParseRes::Ok;
                    }
                }

                ParseRes::Unexpected(c.kind)
            } else {
                ParseRes::Eof
            };

            self.done = self.done.prepend(T::KIND);

            // self.start_node(testing::SyntaxKind::Error);
            // self.errors = self.errors.prepend(format!("Expected {:?}", kind));
            // self.finish_node();
            e
        }

        pub fn start_node<K: Into<rowan::SyntaxKind>>(&mut self, kind: K) {
            self.builder.start_node(kind.into());
        }
        pub fn finish_node(&mut self) {
            self.builder.finish_node();
        }

        pub fn skip_token(&self, token: &FatToken) -> bool {
            match token.kind {
                WHITESPACE => true,
                COMMENT => true,
                _ => false,
            }
        }

        /// Advance one token, adding it to the current branch of the tree builder.
        pub fn bump(&mut self) {
            self.eat_skips();
            let token = self.tokens.head().unwrap();
            if let Ok(t) = testing::SyntaxKind::try_from(token.kind) {
                self.builder.token(t.into(), token.text.as_str());
            } else {
                eprintln!("Failed to build token for {:?}", token);
            }
            self.tokens = self.tokens.tail().unwrap().clone();
        }

        /// Peek at the first unprocessed token that is relevant
        pub fn current(&self) -> Option<&FatToken> {
            self.tokens.iter().skip_while(|t| self.skip_token(t)).next()
        }

        pub fn consumed_since(&self, checkpoint: &Checkpoint) -> bool {
            self.tokens.same(&checkpoint.tokens)
        }

        pub fn eat_skips(&mut self) {
            while let Some(token) = self.tokens.head()
                && self.skip_token(token)
            {
                println!("Eat skips {:?}", token);

                if let Ok(t) = testing::SyntaxKind::try_from(token.kind) {
                    self.builder.token(t.into(), token.text.as_str());
                } else {
                    eprintln!("Failed to build token for {:?}", token);
                }

                self.tokens = self.tokens.tail().unwrap().clone();
            }
        }
    }

    // pub fn parse(text: &str) -> Parse {
    //     let mut tokens = lex(text);
    //     println!("Tokens {:?}", tokens);
    //     tokens.reverse();
    //     Parser {
    //         tokens,
    //         builder: GreenNodeBuilder::new(),
    //         errors: Vec::new(),
    //     }
    //     .parse()
    // }

    pub fn parse_t<T: ParserTrait>(text: &str) -> Parse {
        let tokens = lex(text);
        println!("Tokens {:?}", tokens);
        Parser {
            tokens,
            builder: GreenNodeBuilder::new(),
            errors: List::default(),
            done: List::default(),
        }
        .parse_item::<T>()
    }
}

pub use parser_impl::*;

use crate::list::Inner;
use crate::list::List;
/// Split the input string into a flat list of tokens
/// (such as L_PAREN, WORD, and WHITESPACE)
fn lex(text: &str) -> List<FatToken> {
    fn tok(t: Token) -> m_lexer::TokenKind {
        let i = t as u16;
        m_lexer::TokenKind(i)
    }
    fn kind(t: m_lexer::TokenKind) -> Token {
        unsafe { std::mem::transmute::<u16, Token>(t.0) }
    }

    let lexer = m_lexer::LexerBuilder::new()
        .error_token(tok(ERROR))
        .tokens(&[
            (tok(L_SQ), r"\["),
            (tok(R_SQ), r"\]"),
            (tok(STOP), r"\."),
            (tok(SEMI), r"\;"),
            (tok(COMMA), r"\,"),
            (tok(TERM), r"[a-zA-Z]+"),
            (tok(WHITESPACE), r"\s+"),
            (tok(COMMENT), r"#[^\n]*\n"),
        ])
        .build();

    let vec: Vec<_> = lexer
        .tokenize(text)
        .into_iter()
        .map(|t| (t.len, kind(t.kind)))
        .scan(0usize, |start_offset, (len, kind)| {
            let s: String = text[*start_offset..*start_offset + len].into();
            *start_offset += len;
            Some(FatToken {
                kind,
                text: s,
                old_kind: None,
            })
        })
        .collect();

    vec.into_iter().rfold(Inner::new(), |acc, b| acc.prepend(b))
}
