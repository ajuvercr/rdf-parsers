use std::{fmt::Display, ops::Range};

use crate::Spanned;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StringStyle {
    DoubleLong,
    Double,
    SingleLong,
    Single,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable(pub String, pub usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    RDF(RDFLiteral),
    Boolean(bool),
    Numeric(String),
}

impl Literal {
    pub fn plain_string(&self) -> String {
        match self {
            Literal::RDF(s) => s.plain_string(),
            Literal::Boolean(x) => x.to_string(),
            Literal::Numeric(x) => x.clone(),
        }
    }
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::RDF(x) => x.fmt(f),
            Literal::Boolean(x) => write!(f, "{}", x),
            Literal::Numeric(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RDFLiteral {
    pub value: String,
    pub quote_style: StringStyle,
    /// Language tag (without the leading `@`), spanned over the `@lang` token.
    pub lang: Option<Spanned<String>>,
    /// Base direction of a directional language-tagged string (RDF 1.2,
    /// `"..."@en--ltr`).  Only set together with `lang`.
    pub direction: Option<Direction>,
    /// Datatype IRI, spanned over the `^^<iri>` datatype term.
    pub ty: Option<Spanned<NamedNode>>,
    // Span of tokens
    pub idx: usize,
    pub len: usize,
}

impl RDFLiteral {
    pub fn plain_string(&self) -> String {
        self.value.to_string()
    }
}

impl Display for RDFLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let quote = match self.quote_style {
            StringStyle::DoubleLong => "\"\"\"",
            StringStyle::Double => "\"",
            StringStyle::SingleLong => "'''",
            StringStyle::Single => "'",
        };
        match (&self.lang, &self.ty) {
            (None, None) => write!(f, "{}{}{}", quote, self.value, quote),
            (None, Some(t)) => write!(f, "{}{}{}^^{}", quote, self.value, quote, t.value()),
            (Some(l), None) => write!(f, "{}{}{}@{}", quote, self.value, quote, l.value()),
            (Some(l), Some(t)) => {
                write!(
                    f,
                    "{}{}{}@{}^^{}",
                    quote,
                    self.value,
                    quote,
                    l.value(),
                    t.value()
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NamedNode {
    Full(String, usize),
    Prefixed {
        prefix: String,
        value: String,
        idx: usize,
        /// Pre-resolved absolute IRI, set by parsers (currently JSON-LD) that
        /// expand IRIs against a context whose semantics are *not* equivalent to
        /// plain prefix substitution (whole-term precedence, scoped contexts,
        /// `@vocab`). When present, downstream consumers should use this value
        /// instead of re-expanding `prefix`/`value` against the prefix map.
        /// `None` for the syntactic parsers (Turtle/TriG/SPARQL/N3), which leave
        /// expansion to the consumer.
        computed: Option<String>,
    },
    A(usize),
    Invalid,
}

impl Display for NamedNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedNode::Full(x, _) => write!(f, "<{}>", x),
            NamedNode::Prefixed { prefix, value, .. } => {
                // A JSON-LD whole-term compact IRI is stored as `Prefixed(term, "")`
                // where `term` already contains the ':'; render it verbatim rather
                // than appending a spurious trailing colon.
                if value.is_empty() && prefix.contains(':') {
                    write!(f, "{}", prefix)
                } else {
                    write!(f, "{}:{}", prefix, value)
                }
            }
            NamedNode::A(_) => write!(f, "a"),
            NamedNode::Invalid => write!(f, "invalid"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BlankNode {
    Named(String, usize),
    Unnamed(Vec<Spanned<PO>>, usize, usize),
    Invalid,
}

impl Display for BlankNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlankNode::Named(x, _) => write!(f, "_:{}", x),
            BlankNode::Unnamed(pos, _, _) => {
                if pos.len() == 0 {
                    write!(f, "[ ]")
                } else {
                    write!(f, "[ ")?;

                    for po in pos {
                        write!(f, "{} ;", po.value())?;
                    }

                    write!(f, " ]")
                }
            }
            BlankNode::Invalid => write!(f, "invalid"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Literal(Literal),
    BlankNode(BlankNode),
    NamedNode(NamedNode),
    Collection(Vec<Spanned<Term>>),
    Variable(Variable),
    /// An RDF 1.2 triple term, `<<( s p o )>>`: the triple itself as a
    /// term.  It does not assert the triple.
    TripleTerm(Box<TripleTerm>),
    /// An RDF 1.2 reified triple, `<< s p o ~r >>`: denotes its reifier `r`
    /// (a fresh blank node when there is none) and states
    /// `r rdf:reifies <<( s p o )>>`.  It does not assert the triple either.
    Reified(Box<ReifiedTriple>),
    Invalid,
}

/// A triple used as a term (see `Term::TripleTerm` and `Term::Reified`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TripleTerm {
    pub subject: Spanned<Term>,
    pub predicate: Spanned<Term>,
    pub object: Spanned<Term>,
}

/// `<< s p o ~r >>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReifiedTriple {
    pub triple: TripleTerm,
    pub reifier: Option<Spanned<Reifier>>,
}

/// `~ r`, or a bare `~` (`id: None`): a fresh blank node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reifier {
    pub id: Option<Spanned<Term>>,
}

/// What follows an object in RDF 1.2: `:s :p :o ~:r {| :q :v |}`.  A
/// reifier names a reification of the triple; a block describes the
/// reifier before it, or a fresh one when there is none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Annotation {
    Reifier(Reifier),
    Block(Vec<Spanned<PO>>),
}

/// Base direction of a language-tagged string (RDF 1.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Ltr,
    Rtl,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
        })
    }
}

impl Display for TripleTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.subject.value(),
            self.predicate.value(),
            self.object.value()
        )
    }
}

impl Display for Reifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.id {
            Some(id) => write!(f, "~ {}", id.value()),
            None => f.write_str("~"),
        }
    }
}

impl Display for Annotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Annotation::Reifier(r) => r.fmt(f),
            Annotation::Block(pos) => {
                f.write_str("{| ")?;
                for (i, po) in pos.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" ; ")?;
                    }
                    po.value().fmt(f)?;
                }
                f.write_str(" |}")
            }
        }
    }
}

impl Term {
    pub fn named_node(&self) -> Option<&NamedNode> {
        match self {
            Term::NamedNode(nn) => Some(&nn),
            _ => None,
        }
    }

    pub fn is_subject(&self) -> bool {
        match self {
            Term::BlankNode(_) => true,
            Term::Variable(_) => true,
            Term::NamedNode(NamedNode::A(_)) => false,
            Term::NamedNode(_) => true,
            Term::Invalid => true,
            Term::Collection(_) => true,
            Term::Reified(_) => true,
            _ => false,
        }
    }
    pub fn is_predicate(&self) -> bool {
        match self {
            Term::NamedNode(_) => true,
            Term::Variable(_) => true,
            Term::Invalid => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Term::NamedNode(NamedNode::A(_)) => false,
            Term::Variable(_) => true,
            Term::Invalid => true,
            Term::Collection(_) => true,
            _ => true,
        }
    }
    pub fn is_variable(&self) -> bool {
        match self {
            Term::Variable(_) => true,
            Term::Invalid => true,
            _ => false,
        }
    }
    pub fn ty(&self) -> &'static str {
        match self {
            Term::Literal(_) => "literal",
            Term::BlankNode(_) => "blank node",
            Term::NamedNode(_) => "named node",
            Term::Collection(_) => "collection",
            Term::Invalid => "invalid",
            Term::Variable(_) => "variable",
            Term::TripleTerm(_) => "triple term",
            Term::Reified(_) => "reified triple",
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Literal(l) => l.fmt(f),
            Term::BlankNode(b) => b.fmt(f),
            Term::NamedNode(n) => n.fmt(f),
            Term::Collection(n) => {
                write!(f, "( ")?;
                for l in n {
                    l.fmt(f)?;
                }
                write!(f, "  )")?;
                Ok(())
            }
            Term::Invalid => write!(f, "invalid"),
            Term::Variable(x) => write!(f, "?{}", x.0),
            Term::TripleTerm(t) => write!(f, "<<( {} )>>", t),
            Term::Reified(r) => match &r.reifier {
                Some(reifier) => write!(f, "<< {} {} >>", r.triple, reifier.value()),
                None => write!(f, "<< {} >>", r.triple),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Triple {
    pub subject: Spanned<Term>,
    pub po: Vec<Spanned<PO>>,
    pub graph: Option<Spanned<Term>>,
}

impl Display for Triple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(graph) = &self.graph {
            write!(f, "GRAPH {} {{\n  ", graph.value())?;
        }

        write!(f, "{}", self.subject.value())?;

        if !self.po.is_empty() {
            write!(f, " {}", self.po[0].value())?;
            for po in &self.po[1..] {
                if self.graph.is_some() {
                    write!(f, ";\n    {}", po.value())?;
                } else {
                    write!(f, ";\n  {}", po.value())?;
                }
            }
        }

        write!(f, " .\n")?;

        if self.graph.is_some() {
            write!(f, "}}\n")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PO {
    pub predicate: Spanned<Term>,
    pub object: Vec<Spanned<Term>>,
    /// RDF 1.2 annotations of each object, in order: `annotations[i]`
    /// belongs to `object[i]`.  Empty when no object has any.
    pub annotations: Vec<Vec<Spanned<Annotation>>>,
}

impl PO {
    pub fn new(predicate: Spanned<Term>, object: Vec<Spanned<Term>>) -> Self {
        Self {
            predicate,
            object,
            annotations: Vec::new(),
        }
    }

    /// The annotations of object `i` (empty if it has none).
    pub fn annotations_of(&self, i: usize) -> &[Spanned<Annotation>] {
        self.annotations.get(i).map_or(&[], |a| &a[..])
    }
}

impl Display for PO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.predicate.value())?;
        if self.object.is_empty() {
            write!(f, " undefined")?;
        }
        for (i, o) in self.object.iter().enumerate() {
            write!(f, "{} {}", if i == 0 { "" } else { "," }, o.value())?;
            for a in self.annotations_of(i) {
                write!(f, " {}", a.value())?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Base(pub Range<usize>, pub Spanned<NamedNode>);
impl Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@base {} .", self.1.value())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurtlePrefix {
    pub span: Range<usize>,
    pub prefix: Spanned<String>,
    pub value: Spanned<NamedNode>,
}

impl Display for TurtlePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "@prefix {}: {} .",
            self.prefix.value(),
            self.value.value()
        )
    }
}

#[derive(Debug)]
pub enum TurtleSimpleError {
    UnexpectedBase(&'static str),
    UnexpectedBaseString(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Turtle {
    /// The last `@base` / `BASE` directive.  See `bases` for all of them.
    pub base: Option<Spanned<Base>>,
    /// Every `@base` / `BASE` directive, in document order.  A base applies
    /// to what follows it, so a document that changes its base needs all of
    /// them (see `resolve::resolve`).
    pub bases: Vec<Spanned<Base>>,
    /// `VERSION` / `@version` directives (RDF 1.2), with the version string.
    pub versions: Vec<Spanned<String>>,
    pub prefixes: Vec<Spanned<TurtlePrefix>>,
    pub triples: Vec<Spanned<Triple>>,
    /// Document base URL for prefix/IRI resolution. Set by the language server
    /// after parsing; not populated by the A* parser or CST converter.
    pub set_base: Option<String>,
}

impl Turtle {
    pub fn empty() -> Self {
        Self::new(None, Vec::new(), Vec::new())
    }

    pub fn new(
        base: Option<Spanned<Base>>,
        prefixes: Vec<Spanned<TurtlePrefix>>,
        triples: Vec<Spanned<Triple>>,
    ) -> Self {
        Self {
            bases: base.iter().cloned().collect(),
            base,
            prefixes,
            triples,
            versions: Vec::new(),
            set_base: None,
        }
    }

    /// A document with every base directive, in document order.
    pub fn with_bases(
        bases: Vec<Spanned<Base>>,
        prefixes: Vec<Spanned<TurtlePrefix>>,
        triples: Vec<Spanned<Triple>>,
    ) -> Self {
        Self {
            base: bases.last().cloned(),
            bases,
            prefixes,
            triples,
            versions: Vec::new(),
            set_base: None,
        }
    }
}

impl Display for Turtle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(b) = &self.base {
            writeln!(f, "{}", b.value())?;
        }

        self.prefixes
            .iter()
            .map(|x| x.value())
            .try_for_each(|x| writeln!(f, "{}", x))?;

        self.triples
            .iter()
            .map(|x| x.value())
            .try_for_each(|x| writeln!(f, "{}", x))?;

        Ok(())
    }
}
