use crate::TokenTrait;
pub type SyntaxNode = rowan::SyntaxNode<Lang>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, logos :: Logos)]
# [logos (subpattern HEX = r#"([0-9]|[A-F]|[a-f])"#)]
# [logos (subpattern UCHAR = r#"(\\u(?&HEX)(?&HEX)(?&HEX)(?&HEX)|\\U(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX))"#)]
# [logos (subpattern IRIREF = r#"<(([^\x{00}-\x{20}<>"{}|^`\\]|(?&UCHAR)))*>"#)]
# [logos (subpattern PN_CHARS_BASE = r#"([A-Z]|[a-z]|[\x{00C0}-\x{00D6}]|[\x{00D8}-\x{00F6}]|[\x{00F8}-\x{02FF}]|[\x{0370}-\x{037D}]|[\x{037F}-\x{1FFF}]|[\x{200C}-\x{200D}]|[\x{2070}-\x{218F}]|[\x{2C00}-\x{2FEF}]|[\x{3001}-\x{D7FF}]|[\x{F900}-\x{FDCF}]|[\x{FDF0}-\x{FFFD}]|[\x{10000}-\x{EFFFF}])"#)]
# [logos (subpattern PN_CHARS_U = r#"((?&PN_CHARS_BASE)|_)"#)]
# [logos (subpattern PN_CHARS = r#"((?&PN_CHARS_U)|-|[0-9]|\u00B7|[\x{0300}-\x{036F}]|[\x{203F}-\x{2040}])"#)]
# [logos (subpattern PN_PREFIX = r#"(?&PN_CHARS_BASE)((((?&PN_CHARS)|\.))*(?&PN_CHARS))?"#)]
# [logos (subpattern PNAME_NS = r#"((?&PN_PREFIX))?:"#)]
# [logos (subpattern PERCENT = r#"%(?&HEX)(?&HEX)"#)]
# [logos (subpattern PN_LOCAL_ESC = r#"\\(_|~|\.|-|!|\$|&|'|\(|\)|\*|\+|,|;|=|/|\?|#|@|%)"#)]
# [logos (subpattern PLX = r#"((?&PERCENT)|(?&PN_LOCAL_ESC))"#)]
# [logos (subpattern PN_LOCAL = r#"((?&PN_CHARS_U)|:|[0-9]|(?&PLX))((((?&PN_CHARS)|\.|:|(?&PLX)))*((?&PN_CHARS)|:|(?&PLX)))?"#)]
# [logos (subpattern PNAME_LN = r#"(?&PNAME_NS)(?&PN_LOCAL)"#)]
# [logos (subpattern BLANK_NODE_LABEL = r#"_:((?&PN_CHARS_U)|[0-9])((((?&PN_CHARS)|\.))*(?&PN_CHARS))?"#)]
# [logos (subpattern LANGTAG = r#"@([a-zA-Z])+(-([a-zA-Z0-9])+)*"#)]
# [logos (subpattern INTEGER = r#"([+-])?([0-9])+"#)]
# [logos (subpattern DECIMAL = r#"([+-])?([0-9])*\.([0-9])+"#)]
# [logos (subpattern EXPONENT = r#"[eE]([+-])?([0-9])+"#)]
# [logos (subpattern DOUBLE = r#"([+-])?(([0-9])+\.([0-9])*(?&EXPONENT)|\.([0-9])+(?&EXPONENT)|([0-9])+(?&EXPONENT))"#)]
# [logos (subpattern ECHAR = r#"\\[tbnrf"'\\]"#)]
# [logos (subpattern STRING_LITERAL_QUOTE = r#""(([^\x{22}\x{5C}\x{A}\x{D}]|(?&ECHAR)|(?&UCHAR)))*""#)]
# [logos (subpattern STRING_LITERAL_SINGLE_QUOTE = r#"'(([^\x{27}\x{5C}\x{A}\x{D}]|(?&ECHAR)|(?&UCHAR)))*'"#)]
# [logos (subpattern STRING_LITERAL_LONG_SINGLE_QUOTE = r#"'''((('|''))?([^'\\]|(?&ECHAR)|(?&UCHAR)))*'''"#)]
# [logos (subpattern STRING_LITERAL_LONG_QUOTE = r#""""((("|""))?([^"\\]|(?&ECHAR)|(?&UCHAR)))*""""#)]
# [logos (subpattern WS = r#"(\u0020|\u0009|\u000D|\u000A)"#)]
# [logos (subpattern ANON = r#"\[((?&WS))*\]"#)]
#[repr(u16)]
pub enum SyntaxKind {
    Eof = 0,
    #[regex(r"[ \t\n]+")]
    WhiteSpace,
    #[regex(r"#[^\n]+", allow_greedy = true)]
    Comment,
    #[doc = r" producings"]
    BlankNode,
    BooleanLiteral,
    NumericLiteral,
    PrefixedName,
    Rdfliteral,
    MyString,
    Base,
    Blank,
    BlankNodePropertyList,
    Block,
    Collection,
    Directive,
    GraphBlock,
    Iri,
    LabelOrSubject,
    Literal,
    Object,
    ObjectList,
    Predicate,
    PredicateObjectList,
    PrefixId,
    SparqlBase,
    SparqlPrefix,
    Subject,
    TrigDoc,
    Triples,
    Triples2,
    TriplesBlock,
    TriplesOrGraph,
    Verb,
    WrappedGraph,
    #[doc = r" terminals"]
    #[token("(")]
    ClOpen,
    #[token(")")]
    ClClose,
    #[token(",")]
    Comma,
    #[token(".")]
    Stop,
    #[token(";")]
    Colon,
    #[token("@base")]
    BaseToken,
    #[token("@prefix")]
    PrefixToken,
    #[token("BASE", ignore(case))]
    SparqlBaseToken,
    #[token("GRAPH", ignore(case))]
    GraphToken,
    #[token("PREFIX", ignore(case))]
    SparqlPrefixToken,
    #[token("[")]
    SqOpen,
    #[token("]")]
    SqClose,
    #[token("^^")]
    Datatype,
    #[token("a")]
    Alit,
    #[token("false")]
    FalseLit,
    #[token("true")]
    TrueLit,
    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,
    #[regex("(?&ANON)")]
    Anon,
    #[regex("(?&BLANK_NODE_LABEL)")]
    BlankNodeLabel,
    #[regex("(?&DECIMAL)")]
    Decimal,
    #[regex("(?&DOUBLE)")]
    Double,
    #[regex("(?&INTEGER)")]
    Integer,
    #[regex("(?&IRIREF)")]
    Iriref,
    #[regex("(?&LANGTAG)")]
    Langtag,
    #[regex("(?&PNAME_LN)")]
    PnameLn,
    #[regex("(?&PNAME_NS)")]
    PnameNs,
    #[regex("(?&STRING_LITERAL_LONG_QUOTE)")]
    StringLiteralLongQuote,
    #[regex("(?&STRING_LITERAL_LONG_SINGLE_QUOTE)")]
    StringLiteralLongSingleQuote,
    #[regex("(?&STRING_LITERAL_QUOTE)")]
    StringLiteralQuote,
    #[regex("(?&STRING_LITERAL_SINGLE_QUOTE)")]
    StringLiteralSingleQuote,
    Error,
    ROOT,
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
mod definitions {
    use super::*;
    #[derive(Debug, Clone, Copy)]
    pub struct Rule {
        pub kind: SyntaxKind,
        pub state: usize,
    }
    impl Rule {
        pub fn new(kind: SyntaxKind) -> Self {
            match kind {
                SyntaxKind::TrigDoc => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Block => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::GraphBlock => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::TriplesOrGraph => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::Triples2 => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::WrappedGraph => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::TriplesBlock => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::LabelOrSubject => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Directive => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PrefixId => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::Base => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::SparqlPrefix => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::SparqlBase => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::Triples => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PredicateObjectList => Rule {
                    kind,
                    state: 7usize,
                },
                SyntaxKind::ObjectList => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::Verb => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Subject => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Predicate => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Object => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Literal => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Blank => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::BlankNodePropertyList => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::Collection => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::NumericLiteral => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Rdfliteral => Rule {
                    kind,
                    state: 6usize,
                },
                SyntaxKind::BooleanLiteral => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::MyString => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Iri => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PrefixedName => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::BlankNode => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::ClOpen => Rule { kind, state: 0 },
                SyntaxKind::ClClose => Rule { kind, state: 0 },
                SyntaxKind::Comma => Rule { kind, state: 0 },
                SyntaxKind::Stop => Rule { kind, state: 0 },
                SyntaxKind::Colon => Rule { kind, state: 0 },
                SyntaxKind::BaseToken => Rule { kind, state: 0 },
                SyntaxKind::PrefixToken => Rule { kind, state: 0 },
                SyntaxKind::SparqlBaseToken => Rule { kind, state: 0 },
                SyntaxKind::GraphToken => Rule { kind, state: 0 },
                SyntaxKind::SparqlPrefixToken => Rule { kind, state: 0 },
                SyntaxKind::SqOpen => Rule { kind, state: 0 },
                SyntaxKind::SqClose => Rule { kind, state: 0 },
                SyntaxKind::Datatype => Rule { kind, state: 0 },
                SyntaxKind::Alit => Rule { kind, state: 0 },
                SyntaxKind::FalseLit => Rule { kind, state: 0 },
                SyntaxKind::TrueLit => Rule { kind, state: 0 },
                SyntaxKind::CurlyOpen => Rule { kind, state: 0 },
                SyntaxKind::CurlyClose => Rule { kind, state: 0 },
                SyntaxKind::Anon => Rule { kind, state: 0 },
                SyntaxKind::BlankNodeLabel => Rule { kind, state: 0 },
                SyntaxKind::Decimal => Rule { kind, state: 0 },
                SyntaxKind::Double => Rule { kind, state: 0 },
                SyntaxKind::Integer => Rule { kind, state: 0 },
                SyntaxKind::Iriref => Rule { kind, state: 0 },
                SyntaxKind::Langtag => Rule { kind, state: 0 },
                SyntaxKind::PnameLn => Rule { kind, state: 0 },
                SyntaxKind::PnameNs => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLongQuote => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLongSingleQuote => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralQuote => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralSingleQuote => Rule { kind, state: 0 },
                _ => panic!("Unknown rule kind {:?}", kind),
            }
        }
    }
    pub fn first_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::BlankNode => &[],
            SyntaxKind::BooleanLiteral => &[SyntaxKind::FalseLit, SyntaxKind::TrueLit],
            SyntaxKind::NumericLiteral => &[],
            SyntaxKind::PrefixedName => &[],
            SyntaxKind::Rdfliteral => &[],
            SyntaxKind::MyString => &[],
            SyntaxKind::Base => &[SyntaxKind::BaseToken],
            SyntaxKind::Blank => &[SyntaxKind::ClOpen],
            SyntaxKind::BlankNodePropertyList => &[SyntaxKind::SqOpen],
            SyntaxKind::Block => &[
                SyntaxKind::ClOpen,
                SyntaxKind::CurlyOpen,
                SyntaxKind::GraphToken,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::Collection => &[SyntaxKind::ClOpen],
            SyntaxKind::Directive => &[
                SyntaxKind::BaseToken,
                SyntaxKind::PrefixToken,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SparqlPrefixToken,
            ],
            SyntaxKind::GraphBlock => &[SyntaxKind::CurlyOpen],
            SyntaxKind::Iri => &[],
            SyntaxKind::LabelOrSubject => &[],
            SyntaxKind::Literal => &[SyntaxKind::FalseLit, SyntaxKind::TrueLit],
            SyntaxKind::Object => &[
                SyntaxKind::ClOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::ObjectList => &[
                SyntaxKind::ClOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Predicate => &[],
            SyntaxKind::PredicateObjectList => &[SyntaxKind::Alit],
            SyntaxKind::PrefixId => &[SyntaxKind::PrefixToken],
            SyntaxKind::SparqlBase => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::SparqlPrefix => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::Subject => &[SyntaxKind::ClOpen],
            SyntaxKind::TrigDoc => &[
                SyntaxKind::BaseToken,
                SyntaxKind::ClOpen,
                SyntaxKind::CurlyOpen,
                SyntaxKind::GraphToken,
                SyntaxKind::PrefixToken,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::Triples => &[SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::Triples2 => &[SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::TriplesBlock => &[SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::TriplesOrGraph => &[],
            SyntaxKind::Verb => &[SyntaxKind::Alit],
            SyntaxKind::WrappedGraph => &[SyntaxKind::CurlyOpen],
            SyntaxKind::ClOpen => &[SyntaxKind::ClOpen],
            SyntaxKind::ClClose => &[SyntaxKind::ClClose],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::BaseToken => &[SyntaxKind::BaseToken],
            SyntaxKind::PrefixToken => &[SyntaxKind::PrefixToken],
            SyntaxKind::SparqlBaseToken => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::GraphToken => &[SyntaxKind::GraphToken],
            SyntaxKind::SparqlPrefixToken => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::Alit => &[SyntaxKind::Alit],
            SyntaxKind::FalseLit => &[SyntaxKind::FalseLit],
            SyntaxKind::TrueLit => &[SyntaxKind::TrueLit],
            SyntaxKind::CurlyOpen => &[SyntaxKind::CurlyOpen],
            SyntaxKind::CurlyClose => &[SyntaxKind::CurlyClose],
            SyntaxKind::Anon => &[SyntaxKind::Anon],
            SyntaxKind::BlankNodeLabel => &[SyntaxKind::BlankNodeLabel],
            SyntaxKind::Decimal => &[SyntaxKind::Decimal],
            SyntaxKind::Double => &[SyntaxKind::Double],
            SyntaxKind::Integer => &[SyntaxKind::Integer],
            SyntaxKind::Iriref => &[SyntaxKind::Iriref],
            SyntaxKind::Langtag => &[SyntaxKind::Langtag],
            SyntaxKind::PnameLn => &[SyntaxKind::PnameLn],
            SyntaxKind::PnameNs => &[SyntaxKind::PnameNs],
            SyntaxKind::StringLiteralLongQuote => &[SyntaxKind::StringLiteralLongQuote],
            SyntaxKind::StringLiteralLongSingleQuote => &[SyntaxKind::StringLiteralLongSingleQuote],
            SyntaxKind::StringLiteralQuote => &[SyntaxKind::StringLiteralQuote],
            SyntaxKind::StringLiteralSingleQuote => &[SyntaxKind::StringLiteralSingleQuote],
            _ => &[],
        }
    }
    #[doc = r" Returns the minimum error cost that `kind` must incur when `tok`"]
    #[doc = r" is the current token.  0 means the token is reachable (or the rule"]
    #[doc = r" is nullable); positive means the rule cannot make progress without"]
    #[doc = r" at least that much error cost."]
    pub fn min_error_for_token(kind: SyntaxKind, tok: SyntaxKind) -> isize {
        match kind {
            SyntaxKind::BlankNode => match tok {
                SyntaxKind::Anon | SyntaxKind::BlankNodeLabel => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::BooleanLiteral => match tok {
                SyntaxKind::FalseLit | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::NumericLiteral => match tok {
                SyntaxKind::Decimal | SyntaxKind::Double | SyntaxKind::Integer => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::PrefixedName => match tok {
                SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Rdfliteral => match tok {
                SyntaxKind::Datatype
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::MyString => match tok {
                SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Base => match tok {
                SyntaxKind::BaseToken | SyntaxKind::Iriref | SyntaxKind::Stop => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Blank => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::BlankNodePropertyList => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Block => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::CurlyClose
                | SyntaxKind::CurlyOpen
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::GraphToken
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::Stop
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Collection => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Directive => match tok {
                SyntaxKind::BaseToken
                | SyntaxKind::Iriref
                | SyntaxKind::PnameNs
                | SyntaxKind::PrefixToken
                | SyntaxKind::SparqlBaseToken
                | SyntaxKind::SparqlPrefixToken
                | SyntaxKind::Stop => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::GraphBlock => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::CurlyClose
                | SyntaxKind::CurlyOpen
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::Stop
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Iri => match tok {
                SyntaxKind::Iriref | SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::LabelOrSubject => match tok {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Literal => match tok {
                SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Object => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::ObjectList => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Predicate => match tok {
                SyntaxKind::Iriref | SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::PredicateObjectList => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::PrefixId => match tok {
                SyntaxKind::Iriref
                | SyntaxKind::PnameNs
                | SyntaxKind::PrefixToken
                | SyntaxKind::Stop => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::SparqlBase => match tok {
                SyntaxKind::Iriref | SyntaxKind::SparqlBaseToken => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::SparqlPrefix => match tok {
                SyntaxKind::Iriref | SyntaxKind::PnameNs | SyntaxKind::SparqlPrefixToken => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Subject => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Triples => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Triples2 => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::Stop
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::TriplesBlock => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::Stop
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::TriplesOrGraph => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::Stop
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Verb => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::WrappedGraph => match tok {
                SyntaxKind::Alit
                | SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::CurlyClose
                | SyntaxKind::CurlyOpen
                | SyntaxKind::Datatype
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::Stop
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::ClOpen => match tok {
                SyntaxKind::ClOpen => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::ClClose => match tok {
                SyntaxKind::ClClose => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Comma => match tok {
                SyntaxKind::Comma => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Stop => match tok {
                SyntaxKind::Stop => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Colon => match tok {
                SyntaxKind::Colon => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::BaseToken => match tok {
                SyntaxKind::BaseToken => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::PrefixToken => match tok {
                SyntaxKind::PrefixToken => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::SparqlBaseToken => match tok {
                SyntaxKind::SparqlBaseToken => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::GraphToken => match tok {
                SyntaxKind::GraphToken => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::SparqlPrefixToken => match tok {
                SyntaxKind::SparqlPrefixToken => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::SqOpen => match tok {
                SyntaxKind::SqOpen => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::SqClose => match tok {
                SyntaxKind::SqClose => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Datatype => match tok {
                SyntaxKind::Datatype => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Alit => match tok {
                SyntaxKind::Alit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::FalseLit => match tok {
                SyntaxKind::FalseLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::TrueLit => match tok {
                SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::CurlyOpen => match tok {
                SyntaxKind::CurlyOpen => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::CurlyClose => match tok {
                SyntaxKind::CurlyClose => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Anon => match tok {
                SyntaxKind::Anon => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::BlankNodeLabel => match tok {
                SyntaxKind::BlankNodeLabel => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Decimal => match tok {
                SyntaxKind::Decimal => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Double => match tok {
                SyntaxKind::Double => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Integer => match tok {
                SyntaxKind::Integer => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Iriref => match tok {
                SyntaxKind::Iriref => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Langtag => match tok {
                SyntaxKind::Langtag => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::PnameLn => match tok {
                SyntaxKind::PnameLn => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::PnameNs => match tok {
                SyntaxKind::PnameNs => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::StringLiteralLongQuote => match tok {
                SyntaxKind::StringLiteralLongQuote => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::StringLiteralLongSingleQuote => match tok {
                SyntaxKind::StringLiteralLongSingleQuote => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::StringLiteralQuote => match tok {
                SyntaxKind::StringLiteralQuote => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::StringLiteralSingleQuote => match tok {
                SyntaxKind::StringLiteralSingleQuote => 0,
                _ => kind.max_error_value(),
            },
            _ => 0,
        }
    }
    #[doc = r" Precomputed dist(q, a): minimum insertion cost to reach a point"]
    #[doc = r" where terminal `terminal` can be matched, starting from parser"]
    #[doc = r" state `(kind, state)`.  Returns 0 as the conservative default"]
    #[doc = r" (admissible — parent context might accept the terminal after a pop)."]
    pub fn state_dist(kind: SyntaxKind, state: usize, terminal: SyntaxKind) -> isize {
        match (kind, state, terminal) {
            (SyntaxKind::BlankNode, 3usize, _) => match terminal {
                SyntaxKind::Anon => 0,
                _ => 1isize,
            },
            (SyntaxKind::BlankNode, 2usize, _) => match terminal {
                SyntaxKind::BlankNodeLabel => 0,
                _ => 1isize,
            },
            (SyntaxKind::BlankNode, 1usize, _) => match terminal {
                SyntaxKind::Anon | SyntaxKind::BlankNodeLabel => 0,
                _ => 1isize,
            },
            (SyntaxKind::BooleanLiteral, 2usize, _) => match terminal {
                SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::BooleanLiteral, 3usize, _) => match terminal {
                SyntaxKind::FalseLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::BooleanLiteral, 1usize, _) => match terminal {
                SyntaxKind::FalseLit | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::NumericLiteral, 4usize, _) => match terminal {
                SyntaxKind::Double => 0,
                _ => 1isize,
            },
            (SyntaxKind::NumericLiteral, 2usize, _) => match terminal {
                SyntaxKind::Integer => 0,
                _ => 1isize,
            },
            (SyntaxKind::NumericLiteral, 1usize, _) => match terminal {
                SyntaxKind::Decimal | SyntaxKind::Double | SyntaxKind::Integer => 0,
                _ => 1isize,
            },
            (SyntaxKind::NumericLiteral, 3usize, _) => match terminal {
                SyntaxKind::Decimal => 0,
                _ => 1isize,
            },
            (SyntaxKind::PrefixedName, 3usize, _) => match terminal {
                SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::PrefixedName, 1usize, _) => match terminal {
                SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::PrefixedName, 2usize, _) => match terminal {
                SyntaxKind::PnameLn => 0,
                _ => 1isize,
            },
            (SyntaxKind::Rdfliteral, 6usize, _) => match terminal {
                SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote => 0,
                _ => 1isize,
            },
            (SyntaxKind::Rdfliteral, 2usize, _) => match terminal {
                SyntaxKind::Langtag | SyntaxKind::Datatype => 0,
                _ => 1isize,
            },
            (SyntaxKind::Rdfliteral, 5usize, _) => match terminal {
                SyntaxKind::Alit => 2isize,
                SyntaxKind::Anon => 2isize,
                SyntaxKind::BlankNodeLabel => 2isize,
                SyntaxKind::Decimal => 2isize,
                SyntaxKind::Double => 2isize,
                SyntaxKind::FalseLit => 2isize,
                SyntaxKind::Integer => 2isize,
                SyntaxKind::Iriref => 1isize,
                SyntaxKind::Langtag => 2isize,
                SyntaxKind::PnameLn => 1isize,
                SyntaxKind::PnameNs => 1isize,
                SyntaxKind::StringLiteralLongQuote => 2isize,
                SyntaxKind::StringLiteralLongSingleQuote => 2isize,
                SyntaxKind::StringLiteralQuote => 2isize,
                SyntaxKind::StringLiteralSingleQuote => 2isize,
                SyntaxKind::TrueLit => 2isize,
                SyntaxKind::BaseToken => 2isize,
                SyntaxKind::ClClose => 2isize,
                SyntaxKind::ClOpen => 2isize,
                SyntaxKind::Colon => 2isize,
                SyntaxKind::Comma => 2isize,
                SyntaxKind::CurlyClose => 2isize,
                SyntaxKind::CurlyOpen => 2isize,
                SyntaxKind::GraphToken => 2isize,
                SyntaxKind::PrefixToken => 2isize,
                SyntaxKind::SparqlBaseToken => 2isize,
                SyntaxKind::SparqlPrefixToken => 2isize,
                SyntaxKind::SqClose => 2isize,
                SyntaxKind::SqOpen => 2isize,
                SyntaxKind::Stop => 2isize,
                _ => 0,
            },
            (SyntaxKind::Rdfliteral, 3usize, _) => match terminal {
                SyntaxKind::Langtag => 0,
                _ => 1isize,
            },
            (SyntaxKind::Rdfliteral, 4usize, _) => match terminal {
                SyntaxKind::Iriref | SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::MyString, 1usize, _) => match terminal {
                SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote => 0,
                _ => 1isize,
            },
            (SyntaxKind::MyString, 4usize, _) => match terminal {
                SyntaxKind::StringLiteralLongSingleQuote => 0,
                _ => 1isize,
            },
            (SyntaxKind::MyString, 3usize, _) => match terminal {
                SyntaxKind::StringLiteralSingleQuote => 0,
                _ => 1isize,
            },
            (SyntaxKind::MyString, 5usize, _) => match terminal {
                SyntaxKind::StringLiteralLongQuote => 0,
                _ => 1isize,
            },
            (SyntaxKind::MyString, 2usize, _) => match terminal {
                SyntaxKind::StringLiteralQuote => 0,
                _ => 1isize,
            },
            (SyntaxKind::Base, 1usize, _) => match terminal {
                SyntaxKind::Stop => 0,
                _ => 3isize,
            },
            (SyntaxKind::Base, 2usize, _) => match terminal {
                SyntaxKind::Alit => 4isize,
                SyntaxKind::Anon => 4isize,
                SyntaxKind::BlankNodeLabel => 4isize,
                SyntaxKind::Decimal => 4isize,
                SyntaxKind::Double => 4isize,
                SyntaxKind::FalseLit => 4isize,
                SyntaxKind::Integer => 4isize,
                SyntaxKind::Langtag => 4isize,
                SyntaxKind::PnameLn => 4isize,
                SyntaxKind::PnameNs => 4isize,
                SyntaxKind::StringLiteralLongQuote => 4isize,
                SyntaxKind::StringLiteralLongSingleQuote => 4isize,
                SyntaxKind::StringLiteralQuote => 4isize,
                SyntaxKind::StringLiteralSingleQuote => 4isize,
                SyntaxKind::TrueLit => 4isize,
                SyntaxKind::BaseToken => 4isize,
                SyntaxKind::ClClose => 4isize,
                SyntaxKind::ClOpen => 4isize,
                SyntaxKind::Colon => 4isize,
                SyntaxKind::Comma => 4isize,
                SyntaxKind::CurlyClose => 4isize,
                SyntaxKind::CurlyOpen => 4isize,
                SyntaxKind::Datatype => 4isize,
                SyntaxKind::GraphToken => 4isize,
                SyntaxKind::PrefixToken => 4isize,
                SyntaxKind::SparqlBaseToken => 4isize,
                SyntaxKind::SparqlPrefixToken => 4isize,
                SyntaxKind::SqClose => 4isize,
                SyntaxKind::SqOpen => 4isize,
                SyntaxKind::Stop => 1isize,
                _ => 0,
            },
            (SyntaxKind::Base, 3usize, _) => match terminal {
                SyntaxKind::Alit => 104isize,
                SyntaxKind::Anon => 104isize,
                SyntaxKind::BlankNodeLabel => 104isize,
                SyntaxKind::Decimal => 104isize,
                SyntaxKind::Double => 104isize,
                SyntaxKind::FalseLit => 104isize,
                SyntaxKind::Integer => 104isize,
                SyntaxKind::Iriref => 100isize,
                SyntaxKind::Langtag => 104isize,
                SyntaxKind::PnameLn => 104isize,
                SyntaxKind::PnameNs => 104isize,
                SyntaxKind::StringLiteralLongQuote => 104isize,
                SyntaxKind::StringLiteralLongSingleQuote => 104isize,
                SyntaxKind::StringLiteralQuote => 104isize,
                SyntaxKind::StringLiteralSingleQuote => 104isize,
                SyntaxKind::TrueLit => 104isize,
                SyntaxKind::ClClose => 104isize,
                SyntaxKind::ClOpen => 104isize,
                SyntaxKind::Colon => 104isize,
                SyntaxKind::Comma => 104isize,
                SyntaxKind::CurlyClose => 104isize,
                SyntaxKind::CurlyOpen => 104isize,
                SyntaxKind::Datatype => 104isize,
                SyntaxKind::GraphToken => 104isize,
                SyntaxKind::PrefixToken => 104isize,
                SyntaxKind::SparqlBaseToken => 104isize,
                SyntaxKind::SparqlPrefixToken => 104isize,
                SyntaxKind::SqClose => 104isize,
                SyntaxKind::SqOpen => 104isize,
                SyntaxKind::Stop => 101isize,
                _ => 0,
            },
            (SyntaxKind::Blank, 3usize, _) => match terminal {
                SyntaxKind::ClOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::Blank, 2usize, _) => match terminal {
                SyntaxKind::Anon | SyntaxKind::BlankNodeLabel => 0,
                _ => 1isize,
            },
            (SyntaxKind::Blank, 1usize, _) => match terminal {
                SyntaxKind::Anon | SyntaxKind::BlankNodeLabel | SyntaxKind::ClOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::BlankNodePropertyList, 3usize, _) => match terminal {
                SyntaxKind::Alit => 2isize,
                SyntaxKind::Anon => 3isize,
                SyntaxKind::BlankNodeLabel => 3isize,
                SyntaxKind::Decimal => 3isize,
                SyntaxKind::Double => 3isize,
                SyntaxKind::FalseLit => 3isize,
                SyntaxKind::Integer => 3isize,
                SyntaxKind::Iriref => 2isize,
                SyntaxKind::Langtag => 3isize,
                SyntaxKind::PnameLn => 2isize,
                SyntaxKind::PnameNs => 2isize,
                SyntaxKind::StringLiteralLongQuote => 3isize,
                SyntaxKind::StringLiteralLongSingleQuote => 3isize,
                SyntaxKind::StringLiteralQuote => 3isize,
                SyntaxKind::StringLiteralSingleQuote => 3isize,
                SyntaxKind::TrueLit => 3isize,
                SyntaxKind::BaseToken => 3isize,
                SyntaxKind::ClClose => 3isize,
                SyntaxKind::ClOpen => 3isize,
                SyntaxKind::Colon => 3isize,
                SyntaxKind::Comma => 3isize,
                SyntaxKind::CurlyClose => 3isize,
                SyntaxKind::CurlyOpen => 3isize,
                SyntaxKind::Datatype => 3isize,
                SyntaxKind::GraphToken => 3isize,
                SyntaxKind::PrefixToken => 3isize,
                SyntaxKind::SparqlBaseToken => 3isize,
                SyntaxKind::SparqlPrefixToken => 3isize,
                SyntaxKind::SqClose => 3isize,
                SyntaxKind::Stop => 3isize,
                _ => 0,
            },
            (SyntaxKind::BlankNodePropertyList, 1usize, _) => match terminal {
                SyntaxKind::SqClose => 0,
                _ => 2isize,
            },
            (SyntaxKind::BlankNodePropertyList, 2usize, _) => match terminal {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Block, 5usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Block, 4usize, _) => match terminal {
                SyntaxKind::CurlyOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::Block, 2usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::CurlyOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::Block, 7usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Block, 6usize, _) => match terminal {
                SyntaxKind::Alit => 4isize,
                SyntaxKind::Anon => 3isize,
                SyntaxKind::BlankNodeLabel => 3isize,
                SyntaxKind::Decimal => 4isize,
                SyntaxKind::Double => 4isize,
                SyntaxKind::FalseLit => 4isize,
                SyntaxKind::Integer => 4isize,
                SyntaxKind::Iriref => 3isize,
                SyntaxKind::Langtag => 4isize,
                SyntaxKind::PnameLn => 3isize,
                SyntaxKind::PnameNs => 3isize,
                SyntaxKind::StringLiteralLongQuote => 4isize,
                SyntaxKind::StringLiteralLongSingleQuote => 4isize,
                SyntaxKind::StringLiteralQuote => 4isize,
                SyntaxKind::StringLiteralSingleQuote => 4isize,
                SyntaxKind::TrueLit => 4isize,
                SyntaxKind::BaseToken => 4isize,
                SyntaxKind::ClClose => 4isize,
                SyntaxKind::ClOpen => 4isize,
                SyntaxKind::Colon => 4isize,
                SyntaxKind::Comma => 4isize,
                SyntaxKind::CurlyClose => 4isize,
                SyntaxKind::CurlyOpen => 4isize,
                SyntaxKind::Datatype => 4isize,
                SyntaxKind::PrefixToken => 4isize,
                SyntaxKind::SparqlBaseToken => 4isize,
                SyntaxKind::SparqlPrefixToken => 4isize,
                SyntaxKind::SqClose => 4isize,
                SyntaxKind::SqOpen => 4isize,
                SyntaxKind::Stop => 4isize,
                _ => 0,
            },
            (SyntaxKind::Block, 1usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::ClOpen
                | SyntaxKind::CurlyOpen
                | SyntaxKind::GraphToken
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::Block, 3usize, _) => match terminal {
                SyntaxKind::ClOpen | SyntaxKind::SqOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::Collection, 3usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::Collection, 4usize, _) => match terminal {
                SyntaxKind::ClOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::Collection, 1usize, _) => match terminal {
                SyntaxKind::ClClose => 0,
                _ => 2isize,
            },
            (SyntaxKind::Collection, 2usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit
                | SyntaxKind::ClClose
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::Directive, 1usize, _) => match terminal {
                SyntaxKind::Alit => 101isize,
                SyntaxKind::Anon => 101isize,
                SyntaxKind::BlankNodeLabel => 101isize,
                SyntaxKind::Decimal => 101isize,
                SyntaxKind::Double => 101isize,
                SyntaxKind::FalseLit => 101isize,
                SyntaxKind::Integer => 101isize,
                SyntaxKind::Iriref => 100isize,
                SyntaxKind::Langtag => 101isize,
                SyntaxKind::PnameLn => 101isize,
                SyntaxKind::PnameNs => 100isize,
                SyntaxKind::StringLiteralLongQuote => 101isize,
                SyntaxKind::StringLiteralLongSingleQuote => 101isize,
                SyntaxKind::StringLiteralQuote => 101isize,
                SyntaxKind::StringLiteralSingleQuote => 101isize,
                SyntaxKind::TrueLit => 101isize,
                SyntaxKind::ClClose => 101isize,
                SyntaxKind::ClOpen => 101isize,
                SyntaxKind::Colon => 101isize,
                SyntaxKind::Comma => 101isize,
                SyntaxKind::CurlyClose => 101isize,
                SyntaxKind::CurlyOpen => 101isize,
                SyntaxKind::Datatype => 101isize,
                SyntaxKind::GraphToken => 101isize,
                SyntaxKind::SqClose => 101isize,
                SyntaxKind::SqOpen => 101isize,
                SyntaxKind::Stop => 101isize,
                _ => 0,
            },
            (SyntaxKind::Directive, 3usize, _) => match terminal {
                SyntaxKind::Alit => 104isize,
                SyntaxKind::Anon => 104isize,
                SyntaxKind::BlankNodeLabel => 104isize,
                SyntaxKind::Decimal => 104isize,
                SyntaxKind::Double => 104isize,
                SyntaxKind::FalseLit => 104isize,
                SyntaxKind::Integer => 104isize,
                SyntaxKind::Iriref => 100isize,
                SyntaxKind::Langtag => 104isize,
                SyntaxKind::PnameLn => 104isize,
                SyntaxKind::PnameNs => 104isize,
                SyntaxKind::StringLiteralLongQuote => 104isize,
                SyntaxKind::StringLiteralLongSingleQuote => 104isize,
                SyntaxKind::StringLiteralQuote => 104isize,
                SyntaxKind::StringLiteralSingleQuote => 104isize,
                SyntaxKind::TrueLit => 104isize,
                SyntaxKind::ClClose => 104isize,
                SyntaxKind::ClOpen => 104isize,
                SyntaxKind::Colon => 104isize,
                SyntaxKind::Comma => 104isize,
                SyntaxKind::CurlyClose => 104isize,
                SyntaxKind::CurlyOpen => 104isize,
                SyntaxKind::Datatype => 104isize,
                SyntaxKind::GraphToken => 104isize,
                SyntaxKind::PrefixToken => 104isize,
                SyntaxKind::SparqlBaseToken => 104isize,
                SyntaxKind::SparqlPrefixToken => 104isize,
                SyntaxKind::SqClose => 104isize,
                SyntaxKind::SqOpen => 104isize,
                SyntaxKind::Stop => 101isize,
                _ => 0,
            },
            (SyntaxKind::Directive, 2usize, _) => match terminal {
                SyntaxKind::Alit => 105isize,
                SyntaxKind::Anon => 105isize,
                SyntaxKind::BlankNodeLabel => 105isize,
                SyntaxKind::Decimal => 105isize,
                SyntaxKind::Double => 105isize,
                SyntaxKind::FalseLit => 105isize,
                SyntaxKind::Integer => 105isize,
                SyntaxKind::Iriref => 101isize,
                SyntaxKind::Langtag => 105isize,
                SyntaxKind::PnameLn => 105isize,
                SyntaxKind::PnameNs => 100isize,
                SyntaxKind::StringLiteralLongQuote => 105isize,
                SyntaxKind::StringLiteralLongSingleQuote => 105isize,
                SyntaxKind::StringLiteralQuote => 105isize,
                SyntaxKind::StringLiteralSingleQuote => 105isize,
                SyntaxKind::TrueLit => 105isize,
                SyntaxKind::BaseToken => 105isize,
                SyntaxKind::ClClose => 105isize,
                SyntaxKind::ClOpen => 105isize,
                SyntaxKind::Colon => 105isize,
                SyntaxKind::Comma => 105isize,
                SyntaxKind::CurlyClose => 105isize,
                SyntaxKind::CurlyOpen => 105isize,
                SyntaxKind::Datatype => 105isize,
                SyntaxKind::GraphToken => 105isize,
                SyntaxKind::SparqlBaseToken => 105isize,
                SyntaxKind::SparqlPrefixToken => 105isize,
                SyntaxKind::SqClose => 105isize,
                SyntaxKind::SqOpen => 105isize,
                SyntaxKind::Stop => 102isize,
                _ => 0,
            },
            (SyntaxKind::Directive, 5usize, _) => match terminal {
                SyntaxKind::Alit => 101isize,
                SyntaxKind::Anon => 101isize,
                SyntaxKind::BlankNodeLabel => 101isize,
                SyntaxKind::Decimal => 101isize,
                SyntaxKind::Double => 101isize,
                SyntaxKind::FalseLit => 101isize,
                SyntaxKind::Integer => 101isize,
                SyntaxKind::Iriref => 100isize,
                SyntaxKind::Langtag => 101isize,
                SyntaxKind::PnameLn => 101isize,
                SyntaxKind::PnameNs => 101isize,
                SyntaxKind::StringLiteralLongQuote => 101isize,
                SyntaxKind::StringLiteralLongSingleQuote => 101isize,
                SyntaxKind::StringLiteralQuote => 101isize,
                SyntaxKind::StringLiteralSingleQuote => 101isize,
                SyntaxKind::TrueLit => 101isize,
                SyntaxKind::BaseToken => 101isize,
                SyntaxKind::ClClose => 101isize,
                SyntaxKind::ClOpen => 101isize,
                SyntaxKind::Colon => 101isize,
                SyntaxKind::Comma => 101isize,
                SyntaxKind::CurlyClose => 101isize,
                SyntaxKind::CurlyOpen => 101isize,
                SyntaxKind::Datatype => 101isize,
                SyntaxKind::GraphToken => 101isize,
                SyntaxKind::PrefixToken => 101isize,
                SyntaxKind::SparqlPrefixToken => 101isize,
                SyntaxKind::SqClose => 101isize,
                SyntaxKind::SqOpen => 101isize,
                SyntaxKind::Stop => 101isize,
                _ => 0,
            },
            (SyntaxKind::Directive, 4usize, _) => match terminal {
                SyntaxKind::Alit => 102isize,
                SyntaxKind::Anon => 102isize,
                SyntaxKind::BlankNodeLabel => 102isize,
                SyntaxKind::Decimal => 102isize,
                SyntaxKind::Double => 102isize,
                SyntaxKind::FalseLit => 102isize,
                SyntaxKind::Integer => 102isize,
                SyntaxKind::Iriref => 101isize,
                SyntaxKind::Langtag => 102isize,
                SyntaxKind::PnameLn => 102isize,
                SyntaxKind::PnameNs => 100isize,
                SyntaxKind::StringLiteralLongQuote => 102isize,
                SyntaxKind::StringLiteralLongSingleQuote => 102isize,
                SyntaxKind::StringLiteralQuote => 102isize,
                SyntaxKind::StringLiteralSingleQuote => 102isize,
                SyntaxKind::TrueLit => 102isize,
                SyntaxKind::BaseToken => 102isize,
                SyntaxKind::ClClose => 102isize,
                SyntaxKind::ClOpen => 102isize,
                SyntaxKind::Colon => 102isize,
                SyntaxKind::Comma => 102isize,
                SyntaxKind::CurlyClose => 102isize,
                SyntaxKind::CurlyOpen => 102isize,
                SyntaxKind::Datatype => 102isize,
                SyntaxKind::GraphToken => 102isize,
                SyntaxKind::PrefixToken => 102isize,
                SyntaxKind::SparqlBaseToken => 102isize,
                SyntaxKind::SqClose => 102isize,
                SyntaxKind::SqOpen => 102isize,
                SyntaxKind::Stop => 102isize,
                _ => 0,
            },
            (SyntaxKind::GraphBlock, 3usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::GraphBlock, 2usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::CurlyOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::GraphBlock, 1usize, _) => match terminal {
                SyntaxKind::CurlyOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::Iri, 3usize, _) => match terminal {
                SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Iri, 1usize, _) => match terminal {
                SyntaxKind::Iriref | SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Iri, 2usize, _) => match terminal {
                SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::LabelOrSubject, 2usize, _) => match terminal {
                SyntaxKind::Iriref | SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::LabelOrSubject, 1usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::LabelOrSubject, 3usize, _) => match terminal {
                SyntaxKind::Anon | SyntaxKind::BlankNodeLabel => 0,
                _ => 1isize,
            },
            (SyntaxKind::Literal, 3usize, _) => match terminal {
                SyntaxKind::Decimal | SyntaxKind::Double | SyntaxKind::Integer => 0,
                _ => 1isize,
            },
            (SyntaxKind::Literal, 4usize, _) => match terminal {
                SyntaxKind::FalseLit | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::Literal, 1usize, _) => match terminal {
                SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::Literal, 2usize, _) => match terminal {
                SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote => 0,
                _ => 1isize,
            },
            (SyntaxKind::Object, 1usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::Object, 4usize, _) => match terminal {
                SyntaxKind::Alit => 2isize,
                SyntaxKind::Anon => 3isize,
                SyntaxKind::BlankNodeLabel => 3isize,
                SyntaxKind::Decimal => 3isize,
                SyntaxKind::Double => 3isize,
                SyntaxKind::FalseLit => 3isize,
                SyntaxKind::Integer => 3isize,
                SyntaxKind::Iriref => 2isize,
                SyntaxKind::Langtag => 3isize,
                SyntaxKind::PnameLn => 2isize,
                SyntaxKind::PnameNs => 2isize,
                SyntaxKind::StringLiteralLongQuote => 3isize,
                SyntaxKind::StringLiteralLongSingleQuote => 3isize,
                SyntaxKind::StringLiteralQuote => 3isize,
                SyntaxKind::StringLiteralSingleQuote => 3isize,
                SyntaxKind::TrueLit => 3isize,
                SyntaxKind::BaseToken => 3isize,
                SyntaxKind::ClClose => 3isize,
                SyntaxKind::ClOpen => 3isize,
                SyntaxKind::Colon => 3isize,
                SyntaxKind::Comma => 3isize,
                SyntaxKind::CurlyClose => 3isize,
                SyntaxKind::CurlyOpen => 3isize,
                SyntaxKind::Datatype => 3isize,
                SyntaxKind::GraphToken => 3isize,
                SyntaxKind::PrefixToken => 3isize,
                SyntaxKind::SparqlBaseToken => 3isize,
                SyntaxKind::SparqlPrefixToken => 3isize,
                SyntaxKind::SqClose => 3isize,
                SyntaxKind::Stop => 3isize,
                _ => 0,
            },
            (SyntaxKind::Object, 5usize, _) => match terminal {
                SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::Object, 2usize, _) => match terminal {
                SyntaxKind::Iriref | SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Object, 3usize, _) => match terminal {
                SyntaxKind::Anon | SyntaxKind::BlankNodeLabel | SyntaxKind::ClOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::ObjectList, 4usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::ObjectList, 2usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::ObjectList, 3usize, _) => match terminal {
                SyntaxKind::Alit => 2isize,
                SyntaxKind::Anon => 1isize,
                SyntaxKind::BlankNodeLabel => 1isize,
                SyntaxKind::Decimal => 1isize,
                SyntaxKind::Double => 1isize,
                SyntaxKind::FalseLit => 1isize,
                SyntaxKind::Integer => 1isize,
                SyntaxKind::Iriref => 1isize,
                SyntaxKind::Langtag => 2isize,
                SyntaxKind::PnameLn => 1isize,
                SyntaxKind::PnameNs => 1isize,
                SyntaxKind::StringLiteralLongQuote => 1isize,
                SyntaxKind::StringLiteralLongSingleQuote => 1isize,
                SyntaxKind::StringLiteralQuote => 1isize,
                SyntaxKind::StringLiteralSingleQuote => 1isize,
                SyntaxKind::TrueLit => 1isize,
                SyntaxKind::BaseToken => 2isize,
                SyntaxKind::ClClose => 2isize,
                SyntaxKind::ClOpen => 1isize,
                SyntaxKind::Colon => 2isize,
                SyntaxKind::CurlyClose => 2isize,
                SyntaxKind::CurlyOpen => 2isize,
                SyntaxKind::Datatype => 2isize,
                SyntaxKind::GraphToken => 2isize,
                SyntaxKind::PrefixToken => 2isize,
                SyntaxKind::SparqlBaseToken => 2isize,
                SyntaxKind::SparqlPrefixToken => 2isize,
                SyntaxKind::SqClose => 2isize,
                SyntaxKind::SqOpen => 1isize,
                SyntaxKind::Stop => 2isize,
                _ => 0,
            },
            (SyntaxKind::Predicate, 1usize, _) => match terminal {
                SyntaxKind::Iriref | SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::PredicateObjectList, 3usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::PredicateObjectList, 7usize, _) => match terminal {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::PredicateObjectList, 4usize, _) => match terminal {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::PredicateObjectList, 5usize, _) => match terminal {
                SyntaxKind::Colon => 0,
                _ => 1isize,
            },
            (SyntaxKind::PredicateObjectList, 6usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Decimal
                | SyntaxKind::Double
                | SyntaxKind::FalseLit
                | SyntaxKind::Integer
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::StringLiteralLongQuote
                | SyntaxKind::StringLiteralLongSingleQuote
                | SyntaxKind::StringLiteralQuote
                | SyntaxKind::StringLiteralSingleQuote
                | SyntaxKind::TrueLit
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::PrefixId, 3usize, _) => match terminal {
                SyntaxKind::Alit => 5isize,
                SyntaxKind::Anon => 5isize,
                SyntaxKind::BlankNodeLabel => 5isize,
                SyntaxKind::Decimal => 5isize,
                SyntaxKind::Double => 5isize,
                SyntaxKind::FalseLit => 5isize,
                SyntaxKind::Integer => 5isize,
                SyntaxKind::Iriref => 1isize,
                SyntaxKind::Langtag => 5isize,
                SyntaxKind::PnameLn => 5isize,
                SyntaxKind::StringLiteralLongQuote => 5isize,
                SyntaxKind::StringLiteralLongSingleQuote => 5isize,
                SyntaxKind::StringLiteralQuote => 5isize,
                SyntaxKind::StringLiteralSingleQuote => 5isize,
                SyntaxKind::TrueLit => 5isize,
                SyntaxKind::BaseToken => 5isize,
                SyntaxKind::ClClose => 5isize,
                SyntaxKind::ClOpen => 5isize,
                SyntaxKind::Colon => 5isize,
                SyntaxKind::Comma => 5isize,
                SyntaxKind::CurlyClose => 5isize,
                SyntaxKind::CurlyOpen => 5isize,
                SyntaxKind::Datatype => 5isize,
                SyntaxKind::GraphToken => 5isize,
                SyntaxKind::PrefixToken => 5isize,
                SyntaxKind::SparqlBaseToken => 5isize,
                SyntaxKind::SparqlPrefixToken => 5isize,
                SyntaxKind::SqClose => 5isize,
                SyntaxKind::SqOpen => 5isize,
                SyntaxKind::Stop => 2isize,
                _ => 0,
            },
            (SyntaxKind::PrefixId, 2usize, _) => match terminal {
                SyntaxKind::Alit => 4isize,
                SyntaxKind::Anon => 4isize,
                SyntaxKind::BlankNodeLabel => 4isize,
                SyntaxKind::Decimal => 4isize,
                SyntaxKind::Double => 4isize,
                SyntaxKind::FalseLit => 4isize,
                SyntaxKind::Integer => 4isize,
                SyntaxKind::Langtag => 4isize,
                SyntaxKind::PnameLn => 4isize,
                SyntaxKind::PnameNs => 4isize,
                SyntaxKind::StringLiteralLongQuote => 4isize,
                SyntaxKind::StringLiteralLongSingleQuote => 4isize,
                SyntaxKind::StringLiteralQuote => 4isize,
                SyntaxKind::StringLiteralSingleQuote => 4isize,
                SyntaxKind::TrueLit => 4isize,
                SyntaxKind::BaseToken => 4isize,
                SyntaxKind::ClClose => 4isize,
                SyntaxKind::ClOpen => 4isize,
                SyntaxKind::Colon => 4isize,
                SyntaxKind::Comma => 4isize,
                SyntaxKind::CurlyClose => 4isize,
                SyntaxKind::CurlyOpen => 4isize,
                SyntaxKind::Datatype => 4isize,
                SyntaxKind::GraphToken => 4isize,
                SyntaxKind::PrefixToken => 4isize,
                SyntaxKind::SparqlBaseToken => 4isize,
                SyntaxKind::SparqlPrefixToken => 4isize,
                SyntaxKind::SqClose => 4isize,
                SyntaxKind::SqOpen => 4isize,
                SyntaxKind::Stop => 1isize,
                _ => 0,
            },
            (SyntaxKind::PrefixId, 1usize, _) => match terminal {
                SyntaxKind::Stop => 0,
                _ => 3isize,
            },
            (SyntaxKind::PrefixId, 4usize, _) => match terminal {
                SyntaxKind::Alit => 105isize,
                SyntaxKind::Anon => 105isize,
                SyntaxKind::BlankNodeLabel => 105isize,
                SyntaxKind::Decimal => 105isize,
                SyntaxKind::Double => 105isize,
                SyntaxKind::FalseLit => 105isize,
                SyntaxKind::Integer => 105isize,
                SyntaxKind::Iriref => 101isize,
                SyntaxKind::Langtag => 105isize,
                SyntaxKind::PnameLn => 105isize,
                SyntaxKind::PnameNs => 100isize,
                SyntaxKind::StringLiteralLongQuote => 105isize,
                SyntaxKind::StringLiteralLongSingleQuote => 105isize,
                SyntaxKind::StringLiteralQuote => 105isize,
                SyntaxKind::StringLiteralSingleQuote => 105isize,
                SyntaxKind::TrueLit => 105isize,
                SyntaxKind::BaseToken => 105isize,
                SyntaxKind::ClClose => 105isize,
                SyntaxKind::ClOpen => 105isize,
                SyntaxKind::Colon => 105isize,
                SyntaxKind::Comma => 105isize,
                SyntaxKind::CurlyClose => 105isize,
                SyntaxKind::CurlyOpen => 105isize,
                SyntaxKind::Datatype => 105isize,
                SyntaxKind::GraphToken => 105isize,
                SyntaxKind::SparqlBaseToken => 105isize,
                SyntaxKind::SparqlPrefixToken => 105isize,
                SyntaxKind::SqClose => 105isize,
                SyntaxKind::SqOpen => 105isize,
                SyntaxKind::Stop => 102isize,
                _ => 0,
            },
            (SyntaxKind::SparqlBase, 2usize, _) => match terminal {
                SyntaxKind::Alit => 101isize,
                SyntaxKind::Anon => 101isize,
                SyntaxKind::BlankNodeLabel => 101isize,
                SyntaxKind::Decimal => 101isize,
                SyntaxKind::Double => 101isize,
                SyntaxKind::FalseLit => 101isize,
                SyntaxKind::Integer => 101isize,
                SyntaxKind::Iriref => 100isize,
                SyntaxKind::Langtag => 101isize,
                SyntaxKind::PnameLn => 101isize,
                SyntaxKind::PnameNs => 101isize,
                SyntaxKind::StringLiteralLongQuote => 101isize,
                SyntaxKind::StringLiteralLongSingleQuote => 101isize,
                SyntaxKind::StringLiteralQuote => 101isize,
                SyntaxKind::StringLiteralSingleQuote => 101isize,
                SyntaxKind::TrueLit => 101isize,
                SyntaxKind::BaseToken => 101isize,
                SyntaxKind::ClClose => 101isize,
                SyntaxKind::ClOpen => 101isize,
                SyntaxKind::Colon => 101isize,
                SyntaxKind::Comma => 101isize,
                SyntaxKind::CurlyClose => 101isize,
                SyntaxKind::CurlyOpen => 101isize,
                SyntaxKind::Datatype => 101isize,
                SyntaxKind::GraphToken => 101isize,
                SyntaxKind::PrefixToken => 101isize,
                SyntaxKind::SparqlPrefixToken => 101isize,
                SyntaxKind::SqClose => 101isize,
                SyntaxKind::SqOpen => 101isize,
                SyntaxKind::Stop => 101isize,
                _ => 0,
            },
            (SyntaxKind::SparqlBase, 1usize, _) => match terminal {
                SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::SparqlPrefix, 1usize, _) => match terminal {
                SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::SparqlPrefix, 2usize, _) => match terminal {
                SyntaxKind::Alit => 2isize,
                SyntaxKind::Anon => 2isize,
                SyntaxKind::BlankNodeLabel => 2isize,
                SyntaxKind::Decimal => 2isize,
                SyntaxKind::Double => 2isize,
                SyntaxKind::FalseLit => 2isize,
                SyntaxKind::Integer => 2isize,
                SyntaxKind::Iriref => 1isize,
                SyntaxKind::Langtag => 2isize,
                SyntaxKind::PnameLn => 2isize,
                SyntaxKind::StringLiteralLongQuote => 2isize,
                SyntaxKind::StringLiteralLongSingleQuote => 2isize,
                SyntaxKind::StringLiteralQuote => 2isize,
                SyntaxKind::StringLiteralSingleQuote => 2isize,
                SyntaxKind::TrueLit => 2isize,
                SyntaxKind::BaseToken => 2isize,
                SyntaxKind::ClClose => 2isize,
                SyntaxKind::ClOpen => 2isize,
                SyntaxKind::Colon => 2isize,
                SyntaxKind::Comma => 2isize,
                SyntaxKind::CurlyClose => 2isize,
                SyntaxKind::CurlyOpen => 2isize,
                SyntaxKind::Datatype => 2isize,
                SyntaxKind::GraphToken => 2isize,
                SyntaxKind::PrefixToken => 2isize,
                SyntaxKind::SparqlBaseToken => 2isize,
                SyntaxKind::SparqlPrefixToken => 2isize,
                SyntaxKind::SqClose => 2isize,
                SyntaxKind::SqOpen => 2isize,
                SyntaxKind::Stop => 2isize,
                _ => 0,
            },
            (SyntaxKind::SparqlPrefix, 3usize, _) => match terminal {
                SyntaxKind::Alit => 102isize,
                SyntaxKind::Anon => 102isize,
                SyntaxKind::BlankNodeLabel => 102isize,
                SyntaxKind::Decimal => 102isize,
                SyntaxKind::Double => 102isize,
                SyntaxKind::FalseLit => 102isize,
                SyntaxKind::Integer => 102isize,
                SyntaxKind::Iriref => 101isize,
                SyntaxKind::Langtag => 102isize,
                SyntaxKind::PnameLn => 102isize,
                SyntaxKind::PnameNs => 100isize,
                SyntaxKind::StringLiteralLongQuote => 102isize,
                SyntaxKind::StringLiteralLongSingleQuote => 102isize,
                SyntaxKind::StringLiteralQuote => 102isize,
                SyntaxKind::StringLiteralSingleQuote => 102isize,
                SyntaxKind::TrueLit => 102isize,
                SyntaxKind::BaseToken => 102isize,
                SyntaxKind::ClClose => 102isize,
                SyntaxKind::ClOpen => 102isize,
                SyntaxKind::Colon => 102isize,
                SyntaxKind::Comma => 102isize,
                SyntaxKind::CurlyClose => 102isize,
                SyntaxKind::CurlyOpen => 102isize,
                SyntaxKind::Datatype => 102isize,
                SyntaxKind::GraphToken => 102isize,
                SyntaxKind::PrefixToken => 102isize,
                SyntaxKind::SparqlBaseToken => 102isize,
                SyntaxKind::SqClose => 102isize,
                SyntaxKind::SqOpen => 102isize,
                SyntaxKind::Stop => 102isize,
                _ => 0,
            },
            (SyntaxKind::Subject, 3usize, _) => match terminal {
                SyntaxKind::Anon | SyntaxKind::BlankNodeLabel | SyntaxKind::ClOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::Subject, 1usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::ClOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::Subject, 2usize, _) => match terminal {
                SyntaxKind::Iriref | SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::TrigDoc, 2usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::BaseToken
                | SyntaxKind::ClOpen
                | SyntaxKind::CurlyOpen
                | SyntaxKind::GraphToken
                | SyntaxKind::PrefixToken
                | SyntaxKind::SparqlBaseToken
                | SyntaxKind::SparqlPrefixToken
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::TrigDoc, 3usize, _) => match terminal {
                SyntaxKind::Alit => 101isize,
                SyntaxKind::Anon => 101isize,
                SyntaxKind::BlankNodeLabel => 101isize,
                SyntaxKind::Decimal => 101isize,
                SyntaxKind::Double => 101isize,
                SyntaxKind::FalseLit => 101isize,
                SyntaxKind::Integer => 101isize,
                SyntaxKind::Iriref => 100isize,
                SyntaxKind::Langtag => 101isize,
                SyntaxKind::PnameLn => 101isize,
                SyntaxKind::PnameNs => 100isize,
                SyntaxKind::StringLiteralLongQuote => 101isize,
                SyntaxKind::StringLiteralLongSingleQuote => 101isize,
                SyntaxKind::StringLiteralQuote => 101isize,
                SyntaxKind::StringLiteralSingleQuote => 101isize,
                SyntaxKind::TrueLit => 101isize,
                SyntaxKind::ClClose => 101isize,
                SyntaxKind::ClOpen => 101isize,
                SyntaxKind::Colon => 101isize,
                SyntaxKind::Comma => 101isize,
                SyntaxKind::CurlyClose => 101isize,
                SyntaxKind::CurlyOpen => 101isize,
                SyntaxKind::Datatype => 101isize,
                SyntaxKind::GraphToken => 101isize,
                SyntaxKind::SqClose => 101isize,
                SyntaxKind::SqOpen => 101isize,
                SyntaxKind::Stop => 101isize,
                _ => 0,
            },
            (SyntaxKind::TrigDoc, 4usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::ClOpen
                | SyntaxKind::CurlyOpen
                | SyntaxKind::GraphToken
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triples, 5usize, _) => match terminal {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triples, 6usize, _) => match terminal {
                SyntaxKind::Alit => 2isize,
                SyntaxKind::Anon => 3isize,
                SyntaxKind::BlankNodeLabel => 3isize,
                SyntaxKind::Decimal => 3isize,
                SyntaxKind::Double => 3isize,
                SyntaxKind::FalseLit => 3isize,
                SyntaxKind::Integer => 3isize,
                SyntaxKind::Iriref => 2isize,
                SyntaxKind::Langtag => 3isize,
                SyntaxKind::PnameLn => 2isize,
                SyntaxKind::PnameNs => 2isize,
                SyntaxKind::StringLiteralLongQuote => 3isize,
                SyntaxKind::StringLiteralLongSingleQuote => 3isize,
                SyntaxKind::StringLiteralQuote => 3isize,
                SyntaxKind::StringLiteralSingleQuote => 3isize,
                SyntaxKind::TrueLit => 3isize,
                SyntaxKind::BaseToken => 3isize,
                SyntaxKind::ClClose => 3isize,
                SyntaxKind::ClOpen => 3isize,
                SyntaxKind::Colon => 3isize,
                SyntaxKind::Comma => 3isize,
                SyntaxKind::CurlyClose => 3isize,
                SyntaxKind::CurlyOpen => 3isize,
                SyntaxKind::Datatype => 3isize,
                SyntaxKind::GraphToken => 3isize,
                SyntaxKind::PrefixToken => 3isize,
                SyntaxKind::SparqlBaseToken => 3isize,
                SyntaxKind::SparqlPrefixToken => 3isize,
                SyntaxKind::SqClose => 3isize,
                SyntaxKind::Stop => 3isize,
                _ => 0,
            },
            (SyntaxKind::Triples, 3usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::ClOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triples, 2usize, _) => match terminal {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triples, 1usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triples2, 7usize, _) => match terminal {
                SyntaxKind::ClOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::Triples2, 1usize, _) => match terminal {
                SyntaxKind::Stop => 0,
                _ => 3isize,
            },
            (SyntaxKind::Triples2, 2usize, _) => match terminal {
                SyntaxKind::ClOpen | SyntaxKind::SqOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::Triples2, 4usize, _) => match terminal {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triples2, 6usize, _) => match terminal {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triples2, 3usize, _) => match terminal {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::Stop => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triples2, 5usize, _) => match terminal {
                SyntaxKind::Alit => 2isize,
                SyntaxKind::Anon => 3isize,
                SyntaxKind::BlankNodeLabel => 3isize,
                SyntaxKind::Decimal => 3isize,
                SyntaxKind::Double => 3isize,
                SyntaxKind::FalseLit => 3isize,
                SyntaxKind::Integer => 3isize,
                SyntaxKind::Iriref => 2isize,
                SyntaxKind::Langtag => 3isize,
                SyntaxKind::PnameLn => 2isize,
                SyntaxKind::PnameNs => 2isize,
                SyntaxKind::StringLiteralLongQuote => 3isize,
                SyntaxKind::StringLiteralLongSingleQuote => 3isize,
                SyntaxKind::StringLiteralQuote => 3isize,
                SyntaxKind::StringLiteralSingleQuote => 3isize,
                SyntaxKind::TrueLit => 3isize,
                SyntaxKind::BaseToken => 3isize,
                SyntaxKind::ClClose => 3isize,
                SyntaxKind::ClOpen => 3isize,
                SyntaxKind::Colon => 3isize,
                SyntaxKind::Comma => 3isize,
                SyntaxKind::CurlyClose => 3isize,
                SyntaxKind::CurlyOpen => 3isize,
                SyntaxKind::Datatype => 3isize,
                SyntaxKind::GraphToken => 3isize,
                SyntaxKind::PrefixToken => 3isize,
                SyntaxKind::SparqlBaseToken => 3isize,
                SyntaxKind::SparqlPrefixToken => 3isize,
                SyntaxKind::SqClose => 3isize,
                SyntaxKind::Stop => 3isize,
                _ => 0,
            },
            (SyntaxKind::TriplesBlock, 4usize, _) => match terminal {
                SyntaxKind::Stop => 0,
                _ => 3isize,
            },
            (SyntaxKind::TriplesBlock, 5usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::TriplesBlock, 3usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::TriplesOrGraph, 2usize, _) => match terminal {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::TriplesOrGraph, 1usize, _) => match terminal {
                SyntaxKind::Stop => 0,
                _ => 3isize,
            },
            (SyntaxKind::TriplesOrGraph, 3usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Verb, 2usize, _) => match terminal {
                SyntaxKind::Iriref | SyntaxKind::PnameLn | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Verb, 1usize, _) => match terminal {
                SyntaxKind::Alit
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs => 0,
                _ => 1isize,
            },
            (SyntaxKind::Verb, 3usize, _) => match terminal {
                SyntaxKind::Alit => 0,
                _ => 1isize,
            },
            (SyntaxKind::WrappedGraph, 1usize, _) => match terminal {
                SyntaxKind::CurlyClose => 0,
                _ => 2isize,
            },
            (SyntaxKind::WrappedGraph, 4usize, _) => match terminal {
                SyntaxKind::CurlyOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::WrappedGraph, 3usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::ClOpen
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            (SyntaxKind::WrappedGraph, 2usize, _) => match terminal {
                SyntaxKind::Anon
                | SyntaxKind::BlankNodeLabel
                | SyntaxKind::Iriref
                | SyntaxKind::PnameLn
                | SyntaxKind::PnameNs
                | SyntaxKind::ClOpen
                | SyntaxKind::CurlyClose
                | SyntaxKind::SqOpen => 0,
                _ => 1isize,
            },
            _ => 0,
        }
    }
    impl crate::a_star::ParserTrait for Rule {
        type Kind = SyntaxKind;
        fn step(
            &self,
            element: &crate::a_star::Element<Self>,
            state: &mut crate::a_star::AStar<Self>,
        ) {
            match (self.kind, self.state) {
                (SyntaxKind::TrigDoc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TrigDoc, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                }
                (SyntaxKind::TrigDoc, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::TrigDoc, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Directive,
                                state: 1usize,
                            }),
                        SyntaxKind::Directive,
                    );
                }
                (SyntaxKind::TrigDoc, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Block,
                                state: 1usize,
                            }),
                        SyntaxKind::Block,
                    );
                }
                (SyntaxKind::Block, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Block, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 6usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 7usize,
                    }));
                }
                (SyntaxKind::Block, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphBlock,
                                state: 2usize,
                            }),
                        SyntaxKind::GraphBlock,
                    );
                }
                (SyntaxKind::Block, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Triples2,
                                state: 2usize,
                            }),
                        SyntaxKind::Triples2,
                    );
                }
                (SyntaxKind::Block, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WrappedGraph,
                                state: 4usize,
                            }),
                        SyntaxKind::WrappedGraph,
                    );
                }
                (SyntaxKind::Block, 5usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::LabelOrSubject,
                                state: 1usize,
                            }),
                        SyntaxKind::LabelOrSubject,
                    );
                }
                (SyntaxKind::Block, 6usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::GraphToken);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 5usize,
                        }));
                    }
                }
                (SyntaxKind::Block, 7usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesOrGraph,
                                state: 3usize,
                            }),
                        SyntaxKind::TriplesOrGraph,
                    );
                }
                (SyntaxKind::GraphBlock, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphBlock, 1usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WrappedGraph,
                                state: 4usize,
                            }),
                        SyntaxKind::WrappedGraph,
                    );
                }
                (SyntaxKind::GraphBlock, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::GraphBlock, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::LabelOrSubject,
                                state: 1usize,
                            }),
                        SyntaxKind::LabelOrSubject,
                    );
                }
                (SyntaxKind::TriplesOrGraph, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TriplesOrGraph, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::TriplesOrGraph, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PredicateObjectList,
                                state: 7usize,
                            }),
                        SyntaxKind::PredicateObjectList,
                    );
                }
                (SyntaxKind::TriplesOrGraph, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::LabelOrSubject,
                                state: 1usize,
                            }),
                        SyntaxKind::LabelOrSubject,
                    );
                }
                (SyntaxKind::Triples2, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Triples2, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::Triples2, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 7usize,
                    }));
                }
                (SyntaxKind::Triples2, 3usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::Triples2, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PredicateObjectList,
                                state: 7usize,
                            }),
                        SyntaxKind::PredicateObjectList,
                    );
                }
                (SyntaxKind::Triples2, 5usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNodePropertyList,
                                state: 3usize,
                            }),
                        SyntaxKind::BlankNodePropertyList,
                    );
                }
                (SyntaxKind::Triples2, 6usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::Triples2, 7usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Collection,
                                state: 4usize,
                            }),
                        SyntaxKind::Collection,
                    );
                }
                (SyntaxKind::WrappedGraph, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::WrappedGraph, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::CurlyClose);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::WrappedGraph, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::WrappedGraph, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesBlock,
                                state: 5usize,
                            }),
                        SyntaxKind::TriplesBlock,
                    );
                }
                (SyntaxKind::WrappedGraph, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::CurlyOpen);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }));
                    }
                }
                (SyntaxKind::TriplesBlock, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TriplesBlock, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::TriplesBlock, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::TriplesBlock, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesBlock,
                                state: 5usize,
                            }),
                        SyntaxKind::TriplesBlock,
                    );
                }
                (SyntaxKind::TriplesBlock, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }));
                    }
                }
                (SyntaxKind::TriplesBlock, 5usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Triples,
                                state: 1usize,
                            }),
                        SyntaxKind::Triples,
                    );
                }
                (SyntaxKind::LabelOrSubject, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::LabelOrSubject, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::LabelOrSubject, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                        SyntaxKind::Iri,
                    );
                }
                (SyntaxKind::LabelOrSubject, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNode,
                                state: 1usize,
                            }),
                        SyntaxKind::BlankNode,
                    );
                }
                (SyntaxKind::Directive, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Directive, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                }
                (SyntaxKind::Directive, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PrefixId,
                                state: 4usize,
                            }),
                        SyntaxKind::PrefixId,
                    );
                }
                (SyntaxKind::Directive, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Base,
                                state: 3usize,
                            }),
                        SyntaxKind::Base,
                    );
                }
                (SyntaxKind::Directive, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SparqlPrefix,
                                state: 3usize,
                            }),
                        SyntaxKind::SparqlPrefix,
                    );
                }
                (SyntaxKind::Directive, 5usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SparqlBase,
                                state: 2usize,
                            }),
                        SyntaxKind::SparqlBase,
                    );
                }
                (SyntaxKind::PrefixId, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixId, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::PrefixId, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iriref);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 1usize,
                        }));
                    }
                }
                (SyntaxKind::PrefixId, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::PnameNs);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }));
                    }
                }
                (SyntaxKind::PrefixId, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::PrefixToken);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }));
                    }
                }
                (SyntaxKind::Base, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Base, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::Base, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iriref);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 1usize,
                        }));
                    }
                }
                (SyntaxKind::Base, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BaseToken);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }));
                    }
                }
                (SyntaxKind::SparqlPrefix, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SparqlPrefix, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iriref);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::SparqlPrefix, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::PnameNs);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 1usize,
                        }));
                    }
                }
                (SyntaxKind::SparqlPrefix, 3usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SparqlPrefixToken);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }));
                    }
                }
                (SyntaxKind::SparqlBase, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SparqlBase, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iriref);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::SparqlBase, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SparqlBaseToken);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 1usize,
                        }));
                    }
                }
                (SyntaxKind::Triples, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Triples, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 6usize,
                    }));
                }
                (SyntaxKind::Triples, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PredicateObjectList,
                                state: 7usize,
                            }),
                        SyntaxKind::PredicateObjectList,
                    );
                }
                (SyntaxKind::Triples, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Subject,
                                state: 1usize,
                            }),
                        SyntaxKind::Subject,
                    );
                }
                (SyntaxKind::Triples, 4usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                }
                (SyntaxKind::Triples, 5usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                }
                (SyntaxKind::Triples, 6usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNodePropertyList,
                                state: 3usize,
                            }),
                        SyntaxKind::BlankNodePropertyList,
                    );
                }
                (SyntaxKind::PredicateObjectList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PredicateObjectList, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                }
                (SyntaxKind::PredicateObjectList, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::PredicateObjectList, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ObjectList,
                                state: 4usize,
                            }),
                        SyntaxKind::ObjectList,
                    );
                }
                (SyntaxKind::PredicateObjectList, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Verb,
                                state: 1usize,
                            }),
                        SyntaxKind::Verb,
                    );
                }
                (SyntaxKind::PredicateObjectList, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Colon);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }));
                    }
                }
                (SyntaxKind::PredicateObjectList, 6usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::PredicateObjectList, 7usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Verb,
                                state: 1usize,
                            }),
                        SyntaxKind::Verb,
                    );
                }
                (SyntaxKind::ObjectList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ObjectList, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                }
                (SyntaxKind::ObjectList, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Object,
                                state: 1usize,
                            }),
                        SyntaxKind::Object,
                    );
                }
                (SyntaxKind::ObjectList, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }));
                    }
                }
                (SyntaxKind::ObjectList, 4usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                }
                (SyntaxKind::Verb, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Verb, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }),
                        SyntaxKind::Alit,
                    );
                }
                (SyntaxKind::Verb, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Predicate,
                                state: 1usize,
                            }),
                        SyntaxKind::Predicate,
                    );
                }
                (SyntaxKind::Verb, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Alit);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::Subject, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Subject, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::Subject, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                        SyntaxKind::Iri,
                    );
                }
                (SyntaxKind::Subject, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Blank,
                                state: 1usize,
                            }),
                        SyntaxKind::Blank,
                    );
                }
                (SyntaxKind::Predicate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Predicate, 1usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                        SyntaxKind::Iri,
                    );
                }
                (SyntaxKind::Object, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Object, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                }
                (SyntaxKind::Object, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                        SyntaxKind::Iri,
                    );
                }
                (SyntaxKind::Object, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Blank,
                                state: 1usize,
                            }),
                        SyntaxKind::Blank,
                    );
                }
                (SyntaxKind::Object, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNodePropertyList,
                                state: 3usize,
                            }),
                        SyntaxKind::BlankNodePropertyList,
                    );
                }
                (SyntaxKind::Object, 5usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Literal,
                                state: 1usize,
                            }),
                        SyntaxKind::Literal,
                    );
                }
                (SyntaxKind::Literal, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Literal, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::Literal, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Rdfliteral,
                                state: 6usize,
                            }),
                        SyntaxKind::Rdfliteral,
                    );
                }
                (SyntaxKind::Literal, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NumericLiteral,
                                state: 1usize,
                            }),
                        SyntaxKind::NumericLiteral,
                    );
                }
                (SyntaxKind::Literal, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BooleanLiteral,
                                state: 1usize,
                            }),
                        SyntaxKind::BooleanLiteral,
                    );
                }
                (SyntaxKind::Blank, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Blank, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::Blank, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNode,
                                state: 1usize,
                            }),
                        SyntaxKind::BlankNode,
                    );
                }
                (SyntaxKind::Blank, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Collection,
                                state: 4usize,
                            }),
                        SyntaxKind::Collection,
                    );
                }
                (SyntaxKind::BlankNodePropertyList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BlankNodePropertyList, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::SqClose);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::BlankNodePropertyList, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PredicateObjectList,
                                state: 7usize,
                            }),
                        SyntaxKind::PredicateObjectList,
                    );
                }
                (SyntaxKind::BlankNodePropertyList, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::SqOpen);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }));
                    }
                }
                (SyntaxKind::Collection, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Collection, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ClClose);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::Collection, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                }
                (SyntaxKind::Collection, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Object,
                                state: 1usize,
                            }),
                        SyntaxKind::Object,
                    );
                }
                (SyntaxKind::Collection, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ClOpen);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }));
                    }
                }
                (SyntaxKind::NumericLiteral, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NumericLiteral, 1usize) => {
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }),
                        SyntaxKind::Integer,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }),
                        SyntaxKind::Decimal,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 4usize,
                        }),
                        SyntaxKind::Double,
                    );
                }
                (SyntaxKind::NumericLiteral, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Integer);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::NumericLiteral, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Decimal);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::NumericLiteral, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Double);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::Rdfliteral, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Rdfliteral, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                }
                (SyntaxKind::Rdfliteral, 2usize) => {
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }),
                        SyntaxKind::Langtag,
                    );
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                }
                (SyntaxKind::Rdfliteral, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Langtag);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::Rdfliteral, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                        SyntaxKind::Iri,
                    );
                }
                (SyntaxKind::Rdfliteral, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Datatype);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 4usize,
                        }));
                    }
                }
                (SyntaxKind::Rdfliteral, 6usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MyString,
                                state: 1usize,
                            }),
                        SyntaxKind::MyString,
                    );
                }
                (SyntaxKind::BooleanLiteral, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BooleanLiteral, 1usize) => {
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }),
                        SyntaxKind::TrueLit,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }),
                        SyntaxKind::FalseLit,
                    );
                }
                (SyntaxKind::BooleanLiteral, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::TrueLit);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::BooleanLiteral, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::FalseLit);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::MyString, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::MyString, 1usize) => {
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }),
                        SyntaxKind::StringLiteralQuote,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }),
                        SyntaxKind::StringLiteralSingleQuote,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 4usize,
                        }),
                        SyntaxKind::StringLiteralLongSingleQuote,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 5usize,
                        }),
                        SyntaxKind::StringLiteralLongQuote,
                    );
                }
                (SyntaxKind::MyString, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StringLiteralQuote);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::MyString, 3usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StringLiteralSingleQuote);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::MyString, 4usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StringLiteralLongSingleQuote);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::MyString, 5usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StringLiteralLongQuote);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::Iri, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Iri, 1usize) => {
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }),
                        SyntaxKind::Iriref,
                    );
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::Iri, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iriref);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::Iri, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PrefixedName,
                                state: 1usize,
                            }),
                        SyntaxKind::PrefixedName,
                    );
                }
                (SyntaxKind::PrefixedName, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixedName, 1usize) => {
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }),
                        SyntaxKind::PnameLn,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }),
                        SyntaxKind::PnameNs,
                    );
                }
                (SyntaxKind::PrefixedName, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::PnameLn);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::PrefixedName, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::PnameNs);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::BlankNode, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BlankNode, 1usize) => {
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }),
                        SyntaxKind::BlankNodeLabel,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }),
                        SyntaxKind::Anon,
                    );
                }
                (SyntaxKind::BlankNode, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BlankNodeLabel);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::BlankNode, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Anon);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 0usize,
                        }));
                    }
                }
                (SyntaxKind::ClOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::ClOpen);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ClClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::ClClose);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Comma, _) => {
                    let added = state.expect_as(element, SyntaxKind::Comma);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Stop, _) => {
                    let added = state.expect_as(element, SyntaxKind::Stop);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Colon, _) => {
                    let added = state.expect_as(element, SyntaxKind::Colon);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BaseToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::BaseToken);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::PrefixToken);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SparqlBaseToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::SparqlBaseToken);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::GraphToken);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SparqlPrefixToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::SparqlPrefixToken);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SqOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::SqOpen);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SqClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::SqClose);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Datatype, _) => {
                    let added = state.expect_as(element, SyntaxKind::Datatype);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Alit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Alit);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::FalseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FalseLit);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TrueLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::TrueLit);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::CurlyOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::CurlyOpen);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::CurlyClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::CurlyClose);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Anon, _) => {
                    let added = state.expect_as(element, SyntaxKind::Anon);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BlankNodeLabel, _) => {
                    let added = state.expect_as(element, SyntaxKind::BlankNodeLabel);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Decimal, _) => {
                    let added = state.expect_as(element, SyntaxKind::Decimal);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Double, _) => {
                    let added = state.expect_as(element, SyntaxKind::Double);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Integer, _) => {
                    let added = state.expect_as(element, SyntaxKind::Integer);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Iriref, _) => {
                    let added = state.expect_as(element, SyntaxKind::Iriref);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Langtag, _) => {
                    let added = state.expect_as(element, SyntaxKind::Langtag);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PnameLn, _) => {
                    let added = state.expect_as(element, SyntaxKind::PnameLn);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PnameNs, _) => {
                    let added = state.expect_as(element, SyntaxKind::PnameNs);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteralLongQuote, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralLongQuote);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteralLongSingleQuote, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralLongSingleQuote);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteralQuote, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralQuote);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteralSingleQuote, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralSingleQuote);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                _ => panic!("Aaaah unexpected state {:?}", self),
            }
        }
        fn at(&self) -> usize {
            self.state
        }
        fn element_kind(&self) -> SyntaxKind {
            self.kind
        }
        fn state_dist(&self, terminal: &SyntaxKind) -> isize {
            state_dist(self.kind, self.state, *terminal)
        }
    }
}
pub use definitions::*;
impl TokenTrait for SyntaxKind {
    const ERROR: Self = SyntaxKind::Error;
    const ROOT: Self = SyntaxKind::ROOT;
    fn branch(&self) -> u32 {
        *self as u32
    }
    fn skips(&self) -> bool {
        match self {
            SyntaxKind::WhiteSpace => true,
            SyntaxKind::Error => true,
            SyntaxKind::Comment => true,
            _ => false,
        }
    }
    fn starting_tokens(&self) -> &'static [SyntaxKind] {
        &[]
    }
    fn min_error_for_token(&self, tok: &SyntaxKind) -> isize {
        min_error_for_token(*self, *tok)
    }
    fn ending_tokens(&self) -> &'static [SyntaxKind] {
        &[]
    }
    fn max_error_value(&self) -> isize {
        match self {
            SyntaxKind::ClOpen => 2isize,
            SyntaxKind::ClClose => 2isize,
            SyntaxKind::Stop => 3isize,
            SyntaxKind::BaseToken => 100isize,
            SyntaxKind::PrefixToken => 100isize,
            SyntaxKind::SparqlBaseToken => 100isize,
            SyntaxKind::GraphToken => 3isize,
            SyntaxKind::SparqlPrefixToken => 100isize,
            SyntaxKind::SqOpen => 2isize,
            SyntaxKind::SqClose => 2isize,
            SyntaxKind::CurlyOpen => 2isize,
            SyntaxKind::CurlyClose => 2isize,
            _ => 1isize,
        }
    }
    fn bracket_delta(&self) -> i8 {
        match self {
            SyntaxKind::SqOpen => 1,
            SyntaxKind::ClOpen => 1,
            SyntaxKind::CurlyOpen => 1,
            SyntaxKind::SqClose => -1,
            SyntaxKind::ClClose => -1,
            SyntaxKind::CurlyClose => -1,
            _ => 0,
        }
    }
    fn min_completion_cost(&self) -> isize {
        match self {
            SyntaxKind::Base => 104isize,
            SyntaxKind::BlankNodePropertyList => 6isize,
            SyntaxKind::Block => 4isize,
            SyntaxKind::Collection => 4isize,
            SyntaxKind::Directive => 101isize,
            SyntaxKind::GraphBlock => 4isize,
            SyntaxKind::PredicateObjectList => 2isize,
            SyntaxKind::PrefixId => 105isize,
            SyntaxKind::SparqlBase => 101isize,
            SyntaxKind::SparqlPrefix => 102isize,
            SyntaxKind::Triples => 3isize,
            SyntaxKind::Triples2 => 9isize,
            SyntaxKind::TriplesBlock => 3isize,
            SyntaxKind::TriplesOrGraph => 6isize,
            SyntaxKind::WrappedGraph => 4isize,
            _ => Self::max_error_value(self),
        }
    }
}
