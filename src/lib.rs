use rowan::GreenNode;
use rowan::GreenNodeBuilder;

mod impls;
pub trait ParserTrait {
    fn parse(parser: &mut crate::Parser, context: &mut Context) -> crate::ParseRes;
}
pub struct Context {}

impl Context {
    pub fn checkpoint(&mut self) -> usize {
        0
    }

    pub fn reset(&mut self, checkpoint: usize) {}
}

mod testing {
    use xtask::include_path_code;

    use crate::SyntaxKind as Sk;
    include_path_code!("./xtask/turtle.txt");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    L_SQ = 0,   // '['
    R_SQ,       // ']'
    STOP,       // '.'
    SEMI,       // ';'
    COMMA,      // ','
    TERM,       // '+', '15'
    WHITESPACE, // whitespaces is explicit
    COMMENT,    // comment is explicit
    ERROR,      // as well as errors

    // composite nodes
    TRIPLE,
    SUBJECT,
    PREDICATE,
    OBJECT,
    BN_LIST, // `[1 2;  3 4]`
    PO,      // 1 2
    PO_LIST, // 1 2; 3 4
    O_LIST,  // `1, 2, 3`
    ATOM,    // `+`, `15`, wraps a WORD token
    ROOT,    // top-level node: a list of s-expressions
}
use SyntaxKind::*;

mod parser;
pub use parser::*;

pub struct Parse {
    pub green_node: GreenNode,
    pub errors: Vec<String>,
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

/// Some boilerplate is needed, as rowan settled on using its own
/// `struct SyntaxKind(u16)` internally, instead of accepting the
/// user's `enum SyntaxKind` as a type parameter.
///
/// First, to easily pass the enum variants into rowan via `.into()`:
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

/// Second, implementing the `Language` trait teaches rowan to convert between
/// these two SyntaxKind types, allowing for a nicer SyntaxNode API where
/// "kinds" are values from our `enum SyntaxKind`, instead of plain u16 values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

#[derive(Debug)]
struct Token {
    kind: SyntaxKind,
    text: String,
    old_kind: Option<TermType>,
}

#[derive(PartialEq, Eq, Debug)]
enum TermType {
    Subject,
    Predicate,
    Object,
}
impl From<TermType> for rowan::SyntaxKind {
    fn from(value: TermType) -> Self {
        match value {
            TermType::Subject => SUBJECT.into(),
            TermType::Predicate => PREDICATE.into(),
            TermType::Object => OBJECT.into(),
        }
    }
}

mod parser_impl {
    use rowan::Checkpoint;

    use super::*;

    pub struct Parser {
        /// input tokens, including whitespace,
        /// in *reverse* order.
        tokens: Vec<Token>,
        /// the in-progress tree.
        builder: GreenNodeBuilder<'static>,
        /// the list of syntax errors we've accumulated
        /// so far.
        errors: Vec<String>,
    }

    #[derive(PartialEq, Eq)]
    pub enum ParseRes {
        Ok,
        IncorrectTermType(TermType),
        Unexpected(SyntaxKind),
        Eof,
    }
    impl ParseRes {
        pub fn combine(&mut self, other: ParseRes) {}
    }

    impl Parser {
        fn parse(mut self) -> Parse {
            self.builder.start_node(ROOT.into());

            loop {
                match self.triple() {
                    ParseRes::Ok => (),
                    ParseRes::Unexpected(_) => {
                        self.start_node(ERROR);
                        self.errors.push("some error".to_string());
                        self.bump(); // be sure to chug along in case of error
                        self.builder.finish_node();
                    }
                    ParseRes::IncorrectTermType(_) => {
                        // I think this cannot happen
                    }
                    ParseRes::Eof => break,
                }
            }

            // Don't forget to eat *trailing* whitespace
            self.eat_skips();
            // Close the root node.
            self.builder.finish_node();

            // Turn the builder into a GreenNode
            Parse {
                green_node: self.builder.finish(),
                errors: self.errors,
            }
        }

        fn triple(&mut self) -> ParseRes {
            let checkpoint = self.builder.checkpoint();

            let subject_parsed = self.term(TermType::Subject);
            if subject_parsed == ParseRes::Ok {
                self.builder.start_node_at(checkpoint, TRIPLE.into());
                let out = match self.po_list() {
                    ParseRes::IncorrectTermType(_) => {
                        self.finish_node();
                        return ParseRes::Ok;
                    }
                    ParseRes::Unexpected(STOP) => {
                        self.bump();
                        self.finish_node();
                        ParseRes::Ok
                    }
                    x => return x,
                };
            }

            let res = self.current().map(|x| x.kind);

            if let Some(unexpected) = res {
                if unexpected == STOP {
                    self.builder.start_node_at(checkpoint, TRIPLE.into());
                    self.bump();
                    self.finish_node();
                    ParseRes::Ok
                } else {
                    ParseRes::Unexpected(unexpected)
                }
            } else {
                ParseRes::Eof
            }
        }

        fn term(&mut self, term_kind: TermType) -> ParseRes {
            let kind = self.current().map(|x| x.kind);
            let out = match kind {
                Some(L_SQ) => {
                    self.start_node(term_kind);
                    let out = self.bnode();
                    self.finish_node();
                    out
                }
                Some(TERM) => {
                    self.start_node(term_kind);
                    self.start_node(ATOM);
                    self.bump();
                    self.finish_node();
                    self.finish_node();
                    ParseRes::Ok
                }
                Some(x) => ParseRes::Unexpected(x),
                None => ParseRes::Eof,
            };
            out
        }

        fn bnode(&mut self) -> ParseRes {
            self.start_node(BN_LIST);
            self.bump(); // L_SQ
            let out = match self.po_list() {
                ParseRes::Unexpected(R_SQ) => {
                    self.bump();
                    ParseRes::Ok
                }
                x => x,
            };
            self.finish_node();
            out
        }

        fn po_list(&mut self) -> ParseRes {
            self.start_node(PO_LIST);
            loop {
                let out = match self.po() {
                    ParseRes::Unexpected(SEMI)
                    | ParseRes::IncorrectTermType(TermType::Predicate) => {
                        self.bump();
                        continue;
                    }
                    x => x,
                };
                self.finish_node();
                return out;
            }
        }

        fn po(&mut self) -> ParseRes {
            self.start_node(PO);
            let predicate = self.term(TermType::Predicate);
            match predicate {
                ParseRes::Eof => return ParseRes::Eof,
                _ => (),
            }
            let objects = self.o_list();
            self.finish_node();
            return objects;
        }

        fn o_list(&mut self) -> ParseRes {
            let checkpoint = self.builder.checkpoint();
            let mut started = false;
            loop {
                if started {
                    self.expect(COMMA);
                }
                let t = self.term(TermType::Object);
                let out = match t {
                    ParseRes::Ok => {
                        if !started {
                            self.builder.start_node_at(checkpoint, O_LIST.into());
                            started = true;
                        }

                        if !(self.peek(SEMI) || self.peek(STOP) || self.peek(R_SQ)) {
                            continue;
                        }

                        ParseRes::Ok
                    }
                    ParseRes::Unexpected(COMMA) => {
                        if !started {
                            self.builder.start_node_at(checkpoint, O_LIST.into());
                            started = true;
                        }
                        continue;
                    }
                    x => x,
                };
                if started {
                    self.finish_node();
                }
                return out;
            }
        }

        fn peek(&self, kind: SyntaxKind) -> bool {
            if let Some(c) = self.current() {
                if c.kind == kind {
                    return true;
                }
            }
            return false;
        }
        fn expect(&mut self, kind: SyntaxKind) {
            if let Some(c) = self.current() {
                if c.kind == kind {
                    self.bump();
                    return;
                }
            }

            self.start_node(ERROR);
            self.errors.push(format!("Expected {:?}", kind));
            self.finish_node();
        }

        fn start_node<K: Into<rowan::SyntaxKind>>(&mut self, kind: K) {
            self.builder.start_node(kind.into());
        }
        fn finish_node(&mut self) {
            self.builder.finish_node();
        }

        fn skip_token(&self, token: &Token) -> bool {
            match token.kind {
                WHITESPACE => true,
                COMMENT => true,
                _ => false,
            }
        }

        /// Advance one token, adding it to the current branch of the tree builder.
        fn bump(&mut self) {
            self.eat_skips();
            let token = self.tokens.pop().unwrap();
            self.builder.token(token.kind.into(), token.text.as_str());
        }

        /// Peek at the first unprocessed token that is relevant
        fn current(&self) -> Option<&Token> {
            self.tokens
                .iter()
                .rev()
                .skip_while(|t| self.skip_token(t))
                .next()
        }

        fn eat_skips(&mut self) {
            while let Some(t) = self.tokens.last()
                && self.skip_token(t)
            {
                let token = self.tokens.pop().unwrap();
                self.builder.token(token.kind.into(), token.text.as_str());
            }
        }
    }

    pub fn parse(text: &str) -> Parse {
        let mut tokens = lex(text);
        println!("Tokens {:?}", tokens);
        tokens.reverse();
        Parser {
            tokens,
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
        .parse()
    }
}

pub use parser_impl::*;
/// Split the input string into a flat list of tokens
/// (such as L_PAREN, WORD, and WHITESPACE)
fn lex(text: &str) -> Vec<Token> {
    fn tok(t: SyntaxKind) -> m_lexer::TokenKind {
        m_lexer::TokenKind(rowan::SyntaxKind::from(t).0)
    }
    fn kind(t: m_lexer::TokenKind) -> SyntaxKind {
        match t.0 {
            0 => L_SQ,
            1 => R_SQ,
            2 => STOP,
            3 => SEMI,
            4 => COMMA,
            5 => TERM,
            6 => WHITESPACE,
            7 => COMMENT,
            8 => ERROR,
            _ => unreachable!(),
        }
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

    lexer
        .tokenize(text)
        .into_iter()
        .map(|t| (t.len, kind(t.kind)))
        .scan(0usize, |start_offset, (len, kind)| {
            let s: String = text[*start_offset..*start_offset + len].into();
            *start_offset += len;
            Some(Token {
                kind,
                text: s,
                old_kind: None,
            })
        })
        .collect()
}
