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
    #[token("CONCAT")]
    ConcatLit,
    #[token("LIMIT")]
    LimitLit,
    #[regex("(?&DOUBLE_NEGATIVE)")]
    DoubleNegative,
    #[token("&&")]
    Amp2,
    #[token("DELETE WHERE")]
    DeleteWhereLit,
    #[token("DELETE")]
    DeleteLit,
    #[token("MAX")]
    MaxLit,
    #[token("SEPARATOR")]
    SeparatorLit,
    #[token("^^")]
    Datatype,
    #[token("ASK")]
    AskLit,
    #[token("!=")]
    Neq,
    #[token("MD5")]
    Md5Lit,
    #[regex("(?&STRING_LITERAL_LONG1)")]
    StringLiteralLong1,
    #[token("ALL")]
    AllLit,
    #[token("||")]
    Pipe2,
    #[token("UNDEF")]
    UndefLit,
    #[token("MINUTES")]
    MinutesLit,
    #[token("SUM")]
    SumLit,
    #[token(",")]
    Comma,
    #[token("OPTIONAL")]
    OptionalLit,
    #[token("^")]
    Hat,
    #[regex("(?&VAR1)")]
    Var1,
    #[regex("(?&INTEGER_POSITIVE)")]
    IntegerPositive,
    #[token("DISTINCT")]
    DistinctLit,
    #[token("CREATE")]
    CreateLit,
    #[token("DAY")]
    DayLit,
    #[token(">=")]
    Gte,
    #[token("UNION")]
    UnionLit,
    #[token("BIND")]
    BindLit,
    #[token("IF")]
    IfLit,
    #[token("GROUP")]
    GroupLit,
    #[token("DESCRIBE")]
    DescribeLit,
    #[regex("(?&VAR2)")]
    Var2,
    #[token("DESC")]
    DescLit,
    #[token("/")]
    Div,
    #[token("PREFIX")]
    SparqlPrefixToken,
    #[token(";")]
    Colon,
    #[regex("(?&INTEGER_NEGATIVE)")]
    IntegerNegative,
    #[token("AVG")]
    AvgLit,
    #[token("LANG")]
    LangLit,
    #[token("!")]
    Bang,
    #[token("TIMEZONE")]
    TimezoneLit,
    #[token("{")]
    ClOpen,
    #[token("*")]
    Star,
    #[token("FILTER")]
    FilterLit,
    #[regex("(?&STRING_LITERAL1)")]
    StringLiteral1,
    #[regex("(?&STRING_LITERAL2)")]
    StringLiteral2,
    #[token("DEFAULT")]
    DefaultLit,
    #[token("OFFSET")]
    OffsetLit,
    #[token("true")]
    TrueLit,
    #[regex("(?&BLANK_NODE_LABEL)")]
    BlankNodeLabel,
    #[token("CEIL")]
    CeilLit,
    #[token(">")]
    Gt,
    #[token("SAMPLE")]
    SampleLit,
    #[token("STRLEN")]
    StrlenLit,
    #[token("STRENDS")]
    StrendsLit,
    #[token("MOVE")]
    MoveLit,
    #[token("DELETE DATA")]
    DeleteDataLit,
    #[token("STRBEFORE")]
    StrbeforeLit,
    #[regex("(?&DOUBLE)")]
    Double,
    #[token("INSERT")]
    InsertLit,
    #[token("COALESCE")]
    CoalesceLit,
    #[token("GROUP_CONCAT")]
    GroupConcatLit,
    #[token("NAMED")]
    NamedLit,
    #[token("ROUND")]
    RoundLit,
    #[regex("(?&DECIMAL)")]
    Decimal,
    #[token("BNODE")]
    BnodeLit,
    #[token("+")]
    Plus,
    #[token("NOT")]
    NotLit,
    #[token("CLEAR")]
    ClearLit,
    #[token("YEAR")]
    YearLit,
    #[token("INTO")]
    IntoLit,
    #[regex("(?&DECIMAL_POSITIVE)")]
    DecimalPositive,
    #[regex("(?&NIL)")]
    Nil,
    #[token("IN")]
    InLit,
    #[token("STRLANG")]
    StrlangLit,
    #[token(".")]
    Stop,
    #[token("a")]
    Alit,
    #[token("BOUND")]
    BoundLit,
    #[regex("(?&STRING_LITERAL_LONG2)")]
    StringLiteralLong2,
    #[token("LANGMATCHES")]
    LangmatchesLit,
    #[token(")")]
    BrClose,
    #[token("HAVING")]
    HavingLit,
    #[token("ASC")]
    AscLit,
    #[token("ADD")]
    AddLit,
    #[token("STRSTARTS")]
    StrstartsLit,
    #[token("sameTerm")]
    SameTermLit,
    #[token("RAND")]
    RandLit,
    #[token("(")]
    BrOpen,
    #[token("STRUUID")]
    StruuidLit,
    #[token("MIN")]
    MinLit,
    #[token("MONTH")]
    MonthLit,
    #[token("HOURS")]
    HoursLit,
    #[token("}")]
    ClClose,
    #[token("[")]
    SqOpen,
    #[regex("(?&INTEGER)")]
    Integer,
    #[token("<")]
    Lt,
    #[regex("(?&DECIMAL_NEGATIVE)")]
    DecimalNegative,
    #[token("STR")]
    StrLit,
    #[token("BY")]
    ByLit,
    #[token("SHA384")]
    Sha384Lit,
    #[token("REDUCED")]
    ReducedLit,
    #[token("DROP")]
    DropLit,
    #[token("ORDER")]
    OrderLit,
    #[token("BASE")]
    SparqlBaseToken,
    #[token("LOAD")]
    LoadLit,
    #[token("STRAFTER")]
    StrafterLit,
    #[token("INSERT DATA")]
    InsertDataLit,
    #[regex("(?&PNAME_NS)")]
    PnameNs,
    #[token("SHA256")]
    Sha256Lit,
    #[token("isBLANK")]
    IsBlankLit,
    #[token("ENCODE_FOR_URI")]
    EncodeForUriLit,
    #[token("CONTAINS")]
    ContainsLit,
    #[token("REPLACE")]
    ReplaceLit,
    #[token("<=")]
    Lte,
    #[regex("(?&PNAME_LN)")]
    PnameLn,
    #[token("SECONDS")]
    SecondsLit,
    #[token("GRAPH")]
    GraphLit,
    #[token("EXISTS")]
    ExistsLit,
    #[token("FROM")]
    FromLit,
    #[token("isIRI")]
    IsIriLit,
    #[token("isURI")]
    IsUriLit,
    #[token("WITH")]
    WithLit,
    #[token("]")]
    SqClose,
    #[token("NOW")]
    NowLit,
    #[regex("(?&DOUBLE_POSITIVE)")]
    DoublePositive,
    #[token("COUNT")]
    CountLit,
    #[token("SHA512")]
    Sha512Lit,
    #[token("false")]
    FalseLit,
    #[token("LCASE")]
    LcaseLit,
    #[token("|")]
    Pipe,
    #[token("UUID")]
    UuidLit,
    #[token("COPY")]
    CopyLit,
    #[token("SILENT")]
    SilentLit,
    #[token("URI")]
    UriLit,
    #[token("SHA1")]
    Sha1Lit,
    #[token("STRDT")]
    StrdtLit,
    #[token("DATATYPE")]
    DatatypeLit,
    #[token("MINUS")]
    MinusLit,
    #[regex("(?&IRIREF)")]
    Iriref,
    #[token("SERVICE")]
    ServiceLit,
    #[token("SUBSTR")]
    SubstrLit,
    #[token("SELECT")]
    SelectLit,
    #[token("VALUES")]
    ValuesLit,
    #[token("USING")]
    UsingLit,
    #[token("?")]
    Questionmark,
    #[token("-")]
    Bar,
    #[token("ABS")]
    AbsLit,
    #[token("TZ")]
    TzLit,
    #[regex("(?&LANGTAG)")]
    Langtag,
    #[token("UCASE")]
    UcaseLit,
    #[regex("(?&ANON)")]
    Anon,
    #[token("CONSTRUCT")]
    ConstructLit,
    #[token("WHERE")]
    WhereLit,
    #[token("=")]
    Eq,
    #[token("isNUMERIC")]
    IsNumericLit,
    #[token("AS")]
    AsLit,
    #[token("isLITERAL")]
    IsLiteralLit,
    #[token("FLOOR")]
    FloorLit,
    #[token("IRI")]
    IriLit,
    #[token("TO")]
    ToLit,
    #[token("REGEX")]
    RegexLit,
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
                SyntaxKind::ConcatLit => Rule { kind, state: 0 },
                SyntaxKind::LimitLit => Rule { kind, state: 0 },
                SyntaxKind::DoubleNegative => Rule { kind, state: 0 },
                SyntaxKind::Amp2 => Rule { kind, state: 0 },
                SyntaxKind::DeleteWhereLit => Rule { kind, state: 0 },
                SyntaxKind::DeleteLit => Rule { kind, state: 0 },
                SyntaxKind::MaxLit => Rule { kind, state: 0 },
                SyntaxKind::SeparatorLit => Rule { kind, state: 0 },
                SyntaxKind::Datatype => Rule { kind, state: 0 },
                SyntaxKind::AskLit => Rule { kind, state: 0 },
                SyntaxKind::Neq => Rule { kind, state: 0 },
                SyntaxKind::Md5Lit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLong1 => Rule { kind, state: 0 },
                SyntaxKind::AllLit => Rule { kind, state: 0 },
                SyntaxKind::Pipe2 => Rule { kind, state: 0 },
                SyntaxKind::UndefLit => Rule { kind, state: 0 },
                SyntaxKind::MinutesLit => Rule { kind, state: 0 },
                SyntaxKind::SumLit => Rule { kind, state: 0 },
                SyntaxKind::Comma => Rule { kind, state: 0 },
                SyntaxKind::OptionalLit => Rule { kind, state: 0 },
                SyntaxKind::Hat => Rule { kind, state: 0 },
                SyntaxKind::Var1 => Rule { kind, state: 0 },
                SyntaxKind::IntegerPositive => Rule { kind, state: 0 },
                SyntaxKind::DistinctLit => Rule { kind, state: 0 },
                SyntaxKind::CreateLit => Rule { kind, state: 0 },
                SyntaxKind::DayLit => Rule { kind, state: 0 },
                SyntaxKind::Gte => Rule { kind, state: 0 },
                SyntaxKind::UnionLit => Rule { kind, state: 0 },
                SyntaxKind::BindLit => Rule { kind, state: 0 },
                SyntaxKind::IfLit => Rule { kind, state: 0 },
                SyntaxKind::GroupLit => Rule { kind, state: 0 },
                SyntaxKind::DescribeLit => Rule { kind, state: 0 },
                SyntaxKind::Var2 => Rule { kind, state: 0 },
                SyntaxKind::DescLit => Rule { kind, state: 0 },
                SyntaxKind::Div => Rule { kind, state: 0 },
                SyntaxKind::SparqlPrefixToken => Rule { kind, state: 0 },
                SyntaxKind::Colon => Rule { kind, state: 0 },
                SyntaxKind::IntegerNegative => Rule { kind, state: 0 },
                SyntaxKind::AvgLit => Rule { kind, state: 0 },
                SyntaxKind::LangLit => Rule { kind, state: 0 },
                SyntaxKind::Bang => Rule { kind, state: 0 },
                SyntaxKind::TimezoneLit => Rule { kind, state: 0 },
                SyntaxKind::ClOpen => Rule { kind, state: 0 },
                SyntaxKind::Star => Rule { kind, state: 0 },
                SyntaxKind::FilterLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteral1 => Rule { kind, state: 0 },
                SyntaxKind::StringLiteral2 => Rule { kind, state: 0 },
                SyntaxKind::DefaultLit => Rule { kind, state: 0 },
                SyntaxKind::OffsetLit => Rule { kind, state: 0 },
                SyntaxKind::TrueLit => Rule { kind, state: 0 },
                SyntaxKind::BlankNodeLabel => Rule { kind, state: 0 },
                SyntaxKind::CeilLit => Rule { kind, state: 0 },
                SyntaxKind::Gt => Rule { kind, state: 0 },
                SyntaxKind::SampleLit => Rule { kind, state: 0 },
                SyntaxKind::StrlenLit => Rule { kind, state: 0 },
                SyntaxKind::StrendsLit => Rule { kind, state: 0 },
                SyntaxKind::MoveLit => Rule { kind, state: 0 },
                SyntaxKind::DeleteDataLit => Rule { kind, state: 0 },
                SyntaxKind::StrbeforeLit => Rule { kind, state: 0 },
                SyntaxKind::Double => Rule { kind, state: 0 },
                SyntaxKind::InsertLit => Rule { kind, state: 0 },
                SyntaxKind::CoalesceLit => Rule { kind, state: 0 },
                SyntaxKind::GroupConcatLit => Rule { kind, state: 0 },
                SyntaxKind::NamedLit => Rule { kind, state: 0 },
                SyntaxKind::RoundLit => Rule { kind, state: 0 },
                SyntaxKind::Decimal => Rule { kind, state: 0 },
                SyntaxKind::BnodeLit => Rule { kind, state: 0 },
                SyntaxKind::Plus => Rule { kind, state: 0 },
                SyntaxKind::NotLit => Rule { kind, state: 0 },
                SyntaxKind::ClearLit => Rule { kind, state: 0 },
                SyntaxKind::YearLit => Rule { kind, state: 0 },
                SyntaxKind::IntoLit => Rule { kind, state: 0 },
                SyntaxKind::DecimalPositive => Rule { kind, state: 0 },
                SyntaxKind::Nil => Rule { kind, state: 0 },
                SyntaxKind::InLit => Rule { kind, state: 0 },
                SyntaxKind::StrlangLit => Rule { kind, state: 0 },
                SyntaxKind::Stop => Rule { kind, state: 0 },
                SyntaxKind::Alit => Rule { kind, state: 0 },
                SyntaxKind::BoundLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLong2 => Rule { kind, state: 0 },
                SyntaxKind::LangmatchesLit => Rule { kind, state: 0 },
                SyntaxKind::BrClose => Rule { kind, state: 0 },
                SyntaxKind::HavingLit => Rule { kind, state: 0 },
                SyntaxKind::AscLit => Rule { kind, state: 0 },
                SyntaxKind::AddLit => Rule { kind, state: 0 },
                SyntaxKind::StrstartsLit => Rule { kind, state: 0 },
                SyntaxKind::SameTermLit => Rule { kind, state: 0 },
                SyntaxKind::RandLit => Rule { kind, state: 0 },
                SyntaxKind::BrOpen => Rule { kind, state: 0 },
                SyntaxKind::StruuidLit => Rule { kind, state: 0 },
                SyntaxKind::MinLit => Rule { kind, state: 0 },
                SyntaxKind::MonthLit => Rule { kind, state: 0 },
                SyntaxKind::HoursLit => Rule { kind, state: 0 },
                SyntaxKind::ClClose => Rule { kind, state: 0 },
                SyntaxKind::SqOpen => Rule { kind, state: 0 },
                SyntaxKind::Integer => Rule { kind, state: 0 },
                SyntaxKind::Lt => Rule { kind, state: 0 },
                SyntaxKind::DecimalNegative => Rule { kind, state: 0 },
                SyntaxKind::StrLit => Rule { kind, state: 0 },
                SyntaxKind::ByLit => Rule { kind, state: 0 },
                SyntaxKind::Sha384Lit => Rule { kind, state: 0 },
                SyntaxKind::ReducedLit => Rule { kind, state: 0 },
                SyntaxKind::DropLit => Rule { kind, state: 0 },
                SyntaxKind::OrderLit => Rule { kind, state: 0 },
                SyntaxKind::SparqlBaseToken => Rule { kind, state: 0 },
                SyntaxKind::LoadLit => Rule { kind, state: 0 },
                SyntaxKind::StrafterLit => Rule { kind, state: 0 },
                SyntaxKind::InsertDataLit => Rule { kind, state: 0 },
                SyntaxKind::PnameNs => Rule { kind, state: 0 },
                SyntaxKind::Sha256Lit => Rule { kind, state: 0 },
                SyntaxKind::IsBlankLit => Rule { kind, state: 0 },
                SyntaxKind::EncodeForUriLit => Rule { kind, state: 0 },
                SyntaxKind::ContainsLit => Rule { kind, state: 0 },
                SyntaxKind::ReplaceLit => Rule { kind, state: 0 },
                SyntaxKind::Lte => Rule { kind, state: 0 },
                SyntaxKind::PnameLn => Rule { kind, state: 0 },
                SyntaxKind::SecondsLit => Rule { kind, state: 0 },
                SyntaxKind::GraphLit => Rule { kind, state: 0 },
                SyntaxKind::ExistsLit => Rule { kind, state: 0 },
                SyntaxKind::FromLit => Rule { kind, state: 0 },
                SyntaxKind::IsIriLit => Rule { kind, state: 0 },
                SyntaxKind::IsUriLit => Rule { kind, state: 0 },
                SyntaxKind::WithLit => Rule { kind, state: 0 },
                SyntaxKind::SqClose => Rule { kind, state: 0 },
                SyntaxKind::NowLit => Rule { kind, state: 0 },
                SyntaxKind::DoublePositive => Rule { kind, state: 0 },
                SyntaxKind::CountLit => Rule { kind, state: 0 },
                SyntaxKind::Sha512Lit => Rule { kind, state: 0 },
                SyntaxKind::FalseLit => Rule { kind, state: 0 },
                SyntaxKind::LcaseLit => Rule { kind, state: 0 },
                SyntaxKind::Pipe => Rule { kind, state: 0 },
                SyntaxKind::UuidLit => Rule { kind, state: 0 },
                SyntaxKind::CopyLit => Rule { kind, state: 0 },
                SyntaxKind::SilentLit => Rule { kind, state: 0 },
                SyntaxKind::UriLit => Rule { kind, state: 0 },
                SyntaxKind::Sha1Lit => Rule { kind, state: 0 },
                SyntaxKind::StrdtLit => Rule { kind, state: 0 },
                SyntaxKind::DatatypeLit => Rule { kind, state: 0 },
                SyntaxKind::MinusLit => Rule { kind, state: 0 },
                SyntaxKind::Iriref => Rule { kind, state: 0 },
                SyntaxKind::ServiceLit => Rule { kind, state: 0 },
                SyntaxKind::SubstrLit => Rule { kind, state: 0 },
                SyntaxKind::SelectLit => Rule { kind, state: 0 },
                SyntaxKind::ValuesLit => Rule { kind, state: 0 },
                SyntaxKind::UsingLit => Rule { kind, state: 0 },
                SyntaxKind::Questionmark => Rule { kind, state: 0 },
                SyntaxKind::Bar => Rule { kind, state: 0 },
                SyntaxKind::AbsLit => Rule { kind, state: 0 },
                SyntaxKind::TzLit => Rule { kind, state: 0 },
                SyntaxKind::Langtag => Rule { kind, state: 0 },
                SyntaxKind::UcaseLit => Rule { kind, state: 0 },
                SyntaxKind::Anon => Rule { kind, state: 0 },
                SyntaxKind::ConstructLit => Rule { kind, state: 0 },
                SyntaxKind::WhereLit => Rule { kind, state: 0 },
                SyntaxKind::Eq => Rule { kind, state: 0 },
                SyntaxKind::IsNumericLit => Rule { kind, state: 0 },
                SyntaxKind::AsLit => Rule { kind, state: 0 },
                SyntaxKind::IsLiteralLit => Rule { kind, state: 0 },
                SyntaxKind::FloorLit => Rule { kind, state: 0 },
                SyntaxKind::IriLit => Rule { kind, state: 0 },
                SyntaxKind::ToLit => Rule { kind, state: 0 },
                SyntaxKind::RegexLit => Rule { kind, state: 0 },
                _ => panic!("Unknown rule kind {:?}", kind),
            }
        }
    }
    pub fn first_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::QuadPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::PropertyList => &[SyntaxKind::Alit],
            SyntaxKind::GroupGraphPatternSub => &[
                SyntaxKind::BrOpen,
                SyntaxKind::OptionalLit,
                SyntaxKind::GraphLit,
                SyntaxKind::SqOpen,
                SyntaxKind::ServiceLit,
                SyntaxKind::ClOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
                SyntaxKind::FilterLit,
                SyntaxKind::MinusLit,
                SyntaxKind::BindLit,
                SyntaxKind::ValuesLit,
            ],
            SyntaxKind::DeleteClause => &[SyntaxKind::DeleteLit],
            SyntaxKind::GroupClause => &[SyntaxKind::GroupLit],
            SyntaxKind::GroupCondition => &[
                SyntaxKind::DatatypeLit,
                SyntaxKind::RoundLit,
                SyntaxKind::AvgLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::NotLit,
                SyntaxKind::IfLit,
                SyntaxKind::NowLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::CountLit,
                SyntaxKind::UuidLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::RegexLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::MonthLit,
                SyntaxKind::StrLit,
                SyntaxKind::BoundLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::TzLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::MinLit,
                SyntaxKind::HoursLit,
                SyntaxKind::SampleLit,
                SyntaxKind::FloorLit,
                SyntaxKind::AbsLit,
                SyntaxKind::LangLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::YearLit,
                SyntaxKind::IriLit,
                SyntaxKind::RandLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::BrOpen,
                SyntaxKind::UriLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::MaxLit,
                SyntaxKind::CeilLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::DayLit,
                SyntaxKind::SumLit,
                SyntaxKind::Sha384Lit,
            ],
            SyntaxKind::PrimaryExpression => &[
                SyntaxKind::IsUriLit,
                SyntaxKind::MinLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::AvgLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::IfLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::MonthLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::MaxLit,
                SyntaxKind::RandLit,
                SyntaxKind::YearLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::IriLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::ContainsLit,
                SyntaxKind::BrOpen,
                SyntaxKind::HoursLit,
                SyntaxKind::UriLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::AbsLit,
                SyntaxKind::LangLit,
                SyntaxKind::SumLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::DayLit,
                SyntaxKind::BoundLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::CeilLit,
                SyntaxKind::FloorLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::StruuidLit,
                SyntaxKind::RegexLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::NowLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::UuidLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::TzLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::SampleLit,
                SyntaxKind::NotLit,
                SyntaxKind::CountLit,
                SyntaxKind::FalseLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::StrlangLit,
                SyntaxKind::StrLit,
                SyntaxKind::RoundLit,
            ],
            SyntaxKind::GraphTerm => &[SyntaxKind::TrueLit, SyntaxKind::FalseLit],
            SyntaxKind::QueryUnit => &[
                SyntaxKind::AskLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::DescribeLit,
                SyntaxKind::ConstructLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SelectLit,
            ],
            SyntaxKind::Constraint => &[
                SyntaxKind::DayLit,
                SyntaxKind::SumLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::BrOpen,
                SyntaxKind::UriLit,
                SyntaxKind::CountLit,
                SyntaxKind::StrLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::IfLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::NotLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::SecondsLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::AbsLit,
                SyntaxKind::LangLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::RandLit,
                SyntaxKind::CeilLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::SampleLit,
                SyntaxKind::MaxLit,
                SyntaxKind::MinLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::YearLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::AvgLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::RegexLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::HoursLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::TzLit,
                SyntaxKind::BoundLit,
                SyntaxKind::NowLit,
                SyntaxKind::RoundLit,
                SyntaxKind::UuidLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::IsIriLit,
                SyntaxKind::IriLit,
            ],
            SyntaxKind::GroupGraphPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::Filter => &[SyntaxKind::FilterLit],
            SyntaxKind::PathOneInPropertySet => &[SyntaxKind::Alit, SyntaxKind::Hat],
            SyntaxKind::DataBlock => &[SyntaxKind::BrOpen],
            SyntaxKind::SelectQuery => &[SyntaxKind::SelectLit],
            SyntaxKind::Prologue => &[SyntaxKind::SparqlBaseToken, SyntaxKind::SparqlPrefixToken],
            SyntaxKind::RelationalExpression => &[
                SyntaxKind::NotLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::MaxLit,
                SyntaxKind::YearLit,
                SyntaxKind::FalseLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::SampleLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::MinLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::DayLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::UriLit,
                SyntaxKind::FloorLit,
                SyntaxKind::RandLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::CountLit,
                SyntaxKind::CeilLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::TrueLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::LangLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::Bang,
                SyntaxKind::DatatypeLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::TzLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::HoursLit,
                SyntaxKind::SumLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::RegexLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::Bar,
                SyntaxKind::RoundLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::AvgLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::MonthLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::AbsLit,
                SyntaxKind::BrOpen,
                SyntaxKind::StrstartsLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::StrLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::BoundLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::IfLit,
                SyntaxKind::NowLit,
                SyntaxKind::IriLit,
                SyntaxKind::Plus,
                SyntaxKind::TimezoneLit,
                SyntaxKind::UuidLit,
            ],
            SyntaxKind::AskQuery => &[SyntaxKind::AskLit],
            SyntaxKind::UsingClause => &[SyntaxKind::UsingLit],
            SyntaxKind::WhereClause => &[SyntaxKind::ClOpen, SyntaxKind::WhereLit],
            SyntaxKind::Modify => &[
                SyntaxKind::WithLit,
                SyntaxKind::DeleteLit,
                SyntaxKind::InsertLit,
            ],
            SyntaxKind::BaseDecl => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::PrefixDecl => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::NumericLiteralUnsigned => &[],
            SyntaxKind::BlankNode => &[],
            SyntaxKind::TriplesSameSubject => &[
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::NumericExpression => &[
                SyntaxKind::LangmatchesLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::FloorLit,
                SyntaxKind::Plus,
                SyntaxKind::IsBlankLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::FalseLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::LangLit,
                SyntaxKind::BrOpen,
                SyntaxKind::MaxLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::SecondsLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::Md5Lit,
                SyntaxKind::RegexLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::YearLit,
                SyntaxKind::AbsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::NotLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::MinLit,
                SyntaxKind::IriLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::TrueLit,
                SyntaxKind::TzLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::HoursLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::RandLit,
                SyntaxKind::Bar,
                SyntaxKind::StrLit,
                SyntaxKind::Bang,
                SyntaxKind::UuidLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::SampleLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::SumLit,
                SyntaxKind::CountLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::CeilLit,
                SyntaxKind::NowLit,
                SyntaxKind::UriLit,
                SyntaxKind::BoundLit,
                SyntaxKind::DayLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::RoundLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::IsUriLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::IfLit,
                SyntaxKind::AvgLit,
                SyntaxKind::StrlenLit,
            ],
            SyntaxKind::NotExistsFunc => &[SyntaxKind::NotLit],
            SyntaxKind::ValueLogical => &[
                SyntaxKind::ReplaceLit,
                SyntaxKind::AbsLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::LangLit,
                SyntaxKind::MinLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::SumLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::AvgLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Plus,
                SyntaxKind::TimezoneLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::MaxLit,
                SyntaxKind::BrOpen,
                SyntaxKind::CountLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::BoundLit,
                SyntaxKind::NotLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::FalseLit,
                SyntaxKind::UriLit,
                SyntaxKind::RandLit,
                SyntaxKind::HoursLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::YearLit,
                SyntaxKind::RoundLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::Bang,
                SyntaxKind::IsNumericLit,
                SyntaxKind::DayLit,
                SyntaxKind::MonthLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::SampleLit,
                SyntaxKind::NowLit,
                SyntaxKind::IriLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::UuidLit,
                SyntaxKind::Bar,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::TzLit,
                SyntaxKind::CeilLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::RegexLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::IfLit,
            ],
            SyntaxKind::SourceSelector => &[],
            SyntaxKind::BrackettedExpression => &[SyntaxKind::BrOpen],
            SyntaxKind::TriplesSameSubjectPath => &[
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::SqOpen,
                SyntaxKind::FalseLit,
            ],
            SyntaxKind::GroupOrUnionGraphPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::BooleanLiteral => &[SyntaxKind::TrueLit, SyntaxKind::FalseLit],
            SyntaxKind::MinusGraphPattern => &[SyntaxKind::MinusLit],
            SyntaxKind::ServiceGraphPattern => &[SyntaxKind::ServiceLit],
            SyntaxKind::VarOrTerm => &[SyntaxKind::FalseLit, SyntaxKind::TrueLit],
            SyntaxKind::PathElt => &[SyntaxKind::Alit, SyntaxKind::Bang, SyntaxKind::BrOpen],
            SyntaxKind::DeleteData => &[SyntaxKind::DeleteDataLit],
            SyntaxKind::Copy => &[SyntaxKind::CopyLit],
            SyntaxKind::DefaultGraphClause => &[],
            SyntaxKind::NumericLiteralPositive => &[],
            SyntaxKind::SolutionModifier => &[
                SyntaxKind::LimitLit,
                SyntaxKind::HavingLit,
                SyntaxKind::OffsetLit,
                SyntaxKind::OrderLit,
                SyntaxKind::GroupLit,
            ],
            SyntaxKind::GraphGraphPattern => &[SyntaxKind::GraphLit],
            SyntaxKind::Verb => &[SyntaxKind::Alit],
            SyntaxKind::PathPrimary => &[SyntaxKind::Alit, SyntaxKind::Bang, SyntaxKind::BrOpen],
            SyntaxKind::Drop => &[SyntaxKind::DropLit],
            SyntaxKind::IriOrFunction => &[],
            SyntaxKind::ConditionalAndExpression => &[
                SyntaxKind::StrbeforeLit,
                SyntaxKind::FloorLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::SampleLit,
                SyntaxKind::MaxLit,
                SyntaxKind::DayLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::BoundLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::SumLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::YearLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::Bang,
                SyntaxKind::IsIriLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::MinLit,
                SyntaxKind::RegexLit,
                SyntaxKind::LangLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::RoundLit,
                SyntaxKind::Plus,
                SyntaxKind::NowLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::NotLit,
                SyntaxKind::HoursLit,
                SyntaxKind::IriLit,
                SyntaxKind::TzLit,
                SyntaxKind::AvgLit,
                SyntaxKind::UuidLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::AbsLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::IfLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::StrendsLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::CeilLit,
                SyntaxKind::StrLit,
                SyntaxKind::CountLit,
                SyntaxKind::RandLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Bar,
                SyntaxKind::UriLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::DatatypeLit,
            ],
            SyntaxKind::InsertData => &[SyntaxKind::InsertDataLit],
            SyntaxKind::LimitClause => &[SyntaxKind::LimitLit],
            SyntaxKind::PathAlternative => &[
                SyntaxKind::BrOpen,
                SyntaxKind::Hat,
                SyntaxKind::Alit,
                SyntaxKind::Bang,
            ],
            SyntaxKind::AdditiveExpression => &[
                SyntaxKind::Md5Lit,
                SyntaxKind::Bang,
                SyntaxKind::DayLit,
                SyntaxKind::RegexLit,
                SyntaxKind::NowLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::Plus,
                SyntaxKind::IsNumericLit,
                SyntaxKind::MinLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::RandLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::LcaseLit,
                SyntaxKind::BoundLit,
                SyntaxKind::UriLit,
                SyntaxKind::FalseLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::CountLit,
                SyntaxKind::AbsLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::SampleLit,
                SyntaxKind::YearLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::CeilLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::MaxLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::IriLit,
                SyntaxKind::BrOpen,
                SyntaxKind::Bar,
                SyntaxKind::StrLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::NotLit,
                SyntaxKind::TrueLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::FloorLit,
                SyntaxKind::UuidLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::TzLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::LangLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::HoursLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::SumLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::IfLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::AvgLit,
                SyntaxKind::StrlenLit,
            ],
            SyntaxKind::OffsetClause => &[SyntaxKind::OffsetLit],
            SyntaxKind::TriplesNodePath => &[SyntaxKind::BrOpen, SyntaxKind::SqOpen],
            SyntaxKind::GraphPatternNotTriples => &[
                SyntaxKind::MinusLit,
                SyntaxKind::ClOpen,
                SyntaxKind::GraphLit,
                SyntaxKind::BindLit,
                SyntaxKind::FilterLit,
                SyntaxKind::ServiceLit,
                SyntaxKind::ValuesLit,
                SyntaxKind::OptionalLit,
            ],
            SyntaxKind::GraphRef => &[SyntaxKind::GraphLit],
            SyntaxKind::TriplesTemplate => &[
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::DescribeQuery => &[SyntaxKind::DescribeLit],
            SyntaxKind::QuadsNotTriples => &[SyntaxKind::GraphLit],
            SyntaxKind::PathSequence => &[
                SyntaxKind::Hat,
                SyntaxKind::Alit,
                SyntaxKind::BrOpen,
                SyntaxKind::Bang,
            ],
            SyntaxKind::Query => &[
                SyntaxKind::AskLit,
                SyntaxKind::DescribeLit,
                SyntaxKind::SelectLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::ConstructLit,
            ],
            SyntaxKind::Iri => &[],
            SyntaxKind::NumericLiteralNegative => &[],
            SyntaxKind::HavingClause => &[SyntaxKind::HavingLit],
            SyntaxKind::ObjectList => &[
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
            ],
            SyntaxKind::OrderCondition => &[
                SyntaxKind::StrendsLit,
                SyntaxKind::DayLit,
                SyntaxKind::UuidLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::AscLit,
                SyntaxKind::SumLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::CeilLit,
                SyntaxKind::TzLit,
                SyntaxKind::SampleLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::RoundLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::NowLit,
                SyntaxKind::CountLit,
                SyntaxKind::HoursLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::MaxLit,
                SyntaxKind::MonthLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::LangLit,
                SyntaxKind::MinLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::NotLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrLit,
                SyntaxKind::RandLit,
                SyntaxKind::IfLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::RegexLit,
                SyntaxKind::BrOpen,
                SyntaxKind::BoundLit,
                SyntaxKind::AbsLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::IriLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::UriLit,
                SyntaxKind::AvgLit,
                SyntaxKind::YearLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::StrdtLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::DescLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::LangmatchesLit,
            ],
            SyntaxKind::UnaryExpression => &[
                SyntaxKind::SampleLit,
                SyntaxKind::MinLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::MonthLit,
                SyntaxKind::AvgLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::BoundLit,
                SyntaxKind::NowLit,
                SyntaxKind::Bang,
                SyntaxKind::ReplaceLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::NotLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::UcaseLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Plus,
                SyntaxKind::RegexLit,
                SyntaxKind::StrLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::FloorLit,
                SyntaxKind::UuidLit,
                SyntaxKind::HoursLit,
                SyntaxKind::TzLit,
                SyntaxKind::LangLit,
                SyntaxKind::Bar,
                SyntaxKind::SumLit,
                SyntaxKind::MaxLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::CountLit,
                SyntaxKind::FalseLit,
                SyntaxKind::AbsLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::StrlenLit,
                SyntaxKind::IfLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::BrOpen,
                SyntaxKind::StrendsLit,
                SyntaxKind::UriLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::ContainsLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::IriLit,
                SyntaxKind::YearLit,
                SyntaxKind::RandLit,
                SyntaxKind::CeilLit,
                SyntaxKind::DayLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::LcaseLit,
            ],
            SyntaxKind::GraphNode => &[
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::DataBlockValue => &[
                SyntaxKind::TrueLit,
                SyntaxKind::UndefLit,
                SyntaxKind::FalseLit,
            ],
            SyntaxKind::Rdfliteral => &[],
            SyntaxKind::Var => &[],
            SyntaxKind::Clear => &[SyntaxKind::ClearLit],
            SyntaxKind::BlankNodePropertyListPath => &[SyntaxKind::SqOpen],
            SyntaxKind::ConstructTriples => &[
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::ConstructQuery => &[SyntaxKind::ConstructLit],
            SyntaxKind::StrReplaceExpression => &[SyntaxKind::ReplaceLit],
            SyntaxKind::PrefixedName => &[],
            SyntaxKind::Aggregate => &[
                SyntaxKind::GroupConcatLit,
                SyntaxKind::SumLit,
                SyntaxKind::AvgLit,
                SyntaxKind::CountLit,
                SyntaxKind::MinLit,
                SyntaxKind::MaxLit,
                SyntaxKind::SampleLit,
            ],
            SyntaxKind::VerbPath => &[
                SyntaxKind::Hat,
                SyntaxKind::BrOpen,
                SyntaxKind::Bang,
                SyntaxKind::Alit,
            ],
            SyntaxKind::DatasetClause => &[SyntaxKind::FromLit],
            SyntaxKind::FunctionCall => &[],
            SyntaxKind::NumericLiteral => &[],
            SyntaxKind::Update => &[
                SyntaxKind::CopyLit,
                SyntaxKind::LoadLit,
                SyntaxKind::AddLit,
                SyntaxKind::InsertLit,
                SyntaxKind::InsertDataLit,
                SyntaxKind::DropLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::ClearLit,
                SyntaxKind::WithLit,
                SyntaxKind::CreateLit,
                SyntaxKind::DeleteDataLit,
                SyntaxKind::DeleteLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::MoveLit,
                SyntaxKind::DeleteWhereLit,
            ],
            SyntaxKind::BlankNodePropertyList => &[SyntaxKind::SqOpen],
            SyntaxKind::ConstructTemplate => &[SyntaxKind::ClOpen],
            SyntaxKind::Object => &[
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::PropertyListPathNotEmpty => &[
                SyntaxKind::Bang,
                SyntaxKind::Alit,
                SyntaxKind::BrOpen,
                SyntaxKind::Hat,
            ],
            SyntaxKind::SelectClause => &[SyntaxKind::SelectLit],
            SyntaxKind::GraphNodePath => &[
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::MultiplicativeExpression => &[
                SyntaxKind::StrlenLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::CeilLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::SecondsLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::FalseLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::IfLit,
                SyntaxKind::SumLit,
                SyntaxKind::TzLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::MaxLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::Bar,
                SyntaxKind::Bang,
                SyntaxKind::IriLit,
                SyntaxKind::BoundLit,
                SyntaxKind::SampleLit,
                SyntaxKind::UuidLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::UriLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::RegexLit,
                SyntaxKind::RoundLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::CountLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::HoursLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::NowLit,
                SyntaxKind::BrOpen,
                SyntaxKind::StrstartsLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::YearLit,
                SyntaxKind::Plus,
                SyntaxKind::IsNumericLit,
                SyntaxKind::AvgLit,
                SyntaxKind::StrLit,
                SyntaxKind::LangLit,
                SyntaxKind::MinLit,
                SyntaxKind::MonthLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::TrueLit,
                SyntaxKind::RandLit,
                SyntaxKind::NotLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::FloorLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::StruuidLit,
                SyntaxKind::AbsLit,
                SyntaxKind::DayLit,
                SyntaxKind::StrdtLit,
            ],
            SyntaxKind::ConditionalOrExpression => &[
                SyntaxKind::RoundLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::FloorLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::BoundLit,
                SyntaxKind::SumLit,
                SyntaxKind::HoursLit,
                SyntaxKind::MaxLit,
                SyntaxKind::BrOpen,
                SyntaxKind::ReplaceLit,
                SyntaxKind::YearLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::Bang,
                SyntaxKind::LangLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::IriLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::DayLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::MinLit,
                SyntaxKind::UuidLit,
                SyntaxKind::NotLit,
                SyntaxKind::SampleLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::StrLit,
                SyntaxKind::MonthLit,
                SyntaxKind::CountLit,
                SyntaxKind::RandLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::RegexLit,
                SyntaxKind::NowLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Bar,
                SyntaxKind::UriLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::AvgLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::StrendsLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::TrueLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::Plus,
                SyntaxKind::TzLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::IfLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::CeilLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::AbsLit,
                SyntaxKind::FalseLit,
            ],
            SyntaxKind::ExpressionList => &[SyntaxKind::BrOpen],
            SyntaxKind::NamedGraphClause => &[SyntaxKind::NamedLit],
            SyntaxKind::ObjectListPath => &[
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::TriplesBlock => &[
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::UpdateUnit => &[
                SyntaxKind::DeleteDataLit,
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::InsertDataLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::CreateLit,
                SyntaxKind::CopyLit,
                SyntaxKind::DeleteLit,
                SyntaxKind::MoveLit,
                SyntaxKind::LoadLit,
                SyntaxKind::ClearLit,
                SyntaxKind::AddLit,
                SyntaxKind::DropLit,
                SyntaxKind::InsertLit,
                SyntaxKind::WithLit,
            ],
            SyntaxKind::DeleteWhere => &[SyntaxKind::DeleteWhereLit],
            SyntaxKind::RegexExpression => &[SyntaxKind::RegexLit],
            SyntaxKind::HavingCondition => &[
                SyntaxKind::MonthLit,
                SyntaxKind::NotLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::RoundLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::CountLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::IriLit,
                SyntaxKind::CeilLit,
                SyntaxKind::AvgLit,
                SyntaxKind::YearLit,
                SyntaxKind::StrLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::RandLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::AbsLit,
                SyntaxKind::DayLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::IfLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::HoursLit,
                SyntaxKind::UriLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::SecondsLit,
                SyntaxKind::BoundLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::SampleLit,
                SyntaxKind::RegexLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::NowLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::BrOpen,
                SyntaxKind::TzLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::UuidLit,
                SyntaxKind::SumLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::StrlangLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::MaxLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::LangLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::MinLit,
            ],
            SyntaxKind::PropertyListPath => &[
                SyntaxKind::Alit,
                SyntaxKind::Bang,
                SyntaxKind::Hat,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::Collection => &[SyntaxKind::BrOpen],
            SyntaxKind::Create => &[SyntaxKind::CreateLit],
            SyntaxKind::ExistsFunc => &[SyntaxKind::ExistsLit],
            SyntaxKind::PathMod => &[SyntaxKind::Questionmark, SyntaxKind::Star, SyntaxKind::Plus],
            SyntaxKind::InlineDataFull => &[SyntaxKind::BrOpen],
            SyntaxKind::LimitOffsetClauses => &[SyntaxKind::OffsetLit, SyntaxKind::LimitLit],
            SyntaxKind::ObjectPath => &[
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::Update1 => &[
                SyntaxKind::InsertDataLit,
                SyntaxKind::MoveLit,
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::WithLit,
                SyntaxKind::DropLit,
                SyntaxKind::DeleteDataLit,
                SyntaxKind::DeleteLit,
                SyntaxKind::CreateLit,
                SyntaxKind::CopyLit,
                SyntaxKind::InsertLit,
                SyntaxKind::LoadLit,
                SyntaxKind::AddLit,
                SyntaxKind::ClearLit,
            ],
            SyntaxKind::OrderClause => &[SyntaxKind::OrderLit],
            SyntaxKind::MyString => &[],
            SyntaxKind::PropertyListNotEmpty => &[SyntaxKind::Alit],
            SyntaxKind::GraphOrDefault => &[SyntaxKind::DefaultLit, SyntaxKind::GraphLit],
            SyntaxKind::GraphRefAll => &[
                SyntaxKind::NamedLit,
                SyntaxKind::AllLit,
                SyntaxKind::DefaultLit,
                SyntaxKind::GraphLit,
            ],
            SyntaxKind::InsertClause => &[SyntaxKind::InsertLit],
            SyntaxKind::BuiltInCall => &[
                SyntaxKind::MonthLit,
                SyntaxKind::IriLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::TzLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::StrLit,
                SyntaxKind::RoundLit,
                SyntaxKind::UuidLit,
                SyntaxKind::LangLit,
                SyntaxKind::CountLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::UriLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::RandLit,
                SyntaxKind::YearLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::MaxLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::NowLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::AvgLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::DayLit,
                SyntaxKind::MinLit,
                SyntaxKind::SampleLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::ExistsLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::NotLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::RegexLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::BoundLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::SumLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::HoursLit,
                SyntaxKind::IfLit,
                SyntaxKind::CeilLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::AbsLit,
            ],
            SyntaxKind::InlineData => &[SyntaxKind::ValuesLit],
            SyntaxKind::InlineDataOneVar => &[],
            SyntaxKind::Move => &[SyntaxKind::MoveLit],
            SyntaxKind::TriplesNode => &[SyntaxKind::BrOpen, SyntaxKind::SqOpen],
            SyntaxKind::Quads => &[
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::SqOpen,
                SyntaxKind::GraphLit,
            ],
            SyntaxKind::PathNegatedPropertySet => {
                &[SyntaxKind::Hat, SyntaxKind::Alit, SyntaxKind::BrOpen]
            }
            SyntaxKind::PathEltOrInverse => &[
                SyntaxKind::Alit,
                SyntaxKind::BrOpen,
                SyntaxKind::Hat,
                SyntaxKind::Bang,
            ],
            SyntaxKind::VerbSimple => &[],
            SyntaxKind::Expression => &[
                SyntaxKind::MaxLit,
                SyntaxKind::IfLit,
                SyntaxKind::TrueLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::Bang,
                SyntaxKind::FalseLit,
                SyntaxKind::UriLit,
                SyntaxKind::HoursLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::AbsLit,
                SyntaxKind::CountLit,
                SyntaxKind::UuidLit,
                SyntaxKind::AvgLit,
                SyntaxKind::TzLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::Plus,
                SyntaxKind::CoalesceLit,
                SyntaxKind::YearLit,
                SyntaxKind::DayLit,
                SyntaxKind::FloorLit,
                SyntaxKind::RandLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::MonthLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::CeilLit,
                SyntaxKind::IriLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::BoundLit,
                SyntaxKind::BrOpen,
                SyntaxKind::RegexLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::LangLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::NotLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::MinLit,
                SyntaxKind::SampleLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::Bar,
                SyntaxKind::SubstrLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::NowLit,
                SyntaxKind::RoundLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::StrlenLit,
                SyntaxKind::SumLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::MinutesLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::StrLit,
            ],
            SyntaxKind::SubSelect => &[SyntaxKind::SelectLit],
            SyntaxKind::QuadData => &[SyntaxKind::ClOpen],
            SyntaxKind::Bind => &[SyntaxKind::BindLit],
            SyntaxKind::Path => &[
                SyntaxKind::Hat,
                SyntaxKind::BrOpen,
                SyntaxKind::Bang,
                SyntaxKind::Alit,
            ],
            SyntaxKind::ValuesClause => &[SyntaxKind::ValuesLit],
            SyntaxKind::ArgList => &[SyntaxKind::BrOpen],
            SyntaxKind::VarOrIri => &[],
            SyntaxKind::Load => &[SyntaxKind::LoadLit],
            SyntaxKind::CollectionPath => &[SyntaxKind::BrOpen],
            SyntaxKind::Add => &[SyntaxKind::AddLit],
            SyntaxKind::SubstringExpression => &[SyntaxKind::SubstrLit],
            SyntaxKind::OptionalGraphPattern => &[SyntaxKind::OptionalLit],
            SyntaxKind::ConcatLit => &[SyntaxKind::ConcatLit],
            SyntaxKind::LimitLit => &[SyntaxKind::LimitLit],
            SyntaxKind::DoubleNegative => &[SyntaxKind::DoubleNegative],
            SyntaxKind::Amp2 => &[SyntaxKind::Amp2],
            SyntaxKind::DeleteWhereLit => &[SyntaxKind::DeleteWhereLit],
            SyntaxKind::DeleteLit => &[SyntaxKind::DeleteLit],
            SyntaxKind::MaxLit => &[SyntaxKind::MaxLit],
            SyntaxKind::SeparatorLit => &[SyntaxKind::SeparatorLit],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::AskLit => &[SyntaxKind::AskLit],
            SyntaxKind::Neq => &[SyntaxKind::Neq],
            SyntaxKind::Md5Lit => &[SyntaxKind::Md5Lit],
            SyntaxKind::StringLiteralLong1 => &[SyntaxKind::StringLiteralLong1],
            SyntaxKind::AllLit => &[SyntaxKind::AllLit],
            SyntaxKind::Pipe2 => &[SyntaxKind::Pipe2],
            SyntaxKind::UndefLit => &[SyntaxKind::UndefLit],
            SyntaxKind::MinutesLit => &[SyntaxKind::MinutesLit],
            SyntaxKind::SumLit => &[SyntaxKind::SumLit],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::OptionalLit => &[SyntaxKind::OptionalLit],
            SyntaxKind::Hat => &[SyntaxKind::Hat],
            SyntaxKind::Var1 => &[SyntaxKind::Var1],
            SyntaxKind::IntegerPositive => &[SyntaxKind::IntegerPositive],
            SyntaxKind::DistinctLit => &[SyntaxKind::DistinctLit],
            SyntaxKind::CreateLit => &[SyntaxKind::CreateLit],
            SyntaxKind::DayLit => &[SyntaxKind::DayLit],
            SyntaxKind::Gte => &[SyntaxKind::Gte],
            SyntaxKind::UnionLit => &[SyntaxKind::UnionLit],
            SyntaxKind::BindLit => &[SyntaxKind::BindLit],
            SyntaxKind::IfLit => &[SyntaxKind::IfLit],
            SyntaxKind::GroupLit => &[SyntaxKind::GroupLit],
            SyntaxKind::DescribeLit => &[SyntaxKind::DescribeLit],
            SyntaxKind::Var2 => &[SyntaxKind::Var2],
            SyntaxKind::DescLit => &[SyntaxKind::DescLit],
            SyntaxKind::Div => &[SyntaxKind::Div],
            SyntaxKind::SparqlPrefixToken => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::IntegerNegative => &[SyntaxKind::IntegerNegative],
            SyntaxKind::AvgLit => &[SyntaxKind::AvgLit],
            SyntaxKind::LangLit => &[SyntaxKind::LangLit],
            SyntaxKind::Bang => &[SyntaxKind::Bang],
            SyntaxKind::TimezoneLit => &[SyntaxKind::TimezoneLit],
            SyntaxKind::ClOpen => &[SyntaxKind::ClOpen],
            SyntaxKind::Star => &[SyntaxKind::Star],
            SyntaxKind::FilterLit => &[SyntaxKind::FilterLit],
            SyntaxKind::StringLiteral1 => &[SyntaxKind::StringLiteral1],
            SyntaxKind::StringLiteral2 => &[SyntaxKind::StringLiteral2],
            SyntaxKind::DefaultLit => &[SyntaxKind::DefaultLit],
            SyntaxKind::OffsetLit => &[SyntaxKind::OffsetLit],
            SyntaxKind::TrueLit => &[SyntaxKind::TrueLit],
            SyntaxKind::BlankNodeLabel => &[SyntaxKind::BlankNodeLabel],
            SyntaxKind::CeilLit => &[SyntaxKind::CeilLit],
            SyntaxKind::Gt => &[SyntaxKind::Gt],
            SyntaxKind::SampleLit => &[SyntaxKind::SampleLit],
            SyntaxKind::StrlenLit => &[SyntaxKind::StrlenLit],
            SyntaxKind::StrendsLit => &[SyntaxKind::StrendsLit],
            SyntaxKind::MoveLit => &[SyntaxKind::MoveLit],
            SyntaxKind::DeleteDataLit => &[SyntaxKind::DeleteDataLit],
            SyntaxKind::StrbeforeLit => &[SyntaxKind::StrbeforeLit],
            SyntaxKind::Double => &[SyntaxKind::Double],
            SyntaxKind::InsertLit => &[SyntaxKind::InsertLit],
            SyntaxKind::CoalesceLit => &[SyntaxKind::CoalesceLit],
            SyntaxKind::GroupConcatLit => &[SyntaxKind::GroupConcatLit],
            SyntaxKind::NamedLit => &[SyntaxKind::NamedLit],
            SyntaxKind::RoundLit => &[SyntaxKind::RoundLit],
            SyntaxKind::Decimal => &[SyntaxKind::Decimal],
            SyntaxKind::BnodeLit => &[SyntaxKind::BnodeLit],
            SyntaxKind::Plus => &[SyntaxKind::Plus],
            SyntaxKind::NotLit => &[SyntaxKind::NotLit],
            SyntaxKind::ClearLit => &[SyntaxKind::ClearLit],
            SyntaxKind::YearLit => &[SyntaxKind::YearLit],
            SyntaxKind::IntoLit => &[SyntaxKind::IntoLit],
            SyntaxKind::DecimalPositive => &[SyntaxKind::DecimalPositive],
            SyntaxKind::Nil => &[SyntaxKind::Nil],
            SyntaxKind::InLit => &[SyntaxKind::InLit],
            SyntaxKind::StrlangLit => &[SyntaxKind::StrlangLit],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::Alit => &[SyntaxKind::Alit],
            SyntaxKind::BoundLit => &[SyntaxKind::BoundLit],
            SyntaxKind::StringLiteralLong2 => &[SyntaxKind::StringLiteralLong2],
            SyntaxKind::LangmatchesLit => &[SyntaxKind::LangmatchesLit],
            SyntaxKind::BrClose => &[SyntaxKind::BrClose],
            SyntaxKind::HavingLit => &[SyntaxKind::HavingLit],
            SyntaxKind::AscLit => &[SyntaxKind::AscLit],
            SyntaxKind::AddLit => &[SyntaxKind::AddLit],
            SyntaxKind::StrstartsLit => &[SyntaxKind::StrstartsLit],
            SyntaxKind::SameTermLit => &[SyntaxKind::SameTermLit],
            SyntaxKind::RandLit => &[SyntaxKind::RandLit],
            SyntaxKind::BrOpen => &[SyntaxKind::BrOpen],
            SyntaxKind::StruuidLit => &[SyntaxKind::StruuidLit],
            SyntaxKind::MinLit => &[SyntaxKind::MinLit],
            SyntaxKind::MonthLit => &[SyntaxKind::MonthLit],
            SyntaxKind::HoursLit => &[SyntaxKind::HoursLit],
            SyntaxKind::ClClose => &[SyntaxKind::ClClose],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::Integer => &[SyntaxKind::Integer],
            SyntaxKind::Lt => &[SyntaxKind::Lt],
            SyntaxKind::DecimalNegative => &[SyntaxKind::DecimalNegative],
            SyntaxKind::StrLit => &[SyntaxKind::StrLit],
            SyntaxKind::ByLit => &[SyntaxKind::ByLit],
            SyntaxKind::Sha384Lit => &[SyntaxKind::Sha384Lit],
            SyntaxKind::ReducedLit => &[SyntaxKind::ReducedLit],
            SyntaxKind::DropLit => &[SyntaxKind::DropLit],
            SyntaxKind::OrderLit => &[SyntaxKind::OrderLit],
            SyntaxKind::SparqlBaseToken => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::LoadLit => &[SyntaxKind::LoadLit],
            SyntaxKind::StrafterLit => &[SyntaxKind::StrafterLit],
            SyntaxKind::InsertDataLit => &[SyntaxKind::InsertDataLit],
            SyntaxKind::PnameNs => &[SyntaxKind::PnameNs],
            SyntaxKind::Sha256Lit => &[SyntaxKind::Sha256Lit],
            SyntaxKind::IsBlankLit => &[SyntaxKind::IsBlankLit],
            SyntaxKind::EncodeForUriLit => &[SyntaxKind::EncodeForUriLit],
            SyntaxKind::ContainsLit => &[SyntaxKind::ContainsLit],
            SyntaxKind::ReplaceLit => &[SyntaxKind::ReplaceLit],
            SyntaxKind::Lte => &[SyntaxKind::Lte],
            SyntaxKind::PnameLn => &[SyntaxKind::PnameLn],
            SyntaxKind::SecondsLit => &[SyntaxKind::SecondsLit],
            SyntaxKind::GraphLit => &[SyntaxKind::GraphLit],
            SyntaxKind::ExistsLit => &[SyntaxKind::ExistsLit],
            SyntaxKind::FromLit => &[SyntaxKind::FromLit],
            SyntaxKind::IsIriLit => &[SyntaxKind::IsIriLit],
            SyntaxKind::IsUriLit => &[SyntaxKind::IsUriLit],
            SyntaxKind::WithLit => &[SyntaxKind::WithLit],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::NowLit => &[SyntaxKind::NowLit],
            SyntaxKind::DoublePositive => &[SyntaxKind::DoublePositive],
            SyntaxKind::CountLit => &[SyntaxKind::CountLit],
            SyntaxKind::Sha512Lit => &[SyntaxKind::Sha512Lit],
            SyntaxKind::FalseLit => &[SyntaxKind::FalseLit],
            SyntaxKind::LcaseLit => &[SyntaxKind::LcaseLit],
            SyntaxKind::Pipe => &[SyntaxKind::Pipe],
            SyntaxKind::UuidLit => &[SyntaxKind::UuidLit],
            SyntaxKind::CopyLit => &[SyntaxKind::CopyLit],
            SyntaxKind::SilentLit => &[SyntaxKind::SilentLit],
            SyntaxKind::UriLit => &[SyntaxKind::UriLit],
            SyntaxKind::Sha1Lit => &[SyntaxKind::Sha1Lit],
            SyntaxKind::StrdtLit => &[SyntaxKind::StrdtLit],
            SyntaxKind::DatatypeLit => &[SyntaxKind::DatatypeLit],
            SyntaxKind::MinusLit => &[SyntaxKind::MinusLit],
            SyntaxKind::Iriref => &[SyntaxKind::Iriref],
            SyntaxKind::ServiceLit => &[SyntaxKind::ServiceLit],
            SyntaxKind::SubstrLit => &[SyntaxKind::SubstrLit],
            SyntaxKind::SelectLit => &[SyntaxKind::SelectLit],
            SyntaxKind::ValuesLit => &[SyntaxKind::ValuesLit],
            SyntaxKind::UsingLit => &[SyntaxKind::UsingLit],
            SyntaxKind::Questionmark => &[SyntaxKind::Questionmark],
            SyntaxKind::Bar => &[SyntaxKind::Bar],
            SyntaxKind::AbsLit => &[SyntaxKind::AbsLit],
            SyntaxKind::TzLit => &[SyntaxKind::TzLit],
            SyntaxKind::Langtag => &[SyntaxKind::Langtag],
            SyntaxKind::UcaseLit => &[SyntaxKind::UcaseLit],
            SyntaxKind::Anon => &[SyntaxKind::Anon],
            SyntaxKind::ConstructLit => &[SyntaxKind::ConstructLit],
            SyntaxKind::WhereLit => &[SyntaxKind::WhereLit],
            SyntaxKind::Eq => &[SyntaxKind::Eq],
            SyntaxKind::IsNumericLit => &[SyntaxKind::IsNumericLit],
            SyntaxKind::AsLit => &[SyntaxKind::AsLit],
            SyntaxKind::IsLiteralLit => &[SyntaxKind::IsLiteralLit],
            SyntaxKind::FloorLit => &[SyntaxKind::FloorLit],
            SyntaxKind::IriLit => &[SyntaxKind::IriLit],
            SyntaxKind::ToLit => &[SyntaxKind::ToLit],
            SyntaxKind::RegexLit => &[SyntaxKind::RegexLit],
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
                (SyntaxKind::Query, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iriref,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BaseDecl, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SparqlBaseToken,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BaseDecl, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixDecl, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PnameNs,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::PrefixDecl, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SparqlPrefixToken,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::PrefixDecl, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iriref,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::PrefixDecl, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::SelectQuery, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::SubSelect, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::SelectClause, 16usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 12usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SelectLit,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 9usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 9usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Star,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::SelectClause, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 9usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Star,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DistinctLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ReducedLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::SelectClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SelectClause, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AsLit,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 10usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::ConstructQuery, 10usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ConstructLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 10usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereLit,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Star,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DescribeLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::AskQuery, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AskLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::AskQuery, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::DatasetClause, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::FromLit,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NamedLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::SourceSelector, 0usize) => {
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::WhereClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SolutionModifier, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::GroupClause, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ByLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::GroupClause, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::GroupCondition, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupCondition, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
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
                (SyntaxKind::GroupCondition, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AsLit,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
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
                (SyntaxKind::HavingClause, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::HavingLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::HavingCondition, 0usize) => {
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
                (SyntaxKind::OrderClause, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::OrderLit,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ByLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::OrderCondition, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AscLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DescLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::OrderCondition, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::LimitClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::LimitClause, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::LimitLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::LimitClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Integer,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::OffsetClause, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Integer,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::OffsetClause, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::OffsetLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::OffsetClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ValuesClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ValuesClause, 1usize) => {
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
                                kind: SyntaxKind::ValuesLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Update, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Update, 2usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Colon,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SilentLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Load, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::LoadLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Load, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Load, 1usize) => {
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
                                kind: SyntaxKind::IntoLit,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SilentLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Clear, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClearLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Drop, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DropLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Drop, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SilentLit,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SilentLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Create, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::CreateLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Add, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::Add, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AddLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Add, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ToLit,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SilentLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Move, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ToLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Move, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MoveLit,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SilentLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Move, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SilentLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Copy, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::CopyLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Copy, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ToLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Copy, 0usize) => {
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::InsertDataLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::InsertData, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DeleteData, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DeleteDataLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::DeleteWhere, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DeleteWhereLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::DeleteWhere, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 11usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WithLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Modify, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereLit,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::DeleteClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::DeleteClause, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DeleteLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::InsertClause, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::InsertLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::InsertClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::UsingClause, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UsingLit,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NamedLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::GraphOrDefault, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DefaultLit,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphLit,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::GraphRefAll, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DefaultLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NamedLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AllLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::QuadPattern, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClOpen,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::QuadPattern, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::QuadPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::QuadData, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::QuadData, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::QuadData, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Stop,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::QuadsNotTriples, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::QuadsNotTriples, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::QuadsNotTriples, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::QuadsNotTriples, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::TriplesTemplate, 1usize) => {
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
                                kind: SyntaxKind::Stop,
                                state: 0usize,
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
                (SyntaxKind::TriplesTemplate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupGraphPattern, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::GroupGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::GroupGraphPattern, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClOpen,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Stop,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::TriplesBlock, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::TriplesBlock, 1usize) => {
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
                                kind: SyntaxKind::Stop,
                                state: 0usize,
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
                (SyntaxKind::OptionalGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::OptionalGraphPattern, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::OptionalLit,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GraphLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::GraphGraphPattern, 0usize) => {
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
                (SyntaxKind::ServiceGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SilentLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ServiceGraphPattern, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ServiceLit,
                                state: 0usize,
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
                (SyntaxKind::Bind, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Bind, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AsLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Bind, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BindLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Bind, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::Bind, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::InlineData, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ValuesLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::InlineDataOneVar, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InlineDataOneVar, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClOpen,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 9usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::InlineDataFull, 9usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::InlineDataFull, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::InlineDataFull, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Nil,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::InlineDataFull, 10usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 9usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Nil,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 13usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UndefLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::DataBlockValue, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::MinusGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::MinusGraphPattern, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MinusLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::GroupOrUnionGraphPattern, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UnionLit,
                                state: 0usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Filter, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Filter, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::FilterLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::ArgList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DistinctLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ArgList, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::ArgList, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Nil,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ExpressionList, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ExpressionList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::ExpressionList, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Nil,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::ConstructTemplate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ConstructTemplate, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructTemplate, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::ConstructTemplate, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClOpen,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::ConstructTriples, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ConstructTriples, 1usize) => {
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
                                kind: SyntaxKind::Stop,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::TriplesSameSubject, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::PropertyListNotEmpty, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Colon,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Alit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Verb, 0usize) => {
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
                (SyntaxKind::ObjectList, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::Object, 0usize) => {
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
                                kind: SyntaxKind::GraphNode,
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
                (SyntaxKind::TriplesSameSubjectPath, 0usize) => {
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
                (SyntaxKind::PropertyListPath, 0usize) => {
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Colon,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::VerbSimple, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Path, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PathAlternative, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Pipe,
                                state: 0usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::PathSequence, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Div,
                                state: 0usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Hat,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Questionmark,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Star,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Plus,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Alit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Bang,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::PathPrimary, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::PathPrimary, 0usize) => {
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::PathNegatedPropertySet, 4usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::PathNegatedPropertySet, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Pipe,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Alit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Hat,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::PathOneInPropertySet, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Alit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::TriplesNode, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::BlankNodePropertyList, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SqOpen,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::BlankNodePropertyList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BlankNodePropertyList, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SqClose,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::BlankNodePropertyListPath, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SqOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BlankNodePropertyListPath, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SqClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Collection, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::CollectionPath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::GraphNode, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::VarOrTerm, 0usize) => {
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
                (SyntaxKind::VarOrIri, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Var, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Var, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var1,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var2,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Nil,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Expression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::ConditionalOrExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Pipe2,
                                state: 0usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::ConditionalAndExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Amp2,
                                state: 0usize,
                            }),
                    );
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ValueLogical, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::RelationalExpression, 1usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Eq,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Neq,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Lt,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Gt,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Lte,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Gte,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 11usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::InLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 14usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NotLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::RelationalExpression, 14usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 11usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::InLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::AdditiveExpression, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Star,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Div,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::AdditiveExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Plus,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Bar,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::MultiplicativeExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Star,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Div,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::UnaryExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Bang,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Plus,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Bar,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::PrimaryExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BrackettedExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BrackettedExpression, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
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
                (SyntaxKind::BrackettedExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BuiltInCall, 43usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 41usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 40usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 61usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 63usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Nil,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::BuiltInCall, 56usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 55usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::BuiltInCall, 62usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 67usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Nil,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StrLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::LangLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DatatypeLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::IriLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UriLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AbsLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::CeilLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::FloorLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::RoundLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StrlenLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UcaseLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::LcaseLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::EncodeForUriLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::YearLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MonthLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DayLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::HoursLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MinutesLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SecondsLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TimezoneLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TzLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Md5Lit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Sha1Lit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Sha256Lit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Sha384Lit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Sha512Lit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::IsIriLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::IsUriLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::IsBlankLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::IsLiteralLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::IsNumericLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 43usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::LangmatchesLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 43usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ContainsLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 43usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StrstartsLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 43usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StrendsLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 43usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StrbeforeLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 43usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StrafterLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 43usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StrlangLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 43usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StrdtLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 43usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SameTermLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 56usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::IfLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 59usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BoundLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 61usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BnodeLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 67usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::RandLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 67usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NowLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 67usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::UuidLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 67usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StruuidLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 73usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ConcatLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 73usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::CoalesceLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::BuiltInCall, 54usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 41usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::BuiltInCall, 59usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 58usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::RegexExpression, 9usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::RegexLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::RegexExpression, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
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
                (SyntaxKind::RegexExpression, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::RegexExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::RegexExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::RegexExpression, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::SubstringExpression, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
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
                (SyntaxKind::SubstringExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::SubstringExpression, 9usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SubstrLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::SubstringExpression, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::SubstringExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::SubstringExpression, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::StrReplaceExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::StrReplaceExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::StrReplaceExpression, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 10usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ReplaceLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::StrReplaceExpression, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::StrReplaceExpression, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::StrReplaceExpression, 10usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 9usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::ExistsFunc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::ExistsFunc, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ExistsLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::NotExistsFunc, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NotLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::NotExistsFunc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NotExistsFunc, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ExistsLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Aggregate, 2usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::CountLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 13usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SumLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 13usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MinLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 13usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MaxLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 13usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AvgLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 13usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SampleLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 28usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupConcatLit,
                                state: 0usize,
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 25usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DistinctLit,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Aggregate, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 6usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 10usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DistinctLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Star,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Aggregate, 20usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 23usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Colon,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::Aggregate, 22usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 21usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Eq,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 28usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 26usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 13usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 11usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 23usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 22usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SeparatorLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 6usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Star,
                                state: 0usize,
                            }),
                    );
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DistinctLit,
                                state: 0usize,
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
                (SyntaxKind::Rdfliteral, 1usize) => {
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
                                kind: SyntaxKind::Langtag,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 4usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Datatype,
                                state: 0usize,
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
                (SyntaxKind::Rdfliteral, 0usize) => {
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
                (SyntaxKind::NumericLiteralUnsigned, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Integer,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Decimal,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Double,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::NumericLiteralUnsigned, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NumericLiteralPositive, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NumericLiteralPositive, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::IntegerPositive,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DecimalPositive,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DoublePositive,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::NumericLiteralNegative, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::IntegerNegative,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DecimalNegative,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DoubleNegative,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::NumericLiteralNegative, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BooleanLiteral, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BooleanLiteral, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TrueLit,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::FalseLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::MyString, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::MyString, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StringLiteral1,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StringLiteral2,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StringLiteralLong1,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::StringLiteralLong2,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Iri, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Iriref,
                                state: 0usize,
                            }),
                    );
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
                (SyntaxKind::Iri, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixedName, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::PrefixedName, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PnameLn,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::PnameNs,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BlankNode, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BlankNodeLabel,
                                state: 0usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Anon,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BlankNode, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ConcatLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ConcatLit, 10isize);
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
                (SyntaxKind::DoubleNegative, _) => {
                    let added = state.expect_as(element, SyntaxKind::DoubleNegative, 2isize);
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
                (SyntaxKind::DeleteWhereLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DeleteWhereLit, 10isize);
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
                (SyntaxKind::MaxLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MaxLit, 10isize);
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
                (SyntaxKind::Datatype, _) => {
                    let added = state.expect_as(element, SyntaxKind::Datatype, 10isize);
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
                (SyntaxKind::Neq, _) => {
                    let added = state.expect_as(element, SyntaxKind::Neq, 10isize);
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
                (SyntaxKind::StringLiteralLong1, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralLong1, 2isize);
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
                (SyntaxKind::Pipe2, _) => {
                    let added = state.expect_as(element, SyntaxKind::Pipe2, 10isize);
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
                (SyntaxKind::MinutesLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MinutesLit, 10isize);
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
                (SyntaxKind::Comma, _) => {
                    let added = state.expect_as(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::Hat, _) => {
                    let added = state.expect_as(element, SyntaxKind::Hat, 10isize);
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
                (SyntaxKind::IntegerPositive, _) => {
                    let added = state.expect_as(element, SyntaxKind::IntegerPositive, 2isize);
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
                (SyntaxKind::CreateLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CreateLit, 10isize);
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
                (SyntaxKind::Gte, _) => {
                    let added = state.expect_as(element, SyntaxKind::Gte, 10isize);
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
                (SyntaxKind::BindLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BindLit, 10isize);
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
                (SyntaxKind::GroupLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::GroupLit, 10isize);
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
                (SyntaxKind::Var2, _) => {
                    let added = state.expect_as(element, SyntaxKind::Var2, 2isize);
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
                (SyntaxKind::Div, _) => {
                    let added = state.expect_as(element, SyntaxKind::Div, 10isize);
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
                (SyntaxKind::Colon, _) => {
                    let added = state.expect_as(element, SyntaxKind::Colon, 10isize);
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
                (SyntaxKind::AvgLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AvgLit, 10isize);
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
                (SyntaxKind::Bang, _) => {
                    let added = state.expect_as(element, SyntaxKind::Bang, 10isize);
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
                (SyntaxKind::ClOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::ClOpen, 8isize);
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
                (SyntaxKind::FilterLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FilterLit, 10isize);
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
                (SyntaxKind::StringLiteral2, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteral2, 2isize);
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
                (SyntaxKind::OffsetLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::OffsetLit, 10isize);
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
                (SyntaxKind::BlankNodeLabel, _) => {
                    let added = state.expect_as(element, SyntaxKind::BlankNodeLabel, 2isize);
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
                (SyntaxKind::Gt, _) => {
                    let added = state.expect_as(element, SyntaxKind::Gt, 10isize);
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
                (SyntaxKind::StrlenLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrlenLit, 10isize);
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
                (SyntaxKind::MoveLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MoveLit, 10isize);
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
                (SyntaxKind::StrbeforeLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrbeforeLit, 10isize);
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
                (SyntaxKind::InsertLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::InsertLit, 10isize);
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
                (SyntaxKind::GroupConcatLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::GroupConcatLit, 10isize);
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
                (SyntaxKind::RoundLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::RoundLit, 10isize);
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
                (SyntaxKind::BnodeLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BnodeLit, 10isize);
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
                (SyntaxKind::NotLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::NotLit, 10isize);
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
                (SyntaxKind::YearLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::YearLit, 10isize);
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
                (SyntaxKind::DecimalPositive, _) => {
                    let added = state.expect_as(element, SyntaxKind::DecimalPositive, 2isize);
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
                (SyntaxKind::InLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::InLit, 10isize);
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
                (SyntaxKind::Stop, _) => {
                    let added = state.expect_as(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::BoundLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BoundLit, 10isize);
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
                (SyntaxKind::LangmatchesLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LangmatchesLit, 10isize);
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
                (SyntaxKind::HavingLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::HavingLit, 10isize);
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
                (SyntaxKind::AddLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AddLit, 10isize);
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
                (SyntaxKind::SameTermLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SameTermLit, 10isize);
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
                (SyntaxKind::BrOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::BrOpen, 8isize);
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
                (SyntaxKind::MinLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MinLit, 10isize);
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
                (SyntaxKind::HoursLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::HoursLit, 10isize);
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
                (SyntaxKind::SqOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::SqOpen, 8isize);
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
                (SyntaxKind::Lt, _) => {
                    let added = state.expect_as(element, SyntaxKind::Lt, 10isize);
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
                (SyntaxKind::StrLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrLit, 10isize);
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
                (SyntaxKind::Sha384Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Sha384Lit, 10isize);
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
                (SyntaxKind::DropLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DropLit, 10isize);
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
                (SyntaxKind::SparqlBaseToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::SparqlBaseToken, 100isize);
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
                (SyntaxKind::StrafterLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrafterLit, 10isize);
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
                (SyntaxKind::PnameNs, _) => {
                    let added = state.expect_as(element, SyntaxKind::PnameNs, 2isize);
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
                (SyntaxKind::IsBlankLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsBlankLit, 10isize);
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
                (SyntaxKind::ContainsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ContainsLit, 10isize);
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
                (SyntaxKind::Lte, _) => {
                    let added = state.expect_as(element, SyntaxKind::Lte, 10isize);
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
                (SyntaxKind::SecondsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SecondsLit, 10isize);
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
                (SyntaxKind::ExistsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ExistsLit, 10isize);
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
                (SyntaxKind::IsIriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsIriLit, 10isize);
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
                (SyntaxKind::WithLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::WithLit, 10isize);
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
                (SyntaxKind::NowLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::NowLit, 10isize);
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
                (SyntaxKind::CountLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CountLit, 10isize);
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
                (SyntaxKind::FalseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FalseLit, 10isize);
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
                (SyntaxKind::Pipe, _) => {
                    let added = state.expect_as(element, SyntaxKind::Pipe, 10isize);
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
                (SyntaxKind::CopyLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CopyLit, 10isize);
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
                (SyntaxKind::UriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UriLit, 10isize);
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
                (SyntaxKind::StrdtLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrdtLit, 10isize);
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
                (SyntaxKind::MinusLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MinusLit, 10isize);
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
                (SyntaxKind::ServiceLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ServiceLit, 10isize);
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
                (SyntaxKind::SelectLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SelectLit, 10isize);
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
                (SyntaxKind::UsingLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UsingLit, 10isize);
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
                (SyntaxKind::Bar, _) => {
                    let added = state.expect_as(element, SyntaxKind::Bar, 10isize);
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
                (SyntaxKind::TzLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::TzLit, 10isize);
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
                (SyntaxKind::UcaseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UcaseLit, 10isize);
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
                (SyntaxKind::ConstructLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ConstructLit, 10isize);
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
                (SyntaxKind::Eq, _) => {
                    let added = state.expect_as(element, SyntaxKind::Eq, 10isize);
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
                (SyntaxKind::AsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AsLit, 10isize);
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
                (SyntaxKind::FloorLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FloorLit, 10isize);
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
                (SyntaxKind::ToLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ToLit, 10isize);
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
}
