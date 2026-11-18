//! From the syntactic document model to RDF: absolute IRIs, unescaped
//! strings, generated blank nodes, and the errors RDF 1.1 Turtle and TriG
//! require on top of the grammar.
//!
//! The parsers accept some documents that are syntactically fine but not
//! valid RDF: an escape that encodes a surrogate (`"\ud800"`), an IRI that
//! escapes a character IRIs may not contain (`<a\u0020b>`), a prefix that is
//! used before (or without) being declared.  [`resolve`] reports those, with
//! the span of the offending term, and produces the quads for everything
//! else.  Directives apply in document order: a `@base` or `@prefix` affects
//! the statements after it, and a relative `@base` or prefix IRI is resolved
//! against the base in effect where it appears.
//!
//! ```
//! use rdf_parsers::{parse, resolve::resolve, turtle::{self, Lang, Rule, SyntaxKind}};
//!
//! let src = "@base <http://ex/> . <s> <p> \"o\" . @base <sub/> . <s> <p> <o> .";
//! let (parse, _) = parse(Rule::new(SyntaxKind::TurtleDoc), src);
//! let doc = turtle::convert::convert(&parse.syntax::<Lang>());
//! let rdf = resolve(&doc, None);
//! assert!(rdf.errors.is_empty());
//! assert_eq!(rdf.quads[1].object.to_string(), "<http://ex/sub/o>");
//! ```

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::ops::Range;

use crate::Spanned;
use crate::model::*;

const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
const XSD: &str = "http://www.w3.org/2001/XMLSchema#";

/// An RDF term.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    Iri(String),
    /// A blank node label, unique within one [`resolve`] result.
    Blank(String),
    Literal {
        lexical: String,
        /// Absolute datatype IRI (`rdf:langString` for language-tagged
        /// strings, `rdf:dirLangString` when they have a direction).
        datatype: String,
        /// Lowercased language tag.
        language: Option<String>,
        /// Base direction (RDF 1.2).
        direction: Option<Direction>,
    },
    /// An RDF 1.2 triple term: subject, predicate, object.
    Triple(Box<(Node, Node, Node)>),
}

/// A triple, in a named graph or (`graph: None`) the default graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Quad {
    pub subject: Node,
    pub predicate: Node,
    pub object: Node,
    pub graph: Option<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// A prefixed name whose prefix is not declared before it.
    UndefinedPrefix(String),
    /// A `\u`/`\U` escape that does not encode a Unicode scalar value
    /// (a surrogate, or above U+10FFFF), with the escape's text.
    InvalidEscape(String),
    /// An IRI that contains, once unescaped, a character IRIs may not
    /// contain (control characters, space, `<>"{}|^`\`), with the IRI.
    InvalidIri(String),
    /// A term the parser could not recover (an error node in the tree).
    Invalid,
    /// A triple term whose subject is not an IRI or blank node, or whose
    /// predicate is not an IRI (RDF 1.2).
    InvalidTripleTerm,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::UndefinedPrefix(p) => write!(f, "undefined prefix '{p}:'"),
            ErrorKind::InvalidEscape(e) => write!(f, "'{e}' is not a valid character escape"),
            ErrorKind::InvalidIri(i) => write!(f, "'{i}' is not a valid IRI"),
            ErrorKind::Invalid => write!(f, "invalid term"),
            ErrorKind::InvalidTripleTerm => {
                write!(
                    f,
                    "a triple term needs an IRI or blank node subject and an IRI predicate"
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
    /// Byte range of the term in the source text.
    pub span: Range<usize>,
}

/// The quads of a document, and the errors that make it invalid RDF.  A
/// triple with an erroneous term is left out; the others are kept, so an
/// editor can still use them.
#[derive(Clone, Debug, Default)]
pub struct Resolved {
    pub quads: Vec<Quad>,
    pub errors: Vec<Error>,
}

/// Resolves a converted Turtle, TriG, N-Triples or N3 document.  `base` is
/// the document's own location, used until the first `@base` (and for
/// relative IRIs when there is none).
pub fn resolve(doc: &Turtle, base: Option<&str>) -> Resolved {
    let mut r = Resolver {
        base: base.unwrap_or_default().to_string(),
        prefixes: HashMap::new(),
        labels: HashMap::new(),
        fresh: 0,
        out: Resolved::default(),
    };
    enum Ev<'a> {
        Base(&'a Base),
        Prefix(&'a TurtlePrefix),
        Triple(&'a Triple),
    }
    let mut evs: Vec<(usize, Ev)> = Vec::new();
    // `bases` lists every @base; a model built by hand may only set `base`
    if doc.bases.is_empty() {
        evs.extend(
            doc.base
                .iter()
                .map(|b| (b.span().start, Ev::Base(b.value()))),
        );
    } else {
        evs.extend(
            doc.bases
                .iter()
                .map(|b| (b.span().start, Ev::Base(b.value()))),
        );
    }
    evs.extend(
        doc.prefixes
            .iter()
            .map(|p| (p.span().start, Ev::Prefix(p.value()))),
    );
    evs.extend(
        doc.triples
            .iter()
            .map(|t| (t.span().start, Ev::Triple(t.value()))),
    );
    evs.sort_by_key(|e| e.0);
    for (_, ev) in evs {
        match ev {
            Ev::Base(b) => {
                if let Some(iri) = r.full_iri(&b.1) {
                    r.base = iri;
                }
            }
            Ev::Prefix(p) => {
                if let Some(iri) = r.full_iri(&p.value) {
                    r.prefixes.insert(p.prefix.value().clone(), iri);
                }
            }
            Ev::Triple(t) => r.triple(t),
        }
    }
    r.out
}

struct Resolver {
    base: String,
    prefixes: HashMap<String, String>,
    /// Blank node label in the document -> label in the output.
    labels: HashMap<String, String>,
    fresh: usize,
    out: Resolved,
}

impl Resolver {
    fn error<T>(&mut self, kind: ErrorKind, span: &Range<usize>) -> Option<T> {
        self.out.errors.push(Error {
            kind,
            span: span.clone(),
        });
        None
    }

    fn bnode(&mut self) -> Node {
        self.fresh += 1;
        Node::Blank(format!("b{}", self.fresh))
    }

    fn emit(&mut self, s: &Node, p: &Node, o: Node, graph: &Option<Node>) {
        self.out.quads.push(Quad {
            subject: s.clone(),
            predicate: p.clone(),
            object: o,
            graph: graph.clone(),
        });
    }

    /// The IRI of a `@base` or `@prefix` directive.
    fn full_iri(&mut self, nn: &Spanned<NamedNode>) -> Option<String> {
        match nn.value() {
            NamedNode::Full(..) => self.named(nn.value(), nn.span()),
            _ => self.error(ErrorKind::Invalid, nn.span()),
        }
    }

    fn triple(&mut self, t: &Triple) {
        let graph = match &t.graph {
            Some(g) => match self.term(g.value(), g.span(), &None) {
                Some(g) => Some(g),
                None => return,
            },
            None => None,
        };
        let Some(s) = self.term(t.subject.value(), t.subject.span(), &graph) else {
            return;
        };
        self.predicate_objects(&s, &t.po, &graph);
    }

    fn predicate_objects(&mut self, s: &Node, pos: &[Spanned<PO>], graph: &Option<Node>) {
        for po in pos {
            let po = po.value();
            let Some(p) = self.term(po.predicate.value(), po.predicate.span(), graph) else {
                continue;
            };
            for (i, o) in po.object.iter().enumerate() {
                let Some(o) = self.term(o.value(), o.span(), graph) else {
                    continue;
                };
                self.emit(s, &p, o.clone(), graph);
                let annotations = po.annotations_of(i);
                if !annotations.is_empty() {
                    let triple = Node::Triple(Box::new((s.clone(), p.clone(), o)));
                    self.annotate(triple, annotations, graph);
                }
            }
        }
    }

    /// RDF 1.2 annotations of an asserted triple: every reifier states that
    /// it reifies the triple, and a block describes the reifier right before
    /// it, or a fresh one when there is none.
    fn annotate(
        &mut self,
        triple: Node,
        annotations: &[Spanned<Annotation>],
        graph: &Option<Node>,
    ) {
        let reifies = Node::Iri(format!("{RDF}reifies"));
        let mut pending: Option<Node> = None;
        for a in annotations {
            match a.value() {
                Annotation::Reifier(r) => {
                    let Some(node) = self.reifier(r, graph) else {
                        continue;
                    };
                    self.emit(&node, &reifies, triple.clone(), graph);
                    pending = Some(node);
                }
                Annotation::Block(pos) => {
                    let node = match pending.take() {
                        Some(node) => node,
                        None => {
                            let node = self.bnode();
                            self.emit(&node, &reifies, triple.clone(), graph);
                            node
                        }
                    };
                    self.predicate_objects(&node, pos, graph);
                }
            }
        }
    }

    /// The node a reifier names: its IRI or blank node, or a fresh blank node.
    fn reifier(&mut self, r: &Reifier, graph: &Option<Node>) -> Option<Node> {
        match &r.id {
            Some(id) => self.term(id.value(), id.span(), graph),
            None => Some(self.bnode()),
        }
    }

    /// The three nodes of a triple term, checked: an IRI or blank node
    /// subject, an IRI predicate.
    fn triple_term(
        &mut self,
        t: &TripleTerm,
        span: &Range<usize>,
        graph: &Option<Node>,
    ) -> Option<Node> {
        let s = self.term(t.subject.value(), t.subject.span(), graph)?;
        let p = self.term(t.predicate.value(), t.predicate.span(), graph)?;
        let o = self.term(t.object.value(), t.object.span(), graph)?;
        if !matches!(s, Node::Iri(_) | Node::Blank(_)) || !matches!(p, Node::Iri(_)) {
            return self.error(ErrorKind::InvalidTripleTerm, span);
        }
        Some(Node::Triple(Box::new((s, p, o))))
    }

    fn named(&mut self, nn: &NamedNode, span: &Range<usize>) -> Option<String> {
        match nn {
            NamedNode::Full(iri, _) => {
                let iri = match unescape_iri(iri) {
                    Ok(iri) => iri,
                    Err(kind) => return self.error(kind, span),
                };
                Some(resolve_iri(&iri, &self.base))
            }
            NamedNode::Prefixed {
                computed: Some(iri),
                ..
            } => Some(resolve_iri(iri, &self.base)),
            NamedNode::Prefixed { prefix, value, .. } => match self.prefixes.get(prefix) {
                Some(ns) => Some(format!("{ns}{}", unescape_local(value))),
                None => self.error(ErrorKind::UndefinedPrefix(prefix.clone()), span),
            },
            NamedNode::A(_) => Some(format!("{RDF}type")),
            NamedNode::Invalid => self.error(ErrorKind::Invalid, span),
        }
    }

    fn term(&mut self, term: &Term, span: &Range<usize>, graph: &Option<Node>) -> Option<Node> {
        match term {
            Term::NamedNode(nn) => self.named(nn, span).map(Node::Iri),
            Term::BlankNode(BlankNode::Named(name, _)) => {
                let next = self.labels.len() + 1;
                let label = self
                    .labels
                    .entry(name.clone())
                    .or_insert_with(|| format!("l{next}"));
                Some(Node::Blank(label.clone()))
            }
            Term::BlankNode(BlankNode::Unnamed(pos, _, _)) => {
                let node = self.bnode();
                self.predicate_objects(&node, pos, graph);
                Some(node)
            }
            Term::Collection(items) => {
                let nil = Node::Iri(format!("{RDF}nil"));
                let mut nodes = Vec::with_capacity(items.len());
                for item in items {
                    let node = self.bnode();
                    let value = self.term(item.value(), item.span(), graph)?;
                    nodes.push((node, value));
                }
                let first = Node::Iri(format!("{RDF}first"));
                let rest = Node::Iri(format!("{RDF}rest"));
                for i in 0..nodes.len() {
                    let next = nodes.get(i + 1).map_or(nil.clone(), |n| n.0.clone());
                    let (node, value) = (nodes[i].0.clone(), nodes[i].1.clone());
                    self.emit(&node, &first, value, graph);
                    self.emit(&node, &rest, next, graph);
                }
                Some(nodes.first().map_or(nil, |n| n.0.clone()))
            }
            Term::Literal(lit) => self.literal(lit, span),
            Term::TripleTerm(t) => self.triple_term(t, span, graph),
            Term::Reified(r) => {
                let triple = self.triple_term(&r.triple, span, graph)?;
                let node = match &r.reifier {
                    Some(reifier) => self.reifier(reifier.value(), graph)?,
                    None => self.bnode(),
                };
                let reifies = Node::Iri(format!("{RDF}reifies"));
                self.emit(&node, &reifies, triple, graph);
                Some(node)
            }
            Term::Variable(_) | Term::Invalid | Term::BlankNode(BlankNode::Invalid) => {
                self.error(ErrorKind::Invalid, span)
            }
        }
    }

    fn literal(&mut self, lit: &Literal, span: &Range<usize>) -> Option<Node> {
        Some(match lit {
            Literal::RDF(l) => {
                let lexical = match unescape_string(&l.value) {
                    Ok(v) => v,
                    Err(kind) => return self.error(kind, span),
                };
                let language = l.lang.as_ref().map(|x| x.value().to_lowercase());
                let direction = l.direction.filter(|_| language.is_some());
                let datatype = match (&l.ty, &language, direction) {
                    (Some(nn), ..) => self.named(nn.value(), nn.span())?,
                    (None, Some(_), Some(_)) => format!("{RDF}dirLangString"),
                    (None, Some(_), None) => format!("{RDF}langString"),
                    (None, None, _) => format!("{XSD}string"),
                };
                Node::Literal {
                    lexical,
                    datatype,
                    language,
                    direction,
                }
            }
            Literal::Boolean(b) => Node::Literal {
                lexical: b.to_string(),
                datatype: format!("{XSD}boolean"),
                language: None,
                direction: None,
            },
            Literal::Numeric(n) => {
                let lexical = n.trim().to_string();
                let ty = if lexical.contains(['e', 'E']) {
                    "double"
                } else if lexical.contains('.') {
                    "decimal"
                } else {
                    "integer"
                };
                Node::Literal {
                    lexical,
                    datatype: format!("{XSD}{ty}"),
                    language: None,
                    direction: None,
                }
            }
        })
    }
}

// ---- escapes ------------------------------------------------------------------

/// The character a `\u`/`\U` escape encodes, or an error when it is not a
/// Unicode scalar value.
fn uchar(it: &mut std::str::Chars, long: bool) -> Result<char, ErrorKind> {
    let hex: String = it.by_ref().take(if long { 8 } else { 4 }).collect();
    u32::from_str_radix(&hex, 16)
        .ok()
        .and_then(char::from_u32)
        .ok_or_else(|| ErrorKind::InvalidEscape(format!("\\{}{hex}", if long { 'U' } else { 'u' })))
}

/// Unescapes the content of a string literal (`ECHAR` and `UCHAR`).
pub fn unescape_string(s: &str) -> Result<String, ErrorKind> {
    let mut out = String::with_capacity(s.len());
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match it.next() {
            Some('t') => out.push('\t'),
            Some('b') => out.push('\u{8}'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('f') => out.push('\u{c}'),
            Some(c @ ('u' | 'U')) => out.push(uchar(&mut it, c == 'U')?),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    Ok(out)
}

/// Unescapes the content of an `IRIREF` (`UCHAR` only) and checks that the
/// result contains no character an IRI may not contain.
pub fn unescape_iri(s: &str) -> Result<String, ErrorKind> {
    let mut out = String::with_capacity(s.len());
    let mut it = s.chars();
    while let Some(c) = it.next() {
        match c {
            '\\' => match it.next() {
                Some(c @ ('u' | 'U')) => out.push(uchar(&mut it, c == 'U')?),
                _ => return Err(ErrorKind::InvalidIri(s.to_string())),
            },
            c => out.push(c),
        }
    }
    if out.chars().any(|c| c <= ' ' || "<>\"{}|^`\\".contains(c)) {
        return Err(ErrorKind::InvalidIri(out));
    }
    Ok(out)
}

/// Unescapes the local part of a prefixed name (`PN_LOCAL_ESC`; `%xx`
/// stays as it is).
pub fn unescape_local(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut it = s.chars();
    while let Some(c) = it.next() {
        match c {
            '\\' => out.extend(it.next()),
            c => out.push(c),
        }
    }
    out
}

// ---- output -------------------------------------------------------------------

impl Display for Node {
    /// N-Triples syntax.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Iri(i) => write!(f, "<{i}>"),
            Node::Blank(b) => write!(f, "_:{b}"),
            Node::Triple(t) => write!(f, "<<( {} {} {} )>>", t.0, t.1, t.2),
            Node::Literal {
                lexical,
                datatype,
                language,
                direction,
            } => {
                f.write_str("\"")?;
                for c in lexical.chars() {
                    match c {
                        '\\' => f.write_str("\\\\")?,
                        '"' => f.write_str("\\\"")?,
                        '\n' => f.write_str("\\n")?,
                        '\r' => f.write_str("\\r")?,
                        c => write!(f, "{c}")?,
                    }
                }
                f.write_str("\"")?;
                match language {
                    Some(l) => {
                        write!(f, "@{l}")?;
                        match direction {
                            Some(d) => write!(f, "--{d}"),
                            None => Ok(()),
                        }
                    }
                    None if datatype.strip_prefix(XSD) == Some("string") => Ok(()),
                    None => write!(f, "^^<{datatype}>"),
                }
            }
        }
    }
}

impl Display for Quad {
    /// One N-Quads line (N-Triples when in the default graph), with newline.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.subject, self.predicate, self.object)?;
        if let Some(g) = &self.graph {
            write!(f, " {g}")?;
        }
        f.write_str(" .\n")
    }
}

// ---- RFC 3986, section 5.2 ----------------------------------------------------

struct Parts<'a> {
    scheme: Option<&'a str>,
    auth: Option<&'a str>,
    path: &'a str,
    query: Option<&'a str>,
    frag: Option<&'a str>,
}

fn parts(s: &str) -> Parts<'_> {
    let (s, frag) = match s.find('#') {
        Some(i) => (&s[..i], Some(&s[i + 1..])),
        None => (s, None),
    };
    let (s, query) = match s.find('?') {
        Some(i) => (&s[..i], Some(&s[i + 1..])),
        None => (s, None),
    };
    let (scheme, s) = match s.find(':') {
        Some(i) if i > 0 && !s[..i].contains('/') => (Some(&s[..i]), &s[i + 1..]),
        _ => (None, s),
    };
    let (auth, path) = if let Some(rest) = s.strip_prefix("//") {
        let i = rest.find('/').unwrap_or(rest.len());
        (Some(&rest[..i]), &rest[i..])
    } else {
        (None, s)
    };
    Parts {
        scheme,
        auth,
        path,
        query,
        frag,
    }
}

fn remove_dots(mut p: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    while !p.is_empty() {
        if let Some(r) = p.strip_prefix("../") {
            p = r;
        } else if let Some(r) = p.strip_prefix("./") {
            p = r;
        } else if p.starts_with("/./") {
            p = &p[2..];
        } else if p == "/." {
            p = "/";
        } else if p.starts_with("/../") {
            p = &p[3..];
            out.pop();
        } else if p == "/.." {
            p = "/";
            out.pop();
        } else if p == "." || p == ".." {
            p = "";
        } else {
            let start = usize::from(p.starts_with('/'));
            let i = p[start..].find('/').map_or(p.len(), |i| i + start);
            out.push(&p[..i]);
            p = &p[i..];
        }
    }
    out.concat()
}

/// Resolves an IRI reference against a base IRI (RFC 3986, section 5.2).  With
/// an empty base the reference is returned as it is.
pub fn resolve_iri(reference: &str, base: &str) -> String {
    if base.is_empty() {
        return reference.to_string();
    }
    let (r, b) = (parts(reference), parts(base));
    let (scheme, auth, path, query);
    if r.scheme.is_some() {
        (scheme, auth, path, query) = (r.scheme, r.auth, remove_dots(r.path), r.query);
    } else if r.auth.is_some() {
        (scheme, auth, path, query) = (b.scheme, r.auth, remove_dots(r.path), r.query);
    } else if r.path.is_empty() {
        (scheme, auth, path, query) = (b.scheme, b.auth, b.path.to_string(), r.query.or(b.query));
    } else if r.path.starts_with('/') {
        (scheme, auth, path, query) = (b.scheme, b.auth, remove_dots(r.path), r.query);
    } else {
        let merged = if b.auth.is_some() && b.path.is_empty() {
            format!("/{}", r.path)
        } else {
            match b.path.rfind('/') {
                Some(i) => format!("{}{}", &b.path[..=i], r.path),
                None => r.path.to_string(),
            }
        };
        (scheme, auth, path, query) = (b.scheme, b.auth, remove_dots(&merged), r.query);
    }
    let mut out = String::new();
    if let Some(s) = scheme {
        out.push_str(s);
        out.push(':');
    }
    if let Some(a) = auth {
        out.push_str("//");
        out.push_str(a);
    }
    out.push_str(&path);
    if let Some(q) = query {
        out.push('?');
        out.push_str(q);
    }
    if let Some(f) = r.frag {
        out.push('#');
        out.push_str(f);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse, turtle};

    fn rdf(src: &str) -> Resolved {
        let (p, _) = parse(turtle::Rule::new(turtle::SyntaxKind::TurtleDoc), src);
        resolve(
            &turtle::convert::convert(&p.syntax::<turtle::Lang>()),
            Some("http://doc/"),
        )
    }

    fn lines(r: &Resolved) -> Vec<String> {
        r.quads
            .iter()
            .map(|q| q.to_string().trim_end().to_string())
            .collect()
    }

    #[test]
    fn directives_apply_in_document_order() {
        let r = rdf(
            "<a> <b> <c> . @base <http://ex/ns/> . <a> <b> <c> . @base <foo/> . <a> <b> <c> .
                     @prefix : <bar#> . :a :b :c . @prefix : <http://other#> . :a :b :c .",
        );
        assert!(r.errors.is_empty(), "{:?}", r.errors);
        assert_eq!(
            lines(&r),
            [
                "<http://doc/a> <http://doc/b> <http://doc/c> .",
                "<http://ex/ns/a> <http://ex/ns/b> <http://ex/ns/c> .",
                "<http://ex/ns/foo/a> <http://ex/ns/foo/b> <http://ex/ns/foo/c> .",
                "<http://ex/ns/foo/bar#a> <http://ex/ns/foo/bar#b> <http://ex/ns/foo/bar#c> .",
                "<http://other#a> <http://other#b> <http://other#c> .",
            ]
        );
    }

    #[test]
    fn prefix_used_before_its_declaration() {
        let src = ":a :b :c . @prefix : <http://ex/> . :a :b :c .";
        let r = rdf(src);
        assert_eq!(r.quads.len(), 1);
        assert_eq!(r.errors[0].kind, ErrorKind::UndefinedPrefix(String::new()));
        assert_eq!(&src[r.errors[0].span.clone()], ":a");
    }

    #[test]
    fn surrogate_escapes_are_errors() {
        for src in [
            r#"<s> <p> "\ud800" ."#,
            r#"<s> <p> '''\udfff''' ."#,
            r#"<s> <p> <\ud800> ."#,
        ] {
            let r = rdf(src);
            assert!(
                matches!(
                    r.errors[..],
                    [Error {
                        kind: ErrorKind::InvalidEscape(_),
                        ..
                    }]
                ),
                "{src}"
            );
            assert!(r.quads.is_empty());
        }
        assert_eq!(
            lines(&rdf(r#"<s> <p> "\U0001F600" ."#))[0],
            "<http://doc/s> <http://doc/p> \"😀\" ."
        );
    }

    #[test]
    fn escaped_characters_iris_cannot_contain() {
        for esc in [r"\u0020", r"\u003C", r"\u003E", r"\u0022"] {
            let r = rdf(&format!("<http://ex/{esc}> <p> <o> ."));
            assert!(
                matches!(
                    r.errors[..],
                    [Error {
                        kind: ErrorKind::InvalidIri(_),
                        ..
                    }]
                ),
                "{esc}"
            );
        }
        assert!(rdf(r"<http://ex/\u00E9> <p> <o> .").errors.is_empty());
    }

    #[test]
    fn blank_nodes_and_collections() {
        let r = rdf("_:x <p> ( 1 [ <q> _:x ] ) .");
        assert!(r.errors.is_empty());
        let l = lines(&r);
        assert_eq!(l.len(), 6);
        assert!(l.iter().any(|t| t.ends_with("<http://doc/q> _:l1 .")));
        assert!(l[l.len() - 1].starts_with("_:l1 <http://doc/p> _:b"));
    }

    // ---- RDF 1.2 (no Turtle 1.2 syntax yet: models built by hand) ----------

    fn sp<T>(t: T) -> Spanned<T> {
        Spanned(t, 0..1)
    }
    fn iri(i: &str) -> Spanned<Term> {
        sp(Term::NamedNode(NamedNode::Full(
            format!("http://ex/{i}"),
            0,
        )))
    }
    fn string(v: &str, lang: Option<&str>, direction: Option<Direction>) -> Spanned<Term> {
        sp(Term::Literal(Literal::RDF(RDFLiteral {
            value: v.to_string(),
            quote_style: StringStyle::Double,
            lang: lang.map(|l| sp(l.to_string())),
            direction,
            ty: None,
            idx: 0,
            len: 0,
        })))
    }
    fn tt(s: Spanned<Term>, p: &str, o: Spanned<Term>) -> TripleTerm {
        TripleTerm {
            subject: s,
            predicate: iri(p),
            object: o,
        }
    }
    fn reified(t: TripleTerm, reifier: Option<Option<Spanned<Term>>>) -> Spanned<Term> {
        let reifier = reifier.map(|id| sp(Reifier { id }));
        sp(Term::Reified(Box::new(ReifiedTriple {
            triple: t,
            reifier,
        })))
    }
    fn triple(
        s: Spanned<Term>,
        p: &str,
        o: Spanned<Term>,
        annotations: Vec<Annotation>,
    ) -> Spanned<Triple> {
        let mut po = PO::new(iri(p), vec![o]);
        if !annotations.is_empty() {
            po.annotations = vec![annotations.into_iter().map(sp).collect()];
        }
        sp(Triple {
            subject: s,
            po: vec![sp(po)],
            graph: None,
        })
    }
    fn rdf12(triples: Vec<Spanned<Triple>>) -> Vec<String> {
        let r = resolve(&Turtle::new(None, Vec::new(), triples), None);
        assert!(r.errors.is_empty(), "{:?}", r.errors);
        r.quads
            .iter()
            .map(|q| q.to_string().trim_end().to_string())
            .collect()
    }
    const REIFIES: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#reifies>";

    #[test]
    fn triple_term_object() {
        let t = sp(Term::TripleTerm(Box::new(tt(
            iri("a"),
            "b",
            string("c", None, None),
        ))));
        assert_eq!(
            rdf12(vec![triple(iri("s"), "p", t, vec![])]),
            [r#"<http://ex/s> <http://ex/p> <<( <http://ex/a> <http://ex/b> "c" )>> ."#]
        );
    }

    #[test]
    fn reified_triples_with_and_without_reifier() {
        let named = reified(tt(iri("a"), "b", iri("c")), Some(Some(iri("r"))));
        assert_eq!(
            rdf12(vec![triple(named, "q", iri("v"), vec![])]),
            [
                format!(
                    "<http://ex/r> {REIFIES} <<( <http://ex/a> <http://ex/b> <http://ex/c> )>> ."
                ),
                "<http://ex/r> <http://ex/q> <http://ex/v> .".to_string(),
            ]
        );
        // `<< << :a :b :c >> :p :o >> :q :v`: two fresh reifiers, the outer one
        // reifying a triple about the inner one
        let inner = reified(tt(iri("a"), "b", iri("c")), None);
        let outer = reified(tt(inner, "p", iri("o")), Some(None));
        assert_eq!(
            rdf12(vec![triple(outer, "q", iri("v"), vec![])]),
            [
                format!("_:b1 {REIFIES} <<( <http://ex/a> <http://ex/b> <http://ex/c> )>> ."),
                format!("_:b2 {REIFIES} <<( _:b1 <http://ex/p> <http://ex/o> )>> ."),
                "_:b2 <http://ex/q> <http://ex/v> .".to_string(),
            ]
        );
    }

    #[test]
    fn annotations_reifier_then_blocks() {
        // `:s :p :o ~:r {| :q :v |} {| :q2 :v2 |}`: the first block describes
        // :r, the second a fresh reifier
        let block = |p: &str, o: &str| Annotation::Block(vec![sp(PO::new(iri(p), vec![iri(o)]))]);
        let annotations = vec![
            Annotation::Reifier(Reifier { id: Some(iri("r")) }),
            block("q", "v"),
            block("q2", "v2"),
        ];
        let t = "<<( <http://ex/s> <http://ex/p> <http://ex/o> )>>";
        assert_eq!(
            rdf12(vec![triple(iri("s"), "p", iri("o"), annotations)]),
            [
                "<http://ex/s> <http://ex/p> <http://ex/o> .".to_string(),
                format!("<http://ex/r> {REIFIES} {t} ."),
                "<http://ex/r> <http://ex/q> <http://ex/v> .".to_string(),
                format!("_:b1 {REIFIES} {t} ."),
                "_:b1 <http://ex/q2> <http://ex/v2> .".to_string(),
            ]
        );
    }

    #[test]
    fn directional_language_strings() {
        let r = resolve(
            &Turtle::new(
                None,
                Vec::new(),
                vec![triple(
                    iri("s"),
                    "p",
                    string("x", Some("AR"), Some(Direction::Rtl)),
                    vec![],
                )],
            ),
            None,
        );
        assert_eq!(r.quads[0].object.to_string(), r#""x"@ar--rtl"#);
        assert!(
            matches!(&r.quads[0].object, Node::Literal { datatype, .. } if datatype.ends_with("#dirLangString"))
        );
    }

    #[test]
    fn triple_term_with_a_literal_subject_is_an_error() {
        let bad = sp(Term::TripleTerm(Box::new(tt(
            string("x", None, None),
            "b",
            iri("c"),
        ))));
        let r = resolve(
            &Turtle::new(None, Vec::new(), vec![triple(iri("s"), "p", bad, vec![])]),
            None,
        );
        assert_eq!(r.errors[0].kind, ErrorKind::InvalidTripleTerm);
        assert!(r.quads.is_empty());
    }
}
