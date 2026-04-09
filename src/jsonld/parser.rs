use crate::TokenTrait;
pub type SyntaxNode = rowan::SyntaxNode<Lang>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, logos :: Logos)]
# [logos (subpattern HEX = r#"([0-9]|[A-F]|[a-f])"#)]
# [logos (subpattern JSON_STRING = r#""(([^\x{22}\x{5C}]|\\\u0022|\\\u005C|\\\u002F|\\b|\\f|\\n|\\r|\\t|\\u(?&HEX)(?&HEX)(?&HEX)(?&HEX)))*""#)]
# [logos (subpattern JSON_NUMBER = r#"(-)?(0|[1-9]([0-9])*)(\.([0-9])+)?([eE]([+-])?([0-9])+)?"#)]
#[repr(u16)]
pub enum SyntaxKind {
    Eof = 0,
    #[regex(r"[ \t\n]+")]
    WhiteSpace,
    #[regex(r"#[^\n]+", allow_greedy = true)]
    Comment,
    #[doc = r" producings"]
    JsonArray,
    JsonObject,
    JsonString,
    JsonValue,
    JsonldDoc,
    Member,
    MemberList,
    ValueList,
    #[doc = r" terminals"]
    #[token("\"@base\"")]
    AtBase,
    #[token("\"@container\"")]
    AtContainer,
    #[token("\"@context\"")]
    AtContext,
    #[token("\"@direction\"")]
    AtDirection,
    #[token("\"@graph\"")]
    AtGraph,
    #[token("\"@id\"")]
    AtId,
    #[token("\"@import\"")]
    AtImport,
    #[token("\"@included\"")]
    AtIncluded,
    #[token("\"@index\"")]
    AtIndex,
    #[token("\"@json\"")]
    AtJson,
    #[token("\"@language\"")]
    AtLanguage,
    #[token("\"@list\"")]
    AtList,
    #[token("\"@nest\"")]
    AtNest,
    #[token("\"@none\"")]
    AtNone,
    #[token("\"@prefix\"")]
    AtPrefix,
    #[token("\"@propagate\"")]
    AtPropagate,
    #[token("\"@protected\"")]
    AtProtected,
    #[token("\"@reverse\"")]
    AtReverse,
    #[token("\"@set\"")]
    AtSet,
    #[token("\"@type\"")]
    AtType,
    #[token("\"@value\"")]
    AtValue,
    #[token("\"@version\"")]
    AtVersion,
    #[token("\"@vocab\"")]
    AtVocab,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("[")]
    SqOpen,
    #[token("]")]
    SqClose,
    #[token("false")]
    FalseLit,
    #[token("null")]
    NullLit,
    #[token("true")]
    TrueLit,
    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,
    #[regex("(?&JSON_NUMBER)")]
    JsonNumber,
    #[regex("(?&JSON_STRING)")]
    StringToken,
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
                SyntaxKind::JsonldDoc => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::JsonObject => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::JsonArray => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::MemberList => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::Member => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::JsonValue => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::ValueList => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::JsonString => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::AtBase => Rule { kind, state: 0 },
                SyntaxKind::AtContainer => Rule { kind, state: 0 },
                SyntaxKind::AtContext => Rule { kind, state: 0 },
                SyntaxKind::AtDirection => Rule { kind, state: 0 },
                SyntaxKind::AtGraph => Rule { kind, state: 0 },
                SyntaxKind::AtId => Rule { kind, state: 0 },
                SyntaxKind::AtImport => Rule { kind, state: 0 },
                SyntaxKind::AtIncluded => Rule { kind, state: 0 },
                SyntaxKind::AtIndex => Rule { kind, state: 0 },
                SyntaxKind::AtJson => Rule { kind, state: 0 },
                SyntaxKind::AtLanguage => Rule { kind, state: 0 },
                SyntaxKind::AtList => Rule { kind, state: 0 },
                SyntaxKind::AtNest => Rule { kind, state: 0 },
                SyntaxKind::AtNone => Rule { kind, state: 0 },
                SyntaxKind::AtPrefix => Rule { kind, state: 0 },
                SyntaxKind::AtPropagate => Rule { kind, state: 0 },
                SyntaxKind::AtProtected => Rule { kind, state: 0 },
                SyntaxKind::AtReverse => Rule { kind, state: 0 },
                SyntaxKind::AtSet => Rule { kind, state: 0 },
                SyntaxKind::AtType => Rule { kind, state: 0 },
                SyntaxKind::AtValue => Rule { kind, state: 0 },
                SyntaxKind::AtVersion => Rule { kind, state: 0 },
                SyntaxKind::AtVocab => Rule { kind, state: 0 },
                SyntaxKind::Comma => Rule { kind, state: 0 },
                SyntaxKind::Colon => Rule { kind, state: 0 },
                SyntaxKind::SqOpen => Rule { kind, state: 0 },
                SyntaxKind::SqClose => Rule { kind, state: 0 },
                SyntaxKind::FalseLit => Rule { kind, state: 0 },
                SyntaxKind::NullLit => Rule { kind, state: 0 },
                SyntaxKind::TrueLit => Rule { kind, state: 0 },
                SyntaxKind::CurlyOpen => Rule { kind, state: 0 },
                SyntaxKind::CurlyClose => Rule { kind, state: 0 },
                SyntaxKind::JsonNumber => Rule { kind, state: 0 },
                SyntaxKind::StringToken => Rule { kind, state: 0 },
                _ => panic!("Unknown rule kind {:?}", kind),
            }
        }
    }
    pub fn first_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::JsonArray => &[SyntaxKind::SqOpen],
            SyntaxKind::JsonObject => &[SyntaxKind::CurlyOpen],
            SyntaxKind::JsonString => &[
                SyntaxKind::AtBase,
                SyntaxKind::AtContainer,
                SyntaxKind::AtContext,
                SyntaxKind::AtDirection,
                SyntaxKind::AtGraph,
                SyntaxKind::AtId,
                SyntaxKind::AtImport,
                SyntaxKind::AtIncluded,
                SyntaxKind::AtIndex,
                SyntaxKind::AtJson,
                SyntaxKind::AtLanguage,
                SyntaxKind::AtList,
                SyntaxKind::AtNest,
                SyntaxKind::AtNone,
                SyntaxKind::AtPrefix,
                SyntaxKind::AtPropagate,
                SyntaxKind::AtProtected,
                SyntaxKind::AtReverse,
                SyntaxKind::AtSet,
                SyntaxKind::AtType,
                SyntaxKind::AtValue,
                SyntaxKind::AtVersion,
                SyntaxKind::AtVocab,
            ],
            SyntaxKind::JsonValue => &[
                SyntaxKind::AtBase,
                SyntaxKind::AtContainer,
                SyntaxKind::AtContext,
                SyntaxKind::AtDirection,
                SyntaxKind::AtGraph,
                SyntaxKind::AtId,
                SyntaxKind::AtImport,
                SyntaxKind::AtIncluded,
                SyntaxKind::AtIndex,
                SyntaxKind::AtJson,
                SyntaxKind::AtLanguage,
                SyntaxKind::AtList,
                SyntaxKind::AtNest,
                SyntaxKind::AtNone,
                SyntaxKind::AtPrefix,
                SyntaxKind::AtPropagate,
                SyntaxKind::AtProtected,
                SyntaxKind::AtReverse,
                SyntaxKind::AtSet,
                SyntaxKind::AtType,
                SyntaxKind::AtValue,
                SyntaxKind::AtVersion,
                SyntaxKind::AtVocab,
                SyntaxKind::CurlyOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::NullLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::JsonldDoc => &[
                SyntaxKind::AtBase,
                SyntaxKind::AtContainer,
                SyntaxKind::AtContext,
                SyntaxKind::AtDirection,
                SyntaxKind::AtGraph,
                SyntaxKind::AtId,
                SyntaxKind::AtImport,
                SyntaxKind::AtIncluded,
                SyntaxKind::AtIndex,
                SyntaxKind::AtJson,
                SyntaxKind::AtLanguage,
                SyntaxKind::AtList,
                SyntaxKind::AtNest,
                SyntaxKind::AtNone,
                SyntaxKind::AtPrefix,
                SyntaxKind::AtPropagate,
                SyntaxKind::AtProtected,
                SyntaxKind::AtReverse,
                SyntaxKind::AtSet,
                SyntaxKind::AtType,
                SyntaxKind::AtValue,
                SyntaxKind::AtVersion,
                SyntaxKind::AtVocab,
                SyntaxKind::CurlyOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::NullLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Member => &[
                SyntaxKind::AtBase,
                SyntaxKind::AtContainer,
                SyntaxKind::AtContext,
                SyntaxKind::AtDirection,
                SyntaxKind::AtGraph,
                SyntaxKind::AtId,
                SyntaxKind::AtImport,
                SyntaxKind::AtIncluded,
                SyntaxKind::AtIndex,
                SyntaxKind::AtJson,
                SyntaxKind::AtLanguage,
                SyntaxKind::AtList,
                SyntaxKind::AtNest,
                SyntaxKind::AtNone,
                SyntaxKind::AtPrefix,
                SyntaxKind::AtPropagate,
                SyntaxKind::AtProtected,
                SyntaxKind::AtReverse,
                SyntaxKind::AtSet,
                SyntaxKind::AtType,
                SyntaxKind::AtValue,
                SyntaxKind::AtVersion,
                SyntaxKind::AtVocab,
            ],
            SyntaxKind::MemberList => &[
                SyntaxKind::AtBase,
                SyntaxKind::AtContainer,
                SyntaxKind::AtContext,
                SyntaxKind::AtDirection,
                SyntaxKind::AtGraph,
                SyntaxKind::AtId,
                SyntaxKind::AtImport,
                SyntaxKind::AtIncluded,
                SyntaxKind::AtIndex,
                SyntaxKind::AtJson,
                SyntaxKind::AtLanguage,
                SyntaxKind::AtList,
                SyntaxKind::AtNest,
                SyntaxKind::AtNone,
                SyntaxKind::AtPrefix,
                SyntaxKind::AtPropagate,
                SyntaxKind::AtProtected,
                SyntaxKind::AtReverse,
                SyntaxKind::AtSet,
                SyntaxKind::AtType,
                SyntaxKind::AtValue,
                SyntaxKind::AtVersion,
                SyntaxKind::AtVocab,
            ],
            SyntaxKind::ValueList => &[
                SyntaxKind::AtBase,
                SyntaxKind::AtContainer,
                SyntaxKind::AtContext,
                SyntaxKind::AtDirection,
                SyntaxKind::AtGraph,
                SyntaxKind::AtId,
                SyntaxKind::AtImport,
                SyntaxKind::AtIncluded,
                SyntaxKind::AtIndex,
                SyntaxKind::AtJson,
                SyntaxKind::AtLanguage,
                SyntaxKind::AtList,
                SyntaxKind::AtNest,
                SyntaxKind::AtNone,
                SyntaxKind::AtPrefix,
                SyntaxKind::AtPropagate,
                SyntaxKind::AtProtected,
                SyntaxKind::AtReverse,
                SyntaxKind::AtSet,
                SyntaxKind::AtType,
                SyntaxKind::AtValue,
                SyntaxKind::AtVersion,
                SyntaxKind::AtVocab,
                SyntaxKind::CurlyOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::NullLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::AtBase => &[SyntaxKind::AtBase],
            SyntaxKind::AtContainer => &[SyntaxKind::AtContainer],
            SyntaxKind::AtContext => &[SyntaxKind::AtContext],
            SyntaxKind::AtDirection => &[SyntaxKind::AtDirection],
            SyntaxKind::AtGraph => &[SyntaxKind::AtGraph],
            SyntaxKind::AtId => &[SyntaxKind::AtId],
            SyntaxKind::AtImport => &[SyntaxKind::AtImport],
            SyntaxKind::AtIncluded => &[SyntaxKind::AtIncluded],
            SyntaxKind::AtIndex => &[SyntaxKind::AtIndex],
            SyntaxKind::AtJson => &[SyntaxKind::AtJson],
            SyntaxKind::AtLanguage => &[SyntaxKind::AtLanguage],
            SyntaxKind::AtList => &[SyntaxKind::AtList],
            SyntaxKind::AtNest => &[SyntaxKind::AtNest],
            SyntaxKind::AtNone => &[SyntaxKind::AtNone],
            SyntaxKind::AtPrefix => &[SyntaxKind::AtPrefix],
            SyntaxKind::AtPropagate => &[SyntaxKind::AtPropagate],
            SyntaxKind::AtProtected => &[SyntaxKind::AtProtected],
            SyntaxKind::AtReverse => &[SyntaxKind::AtReverse],
            SyntaxKind::AtSet => &[SyntaxKind::AtSet],
            SyntaxKind::AtType => &[SyntaxKind::AtType],
            SyntaxKind::AtValue => &[SyntaxKind::AtValue],
            SyntaxKind::AtVersion => &[SyntaxKind::AtVersion],
            SyntaxKind::AtVocab => &[SyntaxKind::AtVocab],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::FalseLit => &[SyntaxKind::FalseLit],
            SyntaxKind::NullLit => &[SyntaxKind::NullLit],
            SyntaxKind::TrueLit => &[SyntaxKind::TrueLit],
            SyntaxKind::CurlyOpen => &[SyntaxKind::CurlyOpen],
            SyntaxKind::CurlyClose => &[SyntaxKind::CurlyClose],
            SyntaxKind::JsonNumber => &[SyntaxKind::JsonNumber],
            SyntaxKind::StringToken => &[SyntaxKind::StringToken],
            _ => &[],
        }
    }
    #[doc = r" Returns the minimum error cost that `kind` must incur when `tok`"]
    #[doc = r" is the current token.  0 means the token is reachable (or the rule"]
    #[doc = r" is nullable); positive means the rule cannot make progress without"]
    #[doc = r" at least that much error cost."]
    pub fn min_error_for_token(kind: SyntaxKind, tok: SyntaxKind) -> isize {
        match kind {
            SyntaxKind::JsonArray => match tok {
                SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::CurlyClose
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::JsonNumber
                | SyntaxKind::NullLit
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringToken
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::JsonObject => match tok {
                SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::CurlyClose
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::JsonNumber
                | SyntaxKind::NullLit
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringToken
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::JsonString => match tok {
                SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::StringToken => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::JsonValue => match tok {
                SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::CurlyClose
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::JsonNumber
                | SyntaxKind::NullLit
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringToken
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::JsonldDoc => match tok {
                SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::CurlyClose
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::JsonNumber
                | SyntaxKind::NullLit
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringToken
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Member => match tok {
                SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::CurlyClose
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::JsonNumber
                | SyntaxKind::NullLit
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringToken
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::MemberList => match tok {
                SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::CurlyClose
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::JsonNumber
                | SyntaxKind::NullLit
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringToken
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::ValueList => match tok {
                SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::CurlyClose
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::JsonNumber
                | SyntaxKind::NullLit
                | SyntaxKind::SqClose
                | SyntaxKind::SqOpen
                | SyntaxKind::StringToken
                | SyntaxKind::TrueLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtBase => match tok {
                SyntaxKind::AtBase => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtContainer => match tok {
                SyntaxKind::AtContainer => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtContext => match tok {
                SyntaxKind::AtContext => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtDirection => match tok {
                SyntaxKind::AtDirection => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtGraph => match tok {
                SyntaxKind::AtGraph => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtId => match tok {
                SyntaxKind::AtId => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtImport => match tok {
                SyntaxKind::AtImport => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtIncluded => match tok {
                SyntaxKind::AtIncluded => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtIndex => match tok {
                SyntaxKind::AtIndex => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtJson => match tok {
                SyntaxKind::AtJson => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtLanguage => match tok {
                SyntaxKind::AtLanguage => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtList => match tok {
                SyntaxKind::AtList => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtNest => match tok {
                SyntaxKind::AtNest => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtNone => match tok {
                SyntaxKind::AtNone => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtPrefix => match tok {
                SyntaxKind::AtPrefix => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtPropagate => match tok {
                SyntaxKind::AtPropagate => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtProtected => match tok {
                SyntaxKind::AtProtected => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtReverse => match tok {
                SyntaxKind::AtReverse => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtSet => match tok {
                SyntaxKind::AtSet => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtType => match tok {
                SyntaxKind::AtType => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtValue => match tok {
                SyntaxKind::AtValue => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtVersion => match tok {
                SyntaxKind::AtVersion => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::AtVocab => match tok {
                SyntaxKind::AtVocab => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Comma => match tok {
                SyntaxKind::Comma => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Colon => match tok {
                SyntaxKind::Colon => 0,
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
            SyntaxKind::FalseLit => match tok {
                SyntaxKind::FalseLit => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::NullLit => match tok {
                SyntaxKind::NullLit => 0,
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
            SyntaxKind::JsonNumber => match tok {
                SyntaxKind::JsonNumber => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::StringToken => match tok {
                SyntaxKind::StringToken => 0,
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
            (SyntaxKind::JsonArray, 1usize, _) => match terminal {
                SyntaxKind::SqClose => 0,
                _ => 2isize,
            },
            (SyntaxKind::JsonArray, 3usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::NullLit
                | SyntaxKind::SqOpen
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonArray, 4usize, _) => match terminal {
                SyntaxKind::SqOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::JsonArray, 2usize, _) => match terminal {
                SyntaxKind::Colon => 1isize,
                SyntaxKind::Comma => 1isize,
                SyntaxKind::CurlyClose => 1isize,
                _ => 0,
            },
            (SyntaxKind::JsonObject, 2usize, _) => match terminal {
                SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::CurlyClose => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonObject, 3usize, _) => match terminal {
                SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonObject, 1usize, _) => match terminal {
                SyntaxKind::CurlyClose => 0,
                _ => 2isize,
            },
            (SyntaxKind::JsonObject, 4usize, _) => match terminal {
                SyntaxKind::CurlyOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::JsonString, 2usize, _) => match terminal {
                SyntaxKind::StringToken => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 6usize, _) => match terminal {
                SyntaxKind::AtDirection => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 16usize, _) => match terminal {
                SyntaxKind::AtNone => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 22usize, _) => match terminal {
                SyntaxKind::AtType => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 15usize, _) => match terminal {
                SyntaxKind::AtNest => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 24usize, _) => match terminal {
                SyntaxKind::AtVersion => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 17usize, _) => match terminal {
                SyntaxKind::AtPrefix => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 8usize, _) => match terminal {
                SyntaxKind::AtId => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 3usize, _) => match terminal {
                SyntaxKind::AtBase => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 11usize, _) => match terminal {
                SyntaxKind::AtIndex => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 1usize, _) => match terminal {
                SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 21usize, _) => match terminal {
                SyntaxKind::AtSet => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 19usize, _) => match terminal {
                SyntaxKind::AtProtected => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 23usize, _) => match terminal {
                SyntaxKind::AtValue => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 9usize, _) => match terminal {
                SyntaxKind::AtImport => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 12usize, _) => match terminal {
                SyntaxKind::AtJson => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 4usize, _) => match terminal {
                SyntaxKind::AtContainer => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 14usize, _) => match terminal {
                SyntaxKind::AtList => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 20usize, _) => match terminal {
                SyntaxKind::AtReverse => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 7usize, _) => match terminal {
                SyntaxKind::AtGraph => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 13usize, _) => match terminal {
                SyntaxKind::AtLanguage => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 10usize, _) => match terminal {
                SyntaxKind::AtIncluded => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 18usize, _) => match terminal {
                SyntaxKind::AtPropagate => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonString, 5usize, _) => match terminal {
                SyntaxKind::AtContext => 0,
                _ => 100isize,
            },
            (SyntaxKind::JsonString, 25usize, _) => match terminal {
                SyntaxKind::AtVocab => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 6usize, _) => match terminal {
                SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 8usize, _) => match terminal {
                SyntaxKind::NullLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 2usize, _) => match terminal {
                SyntaxKind::CurlyOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::JsonValue, 4usize, _) => match terminal {
                SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 1usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::NullLit
                | SyntaxKind::SqOpen
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 5usize, _) => match terminal {
                SyntaxKind::JsonNumber => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 7usize, _) => match terminal {
                SyntaxKind::FalseLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 3usize, _) => match terminal {
                SyntaxKind::SqOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::JsonldDoc, 1usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::NullLit
                | SyntaxKind::SqOpen
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::Member, 2usize, _) => match terminal {
                SyntaxKind::JsonNumber => 3isize,
                SyntaxKind::StringToken => 3isize,
                SyntaxKind::AtBase => 3isize,
                SyntaxKind::AtContainer => 3isize,
                SyntaxKind::AtContext => 3isize,
                SyntaxKind::AtDirection => 3isize,
                SyntaxKind::AtGraph => 3isize,
                SyntaxKind::AtId => 3isize,
                SyntaxKind::AtImport => 3isize,
                SyntaxKind::AtIncluded => 3isize,
                SyntaxKind::AtIndex => 3isize,
                SyntaxKind::AtJson => 3isize,
                SyntaxKind::AtLanguage => 3isize,
                SyntaxKind::AtList => 3isize,
                SyntaxKind::AtNest => 3isize,
                SyntaxKind::AtNone => 3isize,
                SyntaxKind::AtPrefix => 3isize,
                SyntaxKind::AtPropagate => 3isize,
                SyntaxKind::AtProtected => 3isize,
                SyntaxKind::AtReverse => 3isize,
                SyntaxKind::AtSet => 3isize,
                SyntaxKind::AtType => 3isize,
                SyntaxKind::AtValue => 3isize,
                SyntaxKind::AtVersion => 3isize,
                SyntaxKind::AtVocab => 3isize,
                SyntaxKind::Comma => 4isize,
                SyntaxKind::CurlyClose => 4isize,
                SyntaxKind::CurlyOpen => 3isize,
                SyntaxKind::FalseLit => 3isize,
                SyntaxKind::NullLit => 3isize,
                SyntaxKind::SqClose => 4isize,
                SyntaxKind::SqOpen => 3isize,
                SyntaxKind::TrueLit => 3isize,
                _ => 0,
            },
            (SyntaxKind::Member, 3usize, _) => match terminal {
                SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab => 0,
                _ => 1isize,
            },
            (SyntaxKind::Member, 1usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::NullLit
                | SyntaxKind::SqOpen
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::MemberList, 3usize, _) => match terminal {
                SyntaxKind::JsonNumber => 3isize,
                SyntaxKind::StringToken => 2isize,
                SyntaxKind::AtBase => 2isize,
                SyntaxKind::AtContainer => 2isize,
                SyntaxKind::AtContext => 2isize,
                SyntaxKind::AtDirection => 2isize,
                SyntaxKind::AtGraph => 2isize,
                SyntaxKind::AtId => 2isize,
                SyntaxKind::AtImport => 2isize,
                SyntaxKind::AtIncluded => 2isize,
                SyntaxKind::AtIndex => 2isize,
                SyntaxKind::AtJson => 2isize,
                SyntaxKind::AtLanguage => 2isize,
                SyntaxKind::AtList => 2isize,
                SyntaxKind::AtNest => 2isize,
                SyntaxKind::AtNone => 2isize,
                SyntaxKind::AtPrefix => 2isize,
                SyntaxKind::AtPropagate => 2isize,
                SyntaxKind::AtProtected => 2isize,
                SyntaxKind::AtReverse => 2isize,
                SyntaxKind::AtSet => 2isize,
                SyntaxKind::AtType => 2isize,
                SyntaxKind::AtValue => 2isize,
                SyntaxKind::AtVersion => 2isize,
                SyntaxKind::AtVocab => 2isize,
                SyntaxKind::Colon => 3isize,
                SyntaxKind::CurlyClose => 3isize,
                SyntaxKind::CurlyOpen => 3isize,
                SyntaxKind::FalseLit => 3isize,
                SyntaxKind::NullLit => 3isize,
                SyntaxKind::SqClose => 3isize,
                SyntaxKind::SqOpen => 3isize,
                SyntaxKind::TrueLit => 3isize,
                _ => 0,
            },
            (SyntaxKind::MemberList, 2usize, _) => match terminal {
                SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab => 0,
                _ => 1isize,
            },
            (SyntaxKind::MemberList, 4usize, _) => match terminal {
                SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab => 0,
                _ => 1isize,
            },
            (SyntaxKind::ValueList, 4usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::NullLit
                | SyntaxKind::SqOpen
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::ValueList, 2usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
                | SyntaxKind::AtBase
                | SyntaxKind::AtContainer
                | SyntaxKind::AtContext
                | SyntaxKind::AtDirection
                | SyntaxKind::AtGraph
                | SyntaxKind::AtId
                | SyntaxKind::AtImport
                | SyntaxKind::AtIncluded
                | SyntaxKind::AtIndex
                | SyntaxKind::AtJson
                | SyntaxKind::AtLanguage
                | SyntaxKind::AtList
                | SyntaxKind::AtNest
                | SyntaxKind::AtNone
                | SyntaxKind::AtPrefix
                | SyntaxKind::AtPropagate
                | SyntaxKind::AtProtected
                | SyntaxKind::AtReverse
                | SyntaxKind::AtSet
                | SyntaxKind::AtType
                | SyntaxKind::AtValue
                | SyntaxKind::AtVersion
                | SyntaxKind::AtVocab
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::NullLit
                | SyntaxKind::SqOpen
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::ValueList, 3usize, _) => match terminal {
                SyntaxKind::JsonNumber => 2isize,
                SyntaxKind::StringToken => 2isize,
                SyntaxKind::AtBase => 2isize,
                SyntaxKind::AtContainer => 2isize,
                SyntaxKind::AtContext => 2isize,
                SyntaxKind::AtDirection => 2isize,
                SyntaxKind::AtGraph => 2isize,
                SyntaxKind::AtId => 2isize,
                SyntaxKind::AtImport => 2isize,
                SyntaxKind::AtIncluded => 2isize,
                SyntaxKind::AtIndex => 2isize,
                SyntaxKind::AtJson => 2isize,
                SyntaxKind::AtLanguage => 2isize,
                SyntaxKind::AtList => 2isize,
                SyntaxKind::AtNest => 2isize,
                SyntaxKind::AtNone => 2isize,
                SyntaxKind::AtPrefix => 2isize,
                SyntaxKind::AtPropagate => 2isize,
                SyntaxKind::AtProtected => 2isize,
                SyntaxKind::AtReverse => 2isize,
                SyntaxKind::AtSet => 2isize,
                SyntaxKind::AtType => 2isize,
                SyntaxKind::AtValue => 2isize,
                SyntaxKind::AtVersion => 2isize,
                SyntaxKind::AtVocab => 2isize,
                SyntaxKind::Colon => 3isize,
                SyntaxKind::CurlyClose => 3isize,
                SyntaxKind::CurlyOpen => 2isize,
                SyntaxKind::FalseLit => 2isize,
                SyntaxKind::NullLit => 2isize,
                SyntaxKind::SqClose => 3isize,
                SyntaxKind::SqOpen => 2isize,
                SyntaxKind::TrueLit => 2isize,
                _ => 0,
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
                (SyntaxKind::JsonldDoc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::JsonldDoc, 1usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::JsonValue,
                                state: 1usize,
                            }),
                        SyntaxKind::JsonValue,
                    );
                }
                (SyntaxKind::JsonObject, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::JsonObject, 1usize) => {
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
                (SyntaxKind::JsonObject, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::JsonObject, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MemberList,
                                state: 4usize,
                            }),
                        SyntaxKind::MemberList,
                    );
                }
                (SyntaxKind::JsonObject, 4usize) => {
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
                (SyntaxKind::JsonArray, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::JsonArray, 1usize) => {
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
                (SyntaxKind::JsonArray, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                }
                (SyntaxKind::JsonArray, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ValueList,
                                state: 4usize,
                            }),
                        SyntaxKind::ValueList,
                    );
                }
                (SyntaxKind::JsonArray, 4usize) => {
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
                (SyntaxKind::MemberList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::MemberList, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                }
                (SyntaxKind::MemberList, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Member,
                                state: 3usize,
                            }),
                        SyntaxKind::Member,
                    );
                }
                (SyntaxKind::MemberList, 3usize) => {
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
                (SyntaxKind::MemberList, 4usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                }
                (SyntaxKind::Member, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Member, 1usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::JsonValue,
                                state: 1usize,
                            }),
                        SyntaxKind::JsonValue,
                    );
                }
                (SyntaxKind::Member, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Colon);
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
                (SyntaxKind::Member, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::JsonString,
                                state: 1usize,
                            }),
                        SyntaxKind::JsonString,
                    );
                }
                (SyntaxKind::JsonValue, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::JsonValue, 1usize) => {
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
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 5usize,
                        }),
                        SyntaxKind::JsonNumber,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 6usize,
                        }),
                        SyntaxKind::TrueLit,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 7usize,
                        }),
                        SyntaxKind::FalseLit,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 8usize,
                        }),
                        SyntaxKind::NullLit,
                    );
                }
                (SyntaxKind::JsonValue, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::JsonObject,
                                state: 4usize,
                            }),
                        SyntaxKind::JsonObject,
                    );
                }
                (SyntaxKind::JsonValue, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::JsonArray,
                                state: 4usize,
                            }),
                        SyntaxKind::JsonArray,
                    );
                }
                (SyntaxKind::JsonValue, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::JsonString,
                                state: 1usize,
                            }),
                        SyntaxKind::JsonString,
                    );
                }
                (SyntaxKind::JsonValue, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::JsonNumber);
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
                (SyntaxKind::JsonValue, 6usize) => {
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
                (SyntaxKind::JsonValue, 7usize) => {
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
                (SyntaxKind::JsonValue, 8usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::NullLit);
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
                (SyntaxKind::ValueList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ValueList, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 3usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                }
                (SyntaxKind::ValueList, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::JsonValue,
                                state: 1usize,
                            }),
                        SyntaxKind::JsonValue,
                    );
                }
                (SyntaxKind::ValueList, 3usize) => {
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
                (SyntaxKind::ValueList, 4usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                }
                (SyntaxKind::JsonString, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::JsonString, 1usize) => {
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }),
                        SyntaxKind::StringToken,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }),
                        SyntaxKind::AtBase,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 4usize,
                        }),
                        SyntaxKind::AtContainer,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 5usize,
                        }),
                        SyntaxKind::AtContext,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 6usize,
                        }),
                        SyntaxKind::AtDirection,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 7usize,
                        }),
                        SyntaxKind::AtGraph,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 8usize,
                        }),
                        SyntaxKind::AtId,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 9usize,
                        }),
                        SyntaxKind::AtImport,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 10usize,
                        }),
                        SyntaxKind::AtIncluded,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 11usize,
                        }),
                        SyntaxKind::AtIndex,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 12usize,
                        }),
                        SyntaxKind::AtJson,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 13usize,
                        }),
                        SyntaxKind::AtLanguage,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 14usize,
                        }),
                        SyntaxKind::AtList,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 15usize,
                        }),
                        SyntaxKind::AtNest,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 16usize,
                        }),
                        SyntaxKind::AtNone,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 17usize,
                        }),
                        SyntaxKind::AtPrefix,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 18usize,
                        }),
                        SyntaxKind::AtPropagate,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 19usize,
                        }),
                        SyntaxKind::AtProtected,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 20usize,
                        }),
                        SyntaxKind::AtReverse,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 21usize,
                        }),
                        SyntaxKind::AtSet,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 22usize,
                        }),
                        SyntaxKind::AtType,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 23usize,
                        }),
                        SyntaxKind::AtValue,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 24usize,
                        }),
                        SyntaxKind::AtVersion,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 25usize,
                        }),
                        SyntaxKind::AtVocab,
                    );
                }
                (SyntaxKind::JsonString, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::StringToken);
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
                (SyntaxKind::JsonString, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtBase);
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
                (SyntaxKind::JsonString, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtContainer);
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
                (SyntaxKind::JsonString, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtContext);
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
                (SyntaxKind::JsonString, 6usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtDirection);
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
                (SyntaxKind::JsonString, 7usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtGraph);
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
                (SyntaxKind::JsonString, 8usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtId);
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
                (SyntaxKind::JsonString, 9usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtImport);
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
                (SyntaxKind::JsonString, 10usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtIncluded);
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
                (SyntaxKind::JsonString, 11usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtIndex);
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
                (SyntaxKind::JsonString, 12usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtJson);
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
                (SyntaxKind::JsonString, 13usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtLanguage);
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
                (SyntaxKind::JsonString, 14usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtList);
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
                (SyntaxKind::JsonString, 15usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtNest);
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
                (SyntaxKind::JsonString, 16usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtNone);
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
                (SyntaxKind::JsonString, 17usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtPrefix);
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
                (SyntaxKind::JsonString, 18usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtPropagate);
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
                (SyntaxKind::JsonString, 19usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtProtected);
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
                (SyntaxKind::JsonString, 20usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtReverse);
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
                (SyntaxKind::JsonString, 21usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtSet);
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
                (SyntaxKind::JsonString, 22usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtType);
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
                (SyntaxKind::JsonString, 23usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtValue);
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
                (SyntaxKind::JsonString, 24usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtVersion);
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
                (SyntaxKind::JsonString, 25usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AtVocab);
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
                (SyntaxKind::AtBase, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtBase);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtContainer, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtContainer);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtContext, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtContext);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtDirection, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtDirection);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtGraph, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtGraph);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtId, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtId);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtImport, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtImport);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtIncluded, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtIncluded);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtIndex, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtIndex);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtJson, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtJson);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtLanguage, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtLanguage);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtList, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtList);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtNest, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtNest);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtNone, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtNone);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtPrefix, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtPrefix);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtPropagate, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtPropagate);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtProtected, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtProtected);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtReverse, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtReverse);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtSet, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtSet);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtType, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtType);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtValue, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtValue);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtVersion, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtVersion);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AtVocab, _) => {
                    let added = state.expect_as(element, SyntaxKind::AtVocab);
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
                (SyntaxKind::Colon, _) => {
                    let added = state.expect_as(element, SyntaxKind::Colon);
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
                (SyntaxKind::FalseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FalseLit);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NullLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::NullLit);
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
                (SyntaxKind::JsonNumber, _) => {
                    let added = state.expect_as(element, SyntaxKind::JsonNumber);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringToken);
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
            SyntaxKind::AtContext => 100isize,
            SyntaxKind::Comma => 2isize,
            SyntaxKind::Colon => 3isize,
            SyntaxKind::SqOpen => 2isize,
            SyntaxKind::SqClose => 2isize,
            SyntaxKind::CurlyOpen => 2isize,
            SyntaxKind::CurlyClose => 2isize,
            _ => 1isize,
        }
    }
    fn bracket_delta(&self) -> i8 {
        match self {
            SyntaxKind::CurlyOpen => 1,
            SyntaxKind::SqOpen => 1,
            SyntaxKind::CurlyClose => -1,
            SyntaxKind::SqClose => -1,
            _ => 0,
        }
    }
    fn min_completion_cost(&self) -> isize {
        match self {
            SyntaxKind::JsonArray => 4isize,
            SyntaxKind::JsonObject => 4isize,
            SyntaxKind::Member => 5isize,
            SyntaxKind::MemberList => 5isize,
            _ => Self::max_error_value(self),
        }
    }
}
