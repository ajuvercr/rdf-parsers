use crate::TokenTrait;
pub type SyntaxNode = rowan::SyntaxNode<Lang>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, logos :: Logos)]
# [logos (subpattern HEX = r#"([0-9]|[A-F]|[a-f])"#)]
# [logos (subpattern UCHAR = r#"(\\u(?&HEX)(?&HEX)(?&HEX)(?&HEX)|\\U(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX)(?&HEX))"#)]
# [logos (subpattern IRIREF = r#"<(([^\x{00}-\x{20}<>"{}|^`\\]|(?&UCHAR)))*>"#)]
# [logos (subpattern PN_CHARS_BASE = r#"([A-Z]|[a-z]|[\x{00C0}-\x{00D6}]|[\x{00D8}-\x{00F6}]|[\x{00F8}-\x{02FF}]|[\x{0370}-\x{037D}]|[\x{037F}-\x{1FFF}]|[\x{200C}-\x{200D}]|[\x{2070}-\x{218F}]|[\x{2C00}-\x{2FEF}]|[\x{3001}-\x{D7FF}]|[\x{F900}-\x{FDCF}]|[\x{FDF0}-\x{FFFD}]|[\x{10000}-\x{EFFFF}])"#)]
# [logos (subpattern PN_CHARS_U = r#"((?&PN_CHARS_BASE)|_|:)"#)]
# [logos (subpattern PN_CHARS = r#"((?&PN_CHARS_U)|-|[0-9]|\u00B7|[\x{0300}-\x{036F}]|[\x{203F}-\x{2040}])"#)]
# [logos (subpattern BLANK_NODE_LABEL = r#"_:((?&PN_CHARS_U)|[0-9])((((?&PN_CHARS)|\.))*(?&PN_CHARS))?"#)]
# [logos (subpattern LANGTAG = r#"@([a-zA-Z])+(-([a-zA-Z0-9])+)*"#)]
# [logos (subpattern ECHAR = r#"\\[tbnrf"'\\]"#)]
# [logos (subpattern STRING_LITERAL_QUOTE = r#""(([^\x{22}\x{5C}\x{A}\x{D}]|(?&ECHAR)|(?&UCHAR)))*""#)]
#[repr(u16)]
pub enum SyntaxKind {
    Eof = 0,
    #[regex(r"[ \t\n]+")]
    WhiteSpace,
    #[regex(r"#[^\n]+", allow_greedy = true)]
    Comment,
    #[doc = r" producings"]
    Literal,
    NtriplesDoc,
    Object,
    Predicate,
    Subject,
    Triple,
    #[doc = r" terminals"]
    #[token(".")]
    Stop,
    #[token("^^")]
    Datatype,
    #[regex("(?&BLANK_NODE_LABEL)")]
    BlankNodeLabel,
    #[regex("(?&IRIREF)")]
    Iriref,
    #[regex("(?&LANGTAG)")]
    Langtag,
    #[regex("(?&STRING_LITERAL_QUOTE)")]
    StringLiteralQuote,
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
                SyntaxKind::NtriplesDoc => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Triple => Rule {
                    kind,
                    state: 4usize,
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
                    state: 6usize,
                },
                SyntaxKind::Stop => Rule { kind, state: 0 },
                SyntaxKind::Datatype => Rule { kind, state: 0 },
                SyntaxKind::BlankNodeLabel => Rule { kind, state: 0 },
                SyntaxKind::Iriref => Rule { kind, state: 0 },
                SyntaxKind::Langtag => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralQuote => Rule { kind, state: 0 },
                _ => panic!("Unknown rule kind {:?}", kind),
            }
        }
    }
    pub fn first_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::Literal => &[],
            SyntaxKind::NtriplesDoc => &[],
            SyntaxKind::Object => &[],
            SyntaxKind::Predicate => &[],
            SyntaxKind::Subject => &[],
            SyntaxKind::Triple => &[],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::BlankNodeLabel => &[SyntaxKind::BlankNodeLabel],
            SyntaxKind::Iriref => &[SyntaxKind::Iriref],
            SyntaxKind::Langtag => &[SyntaxKind::Langtag],
            SyntaxKind::StringLiteralQuote => &[SyntaxKind::StringLiteralQuote],
            _ => &[],
        }
    }
    #[doc = r" Returns the minimum error cost that `kind` must incur when `tok`"]
    #[doc = r" is the current token.  0 means the token is reachable (or the rule"]
    #[doc = r" is nullable); positive means the rule cannot make progress without"]
    #[doc = r" at least that much error cost."]
    pub fn min_error_for_token(kind: SyntaxKind, tok: SyntaxKind) -> isize {
        match kind {
            SyntaxKind::Literal => match tok {
                SyntaxKind::Datatype
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::StringLiteralQuote => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Object => match tok {
                SyntaxKind::BlankNodeLabel
                | SyntaxKind::Datatype
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::StringLiteralQuote => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Predicate => match tok {
                SyntaxKind::Iriref => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Subject => match tok {
                SyntaxKind::BlankNodeLabel | SyntaxKind::Iriref => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Triple => match tok {
                SyntaxKind::BlankNodeLabel
                | SyntaxKind::Datatype
                | SyntaxKind::Iriref
                | SyntaxKind::Langtag
                | SyntaxKind::Stop
                | SyntaxKind::StringLiteralQuote => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Stop => match tok {
                SyntaxKind::Stop => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::Datatype => match tok {
                SyntaxKind::Datatype => 0,
                _ => kind.max_error_value(),
            },
            SyntaxKind::BlankNodeLabel => match tok {
                SyntaxKind::BlankNodeLabel => 0,
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
            SyntaxKind::StringLiteralQuote => match tok {
                SyntaxKind::StringLiteralQuote => 0,
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
            (SyntaxKind::Literal, 4usize, _) => match terminal {
                SyntaxKind::BlankNodeLabel => 2isize,
                SyntaxKind::Iriref => 1isize,
                SyntaxKind::Langtag => 2isize,
                SyntaxKind::StringLiteralQuote => 2isize,
                SyntaxKind::Stop => 2isize,
                _ => 0,
            },
            (SyntaxKind::Literal, 6usize, _) => match terminal {
                SyntaxKind::StringLiteralQuote => 0,
                _ => 1isize,
            },
            (SyntaxKind::Literal, 3usize, _) => match terminal {
                SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::Literal, 5usize, _) => match terminal {
                SyntaxKind::Langtag => 0,
                _ => 1isize,
            },
            (SyntaxKind::Literal, 2usize, _) => match terminal {
                SyntaxKind::Langtag | SyntaxKind::Datatype => 0,
                _ => 1isize,
            },
            (SyntaxKind::NtriplesDoc, 2usize, _) => match terminal {
                SyntaxKind::BlankNodeLabel | SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::Object, 2usize, _) => match terminal {
                SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::Object, 1usize, _) => match terminal {
                SyntaxKind::Langtag => 1isize,
                SyntaxKind::Datatype => 1isize,
                SyntaxKind::Stop => 1isize,
                _ => 0,
            },
            (SyntaxKind::Object, 4usize, _) => match terminal {
                SyntaxKind::StringLiteralQuote => 0,
                _ => 1isize,
            },
            (SyntaxKind::Object, 3usize, _) => match terminal {
                SyntaxKind::BlankNodeLabel => 0,
                _ => 1isize,
            },
            (SyntaxKind::Predicate, 1usize, _) => match terminal {
                SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::Subject, 1usize, _) => match terminal {
                SyntaxKind::BlankNodeLabel | SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::Subject, 2usize, _) => match terminal {
                SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::Subject, 3usize, _) => match terminal {
                SyntaxKind::BlankNodeLabel => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triple, 3usize, _) => match terminal {
                SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triple, 2usize, _) => match terminal {
                SyntaxKind::Langtag => 1isize,
                SyntaxKind::Datatype => 1isize,
                SyntaxKind::Stop => 1isize,
                _ => 0,
            },
            (SyntaxKind::Triple, 4usize, _) => match terminal {
                SyntaxKind::BlankNodeLabel | SyntaxKind::Iriref => 0,
                _ => 1isize,
            },
            (SyntaxKind::Triple, 1usize, _) => match terminal {
                SyntaxKind::Stop => 0,
                _ => 8isize,
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
                (SyntaxKind::NtriplesDoc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NtriplesDoc, 1usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 0usize,
                    }));
                }
                (SyntaxKind::NtriplesDoc, 2usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Triple,
                                state: 4usize,
                            }),
                        SyntaxKind::Triple,
                    );
                }
                (SyntaxKind::Triple, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Triple, 1usize) => {
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
                (SyntaxKind::Triple, 2usize) => {
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
                (SyntaxKind::Triple, 3usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Predicate,
                                state: 1usize,
                            }),
                        SyntaxKind::Predicate,
                    );
                }
                (SyntaxKind::Triple, 4usize) => {
                    state.add_element_checked(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Subject,
                                state: 1usize,
                            }),
                        SyntaxKind::Subject,
                    );
                }
                (SyntaxKind::Subject, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Subject, 1usize) => {
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }),
                        SyntaxKind::Iriref,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }),
                        SyntaxKind::BlankNodeLabel,
                    );
                }
                (SyntaxKind::Subject, 2usize) => {
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
                (SyntaxKind::Subject, 3usize) => {
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
                (SyntaxKind::Predicate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Predicate, 1usize) => {
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
                (SyntaxKind::Object, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Object, 1usize) => {
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 2usize,
                        }),
                        SyntaxKind::Iriref,
                    );
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 3usize,
                        }),
                        SyntaxKind::BlankNodeLabel,
                    );
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                }
                (SyntaxKind::Object, 2usize) => {
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
                                kind: SyntaxKind::Literal,
                                state: 6usize,
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
                        state: 0usize,
                    }));
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 2usize,
                    }));
                }
                (SyntaxKind::Literal, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 4usize,
                    }));
                    state.add_element_checked(
                        element.pop_push(Rule {
                            kind: self.kind,
                            state: 5usize,
                        }),
                        SyntaxKind::Langtag,
                    );
                }
                (SyntaxKind::Literal, 3usize) => {
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
                (SyntaxKind::Literal, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Datatype);
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
                (SyntaxKind::Literal, 5usize) => {
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
                (SyntaxKind::Literal, 6usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StringLiteralQuote);
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
                (SyntaxKind::Stop, _) => {
                    let added = state.expect_as(element, SyntaxKind::Stop);
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
                (SyntaxKind::BlankNodeLabel, _) => {
                    let added = state.expect_as(element, SyntaxKind::BlankNodeLabel);
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
                (SyntaxKind::StringLiteralQuote, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralQuote);
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
            SyntaxKind::Stop => 8isize,
            _ => 1isize,
        }
    }
    fn bracket_delta(&self) -> i8 {
        match self {
            _ => 0,
        }
    }
    fn min_completion_cost(&self) -> isize {
        match self {
            SyntaxKind::Triple => 11isize,
            _ => Self::max_error_value(self),
        }
    }
}
