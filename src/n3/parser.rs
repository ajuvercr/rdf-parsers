use crate::TokenTrait;
pub type SyntaxNode = rowan::SyntaxNode<Lang>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, logos :: Logos)]
# [logos (subpattern BOOLEAN_LITERAL = r#"(true|false)"#)]
# [logos (subpattern ECHAR = r#"\\[tbnrf\\"']"#)]
# [logos (subpattern HEX = r#"([0-9]|[A-F]|[a-f])"#)]
# [logos (subpattern UCHAR = r#"(\\u(?&HEX)(?&HEX)(?&HEX)(?&HEX)|\\U(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX))"#)]
# [logos (subpattern STRING_LITERAL_LONG_QUOTE = r#""""((("|""))?([^"\\]|(?&ECHAR)|(?&UCHAR)))*""""#)]
# [logos (subpattern STRING_LITERAL_LONG_SINGLE_QUOTE = r#"'''((('|''))?([^'\\]|(?&ECHAR)|(?&UCHAR)))*'''"#)]
# [logos (subpattern STRING_LITERAL_QUOTE = r#""(([^>"\x{5C}\x{0A}\x{0D}]|(?&ECHAR)|(?&UCHAR)))*""#)]
# [logos (subpattern STRING_LITERAL_SINGLE_QUOTE = r#"'(([^\x{27}\x{5C}\x{0A}\x{0D}]|(?&ECHAR)|(?&UCHAR)))*'"#)]
# [logos (subpattern STRING = r#"((?&STRING_LITERAL_LONG_SINGLE_QUOTE)|(?&STRING_LITERAL_LONG_QUOTE)|(?&STRING_LITERAL_QUOTE)|(?&STRING_LITERAL_SINGLE_QUOTE))"#)]
# [logos (subpattern WS = r#"(\u0020|\u0009|\u000D|\u000A)"#)]
# [logos (subpattern IPLSTART = r#"\[((?&WS))*id"#)]
# [logos (subpattern IRIREF = r#"<([^\x{00}-\x{20}<>"{}|^`\\])*>"#)]
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
# [logos (subpattern INTEGER = r#"([0-9])+"#)]
# [logos (subpattern DECIMAL = r#"([0-9])*\.([0-9])+"#)]
# [logos (subpattern EXPONENT = r#"[eE]([+-])?([0-9])+"#)]
# [logos (subpattern DOUBLE = r#"(([0-9])+\.([0-9])*(?&EXPONENT)|\.([0-9])+(?&EXPONENT)|([0-9])+(?&EXPONENT))"#)]
# [logos (subpattern ANON = r#"\[((?&WS))*\]"#)]
# [logos (subpattern QUICK_VAR_NAME = r#"\?(?&PN_LOCAL)"#)]
#[repr(u16)]
pub enum SyntaxKind {
    Eof = 0,
    #[regex(r"[ \t\n]+")]
    WhiteSpace,
    #[regex(r"#[^\n]+", allow_greedy = true)]
    Comment,
    #[doc = r" producings"]
    Base,
    BlankNode,
    BlankNodePropertyList,
    Collection,
    Expression,
    Formula,
    FormulaContent,
    Iri,
    IriPropertyList,
    Literal,
    N3Directive,
    N3Doc,
    N3Statement,
    NumericLiteral,
    Object,
    ObjectList,
    Path,
    PathItem,
    Predicate,
    PredicateObjectList,
    PrefixId,
    PrefixedName,
    QuickVar,
    RdfLiteral,
    SparqlBase,
    SparqlDirective,
    SparqlPrefix,
    Subject,
    Triples,
    Verb,
    #[doc = r" terminals"]
    #[token("!", ignore(case))]
    Bang,
    #[token("(", ignore(case))]
    BrOpen,
    #[token(")", ignore(case))]
    BrClose,
    #[token(",", ignore(case))]
    Comma,
    #[token(".", ignore(case))]
    Stop,
    #[token(";", ignore(case))]
    Colon,
    #[token("<-", ignore(case))]
    ArrowLeft,
    #[token("<=", ignore(case))]
    ImplyLeft,
    #[token("=", ignore(case))]
    Eq,
    #[token("=>", ignore(case))]
    ImplyRight,
    #[token("@base", ignore(case))]
    BaseToken,
    #[token("@prefix", ignore(case))]
    PrefixToken,
    #[token("BASE")]
    BaseLit,
    #[token("PREFIX")]
    PrefixLit,
    #[token("[", ignore(case))]
    SqOpen,
    #[token("]", ignore(case))]
    SqClose,
    #[token("^", ignore(case))]
    Hat,
    #[token("^^", ignore(case))]
    Datatype,
    #[token("a", ignore(case))]
    Alit,
    #[token("has", ignore(case))]
    HasLit,
    #[token("is", ignore(case))]
    IsLit,
    #[token("of", ignore(case))]
    OfLit,
    #[token("{", ignore(case))]
    ClOpen,
    #[token("}", ignore(case))]
    ClClose,
    #[regex("(?&ANON)")]
    Anon,
    #[regex("(?&BLANK_NODE_LABEL)")]
    BlankNodeLabel,
    #[regex("(?&BOOLEAN_LITERAL)")]
    BooleanLiteral,
    #[regex("(?&DECIMAL)")]
    Decimal,
    #[regex("(?&DOUBLE)")]
    Double,
    #[regex("(?&INTEGER)")]
    Integer,
    #[regex("(?&IPLSTART)")]
    Iplstart,
    #[regex("(?&IRIREF)")]
    Iriref,
    #[regex("(?&LANGTAG)")]
    Langtag,
    #[regex("(?&PNAME_LN)")]
    PnameLn,
    #[regex("(?&PNAME_NS)")]
    PnameNs,
    #[regex("(?&QUICK_VAR_NAME)")]
    QuickVarName,
    #[regex("(?&STRING)")]
    String,
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
                SyntaxKind::N3Doc => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::N3Statement => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::N3Directive => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::SparqlDirective => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::SparqlBase => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::SparqlPrefix => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::PrefixId => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::Base => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::Triples => Rule {
                    kind,
                    state: 3usize,
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
                SyntaxKind::Expression => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Path => Rule {
                    kind,
                    state: 6usize,
                },
                SyntaxKind::PathItem => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Literal => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::BlankNodePropertyList => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::IriPropertyList => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::Collection => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::Formula => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::FormulaContent => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::NumericLiteral => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::RdfLiteral => Rule {
                    kind,
                    state: 6usize,
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
                SyntaxKind::QuickVar => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Bang => Rule { kind, state: 0 },
                SyntaxKind::BrOpen => Rule { kind, state: 0 },
                SyntaxKind::BrClose => Rule { kind, state: 0 },
                SyntaxKind::Comma => Rule { kind, state: 0 },
                SyntaxKind::Stop => Rule { kind, state: 0 },
                SyntaxKind::Colon => Rule { kind, state: 0 },
                SyntaxKind::ArrowLeft => Rule { kind, state: 0 },
                SyntaxKind::ImplyLeft => Rule { kind, state: 0 },
                SyntaxKind::Eq => Rule { kind, state: 0 },
                SyntaxKind::ImplyRight => Rule { kind, state: 0 },
                SyntaxKind::BaseToken => Rule { kind, state: 0 },
                SyntaxKind::PrefixToken => Rule { kind, state: 0 },
                SyntaxKind::BaseLit => Rule { kind, state: 0 },
                SyntaxKind::PrefixLit => Rule { kind, state: 0 },
                SyntaxKind::SqOpen => Rule { kind, state: 0 },
                SyntaxKind::SqClose => Rule { kind, state: 0 },
                SyntaxKind::Hat => Rule { kind, state: 0 },
                SyntaxKind::Datatype => Rule { kind, state: 0 },
                SyntaxKind::Alit => Rule { kind, state: 0 },
                SyntaxKind::HasLit => Rule { kind, state: 0 },
                SyntaxKind::IsLit => Rule { kind, state: 0 },
                SyntaxKind::OfLit => Rule { kind, state: 0 },
                SyntaxKind::ClOpen => Rule { kind, state: 0 },
                SyntaxKind::ClClose => Rule { kind, state: 0 },
                SyntaxKind::Anon => Rule { kind, state: 0 },
                SyntaxKind::BlankNodeLabel => Rule { kind, state: 0 },
                SyntaxKind::BooleanLiteral => Rule { kind, state: 0 },
                SyntaxKind::Decimal => Rule { kind, state: 0 },
                SyntaxKind::Double => Rule { kind, state: 0 },
                SyntaxKind::Integer => Rule { kind, state: 0 },
                SyntaxKind::Iplstart => Rule { kind, state: 0 },
                SyntaxKind::Iriref => Rule { kind, state: 0 },
                SyntaxKind::Langtag => Rule { kind, state: 0 },
                SyntaxKind::PnameLn => Rule { kind, state: 0 },
                SyntaxKind::PnameNs => Rule { kind, state: 0 },
                SyntaxKind::QuickVarName => Rule { kind, state: 0 },
                SyntaxKind::String => Rule { kind, state: 0 },
                _ => panic!("Unknown rule kind {:?}", kind),
            }
        }
    }
    pub fn first_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::Base => &[SyntaxKind::BaseToken],
            SyntaxKind::BlankNode => &[],
            SyntaxKind::BlankNodePropertyList => &[SyntaxKind::SqOpen],
            SyntaxKind::Collection => &[SyntaxKind::BrOpen],
            SyntaxKind::Expression => &[SyntaxKind::BrOpen, SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::Formula => &[SyntaxKind::ClOpen],
            SyntaxKind::FormulaContent => &[
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BrOpen,
                SyntaxKind::ClOpen,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::Iri => &[],
            SyntaxKind::IriPropertyList => &[],
            SyntaxKind::Literal => &[],
            SyntaxKind::N3Directive => &[SyntaxKind::BaseToken, SyntaxKind::PrefixToken],
            SyntaxKind::N3Doc => &[
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BrOpen,
                SyntaxKind::ClOpen,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::N3Statement => &[
                SyntaxKind::BaseToken,
                SyntaxKind::BrOpen,
                SyntaxKind::ClOpen,
                SyntaxKind::PrefixToken,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::NumericLiteral => &[],
            SyntaxKind::Object => &[SyntaxKind::BrOpen, SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::ObjectList => &[SyntaxKind::BrOpen, SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::Path => &[SyntaxKind::BrOpen, SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::PathItem => &[SyntaxKind::BrOpen, SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::Predicate => &[
                SyntaxKind::ArrowLeft,
                SyntaxKind::BrOpen,
                SyntaxKind::ClOpen,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::PredicateObjectList => &[
                SyntaxKind::Alit,
                SyntaxKind::ArrowLeft,
                SyntaxKind::BrOpen,
                SyntaxKind::ClOpen,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::IsLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::PrefixId => &[SyntaxKind::PrefixToken],
            SyntaxKind::PrefixedName => &[],
            SyntaxKind::QuickVar => &[],
            SyntaxKind::RdfLiteral => &[],
            SyntaxKind::SparqlBase => &[SyntaxKind::BaseLit],
            SyntaxKind::SparqlDirective => &[SyntaxKind::BaseLit, SyntaxKind::PrefixLit],
            SyntaxKind::SparqlPrefix => &[SyntaxKind::PrefixLit],
            SyntaxKind::Subject => &[SyntaxKind::BrOpen, SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::Triples => &[SyntaxKind::BrOpen, SyntaxKind::ClOpen, SyntaxKind::SqOpen],
            SyntaxKind::Verb => &[
                SyntaxKind::Alit,
                SyntaxKind::ArrowLeft,
                SyntaxKind::BrOpen,
                SyntaxKind::ClOpen,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::IsLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::Bang => &[SyntaxKind::Bang],
            SyntaxKind::BrOpen => &[SyntaxKind::BrOpen],
            SyntaxKind::BrClose => &[SyntaxKind::BrClose],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::ArrowLeft => &[SyntaxKind::ArrowLeft],
            SyntaxKind::ImplyLeft => &[SyntaxKind::ImplyLeft],
            SyntaxKind::Eq => &[SyntaxKind::Eq],
            SyntaxKind::ImplyRight => &[SyntaxKind::ImplyRight],
            SyntaxKind::BaseToken => &[SyntaxKind::BaseToken],
            SyntaxKind::PrefixToken => &[SyntaxKind::PrefixToken],
            SyntaxKind::BaseLit => &[SyntaxKind::BaseLit],
            SyntaxKind::PrefixLit => &[SyntaxKind::PrefixLit],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::Hat => &[SyntaxKind::Hat],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::Alit => &[SyntaxKind::Alit],
            SyntaxKind::HasLit => &[SyntaxKind::HasLit],
            SyntaxKind::IsLit => &[SyntaxKind::IsLit],
            SyntaxKind::OfLit => &[SyntaxKind::OfLit],
            SyntaxKind::ClOpen => &[SyntaxKind::ClOpen],
            SyntaxKind::ClClose => &[SyntaxKind::ClClose],
            SyntaxKind::Anon => &[SyntaxKind::Anon],
            SyntaxKind::BlankNodeLabel => &[SyntaxKind::BlankNodeLabel],
            SyntaxKind::BooleanLiteral => &[SyntaxKind::BooleanLiteral],
            SyntaxKind::Decimal => &[SyntaxKind::Decimal],
            SyntaxKind::Double => &[SyntaxKind::Double],
            SyntaxKind::Integer => &[SyntaxKind::Integer],
            SyntaxKind::Iplstart => &[SyntaxKind::Iplstart],
            SyntaxKind::Iriref => &[SyntaxKind::Iriref],
            SyntaxKind::Langtag => &[SyntaxKind::Langtag],
            SyntaxKind::PnameLn => &[SyntaxKind::PnameLn],
            SyntaxKind::PnameNs => &[SyntaxKind::PnameNs],
            SyntaxKind::QuickVarName => &[SyntaxKind::QuickVarName],
            SyntaxKind::String => &[SyntaxKind::String],
            _ => &[],
        }
    }
    #[doc = r" Returns the set of all terminals that can be consumed *anywhere*"]
    #[doc = r" in a parse of `kind` — including inside sub-rules at any depth."]
    #[doc = r#" An empty slice means "unknown / no pruning"."#]
    pub fn all_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::Base => &[SyntaxKind::BaseToken, SyntaxKind::Iriref],
            SyntaxKind::BlankNode => &[SyntaxKind::Anon, SyntaxKind::BlankNodeLabel],
            SyntaxKind::BlankNodePropertyList => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::Collection => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::Expression => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::Formula => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::FormulaContent => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::Iri => &[SyntaxKind::Iriref, SyntaxKind::PnameLn, SyntaxKind::PnameNs],
            SyntaxKind::IriPropertyList => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::Literal => &[
                SyntaxKind::BooleanLiteral,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Integer,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::String,
            ],
            SyntaxKind::N3Directive => &[
                SyntaxKind::BaseToken,
                SyntaxKind::Iriref,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixToken,
            ],
            SyntaxKind::N3Doc => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::N3Statement => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::NumericLiteral => {
                &[SyntaxKind::Decimal, SyntaxKind::Double, SyntaxKind::Integer]
            }
            SyntaxKind::Object => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::ObjectList => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::Path => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::PathItem => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::Predicate => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::PredicateObjectList => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::PrefixId => &[
                SyntaxKind::Iriref,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixToken,
            ],
            SyntaxKind::PrefixedName => &[SyntaxKind::PnameLn, SyntaxKind::PnameNs],
            SyntaxKind::QuickVar => &[SyntaxKind::QuickVarName],
            SyntaxKind::RdfLiteral => &[
                SyntaxKind::Datatype,
                SyntaxKind::Iriref,
                SyntaxKind::Langtag,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::String,
            ],
            SyntaxKind::SparqlBase => &[SyntaxKind::BaseLit, SyntaxKind::Iriref],
            SyntaxKind::SparqlDirective => &[
                SyntaxKind::BaseLit,
                SyntaxKind::Iriref,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
            ],
            SyntaxKind::SparqlPrefix => &[
                SyntaxKind::Iriref,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
            ],
            SyntaxKind::Subject => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::Triples => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::Verb => &[
                SyntaxKind::Alit,
                SyntaxKind::Anon,
                SyntaxKind::ArrowLeft,
                SyntaxKind::Bang,
                SyntaxKind::BaseLit,
                SyntaxKind::BaseToken,
                SyntaxKind::BlankNodeLabel,
                SyntaxKind::BooleanLiteral,
                SyntaxKind::BrClose,
                SyntaxKind::BrOpen,
                SyntaxKind::ClClose,
                SyntaxKind::ClOpen,
                SyntaxKind::Colon,
                SyntaxKind::Comma,
                SyntaxKind::Datatype,
                SyntaxKind::Decimal,
                SyntaxKind::Double,
                SyntaxKind::Eq,
                SyntaxKind::HasLit,
                SyntaxKind::Hat,
                SyntaxKind::ImplyLeft,
                SyntaxKind::ImplyRight,
                SyntaxKind::Integer,
                SyntaxKind::Iplstart,
                SyntaxKind::Iriref,
                SyntaxKind::IsLit,
                SyntaxKind::Langtag,
                SyntaxKind::OfLit,
                SyntaxKind::PnameLn,
                SyntaxKind::PnameNs,
                SyntaxKind::PrefixLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::QuickVarName,
                SyntaxKind::SqClose,
                SyntaxKind::SqOpen,
                SyntaxKind::Stop,
                SyntaxKind::String,
            ],
            SyntaxKind::Bang => &[SyntaxKind::Bang],
            SyntaxKind::BrOpen => &[SyntaxKind::BrOpen],
            SyntaxKind::BrClose => &[SyntaxKind::BrClose],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::ArrowLeft => &[SyntaxKind::ArrowLeft],
            SyntaxKind::ImplyLeft => &[SyntaxKind::ImplyLeft],
            SyntaxKind::Eq => &[SyntaxKind::Eq],
            SyntaxKind::ImplyRight => &[SyntaxKind::ImplyRight],
            SyntaxKind::BaseToken => &[SyntaxKind::BaseToken],
            SyntaxKind::PrefixToken => &[SyntaxKind::PrefixToken],
            SyntaxKind::BaseLit => &[SyntaxKind::BaseLit],
            SyntaxKind::PrefixLit => &[SyntaxKind::PrefixLit],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::Hat => &[SyntaxKind::Hat],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::Alit => &[SyntaxKind::Alit],
            SyntaxKind::HasLit => &[SyntaxKind::HasLit],
            SyntaxKind::IsLit => &[SyntaxKind::IsLit],
            SyntaxKind::OfLit => &[SyntaxKind::OfLit],
            SyntaxKind::ClOpen => &[SyntaxKind::ClOpen],
            SyntaxKind::ClClose => &[SyntaxKind::ClClose],
            SyntaxKind::Anon => &[SyntaxKind::Anon],
            SyntaxKind::BlankNodeLabel => &[SyntaxKind::BlankNodeLabel],
            SyntaxKind::BooleanLiteral => &[SyntaxKind::BooleanLiteral],
            SyntaxKind::Decimal => &[SyntaxKind::Decimal],
            SyntaxKind::Double => &[SyntaxKind::Double],
            SyntaxKind::Integer => &[SyntaxKind::Integer],
            SyntaxKind::Iplstart => &[SyntaxKind::Iplstart],
            SyntaxKind::Iriref => &[SyntaxKind::Iriref],
            SyntaxKind::Langtag => &[SyntaxKind::Langtag],
            SyntaxKind::PnameLn => &[SyntaxKind::PnameLn],
            SyntaxKind::PnameNs => &[SyntaxKind::PnameNs],
            SyntaxKind::QuickVarName => &[SyntaxKind::QuickVarName],
            SyntaxKind::String => &[SyntaxKind::String],
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
                (SyntaxKind::N3Doc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::N3Doc, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                }
                (SyntaxKind::N3Doc, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                }
                (SyntaxKind::N3Doc, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop);
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
                (SyntaxKind::N3Doc, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::N3Statement,
                                state: 1usize,
                            }),
                        SyntaxKind::N3Statement,
                    );
                }
                (SyntaxKind::N3Doc, 5usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SparqlDirective,
                                state: 1usize,
                            }),
                        SyntaxKind::SparqlDirective,
                    );
                }
                (SyntaxKind::N3Statement, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::N3Statement, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::N3Statement, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::N3Directive,
                                state: 1usize,
                            }),
                        SyntaxKind::N3Directive,
                    );
                }
                (SyntaxKind::N3Statement, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Triples,
                                state: 3usize,
                            }),
                        SyntaxKind::Triples,
                    );
                }
                (SyntaxKind::N3Directive, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::N3Directive, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::N3Directive, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PrefixId,
                                state: 3usize,
                            }),
                        SyntaxKind::PrefixId,
                    );
                }
                (SyntaxKind::N3Directive, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Base,
                                state: 2usize,
                            }),
                        SyntaxKind::Base,
                    );
                }
                (SyntaxKind::SparqlDirective, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SparqlDirective, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::SparqlDirective, 2usize) => {
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
                (SyntaxKind::SparqlDirective, 3usize) => {
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BaseLit);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::PrefixLit);
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
                (SyntaxKind::PrefixId, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixId, 1usize) => {
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
                (SyntaxKind::PrefixId, 2usize) => {
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
                (SyntaxKind::PrefixId, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::PrefixToken);
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
                (SyntaxKind::Base, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Base, 1usize) => {
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
                (SyntaxKind::Base, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BaseToken);
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
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
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
                                state: 1usize,
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
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 9usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 10usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 11usize,
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
                (SyntaxKind::Verb, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                        SyntaxKind::Expression,
                    );
                }
                (SyntaxKind::Verb, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::HasLit);
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
                (SyntaxKind::Verb, 6usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::OfLit);
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
                (SyntaxKind::Verb, 7usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                        SyntaxKind::Expression,
                    );
                }
                (SyntaxKind::Verb, 8usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::IsLit);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 7usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 7usize,
                        }));
                    }
                }
                (SyntaxKind::Verb, 9usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Eq);
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
                (SyntaxKind::Verb, 10usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ImplyLeft);
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
                (SyntaxKind::Verb, 11usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ImplyRight);
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
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                        SyntaxKind::Expression,
                    );
                }
                (SyntaxKind::Predicate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Predicate, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::Predicate, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                        SyntaxKind::Expression,
                    );
                }
                (SyntaxKind::Predicate, 3usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                }
                (SyntaxKind::Predicate, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ArrowLeft);
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
                (SyntaxKind::Object, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Object, 1usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                        SyntaxKind::Expression,
                    );
                }
                (SyntaxKind::Expression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Expression, 1usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Path,
                                state: 6usize,
                            }),
                        SyntaxKind::Path,
                    );
                }
                (SyntaxKind::Path, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Path, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::Path, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Path,
                                state: 6usize,
                            }),
                        SyntaxKind::Path,
                    );
                }
                (SyntaxKind::Path, 3usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                }
                (SyntaxKind::Path, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Bang);
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
                (SyntaxKind::Path, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Hat);
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
                (SyntaxKind::Path, 6usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathItem,
                                state: 1usize,
                            }),
                        SyntaxKind::PathItem,
                    );
                }
                (SyntaxKind::PathItem, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PathItem, 1usize) => {
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
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 7usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 9usize,
                    }));
                }
                (SyntaxKind::PathItem, 2usize) => {
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
                (SyntaxKind::PathItem, 3usize) => {
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
                (SyntaxKind::PathItem, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::QuickVar,
                                state: 1usize,
                            }),
                        SyntaxKind::QuickVar,
                    );
                }
                (SyntaxKind::PathItem, 5usize) => {
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
                (SyntaxKind::PathItem, 6usize) => {
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
                (SyntaxKind::PathItem, 7usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::IriPropertyList,
                                state: 4usize,
                            }),
                        SyntaxKind::IriPropertyList,
                    );
                }
                (SyntaxKind::PathItem, 8usize) => {
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
                (SyntaxKind::PathItem, 9usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Formula,
                                state: 4usize,
                            }),
                        SyntaxKind::Formula,
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
                                kind: SyntaxKind::RdfLiteral,
                                state: 6usize,
                            }),
                        SyntaxKind::RdfLiteral,
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BooleanLiteral);
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
                (SyntaxKind::IriPropertyList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IriPropertyList, 1usize) => {
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
                (SyntaxKind::IriPropertyList, 2usize) => {
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
                (SyntaxKind::IriPropertyList, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                        SyntaxKind::Iri,
                    );
                }
                (SyntaxKind::IriPropertyList, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iplstart);
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
                (SyntaxKind::Collection, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Collection, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrClose);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen);
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
                (SyntaxKind::Formula, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Formula, 1usize) => {
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
                (SyntaxKind::Formula, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::Formula, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::FormulaContent,
                                state: 1usize,
                            }),
                        SyntaxKind::FormulaContent,
                    );
                }
                (SyntaxKind::Formula, 4usize) => {
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
                (SyntaxKind::FormulaContent, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::FormulaContent, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 6usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 9usize,
                    }));
                }
                (SyntaxKind::FormulaContent, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                }
                (SyntaxKind::FormulaContent, 3usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::FormulaContent, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::FormulaContent,
                                state: 1usize,
                            }),
                        SyntaxKind::FormulaContent,
                    );
                }
                (SyntaxKind::FormulaContent, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop);
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
                (SyntaxKind::FormulaContent, 6usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::N3Statement,
                                state: 1usize,
                            }),
                        SyntaxKind::N3Statement,
                    );
                }
                (SyntaxKind::FormulaContent, 7usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                }
                (SyntaxKind::FormulaContent, 8usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::FormulaContent, 9usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SparqlDirective,
                                state: 1usize,
                            }),
                        SyntaxKind::SparqlDirective,
                    );
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
                (SyntaxKind::RdfLiteral, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::RdfLiteral, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                }
                (SyntaxKind::RdfLiteral, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 5usize,
                    }));
                }
                (SyntaxKind::RdfLiteral, 3usize) => {
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
                (SyntaxKind::RdfLiteral, 4usize) => {
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
                (SyntaxKind::RdfLiteral, 5usize) => {
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
                (SyntaxKind::RdfLiteral, 6usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::String);
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
                        state: 3usize,
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
                (SyntaxKind::QuickVar, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::QuickVar, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::QuickVarName);
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
                (SyntaxKind::Bang, _) => {
                    let added = state.expect_as(element, SyntaxKind::Bang);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BrOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::BrOpen);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BrClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::BrClose);
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
                (SyntaxKind::ArrowLeft, _) => {
                    let added = state.expect_as(element, SyntaxKind::ArrowLeft);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ImplyLeft, _) => {
                    let added = state.expect_as(element, SyntaxKind::ImplyLeft);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Eq, _) => {
                    let added = state.expect_as(element, SyntaxKind::Eq);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ImplyRight, _) => {
                    let added = state.expect_as(element, SyntaxKind::ImplyRight);
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
                (SyntaxKind::BaseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BaseLit);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::PrefixLit);
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
                (SyntaxKind::Hat, _) => {
                    let added = state.expect_as(element, SyntaxKind::Hat);
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
                (SyntaxKind::HasLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::HasLit);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsLit);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::OfLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::OfLit);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::BooleanLiteral, _) => {
                    let added = state.expect_as(element, SyntaxKind::BooleanLiteral);
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
                (SyntaxKind::Iplstart, _) => {
                    let added = state.expect_as(element, SyntaxKind::Iplstart);
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
                (SyntaxKind::QuickVarName, _) => {
                    let added = state.expect_as(element, SyntaxKind::QuickVarName);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::String, _) => {
                    let added = state.expect_as(element, SyntaxKind::String);
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
            SyntaxKind::Comma => 3isize,
            SyntaxKind::Stop => 5isize,
            SyntaxKind::Colon => 4isize,
            SyntaxKind::BaseToken => 100isize,
            SyntaxKind::PrefixToken => 100isize,
            _ => 1isize,
        }
    }
    fn bracket_delta(&self) -> i8 {
        match self {
            SyntaxKind::SqOpen => 1,
            SyntaxKind::BrOpen => 1,
            SyntaxKind::ClOpen => 1,
            SyntaxKind::SqClose => -1,
            SyntaxKind::BrClose => -1,
            SyntaxKind::ClClose => -1,
            _ => 0,
        }
    }
}
