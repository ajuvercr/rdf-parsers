use crate::TokenTrait;
pub type SyntaxNode = rowan::SyntaxNode<Lang>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, logos :: Logos)]
# [logos (subpattern IRIREF = r#"<([^\x{00}-\x{20}<>"{}|^`\\])*>"#)]
# [logos (subpattern PN_CHARS_BASE = r#"([A-Z]|[a-z]|[\x{00C0}-\x{00D6}]|[\x{00D8}-\x{00F6}]|[\x{00F8}-\x{02FF}]|[\x{0370}-\x{037D}]|[\x{037F}-\x{1FFF}]|[\x{200C}-\x{200D}]|[\x{2070}-\x{218F}]|[\x{2C00}-\x{2FEF}]|[\x{3001}-\x{D7FF}]|[\x{F900}-\x{FDCF}]|[\x{FDF0}-\x{FFFD}]|[\x{10000}-\x{EFFFF}])"#)]
# [logos (subpattern PN_CHARS_U = r#"((?&PN_CHARS_BASE)|_)"#)]
# [logos (subpattern PN_CHARS = r#"((?&PN_CHARS_U)|-|[0-9]|\u00B7|[\x{0300}-\x{036F}]|[\x{203F}-\x{2040}])"#)]
# [logos (subpattern PN_PREFIX = r#"(?&PN_CHARS_BASE)((((?&PN_CHARS)|\.))*(?&PN_CHARS))?"#)]
# [logos (subpattern PNAME_NS = r#"((?&PN_PREFIX))?:"#)]
# [logos (subpattern HEX = r#"([0-9]|[A-F]|[a-f])"#)]
# [logos (subpattern PERCENT = r#"%(?&HEX)(?&HEX)"#)]
# [logos (subpattern PN_LOCAL_ESC = r#"\\(_|~|\.|-|!|\$|&|'|\(|\)|\*|\+|,|;|=|/|\?|#|@|%)"#)]
# [logos (subpattern PLX = r#"((?&PERCENT)|(?&PN_LOCAL_ESC))"#)]
# [logos (subpattern PN_LOCAL = r#"((?&PN_CHARS_U)|:|[0-9]|(?&PLX))((((?&PN_CHARS)|\.|:|(?&PLX)))*((?&PN_CHARS)|:|(?&PLX)))?"#)]
# [logos (subpattern PNAME_LN = r#"(?&PNAME_NS)(?&PN_LOCAL)"#)]
# [logos (subpattern BLANK_NODE_LABEL = r#"_:((?&PN_CHARS_U)|[0-9])((((?&PN_CHARS)|\.))*(?&PN_CHARS))?"#)]
# [logos (subpattern VARNAME = r#"((?&PN_CHARS_U)|[0-9])(((?&PN_CHARS_U)|[0-9]|\u00B7|[\x{0300}-\x{036F}]|[\x{203F}-\x{2040}]))*"#)]
# [logos (subpattern VAR1 = r#"\?(?&VARNAME)"#)]
# [logos (subpattern VAR2 = r#"\$(?&VARNAME)"#)]
# [logos (subpattern LANGTAG = r#"@([a-zA-Z])+(-([a-zA-Z0-9])+)*"#)]
# [logos (subpattern INTEGER = r#"([0-9])+"#)]
# [logos (subpattern DECIMAL = r#"([0-9])*\.([0-9])+"#)]
# [logos (subpattern EXPONENT = r#"[eE]([+-])?([0-9])+"#)]
# [logos (subpattern DOUBLE = r#"(([0-9])+\.([0-9])*(?&EXPONENT)|\.([0-9])+(?&EXPONENT)|([0-9])+(?&EXPONENT))"#)]
# [logos (subpattern INTEGER_POSITIVE = r#"\+(?&INTEGER)"#)]
# [logos (subpattern DECIMAL_POSITIVE = r#"\+(?&DECIMAL)"#)]
# [logos (subpattern DOUBLE_POSITIVE = r#"\+(?&DOUBLE)"#)]
# [logos (subpattern INTEGER_NEGATIVE = r#"-(?&INTEGER)"#)]
# [logos (subpattern DECIMAL_NEGATIVE = r#"-(?&DECIMAL)"#)]
# [logos (subpattern DOUBLE_NEGATIVE = r#"-(?&DOUBLE)"#)]
# [logos (subpattern ECHAR = r#"\\[tbnrf\\"']"#)]
# [logos (subpattern STRING_LITERAL1 = r#"'(([^\x{27}\x{5C}\x{A}\x{D}]|(?&ECHAR)))*'"#)]
# [logos (subpattern STRING_LITERAL2 = r#""(([^\x{22}\x{5C}\x{A}\x{D}]|(?&ECHAR)))*""#)]
# [logos (subpattern STRING_LITERAL_LONG1 = r#"'''((('|''))?([^'\\]|(?&ECHAR)))*'''"#)]
# [logos (subpattern STRING_LITERAL_LONG2 = r#""""((("|""))?([^"\\]|(?&ECHAR)))*""""#)]
# [logos (subpattern WS = r#"(\u0020|\u0009|\u000D|\u000A)"#)]
# [logos (subpattern NIL = r#"\(((?&WS))*\)"#)]
# [logos (subpattern ANON = r#"\[((?&WS))*\]"#)]
#[repr(u16)]
pub enum SyntaxKind {
    Eof = 0,
    #[regex(r"[ \t\n]+")]
    WhiteSpace,
    #[regex(r"\\#[^\n]+", allow_greedy = true)]
    Comment,
    #[doc = r" producings"]
    Add,
    AdditiveExpression,
    Aggregate,
    ArgList,
    AskQuery,
    BaseDecl,
    Bind,
    BlankNode,
    BlankNodePropertyList,
    BlankNodePropertyListPath,
    BooleanLiteral,
    BrackettedExpression,
    BuiltInCall,
    Clear,
    Collection,
    CollectionPath,
    ConditionalAndExpression,
    ConditionalOrExpression,
    Constraint,
    ConstructQuery,
    ConstructTemplate,
    ConstructTriples,
    Copy,
    Create,
    DataBlock,
    DataBlockValue,
    DatasetClause,
    DefaultGraphClause,
    DeleteClause,
    DeleteData,
    DeleteWhere,
    DescribeQuery,
    Drop,
    ExistsFunc,
    Expression,
    ExpressionList,
    Filter,
    FunctionCall,
    GraphGraphPattern,
    GraphNode,
    GraphNodePath,
    GraphOrDefault,
    GraphPatternNotTriples,
    GraphRef,
    GraphRefAll,
    GraphTerm,
    GroupClause,
    GroupCondition,
    GroupGraphPattern,
    GroupGraphPatternSub,
    GroupOrUnionGraphPattern,
    HavingClause,
    HavingCondition,
    InlineData,
    InlineDataFull,
    InlineDataOneVar,
    InsertClause,
    InsertData,
    LimitClause,
    LimitOffsetClauses,
    Load,
    MinusGraphPattern,
    Modify,
    Move,
    MultiplicativeExpression,
    NamedGraphClause,
    NotExistsFunc,
    NumericExpression,
    NumericLiteral,
    NumericLiteralNegative,
    NumericLiteralPositive,
    NumericLiteralUnsigned,
    Object,
    ObjectList,
    ObjectListPath,
    ObjectPath,
    OffsetClause,
    OptionalGraphPattern,
    OrderClause,
    OrderCondition,
    Path,
    PathAlternative,
    PathElt,
    PathEltOrInverse,
    PathMod,
    PathNegatedPropertySet,
    PathOneInPropertySet,
    PathPrimary,
    PathSequence,
    PrefixDecl,
    PrefixedName,
    PrimaryExpression,
    Prologue,
    PropertyList,
    PropertyListNotEmpty,
    PropertyListPath,
    PropertyListPathNotEmpty,
    QuadData,
    QuadPattern,
    Quads,
    QuadsNotTriples,
    Query,
    QueryUnit,
    Rdfliteral,
    RegexExpression,
    RelationalExpression,
    SelectClause,
    SelectQuery,
    ServiceGraphPattern,
    SolutionModifier,
    SourceSelector,
    StrReplaceExpression,
    MyString,
    SubSelect,
    SubstringExpression,
    TriplesBlock,
    TriplesNode,
    TriplesNodePath,
    TriplesSameSubject,
    TriplesSameSubjectPath,
    TriplesTemplate,
    UnaryExpression,
    Update,
    Update1,
    UpdateUnit,
    UsingClause,
    ValueLogical,
    ValuesClause,
    Var,
    VarOrIri,
    VarOrTerm,
    Verb,
    VerbPath,
    VerbSimple,
    WhereClause,
    Iri,
    IriOrFunction,
    #[doc = r" terminals"]
    #[token("NOW")]
    NowLit,
    #[token("REGEX")]
    RegexLit,
    #[regex("(?&ANON)")]
    Anon,
    #[token("isLITERAL")]
    IsLiteralLit,
    #[token("RAND")]
    RandLit,
    #[token("EXISTS")]
    ExistsLit,
    #[token("CREATE")]
    CreateLit,
    #[token("STRLEN")]
    StrlenLit,
    #[token("<=")]
    Lte,
    #[token("SERVICE")]
    ServiceLit,
    #[token("DESC")]
    DescLit,
    #[regex("(?&PNAME_LN)")]
    PnameLn,
    #[token("LOAD")]
    LoadLit,
    #[token("MAX")]
    MaxLit,
    #[token("isURI")]
    IsUriLit,
    #[token("INSERT")]
    InsertLit,
    #[token("NOT")]
    NotLit,
    #[token("TIMEZONE")]
    TimezoneLit,
    #[token("SUM")]
    SumLit,
    #[token("ASK")]
    AskLit,
    #[token("false")]
    FalseLit,
    #[token("[")]
    SqOpen,
    #[token("UCASE")]
    UcaseLit,
    #[token("MONTH")]
    MonthLit,
    #[token("COALESCE")]
    CoalesceLit,
    #[token("]")]
    SqClose,
    #[token("BIND")]
    BindLit,
    #[token("STRBEFORE")]
    StrbeforeLit,
    #[token("*")]
    Star,
    #[token("REPLACE")]
    ReplaceLit,
    #[token("^^")]
    Datatype,
    #[token("SHA256")]
    Sha256Lit,
    #[token("(")]
    BrOpen,
    #[token("UUID")]
    UuidLit,
    #[token("LANGMATCHES")]
    LangmatchesLit,
    #[token("MIN")]
    MinLit,
    #[regex("(?&VAR2)")]
    Var2,
    #[token("FROM")]
    FromLit,
    #[regex("(?&STRING_LITERAL_LONG2)")]
    StringLiteralLong2,
    #[token("!=")]
    Neq,
    #[regex("(?&STRING_LITERAL2)")]
    StringLiteral2,
    #[token("STRSTARTS")]
    StrstartsLit,
    #[token("STR")]
    StrLit,
    #[token("SHA384")]
    Sha384Lit,
    #[token("ORDER")]
    OrderLit,
    #[token("CONTAINS")]
    ContainsLit,
    #[token("DROP")]
    DropLit,
    #[token("IRI")]
    IriLit,
    #[token("GROUP")]
    GroupLit,
    #[token("CLEAR")]
    ClearLit,
    #[token(".")]
    Stop,
    #[token("MINUTES")]
    MinutesLit,
    #[token("AVG")]
    AvgLit,
    #[regex("(?&STRING_LITERAL_LONG1)")]
    StringLiteralLong1,
    #[token("LIMIT")]
    LimitLit,
    #[token("SUBSTR")]
    SubstrLit,
    #[token("DELETE")]
    DeleteLit,
    #[token("LCASE")]
    LcaseLit,
    #[token("STRAFTER")]
    StrafterLit,
    #[token("SAMPLE")]
    SampleLit,
    #[regex("(?&PNAME_NS)")]
    PnameNs,
    #[token("TO")]
    ToLit,
    #[token("UNDEF")]
    UndefLit,
    #[token("FILTER")]
    FilterLit,
    #[token("||")]
    Pipe2,
    #[regex("(?&DOUBLE_POSITIVE)")]
    DoublePositive,
    #[token("BOUND")]
    BoundLit,
    #[token("SHA512")]
    Sha512Lit,
    #[regex("(?&INTEGER_POSITIVE)")]
    IntegerPositive,
    #[token("sameTerm")]
    SameTermLit,
    #[token("ROUND")]
    RoundLit,
    #[token("{")]
    ClOpen,
    #[token("=")]
    Eq,
    #[token("USING")]
    UsingLit,
    #[token("DEFAULT")]
    DefaultLit,
    #[token(">")]
    Gt,
    #[token(">=")]
    Gte,
    #[token("ALL")]
    AllLit,
    #[regex("(?&IRIREF)")]
    Iriref,
    #[token("STRUUID")]
    StruuidLit,
    #[token("PREFIX")]
    SparqlPrefixToken,
    #[token("!")]
    Bang,
    #[token("DAY")]
    DayLit,
    #[token("isBLANK")]
    IsBlankLit,
    #[regex("(?&INTEGER_NEGATIVE)")]
    IntegerNegative,
    #[token("BY")]
    ByLit,
    #[regex("(?&DOUBLE_NEGATIVE)")]
    DoubleNegative,
    #[regex("(?&BLANK_NODE_LABEL)")]
    BlankNodeLabel,
    #[token("TZ")]
    TzLit,
    #[token("DELETE WHERE")]
    DeleteWhereLit,
    #[token("AS")]
    AsLit,
    #[token("IN")]
    InLit,
    #[token("isIRI")]
    IsIriLit,
    #[token("OPTIONAL")]
    OptionalLit,
    #[token("OFFSET")]
    OffsetLit,
    #[token("CONSTRUCT")]
    ConstructLit,
    #[token(",")]
    Comma,
    #[token("LANG")]
    LangLit,
    #[regex("(?&NIL)")]
    Nil,
    #[token("DESCRIBE")]
    DescribeLit,
    #[regex("(?&INTEGER)")]
    Integer,
    #[regex("(?&LANGTAG)")]
    Langtag,
    #[token("MINUS")]
    MinusLit,
    #[regex("(?&VAR1)")]
    Var1,
    #[regex("(?&DECIMAL_NEGATIVE)")]
    DecimalNegative,
    #[token("ADD")]
    AddLit,
    #[token("GRAPH")]
    GraphLit,
    #[token("STRDT")]
    StrdtLit,
    #[regex("(?&DECIMAL_POSITIVE)")]
    DecimalPositive,
    #[token("true")]
    TrueLit,
    #[token("SILENT")]
    SilentLit,
    #[token("STRLANG")]
    StrlangLit,
    #[token("-")]
    Bar,
    #[token("NAMED")]
    NamedLit,
    #[regex("(?&DOUBLE)")]
    Double,
    #[token("HAVING")]
    HavingLit,
    #[token(")")]
    BrClose,
    #[token("SELECT")]
    SelectLit,
    #[token("WHERE")]
    WhereLit,
    #[token("YEAR")]
    YearLit,
    #[token("HOURS")]
    HoursLit,
    #[regex("(?&STRING_LITERAL1)")]
    StringLiteral1,
    #[token("DISTINCT")]
    DistinctLit,
    #[token("ENCODE_FOR_URI")]
    EncodeForUriLit,
    #[token("CEIL")]
    CeilLit,
    #[token("URI")]
    UriLit,
    #[token("BASE")]
    SparqlBaseToken,
    #[token("MOVE")]
    MoveLit,
    #[token("/")]
    Div,
    #[token("isNUMERIC")]
    IsNumericLit,
    #[regex("(?&DECIMAL)")]
    Decimal,
    #[token("ASC")]
    AscLit,
    #[token("COPY")]
    CopyLit,
    #[token("DATATYPE")]
    DatatypeLit,
    #[token("ABS")]
    AbsLit,
    #[token("INSERT DATA")]
    InsertDataLit,
    #[token("BNODE")]
    BnodeLit,
    #[token("SEPARATOR")]
    SeparatorLit,
    #[token("SECONDS")]
    SecondsLit,
    #[token("INTO")]
    IntoLit,
    #[token("WITH")]
    WithLit,
    #[token("SHA1")]
    Sha1Lit,
    #[token("VALUES")]
    ValuesLit,
    #[token("STRENDS")]
    StrendsLit,
    #[token("CONCAT")]
    ConcatLit,
    #[token("&&")]
    Amp2,
    #[token("IF")]
    IfLit,
    #[token("COUNT")]
    CountLit,
    #[token("GROUP_CONCAT")]
    GroupConcatLit,
    #[token("}")]
    ClClose,
    #[token("<")]
    Lt,
    #[token("+")]
    Plus,
    #[token(";")]
    Colon,
    #[token("MD5")]
    Md5Lit,
    #[token("a")]
    Alit,
    #[token("?")]
    Questionmark,
    #[token("REDUCED")]
    ReducedLit,
    #[token("DELETE DATA")]
    DeleteDataLit,
    #[token("^")]
    Hat,
    #[token("|")]
    Pipe,
    #[token("FLOOR")]
    FloorLit,
    #[token("UNION")]
    UnionLit,
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
                SyntaxKind::QueryUnit => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Query => Rule {
                    kind,
                    state: 7usize,
                },
                SyntaxKind::UpdateUnit => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Prologue => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::BaseDecl => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::PrefixDecl => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::SelectQuery => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::SubSelect => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::SelectClause => Rule {
                    kind,
                    state: 16usize,
                },
                SyntaxKind::ConstructQuery => Rule {
                    kind,
                    state: 14usize,
                },
                SyntaxKind::DescribeQuery => Rule {
                    kind,
                    state: 11usize,
                },
                SyntaxKind::AskQuery => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::DatasetClause => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::DefaultGraphClause => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::NamedGraphClause => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::SourceSelector => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::WhereClause => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::SolutionModifier => Rule {
                    kind,
                    state: 7usize,
                },
                SyntaxKind::GroupClause => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::GroupCondition => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::HavingClause => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::HavingCondition => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::OrderClause => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::OrderCondition => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::LimitOffsetClauses => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::LimitClause => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::OffsetClause => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::ValuesClause => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Update => Rule {
                    kind,
                    state: 6usize,
                },
                SyntaxKind::Update1 => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Load => Rule {
                    kind,
                    state: 7usize,
                },
                SyntaxKind::Clear => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::Drop => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::Create => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::Add => Rule {
                    kind,
                    state: 6usize,
                },
                SyntaxKind::Move => Rule {
                    kind,
                    state: 6usize,
                },
                SyntaxKind::Copy => Rule {
                    kind,
                    state: 6usize,
                },
                SyntaxKind::InsertData => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::DeleteData => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::DeleteWhere => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::Modify => Rule {
                    kind,
                    state: 10usize,
                },
                SyntaxKind::DeleteClause => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::InsertClause => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::UsingClause => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::GraphOrDefault => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::GraphRef => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::GraphRefAll => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::QuadPattern => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::QuadData => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::Quads => Rule {
                    kind,
                    state: 7usize,
                },
                SyntaxKind::QuadsNotTriples => Rule {
                    kind,
                    state: 6usize,
                },
                SyntaxKind::TriplesTemplate => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::GroupGraphPattern => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::GroupGraphPatternSub => Rule {
                    kind,
                    state: 7usize,
                },
                SyntaxKind::TriplesBlock => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::GraphPatternNotTriples => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::OptionalGraphPattern => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::GraphGraphPattern => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::ServiceGraphPattern => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::Bind => Rule {
                    kind,
                    state: 6usize,
                },
                SyntaxKind::InlineData => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::DataBlock => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::InlineDataOneVar => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::InlineDataFull => Rule {
                    kind,
                    state: 10usize,
                },
                SyntaxKind::DataBlockValue => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::MinusGraphPattern => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::GroupOrUnionGraphPattern => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::Filter => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::Constraint => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::FunctionCall => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::ArgList => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::ExpressionList => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::ConstructTemplate => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::ConstructTriples => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::TriplesSameSubject => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PropertyList => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PropertyListNotEmpty => Rule {
                    kind,
                    state: 7usize,
                },
                SyntaxKind::Verb => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::ObjectList => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::Object => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::TriplesSameSubjectPath => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PropertyListPath => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PropertyListPathNotEmpty => Rule {
                    kind,
                    state: 9usize,
                },
                SyntaxKind::VerbPath => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::VerbSimple => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::ObjectListPath => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::ObjectPath => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Path => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PathAlternative => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::PathSequence => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::PathElt => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::PathEltOrInverse => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PathMod => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PathPrimary => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PathNegatedPropertySet => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PathOneInPropertySet => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::TriplesNode => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::BlankNodePropertyList => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::TriplesNodePath => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::BlankNodePropertyListPath => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::Collection => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::CollectionPath => Rule {
                    kind,
                    state: 5usize,
                },
                SyntaxKind::GraphNode => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::GraphNodePath => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::VarOrTerm => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::VarOrIri => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Var => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::GraphTerm => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::Expression => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::ConditionalOrExpression => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::ConditionalAndExpression => Rule {
                    kind,
                    state: 4usize,
                },
                SyntaxKind::ValueLogical => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::RelationalExpression => Rule {
                    kind,
                    state: 16usize,
                },
                SyntaxKind::NumericExpression => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::AdditiveExpression => Rule {
                    kind,
                    state: 15usize,
                },
                SyntaxKind::MultiplicativeExpression => Rule {
                    kind,
                    state: 6usize,
                },
                SyntaxKind::UnaryExpression => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::PrimaryExpression => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::BrackettedExpression => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::BuiltInCall => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::RegexExpression => Rule {
                    kind,
                    state: 9usize,
                },
                SyntaxKind::SubstringExpression => Rule {
                    kind,
                    state: 9usize,
                },
                SyntaxKind::StrReplaceExpression => Rule {
                    kind,
                    state: 11usize,
                },
                SyntaxKind::ExistsFunc => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::NotExistsFunc => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::Aggregate => Rule {
                    kind,
                    state: 2usize,
                },
                SyntaxKind::IriOrFunction => Rule {
                    kind,
                    state: 3usize,
                },
                SyntaxKind::Rdfliteral => Rule {
                    kind,
                    state: 6usize,
                },
                SyntaxKind::NumericLiteral => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::NumericLiteralUnsigned => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::NumericLiteralPositive => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::NumericLiteralNegative => Rule {
                    kind,
                    state: 1usize,
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
                SyntaxKind::NowLit => Rule { kind, state: 0 },
                SyntaxKind::RegexLit => Rule { kind, state: 0 },
                SyntaxKind::Anon => Rule { kind, state: 0 },
                SyntaxKind::IsLiteralLit => Rule { kind, state: 0 },
                SyntaxKind::RandLit => Rule { kind, state: 0 },
                SyntaxKind::ExistsLit => Rule { kind, state: 0 },
                SyntaxKind::CreateLit => Rule { kind, state: 0 },
                SyntaxKind::StrlenLit => Rule { kind, state: 0 },
                SyntaxKind::Lte => Rule { kind, state: 0 },
                SyntaxKind::ServiceLit => Rule { kind, state: 0 },
                SyntaxKind::DescLit => Rule { kind, state: 0 },
                SyntaxKind::PnameLn => Rule { kind, state: 0 },
                SyntaxKind::LoadLit => Rule { kind, state: 0 },
                SyntaxKind::MaxLit => Rule { kind, state: 0 },
                SyntaxKind::IsUriLit => Rule { kind, state: 0 },
                SyntaxKind::InsertLit => Rule { kind, state: 0 },
                SyntaxKind::NotLit => Rule { kind, state: 0 },
                SyntaxKind::TimezoneLit => Rule { kind, state: 0 },
                SyntaxKind::SumLit => Rule { kind, state: 0 },
                SyntaxKind::AskLit => Rule { kind, state: 0 },
                SyntaxKind::FalseLit => Rule { kind, state: 0 },
                SyntaxKind::SqOpen => Rule { kind, state: 0 },
                SyntaxKind::UcaseLit => Rule { kind, state: 0 },
                SyntaxKind::MonthLit => Rule { kind, state: 0 },
                SyntaxKind::CoalesceLit => Rule { kind, state: 0 },
                SyntaxKind::SqClose => Rule { kind, state: 0 },
                SyntaxKind::BindLit => Rule { kind, state: 0 },
                SyntaxKind::StrbeforeLit => Rule { kind, state: 0 },
                SyntaxKind::Star => Rule { kind, state: 0 },
                SyntaxKind::ReplaceLit => Rule { kind, state: 0 },
                SyntaxKind::Datatype => Rule { kind, state: 0 },
                SyntaxKind::Sha256Lit => Rule { kind, state: 0 },
                SyntaxKind::BrOpen => Rule { kind, state: 0 },
                SyntaxKind::UuidLit => Rule { kind, state: 0 },
                SyntaxKind::LangmatchesLit => Rule { kind, state: 0 },
                SyntaxKind::MinLit => Rule { kind, state: 0 },
                SyntaxKind::Var2 => Rule { kind, state: 0 },
                SyntaxKind::FromLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLong2 => Rule { kind, state: 0 },
                SyntaxKind::Neq => Rule { kind, state: 0 },
                SyntaxKind::StringLiteral2 => Rule { kind, state: 0 },
                SyntaxKind::StrstartsLit => Rule { kind, state: 0 },
                SyntaxKind::StrLit => Rule { kind, state: 0 },
                SyntaxKind::Sha384Lit => Rule { kind, state: 0 },
                SyntaxKind::OrderLit => Rule { kind, state: 0 },
                SyntaxKind::ContainsLit => Rule { kind, state: 0 },
                SyntaxKind::DropLit => Rule { kind, state: 0 },
                SyntaxKind::IriLit => Rule { kind, state: 0 },
                SyntaxKind::GroupLit => Rule { kind, state: 0 },
                SyntaxKind::ClearLit => Rule { kind, state: 0 },
                SyntaxKind::Stop => Rule { kind, state: 0 },
                SyntaxKind::MinutesLit => Rule { kind, state: 0 },
                SyntaxKind::AvgLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLong1 => Rule { kind, state: 0 },
                SyntaxKind::LimitLit => Rule { kind, state: 0 },
                SyntaxKind::SubstrLit => Rule { kind, state: 0 },
                SyntaxKind::DeleteLit => Rule { kind, state: 0 },
                SyntaxKind::LcaseLit => Rule { kind, state: 0 },
                SyntaxKind::StrafterLit => Rule { kind, state: 0 },
                SyntaxKind::SampleLit => Rule { kind, state: 0 },
                SyntaxKind::PnameNs => Rule { kind, state: 0 },
                SyntaxKind::ToLit => Rule { kind, state: 0 },
                SyntaxKind::UndefLit => Rule { kind, state: 0 },
                SyntaxKind::FilterLit => Rule { kind, state: 0 },
                SyntaxKind::Pipe2 => Rule { kind, state: 0 },
                SyntaxKind::DoublePositive => Rule { kind, state: 0 },
                SyntaxKind::BoundLit => Rule { kind, state: 0 },
                SyntaxKind::Sha512Lit => Rule { kind, state: 0 },
                SyntaxKind::IntegerPositive => Rule { kind, state: 0 },
                SyntaxKind::SameTermLit => Rule { kind, state: 0 },
                SyntaxKind::RoundLit => Rule { kind, state: 0 },
                SyntaxKind::ClOpen => Rule { kind, state: 0 },
                SyntaxKind::Eq => Rule { kind, state: 0 },
                SyntaxKind::UsingLit => Rule { kind, state: 0 },
                SyntaxKind::DefaultLit => Rule { kind, state: 0 },
                SyntaxKind::Gt => Rule { kind, state: 0 },
                SyntaxKind::Gte => Rule { kind, state: 0 },
                SyntaxKind::AllLit => Rule { kind, state: 0 },
                SyntaxKind::Iriref => Rule { kind, state: 0 },
                SyntaxKind::StruuidLit => Rule { kind, state: 0 },
                SyntaxKind::SparqlPrefixToken => Rule { kind, state: 0 },
                SyntaxKind::Bang => Rule { kind, state: 0 },
                SyntaxKind::DayLit => Rule { kind, state: 0 },
                SyntaxKind::IsBlankLit => Rule { kind, state: 0 },
                SyntaxKind::IntegerNegative => Rule { kind, state: 0 },
                SyntaxKind::ByLit => Rule { kind, state: 0 },
                SyntaxKind::DoubleNegative => Rule { kind, state: 0 },
                SyntaxKind::BlankNodeLabel => Rule { kind, state: 0 },
                SyntaxKind::TzLit => Rule { kind, state: 0 },
                SyntaxKind::DeleteWhereLit => Rule { kind, state: 0 },
                SyntaxKind::AsLit => Rule { kind, state: 0 },
                SyntaxKind::InLit => Rule { kind, state: 0 },
                SyntaxKind::IsIriLit => Rule { kind, state: 0 },
                SyntaxKind::OptionalLit => Rule { kind, state: 0 },
                SyntaxKind::OffsetLit => Rule { kind, state: 0 },
                SyntaxKind::ConstructLit => Rule { kind, state: 0 },
                SyntaxKind::Comma => Rule { kind, state: 0 },
                SyntaxKind::LangLit => Rule { kind, state: 0 },
                SyntaxKind::Nil => Rule { kind, state: 0 },
                SyntaxKind::DescribeLit => Rule { kind, state: 0 },
                SyntaxKind::Integer => Rule { kind, state: 0 },
                SyntaxKind::Langtag => Rule { kind, state: 0 },
                SyntaxKind::MinusLit => Rule { kind, state: 0 },
                SyntaxKind::Var1 => Rule { kind, state: 0 },
                SyntaxKind::DecimalNegative => Rule { kind, state: 0 },
                SyntaxKind::AddLit => Rule { kind, state: 0 },
                SyntaxKind::GraphLit => Rule { kind, state: 0 },
                SyntaxKind::StrdtLit => Rule { kind, state: 0 },
                SyntaxKind::DecimalPositive => Rule { kind, state: 0 },
                SyntaxKind::TrueLit => Rule { kind, state: 0 },
                SyntaxKind::SilentLit => Rule { kind, state: 0 },
                SyntaxKind::StrlangLit => Rule { kind, state: 0 },
                SyntaxKind::Bar => Rule { kind, state: 0 },
                SyntaxKind::NamedLit => Rule { kind, state: 0 },
                SyntaxKind::Double => Rule { kind, state: 0 },
                SyntaxKind::HavingLit => Rule { kind, state: 0 },
                SyntaxKind::BrClose => Rule { kind, state: 0 },
                SyntaxKind::SelectLit => Rule { kind, state: 0 },
                SyntaxKind::WhereLit => Rule { kind, state: 0 },
                SyntaxKind::YearLit => Rule { kind, state: 0 },
                SyntaxKind::HoursLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteral1 => Rule { kind, state: 0 },
                SyntaxKind::DistinctLit => Rule { kind, state: 0 },
                SyntaxKind::EncodeForUriLit => Rule { kind, state: 0 },
                SyntaxKind::CeilLit => Rule { kind, state: 0 },
                SyntaxKind::UriLit => Rule { kind, state: 0 },
                SyntaxKind::SparqlBaseToken => Rule { kind, state: 0 },
                SyntaxKind::MoveLit => Rule { kind, state: 0 },
                SyntaxKind::Div => Rule { kind, state: 0 },
                SyntaxKind::IsNumericLit => Rule { kind, state: 0 },
                SyntaxKind::Decimal => Rule { kind, state: 0 },
                SyntaxKind::AscLit => Rule { kind, state: 0 },
                SyntaxKind::CopyLit => Rule { kind, state: 0 },
                SyntaxKind::DatatypeLit => Rule { kind, state: 0 },
                SyntaxKind::AbsLit => Rule { kind, state: 0 },
                SyntaxKind::InsertDataLit => Rule { kind, state: 0 },
                SyntaxKind::BnodeLit => Rule { kind, state: 0 },
                SyntaxKind::SeparatorLit => Rule { kind, state: 0 },
                SyntaxKind::SecondsLit => Rule { kind, state: 0 },
                SyntaxKind::IntoLit => Rule { kind, state: 0 },
                SyntaxKind::WithLit => Rule { kind, state: 0 },
                SyntaxKind::Sha1Lit => Rule { kind, state: 0 },
                SyntaxKind::ValuesLit => Rule { kind, state: 0 },
                SyntaxKind::StrendsLit => Rule { kind, state: 0 },
                SyntaxKind::ConcatLit => Rule { kind, state: 0 },
                SyntaxKind::Amp2 => Rule { kind, state: 0 },
                SyntaxKind::IfLit => Rule { kind, state: 0 },
                SyntaxKind::CountLit => Rule { kind, state: 0 },
                SyntaxKind::GroupConcatLit => Rule { kind, state: 0 },
                SyntaxKind::ClClose => Rule { kind, state: 0 },
                SyntaxKind::Lt => Rule { kind, state: 0 },
                SyntaxKind::Plus => Rule { kind, state: 0 },
                SyntaxKind::Colon => Rule { kind, state: 0 },
                SyntaxKind::Md5Lit => Rule { kind, state: 0 },
                SyntaxKind::Alit => Rule { kind, state: 0 },
                SyntaxKind::Questionmark => Rule { kind, state: 0 },
                SyntaxKind::ReducedLit => Rule { kind, state: 0 },
                SyntaxKind::DeleteDataLit => Rule { kind, state: 0 },
                SyntaxKind::Hat => Rule { kind, state: 0 },
                SyntaxKind::Pipe => Rule { kind, state: 0 },
                SyntaxKind::FloorLit => Rule { kind, state: 0 },
                SyntaxKind::UnionLit => Rule { kind, state: 0 },
                _ => panic!("Unknown rule kind {:?}", kind),
            }
        }
    }
    pub fn first_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::InsertClause => &[SyntaxKind::InsertLit],
            SyntaxKind::UpdateUnit => &[
                SyntaxKind::MoveLit,
                SyntaxKind::ClearLit,
                SyntaxKind::CreateLit,
                SyntaxKind::InsertLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::DeleteDataLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::DeleteLit,
                SyntaxKind::WithLit,
                SyntaxKind::LoadLit,
                SyntaxKind::CopyLit,
                SyntaxKind::AddLit,
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::DropLit,
                SyntaxKind::InsertDataLit,
            ],
            SyntaxKind::Prologue => &[SyntaxKind::SparqlBaseToken, SyntaxKind::SparqlPrefixToken],
            SyntaxKind::SubstringExpression => &[SyntaxKind::SubstrLit],
            SyntaxKind::BrackettedExpression => &[SyntaxKind::BrOpen],
            SyntaxKind::DeleteClause => &[SyntaxKind::DeleteLit],
            SyntaxKind::CollectionPath => &[SyntaxKind::BrOpen],
            SyntaxKind::PropertyList => &[SyntaxKind::Alit],
            SyntaxKind::HavingCondition => &[
                SyntaxKind::AvgLit,
                SyntaxKind::AbsLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::MinLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::RandLit,
                SyntaxKind::IfLit,
                SyntaxKind::RoundLit,
                SyntaxKind::HoursLit,
                SyntaxKind::RegexLit,
                SyntaxKind::DayLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::IriLit,
                SyntaxKind::MaxLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::CountLit,
                SyntaxKind::FloorLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::SumLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::LcaseLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::SampleLit,
                SyntaxKind::UuidLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::NowLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::BrOpen,
                SyntaxKind::UriLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::LangLit,
                SyntaxKind::NotLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::StrendsLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::StrdtLit,
                SyntaxKind::MonthLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::StrLit,
                SyntaxKind::CeilLit,
                SyntaxKind::TzLit,
                SyntaxKind::BoundLit,
                SyntaxKind::YearLit,
            ],
            SyntaxKind::UsingClause => &[SyntaxKind::UsingLit],
            SyntaxKind::NumericLiteral => &[],
            SyntaxKind::TriplesTemplate => &[
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
            ],
            SyntaxKind::AdditiveExpression => &[
                SyntaxKind::SecondsLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::RegexLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::SameTermLit,
                SyntaxKind::Plus,
                SyntaxKind::Bang,
                SyntaxKind::IriLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::CeilLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::YearLit,
                SyntaxKind::UriLit,
                SyntaxKind::NowLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::MinLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::BoundLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::LangLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::IsUriLit,
                SyntaxKind::AvgLit,
                SyntaxKind::SampleLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::TzLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::MaxLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::NotLit,
                SyntaxKind::RandLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::FalseLit,
                SyntaxKind::CountLit,
                SyntaxKind::SumLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::UuidLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::Bar,
                SyntaxKind::DayLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::HoursLit,
                SyntaxKind::FloorLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::AbsLit,
                SyntaxKind::RoundLit,
                SyntaxKind::BrOpen,
                SyntaxKind::IfLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::MinutesLit,
            ],
            SyntaxKind::Drop => &[SyntaxKind::DropLit],
            SyntaxKind::DataBlockValue => &[
                SyntaxKind::FalseLit,
                SyntaxKind::UndefLit,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::HavingClause => &[SyntaxKind::HavingLit],
            SyntaxKind::OrderCondition => &[
                SyntaxKind::StrLit,
                SyntaxKind::NowLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::IfLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::NotLit,
                SyntaxKind::MonthLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::BoundLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::CountLit,
                SyntaxKind::YearLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::AvgLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::MaxLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::RoundLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::SampleLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::CeilLit,
                SyntaxKind::RegexLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::StrlangLit,
                SyntaxKind::RandLit,
                SyntaxKind::IriLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::MinutesLit,
                SyntaxKind::DescLit,
                SyntaxKind::SumLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::AbsLit,
                SyntaxKind::LangLit,
                SyntaxKind::TzLit,
                SyntaxKind::UriLit,
                SyntaxKind::UuidLit,
                SyntaxKind::DayLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::UcaseLit,
                SyntaxKind::HoursLit,
                SyntaxKind::MinLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::AscLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::StrReplaceExpression => &[SyntaxKind::ReplaceLit],
            SyntaxKind::NumericExpression => &[
                SyntaxKind::YearLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::Bang,
                SyntaxKind::IfLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::TzLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::DayLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::CountLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::Bar,
                SyntaxKind::Plus,
                SyntaxKind::TrueLit,
                SyntaxKind::MinLit,
                SyntaxKind::AvgLit,
                SyntaxKind::UriLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::SumLit,
                SyntaxKind::RegexLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::SampleLit,
                SyntaxKind::UuidLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::IsIriLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::MaxLit,
                SyntaxKind::HoursLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::MonthLit,
                SyntaxKind::FalseLit,
                SyntaxKind::CeilLit,
                SyntaxKind::NowLit,
                SyntaxKind::NotLit,
                SyntaxKind::RoundLit,
                SyntaxKind::IriLit,
                SyntaxKind::LangLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::RandLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::AbsLit,
                SyntaxKind::StrbeforeLit,
            ],
            SyntaxKind::TriplesSameSubject => &[
                SyntaxKind::SqOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::VerbSimple => &[],
            SyntaxKind::GraphNode => &[
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::GroupGraphPatternSub => &[
                SyntaxKind::FalseLit,
                SyntaxKind::FilterLit,
                SyntaxKind::BindLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::MinusLit,
                SyntaxKind::GraphLit,
                SyntaxKind::ServiceLit,
                SyntaxKind::BrOpen,
                SyntaxKind::ClOpen,
                SyntaxKind::OptionalLit,
                SyntaxKind::ValuesLit,
            ],
            SyntaxKind::ObjectPath => &[
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::OrderClause => &[SyntaxKind::OrderLit],
            SyntaxKind::TriplesBlock => &[
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::Verb => &[SyntaxKind::Alit],
            SyntaxKind::BuiltInCall => &[
                SyntaxKind::ConcatLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::MaxLit,
                SyntaxKind::RegexLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::MinutesLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::CountLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::HoursLit,
                SyntaxKind::IfLit,
                SyntaxKind::MinLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::SumLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::TzLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::DayLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::AbsLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::StrLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::StrafterLit,
                SyntaxKind::CeilLit,
                SyntaxKind::NowLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::LangLit,
                SyntaxKind::AvgLit,
                SyntaxKind::SampleLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::UuidLit,
                SyntaxKind::YearLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::UriLit,
                SyntaxKind::IriLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::RandLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::NotLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::CoalesceLit,
            ],
            SyntaxKind::DefaultGraphClause => &[],
            SyntaxKind::ValuesClause => &[SyntaxKind::ValuesLit],
            SyntaxKind::TriplesSameSubjectPath => &[
                SyntaxKind::TrueLit,
                SyntaxKind::SqOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::ConditionalOrExpression => &[
                SyntaxKind::Sha512Lit,
                SyntaxKind::StrafterLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::CountLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::IfLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::MaxLit,
                SyntaxKind::FalseLit,
                SyntaxKind::TzLit,
                SyntaxKind::SampleLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::UuidLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::RegexLit,
                SyntaxKind::AvgLit,
                SyntaxKind::AbsLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::MinLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::CeilLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::NowLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::ContainsLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::LcaseLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::BrOpen,
                SyntaxKind::Plus,
                SyntaxKind::IsIriLit,
                SyntaxKind::SumLit,
                SyntaxKind::RandLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::StrLit,
                SyntaxKind::NotLit,
                SyntaxKind::MonthLit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::UriLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::LangLit,
                SyntaxKind::Bar,
                SyntaxKind::Md5Lit,
                SyntaxKind::HoursLit,
                SyntaxKind::FloorLit,
                SyntaxKind::IriLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::Bang,
                SyntaxKind::YearLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::DayLit,
                SyntaxKind::RoundLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::BnodeLit,
            ],
            SyntaxKind::GroupClause => &[SyntaxKind::GroupLit],
            SyntaxKind::QuadPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::QuadsNotTriples => &[SyntaxKind::GraphLit],
            SyntaxKind::InlineData => &[SyntaxKind::ValuesLit],
            SyntaxKind::GraphRefAll => &[
                SyntaxKind::GraphLit,
                SyntaxKind::DefaultLit,
                SyntaxKind::NamedLit,
                SyntaxKind::AllLit,
            ],
            SyntaxKind::LimitOffsetClauses => &[SyntaxKind::LimitLit, SyntaxKind::OffsetLit],
            SyntaxKind::DatasetClause => &[SyntaxKind::FromLit],
            SyntaxKind::SubSelect => &[SyntaxKind::SelectLit],
            SyntaxKind::GraphGraphPattern => &[SyntaxKind::GraphLit],
            SyntaxKind::NotExistsFunc => &[SyntaxKind::NotLit],
            SyntaxKind::Update1 => &[
                SyntaxKind::CreateLit,
                SyntaxKind::DeleteDataLit,
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::DeleteLit,
                SyntaxKind::DropLit,
                SyntaxKind::AddLit,
                SyntaxKind::CopyLit,
                SyntaxKind::WithLit,
                SyntaxKind::LoadLit,
                SyntaxKind::ClearLit,
                SyntaxKind::InsertLit,
                SyntaxKind::InsertDataLit,
                SyntaxKind::MoveLit,
            ],
            SyntaxKind::InsertData => &[SyntaxKind::InsertDataLit],
            SyntaxKind::VarOrIri => &[],
            SyntaxKind::PropertyListPath => &[
                SyntaxKind::Hat,
                SyntaxKind::BrOpen,
                SyntaxKind::Bang,
                SyntaxKind::Alit,
            ],
            SyntaxKind::PathMod => &[SyntaxKind::Star, SyntaxKind::Questionmark, SyntaxKind::Plus],
            SyntaxKind::Update => &[
                SyntaxKind::DeleteLit,
                SyntaxKind::CopyLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::DropLit,
                SyntaxKind::MoveLit,
                SyntaxKind::CreateLit,
                SyntaxKind::LoadLit,
                SyntaxKind::AddLit,
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::ClearLit,
                SyntaxKind::InsertLit,
                SyntaxKind::InsertDataLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::DeleteDataLit,
                SyntaxKind::WithLit,
            ],
            SyntaxKind::SelectQuery => &[SyntaxKind::SelectLit],
            SyntaxKind::OffsetClause => &[SyntaxKind::OffsetLit],
            SyntaxKind::BlankNode => &[],
            SyntaxKind::VerbPath => &[
                SyntaxKind::BrOpen,
                SyntaxKind::Bang,
                SyntaxKind::Alit,
                SyntaxKind::Hat,
            ],
            SyntaxKind::Var => &[],
            SyntaxKind::Aggregate => &[
                SyntaxKind::AvgLit,
                SyntaxKind::SampleLit,
                SyntaxKind::CountLit,
                SyntaxKind::MaxLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::MinLit,
                SyntaxKind::SumLit,
            ],
            SyntaxKind::Collection => &[SyntaxKind::BrOpen],
            SyntaxKind::PathAlternative => &[
                SyntaxKind::Bang,
                SyntaxKind::Hat,
                SyntaxKind::Alit,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::ServiceGraphPattern => &[SyntaxKind::ServiceLit],
            SyntaxKind::Path => &[
                SyntaxKind::BrOpen,
                SyntaxKind::Hat,
                SyntaxKind::Bang,
                SyntaxKind::Alit,
            ],
            SyntaxKind::WhereClause => &[SyntaxKind::ClOpen, SyntaxKind::WhereLit],
            SyntaxKind::InlineDataOneVar => &[],
            SyntaxKind::Bind => &[SyntaxKind::BindLit],
            SyntaxKind::PathPrimary => &[SyntaxKind::Alit, SyntaxKind::Bang, SyntaxKind::BrOpen],
            SyntaxKind::MyString => &[],
            SyntaxKind::ArgList => &[SyntaxKind::BrOpen],
            SyntaxKind::ValueLogical => &[
                SyntaxKind::IsIriLit,
                SyntaxKind::MonthLit,
                SyntaxKind::RoundLit,
                SyntaxKind::DayLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::Plus,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::SumLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::NowLit,
                SyntaxKind::RandLit,
                SyntaxKind::Bang,
                SyntaxKind::LangLit,
                SyntaxKind::CeilLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::MaxLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::AbsLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::BoundLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::IfLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::NotLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::ExistsLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::YearLit,
                SyntaxKind::RegexLit,
                SyntaxKind::CountLit,
                SyntaxKind::SampleLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::Bar,
                SyntaxKind::IsUriLit,
                SyntaxKind::StrLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::StruuidLit,
                SyntaxKind::AvgLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::HoursLit,
                SyntaxKind::UriLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::TzLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::TrueLit,
                SyntaxKind::UuidLit,
                SyntaxKind::BrOpen,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::IriLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::MinLit,
                SyntaxKind::ConcatLit,
            ],
            SyntaxKind::Add => &[SyntaxKind::AddLit],
            SyntaxKind::NumericLiteralNegative => &[],
            SyntaxKind::Filter => &[SyntaxKind::FilterLit],
            SyntaxKind::SolutionModifier => &[
                SyntaxKind::LimitLit,
                SyntaxKind::OrderLit,
                SyntaxKind::HavingLit,
                SyntaxKind::OffsetLit,
                SyntaxKind::GroupLit,
            ],
            SyntaxKind::PrimaryExpression => &[
                SyntaxKind::SampleLit,
                SyntaxKind::RegexLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::BrOpen,
                SyntaxKind::Sha1Lit,
                SyntaxKind::StrlangLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::AvgLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::DayLit,
                SyntaxKind::UuidLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::IriLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::SecondsLit,
                SyntaxKind::MaxLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::MinLit,
                SyntaxKind::RoundLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::TrueLit,
                SyntaxKind::SumLit,
                SyntaxKind::NowLit,
                SyntaxKind::UriLit,
                SyntaxKind::FalseLit,
                SyntaxKind::YearLit,
                SyntaxKind::AbsLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::FloorLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::HoursLit,
                SyntaxKind::IfLit,
                SyntaxKind::BoundLit,
                SyntaxKind::LangLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::MonthLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StrLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::StruuidLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::RandLit,
                SyntaxKind::NotLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::CountLit,
                SyntaxKind::TzLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::CeilLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::ContainsLit,
            ],
            SyntaxKind::PathNegatedPropertySet => {
                &[SyntaxKind::BrOpen, SyntaxKind::Alit, SyntaxKind::Hat]
            }
            SyntaxKind::ConstructTriples => &[
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::MultiplicativeExpression => &[
                SyntaxKind::IriLit,
                SyntaxKind::UriLit,
                SyntaxKind::YearLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::LangLit,
                SyntaxKind::MonthLit,
                SyntaxKind::AvgLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::NotLit,
                SyntaxKind::TrueLit,
                SyntaxKind::Bar,
                SyntaxKind::RegexLit,
                SyntaxKind::RoundLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::CeilLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::Plus,
                SyntaxKind::StrafterLit,
                SyntaxKind::FalseLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::BrOpen,
                SyntaxKind::TimezoneLit,
                SyntaxKind::RandLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::Bang,
                SyntaxKind::StrlangLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::ContainsLit,
                SyntaxKind::StrLit,
                SyntaxKind::SampleLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::CountLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::TzLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::SumLit,
                SyntaxKind::DayLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::HoursLit,
                SyntaxKind::MaxLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::UuidLit,
                SyntaxKind::MinLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::IfLit,
                SyntaxKind::FloorLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::BoundLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::AbsLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::NowLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::GroupConcatLit,
            ],
            SyntaxKind::PathSequence => &[
                SyntaxKind::BrOpen,
                SyntaxKind::Bang,
                SyntaxKind::Hat,
                SyntaxKind::Alit,
            ],
            SyntaxKind::GroupOrUnionGraphPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::GraphNodePath => &[
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::ConstructTemplate => &[SyntaxKind::ClOpen],
            SyntaxKind::RegexExpression => &[SyntaxKind::RegexLit],
            SyntaxKind::PrefixedName => &[],
            SyntaxKind::GroupGraphPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::LimitClause => &[SyntaxKind::LimitLit],
            SyntaxKind::ExistsFunc => &[SyntaxKind::ExistsLit],
            SyntaxKind::ObjectListPath => &[
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::BaseDecl => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::RelationalExpression => &[
                SyntaxKind::Plus,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::YearLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::UuidLit,
                SyntaxKind::StrLit,
                SyntaxKind::BoundLit,
                SyntaxKind::RegexLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::AbsLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::AvgLit,
                SyntaxKind::IriLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::FloorLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::DayLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::BrOpen,
                SyntaxKind::LcaseLit,
                SyntaxKind::SumLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::UriLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::MinLit,
                SyntaxKind::NotLit,
                SyntaxKind::SampleLit,
                SyntaxKind::CountLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::MinutesLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::RandLit,
                SyntaxKind::RoundLit,
                SyntaxKind::LangLit,
                SyntaxKind::FalseLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::Bar,
                SyntaxKind::CeilLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::SameTermLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::MaxLit,
                SyntaxKind::NowLit,
                SyntaxKind::Bang,
                SyntaxKind::TzLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::IfLit,
                SyntaxKind::HoursLit,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Modify => &[
                SyntaxKind::InsertLit,
                SyntaxKind::WithLit,
                SyntaxKind::DeleteLit,
            ],
            SyntaxKind::Expression => &[
                SyntaxKind::NotLit,
                SyntaxKind::MonthLit,
                SyntaxKind::NowLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::SumLit,
                SyntaxKind::IfLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::LcaseLit,
                SyntaxKind::DayLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::Bar,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::UriLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::StrLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::HoursLit,
                SyntaxKind::CountLit,
                SyntaxKind::Plus,
                SyntaxKind::RandLit,
                SyntaxKind::FloorLit,
                SyntaxKind::IriLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::LangLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::TrueLit,
                SyntaxKind::SampleLit,
                SyntaxKind::UuidLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::AvgLit,
                SyntaxKind::RoundLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::MaxLit,
                SyntaxKind::MinLit,
                SyntaxKind::AbsLit,
                SyntaxKind::BrOpen,
                SyntaxKind::Md5Lit,
                SyntaxKind::ConcatLit,
                SyntaxKind::RegexLit,
                SyntaxKind::Bang,
                SyntaxKind::CeilLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::StrendsLit,
                SyntaxKind::TzLit,
                SyntaxKind::FalseLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::BoundLit,
                SyntaxKind::YearLit,
            ],
            SyntaxKind::DeleteWhere => &[SyntaxKind::DeleteWhereLit],
            SyntaxKind::TriplesNodePath => &[SyntaxKind::SqOpen, SyntaxKind::BrOpen],
            SyntaxKind::PropertyListPathNotEmpty => &[
                SyntaxKind::Alit,
                SyntaxKind::Bang,
                SyntaxKind::Hat,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::SourceSelector => &[],
            SyntaxKind::PathElt => &[SyntaxKind::Alit, SyntaxKind::BrOpen, SyntaxKind::Bang],
            SyntaxKind::PrefixDecl => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::FunctionCall => &[],
            SyntaxKind::GroupCondition => &[
                SyntaxKind::Sha1Lit,
                SyntaxKind::ExistsLit,
                SyntaxKind::IriLit,
                SyntaxKind::LangLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::BrOpen,
                SyntaxKind::MaxLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::SumLit,
                SyntaxKind::DayLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::MinLit,
                SyntaxKind::NotLit,
                SyntaxKind::CeilLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::UuidLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::RegexLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::IsUriLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::IsIriLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::UriLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::HoursLit,
                SyntaxKind::SampleLit,
                SyntaxKind::RandLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::NowLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::YearLit,
                SyntaxKind::CountLit,
                SyntaxKind::BoundLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::AvgLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::AbsLit,
                SyntaxKind::IfLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::FloorLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::TzLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::MonthLit,
            ],
            SyntaxKind::GraphRef => &[SyntaxKind::GraphLit],
            SyntaxKind::NumericLiteralPositive => &[],
            SyntaxKind::PathOneInPropertySet => &[SyntaxKind::Alit, SyntaxKind::Hat],
            SyntaxKind::Iri => &[],
            SyntaxKind::ConstructQuery => &[SyntaxKind::ConstructLit],
            SyntaxKind::Load => &[SyntaxKind::LoadLit],
            SyntaxKind::OptionalGraphPattern => &[SyntaxKind::OptionalLit],
            SyntaxKind::DescribeQuery => &[SyntaxKind::DescribeLit],
            SyntaxKind::Rdfliteral => &[],
            SyntaxKind::NumericLiteralUnsigned => &[],
            SyntaxKind::InlineDataFull => &[SyntaxKind::BrOpen],
            SyntaxKind::GraphTerm => &[SyntaxKind::FalseLit, SyntaxKind::TrueLit],
            SyntaxKind::GraphPatternNotTriples => &[
                SyntaxKind::GraphLit,
                SyntaxKind::ValuesLit,
                SyntaxKind::OptionalLit,
                SyntaxKind::ServiceLit,
                SyntaxKind::FilterLit,
                SyntaxKind::ClOpen,
                SyntaxKind::BindLit,
                SyntaxKind::MinusLit,
            ],
            SyntaxKind::UnaryExpression => &[
                SyntaxKind::Bar,
                SyntaxKind::ReplaceLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::FalseLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::SumLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::Bang,
                SyntaxKind::UuidLit,
                SyntaxKind::AvgLit,
                SyntaxKind::TrueLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::HoursLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::IriLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::NowLit,
                SyntaxKind::IfLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::CountLit,
                SyntaxKind::RegexLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::MaxLit,
                SyntaxKind::NotLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::StrendsLit,
                SyntaxKind::DayLit,
                SyntaxKind::LangLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::UriLit,
                SyntaxKind::FloorLit,
                SyntaxKind::YearLit,
                SyntaxKind::TzLit,
                SyntaxKind::CeilLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::SampleLit,
                SyntaxKind::MinLit,
                SyntaxKind::AbsLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::BrOpen,
                SyntaxKind::MonthLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::Plus,
                SyntaxKind::RandLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::StrLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::IsUriLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::RoundLit,
            ],
            SyntaxKind::QueryUnit => &[
                SyntaxKind::DescribeLit,
                SyntaxKind::AskLit,
                SyntaxKind::SelectLit,
                SyntaxKind::ConstructLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SparqlPrefixToken,
            ],
            SyntaxKind::VarOrTerm => &[SyntaxKind::TrueLit, SyntaxKind::FalseLit],
            SyntaxKind::MinusGraphPattern => &[SyntaxKind::MinusLit],
            SyntaxKind::AskQuery => &[SyntaxKind::AskLit],
            SyntaxKind::NamedGraphClause => &[SyntaxKind::NamedLit],
            SyntaxKind::PropertyListNotEmpty => &[SyntaxKind::Alit],
            SyntaxKind::Move => &[SyntaxKind::MoveLit],
            SyntaxKind::Create => &[SyntaxKind::CreateLit],
            SyntaxKind::GraphOrDefault => &[SyntaxKind::DefaultLit, SyntaxKind::GraphLit],
            SyntaxKind::ExpressionList => &[SyntaxKind::BrOpen],
            SyntaxKind::SelectClause => &[SyntaxKind::SelectLit],
            SyntaxKind::Object => &[
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::Query => &[
                SyntaxKind::ConstructLit,
                SyntaxKind::DescribeLit,
                SyntaxKind::AskLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SelectLit,
                SyntaxKind::SparqlPrefixToken,
            ],
            SyntaxKind::Clear => &[SyntaxKind::ClearLit],
            SyntaxKind::BlankNodePropertyList => &[SyntaxKind::SqOpen],
            SyntaxKind::Copy => &[SyntaxKind::CopyLit],
            SyntaxKind::IriOrFunction => &[],
            SyntaxKind::QuadData => &[SyntaxKind::ClOpen],
            SyntaxKind::DataBlock => &[SyntaxKind::BrOpen],
            SyntaxKind::ObjectList => &[
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::BlankNodePropertyListPath => &[SyntaxKind::SqOpen],
            SyntaxKind::Quads => &[
                SyntaxKind::GraphLit,
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::DeleteData => &[SyntaxKind::DeleteDataLit],
            SyntaxKind::BooleanLiteral => &[SyntaxKind::TrueLit, SyntaxKind::FalseLit],
            SyntaxKind::ConditionalAndExpression => &[
                SyntaxKind::IsIriLit,
                SyntaxKind::Plus,
                SyntaxKind::Sha512Lit,
                SyntaxKind::Bang,
                SyntaxKind::TrueLit,
                SyntaxKind::MinLit,
                SyntaxKind::NowLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::AbsLit,
                SyntaxKind::YearLit,
                SyntaxKind::UriLit,
                SyntaxKind::SumLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::CountLit,
                SyntaxKind::Bar,
                SyntaxKind::SecondsLit,
                SyntaxKind::UuidLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::MaxLit,
                SyntaxKind::RandLit,
                SyntaxKind::IriLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::CeilLit,
                SyntaxKind::LangLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::StrlenLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::StrLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::RegexLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::NotLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::HoursLit,
                SyntaxKind::FloorLit,
                SyntaxKind::AvgLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SampleLit,
                SyntaxKind::MonthLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::RoundLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::UcaseLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::IfLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::BrOpen,
                SyntaxKind::ContainsLit,
                SyntaxKind::TzLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::DayLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::BnodeLit,
            ],
            SyntaxKind::PathEltOrInverse => &[
                SyntaxKind::BrOpen,
                SyntaxKind::Hat,
                SyntaxKind::Bang,
                SyntaxKind::Alit,
            ],
            SyntaxKind::Constraint => &[
                SyntaxKind::TzLit,
                SyntaxKind::RegexLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::YearLit,
                SyntaxKind::BrOpen,
                SyntaxKind::BoundLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::LangLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::UriLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::NotLit,
                SyntaxKind::MinLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::CeilLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::SampleLit,
                SyntaxKind::FloorLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::RoundLit,
                SyntaxKind::CountLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::AvgLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::UuidLit,
                SyntaxKind::SumLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::NowLit,
                SyntaxKind::AbsLit,
                SyntaxKind::StrLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::HoursLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::IsIriLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::MaxLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::DayLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::RandLit,
                SyntaxKind::IfLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::IriLit,
            ],
            SyntaxKind::TriplesNode => &[SyntaxKind::SqOpen, SyntaxKind::BrOpen],
            SyntaxKind::NowLit => &[SyntaxKind::NowLit],
            SyntaxKind::RegexLit => &[SyntaxKind::RegexLit],
            SyntaxKind::Anon => &[SyntaxKind::Anon],
            SyntaxKind::IsLiteralLit => &[SyntaxKind::IsLiteralLit],
            SyntaxKind::RandLit => &[SyntaxKind::RandLit],
            SyntaxKind::ExistsLit => &[SyntaxKind::ExistsLit],
            SyntaxKind::CreateLit => &[SyntaxKind::CreateLit],
            SyntaxKind::StrlenLit => &[SyntaxKind::StrlenLit],
            SyntaxKind::Lte => &[SyntaxKind::Lte],
            SyntaxKind::ServiceLit => &[SyntaxKind::ServiceLit],
            SyntaxKind::DescLit => &[SyntaxKind::DescLit],
            SyntaxKind::PnameLn => &[SyntaxKind::PnameLn],
            SyntaxKind::LoadLit => &[SyntaxKind::LoadLit],
            SyntaxKind::MaxLit => &[SyntaxKind::MaxLit],
            SyntaxKind::IsUriLit => &[SyntaxKind::IsUriLit],
            SyntaxKind::InsertLit => &[SyntaxKind::InsertLit],
            SyntaxKind::NotLit => &[SyntaxKind::NotLit],
            SyntaxKind::TimezoneLit => &[SyntaxKind::TimezoneLit],
            SyntaxKind::SumLit => &[SyntaxKind::SumLit],
            SyntaxKind::AskLit => &[SyntaxKind::AskLit],
            SyntaxKind::FalseLit => &[SyntaxKind::FalseLit],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::UcaseLit => &[SyntaxKind::UcaseLit],
            SyntaxKind::MonthLit => &[SyntaxKind::MonthLit],
            SyntaxKind::CoalesceLit => &[SyntaxKind::CoalesceLit],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::BindLit => &[SyntaxKind::BindLit],
            SyntaxKind::StrbeforeLit => &[SyntaxKind::StrbeforeLit],
            SyntaxKind::Star => &[SyntaxKind::Star],
            SyntaxKind::ReplaceLit => &[SyntaxKind::ReplaceLit],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::Sha256Lit => &[SyntaxKind::Sha256Lit],
            SyntaxKind::BrOpen => &[SyntaxKind::BrOpen],
            SyntaxKind::UuidLit => &[SyntaxKind::UuidLit],
            SyntaxKind::LangmatchesLit => &[SyntaxKind::LangmatchesLit],
            SyntaxKind::MinLit => &[SyntaxKind::MinLit],
            SyntaxKind::Var2 => &[SyntaxKind::Var2],
            SyntaxKind::FromLit => &[SyntaxKind::FromLit],
            SyntaxKind::StringLiteralLong2 => &[SyntaxKind::StringLiteralLong2],
            SyntaxKind::Neq => &[SyntaxKind::Neq],
            SyntaxKind::StringLiteral2 => &[SyntaxKind::StringLiteral2],
            SyntaxKind::StrstartsLit => &[SyntaxKind::StrstartsLit],
            SyntaxKind::StrLit => &[SyntaxKind::StrLit],
            SyntaxKind::Sha384Lit => &[SyntaxKind::Sha384Lit],
            SyntaxKind::OrderLit => &[SyntaxKind::OrderLit],
            SyntaxKind::ContainsLit => &[SyntaxKind::ContainsLit],
            SyntaxKind::DropLit => &[SyntaxKind::DropLit],
            SyntaxKind::IriLit => &[SyntaxKind::IriLit],
            SyntaxKind::GroupLit => &[SyntaxKind::GroupLit],
            SyntaxKind::ClearLit => &[SyntaxKind::ClearLit],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::MinutesLit => &[SyntaxKind::MinutesLit],
            SyntaxKind::AvgLit => &[SyntaxKind::AvgLit],
            SyntaxKind::StringLiteralLong1 => &[SyntaxKind::StringLiteralLong1],
            SyntaxKind::LimitLit => &[SyntaxKind::LimitLit],
            SyntaxKind::SubstrLit => &[SyntaxKind::SubstrLit],
            SyntaxKind::DeleteLit => &[SyntaxKind::DeleteLit],
            SyntaxKind::LcaseLit => &[SyntaxKind::LcaseLit],
            SyntaxKind::StrafterLit => &[SyntaxKind::StrafterLit],
            SyntaxKind::SampleLit => &[SyntaxKind::SampleLit],
            SyntaxKind::PnameNs => &[SyntaxKind::PnameNs],
            SyntaxKind::ToLit => &[SyntaxKind::ToLit],
            SyntaxKind::UndefLit => &[SyntaxKind::UndefLit],
            SyntaxKind::FilterLit => &[SyntaxKind::FilterLit],
            SyntaxKind::Pipe2 => &[SyntaxKind::Pipe2],
            SyntaxKind::DoublePositive => &[SyntaxKind::DoublePositive],
            SyntaxKind::BoundLit => &[SyntaxKind::BoundLit],
            SyntaxKind::Sha512Lit => &[SyntaxKind::Sha512Lit],
            SyntaxKind::IntegerPositive => &[SyntaxKind::IntegerPositive],
            SyntaxKind::SameTermLit => &[SyntaxKind::SameTermLit],
            SyntaxKind::RoundLit => &[SyntaxKind::RoundLit],
            SyntaxKind::ClOpen => &[SyntaxKind::ClOpen],
            SyntaxKind::Eq => &[SyntaxKind::Eq],
            SyntaxKind::UsingLit => &[SyntaxKind::UsingLit],
            SyntaxKind::DefaultLit => &[SyntaxKind::DefaultLit],
            SyntaxKind::Gt => &[SyntaxKind::Gt],
            SyntaxKind::Gte => &[SyntaxKind::Gte],
            SyntaxKind::AllLit => &[SyntaxKind::AllLit],
            SyntaxKind::Iriref => &[SyntaxKind::Iriref],
            SyntaxKind::StruuidLit => &[SyntaxKind::StruuidLit],
            SyntaxKind::SparqlPrefixToken => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::Bang => &[SyntaxKind::Bang],
            SyntaxKind::DayLit => &[SyntaxKind::DayLit],
            SyntaxKind::IsBlankLit => &[SyntaxKind::IsBlankLit],
            SyntaxKind::IntegerNegative => &[SyntaxKind::IntegerNegative],
            SyntaxKind::ByLit => &[SyntaxKind::ByLit],
            SyntaxKind::DoubleNegative => &[SyntaxKind::DoubleNegative],
            SyntaxKind::BlankNodeLabel => &[SyntaxKind::BlankNodeLabel],
            SyntaxKind::TzLit => &[SyntaxKind::TzLit],
            SyntaxKind::DeleteWhereLit => &[SyntaxKind::DeleteWhereLit],
            SyntaxKind::AsLit => &[SyntaxKind::AsLit],
            SyntaxKind::InLit => &[SyntaxKind::InLit],
            SyntaxKind::IsIriLit => &[SyntaxKind::IsIriLit],
            SyntaxKind::OptionalLit => &[SyntaxKind::OptionalLit],
            SyntaxKind::OffsetLit => &[SyntaxKind::OffsetLit],
            SyntaxKind::ConstructLit => &[SyntaxKind::ConstructLit],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::LangLit => &[SyntaxKind::LangLit],
            SyntaxKind::Nil => &[SyntaxKind::Nil],
            SyntaxKind::DescribeLit => &[SyntaxKind::DescribeLit],
            SyntaxKind::Integer => &[SyntaxKind::Integer],
            SyntaxKind::Langtag => &[SyntaxKind::Langtag],
            SyntaxKind::MinusLit => &[SyntaxKind::MinusLit],
            SyntaxKind::Var1 => &[SyntaxKind::Var1],
            SyntaxKind::DecimalNegative => &[SyntaxKind::DecimalNegative],
            SyntaxKind::AddLit => &[SyntaxKind::AddLit],
            SyntaxKind::GraphLit => &[SyntaxKind::GraphLit],
            SyntaxKind::StrdtLit => &[SyntaxKind::StrdtLit],
            SyntaxKind::DecimalPositive => &[SyntaxKind::DecimalPositive],
            SyntaxKind::TrueLit => &[SyntaxKind::TrueLit],
            SyntaxKind::SilentLit => &[SyntaxKind::SilentLit],
            SyntaxKind::StrlangLit => &[SyntaxKind::StrlangLit],
            SyntaxKind::Bar => &[SyntaxKind::Bar],
            SyntaxKind::NamedLit => &[SyntaxKind::NamedLit],
            SyntaxKind::Double => &[SyntaxKind::Double],
            SyntaxKind::HavingLit => &[SyntaxKind::HavingLit],
            SyntaxKind::BrClose => &[SyntaxKind::BrClose],
            SyntaxKind::SelectLit => &[SyntaxKind::SelectLit],
            SyntaxKind::WhereLit => &[SyntaxKind::WhereLit],
            SyntaxKind::YearLit => &[SyntaxKind::YearLit],
            SyntaxKind::HoursLit => &[SyntaxKind::HoursLit],
            SyntaxKind::StringLiteral1 => &[SyntaxKind::StringLiteral1],
            SyntaxKind::DistinctLit => &[SyntaxKind::DistinctLit],
            SyntaxKind::EncodeForUriLit => &[SyntaxKind::EncodeForUriLit],
            SyntaxKind::CeilLit => &[SyntaxKind::CeilLit],
            SyntaxKind::UriLit => &[SyntaxKind::UriLit],
            SyntaxKind::SparqlBaseToken => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::MoveLit => &[SyntaxKind::MoveLit],
            SyntaxKind::Div => &[SyntaxKind::Div],
            SyntaxKind::IsNumericLit => &[SyntaxKind::IsNumericLit],
            SyntaxKind::Decimal => &[SyntaxKind::Decimal],
            SyntaxKind::AscLit => &[SyntaxKind::AscLit],
            SyntaxKind::CopyLit => &[SyntaxKind::CopyLit],
            SyntaxKind::DatatypeLit => &[SyntaxKind::DatatypeLit],
            SyntaxKind::AbsLit => &[SyntaxKind::AbsLit],
            SyntaxKind::InsertDataLit => &[SyntaxKind::InsertDataLit],
            SyntaxKind::BnodeLit => &[SyntaxKind::BnodeLit],
            SyntaxKind::SeparatorLit => &[SyntaxKind::SeparatorLit],
            SyntaxKind::SecondsLit => &[SyntaxKind::SecondsLit],
            SyntaxKind::IntoLit => &[SyntaxKind::IntoLit],
            SyntaxKind::WithLit => &[SyntaxKind::WithLit],
            SyntaxKind::Sha1Lit => &[SyntaxKind::Sha1Lit],
            SyntaxKind::ValuesLit => &[SyntaxKind::ValuesLit],
            SyntaxKind::StrendsLit => &[SyntaxKind::StrendsLit],
            SyntaxKind::ConcatLit => &[SyntaxKind::ConcatLit],
            SyntaxKind::Amp2 => &[SyntaxKind::Amp2],
            SyntaxKind::IfLit => &[SyntaxKind::IfLit],
            SyntaxKind::CountLit => &[SyntaxKind::CountLit],
            SyntaxKind::GroupConcatLit => &[SyntaxKind::GroupConcatLit],
            SyntaxKind::ClClose => &[SyntaxKind::ClClose],
            SyntaxKind::Lt => &[SyntaxKind::Lt],
            SyntaxKind::Plus => &[SyntaxKind::Plus],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::Md5Lit => &[SyntaxKind::Md5Lit],
            SyntaxKind::Alit => &[SyntaxKind::Alit],
            SyntaxKind::Questionmark => &[SyntaxKind::Questionmark],
            SyntaxKind::ReducedLit => &[SyntaxKind::ReducedLit],
            SyntaxKind::DeleteDataLit => &[SyntaxKind::DeleteDataLit],
            SyntaxKind::Hat => &[SyntaxKind::Hat],
            SyntaxKind::Pipe => &[SyntaxKind::Pipe],
            SyntaxKind::FloorLit => &[SyntaxKind::FloorLit],
            SyntaxKind::UnionLit => &[SyntaxKind::UnionLit],
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
                (SyntaxKind::QueryUnit, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::QueryUnit, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Query,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::Query, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SelectQuery,
                                state: 5usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ConstructQuery,
                                state: 14usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DescribeQuery,
                                state: 11usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AskQuery,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::Query, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ValuesClause,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Query, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Prologue,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Query, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::UpdateUnit, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::UpdateUnit, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Update,
                                state: 6usize,
                            }),
                    );
                }
                (SyntaxKind::Prologue, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BaseDecl,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PrefixDecl,
                                state: 3usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BaseDecl, 1usize) => {
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
                (SyntaxKind::BaseDecl, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SparqlBaseToken, 100isize);
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
                (SyntaxKind::BaseDecl, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixDecl, 2usize) => {
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
                (SyntaxKind::PrefixDecl, 1usize) => {
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
                (SyntaxKind::PrefixDecl, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixDecl, 3usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SparqlPrefixToken, 100isize);
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
                (SyntaxKind::SelectQuery, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SelectClause,
                                state: 16usize,
                            }),
                    );
                }
                (SyntaxKind::SelectQuery, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SelectQuery, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SolutionModifier,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::SelectQuery, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DatasetClause,
                                state: 4usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::SubSelect, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SubSelect, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::SubSelect, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SolutionModifier,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::SubSelect, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SelectClause,
                                state: 16usize,
                            }),
                    );
                }
                (SyntaxKind::SubSelect, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ValuesClause,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::SelectClause, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::SelectClause, 6usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::SelectClause, 12usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 9usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 9usize,
                        }));
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Star, 10isize);
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
                        state.expect_as_inline(element, SyntaxKind::DistinctLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ReducedLit, 10isize);
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
                (SyntaxKind::SelectClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SelectClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 9usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 9usize,
                        }));
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Star, 10isize);
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
                (SyntaxKind::SelectClause, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 9usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 9usize,
                        }));
                    }
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SelectClause, 9usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::SelectClause, 16usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SelectLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 12usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 12usize,
                        }));
                    }
                }
                (SyntaxKind::SelectClause, 8usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AsLit, 10isize);
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
                (SyntaxKind::ConstructQuery, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ConstructQuery, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DatasetClause,
                                state: 4usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 14usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ConstructLit, 10isize);
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
                (SyntaxKind::ConstructQuery, 10usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ClOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 8usize,
                        }));
                    }
                }
                (SyntaxKind::ConstructQuery, 7usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ClClose, 8isize);
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
                (SyntaxKind::ConstructQuery, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SolutionModifier,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 8usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ClClose, 8isize);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesTemplate,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 12usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 12usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DatasetClause,
                                state: 4usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::WhereLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 10usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 10usize,
                        }));
                    }
                }
                (SyntaxKind::ConstructQuery, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ConstructTemplate,
                                state: 4usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 12usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DatasetClause,
                                state: 4usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::WhereLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 10usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 10usize,
                        }));
                    }
                }
                (SyntaxKind::DescribeQuery, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SolutionModifier,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::DescribeQuery, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DescribeQuery, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VarOrIri,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Star, 10isize);
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
                (SyntaxKind::DescribeQuery, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DatasetClause,
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
                                kind: SyntaxKind::SolutionModifier,
                                state: 7usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::DescribeQuery, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VarOrIri,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DatasetClause,
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
                                kind: SyntaxKind::SolutionModifier,
                                state: 7usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::DescribeQuery, 11usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DescribeLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 6usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 6usize,
                        }));
                    }
                }
                (SyntaxKind::AskQuery, 5usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::AskLit, 10isize);
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
                (SyntaxKind::AskQuery, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SolutionModifier,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::AskQuery, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AskQuery, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DatasetClause,
                                state: 4usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::DatasetClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DatasetClause, 4usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::FromLit, 10isize);
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
                (SyntaxKind::DatasetClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DefaultGraphClause,
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
                                kind: SyntaxKind::NamedGraphClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::DefaultGraphClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DefaultGraphClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SourceSelector,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::NamedGraphClause, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::NamedLit, 10isize);
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
                (SyntaxKind::NamedGraphClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SourceSelector,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::NamedGraphClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SourceSelector, 1usize) => {
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
                (SyntaxKind::SourceSelector, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::WhereClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::WhereClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::WhereClause, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::WhereLit, 10isize);
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
                (SyntaxKind::SolutionModifier, 5usize) => {
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
                                kind: SyntaxKind::LimitOffsetClauses,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::OrderClause,
                                state: 5usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::HavingClause,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::SolutionModifier, 7usize) => {
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
                                kind: SyntaxKind::LimitOffsetClauses,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::OrderClause,
                                state: 5usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::HavingClause,
                                state: 4usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupClause,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::SolutionModifier, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SolutionModifier, 3usize) => {
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
                                kind: SyntaxKind::LimitOffsetClauses,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::OrderClause,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::SolutionModifier, 1usize) => {
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
                                kind: SyntaxKind::LimitOffsetClauses,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::GroupClause, 5usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::GroupLit, 10isize);
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
                (SyntaxKind::GroupClause, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupCondition,
                                state: 1usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupCondition,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::GroupClause, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ByLit, 10isize);
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
                (SyntaxKind::GroupCondition, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupCondition, 5usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AsLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 6usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 6usize,
                        }));
                    }
                }
                (SyntaxKind::GroupCondition, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::GroupCondition, 4usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::GroupCondition, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BuiltInCall,
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
                                kind: SyntaxKind::FunctionCall,
                                state: 2usize,
                            }),
                    );
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 8usize,
                        }));
                    }
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::GroupCondition, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::HavingClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::HavingCondition,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::HavingClause, 4usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::HavingLit, 10isize);
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
                (SyntaxKind::HavingClause, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::HavingCondition,
                                state: 1usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::HavingCondition, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Constraint,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::HavingCondition, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::OrderClause, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::OrderCondition,
                                state: 1usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::OrderClause, 5usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::OrderLit, 10isize);
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
                (SyntaxKind::OrderClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::OrderCondition,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::OrderClause, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ByLit, 10isize);
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
                (SyntaxKind::OrderCondition, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrackettedExpression,
                                state: 3usize,
                            }),
                    );
                }
                (SyntaxKind::OrderCondition, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::AscLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DescLit, 10isize);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Constraint,
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
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::OrderCondition, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::LimitOffsetClauses, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::LimitOffsetClauses, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::LimitClause,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::OffsetClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::LimitOffsetClauses, 5usize) => {
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
                                kind: SyntaxKind::LimitClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::LimitOffsetClauses, 2usize) => {
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
                                kind: SyntaxKind::OffsetClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::LimitClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::LimitClause, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::LimitLit, 10isize);
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
                (SyntaxKind::LimitClause, 1usize) => {
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
                }
                (SyntaxKind::OffsetClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::OffsetClause, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::OffsetLit, 10isize);
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
                (SyntaxKind::OffsetClause, 1usize) => {
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
                }
                (SyntaxKind::ValuesClause, 1usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ValuesLit, 10isize);
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
                (SyntaxKind::ValuesClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ValuesClause, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DataBlock,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Update, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Update,
                                state: 6usize,
                            }),
                    );
                }
                (SyntaxKind::Update, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Update, 2usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Colon, 10isize);
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
                (SyntaxKind::Update, 1usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Update1,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Update, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Prologue,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Update1, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Update1, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Load,
                                state: 7usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Clear,
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
                                kind: SyntaxKind::Drop,
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
                                kind: SyntaxKind::Add,
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
                                kind: SyntaxKind::Move,
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
                                kind: SyntaxKind::Copy,
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
                                kind: SyntaxKind::Create,
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
                                kind: SyntaxKind::InsertData,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DeleteData,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DeleteWhere,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Modify,
                                state: 10usize,
                            }),
                    );
                }
                (SyntaxKind::Load, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Load, 1usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::IntoLit, 10isize);
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
                (SyntaxKind::Load, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Load, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphRef,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::Load, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SilentLit, 10isize);
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
                (SyntaxKind::Load, 7usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::LoadLit, 10isize);
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
                (SyntaxKind::Clear, 4usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ClearLit, 10isize);
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
                (SyntaxKind::Clear, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphRefAll,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Clear, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Clear, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphRefAll,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SilentLit, 10isize);
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
                (SyntaxKind::Drop, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphRefAll,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Drop, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Drop, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphRefAll,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SilentLit, 10isize);
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
                (SyntaxKind::Drop, 4usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DropLit, 10isize);
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
                (SyntaxKind::Create, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphRef,
                                state: 2usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SilentLit, 10isize);
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
                (SyntaxKind::Create, 4usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::CreateLit, 10isize);
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
                (SyntaxKind::Create, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphRef,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::Create, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Add, 6usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::AddLit, 10isize);
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
                (SyntaxKind::Add, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphOrDefault,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SilentLit, 10isize);
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
                (SyntaxKind::Add, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Add, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ToLit, 10isize);
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
                (SyntaxKind::Add, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphOrDefault,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Add, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphOrDefault,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Move, 6usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::MoveLit, 10isize);
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
                (SyntaxKind::Move, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ToLit, 10isize);
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
                (SyntaxKind::Move, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Move, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphOrDefault,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Move, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphOrDefault,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Move, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphOrDefault,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SilentLit, 10isize);
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
                (SyntaxKind::Copy, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Copy, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::ToLit, 10isize);
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
                (SyntaxKind::Copy, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphOrDefault,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SilentLit, 10isize);
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
                (SyntaxKind::Copy, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphOrDefault,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Copy, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphOrDefault,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Copy, 6usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::CopyLit, 10isize);
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
                (SyntaxKind::InsertData, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InsertData, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::QuadData,
                                state: 3usize,
                            }),
                    );
                }
                (SyntaxKind::InsertData, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::InsertDataLit, 10isize);
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
                (SyntaxKind::DeleteData, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DeleteData, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::QuadData,
                                state: 3usize,
                            }),
                    );
                }
                (SyntaxKind::DeleteData, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DeleteDataLit, 10isize);
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
                (SyntaxKind::DeleteWhere, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DeleteWhereLit, 10isize);
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
                (SyntaxKind::DeleteWhere, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::QuadPattern,
                                state: 3usize,
                            }),
                    );
                }
                (SyntaxKind::DeleteWhere, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Modify, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UsingClause,
                                state: 5usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::WhereLit, 10isize);
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
                (SyntaxKind::Modify, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UsingClause,
                                state: 5usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::WhereLit, 10isize);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::InsertClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::Modify, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DeleteClause,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::InsertClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::Modify, 11usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Modify, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::Modify, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Modify, 10usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DeleteClause,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::InsertClause,
                                state: 2usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::WithLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 11usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 11usize,
                        }));
                    }
                }
                (SyntaxKind::DeleteClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::QuadPattern,
                                state: 3usize,
                            }),
                    );
                }
                (SyntaxKind::DeleteClause, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DeleteLit, 10isize);
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
                (SyntaxKind::DeleteClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InsertClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InsertClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::QuadPattern,
                                state: 3usize,
                            }),
                    );
                }
                (SyntaxKind::InsertClause, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::InsertLit, 10isize);
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
                (SyntaxKind::UsingClause, 1usize) => {
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::NamedLit, 10isize);
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
                (SyntaxKind::UsingClause, 5usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::UsingLit, 10isize);
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
                (SyntaxKind::UsingClause, 3usize) => {
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
                (SyntaxKind::UsingClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphOrDefault, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DefaultLit, 10isize);
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
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::GraphLit, 10isize);
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
                (SyntaxKind::GraphOrDefault, 3usize) => {
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
                (SyntaxKind::GraphOrDefault, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphRef, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::GraphLit, 10isize);
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
                (SyntaxKind::GraphRef, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphRef, 1usize) => {
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
                (SyntaxKind::GraphRefAll, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphRef,
                                state: 2usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DefaultLit, 10isize);
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
                        state.expect_as_inline(element, SyntaxKind::NamedLit, 10isize);
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
                        state.expect_as_inline(element, SyntaxKind::AllLit, 10isize);
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
                (SyntaxKind::GraphRefAll, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::QuadPattern, 1usize) => {
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
                (SyntaxKind::QuadPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::QuadPattern, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Quads,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::QuadPattern, 3usize) => {
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
                (SyntaxKind::QuadData, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::QuadData, 3usize) => {
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
                (SyntaxKind::QuadData, 1usize) => {
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
                (SyntaxKind::QuadData, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Quads,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::Quads, 4usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesTemplate,
                                state: 5usize,
                            }),
                    );
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::Quads, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::QuadsNotTriples,
                                state: 6usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesTemplate,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::Quads, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::QuadsNotTriples,
                                state: 6usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Quads, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesTemplate,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::QuadsNotTriples, 2usize) => {
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesTemplate,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::QuadsNotTriples, 4usize) => {
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
                (SyntaxKind::QuadsNotTriples, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VarOrIri,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::QuadsNotTriples, 6usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::GraphLit, 10isize);
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
                (SyntaxKind::QuadsNotTriples, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::QuadsNotTriples, 1usize) => {
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
                (SyntaxKind::TriplesTemplate, 1usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::TriplesTemplate, 2usize) => {
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
                                kind: SyntaxKind::TriplesTemplate,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesTemplate, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesSameSubject,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesTemplate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupGraphPattern, 1usize) => {
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
                (SyntaxKind::GroupGraphPattern, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SubSelect,
                                state: 4usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPatternSub,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::GroupGraphPattern, 5usize) => {
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
                (SyntaxKind::GroupGraphPatternSub, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphPatternNotTriples,
                                state: 1usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupGraphPatternSub, 2usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesBlock,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::GroupGraphPatternSub, 4usize) => {
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesBlock,
                                state: 5usize,
                            }),
                    );
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::GroupGraphPatternSub, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphPatternNotTriples,
                                state: 1usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesBlock,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesBlock, 2usize) => {
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
                                kind: SyntaxKind::TriplesBlock,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesBlock, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TriplesBlock, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesSameSubjectPath,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesBlock, 1usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::GraphPatternNotTriples, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphPatternNotTriples, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupOrUnionGraphPattern,
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
                                kind: SyntaxKind::OptionalGraphPattern,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MinusGraphPattern,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphGraphPattern,
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
                                kind: SyntaxKind::ServiceGraphPattern,
                                state: 5usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Filter,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Bind,
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
                                kind: SyntaxKind::InlineData,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::OptionalGraphPattern, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::OptionalGraphPattern, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::OptionalLit, 10isize);
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
                (SyntaxKind::OptionalGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphGraphPattern, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VarOrIri,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::GraphGraphPattern, 3usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::GraphLit, 10isize);
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
                (SyntaxKind::GraphGraphPattern, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::ServiceGraphPattern, 5usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ServiceLit, 10isize);
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
                (SyntaxKind::ServiceGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ServiceGraphPattern, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VarOrIri,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ServiceGraphPattern, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::ServiceGraphPattern, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VarOrIri,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SilentLit, 10isize);
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
                (SyntaxKind::Bind, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Bind, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::AsLit, 10isize);
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
                (SyntaxKind::Bind, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::Bind, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Bind, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Bind, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::Bind, 6usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BindLit, 10isize);
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
                (SyntaxKind::InlineData, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InlineData, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DataBlock,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::InlineData, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ValuesLit, 10isize);
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
                (SyntaxKind::DataBlock, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DataBlock, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::InlineDataOneVar,
                                state: 5usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::InlineDataFull,
                                state: 10usize,
                            }),
                    );
                }
                (SyntaxKind::InlineDataOneVar, 4usize) => {
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
                (SyntaxKind::InlineDataOneVar, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DataBlockValue,
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
                (SyntaxKind::InlineDataOneVar, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InlineDataOneVar, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::InlineDataFull, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InlineDataFull, 9usize) => {
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
                (SyntaxKind::InlineDataFull, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DataBlockValue,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::InlineDataFull, 2usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Nil, 2isize);
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
                (SyntaxKind::InlineDataFull, 13usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 13usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 9usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 9usize,
                        }));
                    }
                }
                (SyntaxKind::InlineDataFull, 10usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Nil, 2isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 9usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 9usize,
                        }));
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 13usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 13usize,
                        }));
                    }
                }
                (SyntaxKind::DataBlockValue, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DataBlockValue, 1usize) => {
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::UndefLit, 10isize);
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
                (SyntaxKind::MinusGraphPattern, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::MinusGraphPattern, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::MinusLit, 10isize);
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
                (SyntaxKind::MinusGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupOrUnionGraphPattern, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::GroupOrUnionGraphPattern, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::GroupOrUnionGraphPattern, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::UnionLit, 10isize);
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
                (SyntaxKind::Filter, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Filter, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Constraint,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Filter, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::FilterLit, 10isize);
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
                (SyntaxKind::Constraint, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Constraint, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrackettedExpression,
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
                                kind: SyntaxKind::BuiltInCall,
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
                                kind: SyntaxKind::FunctionCall,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::FunctionCall, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ArgList,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::FunctionCall, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::FunctionCall, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ArgList, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Nil, 2isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 8usize,
                        }));
                    }
                }
                (SyntaxKind::ArgList, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DistinctLit, 10isize);
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
                (SyntaxKind::ArgList, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::ArgList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ArgList, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ArgList, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ExpressionList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ExpressionList, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Nil, 2isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::ExpressionList, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ExpressionList, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ExpressionList, 4usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::ConstructTemplate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ConstructTemplate, 2usize) => {
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ConstructTriples,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructTemplate, 1usize) => {
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
                (SyntaxKind::ConstructTemplate, 4usize) => {
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
                (SyntaxKind::ConstructTriples, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ConstructTriples, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesSameSubject,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructTriples, 1usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::ConstructTriples, 2usize) => {
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
                                kind: SyntaxKind::ConstructTriples,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesSameSubject, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TriplesSameSubject, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PropertyList,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesSameSubject, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PropertyListNotEmpty,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesSameSubject, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VarOrTerm,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesNode,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PropertyList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PropertyList, 1usize) => {
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
                                kind: SyntaxKind::PropertyListNotEmpty,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::PropertyListNotEmpty, 3usize) => {
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
                (SyntaxKind::PropertyListNotEmpty, 6usize) => {
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
                (SyntaxKind::PropertyListNotEmpty, 1usize) => {
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
                (SyntaxKind::PropertyListNotEmpty, 2usize) => {
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
                (SyntaxKind::PropertyListNotEmpty, 7usize) => {
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
                                kind: SyntaxKind::VarOrIri,
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
                (SyntaxKind::ObjectList, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::Object, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphNode,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Object, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TriplesSameSubjectPath, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PropertyListPath,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesSameSubjectPath, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VarOrTerm,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesNodePath,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesSameSubjectPath, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PropertyListPathNotEmpty,
                                state: 9usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesSameSubjectPath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PropertyListPath, 1usize) => {
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
                                kind: SyntaxKind::PropertyListPathNotEmpty,
                                state: 9usize,
                            }),
                    );
                }
                (SyntaxKind::PropertyListPath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PropertyListPathNotEmpty, 3usize) => {
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
                (SyntaxKind::PropertyListPathNotEmpty, 2usize) => {
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
                                kind: SyntaxKind::VerbPath,
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
                                kind: SyntaxKind::VerbSimple,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PropertyListPathNotEmpty, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ObjectListPath,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::PropertyListPathNotEmpty, 1usize) => {
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
                (SyntaxKind::PropertyListPathNotEmpty, 9usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VerbPath,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VerbSimple,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::VerbPath, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Path,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::VerbPath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::VerbSimple, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::VerbSimple, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ObjectListPath, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ObjectPath,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ObjectListPath, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ObjectPath,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ObjectListPath, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::ObjectPath, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphNodePath,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ObjectPath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Path, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Path, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathAlternative,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::PathAlternative, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathSequence,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::PathAlternative, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathSequence,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::PathAlternative, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Pipe, 10isize);
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
                (SyntaxKind::PathSequence, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Div, 10isize);
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
                (SyntaxKind::PathSequence, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathEltOrInverse,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PathSequence, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathEltOrInverse,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PathElt, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PathElt, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathPrimary,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PathElt, 1usize) => {
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
                                kind: SyntaxKind::PathMod,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PathEltOrInverse, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathElt,
                                state: 3usize,
                            }),
                    );
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Hat, 10isize);
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
                (SyntaxKind::PathEltOrInverse, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PathEltOrInverse, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathElt,
                                state: 3usize,
                            }),
                    );
                }
                (SyntaxKind::PathMod, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PathMod, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::Questionmark, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Star, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Plus, 10isize);
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
                (SyntaxKind::PathPrimary, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Path,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PathPrimary, 1usize) => {
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Bang, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::PathPrimary, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathNegatedPropertySet,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PathPrimary, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PathPrimary, 6usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::PathNegatedPropertySet, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathOneInPropertySet,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PathNegatedPropertySet, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PathNegatedPropertySet, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathOneInPropertySet,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::PathNegatedPropertySet, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Pipe, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 6usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 6usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::PathNegatedPropertySet, 4usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PathOneInPropertySet,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PathOneInPropertySet, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PathOneInPropertySet, 1usize) => {
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Hat, 10isize);
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
                (SyntaxKind::PathOneInPropertySet, 4usize) => {
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
                (SyntaxKind::TriplesNode, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Collection,
                                state: 5usize,
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
                }
                (SyntaxKind::TriplesNode, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                                kind: SyntaxKind::PropertyListNotEmpty,
                                state: 7usize,
                            }),
                    );
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
                (SyntaxKind::TriplesNodePath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TriplesNodePath, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::CollectionPath,
                                state: 5usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNodePropertyListPath,
                                state: 3usize,
                            }),
                    );
                }
                (SyntaxKind::BlankNodePropertyListPath, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PropertyListPathNotEmpty,
                                state: 9usize,
                            }),
                    );
                }
                (SyntaxKind::BlankNodePropertyListPath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BlankNodePropertyListPath, 1usize) => {
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
                (SyntaxKind::BlankNodePropertyListPath, 3usize) => {
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
                (SyntaxKind::Collection, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Collection, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphNode,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Collection, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::Collection, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphNode,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::CollectionPath, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphNodePath,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::CollectionPath, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphNodePath,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::CollectionPath, 5usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::CollectionPath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphNode, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphNode, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VarOrTerm,
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
                                kind: SyntaxKind::TriplesNode,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::GraphNodePath, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::VarOrTerm,
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
                                kind: SyntaxKind::TriplesNodePath,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::GraphNodePath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::VarOrTerm, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::VarOrTerm, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
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
                                kind: SyntaxKind::GraphTerm,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::VarOrIri, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::VarOrIri, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
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
                                kind: SyntaxKind::Iri,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Var, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Var1, 2isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Var2, 2isize);
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
                (SyntaxKind::Var, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphTerm, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphTerm, 1usize) => {
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Nil, 2isize);
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
                (SyntaxKind::Expression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ConditionalOrExpression,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::Expression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ConditionalOrExpression, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Pipe2, 10isize);
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
                (SyntaxKind::ConditionalOrExpression, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ConditionalAndExpression,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::ConditionalOrExpression, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ConditionalAndExpression,
                                state: 4usize,
                            }),
                    );
                }
                (SyntaxKind::ConditionalAndExpression, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Amp2, 10isize);
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
                (SyntaxKind::ConditionalAndExpression, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ValueLogical,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ConditionalAndExpression, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ValueLogical,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::ValueLogical, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::RelationalExpression,
                                state: 16usize,
                            }),
                    );
                }
                (SyntaxKind::ValueLogical, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::RelationalExpression, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NumericExpression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::RelationalExpression, 14usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::InLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 11usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 11usize,
                        }));
                    }
                }
                (SyntaxKind::RelationalExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::RelationalExpression, 11usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ExpressionList,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::RelationalExpression, 1usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Eq, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Neq, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Lt, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Gt, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Lte, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Gte, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::InLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 11usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 11usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::NotLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 14usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 14usize,
                        }));
                    }
                }
                (SyntaxKind::RelationalExpression, 16usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NumericExpression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::NumericExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NumericExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AdditiveExpression,
                                state: 15usize,
                            }),
                    );
                }
                (SyntaxKind::AdditiveExpression, 15usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MultiplicativeExpression,
                                state: 6usize,
                            }),
                    );
                }
                (SyntaxKind::AdditiveExpression, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Plus, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Bar, 10isize);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NumericLiteralPositive,
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NumericLiteralNegative,
                                state: 1usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AdditiveExpression, 7usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Star, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 8usize,
                        }));
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Div, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 8usize,
                        }));
                    }
                    state.add_element(element.pop_push(Rule {
                        kind: self.kind,
                        state: 1usize,
                    }));
                }
                (SyntaxKind::AdditiveExpression, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MultiplicativeExpression,
                                state: 6usize,
                            }),
                    );
                }
                (SyntaxKind::AdditiveExpression, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UnaryExpression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::MultiplicativeExpression, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Star, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Div, 10isize);
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
                (SyntaxKind::MultiplicativeExpression, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UnaryExpression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::MultiplicativeExpression, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UnaryExpression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::UnaryExpression, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PrimaryExpression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::UnaryExpression, 1usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Bang, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Plus, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Bar, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 6usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 6usize,
                        }));
                    }
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PrimaryExpression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::UnaryExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::UnaryExpression, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PrimaryExpression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::UnaryExpression, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PrimaryExpression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::PrimaryExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrimaryExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrackettedExpression,
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
                                kind: SyntaxKind::BuiltInCall,
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
                                kind: SyntaxKind::IriOrFunction,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BrackettedExpression, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BrackettedExpression, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::BrackettedExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BrackettedExpression, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::BuiltInCall, 59usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 58usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 58usize,
                        }));
                    }
                }
                (SyntaxKind::BuiltInCall, 56usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 55usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 55usize,
                        }));
                    }
                }
                (SyntaxKind::BuiltInCall, 41usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 40usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 61usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 63usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 63usize,
                        }));
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Nil, 2isize);
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
                (SyntaxKind::BuiltInCall, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 63usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 62usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 54usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 41usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 41usize,
                        }));
                    }
                }
                (SyntaxKind::BuiltInCall, 7usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::BuiltInCall, 58usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 40usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::BuiltInCall, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Aggregate,
                                state: 2usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StrLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::LangLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DatatypeLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::IriLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::UriLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::AbsLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::CeilLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::FloorLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::RoundLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StrlenLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::UcaseLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::LcaseLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::EncodeForUriLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::YearLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::MonthLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DayLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::HoursLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::MinutesLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SecondsLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::TimezoneLit, 10isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::TzLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::Md5Lit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::Sha1Lit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::Sha256Lit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::Sha384Lit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::Sha512Lit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::IsIriLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::IsUriLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::IsBlankLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::IsLiteralLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::IsNumericLit, 10isize);
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
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::LangmatchesLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 43usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 43usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ContainsLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 43usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 43usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StrstartsLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 43usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 43usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StrendsLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 43usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 43usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StrbeforeLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 43usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 43usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StrafterLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 43usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 43usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StrlangLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 43usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 43usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StrdtLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 43usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 43usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SameTermLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 43usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 43usize,
                        }));
                    }
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::IfLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 56usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 56usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BoundLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 59usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 59usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BnodeLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 61usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 61usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::RandLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 67usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 67usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::NowLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 67usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 67usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::UuidLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 67usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 67usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StruuidLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 67usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 67usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ConcatLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 73usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 73usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::CoalesceLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 73usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 73usize,
                        }));
                    }
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SubstringExpression,
                                state: 9usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StrReplaceExpression,
                                state: 11usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::RegexExpression,
                                state: 9usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ExistsFunc,
                                state: 2usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NotExistsFunc,
                                state: 3usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 67usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Nil, 2isize);
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
                (SyntaxKind::BuiltInCall, 55usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 54usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 43usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 41usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 41usize,
                        }));
                    }
                }
                (SyntaxKind::BuiltInCall, 3usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::BuiltInCall, 62usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::BuiltInCall, 73usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ExpressionList,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::RegexExpression, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::RegexExpression, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::RegexExpression, 8usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::RegexExpression, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::RegexExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::RegexExpression, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::RegexExpression, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::RegexExpression, 9usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::RegexLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 8usize,
                        }));
                    }
                }
                (SyntaxKind::RegexExpression, 6usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::SubstringExpression, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::SubstringExpression, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::SubstringExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SubstringExpression, 6usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::SubstringExpression, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::SubstringExpression, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::SubstringExpression, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::SubstringExpression, 8usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::SubstringExpression, 9usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SubstrLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 8usize,
                        }));
                    }
                }
                (SyntaxKind::StrReplaceExpression, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::StrReplaceExpression, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::StrReplaceExpression, 10usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 9usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 9usize,
                        }));
                    }
                }
                (SyntaxKind::StrReplaceExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StrReplaceExpression, 6usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::StrReplaceExpression, 8usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::StrReplaceExpression, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::StrReplaceExpression, 9usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::StrReplaceExpression, 11usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ReplaceLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 10usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 10usize,
                        }));
                    }
                }
                (SyntaxKind::StrReplaceExpression, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::StrReplaceExpression, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::ExistsFunc, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ExistsLit, 10isize);
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
                (SyntaxKind::ExistsFunc, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::ExistsFunc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NotExistsFunc, 3usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::NotLit, 10isize);
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
                (SyntaxKind::NotExistsFunc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NotExistsFunc, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupGraphPattern,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::NotExistsFunc, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::ExistsLit, 10isize);
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
                (SyntaxKind::Aggregate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Aggregate, 25usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 20usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 13usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 11usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 11usize,
                        }));
                    }
                }
                (SyntaxKind::Aggregate, 2usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::CountLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 8usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 8usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SumLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 13usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 13usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::MinLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 13usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 13usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::MaxLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 13usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 13usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::AvgLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 13usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 13usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SampleLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 13usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 13usize,
                        }));
                    }
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::GroupConcatLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 28usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 28usize,
                        }));
                    }
                }
                (SyntaxKind::Aggregate, 3usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Star, 10isize);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::Aggregate, 20usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::BrClose, 8isize);
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
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Colon, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 23usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 23usize,
                        }));
                    }
                }
                (SyntaxKind::Aggregate, 22usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Eq, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 21usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 21usize,
                        }));
                    }
                }
                (SyntaxKind::Aggregate, 23usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::SeparatorLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 22usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 22usize,
                        }));
                    }
                }
                (SyntaxKind::Aggregate, 8usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 6usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 6usize,
                        }));
                    }
                }
                (SyntaxKind::Aggregate, 10usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 11usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DistinctLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 10usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 10usize,
                        }));
                    }
                }
                (SyntaxKind::Aggregate, 28usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::BrOpen, 8isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 26usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 26usize,
                        }));
                    }
                }
                (SyntaxKind::Aggregate, 6usize) => {
                    let (matched, fb) = state.expect_as_inline(element, SyntaxKind::Star, 10isize);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DistinctLit, 10isize);
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
                (SyntaxKind::Aggregate, 21usize) => {
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
                (SyntaxKind::Aggregate, 26usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 20usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::DistinctLit, 10isize);
                    state.add_element(matched.pop_push(Rule {
                        kind: self.kind,
                        state: 25usize,
                    }));
                    if let Some(fb) = fb {
                        state.add_element(fb.pop_push(Rule {
                            kind: self.kind,
                            state: 25usize,
                        }));
                    }
                }
                (SyntaxKind::IriOrFunction, 1usize) => {
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
                                kind: SyntaxKind::ArgList,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::IriOrFunction, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IriOrFunction, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
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
                (SyntaxKind::NumericLiteral, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NumericLiteralUnsigned,
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
                                kind: SyntaxKind::NumericLiteralPositive,
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
                                kind: SyntaxKind::NumericLiteralNegative,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::NumericLiteral, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NumericLiteralUnsigned, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NumericLiteralUnsigned, 1usize) => {
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
                (SyntaxKind::NumericLiteralPositive, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NumericLiteralPositive, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::IntegerPositive, 2isize);
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
                        state.expect_as_inline(element, SyntaxKind::DecimalPositive, 2isize);
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
                        state.expect_as_inline(element, SyntaxKind::DoublePositive, 2isize);
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
                (SyntaxKind::NumericLiteralNegative, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NumericLiteralNegative, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::IntegerNegative, 2isize);
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
                        state.expect_as_inline(element, SyntaxKind::DecimalNegative, 2isize);
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
                        state.expect_as_inline(element, SyntaxKind::DoubleNegative, 2isize);
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
                (SyntaxKind::MyString, 1usize) => {
                    let (matched, fb) =
                        state.expect_as_inline(element, SyntaxKind::StringLiteral1, 2isize);
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
                        state.expect_as_inline(element, SyntaxKind::StringLiteral2, 2isize);
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
                        state.expect_as_inline(element, SyntaxKind::StringLiteralLong1, 2isize);
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
                        state.expect_as_inline(element, SyntaxKind::StringLiteralLong2, 2isize);
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
                (SyntaxKind::BlankNode, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                }
                (SyntaxKind::NowLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::NowLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::RegexLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::RegexLit, 10isize);
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
                (SyntaxKind::IsLiteralLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsLiteralLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::RandLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::RandLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ExistsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ExistsLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::CreateLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CreateLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StrlenLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrlenLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Lte, _) => {
                    let added = state.expect_as(element, SyntaxKind::Lte, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ServiceLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ServiceLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DescLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DescLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PnameLn, _) => {
                    let added = state.expect_as(element, SyntaxKind::PnameLn, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::LoadLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LoadLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::MaxLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MaxLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IsUriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsUriLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InsertLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::InsertLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NotLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::NotLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TimezoneLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::TimezoneLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SumLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SumLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AskLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AskLit, 10isize);
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
                (SyntaxKind::SqOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::SqOpen, 8isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::UcaseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UcaseLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::MonthLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MonthLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::CoalesceLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CoalesceLit, 10isize);
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
                (SyntaxKind::BindLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BindLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StrbeforeLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrbeforeLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Star, _) => {
                    let added = state.expect_as(element, SyntaxKind::Star, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ReplaceLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ReplaceLit, 10isize);
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
                (SyntaxKind::Sha256Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Sha256Lit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BrOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::BrOpen, 8isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::UuidLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UuidLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::LangmatchesLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LangmatchesLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::MinLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MinLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Var2, _) => {
                    let added = state.expect_as(element, SyntaxKind::Var2, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::FromLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FromLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteralLong2, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralLong2, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Neq, _) => {
                    let added = state.expect_as(element, SyntaxKind::Neq, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteral2, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteral2, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StrstartsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrstartsLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StrLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Sha384Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Sha384Lit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::OrderLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::OrderLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ContainsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ContainsLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DropLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DropLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IriLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::GroupLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ClearLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ClearLit, 10isize);
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
                (SyntaxKind::MinutesLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MinutesLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AvgLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AvgLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteralLong1, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralLong1, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::LimitLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LimitLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SubstrLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SubstrLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DeleteLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DeleteLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::LcaseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LcaseLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StrafterLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrafterLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SampleLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SampleLit, 10isize);
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
                (SyntaxKind::ToLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ToLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::UndefLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UndefLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::FilterLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FilterLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Pipe2, _) => {
                    let added = state.expect_as(element, SyntaxKind::Pipe2, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DoublePositive, _) => {
                    let added = state.expect_as(element, SyntaxKind::DoublePositive, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BoundLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BoundLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Sha512Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Sha512Lit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IntegerPositive, _) => {
                    let added = state.expect_as(element, SyntaxKind::IntegerPositive, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SameTermLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SameTermLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::RoundLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::RoundLit, 10isize);
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
                (SyntaxKind::Eq, _) => {
                    let added = state.expect_as(element, SyntaxKind::Eq, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::UsingLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UsingLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DefaultLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DefaultLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Gt, _) => {
                    let added = state.expect_as(element, SyntaxKind::Gt, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Gte, _) => {
                    let added = state.expect_as(element, SyntaxKind::Gte, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AllLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AllLit, 10isize);
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
                (SyntaxKind::StruuidLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StruuidLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SparqlPrefixToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::SparqlPrefixToken, 100isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Bang, _) => {
                    let added = state.expect_as(element, SyntaxKind::Bang, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DayLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DayLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IsBlankLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsBlankLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IntegerNegative, _) => {
                    let added = state.expect_as(element, SyntaxKind::IntegerNegative, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ByLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ByLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DoubleNegative, _) => {
                    let added = state.expect_as(element, SyntaxKind::DoubleNegative, 2isize);
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
                (SyntaxKind::TzLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::TzLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DeleteWhereLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DeleteWhereLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AsLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::InLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IsIriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsIriLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::OptionalLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::OptionalLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::OffsetLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::OffsetLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ConstructLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ConstructLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Comma, _) => {
                    let added = state.expect_as(element, SyntaxKind::Comma, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::LangLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LangLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Nil, _) => {
                    let added = state.expect_as(element, SyntaxKind::Nil, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DescribeLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DescribeLit, 10isize);
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
                (SyntaxKind::Langtag, _) => {
                    let added = state.expect_as(element, SyntaxKind::Langtag, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::MinusLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MinusLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Var1, _) => {
                    let added = state.expect_as(element, SyntaxKind::Var1, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DecimalNegative, _) => {
                    let added = state.expect_as(element, SyntaxKind::DecimalNegative, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AddLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AddLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GraphLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::GraphLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StrdtLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrdtLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DecimalPositive, _) => {
                    let added = state.expect_as(element, SyntaxKind::DecimalPositive, 2isize);
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
                (SyntaxKind::SilentLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SilentLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StrlangLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrlangLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Bar, _) => {
                    let added = state.expect_as(element, SyntaxKind::Bar, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NamedLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::NamedLit, 10isize);
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
                (SyntaxKind::HavingLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::HavingLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BrClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::BrClose, 8isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SelectLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SelectLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::WhereLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::WhereLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::YearLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::YearLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::HoursLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::HoursLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StringLiteral1, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteral1, 2isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DistinctLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DistinctLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::EncodeForUriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::EncodeForUriLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::CeilLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CeilLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::UriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UriLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SparqlBaseToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::SparqlBaseToken, 100isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::MoveLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MoveLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Div, _) => {
                    let added = state.expect_as(element, SyntaxKind::Div, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IsNumericLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsNumericLit, 10isize);
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
                (SyntaxKind::AscLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AscLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::CopyLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CopyLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DatatypeLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DatatypeLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AbsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AbsLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InsertDataLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::InsertDataLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BnodeLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BnodeLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SeparatorLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SeparatorLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SecondsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SecondsLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IntoLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IntoLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::WithLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::WithLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Sha1Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Sha1Lit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ValuesLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ValuesLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StrendsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrendsLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ConcatLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ConcatLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Amp2, _) => {
                    let added = state.expect_as(element, SyntaxKind::Amp2, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::IfLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IfLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::CountLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CountLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupConcatLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::GroupConcatLit, 10isize);
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
                (SyntaxKind::Lt, _) => {
                    let added = state.expect_as(element, SyntaxKind::Lt, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Plus, _) => {
                    let added = state.expect_as(element, SyntaxKind::Plus, 10isize);
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
                (SyntaxKind::Md5Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Md5Lit, 10isize);
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
                (SyntaxKind::Questionmark, _) => {
                    let added = state.expect_as(element, SyntaxKind::Questionmark, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ReducedLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ReducedLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DeleteDataLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DeleteDataLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Hat, _) => {
                    let added = state.expect_as(element, SyntaxKind::Hat, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Pipe, _) => {
                    let added = state.expect_as(element, SyntaxKind::Pipe, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::FloorLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FloorLit, 10isize);
                    if let Some(parent) = added.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::UnionLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UnionLit, 10isize);
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
            _ => None,
        }
    }
}
