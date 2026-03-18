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
    #[regex(r"\\#[^\n]+", allow_greedy = true)]
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
    #[regex("(?&PNAME_LN)")]
    PnameLn,
    #[token(";")]
    Colon,
    #[regex("(?&ANON)")]
    Anon,
    #[token("(")]
    ClOpen,
    #[token("@prefix")]
    PrefixToken,
    #[token("^^")]
    Datatype,
    #[regex("(?&STRING_LITERAL_LONG_QUOTE)")]
    StringLiteralLongQuote,
    #[regex("(?&DECIMAL)")]
    Decimal,
    #[regex("(?&BLANK_NODE_LABEL)")]
    BlankNodeLabel,
    #[token(",")]
    Comma,
    #[token("]")]
    SqClose,
    #[regex("(?&STRING_LITERAL_QUOTE)")]
    StringLiteralQuote,
    #[token("BASE")]
    BaseLit,
    #[token("PREFIX")]
    PrefixLit,
    #[regex("(?&STRING_LITERAL_SINGLE_QUOTE)")]
    StringLiteralSingleQuote,
    #[token("true")]
    TrueLit,
    #[regex("(?&STRING_LITERAL_LONG_SINGLE_QUOTE)")]
    StringLiteralLongSingleQuote,
    #[regex("(?&LANGTAG)")]
    Langtag,
    #[token("@base")]
    BaseToken,
    #[token(")")]
    ClClose,
    #[regex("(?&IRIREF)")]
    Iriref,
    #[token("a")]
    Alit,
    #[regex("(?&DOUBLE)")]
    Double,
    #[token(".")]
    Stop,
    #[token("[")]
    SqOpen,
    #[regex("(?&PNAME_NS)")]
    PnameNs,
    #[regex("(?&INTEGER)")]
    Integer,
    #[token("false")]
    FalseLit,
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
                SyntaxKind::PnameLn => Rule { kind, state: 0 },
                SyntaxKind::Colon => Rule { kind, state: 0 },
                SyntaxKind::Anon => Rule { kind, state: 0 },
                SyntaxKind::ClOpen => Rule { kind, state: 0 },
                SyntaxKind::PrefixToken => Rule { kind, state: 0 },
                SyntaxKind::Datatype => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLongQuote => Rule { kind, state: 0 },
                SyntaxKind::Decimal => Rule { kind, state: 0 },
                SyntaxKind::BlankNodeLabel => Rule { kind, state: 0 },
                SyntaxKind::Comma => Rule { kind, state: 0 },
                SyntaxKind::SqClose => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralQuote => Rule { kind, state: 0 },
                SyntaxKind::BaseLit => Rule { kind, state: 0 },
                SyntaxKind::PrefixLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralSingleQuote => Rule { kind, state: 0 },
                SyntaxKind::TrueLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLongSingleQuote => Rule { kind, state: 0 },
                SyntaxKind::Langtag => Rule { kind, state: 0 },
                SyntaxKind::BaseToken => Rule { kind, state: 0 },
                SyntaxKind::ClClose => Rule { kind, state: 0 },
                SyntaxKind::Iriref => Rule { kind, state: 0 },
                SyntaxKind::Alit => Rule { kind, state: 0 },
                SyntaxKind::Double => Rule { kind, state: 0 },
                SyntaxKind::Stop => Rule { kind, state: 0 },
                SyntaxKind::SqOpen => Rule { kind, state: 0 },
                SyntaxKind::PnameNs => Rule { kind, state: 0 },
                SyntaxKind::Integer => Rule { kind, state: 0 },
                SyntaxKind::FalseLit => Rule { kind, state: 0 },
                _ => panic!("Unknown rule kind {:?}", kind),
            }
        }
    }
    pub fn first_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::Verb => &[SyntaxKind::Alit],
            SyntaxKind::PrefixId => &[SyntaxKind::PrefixToken],
            SyntaxKind::BlankNode => &[SyntaxKind::SqOpen],
            SyntaxKind::Iri => &[],
            SyntaxKind::Statement => &[
                SyntaxKind::SqOpen,
                SyntaxKind::ClOpen,
                SyntaxKind::PrefixToken,
                SyntaxKind::BaseLit,
                SyntaxKind::PrefixLit,
                SyntaxKind::BaseToken,
            ],
            SyntaxKind::Subject => &[SyntaxKind::SqOpen, SyntaxKind::ClOpen],
            SyntaxKind::Triples => &[SyntaxKind::SqOpen, SyntaxKind::ClOpen],
            SyntaxKind::Base => &[SyntaxKind::BaseToken],
            SyntaxKind::PredicateObjectList => &[SyntaxKind::Alit],
            SyntaxKind::Collection => &[SyntaxKind::ClOpen],
            SyntaxKind::NumericLiteral => &[],
            SyntaxKind::BooleanLiteral => &[SyntaxKind::TrueLit, SyntaxKind::FalseLit],
            SyntaxKind::Directive => &[
                SyntaxKind::BaseToken,
                SyntaxKind::BaseLit,
                SyntaxKind::PrefixToken,
                SyntaxKind::PrefixLit,
            ],
            SyntaxKind::SparqlBase => &[SyntaxKind::BaseLit],
            SyntaxKind::BlankNodePropertyList => &[SyntaxKind::SqOpen],
            SyntaxKind::ObjectList => &[
                SyntaxKind::ClOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::TurtleDoc => &[
                SyntaxKind::BaseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BaseToken,
                SyntaxKind::ClOpen,
                SyntaxKind::PrefixToken,
                SyntaxKind::PrefixLit,
            ],
            SyntaxKind::Rdfliteral => &[],
            SyntaxKind::Literal => &[SyntaxKind::TrueLit, SyntaxKind::FalseLit],
            SyntaxKind::Predicate => &[],
            SyntaxKind::MyString => &[],
            SyntaxKind::PrefixedName => &[],
            SyntaxKind::SparqlPrefix => &[SyntaxKind::PrefixLit],
            SyntaxKind::Object => &[
                SyntaxKind::ClOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::PnameLn => &[SyntaxKind::PnameLn],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::Anon => &[SyntaxKind::Anon],
            SyntaxKind::ClOpen => &[SyntaxKind::ClOpen],
            SyntaxKind::PrefixToken => &[SyntaxKind::PrefixToken],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::StringLiteralLongQuote => &[SyntaxKind::StringLiteralLongQuote],
            SyntaxKind::Decimal => &[SyntaxKind::Decimal],
            SyntaxKind::BlankNodeLabel => &[SyntaxKind::BlankNodeLabel],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::StringLiteralQuote => &[SyntaxKind::StringLiteralQuote],
            SyntaxKind::BaseLit => &[SyntaxKind::BaseLit],
            SyntaxKind::PrefixLit => &[SyntaxKind::PrefixLit],
            SyntaxKind::StringLiteralSingleQuote => &[SyntaxKind::StringLiteralSingleQuote],
            SyntaxKind::TrueLit => &[SyntaxKind::TrueLit],
            SyntaxKind::StringLiteralLongSingleQuote => &[SyntaxKind::StringLiteralLongSingleQuote],
            SyntaxKind::Langtag => &[SyntaxKind::Langtag],
            SyntaxKind::BaseToken => &[SyntaxKind::BaseToken],
            SyntaxKind::ClClose => &[SyntaxKind::ClClose],
            SyntaxKind::Iriref => &[SyntaxKind::Iriref],
            SyntaxKind::Alit => &[SyntaxKind::Alit],
            SyntaxKind::Double => &[SyntaxKind::Double],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::PnameNs => &[SyntaxKind::PnameNs],
            SyntaxKind::Integer => &[SyntaxKind::Integer],
            SyntaxKind::FalseLit => &[SyntaxKind::FalseLit],
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
                (SyntaxKind::TurtleDoc, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Statement,
                                state: 1usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Statement, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Statement, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::Statement, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Directive,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Triples,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Directive, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PrefixId,
                                state: 4usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Base,
                                state: 3usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SparqlPrefix,
                                state: 3usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SparqlBase,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::Directive, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixId, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixId, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::PrefixId, 4usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::PrefixToken, 100isize);
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
                (SyntaxKind::PrefixId, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iriref, 2isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::PnameNs, 2isize);
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
                (SyntaxKind::Base, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::Base, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Base, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iriref, 2isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BaseToken, 100isize);
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
                (SyntaxKind::SparqlBase, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BaseLit, 10isize);
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
                (SyntaxKind::SparqlBase, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SparqlBase, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iriref, 2isize);
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
                (SyntaxKind::SparqlPrefix, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SparqlPrefix, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::PnameNs, 2isize);
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
                        state.expect_as_inline(element, SyntaxKind::PrefixLit, 10isize);
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
                (SyntaxKind::SparqlPrefix, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iriref, 2isize);
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
                (SyntaxKind::Triples, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNodePropertyList,
                                state: 3usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Subject,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Triples, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PredicateObjectList,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::Triples, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Triples, 2usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PredicateObjectList,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::PredicateObjectList, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ObjectList,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::PredicateObjectList, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Verb,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PredicateObjectList, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Colon, 10isize);
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
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PredicateObjectList, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ObjectList,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::PredicateObjectList, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Verb,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ObjectList, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Object,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ObjectList, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Object,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ObjectList, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 10isize);
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
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Verb, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Verb, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Predicate,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Alit, 10isize);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNode,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Collection,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::Predicate, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Predicate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Object, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNode,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Collection,
                                state: 4usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNodePropertyList,
                                state: 3usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Literal,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Object, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Literal, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Rdfliteral,
                                state: 6usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NumericLiteral,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BooleanLiteral,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Literal, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BlankNodePropertyList, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::SqOpen, 8isize);
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
                (SyntaxKind::BlankNodePropertyList, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SqClose, 8isize);
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
                (SyntaxKind::BlankNodePropertyList, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PredicateObjectList,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::Collection, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Object,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ClClose, 8isize);
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
                (SyntaxKind::Collection, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ClOpen, 8isize);
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
                (SyntaxKind::NumericLiteral, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NumericLiteral, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::Integer, 2isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::Decimal, 2isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Double, 2isize);
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
                (SyntaxKind::Rdfliteral, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Rdfliteral, 1usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::Langtag, 2isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::Datatype, 10isize);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MyString,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BooleanLiteral, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BooleanLiteral, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::TrueLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::FalseLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StringLiteralQuote, 2isize);
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
                    let (matched, fb) = state.expect_as_inline(
                        element,
                        SyntaxKind::StringLiteralSingleQuote,
                        2isize,
                    );
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
                    let (matched, fb) = state.expect_as_inline(
                        element,
                        SyntaxKind::StringLiteralLongSingleQuote,
                        2isize,
                    );
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StringLiteralLongQuote, 2isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Iriref, 2isize);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PrefixedName,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PrefixedName, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixedName, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::PnameLn, 2isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::PnameNs, 2isize);
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
                (SyntaxKind::BlankNode, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BlankNodeLabel, 2isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Anon, 2isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::SqOpen, 8isize);
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
                (SyntaxKind::BlankNode, 4usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SqClose, 8isize);
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
                (SyntaxKind::PnameLn, _) => {
                    let added = state.expect_as(element, SyntaxKind::PnameLn, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Colon, _) => {
                    let added = state.expect_as(element, SyntaxKind::Colon, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Anon, _) => {
                    let added = state.expect_as(element, SyntaxKind::Anon, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ClOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::ClOpen, 8isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::PrefixToken, 100isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Datatype, _) => {
                    let added = state.expect_as(element, SyntaxKind::Datatype, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteralLongQuote, _) => {
                    let added =
                        state.expect_as(element, SyntaxKind::StringLiteralLongQuote, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Decimal, _) => {
                    let added = state.expect_as(element, SyntaxKind::Decimal, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BlankNodeLabel, _) => {
                    let added = state.expect_as(element, SyntaxKind::BlankNodeLabel, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Comma, _) => {
                    let added = state.expect_as(element, SyntaxKind::Comma, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SqClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::SqClose, 8isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteralQuote, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralQuote, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BaseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BaseLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::PrefixLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteralSingleQuote, _) => {
                    let added =
                        state.expect_as(element, SyntaxKind::StringLiteralSingleQuote, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TrueLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::TrueLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteralLongSingleQuote, _) => {
                    let added =
                        state.expect_as(element, SyntaxKind::StringLiteralLongSingleQuote, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Langtag, _) => {
                    let added = state.expect_as(element, SyntaxKind::Langtag, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BaseToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::BaseToken, 100isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ClClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::ClClose, 8isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Iriref, _) => {
                    let added = state.expect_as(element, SyntaxKind::Iriref, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Alit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Alit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Double, _) => {
                    let added = state.expect_as(element, SyntaxKind::Double, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Stop, _) => {
                    let added = state.expect_as(element, SyntaxKind::Stop, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SqOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::SqOpen, 8isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PnameNs, _) => {
                    let added = state.expect_as(element, SyntaxKind::PnameNs, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Integer, _) => {
                    let added = state.expect_as(element, SyntaxKind::Integer, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::FalseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FalseLit, 10isize);
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
    fn ending_tokens(&self) -> &'static [SyntaxKind] {
        &[]
    }
    fn term_type(&self) -> Option<crate::TermType> {
        match self {
            SyntaxKind::Object => Some(crate::TermType::Object),
            SyntaxKind::Subject => Some(crate::TermType::Subject),
            SyntaxKind::Predicate => Some(crate::TermType::Predicate),
            SyntaxKind::Verb => Some(crate::TermType::Predicate),
            _ => None,
        }
    }
}
