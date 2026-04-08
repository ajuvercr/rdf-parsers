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
    BlankNodePropertyList,
    BlankNodePropertyList2,
    Collection,
    Directive,
    Iri,
    Literal,
    Object,
    ObjectList,
    Predicate,
    PredicateObjectList,
    PrefixId,
    SparqlBase,
    SparqlPrefix,
    Statement,
    Subject,
    Triples,
    TurtleDoc,
    Verb,
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
                SyntaxKind::TurtleDoc => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Statement => Rule {
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
                SyntaxKind::SparqlBase => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::SparqlPrefix => Rule {
                    kind,
                    state: 3usize,
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
                SyntaxKind::BlankNodePropertyList2 => Rule {
                    kind,
                    state: 4usize,
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
                SyntaxKind::SparqlPrefixToken => Rule { kind, state: 0 },
                SyntaxKind::SqOpen => Rule { kind, state: 0 },
                SyntaxKind::SqClose => Rule { kind, state: 0 },
                SyntaxKind::Datatype => Rule { kind, state: 0 },
                SyntaxKind::Alit => Rule { kind, state: 0 },
                SyntaxKind::FalseLit => Rule { kind, state: 0 },
                SyntaxKind::TrueLit => Rule { kind, state: 0 },
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
            SyntaxKind::BlankNode => &[SyntaxKind::SqOpen],
            SyntaxKind::BooleanLiteral => &[SyntaxKind::FalseLit, SyntaxKind::TrueLit],
            SyntaxKind::NumericLiteral => &[],
            SyntaxKind::PrefixedName => &[],
            SyntaxKind::Rdfliteral => &[],
            SyntaxKind::MyString => &[],
            SyntaxKind::Base => &[SyntaxKind::BaseToken],
            SyntaxKind::BlankNodePropertyList => &[SyntaxKind::SqOpen],
            SyntaxKind::BlankNodePropertyList2 => &[SyntaxKind::SqOpen],
            SyntaxKind::Collection => &[SyntaxKind::ClOpen],
            SyntaxKind::Directive => &[
                SyntaxKind::BaseToken,
                SyntaxKind::PrefixToken,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SparqlPrefixToken,
            ],
            SyntaxKind::Iri => &[],
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
            SyntaxKind::Statement => &[
                SyntaxKind::BaseToken,
                SyntaxKind::ClOpen,
                SyntaxKind::PrefixToken,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::Subject => &[SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::Triples => &[SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::TurtleDoc => &[
                SyntaxKind::BaseToken,
                SyntaxKind::ClOpen,
                SyntaxKind::PrefixToken,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::Verb => &[SyntaxKind::Alit],
            SyntaxKind::ClOpen => &[SyntaxKind::ClOpen],
            SyntaxKind::ClClose => &[SyntaxKind::ClClose],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::BaseToken => &[SyntaxKind::BaseToken],
            SyntaxKind::PrefixToken => &[SyntaxKind::PrefixToken],
            SyntaxKind::SparqlBaseToken => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::SparqlPrefixToken => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::Alit => &[SyntaxKind::Alit],
            SyntaxKind::FalseLit => &[SyntaxKind::FalseLit],
            SyntaxKind::TrueLit => &[SyntaxKind::TrueLit],
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
    #[doc = r" Returns the set of all terminals that can be consumed *anywhere*"]
    #[doc = r" in a parse of `kind` — including inside sub-rules at any depth."]
    #[doc = r#" An empty slice means "unknown / no pruning"."#]
    pub fn all_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::BlankNode => &[
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::BooleanLiteral => &[SyntaxKind::FalseLit, SyntaxKind::TrueLit],
            SyntaxKind::NumericLiteral => {
                &[SyntaxKind::Decimal, SyntaxKind::Double, SyntaxKind::Integer]
            }
            SyntaxKind::PrefixedName => &[SyntaxKind::PnameLn, SyntaxKind::PnameNs],
            SyntaxKind::Rdfliteral => &[
                SyntaxKind::Datatype,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
            ],
            SyntaxKind::MyString => &[
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
            ],
            SyntaxKind::Base => &[SyntaxKind::BaseToken, SyntaxKind::Iriref, SyntaxKind::Stop],
            SyntaxKind::BlankNodePropertyList => &[
                SyntaxKind::Alit,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::BlankNodePropertyList2 => &[
                SyntaxKind::Alit,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Collection => &[
                SyntaxKind::Alit,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Directive => &[
                SyntaxKind::BaseToken,
                SyntaxKind::Iriref,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixToken,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::Stop,
            ],
            SyntaxKind::Iri => &[SyntaxKind::Iriref, SyntaxKind::PnameLn, SyntaxKind::PnameNs],
            SyntaxKind::Literal => &[
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Object => &[
                SyntaxKind::Alit,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::ObjectList => &[
                SyntaxKind::Alit,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Predicate => {
                &[SyntaxKind::Iriref, SyntaxKind::PnameLn, SyntaxKind::PnameNs]
            }
            SyntaxKind::PredicateObjectList => &[
                SyntaxKind::Alit,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::PrefixId => &[
                SyntaxKind::Iriref,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixToken,
                SyntaxKind::Stop,
            ],
            SyntaxKind::SparqlBase => &[SyntaxKind::Iriref, SyntaxKind::SparqlBaseToken],
            SyntaxKind::SparqlPrefix => &[
                SyntaxKind::Iriref,
                SyntaxKind::PnameNs,
                SyntaxKind::SparqlPrefixToken,
            ],
            SyntaxKind::Statement => &[
                SyntaxKind::Alit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixToken,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Subject => &[
                SyntaxKind::Alit,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Triples => &[
                SyntaxKind::Alit,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::TurtleDoc => &[
                SyntaxKind::Alit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::FalseLit,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixToken,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::StringLiteralLongQuote,
                SyntaxKind::StringLiteralLongSingleQuote,
                SyntaxKind::StringLiteralQuote,
                SyntaxKind::StringLiteralSingleQuote,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Verb => &[
                SyntaxKind::Alit,
                SyntaxKind::Iriref,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
            ],
            SyntaxKind::ClOpen => &[SyntaxKind::ClOpen],
            SyntaxKind::ClClose => &[SyntaxKind::ClClose],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::BaseToken => &[SyntaxKind::BaseToken],
            SyntaxKind::PrefixToken => &[SyntaxKind::PrefixToken],
            SyntaxKind::SparqlBaseToken => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::SparqlPrefixToken => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::Alit => &[SyntaxKind::Alit],
            SyntaxKind::FalseLit => &[SyntaxKind::FalseLit],
            SyntaxKind::TrueLit => &[SyntaxKind::TrueLit],
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
    impl crate::a_star::ParserTrait for Rule {
        type Kind = SyntaxKind;
        fn step(
            &self,
            element: &crate::a_star::Element<Self>,
            state: &mut crate::a_star::AStar<Self>,
        ) {
            match (self.kind, self.state) {
                (SyntaxKind::TurtleDoc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TurtleDoc, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                }
                (SyntaxKind::TurtleDoc, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Statement,
                                state: 1usize,
                            }),
                        SyntaxKind::Statement,
                    );
                }
                (SyntaxKind::Statement, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Statement, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::Statement, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Directive,
                                state: 1usize,
                            }),
                        SyntaxKind::Directive,
                    );
                }
                (SyntaxKind::Statement, 3usize) => {
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
                (SyntaxKind::Statement, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Triples,
                                state: 1usize,
                            }),
                        SyntaxKind::Triples,
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
                (SyntaxKind::Triples, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Triples, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 6usize,
                    }));
                }
                (SyntaxKind::Triples, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::Triples, 3usize) => {
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
                (SyntaxKind::Triples, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNodePropertyList,
                                state: 3usize,
                            }),
                        SyntaxKind::BlankNodePropertyList,
                    );
                }
                (SyntaxKind::Triples, 5usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::Triples, 6usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Subject,
                                state: 1usize,
                            }),
                        SyntaxKind::Subject,
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
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
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
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
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
                                kind: SyntaxKind::BlankNode,
                                state: 1usize,
                            }),
                        SyntaxKind::BlankNode,
                    );
                }
                (SyntaxKind::Subject, 4usize) => {
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
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 6usize,
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
                (SyntaxKind::Object, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNodePropertyList2,
                                state: 4usize,
                            }),
                        SyntaxKind::BlankNodePropertyList2,
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
                                kind: SyntaxKind::Collection,
                                state: 4usize,
                            }),
                        SyntaxKind::Collection,
                    );
                }
                (SyntaxKind::Object, 6usize) => {
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
                (SyntaxKind::BlankNodePropertyList2, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BlankNodePropertyList2, 1usize) => {
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
                (SyntaxKind::BlankNodePropertyList2, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::BlankNodePropertyList2, 3usize) => {
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
                (SyntaxKind::BlankNodePropertyList2, 4usize) => {
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
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
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
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
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
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
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
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
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
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
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
                (SyntaxKind::BlankNode, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::SqOpen);
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
    fn all_reachable_tokens(&self) -> &'static [SyntaxKind] {
        all_tokens(*self)
    }
    fn ending_tokens(&self) -> &'static [SyntaxKind] {
        &[]
    }
    fn max_error_value(&self) -> isize {
        match self {
            SyntaxKind::ClOpen => 2isize,
            SyntaxKind::ClClose => 2isize,
            SyntaxKind::Comma => 2isize,
            SyntaxKind::Stop => 3isize,
            SyntaxKind::Colon => 2isize,
            SyntaxKind::BaseToken => 100isize,
            SyntaxKind::PrefixToken => 100isize,
            SyntaxKind::SparqlBaseToken => 100isize,
            SyntaxKind::SparqlPrefixToken => 100isize,
            SyntaxKind::SqOpen => 2isize,
            SyntaxKind::SqClose => 2isize,
            _ => 1isize,
        }
    }
    fn bracket_delta(&self) -> i8 {
        match self {
            SyntaxKind::SqOpen => 1,
            SyntaxKind::ClOpen => 1,
            SyntaxKind::SqClose => -1,
            SyntaxKind::ClClose => -1,
            _ => 0,
        }
    }
}
