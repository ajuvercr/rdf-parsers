use crate::list::Inner;
use crate::list::List;
use Token::*;
pub use parser::*;

mod impls;
mod list;
mod parser;

pub trait ParserTrait {
    const KIND: testing::SyntaxKind;
    fn parse(parser: &mut crate::Parser, context: &mut Context);
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
            Some(FatToken::new(kind, s))
        })
        .collect();

    vec.into_iter().rfold(Inner::new(), |acc, b| acc.prepend(b))
}

pub fn parse_t<T: ParserTrait>(text: &str) -> Parse {
    let tokens = lex(text);
    println!("Tokens {:?}", tokens);
    Parser::new(tokens).parse_item::<T>()
}
