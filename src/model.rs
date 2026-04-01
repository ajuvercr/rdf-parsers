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
    pub lang: Option<String>,
    pub ty: Option<NamedNode>,
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
            (None, Some(t)) => write!(f, "{}{}{}^^{}", quote, self.value, quote, t),
            (Some(l), None) => write!(f, "{}{}{}@{}", quote, self.value, quote, l),
            (Some(l), Some(t)) => write!(f, "{}{}{}@{}^^{}", quote, self.value, quote, l, t),
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
    },
    A(usize),
    Invalid,
}

impl Display for NamedNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedNode::Full(x, _) => write!(f, "<{}>", x),
            NamedNode::Prefixed {
                prefix,
                value,
                idx: _,
            } => write!(f, "{}:{}", prefix, value),
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
    Invalid,
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
}

impl Display for PO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.predicate.value(), self.object[0].value())?;

        for po in &self.object[1..] {
            write!(f, ", {}", po.value())?;
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
    pub base: Option<Spanned<Base>>,
    pub prefixes: Vec<Spanned<TurtlePrefix>>,
    pub triples: Vec<Spanned<Triple>>,
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
            base,
            prefixes,
            triples,
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
