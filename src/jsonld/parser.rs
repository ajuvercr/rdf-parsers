use crate::TokenTrait;
pub type SyntaxNode = rowan::SyntaxNode<Lang>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, logos :: Logos)]
# [logos (subpattern HEX = r#"([0-9]|[A-F]|[a-f])"#)]
# [logos (subpattern JSON_STRING = r#""(([^\x{22}\x{5C}]|\\\u0022|\\\u005C|\\\u002F|\\b|\\f|\\n|\\r|\\t|\\u(?&HEX)(?&HEX)(?&HEX)(?&HEX)))*""#)]
# [logos (subpattern JSON_NUMBER = r#"(-)?(0|[1-9]([0-9])*)(\.([0-9])+)?([eE]([+-])?([0-9])+)?"#)]
#[repr(u16)]
pub enum SyntaxKind {
    Eof = 0,
    #[regex(r"[ \t\n\r]+")]
    WhiteSpace,
    #[regex(r"#[^\n]*", allow_greedy = true)]
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
            SyntaxKind::JsonString => &[],
            SyntaxKind::JsonValue => &[
                SyntaxKind::CurlyOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::NullLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::JsonldDoc => &[
                SyntaxKind::CurlyOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::NullLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Member => &[],
            SyntaxKind::MemberList => &[],
            SyntaxKind::ValueList => &[
                SyntaxKind::CurlyOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::NullLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
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
                SyntaxKind::Colon
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
                SyntaxKind::Colon
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
                SyntaxKind::StringToken => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::JsonValue => match tok {
                SyntaxKind::Colon
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
                SyntaxKind::Colon
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
                SyntaxKind::Colon
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
                SyntaxKind::Colon
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
                SyntaxKind::Colon
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
                _ => 4isize,
            },
            (SyntaxKind::JsonArray, 2usize, _) => match terminal {
                SyntaxKind::Colon => 1isize,
                SyntaxKind::Comma => 1isize,
                SyntaxKind::CurlyClose => 1isize,
                _ => 0,
            },
            (SyntaxKind::JsonArray, 3usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::NullLit
                | SyntaxKind::SqOpen
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonArray, 4usize, _) => match terminal {
                SyntaxKind::SqOpen => 0,
                _ => 4isize,
            },
            (SyntaxKind::JsonObject, 1usize, _) => match terminal {
                SyntaxKind::CurlyClose => 0,
                _ => 2isize,
            },
            (SyntaxKind::JsonObject, 2usize, _) => match terminal {
                SyntaxKind::StringToken | SyntaxKind::CurlyClose => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonObject, 3usize, _) => match terminal {
                SyntaxKind::StringToken => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonObject, 4usize, _) => match terminal {
                SyntaxKind::CurlyOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::JsonString, 1usize, _) => match terminal {
                SyntaxKind::StringToken => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 1usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::NullLit
                | SyntaxKind::SqOpen
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 2usize, _) => match terminal {
                SyntaxKind::CurlyOpen => 0,
                _ => 2isize,
            },
            (SyntaxKind::JsonValue, 3usize, _) => match terminal {
                SyntaxKind::SqOpen => 0,
                _ => 4isize,
            },
            (SyntaxKind::JsonValue, 4usize, _) => match terminal {
                SyntaxKind::StringToken => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 5usize, _) => match terminal {
                SyntaxKind::JsonNumber => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 6usize, _) => match terminal {
                SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 7usize, _) => match terminal {
                SyntaxKind::FalseLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonValue, 8usize, _) => match terminal {
                SyntaxKind::NullLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::JsonldDoc, 1usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::NullLit
                | SyntaxKind::SqOpen
                | SyntaxKind::TrueLit => 0,
                _ => 1isize,
            },
            (SyntaxKind::Member, 1usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
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
                SyntaxKind::StringToken => 0,
                _ => 1isize,
            },
            (SyntaxKind::MemberList, 2usize, _) => match terminal {
                SyntaxKind::StringToken => 0,
                _ => 1isize,
            },
            (SyntaxKind::MemberList, 3usize, _) => match terminal {
                SyntaxKind::JsonNumber => 3isize,
                SyntaxKind::StringToken => 2isize,
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
            (SyntaxKind::MemberList, 4usize, _) => match terminal {
                SyntaxKind::StringToken => 0,
                _ => 1isize,
            },
            (SyntaxKind::ValueList, 2usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
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
            (SyntaxKind::ValueList, 4usize, _) => match terminal {
                SyntaxKind::JsonNumber
                | SyntaxKind::StringToken
                | SyntaxKind::CurlyOpen
                | SyntaxKind::FalseLit
                | SyntaxKind::NullLit
                | SyntaxKind::SqOpen
                | SyntaxKind::TrueLit => 0,
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
        fn from_parts(kind: SyntaxKind, at: usize) -> Self {
            Rule { kind, state: at }
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
            SyntaxKind::Comma => 2isize,
            SyntaxKind::Colon => 3isize,
            SyntaxKind::SqOpen => 4isize,
            SyntaxKind::SqClose => 4isize,
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
            SyntaxKind::JsonArray => 8isize,
            SyntaxKind::JsonObject => 4isize,
            SyntaxKind::Member => 5isize,
            SyntaxKind::MemberList => 5isize,
            _ => Self::max_error_value(self),
        }
    }
    fn deletion_cost(&self) -> isize {
        match self {
            _ => Self::max_error_value(self) * 5,
        }
    }
}
pub mod format {
    use super::{Lang, SyntaxKind, SyntaxNode};
    use crate::format::{Doc, FormatHook, Formatter, NoHook};
    use crate::TokenTrait;
    use rowan::NodeOrToken;
    #[derive(Default)]
    struct Hints {
        space: bool,
        line: bool,
        hardline: bool,
        blankline: bool,
        indent: bool,
        dedent: bool,
        nest: bool,
        glue: bool,
        if_break: Option<&'static str>,
    }
    impl Hints {
        fn to_docs(&self) -> Vec<Doc> {
            let mut v = Vec::new();
            if let Some(s) = self.if_break {
                v.push(Doc::if_break(s));
            }
            if self.blankline {
                v.push(Doc::HardLine);
                v.push(Doc::HardLine);
            } else if self.hardline {
                v.push(Doc::HardLine);
            } else if self.line {
                v.push(Doc::Line);
            } else if self.space {
                v.push(Doc::text(" "));
            }
            v
        }
    }
    fn format_hints(parent: SyntaxKind, token: SyntaxKind) -> (Hints, Hints) {
        match (parent, token) {
            (SyntaxKind::JsonObject, SyntaxKind::MemberList) => (
                Hints {
                    space: false,
                    line: true,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: true,
                    glue: false,
                    if_break: None,
                },
                Hints {
                    space: false,
                    line: false,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: false,
                    glue: false,
                    if_break: None,
                },
            ),
            (SyntaxKind::JsonObject, SyntaxKind::CurlyClose) => (
                Hints {
                    space: false,
                    line: true,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: false,
                    glue: false,
                    if_break: None,
                },
                Hints {
                    space: false,
                    line: false,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: false,
                    glue: false,
                    if_break: None,
                },
            ),
            (SyntaxKind::JsonArray, SyntaxKind::ValueList) => (
                Hints {
                    space: false,
                    line: true,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: true,
                    glue: false,
                    if_break: None,
                },
                Hints {
                    space: false,
                    line: false,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: false,
                    glue: false,
                    if_break: None,
                },
            ),
            (SyntaxKind::JsonArray, SyntaxKind::SqClose) => (
                Hints {
                    space: false,
                    line: true,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: false,
                    glue: false,
                    if_break: None,
                },
                Hints {
                    space: false,
                    line: false,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: false,
                    glue: false,
                    if_break: None,
                },
            ),
            (_, SyntaxKind::Comma) => (
                Hints {
                    space: false,
                    line: false,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: false,
                    glue: false,
                    if_break: None,
                },
                Hints {
                    space: false,
                    line: true,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: false,
                    glue: false,
                    if_break: None,
                },
            ),
            (_, SyntaxKind::Colon) => (
                Hints {
                    space: false,
                    line: false,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: false,
                    glue: false,
                    if_break: None,
                },
                Hints {
                    space: true,
                    line: false,
                    hardline: false,
                    blankline: false,
                    indent: false,
                    dedent: false,
                    nest: false,
                    glue: false,
                    if_break: None,
                },
            ),
            _ => (Hints::default(), Hints::default()),
        }
    }
    fn is_group(kind: SyntaxKind) -> bool {
        matches!(kind, SyntaxKind::JsonObject | SyntaxKind::JsonArray)
    }
    fn is_break(doc: &Doc) -> bool {
        matches!(doc, Doc::Line | Doc::HardLine) || matches ! (doc , Doc :: Text (s) if s == " ")
    }
    #[doc = r" Append `docs`. After a comment the line must end: the first"]
    #[doc = r" break becomes a hard line, or one is inserted. Emitting it here"]
    #[doc = r" rather than with the comment places it inside the next nest, so"]
    #[doc = r" the following line gets that nest's indentation."]
    fn push_all(part: &mut Vec<Doc>, docs: Vec<Doc>, after_comment: &mut bool) {
        let mut have = crate::format::trailing_hardlines(part);
        for doc in docs {
            if std::mem::take(after_comment) && have == 0 {
                part.push(Doc::HardLine);
                have = 1;
            }
            if have > 0 && is_break(&doc) {
                if matches!(doc, Doc::HardLine) {
                    have -= 1;
                }
                continue;
            }
            let trailing = crate::format::trailing_hardlines(part);
            if trailing > 0 && !is_break(&doc) {
                let lead = crate::format::leading_hardlines(&doc);
                crate::format::remove_trailing_hardlines(part, trailing.min(lead));
            }
            have = 0;
            let left = part.iter().rev().find_map(Doc::last_char);
            match doc {
                Doc::Text(ref t) if t == " " && matches!(left, Some(' ' | '\n')) => continue,
                Doc::Line if matches ! (part . last () , Some (Doc :: Text (t)) if t == " ") => {
                    part.pop();
                }
                _ => {}
            }
            if crate::format::needs_space(left, doc.first_char()) || lexes_joined(part, &doc) {
                part.push(Doc::text(" "));
            }
            part.push(doc);
        }
    }
    #[doc = r" Whether the tokens where `part` ends and `doc` starts would lex"]
    #[doc = r" differently when printed without anything in between."]
    fn lexes_joined(part: &[Doc], doc: &Doc) -> bool {
        let left = crate::format::edge_text(part, true);
        let right = crate::format::edge_text(std::slice::from_ref(doc), false);
        match (left, right) {
            (Some(left), Some(right)) => crate::format::glues(left, right, |text| {
                crate::tokenize::<SyntaxKind>(text)
                    .into_iter()
                    .map(|t| t.range)
                    .collect()
            }),
            _ => false,
        }
    }
    #[doc = r" A node wrapping exactly one token (e.g. `Colon` around `;`)."]
    fn is_terminal_wrapper(n: &SyntaxNode) -> bool {
        let mut it = n.children_with_tokens().filter(|c| match c {
            NodeOrToken::Token(t) => t.kind() != SyntaxKind::WhiteSpace,
            NodeOrToken::Node(_) => true,
        });
        matches!(it.next(), Some(NodeOrToken::Token(_))) && it.next().is_none()
    }
    #[doc = r" Format a child node: hook override or hint-driven default,"]
    #[doc = r" wrapped in a group when the grammar says so."]
    fn format_child(node: &SyntaxNode, hook: &dyn FormatHook<Lang>) -> Doc {
        let doc = hooked(node, hook);
        if is_group(node.kind()) {
            Doc::group(doc)
        } else {
            doc
        }
    }
    fn hooked(node: &SyntaxNode, hook: &dyn FormatHook<Lang>) -> Doc {
        let child = |n: &SyntaxNode| format_child(n, hook);
        let default = |n: &SyntaxNode| node_doc(n, hook);
        hook.node(node, &Formatter::new(&child, &default))
            .unwrap_or_else(|| node_doc(node, hook))
    }
    fn ends_line(doc: Option<&Doc>) -> bool {
        match doc {
            Some(Doc::HardLine) => true,
            Some(Doc::Concat(ds)) => ends_line(ds.last()),
            _ => false,
        }
    }
    fn node_doc(node: &SyntaxNode, hook: &dyn FormatHook<Lang>) -> Doc {
        let mut parts: Vec<Vec<Doc>> = vec![vec![]];
        let mut after_comment = false;
        let mut newlines = 0;
        let mut spaced = false;
        let mut own_line: Vec<(String, bool)> = Vec::new();
        for child in node.children_with_tokens() {
            let newlines_before = std::mem::take(&mut newlines);
            let spaced_before = std::mem::take(&mut spaced);
            let (kind, doc) = match &child {
                NodeOrToken::Token(t) => {
                    if t.kind() == SyntaxKind::WhiteSpace {
                        newlines = t.text().matches('\n').count();
                        spaced = true;
                        continue;
                    }
                    if t.kind() == SyntaxKind::Comment {
                        let part = parts.last_mut().unwrap();
                        let mut at = part.len();
                        while at > 0 && is_break(&part[at - 1]) {
                            at -= 1;
                        }
                        let trailing = newlines_before == 0
                            && own_line.is_empty()
                            && !after_comment
                            && at > 0
                            && crate::format::trailing_hardlines(&part[..at]) == 0;
                        if trailing {
                            part.insert(at, Doc::text(t.text().to_string()));
                            part.insert(at, Doc::text(" "));
                            match part.get(at + 2) {
                                None => after_comment = true,
                                Some(Doc::HardLine) => {}
                                Some(_) => part[at + 2] = Doc::HardLine,
                            }
                        } else {
                            let blank =
                                newlines_before > 1 && (!part.is_empty() || !own_line.is_empty());
                            own_line.push((t.text().to_string(), blank));
                        }
                        continue;
                    }
                    if t.kind() == SyntaxKind::ERROR {
                        let sep = if newlines_before > 0 {
                            Doc::HardLine
                        } else if spaced_before {
                            Doc::text(" ")
                        } else {
                            Doc::nil()
                        };
                        (
                            t.kind(),
                            Doc::concat(vec![sep, Doc::text(t.text().to_string())]),
                        )
                    } else {
                        if !hook.keep(&child) {
                            continue;
                        }
                        (t.kind(), Doc::text(t.text().to_string()))
                    }
                }
                NodeOrToken::Node(n) => {
                    if n.kind() == SyntaxKind::ERROR {
                        let sep = if newlines_before > 0 {
                            Doc::HardLine
                        } else if spaced_before {
                            Doc::text(" ")
                        } else {
                            Doc::nil()
                        };
                        let text = Doc::text(n.text().to_string());
                        (n.kind(), Doc::concat(vec![sep, text]))
                    } else {
                        if !hook.keep(&child) {
                            continue;
                        }
                        if is_terminal_wrapper(n) {
                            (n.kind(), Doc::text(n.text().to_string()))
                        } else {
                            (n.kind(), format_child(n, hook))
                        }
                    }
                }
            };
            let (before, after) = format_hints(node.kind(), kind);
            if before.dedent && parts.len() > 1 {
                let nested = parts.pop().unwrap_or_default();
                parts
                    .last_mut()
                    .unwrap()
                    .push(Doc::nest(2, Doc::concat(nested)));
            }
            if before.glue {
                crate::format::strip_trailing_breaks(parts.last_mut().unwrap());
            }
            let mut seq = before.to_docs();
            let blank = match own_line.first() {
                Some((_, blank)) => *blank,
                None => newlines_before > 1,
            };
            if blank && !before.blankline {
                if let Some(i) = seq.iter().position(|d| matches!(d, Doc::HardLine)) {
                    seq.insert(i, Doc::HardLine);
                }
            }
            if !own_line.is_empty() {
                let part = parts.last().unwrap();
                let at_line_start = part.is_empty() || ends_line(part.last());
                let at = match seq.iter().position(is_break) {
                    Some(first) => {
                        let mut end = first;
                        while end < seq.len() && is_break(&seq[end]) {
                            seq[end] = Doc::HardLine;
                            end += 1;
                        }
                        end
                    }
                    None if at_line_start => 0,
                    None => {
                        seq.insert(0, Doc::HardLine);
                        1
                    }
                };
                let mut comments = Vec::new();
                for (i, (text, blank)) in std::mem::take(&mut own_line).into_iter().enumerate() {
                    if blank && i > 0 {
                        comments.push(Doc::HardLine);
                    }
                    comments.push(Doc::text(text));
                    comments.push(Doc::HardLine);
                }
                if newlines_before > 1 {
                    comments.push(Doc::HardLine);
                }
                seq.splice(at..at, comments);
            }
            seq.push(doc);
            if before.nest || after.nest {
                let mut nested = Vec::new();
                push_all(&mut nested, seq, &mut after_comment);
                seq = vec![Doc::nest(2, Doc::concat(nested))];
            }
            push_all(parts.last_mut().unwrap(), seq, &mut after_comment);
            let after_docs = after.to_docs();
            if after.indent {
                parts.push(after_docs);
            } else {
                push_all(parts.last_mut().unwrap(), after_docs, &mut after_comment);
            }
        }
        let part = parts.last_mut().unwrap();
        for (text, blank) in own_line {
            if !after_comment && !part.is_empty() && !ends_line(part.last()) {
                part.push(Doc::HardLine);
            }
            if blank {
                part.push(Doc::HardLine);
            }
            part.push(Doc::text(text));
            part.push(Doc::HardLine);
            after_comment = false;
        }
        if after_comment {
            part.push(Doc::HardLine);
        }
        while parts.len() > 1 {
            let nested = parts.pop().unwrap();
            parts
                .last_mut()
                .unwrap()
                .push(Doc::nest(2, Doc::concat(nested)));
        }
        Doc::concat(parts.pop().unwrap_or_default())
    }
    pub fn to_doc(node: &SyntaxNode) -> Doc {
        to_doc_with(node, &NoHook)
    }
    pub fn to_doc_with(node: &SyntaxNode, hook: &dyn FormatHook<Lang>) -> Doc {
        hooked(node, hook)
    }
    pub fn format(node: &SyntaxNode, width: usize) -> String {
        format_with(node, width, &NoHook)
    }
    pub fn format_with(node: &SyntaxNode, width: usize, hook: &dyn FormatHook<Lang>) -> String {
        let doc = to_doc_with(node, hook);
        let s = crate::format::render(&doc, width);
        let s = s.trim_start_matches('\n').trim_end_matches('\n');
        format!("{s}\n")
    }
}
