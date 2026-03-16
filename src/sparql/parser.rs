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
    #[token("OFFSET")]
    OffsetLit,
    #[token("?")]
    Questionmark,
    #[token("SEPARATOR")]
    SeparatorLit,
    #[token("NOT")]
    NotLit,
    #[token("NAMED")]
    NamedLit,
    #[token("STRSTARTS")]
    StrstartsLit,
    #[regex("(?&DECIMAL_NEGATIVE)")]
    DecimalNegative,
    #[token("GROUP")]
    GroupLit,
    #[token("DELETE DATA")]
    DeleteDataLit,
    #[token("REDUCED")]
    ReducedLit,
    #[token("INSERT DATA")]
    InsertDataLit,
    #[token("RAND")]
    RandLit,
    #[regex("(?&IRIREF)")]
    Iriref,
    #[token("{")]
    ClOpen,
    #[token("CREATE")]
    CreateLit,
    #[token("BY")]
    ByLit,
    #[token("COALESCE")]
    CoalesceLit,
    #[token("sameTerm")]
    SameTermLit,
    #[token("isBLANK")]
    IsBlankLit,
    #[regex("(?&BLANK_NODE_LABEL)")]
    BlankNodeLabel,
    #[token("^^")]
    Datatype,
    #[token("SELECT")]
    SelectLit,
    #[token("DEFAULT")]
    DefaultLit,
    #[token("UNDEF")]
    UndefLit,
    #[token("isNUMERIC")]
    IsNumericLit,
    #[token("COPY")]
    CopyLit,
    #[token("MINUTES")]
    MinutesLit,
    #[regex("(?&PNAME_LN)")]
    PnameLn,
    #[token("isURI")]
    IsUriLit,
    #[token("AS")]
    AsLit,
    #[token("WITH")]
    WithLit,
    #[token("REGEX")]
    RegexLit,
    #[token("SAMPLE")]
    SampleLit,
    #[token("|")]
    Pipe,
    #[token("SUBSTR")]
    SubstrLit,
    #[regex("(?&LANGTAG)")]
    Langtag,
    #[token("FILTER")]
    FilterLit,
    #[token("ENCODE_FOR_URI")]
    EncodeForUriLit,
    #[token("SECONDS")]
    SecondsLit,
    #[token("ROUND")]
    RoundLit,
    #[token("*")]
    Star,
    #[token("UCASE")]
    UcaseLit,
    #[token("UNION")]
    UnionLit,
    #[token("IF")]
    IfLit,
    #[regex("(?&STRING_LITERAL_LONG1)")]
    StringLiteralLong1,
    #[token("CONCAT")]
    ConcatLit,
    #[token("CLEAR")]
    ClearLit,
    #[token("ALL")]
    AllLit,
    #[token("DESCRIBE")]
    DescribeLit,
    #[token("STRBEFORE")]
    StrbeforeLit,
    #[token("MIN")]
    MinLit,
    #[token("SILENT")]
    SilentLit,
    #[token("<")]
    Lt,
    #[token("a")]
    Alit,
    #[token("INTO")]
    IntoLit,
    #[regex("(?&ANON)")]
    Anon,
    #[token("-")]
    Bar,
    #[token("NOW")]
    NowLit,
    #[token("DAY")]
    DayLit,
    #[token("MAX")]
    MaxLit,
    #[regex("(?&DOUBLE)")]
    Double,
    #[token("^")]
    Hat,
    #[token("HAVING")]
    HavingLit,
    #[token("true")]
    TrueLit,
    #[token("<=")]
    Lte,
    #[token("CONSTRUCT")]
    ConstructLit,
    #[token("ADD")]
    AddLit,
    #[regex("(?&INTEGER)")]
    Integer,
    #[token("TIMEZONE")]
    TimezoneLit,
    #[regex("(?&VAR1)")]
    Var1,
    #[token("LOAD")]
    LoadLit,
    #[token("OPTIONAL")]
    OptionalLit,
    #[token("BOUND")]
    BoundLit,
    #[regex("(?&NIL)")]
    Nil,
    #[token("STRLEN")]
    StrlenLit,
    #[token("IRI")]
    IriLit,
    #[token("TZ")]
    TzLit,
    #[token("SHA512")]
    Sha512Lit,
    #[token(".")]
    Stop,
    #[token("||")]
    Pipe2,
    #[token("LCASE")]
    LcaseLit,
    #[token("SUM")]
    SumLit,
    #[token("TO")]
    ToLit,
    #[token("REPLACE")]
    ReplaceLit,
    #[token("GROUP_CONCAT")]
    GroupConcatLit,
    #[regex("(?&STRING_LITERAL1)")]
    StringLiteral1,
    #[token("SHA256")]
    Sha256Lit,
    #[token("LANGMATCHES")]
    LangmatchesLit,
    #[token("STRAFTER")]
    StrafterLit,
    #[token("SERVICE")]
    ServiceLit,
    #[token("ABS")]
    AbsLit,
    #[token("false")]
    FalseLit,
    #[token(",")]
    Comma,
    #[regex("(?&INTEGER_POSITIVE)")]
    IntegerPositive,
    #[token("[")]
    SqOpen,
    #[token("AVG")]
    AvgLit,
    #[token("ASK")]
    AskLit,
    #[token("LANG")]
    LangLit,
    #[token("DELETE")]
    DeleteLit,
    #[regex("(?&STRING_LITERAL2)")]
    StringLiteral2,
    #[token("USING")]
    UsingLit,
    #[token("HOURS")]
    HoursLit,
    #[token("isIRI")]
    IsIriLit,
    #[regex("(?&STRING_LITERAL_LONG2)")]
    StringLiteralLong2,
    #[token("]")]
    SqClose,
    #[regex("(?&VAR2)")]
    Var2,
    #[token(">")]
    Gt,
    #[token("IN")]
    InLit,
    #[token("URI")]
    UriLit,
    #[regex("(?&INTEGER_NEGATIVE)")]
    IntegerNegative,
    #[token("DISTINCT")]
    DistinctLit,
    #[token("CONTAINS")]
    ContainsLit,
    #[token("UUID")]
    UuidLit,
    #[token("DROP")]
    DropLit,
    #[token("(")]
    BrOpen,
    #[token("FLOOR")]
    FloorLit,
    #[regex("(?&DECIMAL_POSITIVE)")]
    DecimalPositive,
    #[token("GRAPH")]
    GraphLit,
    #[token("MINUS")]
    MinusLit,
    #[token("STRENDS")]
    StrendsLit,
    #[token("BNODE")]
    BnodeLit,
    #[token("EXISTS")]
    ExistsLit,
    #[token("FROM")]
    FromLit,
    #[token("CEIL")]
    CeilLit,
    #[token("DELETE WHERE")]
    DeleteWhereLit,
    #[token("SHA1")]
    Sha1Lit,
    #[token("/")]
    Div,
    #[token("STRLANG")]
    StrlangLit,
    #[regex("(?&DECIMAL)")]
    Decimal,
    #[token(">=")]
    Gte,
    #[token("WHERE")]
    WhereLit,
    #[token("MD5")]
    Md5Lit,
    #[token(")")]
    BrClose,
    #[token("=")]
    Eq,
    #[token("ASC")]
    AscLit,
    #[token("YEAR")]
    YearLit,
    #[token("VALUES")]
    ValuesLit,
    #[token("}")]
    ClClose,
    #[token(";")]
    Colon,
    #[token("STRUUID")]
    StruuidLit,
    #[token("STRDT")]
    StrdtLit,
    #[token("MOVE")]
    MoveLit,
    #[token("isLITERAL")]
    IsLiteralLit,
    #[regex("(?&DOUBLE_POSITIVE)")]
    DoublePositive,
    #[token("MONTH")]
    MonthLit,
    #[token("DATATYPE")]
    DatatypeLit,
    #[token("INSERT")]
    InsertLit,
    #[token("PREFIX")]
    SparqlPrefixToken,
    #[token("BIND")]
    BindLit,
    #[token("STR")]
    StrLit,
    #[token("COUNT")]
    CountLit,
    #[token("SHA384")]
    Sha384Lit,
    #[regex("(?&DOUBLE_NEGATIVE)")]
    DoubleNegative,
    #[token("!")]
    Bang,
    #[token("BASE")]
    SparqlBaseToken,
    #[regex("(?&PNAME_NS)")]
    PnameNs,
    #[token("ORDER")]
    OrderLit,
    #[token("LIMIT")]
    LimitLit,
    #[token("&&")]
    Amp2,
    #[token("DESC")]
    DescLit,
    #[token("!=")]
    Neq,
    #[token("+")]
    Plus,
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
                    state: 15usize,
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
                    state: 20usize,
                },
                SyntaxKind::NumericExpression => Rule {
                    kind,
                    state: 1usize,
                },
                SyntaxKind::AdditiveExpression => Rule {
                    kind,
                    state: 16usize,
                },
                SyntaxKind::MultiplicativeExpression => Rule {
                    kind,
                    state: 7usize,
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
                    state: 1usize,
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
                SyntaxKind::OffsetLit => Rule { kind, state: 0 },
                SyntaxKind::Questionmark => Rule { kind, state: 0 },
                SyntaxKind::SeparatorLit => Rule { kind, state: 0 },
                SyntaxKind::NotLit => Rule { kind, state: 0 },
                SyntaxKind::NamedLit => Rule { kind, state: 0 },
                SyntaxKind::StrstartsLit => Rule { kind, state: 0 },
                SyntaxKind::DecimalNegative => Rule { kind, state: 0 },
                SyntaxKind::GroupLit => Rule { kind, state: 0 },
                SyntaxKind::DeleteDataLit => Rule { kind, state: 0 },
                SyntaxKind::ReducedLit => Rule { kind, state: 0 },
                SyntaxKind::InsertDataLit => Rule { kind, state: 0 },
                SyntaxKind::RandLit => Rule { kind, state: 0 },
                SyntaxKind::Iriref => Rule { kind, state: 0 },
                SyntaxKind::ClOpen => Rule { kind, state: 0 },
                SyntaxKind::CreateLit => Rule { kind, state: 0 },
                SyntaxKind::ByLit => Rule { kind, state: 0 },
                SyntaxKind::CoalesceLit => Rule { kind, state: 0 },
                SyntaxKind::SameTermLit => Rule { kind, state: 0 },
                SyntaxKind::IsBlankLit => Rule { kind, state: 0 },
                SyntaxKind::BlankNodeLabel => Rule { kind, state: 0 },
                SyntaxKind::Datatype => Rule { kind, state: 0 },
                SyntaxKind::SelectLit => Rule { kind, state: 0 },
                SyntaxKind::DefaultLit => Rule { kind, state: 0 },
                SyntaxKind::UndefLit => Rule { kind, state: 0 },
                SyntaxKind::IsNumericLit => Rule { kind, state: 0 },
                SyntaxKind::CopyLit => Rule { kind, state: 0 },
                SyntaxKind::MinutesLit => Rule { kind, state: 0 },
                SyntaxKind::PnameLn => Rule { kind, state: 0 },
                SyntaxKind::IsUriLit => Rule { kind, state: 0 },
                SyntaxKind::AsLit => Rule { kind, state: 0 },
                SyntaxKind::WithLit => Rule { kind, state: 0 },
                SyntaxKind::RegexLit => Rule { kind, state: 0 },
                SyntaxKind::SampleLit => Rule { kind, state: 0 },
                SyntaxKind::Pipe => Rule { kind, state: 0 },
                SyntaxKind::SubstrLit => Rule { kind, state: 0 },
                SyntaxKind::Langtag => Rule { kind, state: 0 },
                SyntaxKind::FilterLit => Rule { kind, state: 0 },
                SyntaxKind::EncodeForUriLit => Rule { kind, state: 0 },
                SyntaxKind::SecondsLit => Rule { kind, state: 0 },
                SyntaxKind::RoundLit => Rule { kind, state: 0 },
                SyntaxKind::Star => Rule { kind, state: 0 },
                SyntaxKind::UcaseLit => Rule { kind, state: 0 },
                SyntaxKind::UnionLit => Rule { kind, state: 0 },
                SyntaxKind::IfLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLong1 => Rule { kind, state: 0 },
                SyntaxKind::ConcatLit => Rule { kind, state: 0 },
                SyntaxKind::ClearLit => Rule { kind, state: 0 },
                SyntaxKind::AllLit => Rule { kind, state: 0 },
                SyntaxKind::DescribeLit => Rule { kind, state: 0 },
                SyntaxKind::StrbeforeLit => Rule { kind, state: 0 },
                SyntaxKind::MinLit => Rule { kind, state: 0 },
                SyntaxKind::SilentLit => Rule { kind, state: 0 },
                SyntaxKind::Lt => Rule { kind, state: 0 },
                SyntaxKind::Alit => Rule { kind, state: 0 },
                SyntaxKind::IntoLit => Rule { kind, state: 0 },
                SyntaxKind::Anon => Rule { kind, state: 0 },
                SyntaxKind::Bar => Rule { kind, state: 0 },
                SyntaxKind::NowLit => Rule { kind, state: 0 },
                SyntaxKind::DayLit => Rule { kind, state: 0 },
                SyntaxKind::MaxLit => Rule { kind, state: 0 },
                SyntaxKind::Double => Rule { kind, state: 0 },
                SyntaxKind::Hat => Rule { kind, state: 0 },
                SyntaxKind::HavingLit => Rule { kind, state: 0 },
                SyntaxKind::TrueLit => Rule { kind, state: 0 },
                SyntaxKind::Lte => Rule { kind, state: 0 },
                SyntaxKind::ConstructLit => Rule { kind, state: 0 },
                SyntaxKind::AddLit => Rule { kind, state: 0 },
                SyntaxKind::Integer => Rule { kind, state: 0 },
                SyntaxKind::TimezoneLit => Rule { kind, state: 0 },
                SyntaxKind::Var1 => Rule { kind, state: 0 },
                SyntaxKind::LoadLit => Rule { kind, state: 0 },
                SyntaxKind::OptionalLit => Rule { kind, state: 0 },
                SyntaxKind::BoundLit => Rule { kind, state: 0 },
                SyntaxKind::Nil => Rule { kind, state: 0 },
                SyntaxKind::StrlenLit => Rule { kind, state: 0 },
                SyntaxKind::IriLit => Rule { kind, state: 0 },
                SyntaxKind::TzLit => Rule { kind, state: 0 },
                SyntaxKind::Sha512Lit => Rule { kind, state: 0 },
                SyntaxKind::Stop => Rule { kind, state: 0 },
                SyntaxKind::Pipe2 => Rule { kind, state: 0 },
                SyntaxKind::LcaseLit => Rule { kind, state: 0 },
                SyntaxKind::SumLit => Rule { kind, state: 0 },
                SyntaxKind::ToLit => Rule { kind, state: 0 },
                SyntaxKind::ReplaceLit => Rule { kind, state: 0 },
                SyntaxKind::GroupConcatLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteral1 => Rule { kind, state: 0 },
                SyntaxKind::Sha256Lit => Rule { kind, state: 0 },
                SyntaxKind::LangmatchesLit => Rule { kind, state: 0 },
                SyntaxKind::StrafterLit => Rule { kind, state: 0 },
                SyntaxKind::ServiceLit => Rule { kind, state: 0 },
                SyntaxKind::AbsLit => Rule { kind, state: 0 },
                SyntaxKind::FalseLit => Rule { kind, state: 0 },
                SyntaxKind::Comma => Rule { kind, state: 0 },
                SyntaxKind::IntegerPositive => Rule { kind, state: 0 },
                SyntaxKind::SqOpen => Rule { kind, state: 0 },
                SyntaxKind::AvgLit => Rule { kind, state: 0 },
                SyntaxKind::AskLit => Rule { kind, state: 0 },
                SyntaxKind::LangLit => Rule { kind, state: 0 },
                SyntaxKind::DeleteLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteral2 => Rule { kind, state: 0 },
                SyntaxKind::UsingLit => Rule { kind, state: 0 },
                SyntaxKind::HoursLit => Rule { kind, state: 0 },
                SyntaxKind::IsIriLit => Rule { kind, state: 0 },
                SyntaxKind::StringLiteralLong2 => Rule { kind, state: 0 },
                SyntaxKind::SqClose => Rule { kind, state: 0 },
                SyntaxKind::Var2 => Rule { kind, state: 0 },
                SyntaxKind::Gt => Rule { kind, state: 0 },
                SyntaxKind::InLit => Rule { kind, state: 0 },
                SyntaxKind::UriLit => Rule { kind, state: 0 },
                SyntaxKind::IntegerNegative => Rule { kind, state: 0 },
                SyntaxKind::DistinctLit => Rule { kind, state: 0 },
                SyntaxKind::ContainsLit => Rule { kind, state: 0 },
                SyntaxKind::UuidLit => Rule { kind, state: 0 },
                SyntaxKind::DropLit => Rule { kind, state: 0 },
                SyntaxKind::BrOpen => Rule { kind, state: 0 },
                SyntaxKind::FloorLit => Rule { kind, state: 0 },
                SyntaxKind::DecimalPositive => Rule { kind, state: 0 },
                SyntaxKind::GraphLit => Rule { kind, state: 0 },
                SyntaxKind::MinusLit => Rule { kind, state: 0 },
                SyntaxKind::StrendsLit => Rule { kind, state: 0 },
                SyntaxKind::BnodeLit => Rule { kind, state: 0 },
                SyntaxKind::ExistsLit => Rule { kind, state: 0 },
                SyntaxKind::FromLit => Rule { kind, state: 0 },
                SyntaxKind::CeilLit => Rule { kind, state: 0 },
                SyntaxKind::DeleteWhereLit => Rule { kind, state: 0 },
                SyntaxKind::Sha1Lit => Rule { kind, state: 0 },
                SyntaxKind::Div => Rule { kind, state: 0 },
                SyntaxKind::StrlangLit => Rule { kind, state: 0 },
                SyntaxKind::Decimal => Rule { kind, state: 0 },
                SyntaxKind::Gte => Rule { kind, state: 0 },
                SyntaxKind::WhereLit => Rule { kind, state: 0 },
                SyntaxKind::Md5Lit => Rule { kind, state: 0 },
                SyntaxKind::BrClose => Rule { kind, state: 0 },
                SyntaxKind::Eq => Rule { kind, state: 0 },
                SyntaxKind::AscLit => Rule { kind, state: 0 },
                SyntaxKind::YearLit => Rule { kind, state: 0 },
                SyntaxKind::ValuesLit => Rule { kind, state: 0 },
                SyntaxKind::ClClose => Rule { kind, state: 0 },
                SyntaxKind::Colon => Rule { kind, state: 0 },
                SyntaxKind::StruuidLit => Rule { kind, state: 0 },
                SyntaxKind::StrdtLit => Rule { kind, state: 0 },
                SyntaxKind::MoveLit => Rule { kind, state: 0 },
                SyntaxKind::IsLiteralLit => Rule { kind, state: 0 },
                SyntaxKind::DoublePositive => Rule { kind, state: 0 },
                SyntaxKind::MonthLit => Rule { kind, state: 0 },
                SyntaxKind::DatatypeLit => Rule { kind, state: 0 },
                SyntaxKind::InsertLit => Rule { kind, state: 0 },
                SyntaxKind::SparqlPrefixToken => Rule { kind, state: 0 },
                SyntaxKind::BindLit => Rule { kind, state: 0 },
                SyntaxKind::StrLit => Rule { kind, state: 0 },
                SyntaxKind::CountLit => Rule { kind, state: 0 },
                SyntaxKind::Sha384Lit => Rule { kind, state: 0 },
                SyntaxKind::DoubleNegative => Rule { kind, state: 0 },
                SyntaxKind::Bang => Rule { kind, state: 0 },
                SyntaxKind::SparqlBaseToken => Rule { kind, state: 0 },
                SyntaxKind::PnameNs => Rule { kind, state: 0 },
                SyntaxKind::OrderLit => Rule { kind, state: 0 },
                SyntaxKind::LimitLit => Rule { kind, state: 0 },
                SyntaxKind::Amp2 => Rule { kind, state: 0 },
                SyntaxKind::DescLit => Rule { kind, state: 0 },
                SyntaxKind::Neq => Rule { kind, state: 0 },
                SyntaxKind::Plus => Rule { kind, state: 0 },
                _ => panic!("Unknown rule kind {:?}", kind),
            }
        }
    }
    pub fn first_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
        match kind {
            SyntaxKind::DataBlockValue => &[
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
                SyntaxKind::UndefLit,
            ],
            SyntaxKind::PathMod => &[SyntaxKind::Star, SyntaxKind::Questionmark, SyntaxKind::Plus],
            SyntaxKind::Query => &[
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::SelectLit,
                SyntaxKind::DescribeLit,
                SyntaxKind::AskLit,
                SyntaxKind::ConstructLit,
                SyntaxKind::SparqlBaseToken,
            ],
            SyntaxKind::ConditionalOrExpression => &[
                SyntaxKind::RegexLit,
                SyntaxKind::Bar,
                SyntaxKind::HoursLit,
                SyntaxKind::FloorLit,
                SyntaxKind::IfLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::CountLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::CeilLit,
                SyntaxKind::UuidLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::YearLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::RoundLit,
                SyntaxKind::Bang,
                SyntaxKind::TrueLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::NotLit,
                SyntaxKind::MonthLit,
                SyntaxKind::IriLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::DayLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::AbsLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StrLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::UriLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::NowLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::Plus,
                SyntaxKind::CoalesceLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::FalseLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::MaxLit,
                SyntaxKind::RandLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::SumLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::AvgLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::LangLit,
                SyntaxKind::BrOpen,
                SyntaxKind::IsBlankLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::SampleLit,
                SyntaxKind::MinLit,
                SyntaxKind::TzLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::ContainsLit,
            ],
            SyntaxKind::DeleteClause => &[SyntaxKind::DeleteLit],
            SyntaxKind::UsingClause => &[SyntaxKind::UsingLit],
            SyntaxKind::QuadData => &[SyntaxKind::ClOpen],
            SyntaxKind::ObjectPath => &[
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
            ],
            SyntaxKind::PathOneInPropertySet => &[SyntaxKind::Alit, SyntaxKind::Hat],
            SyntaxKind::RegexExpression => &[SyntaxKind::RegexLit],
            SyntaxKind::Create => &[SyntaxKind::CreateLit],
            SyntaxKind::OffsetClause => &[SyntaxKind::OffsetLit],
            SyntaxKind::ValueLogical => &[
                SyntaxKind::StrafterLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::AbsLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::Bang,
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
                SyntaxKind::IfLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::MaxLit,
                SyntaxKind::Bar,
                SyntaxKind::FloorLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::NowLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::YearLit,
                SyntaxKind::CountLit,
                SyntaxKind::MonthLit,
                SyntaxKind::DayLit,
                SyntaxKind::UriLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::SampleLit,
                SyntaxKind::CeilLit,
                SyntaxKind::LangLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::NotLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::MinLit,
                SyntaxKind::RandLit,
                SyntaxKind::BoundLit,
                SyntaxKind::HoursLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::AvgLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::StrendsLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::RoundLit,
                SyntaxKind::SumLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::UuidLit,
                SyntaxKind::TzLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::UcaseLit,
                SyntaxKind::IriLit,
                SyntaxKind::BrOpen,
                SyntaxKind::SubstrLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::RegexLit,
                SyntaxKind::Plus,
                SyntaxKind::MinutesLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::StrLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::ContainsLit,
            ],
            SyntaxKind::AskQuery => &[SyntaxKind::AskLit],
            SyntaxKind::DataBlock => &[SyntaxKind::BrOpen],
            SyntaxKind::NumericLiteralUnsigned => &[],
            SyntaxKind::NumericLiteralPositive => &[],
            SyntaxKind::UpdateUnit => &[
                SyntaxKind::AddLit,
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::WithLit,
                SyntaxKind::DropLit,
                SyntaxKind::DeleteDataLit,
                SyntaxKind::InsertDataLit,
                SyntaxKind::LoadLit,
                SyntaxKind::InsertLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::MoveLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::CreateLit,
                SyntaxKind::DeleteLit,
                SyntaxKind::ClearLit,
                SyntaxKind::CopyLit,
            ],
            SyntaxKind::MyString => &[],
            SyntaxKind::DeleteData => &[SyntaxKind::DeleteDataLit],
            SyntaxKind::LimitClause => &[SyntaxKind::LimitLit],
            SyntaxKind::MinusGraphPattern => &[SyntaxKind::MinusLit],
            SyntaxKind::Path => &[
                SyntaxKind::Alit,
                SyntaxKind::Hat,
                SyntaxKind::Bang,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::NumericExpression => &[
                SyntaxKind::SampleLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StrLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::CountLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::FalseLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::CeilLit,
                SyntaxKind::IfLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::MinLit,
                SyntaxKind::RegexLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::Bar,
                SyntaxKind::StrstartsLit,
                SyntaxKind::AbsLit,
                SyntaxKind::Bang,
                SyntaxKind::StrlenLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::TrueLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::BnodeLit,
                SyntaxKind::UuidLit,
                SyntaxKind::RandLit,
                SyntaxKind::FloorLit,
                SyntaxKind::AvgLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::IriLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::Plus,
                SyntaxKind::ExistsLit,
                SyntaxKind::YearLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::MaxLit,
                SyntaxKind::DayLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::NotLit,
                SyntaxKind::UriLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::LangLit,
                SyntaxKind::TzLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::HoursLit,
                SyntaxKind::BrOpen,
                SyntaxKind::Sha384Lit,
                SyntaxKind::SumLit,
                SyntaxKind::NowLit,
            ],
            SyntaxKind::GraphTerm => &[SyntaxKind::FalseLit, SyntaxKind::TrueLit],
            SyntaxKind::Verb => &[SyntaxKind::Alit],
            SyntaxKind::PathElt => &[SyntaxKind::Alit, SyntaxKind::BrOpen, SyntaxKind::Bang],
            SyntaxKind::SourceSelector => &[],
            SyntaxKind::Prologue => &[SyntaxKind::SparqlPrefixToken, SyntaxKind::SparqlBaseToken],
            SyntaxKind::PathNegatedPropertySet => {
                &[SyntaxKind::Alit, SyntaxKind::BrOpen, SyntaxKind::Hat]
            }
            SyntaxKind::VerbPath => &[
                SyntaxKind::Alit,
                SyntaxKind::Bang,
                SyntaxKind::BrOpen,
                SyntaxKind::Hat,
            ],
            SyntaxKind::MultiplicativeExpression => &[
                SyntaxKind::HoursLit,
                SyntaxKind::RandLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::UriLit,
                SyntaxKind::FloorLit,
                SyntaxKind::UuidLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::SampleLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::MaxLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::AbsLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::TzLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::MinLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::AvgLit,
                SyntaxKind::CountLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::YearLit,
                SyntaxKind::IriLit,
                SyntaxKind::BrOpen,
                SyntaxKind::Bang,
                SyntaxKind::NotLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::FalseLit,
                SyntaxKind::IfLit,
                SyntaxKind::Bar,
                SyntaxKind::StrdtLit,
                SyntaxKind::LangLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::NowLit,
                SyntaxKind::RegexLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::StrlangLit,
                SyntaxKind::Plus,
                SyntaxKind::DayLit,
                SyntaxKind::BoundLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::CeilLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::SecondsLit,
                SyntaxKind::RoundLit,
                SyntaxKind::SumLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::EncodeForUriLit,
            ],
            SyntaxKind::NotExistsFunc => &[SyntaxKind::NotLit],
            SyntaxKind::PrimaryExpression => &[
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::SumLit,
                SyntaxKind::TzLit,
                SyntaxKind::UuidLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::DayLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::NowLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::SameTermLit,
                SyntaxKind::IriLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::TrueLit,
                SyntaxKind::UriLit,
                SyntaxKind::CountLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::RoundLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrLit,
                SyntaxKind::SampleLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::FalseLit,
                SyntaxKind::CeilLit,
                SyntaxKind::IfLit,
                SyntaxKind::MaxLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::HoursLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::BrOpen,
                SyntaxKind::AbsLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::RegexLit,
                SyntaxKind::RandLit,
                SyntaxKind::NotLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::MinLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::LangLit,
                SyntaxKind::MonthLit,
                SyntaxKind::AvgLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::YearLit,
            ],
            SyntaxKind::QuadPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::Var => &[],
            SyntaxKind::GroupCondition => &[
                SyntaxKind::AvgLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::CeilLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::StrdtLit,
                SyntaxKind::UuidLit,
                SyntaxKind::MaxLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::SumLit,
                SyntaxKind::HoursLit,
                SyntaxKind::IriLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::BrOpen,
                SyntaxKind::IsBlankLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::CountLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::MonthLit,
                SyntaxKind::UriLit,
                SyntaxKind::LangLit,
                SyntaxKind::YearLit,
                SyntaxKind::NotLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::FloorLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::RandLit,
                SyntaxKind::TzLit,
                SyntaxKind::AbsLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::BoundLit,
                SyntaxKind::RegexLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::MinLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::ExistsLit,
                SyntaxKind::StrLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::SampleLit,
                SyntaxKind::NowLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::IfLit,
                SyntaxKind::DayLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::ConcatLit,
            ],
            SyntaxKind::UnaryExpression => &[
                SyntaxKind::IsUriLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::SecondsLit,
                SyntaxKind::BoundLit,
                SyntaxKind::StrLit,
                SyntaxKind::IriLit,
                SyntaxKind::RoundLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::MaxLit,
                SyntaxKind::UuidLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::AvgLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::TzLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::CountLit,
                SyntaxKind::RegexLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::FalseLit,
                SyntaxKind::IfLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::NotLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::SumLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::Plus,
                SyntaxKind::Bar,
                SyntaxKind::SameTermLit,
                SyntaxKind::RandLit,
                SyntaxKind::Bang,
                SyntaxKind::AbsLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::MonthLit,
                SyntaxKind::YearLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::NowLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::UriLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::TrueLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::MinLit,
                SyntaxKind::LangLit,
                SyntaxKind::DayLit,
                SyntaxKind::CeilLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::BrOpen,
                SyntaxKind::SampleLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::LcaseLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::HoursLit,
                SyntaxKind::ExistsLit,
            ],
            SyntaxKind::ArgList => &[SyntaxKind::BrOpen],
            SyntaxKind::InlineData => &[SyntaxKind::ValuesLit],
            SyntaxKind::AdditiveExpression => &[
                SyntaxKind::SecondsLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::UriLit,
                SyntaxKind::MinLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::MaxLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::NotLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::MinutesLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::BrOpen,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::FalseLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::RandLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::SumLit,
                SyntaxKind::CountLit,
                SyntaxKind::IriLit,
                SyntaxKind::HoursLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::TrueLit,
                SyntaxKind::AbsLit,
                SyntaxKind::RoundLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::UuidLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::LangLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::UcaseLit,
                SyntaxKind::Bar,
                SyntaxKind::TzLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::AvgLit,
                SyntaxKind::IfLit,
                SyntaxKind::MonthLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::DayLit,
                SyntaxKind::RegexLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::SampleLit,
                SyntaxKind::Bang,
                SyntaxKind::Sha1Lit,
                SyntaxKind::BoundLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::CeilLit,
                SyntaxKind::Plus,
                SyntaxKind::YearLit,
                SyntaxKind::NowLit,
            ],
            SyntaxKind::GraphRef => &[SyntaxKind::GraphLit],
            SyntaxKind::TriplesBlock => &[
                SyntaxKind::SqOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Rdfliteral => &[],
            SyntaxKind::TriplesNode => &[SyntaxKind::BrOpen, SyntaxKind::SqOpen],
            SyntaxKind::PathAlternative => &[
                SyntaxKind::BrOpen,
                SyntaxKind::Alit,
                SyntaxKind::Bang,
                SyntaxKind::Hat,
            ],
            SyntaxKind::SubstringExpression => &[SyntaxKind::SubstrLit],
            SyntaxKind::Collection => &[SyntaxKind::BrOpen],
            SyntaxKind::NumericLiteralNegative => &[],
            SyntaxKind::DatasetClause => &[SyntaxKind::FromLit],
            SyntaxKind::Update1 => &[
                SyntaxKind::DeleteDataLit,
                SyntaxKind::AddLit,
                SyntaxKind::DropLit,
                SyntaxKind::ClearLit,
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::DeleteLit,
                SyntaxKind::MoveLit,
                SyntaxKind::CopyLit,
                SyntaxKind::InsertLit,
                SyntaxKind::LoadLit,
                SyntaxKind::CreateLit,
                SyntaxKind::InsertDataLit,
                SyntaxKind::WithLit,
            ],
            SyntaxKind::Move => &[SyntaxKind::MoveLit],
            SyntaxKind::HavingClause => &[SyntaxKind::HavingLit],
            SyntaxKind::Bind => &[SyntaxKind::BindLit],
            SyntaxKind::TriplesNodePath => &[SyntaxKind::SqOpen, SyntaxKind::BrOpen],
            SyntaxKind::ExistsFunc => &[SyntaxKind::ExistsLit],
            SyntaxKind::GraphOrDefault => &[SyntaxKind::DefaultLit, SyntaxKind::GraphLit],
            SyntaxKind::DeleteWhere => &[SyntaxKind::DeleteWhereLit],
            SyntaxKind::BlankNodePropertyListPath => &[SyntaxKind::SqOpen],
            SyntaxKind::GroupOrUnionGraphPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::OrderClause => &[SyntaxKind::OrderLit],
            SyntaxKind::OptionalGraphPattern => &[SyntaxKind::OptionalLit],
            SyntaxKind::FunctionCall => &[],
            SyntaxKind::InsertClause => &[SyntaxKind::InsertLit],
            SyntaxKind::StrReplaceExpression => &[SyntaxKind::ReplaceLit],
            SyntaxKind::PathPrimary => &[SyntaxKind::BrOpen, SyntaxKind::Bang, SyntaxKind::Alit],
            SyntaxKind::Load => &[SyntaxKind::LoadLit],
            SyntaxKind::Clear => &[SyntaxKind::ClearLit],
            SyntaxKind::GroupGraphPattern => &[SyntaxKind::ClOpen],
            SyntaxKind::BrackettedExpression => &[SyntaxKind::BrOpen],
            SyntaxKind::DescribeQuery => &[SyntaxKind::DescribeLit],
            SyntaxKind::ValuesClause => &[SyntaxKind::ValuesLit],
            SyntaxKind::TriplesTemplate => &[
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::GraphPatternNotTriples => &[
                SyntaxKind::MinusLit,
                SyntaxKind::BindLit,
                SyntaxKind::ValuesLit,
                SyntaxKind::FilterLit,
                SyntaxKind::OptionalLit,
                SyntaxKind::ClOpen,
                SyntaxKind::GraphLit,
                SyntaxKind::ServiceLit,
            ],
            SyntaxKind::GroupGraphPatternSub => &[
                SyntaxKind::TrueLit,
                SyntaxKind::FilterLit,
                SyntaxKind::GraphLit,
                SyntaxKind::BindLit,
                SyntaxKind::ClOpen,
                SyntaxKind::OptionalLit,
                SyntaxKind::MinusLit,
                SyntaxKind::ValuesLit,
                SyntaxKind::ServiceLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::Aggregate => &[
                SyntaxKind::CountLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::SumLit,
                SyntaxKind::MaxLit,
                SyntaxKind::AvgLit,
                SyntaxKind::SampleLit,
                SyntaxKind::MinLit,
            ],
            SyntaxKind::Constraint => &[
                SyntaxKind::StrdtLit,
                SyntaxKind::SumLit,
                SyntaxKind::TzLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::MinLit,
                SyntaxKind::DayLit,
                SyntaxKind::SampleLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::UuidLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::CeilLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::MaxLit,
                SyntaxKind::IriLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::CountLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::FloorLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::BoundLit,
                SyntaxKind::RegexLit,
                SyntaxKind::MonthLit,
                SyntaxKind::IfLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::AbsLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::NotLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::ExistsLit,
                SyntaxKind::AvgLit,
                SyntaxKind::LangLit,
                SyntaxKind::RandLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::YearLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::NowLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::StrafterLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::HoursLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::UriLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::RoundLit,
            ],
            SyntaxKind::VarOrIri => &[],
            SyntaxKind::BaseDecl => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::BlankNode => &[],
            SyntaxKind::InsertData => &[SyntaxKind::InsertDataLit],
            SyntaxKind::NumericLiteral => &[],
            SyntaxKind::QueryUnit => &[
                SyntaxKind::SelectLit,
                SyntaxKind::ConstructLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::AskLit,
                SyntaxKind::DescribeLit,
                SyntaxKind::SparqlBaseToken,
            ],
            SyntaxKind::Copy => &[SyntaxKind::CopyLit],
            SyntaxKind::Update => &[
                SyntaxKind::DeleteDataLit,
                SyntaxKind::DropLit,
                SyntaxKind::InsertDataLit,
                SyntaxKind::DeleteLit,
                SyntaxKind::InsertLit,
                SyntaxKind::LoadLit,
                SyntaxKind::DeleteWhereLit,
                SyntaxKind::MoveLit,
                SyntaxKind::WithLit,
                SyntaxKind::SparqlBaseToken,
                SyntaxKind::CreateLit,
                SyntaxKind::AddLit,
                SyntaxKind::SparqlPrefixToken,
                SyntaxKind::ClearLit,
                SyntaxKind::CopyLit,
            ],
            SyntaxKind::BuiltInCall => &[
                SyntaxKind::SecondsLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::NotLit,
                SyntaxKind::CountLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::IriLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::AbsLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::SumLit,
                SyntaxKind::RegexLit,
                SyntaxKind::TzLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::SampleLit,
                SyntaxKind::YearLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::StruuidLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::AvgLit,
                SyntaxKind::MaxLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::StrLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::CeilLit,
                SyntaxKind::RoundLit,
                SyntaxKind::UriLit,
                SyntaxKind::MinLit,
                SyntaxKind::NowLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::IfLit,
                SyntaxKind::UuidLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::LangLit,
                SyntaxKind::RandLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::DayLit,
                SyntaxKind::HoursLit,
                SyntaxKind::MonthLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::StrlenLit,
                SyntaxKind::BoundLit,
            ],
            SyntaxKind::PathEltOrInverse => &[
                SyntaxKind::Hat,
                SyntaxKind::Alit,
                SyntaxKind::Bang,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::OrderCondition => &[
                SyntaxKind::LangmatchesLit,
                SyntaxKind::NowLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::RandLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::AbsLit,
                SyntaxKind::YearLit,
                SyntaxKind::TzLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::UuidLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::IsIriLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::SameTermLit,
                SyntaxKind::RoundLit,
                SyntaxKind::SumLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::SampleLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::CountLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::IriLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::AvgLit,
                SyntaxKind::MonthLit,
                SyntaxKind::DescLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::LangLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::MaxLit,
                SyntaxKind::BoundLit,
                SyntaxKind::NotLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::RegexLit,
                SyntaxKind::HoursLit,
                SyntaxKind::CeilLit,
                SyntaxKind::BrOpen,
                SyntaxKind::Sha384Lit,
                SyntaxKind::UriLit,
                SyntaxKind::MinLit,
                SyntaxKind::IfLit,
                SyntaxKind::AscLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::SubstrLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::DayLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::IsUriLit,
            ],
            SyntaxKind::ConstructTriples => &[
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::PathSequence => &[
                SyntaxKind::Hat,
                SyntaxKind::BrOpen,
                SyntaxKind::Alit,
                SyntaxKind::Bang,
            ],
            SyntaxKind::PropertyListNotEmpty => &[SyntaxKind::Alit],
            SyntaxKind::ConstructTemplate => &[SyntaxKind::ClOpen],
            SyntaxKind::IriOrFunction => &[],
            SyntaxKind::ObjectListPath => &[
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::GraphRefAll => &[
                SyntaxKind::AllLit,
                SyntaxKind::DefaultLit,
                SyntaxKind::GraphLit,
                SyntaxKind::NamedLit,
            ],
            SyntaxKind::SelectQuery => &[SyntaxKind::SelectLit],
            SyntaxKind::BlankNodePropertyList => &[SyntaxKind::SqOpen],
            SyntaxKind::ConstructQuery => &[SyntaxKind::ConstructLit],
            SyntaxKind::NamedGraphClause => &[SyntaxKind::NamedLit],
            SyntaxKind::Filter => &[SyntaxKind::FilterLit],
            SyntaxKind::DefaultGraphClause => &[],
            SyntaxKind::BooleanLiteral => &[SyntaxKind::FalseLit, SyntaxKind::TrueLit],
            SyntaxKind::PropertyList => &[SyntaxKind::Alit],
            SyntaxKind::PrefixedName => &[],
            SyntaxKind::VerbSimple => &[],
            SyntaxKind::ConditionalAndExpression => &[
                SyntaxKind::HoursLit,
                SyntaxKind::SumLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::FloorLit,
                SyntaxKind::RandLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::BoundLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::StrLit,
                SyntaxKind::MonthLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::RoundLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::DayLit,
                SyntaxKind::Plus,
                SyntaxKind::NowLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::TzLit,
                SyntaxKind::AvgLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::AbsLit,
                SyntaxKind::CeilLit,
                SyntaxKind::MaxLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::IriLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::CountLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::UriLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::UuidLit,
                SyntaxKind::TrueLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::SampleLit,
                SyntaxKind::LangLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::FalseLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::NotLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::BrOpen,
                SyntaxKind::RegexLit,
                SyntaxKind::YearLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::SameTermLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::MinLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::Bar,
                SyntaxKind::StrlangLit,
                SyntaxKind::Bang,
                SyntaxKind::StrafterLit,
                SyntaxKind::IfLit,
            ],
            SyntaxKind::Expression => &[
                SyntaxKind::BrOpen,
                SyntaxKind::Sha256Lit,
                SyntaxKind::UuidLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::IfLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::YearLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::IriLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::MinLit,
                SyntaxKind::Bar,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::BoundLit,
                SyntaxKind::BnodeLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::LangLit,
                SyntaxKind::CountLit,
                SyntaxKind::TrueLit,
                SyntaxKind::UriLit,
                SyntaxKind::AvgLit,
                SyntaxKind::Plus,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::CeilLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::ContainsLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::AbsLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::HoursLit,
                SyntaxKind::MaxLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::TzLit,
                SyntaxKind::NowLit,
                SyntaxKind::Bang,
                SyntaxKind::RegexLit,
                SyntaxKind::NotLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::Md5Lit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::SampleLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SumLit,
                SyntaxKind::RandLit,
                SyntaxKind::MonthLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::RoundLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::DayLit,
            ],
            SyntaxKind::HavingCondition => &[
                SyntaxKind::DatatypeLit,
                SyntaxKind::FloorLit,
                SyntaxKind::RegexLit,
                SyntaxKind::AbsLit,
                SyntaxKind::CeilLit,
                SyntaxKind::StrafterLit,
                SyntaxKind::SampleLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::LangLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::BnodeLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::UriLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::IriLit,
                SyntaxKind::SumLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::NowLit,
                SyntaxKind::BoundLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::HoursLit,
                SyntaxKind::UuidLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::TzLit,
                SyntaxKind::YearLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::IsUriLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::RandLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::BrOpen,
                SyntaxKind::DayLit,
                SyntaxKind::AvgLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::StrLit,
                SyntaxKind::ContainsLit,
                SyntaxKind::SameTermLit,
                SyntaxKind::NotLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::MinLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::MonthLit,
                SyntaxKind::MaxLit,
                SyntaxKind::RoundLit,
                SyntaxKind::CountLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::IfLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::TimezoneLit,
            ],
            SyntaxKind::GraphNode => &[
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::Drop => &[SyntaxKind::DropLit],
            SyntaxKind::Add => &[SyntaxKind::AddLit],
            SyntaxKind::GroupClause => &[SyntaxKind::GroupLit],
            SyntaxKind::PropertyListPath => &[
                SyntaxKind::Hat,
                SyntaxKind::Bang,
                SyntaxKind::BrOpen,
                SyntaxKind::Alit,
            ],
            SyntaxKind::VarOrTerm => &[SyntaxKind::FalseLit, SyntaxKind::TrueLit],
            SyntaxKind::Object => &[
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::ObjectList => &[
                SyntaxKind::FalseLit,
                SyntaxKind::TrueLit,
                SyntaxKind::BrOpen,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::RelationalExpression => &[
                SyntaxKind::TzLit,
                SyntaxKind::HoursLit,
                SyntaxKind::YearLit,
                SyntaxKind::NotLit,
                SyntaxKind::CoalesceLit,
                SyntaxKind::ReplaceLit,
                SyntaxKind::NowLit,
                SyntaxKind::StruuidLit,
                SyntaxKind::StrstartsLit,
                SyntaxKind::AvgLit,
                SyntaxKind::SubstrLit,
                SyntaxKind::TrueLit,
                SyntaxKind::LangmatchesLit,
                SyntaxKind::DatatypeLit,
                SyntaxKind::IsIriLit,
                SyntaxKind::ExistsLit,
                SyntaxKind::CountLit,
                SyntaxKind::IsBlankLit,
                SyntaxKind::SecondsLit,
                SyntaxKind::UcaseLit,
                SyntaxKind::TimezoneLit,
                SyntaxKind::AbsLit,
                SyntaxKind::StrlenLit,
                SyntaxKind::IsLiteralLit,
                SyntaxKind::Md5Lit,
                SyntaxKind::StrafterLit,
                SyntaxKind::StrendsLit,
                SyntaxKind::Sha512Lit,
                SyntaxKind::MonthLit,
                SyntaxKind::Plus,
                SyntaxKind::SameTermLit,
                SyntaxKind::StrbeforeLit,
                SyntaxKind::IriLit,
                SyntaxKind::StrlangLit,
                SyntaxKind::RoundLit,
                SyntaxKind::SumLit,
                SyntaxKind::Sha384Lit,
                SyntaxKind::Bang,
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::RandLit,
                SyntaxKind::MaxLit,
                SyntaxKind::Sha256Lit,
                SyntaxKind::IsUriLit,
                SyntaxKind::MinLit,
                SyntaxKind::IsNumericLit,
                SyntaxKind::IfLit,
                SyntaxKind::Bar,
                SyntaxKind::ContainsLit,
                SyntaxKind::GroupConcatLit,
                SyntaxKind::UuidLit,
                SyntaxKind::EncodeForUriLit,
                SyntaxKind::FloorLit,
                SyntaxKind::StrdtLit,
                SyntaxKind::RegexLit,
                SyntaxKind::LcaseLit,
                SyntaxKind::MinutesLit,
                SyntaxKind::StrLit,
                SyntaxKind::CeilLit,
                SyntaxKind::SampleLit,
                SyntaxKind::DayLit,
                SyntaxKind::UriLit,
                SyntaxKind::Sha1Lit,
                SyntaxKind::BnodeLit,
                SyntaxKind::ConcatLit,
                SyntaxKind::BoundLit,
                SyntaxKind::LangLit,
            ],
            SyntaxKind::InlineDataOneVar => &[],
            SyntaxKind::ServiceGraphPattern => &[SyntaxKind::ServiceLit],
            SyntaxKind::PropertyListPathNotEmpty => &[
                SyntaxKind::Alit,
                SyntaxKind::Bang,
                SyntaxKind::BrOpen,
                SyntaxKind::Hat,
            ],
            SyntaxKind::ExpressionList => &[SyntaxKind::BrOpen],
            SyntaxKind::GraphNodePath => &[
                SyntaxKind::BrOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::CollectionPath => &[SyntaxKind::BrOpen],
            SyntaxKind::TriplesSameSubject => &[
                SyntaxKind::SqOpen,
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::TrueLit,
            ],
            SyntaxKind::Modify => &[
                SyntaxKind::DeleteLit,
                SyntaxKind::WithLit,
                SyntaxKind::InsertLit,
            ],
            SyntaxKind::PrefixDecl => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::WhereClause => &[SyntaxKind::ClOpen, SyntaxKind::WhereLit],
            SyntaxKind::SolutionModifier => &[
                SyntaxKind::GroupLit,
                SyntaxKind::OrderLit,
                SyntaxKind::LimitLit,
                SyntaxKind::HavingLit,
                SyntaxKind::OffsetLit,
            ],
            SyntaxKind::TriplesSameSubjectPath => &[
                SyntaxKind::TrueLit,
                SyntaxKind::FalseLit,
                SyntaxKind::SqOpen,
                SyntaxKind::BrOpen,
            ],
            SyntaxKind::QuadsNotTriples => &[SyntaxKind::GraphLit],
            SyntaxKind::Iri => &[],
            SyntaxKind::GraphGraphPattern => &[SyntaxKind::GraphLit],
            SyntaxKind::SubSelect => &[SyntaxKind::SelectLit],
            SyntaxKind::LimitOffsetClauses => &[SyntaxKind::OffsetLit, SyntaxKind::LimitLit],
            SyntaxKind::SelectClause => &[SyntaxKind::SelectLit],
            SyntaxKind::InlineDataFull => &[SyntaxKind::BrOpen],
            SyntaxKind::Quads => &[
                SyntaxKind::TrueLit,
                SyntaxKind::GraphLit,
                SyntaxKind::FalseLit,
                SyntaxKind::BrOpen,
                SyntaxKind::SqOpen,
            ],
            SyntaxKind::OffsetLit => &[SyntaxKind::OffsetLit],
            SyntaxKind::Questionmark => &[SyntaxKind::Questionmark],
            SyntaxKind::SeparatorLit => &[SyntaxKind::SeparatorLit],
            SyntaxKind::NotLit => &[SyntaxKind::NotLit],
            SyntaxKind::NamedLit => &[SyntaxKind::NamedLit],
            SyntaxKind::StrstartsLit => &[SyntaxKind::StrstartsLit],
            SyntaxKind::DecimalNegative => &[SyntaxKind::DecimalNegative],
            SyntaxKind::GroupLit => &[SyntaxKind::GroupLit],
            SyntaxKind::DeleteDataLit => &[SyntaxKind::DeleteDataLit],
            SyntaxKind::ReducedLit => &[SyntaxKind::ReducedLit],
            SyntaxKind::InsertDataLit => &[SyntaxKind::InsertDataLit],
            SyntaxKind::RandLit => &[SyntaxKind::RandLit],
            SyntaxKind::Iriref => &[SyntaxKind::Iriref],
            SyntaxKind::ClOpen => &[SyntaxKind::ClOpen],
            SyntaxKind::CreateLit => &[SyntaxKind::CreateLit],
            SyntaxKind::ByLit => &[SyntaxKind::ByLit],
            SyntaxKind::CoalesceLit => &[SyntaxKind::CoalesceLit],
            SyntaxKind::SameTermLit => &[SyntaxKind::SameTermLit],
            SyntaxKind::IsBlankLit => &[SyntaxKind::IsBlankLit],
            SyntaxKind::BlankNodeLabel => &[SyntaxKind::BlankNodeLabel],
            SyntaxKind::Datatype => &[SyntaxKind::Datatype],
            SyntaxKind::SelectLit => &[SyntaxKind::SelectLit],
            SyntaxKind::DefaultLit => &[SyntaxKind::DefaultLit],
            SyntaxKind::UndefLit => &[SyntaxKind::UndefLit],
            SyntaxKind::IsNumericLit => &[SyntaxKind::IsNumericLit],
            SyntaxKind::CopyLit => &[SyntaxKind::CopyLit],
            SyntaxKind::MinutesLit => &[SyntaxKind::MinutesLit],
            SyntaxKind::PnameLn => &[SyntaxKind::PnameLn],
            SyntaxKind::IsUriLit => &[SyntaxKind::IsUriLit],
            SyntaxKind::AsLit => &[SyntaxKind::AsLit],
            SyntaxKind::WithLit => &[SyntaxKind::WithLit],
            SyntaxKind::RegexLit => &[SyntaxKind::RegexLit],
            SyntaxKind::SampleLit => &[SyntaxKind::SampleLit],
            SyntaxKind::Pipe => &[SyntaxKind::Pipe],
            SyntaxKind::SubstrLit => &[SyntaxKind::SubstrLit],
            SyntaxKind::Langtag => &[SyntaxKind::Langtag],
            SyntaxKind::FilterLit => &[SyntaxKind::FilterLit],
            SyntaxKind::EncodeForUriLit => &[SyntaxKind::EncodeForUriLit],
            SyntaxKind::SecondsLit => &[SyntaxKind::SecondsLit],
            SyntaxKind::RoundLit => &[SyntaxKind::RoundLit],
            SyntaxKind::Star => &[SyntaxKind::Star],
            SyntaxKind::UcaseLit => &[SyntaxKind::UcaseLit],
            SyntaxKind::UnionLit => &[SyntaxKind::UnionLit],
            SyntaxKind::IfLit => &[SyntaxKind::IfLit],
            SyntaxKind::StringLiteralLong1 => &[SyntaxKind::StringLiteralLong1],
            SyntaxKind::ConcatLit => &[SyntaxKind::ConcatLit],
            SyntaxKind::ClearLit => &[SyntaxKind::ClearLit],
            SyntaxKind::AllLit => &[SyntaxKind::AllLit],
            SyntaxKind::DescribeLit => &[SyntaxKind::DescribeLit],
            SyntaxKind::StrbeforeLit => &[SyntaxKind::StrbeforeLit],
            SyntaxKind::MinLit => &[SyntaxKind::MinLit],
            SyntaxKind::SilentLit => &[SyntaxKind::SilentLit],
            SyntaxKind::Lt => &[SyntaxKind::Lt],
            SyntaxKind::Alit => &[SyntaxKind::Alit],
            SyntaxKind::IntoLit => &[SyntaxKind::IntoLit],
            SyntaxKind::Anon => &[SyntaxKind::Anon],
            SyntaxKind::Bar => &[SyntaxKind::Bar],
            SyntaxKind::NowLit => &[SyntaxKind::NowLit],
            SyntaxKind::DayLit => &[SyntaxKind::DayLit],
            SyntaxKind::MaxLit => &[SyntaxKind::MaxLit],
            SyntaxKind::Double => &[SyntaxKind::Double],
            SyntaxKind::Hat => &[SyntaxKind::Hat],
            SyntaxKind::HavingLit => &[SyntaxKind::HavingLit],
            SyntaxKind::TrueLit => &[SyntaxKind::TrueLit],
            SyntaxKind::Lte => &[SyntaxKind::Lte],
            SyntaxKind::ConstructLit => &[SyntaxKind::ConstructLit],
            SyntaxKind::AddLit => &[SyntaxKind::AddLit],
            SyntaxKind::Integer => &[SyntaxKind::Integer],
            SyntaxKind::TimezoneLit => &[SyntaxKind::TimezoneLit],
            SyntaxKind::Var1 => &[SyntaxKind::Var1],
            SyntaxKind::LoadLit => &[SyntaxKind::LoadLit],
            SyntaxKind::OptionalLit => &[SyntaxKind::OptionalLit],
            SyntaxKind::BoundLit => &[SyntaxKind::BoundLit],
            SyntaxKind::Nil => &[SyntaxKind::Nil],
            SyntaxKind::StrlenLit => &[SyntaxKind::StrlenLit],
            SyntaxKind::IriLit => &[SyntaxKind::IriLit],
            SyntaxKind::TzLit => &[SyntaxKind::TzLit],
            SyntaxKind::Sha512Lit => &[SyntaxKind::Sha512Lit],
            SyntaxKind::Stop => &[SyntaxKind::Stop],
            SyntaxKind::Pipe2 => &[SyntaxKind::Pipe2],
            SyntaxKind::LcaseLit => &[SyntaxKind::LcaseLit],
            SyntaxKind::SumLit => &[SyntaxKind::SumLit],
            SyntaxKind::ToLit => &[SyntaxKind::ToLit],
            SyntaxKind::ReplaceLit => &[SyntaxKind::ReplaceLit],
            SyntaxKind::GroupConcatLit => &[SyntaxKind::GroupConcatLit],
            SyntaxKind::StringLiteral1 => &[SyntaxKind::StringLiteral1],
            SyntaxKind::Sha256Lit => &[SyntaxKind::Sha256Lit],
            SyntaxKind::LangmatchesLit => &[SyntaxKind::LangmatchesLit],
            SyntaxKind::StrafterLit => &[SyntaxKind::StrafterLit],
            SyntaxKind::ServiceLit => &[SyntaxKind::ServiceLit],
            SyntaxKind::AbsLit => &[SyntaxKind::AbsLit],
            SyntaxKind::FalseLit => &[SyntaxKind::FalseLit],
            SyntaxKind::Comma => &[SyntaxKind::Comma],
            SyntaxKind::IntegerPositive => &[SyntaxKind::IntegerPositive],
            SyntaxKind::SqOpen => &[SyntaxKind::SqOpen],
            SyntaxKind::AvgLit => &[SyntaxKind::AvgLit],
            SyntaxKind::AskLit => &[SyntaxKind::AskLit],
            SyntaxKind::LangLit => &[SyntaxKind::LangLit],
            SyntaxKind::DeleteLit => &[SyntaxKind::DeleteLit],
            SyntaxKind::StringLiteral2 => &[SyntaxKind::StringLiteral2],
            SyntaxKind::UsingLit => &[SyntaxKind::UsingLit],
            SyntaxKind::HoursLit => &[SyntaxKind::HoursLit],
            SyntaxKind::IsIriLit => &[SyntaxKind::IsIriLit],
            SyntaxKind::StringLiteralLong2 => &[SyntaxKind::StringLiteralLong2],
            SyntaxKind::SqClose => &[SyntaxKind::SqClose],
            SyntaxKind::Var2 => &[SyntaxKind::Var2],
            SyntaxKind::Gt => &[SyntaxKind::Gt],
            SyntaxKind::InLit => &[SyntaxKind::InLit],
            SyntaxKind::UriLit => &[SyntaxKind::UriLit],
            SyntaxKind::IntegerNegative => &[SyntaxKind::IntegerNegative],
            SyntaxKind::DistinctLit => &[SyntaxKind::DistinctLit],
            SyntaxKind::ContainsLit => &[SyntaxKind::ContainsLit],
            SyntaxKind::UuidLit => &[SyntaxKind::UuidLit],
            SyntaxKind::DropLit => &[SyntaxKind::DropLit],
            SyntaxKind::BrOpen => &[SyntaxKind::BrOpen],
            SyntaxKind::FloorLit => &[SyntaxKind::FloorLit],
            SyntaxKind::DecimalPositive => &[SyntaxKind::DecimalPositive],
            SyntaxKind::GraphLit => &[SyntaxKind::GraphLit],
            SyntaxKind::MinusLit => &[SyntaxKind::MinusLit],
            SyntaxKind::StrendsLit => &[SyntaxKind::StrendsLit],
            SyntaxKind::BnodeLit => &[SyntaxKind::BnodeLit],
            SyntaxKind::ExistsLit => &[SyntaxKind::ExistsLit],
            SyntaxKind::FromLit => &[SyntaxKind::FromLit],
            SyntaxKind::CeilLit => &[SyntaxKind::CeilLit],
            SyntaxKind::DeleteWhereLit => &[SyntaxKind::DeleteWhereLit],
            SyntaxKind::Sha1Lit => &[SyntaxKind::Sha1Lit],
            SyntaxKind::Div => &[SyntaxKind::Div],
            SyntaxKind::StrlangLit => &[SyntaxKind::StrlangLit],
            SyntaxKind::Decimal => &[SyntaxKind::Decimal],
            SyntaxKind::Gte => &[SyntaxKind::Gte],
            SyntaxKind::WhereLit => &[SyntaxKind::WhereLit],
            SyntaxKind::Md5Lit => &[SyntaxKind::Md5Lit],
            SyntaxKind::BrClose => &[SyntaxKind::BrClose],
            SyntaxKind::Eq => &[SyntaxKind::Eq],
            SyntaxKind::AscLit => &[SyntaxKind::AscLit],
            SyntaxKind::YearLit => &[SyntaxKind::YearLit],
            SyntaxKind::ValuesLit => &[SyntaxKind::ValuesLit],
            SyntaxKind::ClClose => &[SyntaxKind::ClClose],
            SyntaxKind::Colon => &[SyntaxKind::Colon],
            SyntaxKind::StruuidLit => &[SyntaxKind::StruuidLit],
            SyntaxKind::StrdtLit => &[SyntaxKind::StrdtLit],
            SyntaxKind::MoveLit => &[SyntaxKind::MoveLit],
            SyntaxKind::IsLiteralLit => &[SyntaxKind::IsLiteralLit],
            SyntaxKind::DoublePositive => &[SyntaxKind::DoublePositive],
            SyntaxKind::MonthLit => &[SyntaxKind::MonthLit],
            SyntaxKind::DatatypeLit => &[SyntaxKind::DatatypeLit],
            SyntaxKind::InsertLit => &[SyntaxKind::InsertLit],
            SyntaxKind::SparqlPrefixToken => &[SyntaxKind::SparqlPrefixToken],
            SyntaxKind::BindLit => &[SyntaxKind::BindLit],
            SyntaxKind::StrLit => &[SyntaxKind::StrLit],
            SyntaxKind::CountLit => &[SyntaxKind::CountLit],
            SyntaxKind::Sha384Lit => &[SyntaxKind::Sha384Lit],
            SyntaxKind::DoubleNegative => &[SyntaxKind::DoubleNegative],
            SyntaxKind::Bang => &[SyntaxKind::Bang],
            SyntaxKind::SparqlBaseToken => &[SyntaxKind::SparqlBaseToken],
            SyntaxKind::PnameNs => &[SyntaxKind::PnameNs],
            SyntaxKind::OrderLit => &[SyntaxKind::OrderLit],
            SyntaxKind::LimitLit => &[SyntaxKind::LimitLit],
            SyntaxKind::Amp2 => &[SyntaxKind::Amp2],
            SyntaxKind::DescLit => &[SyntaxKind::DescLit],
            SyntaxKind::Neq => &[SyntaxKind::Neq],
            SyntaxKind::Plus => &[SyntaxKind::Plus],
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
                                state: 15usize,
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
                (SyntaxKind::BaseDecl, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                                state: 2usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereClause,
                                state: 2usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 9usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
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
                                state: 8usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::TriplesTemplate,
                                state: 5usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 11usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 9usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 1usize) => {
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
                                state: 13usize,
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
                                state: 11usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::WhereLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 2usize) => {
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ClClose,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::ConstructQuery, 13usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 13usize,
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
                                state: 11usize,
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
                (SyntaxKind::ConstructQuery, 7usize) => {
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
                (SyntaxKind::ConstructQuery, 15usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::ConstructLit,
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
                (SyntaxKind::DatasetClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::NamedGraphClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::OrderCondition, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::OffsetClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::ValuesClause, 0usize) => {
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
                (SyntaxKind::Load, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::Clear, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::Create, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::Add, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::Copy, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::DeleteWhere, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::Modify, 0usize) => {
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
                (SyntaxKind::DeleteClause, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::UsingClause, 0usize) => {
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
                (SyntaxKind::GraphOrDefault, 0usize) => {
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
                (SyntaxKind::QuadData, 0usize) => {
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
                (SyntaxKind::GroupGraphPattern, 0usize) => {
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
                (SyntaxKind::ServiceGraphPattern, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::Bind, 0usize) => {
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
                (SyntaxKind::InlineData, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::DataBlock, 0usize) => {
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
                (SyntaxKind::InlineDataOneVar, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::ArgList, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::ConstructTemplate, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::ConstructTriples, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::TriplesSameSubject, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::VerbPath, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::PathElt, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::PathEltOrInverse, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                                state: 20usize,
                            }),
                    );
                }
                (SyntaxKind::RelationalExpression, 5usize) => {
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
                (SyntaxKind::RelationalExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::RelationalExpression, 18usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 17usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::InLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::RelationalExpression, 20usize) => {
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
                (SyntaxKind::RelationalExpression, 7usize) => {
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
                (SyntaxKind::RelationalExpression, 11usize) => {
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
                (SyntaxKind::RelationalExpression, 13usize) => {
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
                (SyntaxKind::RelationalExpression, 15usize) => {
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
                (SyntaxKind::RelationalExpression, 9usize) => {
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
                (SyntaxKind::RelationalExpression, 17usize) => {
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
                                state: 5usize,
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
                                state: 7usize,
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
                                state: 9usize,
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
                                state: 11usize,
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
                                state: 13usize,
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
                                state: 15usize,
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
                                state: 18usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::NotLit,
                                state: 0usize,
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
                (SyntaxKind::NumericExpression, 1usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 0usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::AdditiveExpression,
                                state: 16usize,
                            }),
                    );
                }
                (SyntaxKind::NumericExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::AdditiveExpression, 9usize) => {
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
                (SyntaxKind::AdditiveExpression, 11usize) => {
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
                (SyntaxKind::AdditiveExpression, 3usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MultiplicativeExpression,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::AdditiveExpression, 7usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 9usize,
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
                                state: 11usize,
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
                (SyntaxKind::AdditiveExpression, 16usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MultiplicativeExpression,
                                state: 7usize,
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
                                state: 5usize,
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
                (SyntaxKind::AdditiveExpression, 5usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 1usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MultiplicativeExpression,
                                state: 7usize,
                            }),
                    );
                }
                (SyntaxKind::MultiplicativeExpression, 5usize) => {
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
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 3usize,
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
                                state: 5usize,
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
                (SyntaxKind::MultiplicativeExpression, 7usize) => {
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
                (SyntaxKind::MultiplicativeExpression, 3usize) => {
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
                (SyntaxKind::UnaryExpression, 0usize) => {
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
                (SyntaxKind::PrimaryExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::BrackettedExpression, 0usize) => {
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
                (SyntaxKind::BuiltInCall, 84usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 83usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 77usize) => {
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
                (SyntaxKind::BuiltInCall, 95usize) => {
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
                (SyntaxKind::BuiltInCall, 172usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 171usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 108usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 107usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 71usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 70usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 182usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 181usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 7usize) => {
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
                (SyntaxKind::BuiltInCall, 167usize) => {
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
                (SyntaxKind::BuiltInCall, 73usize) => {
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
                (SyntaxKind::BuiltInCall, 46usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 45usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 21usize) => {
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
                (SyntaxKind::BuiltInCall, 178usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 177usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 210usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 209usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 9usize) => {
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
                (SyntaxKind::BuiltInCall, 57usize) => {
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
                (SyntaxKind::BuiltInCall, 96usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 95usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 86usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 85usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 121usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 120usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 136usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 135usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 31usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 30usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 47usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 46usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 195usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 194usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 199usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 198usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 70usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 69usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 30usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 29usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 141usize) => {
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
                (SyntaxKind::BuiltInCall, 4usize) => {
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
                (SyntaxKind::BuiltInCall, 150usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 149usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 109usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 108usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 197usize) => {
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
                (SyntaxKind::BuiltInCall, 190usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 189usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 203usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 202usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 62usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 61usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 101usize) => {
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
                (SyntaxKind::BuiltInCall, 207usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 206usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 79usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 78usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 155usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 154usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 15usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 14usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 18usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 17usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 170usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 169usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 81usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 80usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 125usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 124usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 165usize) => {
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
                (SyntaxKind::BuiltInCall, 193usize) => {
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
                (SyntaxKind::BuiltInCall, 123usize) => {
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
                (SyntaxKind::BuiltInCall, 132usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 131usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 211usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 210usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 83usize) => {
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
                (SyntaxKind::BuiltInCall, 5usize) => {
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
                (SyntaxKind::BuiltInCall, 175usize) => {
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
                (SyntaxKind::BuiltInCall, 181usize) => {
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
                (SyntaxKind::BuiltInCall, 12usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 11usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 98usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 97usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 29usize) => {
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
                (SyntaxKind::BuiltInCall, 158usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 157usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 209usize) => {
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
                (SyntaxKind::BuiltInCall, 11usize) => {
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
                (SyntaxKind::BuiltInCall, 25usize) => {
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
                (SyntaxKind::BuiltInCall, 147usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 146usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 163usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 162usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 184usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 183usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 43usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 42usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 22usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 21usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Var,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 93usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 92usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 26usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 25usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 67usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 66usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 189usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 188usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 143usize) => {
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
                (SyntaxKind::BuiltInCall, 127usize) => {
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
                (SyntaxKind::BuiltInCall, 119usize) => {
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
                                state: 60usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 74usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 73usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 112usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 111usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 66usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 65usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 97usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 96usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 49usize) => {
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
                (SyntaxKind::BuiltInCall, 35usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 34usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 13usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 12usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 145usize) => {
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
                (SyntaxKind::BuiltInCall, 157usize) => {
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
                (SyntaxKind::BuiltInCall, 65usize) => {
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
                (SyntaxKind::BuiltInCall, 169usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 168usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 173usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 172usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 177usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 176usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 179usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 178usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 149usize) => {
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
                (SyntaxKind::BuiltInCall, 87usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 86usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 146usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 145usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 183usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 182usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 187usize) => {
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
                (SyntaxKind::BuiltInCall, 188usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 187usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 75usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 74usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 206usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 205usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 54usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 53usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
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
                (SyntaxKind::BuiltInCall, 8usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 7usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 45usize) => {
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
                (SyntaxKind::BuiltInCall, 50usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 49usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 105usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 104usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 162usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 161usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 104usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 103usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 19usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 18usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 60usize) => {
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
                (SyntaxKind::BuiltInCall, 133usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 132usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 135usize) => {
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
                (SyntaxKind::BuiltInCall, 102usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 101usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 120usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 119usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 131usize) => {
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
                (SyntaxKind::BuiltInCall, 53usize) => {
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
                (SyntaxKind::BuiltInCall, 154usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 153usize,
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
                                state: 1usize,
                            }),
                    );
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 5usize,
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
                                state: 9usize,
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
                                state: 15usize,
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
                                state: 19usize,
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
                                state: 23usize,
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
                                state: 27usize,
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
                                state: 31usize,
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
                                state: 33usize,
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
                                state: 39usize,
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
                                state: 43usize,
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
                                state: 47usize,
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
                                state: 51usize,
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
                                state: 55usize,
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
                                state: 57usize,
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
                                state: 62usize,
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
                                state: 67usize,
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
                                state: 71usize,
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
                                state: 75usize,
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
                                state: 81usize,
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
                                state: 87usize,
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
                                state: 93usize,
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
                                state: 99usize,
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
                                state: 105usize,
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
                                state: 109usize,
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
                                state: 113usize,
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
                                state: 117usize,
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
                                state: 121usize,
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
                                state: 125usize,
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
                                state: 129usize,
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
                                state: 133usize,
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
                                state: 137usize,
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
                                state: 139usize,
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
                                state: 141usize,
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
                                state: 143usize,
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
                                state: 147usize,
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
                                state: 151usize,
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
                                state: 155usize,
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
                                state: 159usize,
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
                                state: 163usize,
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
                                state: 165usize,
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
                                state: 173usize,
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
                                state: 179usize,
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
                                state: 185usize,
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
                                state: 191usize,
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
                                state: 195usize,
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
                                state: 199usize,
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
                                state: 203usize,
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
                                state: 207usize,
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
                                state: 211usize,
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
                (SyntaxKind::BuiltInCall, 42usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 41usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 153usize) => {
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
                (SyntaxKind::BuiltInCall, 176usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 175usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 201usize) => {
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
                (SyntaxKind::BuiltInCall, 107usize) => {
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
                (SyntaxKind::BuiltInCall, 117usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 116usize,
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
                (SyntaxKind::BuiltInCall, 69usize) => {
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
                (SyntaxKind::BuiltInCall, 171usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 170usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 194usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 193usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 198usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 197usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 91usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 90usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 124usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 123usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 17usize) => {
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
                (SyntaxKind::BuiltInCall, 41usize) => {
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
                (SyntaxKind::BuiltInCall, 14usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 13usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 51usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 50usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 89usize) => {
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
                (SyntaxKind::BuiltInCall, 128usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 127usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
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
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 139usize) => {
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
                (SyntaxKind::BuiltInCall, 205usize) => {
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
                (SyntaxKind::BuiltInCall, 23usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 22usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 99usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 98usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 161usize) => {
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
                (SyntaxKind::BuiltInCall, 129usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 128usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 103usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 102usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 78usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 77usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 151usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 150usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 80usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 79usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 90usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 89usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 202usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 201usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 34usize) => {
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
                (SyntaxKind::BuiltInCall, 85usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 84usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Comma,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 92usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 91usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 137usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 136usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 33usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 35usize,
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
                (SyntaxKind::BuiltInCall, 111usize) => {
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
                (SyntaxKind::BuiltInCall, 159usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 158usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 115usize) => {
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
                (SyntaxKind::BuiltInCall, 39usize) => {
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
                (SyntaxKind::BuiltInCall, 168usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 167usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 185usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 184usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 27usize) => {
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
                (SyntaxKind::BuiltInCall, 191usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 190usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 116usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 115usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::BuiltInCall, 113usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 112usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::RegexExpression, 0usize) => {
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
                (SyntaxKind::StrReplaceExpression, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
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
                (SyntaxKind::NotExistsFunc, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::Aggregate, 49usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 47usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 14usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 12usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 36usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 34usize,
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
                                state: 35usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DistinctLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 35usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 34usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 43usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 42usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Eq,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 32usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 30usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
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
                (SyntaxKind::Aggregate, 11usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 10usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 18usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 16usize,
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
                                state: 17usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DistinctLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 20usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 18usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 2usize) => {
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
                (SyntaxKind::Aggregate, 40usize) => {
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
                (SyntaxKind::Aggregate, 29usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 28usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 42usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 40usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::MyString,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 44usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 43usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::SeparatorLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 34usize) => {
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
                (SyntaxKind::Aggregate, 12usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 10usize,
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
                                state: 11usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DistinctLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 17usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 16usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 38usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 36usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 46usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 41usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 28usize) => {
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
                (SyntaxKind::Aggregate, 22usize) => {
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
                (SyntaxKind::Aggregate, 3usize) => {
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
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 26usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 24usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::BrOpen,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 47usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 41usize,
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
                                state: 46usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DistinctLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 1usize) => {
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
                                state: 14usize,
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
                                state: 20usize,
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
                                state: 26usize,
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
                                state: 32usize,
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
                                state: 38usize,
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
                                state: 49usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::GroupConcatLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 16usize) => {
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
                (SyntaxKind::Aggregate, 41usize) => {
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
                                state: 44usize,
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
                (SyntaxKind::Aggregate, 10usize) => {
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
                (SyntaxKind::Aggregate, 6usize) => {
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
                (SyntaxKind::Aggregate, 23usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 22usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::Expression,
                                state: 1usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 24usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 22usize,
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
                                state: 23usize,
                            })
                            .push(Rule {
                                kind: SyntaxKind::DistinctLit,
                                state: 0usize,
                            }),
                    );
                }
                (SyntaxKind::Aggregate, 30usize) => {
                    state.add_element(
                        element
                            .pop_push(Rule {
                                kind: self.kind,
                                state: 28usize,
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
                                state: 29usize,
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
                (SyntaxKind::NumericLiteralUnsigned, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::PrefixedName, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
                }
                (SyntaxKind::BlankNode, 0usize) => {
                    if let Some(parent) = element.pop() {
                        state.add_element(parent);
                    }
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
                (SyntaxKind::OffsetLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::OffsetLit, 10isize);
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
                (SyntaxKind::SeparatorLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SeparatorLit, 10isize);
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
                (SyntaxKind::NamedLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::NamedLit, 10isize);
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
                (SyntaxKind::DecimalNegative, _) => {
                    let added = state.expect_as(element, SyntaxKind::DecimalNegative, 2isize);
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
                (SyntaxKind::DeleteDataLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DeleteDataLit, 10isize);
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
                (SyntaxKind::InsertDataLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::InsertDataLit, 10isize);
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
                (SyntaxKind::Iriref, _) => {
                    let added = state.expect_as(element, SyntaxKind::Iriref, 2isize);
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
                (SyntaxKind::CreateLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CreateLit, 10isize);
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
                (SyntaxKind::CoalesceLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CoalesceLit, 10isize);
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
                (SyntaxKind::IsBlankLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsBlankLit, 10isize);
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
                (SyntaxKind::Datatype, _) => {
                    let added = state.expect_as(element, SyntaxKind::Datatype, 10isize);
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
                (SyntaxKind::DefaultLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DefaultLit, 10isize);
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
                (SyntaxKind::IsNumericLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsNumericLit, 10isize);
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
                (SyntaxKind::MinutesLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MinutesLit, 10isize);
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
                (SyntaxKind::IsUriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsUriLit, 10isize);
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
                (SyntaxKind::WithLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::WithLit, 10isize);
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
                (SyntaxKind::SampleLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SampleLit, 10isize);
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
                (SyntaxKind::SubstrLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SubstrLit, 10isize);
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
                (SyntaxKind::FilterLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FilterLit, 10isize);
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
                (SyntaxKind::SecondsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::SecondsLit, 10isize);
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
                (SyntaxKind::Star, _) => {
                    let added = state.expect_as(element, SyntaxKind::Star, 10isize);
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
                (SyntaxKind::UnionLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UnionLit, 10isize);
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
                (SyntaxKind::StringLiteralLong1, _) => {
                    let added = state.expect_as(element, SyntaxKind::StringLiteralLong1, 2isize);
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
                (SyntaxKind::ClearLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ClearLit, 10isize);
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
                (SyntaxKind::DescribeLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DescribeLit, 10isize);
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
                (SyntaxKind::MinLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MinLit, 10isize);
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
                (SyntaxKind::Lt, _) => {
                    let added = state.expect_as(element, SyntaxKind::Lt, 10isize);
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
                (SyntaxKind::IntoLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IntoLit, 10isize);
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
                (SyntaxKind::Bar, _) => {
                    let added = state.expect_as(element, SyntaxKind::Bar, 10isize);
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
                (SyntaxKind::DayLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DayLit, 10isize);
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
                (SyntaxKind::Double, _) => {
                    let added = state.expect_as(element, SyntaxKind::Double, 2isize);
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
                (SyntaxKind::HavingLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::HavingLit, 10isize);
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
                (SyntaxKind::Lte, _) => {
                    let added = state.expect_as(element, SyntaxKind::Lte, 10isize);
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
                (SyntaxKind::AddLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AddLit, 10isize);
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
                (SyntaxKind::TimezoneLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::TimezoneLit, 10isize);
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
                (SyntaxKind::LoadLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LoadLit, 10isize);
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
                (SyntaxKind::BoundLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::BoundLit, 10isize);
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
                (SyntaxKind::StrlenLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrlenLit, 10isize);
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
                (SyntaxKind::TzLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::TzLit, 10isize);
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
                (SyntaxKind::Stop, _) => {
                    let added = state.expect_as(element, SyntaxKind::Stop, 10isize);
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
                (SyntaxKind::LcaseLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LcaseLit, 10isize);
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
                (SyntaxKind::ToLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::ToLit, 10isize);
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
                (SyntaxKind::GroupConcatLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::GroupConcatLit, 10isize);
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
                (SyntaxKind::Sha256Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Sha256Lit, 10isize);
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
                (SyntaxKind::StrafterLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrafterLit, 10isize);
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
                (SyntaxKind::AbsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AbsLit, 10isize);
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
                (SyntaxKind::Comma, _) => {
                    let added = state.expect_as(element, SyntaxKind::Comma, 2isize);
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
                (SyntaxKind::SqOpen, _) => {
                    let added = state.expect_as(element, SyntaxKind::SqOpen, 8isize);
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
                (SyntaxKind::AskLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::AskLit, 10isize);
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
                (SyntaxKind::DeleteLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DeleteLit, 10isize);
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
                (SyntaxKind::UsingLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UsingLit, 10isize);
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
                (SyntaxKind::IsIriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::IsIriLit, 10isize);
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
                (SyntaxKind::SqClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::SqClose, 8isize);
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
                (SyntaxKind::Gt, _) => {
                    let added = state.expect_as(element, SyntaxKind::Gt, 10isize);
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
                (SyntaxKind::UriLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::UriLit, 10isize);
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
                (SyntaxKind::DropLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DropLit, 10isize);
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
                (SyntaxKind::FloorLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::FloorLit, 10isize);
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
                (SyntaxKind::GraphLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::GraphLit, 10isize);
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
                (SyntaxKind::StrendsLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrendsLit, 10isize);
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
                (SyntaxKind::CeilLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::CeilLit, 10isize);
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
                (SyntaxKind::Sha1Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Sha1Lit, 10isize);
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
                (SyntaxKind::StrlangLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrlangLit, 10isize);
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
                (SyntaxKind::Gte, _) => {
                    let added = state.expect_as(element, SyntaxKind::Gte, 10isize);
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
                (SyntaxKind::Md5Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Md5Lit, 10isize);
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
                (SyntaxKind::Eq, _) => {
                    let added = state.expect_as(element, SyntaxKind::Eq, 10isize);
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
                (SyntaxKind::YearLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::YearLit, 10isize);
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
                (SyntaxKind::ClClose, _) => {
                    let added = state.expect_as(element, SyntaxKind::ClClose, 8isize);
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
                (SyntaxKind::StruuidLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StruuidLit, 10isize);
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
                (SyntaxKind::MoveLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::MoveLit, 10isize);
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
                (SyntaxKind::DoublePositive, _) => {
                    let added = state.expect_as(element, SyntaxKind::DoublePositive, 2isize);
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
                (SyntaxKind::DatatypeLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DatatypeLit, 10isize);
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
                (SyntaxKind::SparqlPrefixToken, _) => {
                    let added = state.expect_as(element, SyntaxKind::SparqlPrefixToken, 100isize);
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
                (SyntaxKind::StrLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::StrLit, 10isize);
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
                (SyntaxKind::Sha384Lit, _) => {
                    let added = state.expect_as(element, SyntaxKind::Sha384Lit, 10isize);
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
                (SyntaxKind::Bang, _) => {
                    let added = state.expect_as(element, SyntaxKind::Bang, 10isize);
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
                (SyntaxKind::PnameNs, _) => {
                    let added = state.expect_as(element, SyntaxKind::PnameNs, 2isize);
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
                (SyntaxKind::LimitLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::LimitLit, 10isize);
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
                (SyntaxKind::DescLit, _) => {
                    let added = state.expect_as(element, SyntaxKind::DescLit, 10isize);
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
                (SyntaxKind::Plus, _) => {
                    let added = state.expect_as(element, SyntaxKind::Plus, 10isize);
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
