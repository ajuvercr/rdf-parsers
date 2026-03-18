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
    #[token("{")]
    ClOpen,
    #[token("TO")]
    ToLit,
    #[token("BIND")]
    BindLit,
    #[token("a")]
    Alit,
    #[token("SELECT")]
    SelectLit,
    #[token("TIMEZONE")]
    TimezoneLit,
    #[token("SILENT")]
    SilentLit,
    #[token("CREATE")]
    CreateLit,
    #[token("COPY")]
    CopyLit,
    #[token("DATATYPE")]
    DatatypeLit,
    #[token("BASE")]
    SparqlBaseToken,
    #[token("USING")]
    UsingLit,
    #[token(">")]
    Gt,
    #[token("RAND")]
    RandLit,
    #[token("MD5")]
    Md5Lit,
    #[token("INSERT")]
    InsertLit,
    #[token("DELETE")]
    DeleteLit,
    #[token("ABS")]
    AbsLit,
    #[token("IRI")]
    IriLit,
    #[token("UCASE")]
    UcaseLit,
    #[token("YEAR")]
    YearLit,
    #[token("FROM")]
    FromLit,
    #[token("NOW")]
    NowLit,
    #[token("[")]
    SqOpen,
    #[token("=")]
    Eq,
    #[token("isIRI")]
    IsIriLit,
    #[token("OFFSET")]
    OffsetLit,
    #[token("CONCAT")]
    ConcatLit,
    #[token("^^")]
    Datatype,
    #[token("SUM")]
    SumLit,
    #[token("MOVE")]
    MoveLit,
    #[token("LOAD")]
    LoadLit,
    #[token("STR")]
    StrLit,
    #[token("-")]
    Bar,
    #[token("DROP")]
    DropLit,
    #[token("/")]
    Div,
    #[token("INTO")]
    IntoLit,
    #[token("ROUND")]
    RoundLit,
    #[token("SHA512")]
    Sha512Lit,
    #[token("CLEAR")]
    ClearLit,
    #[token("?")]
    Questionmark,
    #[token("STRAFTER")]
    StrafterLit,
    #[token("REGEX")]
    RegexLit,
    #[token("true")]
    TrueLit,
    #[token("AVG")]
    AvgLit,
    #[regex("(?&STRING_LITERAL1)")]
    StringLiteral1,
    #[token("LCASE")]
    LcaseLit,
    #[token("false")]
    FalseLit,
    #[token("SUBSTR")]
    SubstrLit,
    #[regex("(?&DOUBLE)")]
    Double,
    #[token("COUNT")]
    CountLit,
    #[regex("(?&PNAME_NS)")]
    PnameNs,
    #[token("FILTER")]
    FilterLit,
    #[token("STRENDS")]
    StrendsLit,
    #[token("STRDT")]
    StrdtLit,
    #[token("ASK")]
    AskLit,
    #[token("HAVING")]
    HavingLit,
    #[token("ASC")]
    AscLit,
    #[token("UNION")]
    UnionLit,
    #[token("SHA1")]
    Sha1Lit,
    #[token("PREFIX")]
    SparqlPrefixToken,
    #[regex("(?&INTEGER_NEGATIVE)")]
    IntegerNegative,
    #[token("DISTINCT")]
    DistinctLit,
    #[token("}")]
    ClClose,
    #[token("CONSTRUCT")]
    ConstructLit,
    #[token("MINUS")]
    MinusLit,
    #[regex("(?&DOUBLE_NEGATIVE)")]
    DoubleNegative,
    #[token("DELETE DATA")]
    DeleteDataLit,
    #[regex("(?&DECIMAL_POSITIVE)")]
    DecimalPositive,
    #[token("SECONDS")]
    SecondsLit,
    #[token("]")]
    SqClose,
    #[token("<")]
    Lt,
    #[token("SEPARATOR")]
    SeparatorLit,
    #[regex("(?&STRING_LITERAL_LONG2)")]
    StringLiteralLong2,
    #[token("REDUCED")]
    ReducedLit,
    #[token("LANGMATCHES")]
    LangmatchesLit,
    #[token("BNODE")]
    BnodeLit,
    #[token("SAMPLE")]
    SampleLit,
    #[token("VALUES")]
    ValuesLit,
    #[token("!=")]
    Neq,
    #[token("REPLACE")]
    ReplaceLit,
    #[regex("(?&DOUBLE_POSITIVE)")]
    DoublePositive,
    #[token("INSERT DATA")]
    InsertDataLit,
    #[token("DEFAULT")]
    DefaultLit,
    #[token("STRLEN")]
    StrlenLit,
    #[token("GRAPH")]
    GraphLit,
    #[token("<=")]
    Lte,
    #[token("OPTIONAL")]
    OptionalLit,
    #[token("NOT")]
    NotLit,
    #[token("BY")]
    ByLit,
    #[token("+")]
    Plus,
    #[token("DELETE WHERE")]
    DeleteWhereLit,
    #[regex("(?&VAR1)")]
    Var1,
    #[token("GROUP_CONCAT")]
    GroupConcatLit,
    #[regex("(?&IRIREF)")]
    Iriref,
    #[regex("(?&PNAME_LN)")]
    PnameLn,
    #[token(")")]
    BrClose,
    #[token("ORDER")]
    OrderLit,
    #[token("IF")]
    IfLit,
    #[regex("(?&LANGTAG)")]
    Langtag,
    #[regex("(?&VAR2)")]
    Var2,
    #[regex("(?&NIL)")]
    Nil,
    #[token("URI")]
    UriLit,
    #[regex("(?&BLANK_NODE_LABEL)")]
    BlankNodeLabel,
    #[token("ENCODE_FOR_URI")]
    EncodeForUriLit,
    #[token("IN")]
    InLit,
    #[token(",")]
    Comma,
    #[token("^")]
    Hat,
    #[token("isLITERAL")]
    IsLiteralLit,
    #[token("STRSTARTS")]
    StrstartsLit,
    #[token("WHERE")]
    WhereLit,
    #[token("SERVICE")]
    ServiceLit,
    #[token("isBLANK")]
    IsBlankLit,
    #[token("TZ")]
    TzLit,
    #[token("FLOOR")]
    FloorLit,
    #[token("BOUND")]
    BoundLit,
    #[token(".")]
    Stop,
    #[token("SHA384")]
    Sha384Lit,
    #[token("MINUTES")]
    MinutesLit,
    #[token("(")]
    BrOpen,
    #[token("LANG")]
    LangLit,
    #[token("STRUUID")]
    StruuidLit,
    #[token(";")]
    Colon,
    #[token("*")]
    Star,
    #[token("!")]
    Bang,
    #[token("AS")]
    AsLit,
    #[token("|")]
    Pipe,
    #[token(">=")]
    Gte,
    #[token("SHA256")]
    Sha256Lit,
    #[token("isURI")]
    IsUriLit,
    #[token("STRBEFORE")]
    StrbeforeLit,
    #[token("MAX")]
    MaxLit,
    #[token("NAMED")]
    NamedLit,
    #[token("DESC")]
    DescLit,
    #[token("CONTAINS")]
    ContainsLit,
    #[token("UUID")]
    UuidLit,
    #[token("||")]
    Pipe2,
    #[token("DESCRIBE")]
    DescribeLit,
    #[token("MIN")]
    MinLit,
    #[regex("(?&STRING_LITERAL2)")]
    StringLiteral2,
    #[token("ALL")]
    AllLit,
    #[token("DAY")]
    DayLit,
    #[token("EXISTS")]
    ExistsLit,
    #[token("COALESCE")]
    CoalesceLit,
    #[token("GROUP")]
    GroupLit,
    #[token("MONTH")]
    MonthLit,
    #[regex("(?&DECIMAL)")]
    Decimal,
    #[token("WITH")]
    WithLit,
    #[token("ADD")]
    AddLit,
    #[regex("(?&INTEGER_POSITIVE)")]
    IntegerPositive,
    #[regex("(?&DECIMAL_NEGATIVE)")]
    DecimalNegative,
    #[token("UNDEF")]
    UndefLit,
    #[regex("(?&STRING_LITERAL_LONG1)")]
    StringLiteralLong1,
    #[regex("(?&INTEGER)")]
    Integer,
    #[token("LIMIT")]
    LimitLit,
    #[token("sameTerm")]
    SameTermLit,
    #[regex("(?&ANON)")]
    Anon,
    #[token("HOURS")]
    HoursLit,
    #[token("isNUMERIC")]
    IsNumericLit,
    #[token("&&")]
    Amp2,
    #[token("CEIL")]
    CeilLit,
    #[token("STRLANG")]
    StrlangLit,
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
                SyntaxKind::ClOpen => Rule { kind, state: 0 },
                SyntaxKind::ToLit => Rule { kind, state: 0 },
                SyntaxKind::BindLit => Rule { kind, state: 0 },
                SyntaxKind::Alit => Rule { kind, state: 0 },
                SyntaxKind::SelectLit => Rule { kind, state: 0 },
                SyntaxKind::TimezoneLit => Rule { kind, state: 0 },
                SyntaxKind::SilentLit => Rule { kind, state: 0 },
                SyntaxKind::CreateLit => Rule { kind, state: 0 },
                SyntaxKind::CopyLit => Rule { kind, state: 0 },
                SyntaxKind::DatatypeLit => Rule { kind, state: 0 },
                SyntaxKind::SparqlBaseToken => Rule { kind, state: 0 },
                SyntaxKind::UsingLit => Rule { kind, state: 0 },
                SyntaxKind::Gt => Rule { kind, state: 0 },
                SyntaxKind::RandLit => Rule { kind, state: 0 },
                SyntaxKind::Md5Lit => Rule { kind, state: 0 },
                SyntaxKind::InsertLit => Rule { kind, state: 0 },
                SyntaxKind::DeleteLit => Rule { kind, state: 0 },
                SyntaxKind::AbsLit => Rule { kind, state: 0 },
                SyntaxKind::IriLit => Rule { kind, state: 0 },
                SyntaxKind::UcaseLit => Rule { kind, state: 0 },
                SyntaxKind::YearLit => Rule { kind, state: 0 },
                SyntaxKind::FromLit => Rule { kind, state: 0 },
                SyntaxKind::NowLit => Rule { kind, state: 0 },
                SyntaxKind::SqOpen => Rule { kind, state: 0 },
                SyntaxKind::Eq => Rule { kind, state: 0 },
                SyntaxKind::IsIriLit => Rule { kind, state: 0 },
                SyntaxKind::OffsetLit => Rule { kind, state: 0 },
                SyntaxKind::ConcatLit => Rule { kind, state: 0 },
                SyntaxKind::Datatype => Rule { kind, state: 0 },
                SyntaxKind::SumLit => Rule { kind, state: 0 },
                SyntaxKind::MoveLit => Rule { kind, state: 0 },
                SyntaxKind::LoadLit => Rule { kind, state: 0 },
                SyntaxKind::StrLit => Rule { kind, state: 0 },
                SyntaxKind::Bar => Rule { kind, state: 0 },
                SyntaxKind::DropLit => Rule { kind, state: 0 },
                SyntaxKind::Div => Rule { kind, state: 0 },
                SyntaxKind::IntoLit => Rule { kind, state: 0 },
                SyntaxKind::RoundLit => Rule { kind, state: 0 },
                SyntaxKind::Sha512Lit => Rule { kind, state: 0 },
                SyntaxKind::ClearLit => Rule { kind, state: 0 },
                SyntaxKind::Questionmark => Rule { kind, state: 0 },
                SyntaxKind::StrafterLit => Rule { kind, state: 0 },
                SyntaxKind::RegexLit => Rule { kind, state: 0 },
                SyntaxKind::TrueLit => Rule { kind, state: 0 },
                SyntaxKind::AvgLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteral1 => Rule { kind, state: 0 },
                SyntaxKind::LcaseLit => Rule { kind, state: 0 },
                SyntaxKind::FalseLit => Rule { kind, state: 0 },
                SyntaxKind::SubstrLit => Rule { kind, state: 0 },
                SyntaxKind::Double => Rule { kind, state: 0 },
                SyntaxKind::CountLit => Rule { kind, state: 0 },
                SyntaxKind::PnameNs => Rule { kind, state: 0 },
                SyntaxKind::FilterLit => Rule { kind, state: 0 },
                SyntaxKind::StrendsLit => Rule { kind, state: 0 },
                SyntaxKind::StrdtLit => Rule { kind, state: 0 },
                SyntaxKind::AskLit => Rule { kind, state: 0 },
                SyntaxKind::HavingLit => Rule { kind, state: 0 },
                SyntaxKind::AscLit => Rule { kind, state: 0 },
                SyntaxKind::UnionLit => Rule { kind, state: 0 },
                SyntaxKind::Sha1Lit => Rule { kind, state: 0 },
                SyntaxKind::SparqlPrefixToken => Rule { kind, state: 0 },
                SyntaxKind::IntegerNegative => Rule { kind, state: 0 },
                SyntaxKind::DistinctLit => Rule { kind, state: 0 },
                SyntaxKind::ClClose => Rule { kind, state: 0 },
                SyntaxKind::ConstructLit => Rule { kind, state: 0 },
                SyntaxKind::MinusLit => Rule { kind, state: 0 },
                SyntaxKind::DoubleNegative => Rule { kind, state: 0 },
                SyntaxKind::DeleteDataLit => Rule { kind, state: 0 },
                SyntaxKind::DecimalPositive => Rule { kind, state: 0 },
                SyntaxKind::SecondsLit => Rule { kind, state: 0 },
                SyntaxKind::SqClose => Rule { kind, state: 0 },
                SyntaxKind::Lt => Rule { kind, state: 0 },
                SyntaxKind::SeparatorLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLong2 => Rule { kind, state: 0 },
                SyntaxKind::ReducedLit => Rule { kind, state: 0 },
                SyntaxKind::LangmatchesLit => Rule { kind, state: 0 },
                SyntaxKind::BnodeLit => Rule { kind, state: 0 },
                SyntaxKind::SampleLit => Rule { kind, state: 0 },
                SyntaxKind::ValuesLit => Rule { kind, state: 0 },
                SyntaxKind::Neq => Rule { kind, state: 0 },
                SyntaxKind::ReplaceLit => Rule { kind, state: 0 },
                SyntaxKind::DoublePositive => Rule { kind, state: 0 },
                SyntaxKind::InsertDataLit => Rule { kind, state: 0 },
                SyntaxKind::DefaultLit => Rule { kind, state: 0 },
                SyntaxKind::StrlenLit => Rule { kind, state: 0 },
                SyntaxKind::GraphLit => Rule { kind, state: 0 },
                SyntaxKind::Lte => Rule { kind, state: 0 },
                SyntaxKind::OptionalLit => Rule { kind, state: 0 },
                SyntaxKind::NotLit => Rule { kind, state: 0 },
                SyntaxKind::ByLit => Rule { kind, state: 0 },
                SyntaxKind::Plus => Rule { kind, state: 0 },
                SyntaxKind::DeleteWhereLit => Rule { kind, state: 0 },
                SyntaxKind::Var1 => Rule { kind, state: 0 },
                SyntaxKind::GroupConcatLit => Rule { kind, state: 0 },
                SyntaxKind::Iriref => Rule { kind, state: 0 },
                SyntaxKind::PnameLn => Rule { kind, state: 0 },
                SyntaxKind::BrClose => Rule { kind, state: 0 },
                SyntaxKind::OrderLit => Rule { kind, state: 0 },
                SyntaxKind::IfLit => Rule { kind, state: 0 },
                SyntaxKind::Langtag => Rule { kind, state: 0 },
                SyntaxKind::Var2 => Rule { kind, state: 0 },
                SyntaxKind::Nil => Rule { kind, state: 0 },
                SyntaxKind::UriLit => Rule { kind, state: 0 },
                SyntaxKind::BlankNodeLabel => Rule { kind, state: 0 },
                SyntaxKind::EncodeForUriLit => Rule { kind, state: 0 },
                SyntaxKind::InLit => Rule { kind, state: 0 },
                SyntaxKind::Comma => Rule { kind, state: 0 },
                SyntaxKind::Hat => Rule { kind, state: 0 },
                SyntaxKind::IsLiteralLit => Rule { kind, state: 0 },
                SyntaxKind::StrstartsLit => Rule { kind, state: 0 },
                SyntaxKind::WhereLit => Rule { kind, state: 0 },
                SyntaxKind::ServiceLit => Rule { kind, state: 0 },
                SyntaxKind::IsBlankLit => Rule { kind, state: 0 },
                SyntaxKind::TzLit => Rule { kind, state: 0 },
                SyntaxKind::FloorLit => Rule { kind, state: 0 },
                SyntaxKind::BoundLit => Rule { kind, state: 0 },
                SyntaxKind::Stop => Rule { kind, state: 0 },
                SyntaxKind::Sha384Lit => Rule { kind, state: 0 },
                SyntaxKind::MinutesLit => Rule { kind, state: 0 },
                SyntaxKind::BrOpen => Rule { kind, state: 0 },
                SyntaxKind::LangLit => Rule { kind, state: 0 },
                SyntaxKind::StruuidLit => Rule { kind, state: 0 },
                SyntaxKind::Colon => Rule { kind, state: 0 },
                SyntaxKind::Star => Rule { kind, state: 0 },
                SyntaxKind::Bang => Rule { kind, state: 0 },
                SyntaxKind::AsLit => Rule { kind, state: 0 },
                SyntaxKind::Pipe => Rule { kind, state: 0 },
                SyntaxKind::Gte => Rule { kind, state: 0 },
                SyntaxKind::Sha256Lit => Rule { kind, state: 0 },
                SyntaxKind::IsUriLit => Rule { kind, state: 0 },
                SyntaxKind::StrbeforeLit => Rule { kind, state: 0 },
                SyntaxKind::MaxLit => Rule { kind, state: 0 },
                SyntaxKind::NamedLit => Rule { kind, state: 0 },
                SyntaxKind::DescLit => Rule { kind, state: 0 },
                SyntaxKind::ContainsLit => Rule { kind, state: 0 },
                SyntaxKind::UuidLit => Rule { kind, state: 0 },
                SyntaxKind::Pipe2 => Rule { kind, state: 0 },
                SyntaxKind::DescribeLit => Rule { kind, state: 0 },
                SyntaxKind::MinLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteral2 => Rule { kind, state: 0 },
                SyntaxKind::AllLit => Rule { kind, state: 0 },
                SyntaxKind::DayLit => Rule { kind, state: 0 },
                SyntaxKind::ExistsLit => Rule { kind, state: 0 },
                SyntaxKind::CoalesceLit => Rule { kind, state: 0 },
                SyntaxKind::GroupLit => Rule { kind, state: 0 },
                SyntaxKind::MonthLit => Rule { kind, state: 0 },
                SyntaxKind::Decimal => Rule { kind, state: 0 },
                SyntaxKind::WithLit => Rule { kind, state: 0 },
                SyntaxKind::AddLit => Rule { kind, state: 0 },
                SyntaxKind::IntegerPositive => Rule { kind, state: 0 },
                SyntaxKind::DecimalNegative => Rule { kind, state: 0 },
                SyntaxKind::UndefLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLong1 => Rule { kind, state: 0 },
                SyntaxKind::Integer => Rule { kind, state: 0 },
                SyntaxKind::LimitLit => Rule { kind, state: 0 },
                SyntaxKind::SameTermLit => Rule { kind, state: 0 },
                SyntaxKind::Anon => Rule { kind, state: 0 },
                SyntaxKind::HoursLit => Rule { kind, state: 0 },
                SyntaxKind::IsNumericLit => Rule { kind, state: 0 },
                SyntaxKind::Amp2 => Rule { kind, state: 0 },
                SyntaxKind::CeilLit => Rule { kind, state: 0 },
                SyntaxKind::StrlangLit => Rule { kind, state: 0 },
                _ => panic!("Unknown rule kind {:?}", kind),
            }
        }
    }
    pub fn first_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::Query => &[
                SyntaxKind::AskLit,
                SyntaxKind::ConstructLit,
                SyntaxKind::SelectLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::DescribeLit,
            ],
            SyntaxKind::ExistsFunc => &[SyntaxKind::ExistsLit],
            SyntaxKind::Move => &[SyntaxKind::MoveLit],
            SyntaxKind::StrReplaceExpression => &[SyntaxKind::ReplaceLit],
            SyntaxKind::FunctionCall => &[],
            SyntaxKind::BooleanLiteral => &[SyntaxKind::TrueLit, SyntaxKind::FalseLit],
            SyntaxKind::QueryUnit => &[
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::DescribeLit,
                SyntaxKind::AskLit,
                SyntaxKind::SelectLit,
                SyntaxKind::ConstructLit,
                SyntaxKind::SparqlPrefixToken,
            ],
            SyntaxKind::PropertyListNotEmpty => &[SyntaxKind::Alit],
            SyntaxKind::Load => &[SyntaxKind::LoadLit],
            SyntaxKind::Constraint => &[
                SyntaxKind::SampleLit,
                SyntaxKind::MonthLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::RoundLit,
                SyntaxKind::HoursLit,
                SyntaxKind::IriLit,
                SyntaxKind::AbsLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::FloorLit,
                SyntaxKind::CeilLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::BoundLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::MinLit,
                SyntaxKind::StrLit,
                SyntaxKind::NotLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::CountLit,
                SyntaxKind::LangLit,
                SyntaxKind::AvgLit,
                SyntaxKind::YearLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::NowLit,
                SyntaxKind::DayLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::UriLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::SameTermLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::MaxLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::SumLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::RegexLit,
                SyntaxKind::IfLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::RandLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::BrOpen,
                SyntaxKind::UuidLit,
                SyntaxKind::TzLit,
            ],
            SyntaxKind::RegexExpression => &[SyntaxKind::RegexLit],
            SyntaxKind::NumericLiteral => &[],
            SyntaxKind::PropertyListPathNotEmpty => &[
                SyntaxKind::Bang,
                SyntaxKind::Alit,
                SyntaxKind::BrOpen,
                SyntaxKind::Hat,
            ],
            SyntaxKind::WhereClause => &[SyntaxKind::ClOpen, SyntaxKind::WhereLit],
            SyntaxKind::UsingClause => &[SyntaxKind::UsingLit],
            SyntaxKind::OrderClause => &[SyntaxKind::OrderLit],
            SyntaxKind::ValuesClause => &[SyntaxKind::ValuesLit],
            SyntaxKind::BrackettedExpression => &[SyntaxKind::BrOpen],
            SyntaxKind::GraphRef => &[SyntaxKind::GraphLit],
            SyntaxKind::Rdfliteral => &[],
            SyntaxKind::TriplesBlock => &[
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::TriplesSameSubjectPath => &[
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::ArgList => &[SyntaxKind::BrOpen],
            SyntaxKind::DataBlock => &[SyntaxKind::BrOpen],
            SyntaxKind::MultiplicativeExpression => &[
                SyntaxKind::NotLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::CeilLit,
                SyntaxKind::RoundLit,
                SyntaxKind::IfLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::NowLit,
                SyntaxKind::MonthLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::DayLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::UriLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::Bar,
                SyntaxKind::SameTermLit,
                SyntaxKind::YearLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::MaxLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::LangLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::BnodeLit,
                SyntaxKind::FloorLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::UuidLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::IriLit,
                SyntaxKind::AvgLit,
                SyntaxKind::SumLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::RegexLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::TzLit,
                SyntaxKind::RandLit,
                SyntaxKind::HoursLit,
                SyntaxKind::AbsLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::BoundLit,
                SyntaxKind::BrOpen,
                SyntaxKind::Md5Lit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::IsUriLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::Plus,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::MinLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::SampleLit,
                SyntaxKind::FalseLit,
                SyntaxKind::Bang,
                SyntaxKind::MinutesLit,
                SyntaxKind::CountLit,
            ],
            SyntaxKind::Drop => &[SyntaxKind::DropLit],
            SyntaxKind::BuiltInCall => &[
                SyntaxKind::Sha384Lit,
                SyntaxKind::MinutesLit,
                SyntaxKind::IfLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::MonthLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::LangLit,
                SyntaxKind::TzLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::StrLit,
                SyntaxKind::UriLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::CountLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::DayLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::SampleLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::RegexLit,
                SyntaxKind::UuidLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::MaxLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::NotLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::YearLit,
                SyntaxKind::BoundLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::RandLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::AbsLit,
                SyntaxKind::NowLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::ConcatLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::StrdtLit,
                SyntaxKind::FloorLit,
                SyntaxKind::HoursLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::SumLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::AvgLit,
                SyntaxKind::IriLit,
                SyntaxKind::CeilLit,
                SyntaxKind::MinLit,
            ],
            SyntaxKind::TriplesTemplate => &[
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::SolutionModifier => &[
                SyntaxKind::GroupLit,
                SyntaxKind::OrderLit,
                SyntaxKind::LimitLit,
                SyntaxKind::OffsetLit,
                SyntaxKind::HavingLit,
            ],
            SyntaxKind::CollectionPath => &[SyntaxKind::BrOpen],
            SyntaxKind::GraphTerm => &[SyntaxKind::TrueLit, SyntaxKind::FalseLit],
            SyntaxKind::Expression => &[
                SyntaxKind::MaxLit,
                SyntaxKind::IfLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::TzLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::StrLit,
                SyntaxKind::NowLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::CeilLit,
                SyntaxKind::IriLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::CountLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::Bang,
                SyntaxKind::SumLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::BrOpen,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::RandLit,
                SyntaxKind::YearLit,
                SyntaxKind::FloorLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::AbsLit,
                SyntaxKind::NotLit,
                SyntaxKind::MonthLit,
                SyntaxKind::Bar,
                SyntaxKind::BoundLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::FalseLit,
                SyntaxKind::RoundLit,
                SyntaxKind::Plus,
                SyntaxKind::RegexLit,
                SyntaxKind::HoursLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::ConcatLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::AvgLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::UriLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::LangLit,
                SyntaxKind::DayLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::SampleLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::UuidLit,
                SyntaxKind::MinLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::IsNumericLit,
            ],
            SyntaxKind::ExpressionList => &[SyntaxKind::BrOpen],
            SyntaxKind::PathElt => &[SyntaxKind::BrOpen, SyntaxKind::Alit, SyntaxKind::Bang],
            SyntaxKind::RelationalExpression => &[
                SyntaxKind::Sha384Lit,
                SyntaxKind::StrafterLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::YearLit,
                SyntaxKind::LangLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::SumLit,
                SyntaxKind::CeilLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::RandLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::SampleLit,
                SyntaxKind::FloorLit,
                SyntaxKind::FalseLit,
                SyntaxKind::HoursLit,
                SyntaxKind::MinLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::TrueLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::CountLit,
                SyntaxKind::DayLit,
                SyntaxKind::Bar,
                SyntaxKind::StrendsLit,
                SyntaxKind::NowLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::RoundLit,
                SyntaxKind::RegexLit,
                SyntaxKind::BrOpen,
                SyntaxKind::TimezoneLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::UriLit,
                SyntaxKind::Bang,
                SyntaxKind::StrstartsLit,
                SyntaxKind::TzLit,
                SyntaxKind::IriLit,
                SyntaxKind::BoundLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::MaxLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::AbsLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::StrlenLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::NotLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::IfLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::UuidLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::AvgLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::Plus,
                SyntaxKind::MonthLit,
            ],
            SyntaxKind::Clear => &[SyntaxKind::ClearLit],
            SyntaxKind::Iri => &[],
            SyntaxKind::Modify => &[
                SyntaxKind::WithLit,
                SyntaxKind::InsertLit,
                SyntaxKind::DeleteLit,
            ],
            SyntaxKind::Copy => &[SyntaxKind::CopyLit],
            SyntaxKind::GraphNode => &[
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::NumericLiteralUnsigned => &[],
            SyntaxKind::AdditiveExpression => &[
                SyntaxKind::StruuidLit,
                SyntaxKind::UuidLit,
                SyntaxKind::Bang,
                SyntaxKind::YearLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::MonthLit,
                SyntaxKind::Bar,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::HoursLit,
                SyntaxKind::MinLit,
                SyntaxKind::RoundLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::SecondsLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::StrendsLit,
                SyntaxKind::NowLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::AvgLit,
                SyntaxKind::NotLit,
                SyntaxKind::RegexLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::FalseLit,
                SyntaxKind::AbsLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::BrOpen,
                SyntaxKind::UriLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::Plus,
                SyntaxKind::Sha1Lit,
                SyntaxKind::SameTermLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::StrLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::FloorLit,
                SyntaxKind::IfLit,
                SyntaxKind::SampleLit,
                SyntaxKind::DayLit,
                SyntaxKind::MaxLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::TrueLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::TzLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::IriLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::CountLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::SumLit,
                SyntaxKind::LangLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::RandLit,
                SyntaxKind::CeilLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::ExistsLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StrdtLit,
            ],
            SyntaxKind::DatasetClause => &[SyntaxKind::FromLit],
            SyntaxKind::BlankNodePropertyListPath => &[SyntaxKind::SqOpen],
            SyntaxKind::PathEltOrInverse => &[
                SyntaxKind::BrOpen,
                SyntaxKind::Hat,
                SyntaxKind::Bang,
                SyntaxKind::Alit,
            ],
            SyntaxKind::HavingClause => &[SyntaxKind::HavingLit],
            SyntaxKind::OffsetClause => &[SyntaxKind::OffsetLit],
            SyntaxKind::InlineDataOneVar => &[],
            SyntaxKind::MyString => &[],
            SyntaxKind::Bind => &[SyntaxKind::BindLit],
            SyntaxKind::VerbPath => &[
                SyntaxKind::Alit,
                SyntaxKind::Hat,
                SyntaxKind::BrOpen,
                SyntaxKind::Bang,
            ],
            SyntaxKind::ConditionalOrExpression => &[
                SyntaxKind::NotLit,
                SyntaxKind::CeilLit,
                SyntaxKind::AvgLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::FalseLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::Plus,
                SyntaxKind::StrendsLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::UriLit,
                SyntaxKind::IriLit,
                SyntaxKind::YearLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::TzLit,
                SyntaxKind::RoundLit,
                SyntaxKind::CountLit,
                SyntaxKind::RandLit,
                SyntaxKind::HoursLit,
                SyntaxKind::DayLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::TrueLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::SumLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::RegexLit,
                SyntaxKind::MinLit,
                SyntaxKind::MaxLit,
                SyntaxKind::LangLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::NowLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::Bang,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::Bar,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::IfLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::SampleLit,
                SyntaxKind::UuidLit,
                SyntaxKind::AbsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::StrLit,
                SyntaxKind::SecondsLit,
            ],
            SyntaxKind::PathSequence => &[
                SyntaxKind::Bang,
                SyntaxKind::Hat,
                SyntaxKind::BrOpen,
                SyntaxKind::Alit,
            ],
            SyntaxKind::LimitOffsetClauses => &[SyntaxKind::LimitLit, SyntaxKind::OffsetLit],
            SyntaxKind::NotExistsFunc => &[SyntaxKind::NotLit],
            SyntaxKind::ObjectPath => &[
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::IriOrFunction => &[],
            SyntaxKind::BaseDecl => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::DeleteWhere => &[SyntaxKind::DeleteWhereLit],
            SyntaxKind::QuadPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::SelectQuery => &[SyntaxKind::SelectLit],
            SyntaxKind::SourceSelector => &[],
            SyntaxKind::GroupClause => &[SyntaxKind::GroupLit],
            SyntaxKind::DataBlockValue => &[
                SyntaxKind::TrueLit,
                SyntaxKind::UndefLit,
                SyntaxKind::FalseLit,
            ],
            SyntaxKind::VerbSimple => &[],
            SyntaxKind::GraphNodePath => &[
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::UpdateUnit => &[
                SyntaxKind::InsertDataLit,
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::DeleteLit,
                SyntaxKind::InsertLit,
                SyntaxKind::DeleteDataLit,
                SyntaxKind::ClearLit,
                SyntaxKind::LoadLit,
                SyntaxKind::CopyLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::DropLit,
                SyntaxKind::CreateLit,
                SyntaxKind::AddLit,
                SyntaxKind::MoveLit,
                SyntaxKind::WithLit,
            ],
            SyntaxKind::HavingCondition => &[
                SyntaxKind::MinLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::CeilLit,
                SyntaxKind::RoundLit,
                SyntaxKind::RegexLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::AbsLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::RandLit,
                SyntaxKind::CountLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::LangLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::SampleLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::YearLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::IfLit,
                SyntaxKind::SumLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::MaxLit,
                SyntaxKind::BoundLit,
                SyntaxKind::HoursLit,
                SyntaxKind::StrLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::NowLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::UuidLit,
                SyntaxKind::NotLit,
                SyntaxKind::MonthLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::UriLit,
                SyntaxKind::AvgLit,
                SyntaxKind::TzLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::StrlangLit,
                SyntaxKind::BrOpen,
                SyntaxKind::DayLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::IriLit,
            ],
            SyntaxKind::PrefixedName => &[],
            SyntaxKind::ObjectListPath => &[
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::ValueLogical => &[
                SyntaxKind::Bang,
                SyntaxKind::IriLit,
                SyntaxKind::FloorLit,
                SyntaxKind::NowLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::SecondsLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::SampleLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::TzLit,
                SyntaxKind::CeilLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::YearLit,
                SyntaxKind::SumLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::UriLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::IfLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::DayLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::FalseLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::AbsLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::UuidLit,
                SyntaxKind::TrueLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::NotLit,
                SyntaxKind::StrLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::MaxLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::HoursLit,
                SyntaxKind::AvgLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::Plus,
                SyntaxKind::Md5Lit,
                SyntaxKind::MonthLit,
                SyntaxKind::RandLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::LangLit,
                SyntaxKind::BoundLit,
                SyntaxKind::MinLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::Bar,
                SyntaxKind::BrOpen,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::CountLit,
                SyntaxKind::RegexLit,
            ],
            SyntaxKind::GroupOrUnionGraphPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::NumericLiteralNegative => &[],
            SyntaxKind::Object => &[
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::Prologue => &[SyntaxKind::SparqlBaseToken, SyntaxKind::SparqlPrefixToken],
            SyntaxKind::PrimaryExpression => &[
                SyntaxKind::StrLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::CountLit,
                SyntaxKind::TzLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::IfLit,
                SyntaxKind::LangLit,
                SyntaxKind::DayLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::UriLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::UuidLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::ConcatLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::BoundLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::BrOpen,
                SyntaxKind::StruuidLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::SampleLit,
                SyntaxKind::RandLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::FalseLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::HoursLit,
                SyntaxKind::IriLit,
                SyntaxKind::RoundLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::MaxLit,
                SyntaxKind::RegexLit,
                SyntaxKind::FloorLit,
                SyntaxKind::AbsLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::SumLit,
                SyntaxKind::NotLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::MonthLit,
                SyntaxKind::AvgLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::YearLit,
                SyntaxKind::CeilLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::NowLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::MinLit,
            ],
            SyntaxKind::MinusGraphPattern => &[SyntaxKind::MinusLit],
            SyntaxKind::Update => &[
                SyntaxKind::ClearLit,
                SyntaxKind::AddLit,
                SyntaxKind::InsertDataLit,
                SyntaxKind::DeleteDataLit,
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::DropLit,
                SyntaxKind::DeleteLit,
                SyntaxKind::LoadLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::CopyLit,
                SyntaxKind::InsertLit,
                SyntaxKind::CreateLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::WithLit,
                SyntaxKind::MoveLit,
            ],
            SyntaxKind::OptionalGraphPattern => &[SyntaxKind::OptionalLit],
            SyntaxKind::ConstructTriples => &[
                SyntaxKind::SqOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::NumericExpression => &[
                SyntaxKind::Sha512Lit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::NotLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::CountLit,
                SyntaxKind::BrOpen,
                SyntaxKind::StruuidLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::FloorLit,
                SyntaxKind::MinLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::MonthLit,
                SyntaxKind::SampleLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::Bar,
                SyntaxKind::StrdtLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::RegexLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::AvgLit,
                SyntaxKind::IriLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::UuidLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::UriLit,
                SyntaxKind::StrLit,
                SyntaxKind::Bang,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::HoursLit,
                SyntaxKind::Plus,
                SyntaxKind::DayLit,
                SyntaxKind::TrueLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::AbsLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::TzLit,
                SyntaxKind::SumLit,
                SyntaxKind::FalseLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::NowLit,
                SyntaxKind::YearLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::LangLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::CeilLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::IfLit,
                SyntaxKind::BoundLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::MaxLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::RandLit,
            ],
            SyntaxKind::Collection => &[SyntaxKind::BrOpen],
            SyntaxKind::Aggregate => &[
                SyntaxKind::SumLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::MaxLit,
                SyntaxKind::MinLit,
                SyntaxKind::AvgLit,
                SyntaxKind::CountLit,
                SyntaxKind::SampleLit,
            ],
            SyntaxKind::PrefixDecl => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::InsertClause => &[SyntaxKind::InsertLit],
            SyntaxKind::SelectClause => &[SyntaxKind::SelectLit],
            SyntaxKind::PropertyList => &[SyntaxKind::Alit],
            SyntaxKind::ObjectList => &[
                SyntaxKind::BrOpen,
                SyntaxKind::SqOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::SubstringExpression => &[SyntaxKind::SubstrLit],
            SyntaxKind::QuadData => &[SyntaxKind::ClOpen],
            SyntaxKind::BlankNodePropertyList => &[SyntaxKind::SqOpen],
            SyntaxKind::PathAlternative => &[
                SyntaxKind::Bang,
                SyntaxKind::BrOpen,
                SyntaxKind::Alit,
                SyntaxKind::Hat,
            ],
            SyntaxKind::ConstructQuery => &[SyntaxKind::ConstructLit],
            SyntaxKind::LimitClause => &[SyntaxKind::LimitLit],
            SyntaxKind::OrderCondition => &[
                SyntaxKind::BnodeLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::TzLit,
                SyntaxKind::RoundLit,
                SyntaxKind::AvgLit,
                SyntaxKind::UuidLit,
                SyntaxKind::UriLit,
                SyntaxKind::StrLit,
                SyntaxKind::SampleLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::FloorLit,
                SyntaxKind::IfLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::RegexLit,
                SyntaxKind::YearLit,
                SyntaxKind::HoursLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::NowLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::LangLit,
                SyntaxKind::MaxLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::SumLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::NotLit,
                SyntaxKind::BoundLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::AscLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::StrendsLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::CeilLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::CountLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::MonthLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::AbsLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::DescLit,
                SyntaxKind::DayLit,
                SyntaxKind::MinLit,
                SyntaxKind::IriLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::BrOpen,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::RandLit,
            ],
            SyntaxKind::AskQuery => &[SyntaxKind::AskLit],
            SyntaxKind::GraphOrDefault => &[SyntaxKind::GraphLit, SyntaxKind::DefaultLit],
            SyntaxKind::Quads => &[
                SyntaxKind::GraphLit,
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::Update1 => &[
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::AddLit,
                SyntaxKind::DeleteDataLit,
                SyntaxKind::InsertDataLit,
                SyntaxKind::DeleteLit,
                SyntaxKind::ClearLit,
                SyntaxKind::CreateLit,
                SyntaxKind::LoadLit,
                SyntaxKind::CopyLit,
                SyntaxKind::WithLit,
                SyntaxKind::DropLit,
                SyntaxKind::MoveLit,
                SyntaxKind::InsertLit,
            ],
            SyntaxKind::GraphGraphPattern => &[SyntaxKind::GraphLit],
            SyntaxKind::InlineData => &[SyntaxKind::ValuesLit],
            SyntaxKind::VarOrIri => &[],
            SyntaxKind::NamedGraphClause => &[SyntaxKind::NamedLit],
            SyntaxKind::DescribeQuery => &[SyntaxKind::DescribeLit],
            SyntaxKind::GraphRefAll => &[
                SyntaxKind::NamedLit,
                SyntaxKind::GraphLit,
                SyntaxKind::AllLit,
                SyntaxKind::DefaultLit,
            ],
            SyntaxKind::VarOrTerm => &[SyntaxKind::TrueLit, SyntaxKind::FalseLit],
            SyntaxKind::Path => &[
                SyntaxKind::Alit,
                SyntaxKind::Hat,
                SyntaxKind::BrOpen,
                SyntaxKind::Bang,
            ],
            SyntaxKind::PropertyListPath => &[
                SyntaxKind::Alit,
                SyntaxKind::Hat,
                SyntaxKind::BrOpen,
                SyntaxKind::Bang,
            ],
            SyntaxKind::PathMod => &[SyntaxKind::Star, SyntaxKind::Plus, SyntaxKind::Questionmark],
            SyntaxKind::TriplesNodePath => &[SyntaxKind::BrOpen, SyntaxKind::SqOpen],
            SyntaxKind::UnaryExpression => &[
                SyntaxKind::MinutesLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::AbsLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::NotLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::Bar,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::UuidLit,
                SyntaxKind::AvgLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::BoundLit,
                SyntaxKind::UriLit,
                SyntaxKind::YearLit,
                SyntaxKind::StrLit,
                SyntaxKind::SumLit,
                SyntaxKind::LangLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::DayLit,
                SyntaxKind::HoursLit,
                SyntaxKind::Plus,
                SyntaxKind::FalseLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::TzLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::MonthLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::StrlangLit,
                SyntaxKind::NowLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::MinLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::RoundLit,
                SyntaxKind::TrueLit,
                SyntaxKind::IfLit,
                SyntaxKind::MaxLit,
                SyntaxKind::IriLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::StrendsLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::FloorLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::RegexLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::CeilLit,
                SyntaxKind::Bang,
                SyntaxKind::SampleLit,
                SyntaxKind::CountLit,
                SyntaxKind::RandLit,
                SyntaxKind::DatatypeLit,
            ],
            SyntaxKind::GroupGraphPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::InsertData => &[SyntaxKind::InsertDataLit],
            SyntaxKind::PathOneInPropertySet => &[SyntaxKind::Alit, SyntaxKind::Hat],
            SyntaxKind::TriplesNode => &[SyntaxKind::BrOpen, SyntaxKind::SqOpen],
            SyntaxKind::Var => &[],
            SyntaxKind::ConditionalAndExpression => &[
                SyntaxKind::IsLiteralLit,
                SyntaxKind::Plus,
                SyntaxKind::MonthLit,
                SyntaxKind::DayLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::Bang,
                SyntaxKind::CoalesceLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::HoursLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::FalseLit,
                SyntaxKind::CeilLit,
                SyntaxKind::AbsLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::Bar,
                SyntaxKind::StrstartsLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::NowLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::BrOpen,
                SyntaxKind::StrlenLit,
                SyntaxKind::FloorLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::MinLit,
                SyntaxKind::SampleLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::TzLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::IfLit,
                SyntaxKind::UriLit,
                SyntaxKind::BoundLit,
                SyntaxKind::RoundLit,
                SyntaxKind::RegexLit,
                SyntaxKind::UuidLit,
                SyntaxKind::CountLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::SumLit,
                SyntaxKind::NotLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::RandLit,
                SyntaxKind::MaxLit,
                SyntaxKind::AvgLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::StrLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::LangLit,
                SyntaxKind::IriLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::YearLit,
            ],
            SyntaxKind::Create => &[SyntaxKind::CreateLit],
            SyntaxKind::TriplesSameSubject => &[
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
            ],
            SyntaxKind::GroupGraphPatternSub => &[
                SyntaxKind::BrOpen,
                SyntaxKind::OptionalLit,
                SyntaxKind::ClOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::BindLit,
                SyntaxKind::FalseLit,
                SyntaxKind::ServiceLit,
                SyntaxKind::ValuesLit,
                SyntaxKind::SqOpen,
                SyntaxKind::GraphLit,
                SyntaxKind::MinusLit,
                SyntaxKind::FilterLit,
            ],
            SyntaxKind::ConstructTemplate => &[SyntaxKind::ClOpen],
            SyntaxKind::PathPrimary => &[SyntaxKind::Alit, SyntaxKind::Bang, SyntaxKind::BrOpen],
            SyntaxKind::PathNegatedPropertySet => {
                &[SyntaxKind::Alit, SyntaxKind::BrOpen, SyntaxKind::Hat]
            }
            SyntaxKind::GroupCondition => &[
                SyntaxKind::CountLit,
                SyntaxKind::BoundLit,
                SyntaxKind::MaxLit,
                SyntaxKind::IfLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::AbsLit,
                SyntaxKind::DayLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StrLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::UuidLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::FloorLit,
                SyntaxKind::MinLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::HoursLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::SumLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::UriLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::RegexLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::NotLit,
                SyntaxKind::MonthLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::IriLit,
                SyntaxKind::CeilLit,
                SyntaxKind::BrOpen,
                SyntaxKind::RandLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::StrafterLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::YearLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::NowLit,
                SyntaxKind::SampleLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::LangLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::AvgLit,
                SyntaxKind::TzLit,
                SyntaxKind::SubstrLit,
            ],
            SyntaxKind::InlineDataFull => &[SyntaxKind::BrOpen],
            SyntaxKind::NumericLiteralPositive => &[],
            SyntaxKind::SubSelect => &[SyntaxKind::SelectLit],
            SyntaxKind::Add => &[SyntaxKind::AddLit],
            SyntaxKind::DefaultGraphClause => &[],
            SyntaxKind::DeleteClause => &[SyntaxKind::DeleteLit],
            SyntaxKind::GraphPatternNotTriples => &[
                SyntaxKind::GraphLit,
                SyntaxKind::MinusLit,
                SyntaxKind::ValuesLit,
                SyntaxKind::ClOpen,
                SyntaxKind::OptionalLit,
                SyntaxKind::FilterLit,
                SyntaxKind::BindLit,
                SyntaxKind::ServiceLit,
            ],
            SyntaxKind::BlankNode => &[],
            SyntaxKind::QuadsNotTriples => &[SyntaxKind::GraphLit],
            SyntaxKind::DeleteData => &[SyntaxKind::DeleteDataLit],
            SyntaxKind::Verb => &[SyntaxKind::Alit],
            SyntaxKind::Filter => &[SyntaxKind::FilterLit],
            SyntaxKind::ServiceGraphPattern => &[SyntaxKind::ServiceLit],
            SyntaxKind::ClOpen => &[SyntaxKind::ClOpen],
            SyntaxKind::ToLit => &[SyntaxKind::ToLit],
            SyntaxKind::BindLit => &[SyntaxKind::BindLit],
            SyntaxKind::Alit => &[SyntaxKind::Alit],
            SyntaxKind::SelectLit => &[SyntaxKind::SelectLit],
            SyntaxKind::TimezoneLit => &[SyntaxKind::TimezoneLit],
            SyntaxKind::SilentLit => &[SyntaxKind::SilentLit],
            SyntaxKind::CreateLit => &[SyntaxKind::CreateLit],
            SyntaxKind::CopyLit => &[SyntaxKind::CopyLit],
            SyntaxKind::DatatypeLit => &[SyntaxKind::DatatypeLit],
            SyntaxKind::SparqlBaseToken => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::UsingLit => &[SyntaxKind::UsingLit],
            SyntaxKind::Gt => &[SyntaxKind::Gt],
            SyntaxKind::RandLit => &[SyntaxKind::RandLit],
            SyntaxKind::Md5Lit => &[SyntaxKind::Md5Lit],
            SyntaxKind::InsertLit => &[SyntaxKind::InsertLit],
            SyntaxKind::DeleteLit => &[SyntaxKind::DeleteLit],
            SyntaxKind::AbsLit => &[SyntaxKind::AbsLit],
            SyntaxKind::IriLit => &[SyntaxKind::IriLit],
            SyntaxKind::UcaseLit => &[SyntaxKind::UcaseLit],
            SyntaxKind::YearLit => &[SyntaxKind::YearLit],
            SyntaxKind::FromLit => &[SyntaxKind::FromLit],
            SyntaxKind::NowLit => &[SyntaxKind::NowLit],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::Eq => &[SyntaxKind::Eq],
            SyntaxKind::IsIriLit => &[SyntaxKind::IsIriLit],
            SyntaxKind::OffsetLit => &[SyntaxKind::OffsetLit],
            SyntaxKind::ConcatLit => &[SyntaxKind::ConcatLit],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::SumLit => &[SyntaxKind::SumLit],
            SyntaxKind::MoveLit => &[SyntaxKind::MoveLit],
            SyntaxKind::LoadLit => &[SyntaxKind::LoadLit],
            SyntaxKind::StrLit => &[SyntaxKind::StrLit],
            SyntaxKind::Bar => &[SyntaxKind::Bar],
            SyntaxKind::DropLit => &[SyntaxKind::DropLit],
            SyntaxKind::Div => &[SyntaxKind::Div],
            SyntaxKind::IntoLit => &[SyntaxKind::IntoLit],
            SyntaxKind::RoundLit => &[SyntaxKind::RoundLit],
            SyntaxKind::Sha512Lit => &[SyntaxKind::Sha512Lit],
            SyntaxKind::ClearLit => &[SyntaxKind::ClearLit],
            SyntaxKind::Questionmark => &[SyntaxKind::Questionmark],
            SyntaxKind::StrafterLit => &[SyntaxKind::StrafterLit],
            SyntaxKind::RegexLit => &[SyntaxKind::RegexLit],
            SyntaxKind::TrueLit => &[SyntaxKind::TrueLit],
            SyntaxKind::AvgLit => &[SyntaxKind::AvgLit],
            SyntaxKind::StringLiteral1 => &[SyntaxKind::StringLiteral1],
            SyntaxKind::LcaseLit => &[SyntaxKind::LcaseLit],
            SyntaxKind::FalseLit => &[SyntaxKind::FalseLit],
            SyntaxKind::SubstrLit => &[SyntaxKind::SubstrLit],
            SyntaxKind::Double => &[SyntaxKind::Double],
            SyntaxKind::CountLit => &[SyntaxKind::CountLit],
            SyntaxKind::PnameNs => &[SyntaxKind::PnameNs],
            SyntaxKind::FilterLit => &[SyntaxKind::FilterLit],
            SyntaxKind::StrendsLit => &[SyntaxKind::StrendsLit],
            SyntaxKind::StrdtLit => &[SyntaxKind::StrdtLit],
            SyntaxKind::AskLit => &[SyntaxKind::AskLit],
            SyntaxKind::HavingLit => &[SyntaxKind::HavingLit],
            SyntaxKind::AscLit => &[SyntaxKind::AscLit],
            SyntaxKind::UnionLit => &[SyntaxKind::UnionLit],
            SyntaxKind::Sha1Lit => &[SyntaxKind::Sha1Lit],
            SyntaxKind::SparqlPrefixToken => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::IntegerNegative => &[SyntaxKind::IntegerNegative],
            SyntaxKind::DistinctLit => &[SyntaxKind::DistinctLit],
            SyntaxKind::ClClose => &[SyntaxKind::ClClose],
            SyntaxKind::ConstructLit => &[SyntaxKind::ConstructLit],
            SyntaxKind::MinusLit => &[SyntaxKind::MinusLit],
            SyntaxKind::DoubleNegative => &[SyntaxKind::DoubleNegative],
            SyntaxKind::DeleteDataLit => &[SyntaxKind::DeleteDataLit],
            SyntaxKind::DecimalPositive => &[SyntaxKind::DecimalPositive],
            SyntaxKind::SecondsLit => &[SyntaxKind::SecondsLit],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::Lt => &[SyntaxKind::Lt],
            SyntaxKind::SeparatorLit => &[SyntaxKind::SeparatorLit],
            SyntaxKind::StringLiteralLong2 => &[SyntaxKind::StringLiteralLong2],
            SyntaxKind::ReducedLit => &[SyntaxKind::ReducedLit],
            SyntaxKind::LangmatchesLit => &[SyntaxKind::LangmatchesLit],
            SyntaxKind::BnodeLit => &[SyntaxKind::BnodeLit],
            SyntaxKind::SampleLit => &[SyntaxKind::SampleLit],
            SyntaxKind::ValuesLit => &[SyntaxKind::ValuesLit],
            SyntaxKind::Neq => &[SyntaxKind::Neq],
            SyntaxKind::ReplaceLit => &[SyntaxKind::ReplaceLit],
            SyntaxKind::DoublePositive => &[SyntaxKind::DoublePositive],
            SyntaxKind::InsertDataLit => &[SyntaxKind::InsertDataLit],
            SyntaxKind::DefaultLit => &[SyntaxKind::DefaultLit],
            SyntaxKind::StrlenLit => &[SyntaxKind::StrlenLit],
            SyntaxKind::GraphLit => &[SyntaxKind::GraphLit],
            SyntaxKind::Lte => &[SyntaxKind::Lte],
            SyntaxKind::OptionalLit => &[SyntaxKind::OptionalLit],
            SyntaxKind::NotLit => &[SyntaxKind::NotLit],
            SyntaxKind::ByLit => &[SyntaxKind::ByLit],
            SyntaxKind::Plus => &[SyntaxKind::Plus],
            SyntaxKind::DeleteWhereLit => &[SyntaxKind::DeleteWhereLit],
            SyntaxKind::Var1 => &[SyntaxKind::Var1],
            SyntaxKind::GroupConcatLit => &[SyntaxKind::GroupConcatLit],
            SyntaxKind::Iriref => &[SyntaxKind::Iriref],
            SyntaxKind::PnameLn => &[SyntaxKind::PnameLn],
            SyntaxKind::BrClose => &[SyntaxKind::BrClose],
            SyntaxKind::OrderLit => &[SyntaxKind::OrderLit],
            SyntaxKind::IfLit => &[SyntaxKind::IfLit],
            SyntaxKind::Langtag => &[SyntaxKind::Langtag],
            SyntaxKind::Var2 => &[SyntaxKind::Var2],
            SyntaxKind::Nil => &[SyntaxKind::Nil],
            SyntaxKind::UriLit => &[SyntaxKind::UriLit],
            SyntaxKind::BlankNodeLabel => &[SyntaxKind::BlankNodeLabel],
            SyntaxKind::EncodeForUriLit => &[SyntaxKind::EncodeForUriLit],
            SyntaxKind::InLit => &[SyntaxKind::InLit],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::Hat => &[SyntaxKind::Hat],
            SyntaxKind::IsLiteralLit => &[SyntaxKind::IsLiteralLit],
            SyntaxKind::StrstartsLit => &[SyntaxKind::StrstartsLit],
            SyntaxKind::WhereLit => &[SyntaxKind::WhereLit],
            SyntaxKind::ServiceLit => &[SyntaxKind::ServiceLit],
            SyntaxKind::IsBlankLit => &[SyntaxKind::IsBlankLit],
            SyntaxKind::TzLit => &[SyntaxKind::TzLit],
            SyntaxKind::FloorLit => &[SyntaxKind::FloorLit],
            SyntaxKind::BoundLit => &[SyntaxKind::BoundLit],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::Sha384Lit => &[SyntaxKind::Sha384Lit],
            SyntaxKind::MinutesLit => &[SyntaxKind::MinutesLit],
            SyntaxKind::BrOpen => &[SyntaxKind::BrOpen],
            SyntaxKind::LangLit => &[SyntaxKind::LangLit],
            SyntaxKind::StruuidLit => &[SyntaxKind::StruuidLit],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::Star => &[SyntaxKind::Star],
            SyntaxKind::Bang => &[SyntaxKind::Bang],
            SyntaxKind::AsLit => &[SyntaxKind::AsLit],
            SyntaxKind::Pipe => &[SyntaxKind::Pipe],
            SyntaxKind::Gte => &[SyntaxKind::Gte],
            SyntaxKind::Sha256Lit => &[SyntaxKind::Sha256Lit],
            SyntaxKind::IsUriLit => &[SyntaxKind::IsUriLit],
            SyntaxKind::StrbeforeLit => &[SyntaxKind::StrbeforeLit],
            SyntaxKind::MaxLit => &[SyntaxKind::MaxLit],
            SyntaxKind::NamedLit => &[SyntaxKind::NamedLit],
            SyntaxKind::DescLit => &[SyntaxKind::DescLit],
            SyntaxKind::ContainsLit => &[SyntaxKind::ContainsLit],
            SyntaxKind::UuidLit => &[SyntaxKind::UuidLit],
            SyntaxKind::Pipe2 => &[SyntaxKind::Pipe2],
            SyntaxKind::DescribeLit => &[SyntaxKind::DescribeLit],
            SyntaxKind::MinLit => &[SyntaxKind::MinLit],
            SyntaxKind::StringLiteral2 => &[SyntaxKind::StringLiteral2],
            SyntaxKind::AllLit => &[SyntaxKind::AllLit],
            SyntaxKind::DayLit => &[SyntaxKind::DayLit],
            SyntaxKind::ExistsLit => &[SyntaxKind::ExistsLit],
            SyntaxKind::CoalesceLit => &[SyntaxKind::CoalesceLit],
            SyntaxKind::GroupLit => &[SyntaxKind::GroupLit],
            SyntaxKind::MonthLit => &[SyntaxKind::MonthLit],
            SyntaxKind::Decimal => &[SyntaxKind::Decimal],
            SyntaxKind::WithLit => &[SyntaxKind::WithLit],
            SyntaxKind::AddLit => &[SyntaxKind::AddLit],
            SyntaxKind::IntegerPositive => &[SyntaxKind::IntegerPositive],
            SyntaxKind::DecimalNegative => &[SyntaxKind::DecimalNegative],
            SyntaxKind::UndefLit => &[SyntaxKind::UndefLit],
            SyntaxKind::StringLiteralLong1 => &[SyntaxKind::StringLiteralLong1],
            SyntaxKind::Integer => &[SyntaxKind::Integer],
            SyntaxKind::LimitLit => &[SyntaxKind::LimitLit],
            SyntaxKind::SameTermLit => &[SyntaxKind::SameTermLit],
            SyntaxKind::Anon => &[SyntaxKind::Anon],
            SyntaxKind::HoursLit => &[SyntaxKind::HoursLit],
            SyntaxKind::IsNumericLit => &[SyntaxKind::IsNumericLit],
            SyntaxKind::Amp2 => &[SyntaxKind::Amp2],
            SyntaxKind::CeilLit => &[SyntaxKind::CeilLit],
            SyntaxKind::StrlangLit => &[SyntaxKind::StrlangLit],
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
                (SyntaxKind::QueryUnit, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::SubSelect, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::ConstructQuery, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::AskQuery, 0usize) => {
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
                (SyntaxKind::DatasetClause, 0usize) => {
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
                (SyntaxKind::DefaultGraphClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::NamedGraphClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::GroupCondition, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::Update1, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::Load, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::Drop, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::Create, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::Add, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::Copy, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::DeleteWhere, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::Modify, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::UsingClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::GraphOrDefault, 0usize) => {
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
                (SyntaxKind::GraphRef, 0usize) => {
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
                (SyntaxKind::QuadPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::QuadData, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::GroupGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::TriplesBlock, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::GraphPatternNotTriples, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::OptionalGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::ServiceGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::InlineData, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::InlineDataFull, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::DataBlockValue, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::MinusGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::Constraint, 0usize) => {
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
                (SyntaxKind::FunctionCall, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::ExpressionList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::PropertyList, 0usize) => {
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
                (SyntaxKind::TriplesSameSubjectPath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::ObjectPath, 0usize) => {
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
                (SyntaxKind::PathElt, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::PathMod, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::PathNegatedPropertySet, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::PathOneInPropertySet, 0usize) => {
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
                (SyntaxKind::TriplesNodePath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::BlankNodePropertyListPath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::GraphNodePath, 0usize) => {
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
                (SyntaxKind::NumericExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::UnaryExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::BrackettedExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::BuiltInCall, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::RegexExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::SubstringExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::StrReplaceExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::ExistsFunc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::Aggregate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::IriOrFunction, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::NumericLiteralPositive, 0usize) => {
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
                (SyntaxKind::PrefixedName, 0usize) => {
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
                (SyntaxKind::BlankNode, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::ClOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::ClOpen, 8isize);
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
                (SyntaxKind::BindLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BindLit, 10isize);
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
                (SyntaxKind::SelectLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SelectLit, 10isize);
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
                (SyntaxKind::SilentLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SilentLit, 10isize);
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
                (SyntaxKind::SparqlBaseToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::SparqlBaseToken, 100isize);
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
                (SyntaxKind::Gt, _) => {
                    let added = state.expect_as(element, SyntaxKind::Gt, 10isize);
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
                (SyntaxKind::Md5Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Md5Lit, 10isize);
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
                (SyntaxKind::DeleteLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DeleteLit, 10isize);
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
                (SyntaxKind::IriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IriLit, 10isize);
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
                (SyntaxKind::YearLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::YearLit, 10isize);
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
                (SyntaxKind::NowLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::NowLit, 10isize);
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
                (SyntaxKind::Eq, _) => {
                    let added = state.expect_as(element, SyntaxKind::Eq, 10isize);
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
                (SyntaxKind::OffsetLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::OffsetLit, 10isize);
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
                (SyntaxKind::Datatype, _) => {
                    let added = state.expect_as(element, SyntaxKind::Datatype, 10isize);
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
                (SyntaxKind::MoveLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MoveLit, 10isize);
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
                (SyntaxKind::StrLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrLit, 10isize);
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
                (SyntaxKind::DropLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DropLit, 10isize);
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
                (SyntaxKind::IntoLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IntoLit, 10isize);
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
                (SyntaxKind::Sha512Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Sha512Lit, 10isize);
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
                (SyntaxKind::Questionmark, _) => {
                    let added = state.expect_as(element, SyntaxKind::Questionmark, 10isize);
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
                (SyntaxKind::RegexLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::RegexLit, 10isize);
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
                (SyntaxKind::AvgLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AvgLit, 10isize);
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
                (SyntaxKind::LcaseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LcaseLit, 10isize);
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
                (SyntaxKind::SubstrLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SubstrLit, 10isize);
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
                (SyntaxKind::CountLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CountLit, 10isize);
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
                (SyntaxKind::FilterLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FilterLit, 10isize);
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
                (SyntaxKind::StrdtLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrdtLit, 10isize);
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
                (SyntaxKind::UnionLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UnionLit, 10isize);
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
                (SyntaxKind::SparqlPrefixToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::SparqlPrefixToken, 100isize);
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
                (SyntaxKind::DistinctLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DistinctLit, 10isize);
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
                (SyntaxKind::ConstructLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ConstructLit, 10isize);
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
                (SyntaxKind::DoubleNegative, _) => {
                    let added = state.expect_as(element, SyntaxKind::DoubleNegative, 2isize);
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
                (SyntaxKind::DecimalPositive, _) => {
                    let added = state.expect_as(element, SyntaxKind::DecimalPositive, 2isize);
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
                (SyntaxKind::SqClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::SqClose, 8isize);
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
                (SyntaxKind::SeparatorLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SeparatorLit, 10isize);
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
                (SyntaxKind::ReducedLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ReducedLit, 10isize);
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
                (SyntaxKind::BnodeLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BnodeLit, 10isize);
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
                (SyntaxKind::ValuesLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ValuesLit, 10isize);
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
                (SyntaxKind::ReplaceLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ReplaceLit, 10isize);
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
                (SyntaxKind::InsertDataLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::InsertDataLit, 10isize);
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
                (SyntaxKind::StrlenLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrlenLit, 10isize);
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
                (SyntaxKind::Lte, _) => {
                    let added = state.expect_as(element, SyntaxKind::Lte, 10isize);
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
                (SyntaxKind::NotLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::NotLit, 10isize);
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
                (SyntaxKind::Plus, _) => {
                    let added = state.expect_as(element, SyntaxKind::Plus, 10isize);
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
                (SyntaxKind::Var1, _) => {
                    let added = state.expect_as(element, SyntaxKind::Var1, 2isize);
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
                (SyntaxKind::Iriref, _) => {
                    let added = state.expect_as(element, SyntaxKind::Iriref, 2isize);
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
                (SyntaxKind::BrClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::BrClose, 8isize);
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
                (SyntaxKind::IfLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IfLit, 10isize);
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
                (SyntaxKind::Var2, _) => {
                    let added = state.expect_as(element, SyntaxKind::Var2, 2isize);
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
                (SyntaxKind::UriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UriLit, 10isize);
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
                (SyntaxKind::EncodeForUriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::EncodeForUriLit, 10isize);
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
                (SyntaxKind::Comma, _) => {
                    let added = state.expect_as(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::IsLiteralLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsLiteralLit, 10isize);
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
                (SyntaxKind::WhereLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::WhereLit, 10isize);
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
                (SyntaxKind::IsBlankLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsBlankLit, 10isize);
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
                (SyntaxKind::FloorLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FloorLit, 10isize);
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
                (SyntaxKind::Stop, _) => {
                    let added = state.expect_as(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::MinutesLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MinutesLit, 10isize);
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
                (SyntaxKind::LangLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LangLit, 10isize);
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
                (SyntaxKind::Colon, _) => {
                    let added = state.expect_as(element, SyntaxKind::Colon, 10isize);
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
                (SyntaxKind::Bang, _) => {
                    let added = state.expect_as(element, SyntaxKind::Bang, 10isize);
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
                (SyntaxKind::Pipe, _) => {
                    let added = state.expect_as(element, SyntaxKind::Pipe, 10isize);
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
                (SyntaxKind::Sha256Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Sha256Lit, 10isize);
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
                (SyntaxKind::StrbeforeLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrbeforeLit, 10isize);
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
                (SyntaxKind::NamedLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::NamedLit, 10isize);
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
                (SyntaxKind::ContainsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ContainsLit, 10isize);
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
                (SyntaxKind::Pipe2, _) => {
                    let added = state.expect_as(element, SyntaxKind::Pipe2, 10isize);
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
                (SyntaxKind::MinLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MinLit, 10isize);
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
                (SyntaxKind::AllLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AllLit, 10isize);
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
                (SyntaxKind::ExistsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ExistsLit, 10isize);
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
                (SyntaxKind::GroupLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::GroupLit, 10isize);
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
                (SyntaxKind::Decimal, _) => {
                    let added = state.expect_as(element, SyntaxKind::Decimal, 2isize);
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
                (SyntaxKind::AddLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AddLit, 10isize);
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
                (SyntaxKind::DecimalNegative, _) => {
                    let added = state.expect_as(element, SyntaxKind::DecimalNegative, 2isize);
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
                (SyntaxKind::StringLiteralLong1, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralLong1, 2isize);
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
                (SyntaxKind::LimitLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LimitLit, 10isize);
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
                (SyntaxKind::Anon, _) => {
                    let added = state.expect_as(element, SyntaxKind::Anon, 2isize);
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
                (SyntaxKind::IsNumericLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsNumericLit, 10isize);
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
                (SyntaxKind::CeilLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CeilLit, 10isize);
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
