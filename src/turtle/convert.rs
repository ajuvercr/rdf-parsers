use rowan::SyntaxNode;

use crate::Spanned;

use super::parser::{Lang, SyntaxKind};
use crate::model::*;

type Node = SyntaxNode<Lang>;

fn text_range(node: &Node) -> std::ops::Range<usize> {
    let r = node.text_range();
    r.start().into()..r.end().into()
}

fn child(node: &Node, kind: SyntaxKind) -> Option<Node> {
    node.children().find(|c| c.kind() == kind)
}

fn children(node: &Node, kind: SyntaxKind) -> impl Iterator<Item = Node> {
    node.children().filter(move |c| c.kind() == kind)
}

/// Get text from a terminal node (a node that wraps a single token of the same kind).
fn terminal_text(node: &Node) -> String {
    node.text().to_string()
}

pub fn convert(root: &Node) -> Turtle {
    // TurtleDoc is the initial rule — its contents are direct children of ROOT.
    let mut base = None;
    let mut prefixes = Vec::new();
    let mut triples = Vec::new();

    for stmt in children(root, SyntaxKind::Statement) {
        if let Some(dir) = child(&stmt, SyntaxKind::Directive) {
            if let Some(b) =
                child(&dir, SyntaxKind::Base).or_else(|| child(&dir, SyntaxKind::SparqlBase))
            {
                base = Some(Spanned(convert_base(&b), text_range(&b)));
            } else if let Some(p) =
                child(&dir, SyntaxKind::PrefixId).or_else(|| child(&dir, SyntaxKind::SparqlPrefix))
            {
                prefixes.push(Spanned(convert_prefix(&p), text_range(&p)));
            }
        } else if let Some(t) = child(&stmt, SyntaxKind::Triples) {
            let range = text_range(&stmt);
            triples.push(Spanned(convert_triples(&t), range));
        }
    }

    Turtle::new(base, prefixes, triples)
}

fn convert_base(node: &Node) -> Base {
    let range = text_range(node);
    // base ::= '@base' IRIREF '.' — IRIREF is a direct terminal child
    let nn = child(node, SyntaxKind::Iriref)
        .map(|n| iri_from_iriref_node(&n))
        .or_else(|| child(node, SyntaxKind::Iri).map(|n| convert_iri(&n)))
        .unwrap_or(NamedNode::Invalid);

    let nn_range = child(node, SyntaxKind::Iriref)
        .or_else(|| child(node, SyntaxKind::Iri))
        .map(|n| text_range(&n))
        .unwrap_or(range.clone());

    Base(range, Spanned(nn, nn_range))
}

fn convert_prefix(node: &Node) -> TurtlePrefix {
    let range = text_range(node);

    // prefixID ::= '@prefix' PNAME_NS IRIREF '.'
    // SparqlPrefix ::= "PREFIX" PNAME_NS IRIREF
    let prefix_text = child(node, SyntaxKind::PnameNs)
        .map(|n| {
            let text = terminal_text(&n);
            let tr = text_range(&n);
            Spanned(text.trim_end_matches(':').to_string(), tr)
        })
        .unwrap_or_else(|| Spanned(String::new(), range.clone()));

    // IRIREF is a direct terminal child of prefixID/sparqlPrefix
    let value = child(node, SyntaxKind::Iriref)
        .map(|n| {
            let tr = text_range(&n);
            Spanned(iri_from_iriref_node(&n), tr)
        })
        .or_else(|| {
            child(node, SyntaxKind::Iri).map(|n| {
                let r = text_range(&n);
                Spanned(convert_iri(&n), r)
            })
        })
        .unwrap_or_else(|| Spanned(NamedNode::Invalid, range.clone()));

    TurtlePrefix {
        span: range,
        prefix: prefix_text,
        value,
    }
}

fn convert_triples(node: &Node) -> Triple {
    // triples ::= blankNodePropertyList predicateObjectList? | subject predicateObjectList
    let (subject, po_node) = if let Some(bpl) = child(node, SyntaxKind::BlankNodePropertyList) {
        let range = text_range(&bpl);
        let subject = convert_blank_node_property_list(&bpl);
        (
            Spanned(subject, range),
            child(node, SyntaxKind::PredicateObjectList),
        )
    } else if let Some(subj) = child(node, SyntaxKind::Subject) {
        let range = text_range(&subj);
        (
            Spanned(convert_subject(&subj), range),
            child(node, SyntaxKind::PredicateObjectList),
        )
    } else {
        (Spanned(Term::Invalid, text_range(node)), None)
    };

    let po = po_node
        .map(|n| convert_predicate_object_list(&n))
        .unwrap_or_default();

    Triple {
        subject,
        po,
        graph: None,
    }
}

fn convert_predicate_object_list(node: &Node) -> Vec<Spanned<PO>> {
    // predicateObjectList ::= verb objectList (';' (verb objectList)?)*
    let verbs: Vec<_> = children(node, SyntaxKind::Verb).collect();
    let object_lists: Vec<_> = children(node, SyntaxKind::ObjectList).collect();

    verbs
        .into_iter()
        .zip(object_lists.into_iter())
        .map(|(v, ol)| {
            let range = text_range(&v).start..text_range(&ol).end;
            let pred_range = text_range(&v);
            let predicate = convert_verb(&v);
            let objects = convert_object_list(&ol);
            Spanned(
                PO {
                    predicate: Spanned(predicate, pred_range),
                    object: objects,
                },
                range,
            )
        })
        .collect()
}

fn convert_verb(node: &Node) -> Term {
    // verb ::= predicate | 'a'
    if let Some(pred) = child(node, SyntaxKind::Predicate) {
        if let Some(iri) = child(&pred, SyntaxKind::Iri) {
            Term::NamedNode(convert_iri(&iri))
        } else {
            Term::Invalid
        }
    } else if let Some(alit) = child(node, SyntaxKind::Alit) {
        Term::NamedNode(NamedNode::A(alit.text_range().start().into()))
    } else {
        Term::Invalid
    }
}

fn convert_object_list(node: &Node) -> Vec<Spanned<Term>> {
    children(node, SyntaxKind::Object)
        .map(|o| {
            let range = text_range(&o);
            Spanned(convert_object(&o), range)
        })
        .collect()
}

fn convert_subject(node: &Node) -> Term {
    // subject ::= iri | BlankNode | collection
    if let Some(iri) = child(node, SyntaxKind::Iri) {
        Term::NamedNode(convert_iri(&iri))
    } else if let Some(bn) = child(node, SyntaxKind::BlankNode) {
        Term::BlankNode(convert_blank_node(&bn))
    } else if let Some(coll) = child(node, SyntaxKind::Collection) {
        Term::Collection(convert_collection(&coll))
    } else {
        Term::Invalid
    }
}

fn convert_object(node: &Node) -> Term {
    // object ::= iri | BlankNode | collection | blankNodePropertyList | literal
    if let Some(iri) = child(node, SyntaxKind::Iri) {
        Term::NamedNode(convert_iri(&iri))
    } else if let Some(label) = child(node, SyntaxKind::BlankNodeLabel) {
        let text = terminal_text(&label);
        let offset: usize = label.text_range().start().into();
        let name = text.strip_prefix("_:").unwrap_or(&text);
        Term::BlankNode(BlankNode::Named(name.to_string(), offset))
    } else if let Some(bn) = child(node, SyntaxKind::BlankNode) {
        Term::BlankNode(convert_blank_node(&bn))
    } else if let Some(coll) = child(node, SyntaxKind::Collection) {
        Term::Collection(convert_collection(&coll))
    } else if let Some(bpl) = child(node, SyntaxKind::BlankNodePropertyList2) {
        convert_blank_node_property_list(&bpl)
    } else if let Some(lit) = child(node, SyntaxKind::Literal) {
        Term::Literal(convert_literal(&lit))
    } else {
        Term::Invalid
    }
}

fn convert_iri(node: &Node) -> NamedNode {
    // iri ::= IRIREF | PrefixedName
    // Both appear as terminal nodes (Iriref, PrefixedName) wrapping their token.
    if let Some(iriref_node) = child(node, SyntaxKind::Iriref) {
        iri_from_iriref_node(&iriref_node)
    } else if let Some(pn) = child(node, SyntaxKind::PrefixedName) {
        convert_prefixed_name(&pn)
    } else {
        NamedNode::Invalid
    }
}

fn iri_from_iriref_node(node: &Node) -> NamedNode {
    let text = terminal_text(node);
    let offset: usize = node.text_range().start().into();
    let inner = text.trim_start_matches('<').trim_end_matches('>');
    NamedNode::Full(inner.to_string(), offset)
}

fn convert_prefixed_name(node: &Node) -> NamedNode {
    // PrefixedName ::= PNAME_LN | PNAME_NS
    if let Some(pname_ln) = child(node, SyntaxKind::PnameLn) {
        let text = terminal_text(&pname_ln);
        let offset: usize = pname_ln.text_range().start().into();
        let (prefix, value) = text.split_once(':').unwrap_or((&text, ""));
        NamedNode::Prefixed {
            prefix: prefix.to_string(),
            value: value.to_string(),
            idx: offset,
        }
    } else if let Some(pname_ns) = child(node, SyntaxKind::PnameNs) {
        let text = terminal_text(&pname_ns);
        let offset: usize = pname_ns.text_range().start().into();
        let prefix = text.trim_end_matches(':');
        NamedNode::Prefixed {
            prefix: prefix.to_string(),
            value: String::new(),
            idx: offset,
        }
    } else {
        NamedNode::Invalid
    }
}

fn convert_blank_node(node: &Node) -> BlankNode {
    // BlankNode ::= BLANK_NODE_LABEL | ANON | '[' ']'
    if let Some(label) = child(node, SyntaxKind::BlankNodeLabel) {
        let text = terminal_text(&label);
        let offset: usize = label.text_range().start().into();
        let name = text.strip_prefix("_:").unwrap_or(&text);
        BlankNode::Named(name.to_string(), offset)
    } else {
        // ANON or '[ ]'
        let offset: usize = node.text_range().start().into();
        BlankNode::Unnamed(Vec::new(), offset, offset)
    }
}

fn convert_blank_node_property_list(node: &Node) -> Term {
    // blankNodePropertyList ::= '[' predicateObjectList ']'
    let offset: usize = node.text_range().start().into();
    let end: usize = node.text_range().end().into();
    let po = child(node, SyntaxKind::PredicateObjectList)
        .map(|n| convert_predicate_object_list(&n))
        .unwrap_or_default();
    Term::BlankNode(BlankNode::Unnamed(po, offset, end))
}

fn convert_collection(node: &Node) -> Vec<Spanned<Term>> {
    children(node, SyntaxKind::Object)
        .map(|o| {
            let range = text_range(&o);
            Spanned(convert_object(&o), range)
        })
        .collect()
}

fn convert_literal(node: &Node) -> Literal {
    // literal ::= RDFLiteral | NumericLiteral | BooleanLiteral
    if let Some(rdf) = child(node, SyntaxKind::Rdfliteral) {
        Literal::RDF(convert_rdf_literal(&rdf))
    } else if let Some(num) = child(node, SyntaxKind::NumericLiteral) {
        Literal::Numeric(num.text().to_string().trim().to_string())
    } else if let Some(b) = child(node, SyntaxKind::BooleanLiteral) {
        Literal::Boolean(b.text().to_string().trim() == "true")
    } else {
        Literal::Numeric(String::new())
    }
}

fn convert_rdf_literal(node: &Node) -> RDFLiteral {
    // RDFLiteral ::= String (LANGTAG | '^^' iri)?
    let offset: usize = node.text_range().start().into();

    let (value, quote_style, len) = if let Some(str_node) = child(node, SyntaxKind::MyString) {
        // MyString contains one of the four string terminal nodes
        let string_node = str_node.children().next();
        if let Some(sn) = string_node {
            let text = terminal_text(&sn);
            let (inner, style) = strip_string_delimiters(&text);
            (inner.to_string(), style, 1)
        } else {
            (String::new(), StringStyle::Double, 0)
        }
    } else {
        (String::new(), StringStyle::Double, 0)
    };

    let lang = child(node, SyntaxKind::Langtag).map(|n| {
        let text = terminal_text(&n);
        text.strip_prefix('@').unwrap_or(&text).to_string()
    });

    let ty = child(node, SyntaxKind::Iri).map(|n| convert_iri(&n));

    RDFLiteral {
        value,
        quote_style,
        lang,
        ty,
        idx: offset,
        len,
    }
}

fn strip_string_delimiters(text: &str) -> (&str, StringStyle) {
    if let Some(inner) = text
        .strip_prefix("\"\"\"")
        .and_then(|s| s.strip_suffix("\"\"\""))
    {
        (inner, StringStyle::DoubleLong)
    } else if let Some(inner) = text.strip_prefix("'''").and_then(|s| s.strip_suffix("'''")) {
        (inner, StringStyle::SingleLong)
    } else if let Some(inner) = text.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        (inner, StringStyle::Double)
    } else if let Some(inner) = text.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
        (inner, StringStyle::Single)
    } else {
        (text, StringStyle::Double)
    }
}

#[cfg(test)]
mod tests {
    use rowan::TextSize;

    use super::*;
    use crate::{parse as crate_parse, turtle::parser as lang};

    fn parse(input: &str) -> Turtle {
        let (result, _) = crate_parse(lang::Rule::new(lang::SyntaxKind::TurtleDoc), input);
        let root = result.syntax::<lang::Lang>();
        convert(&root)
    }

    // ── helpers ──────────────────────────────────────────────────────────────

    fn prefixed(prefix: &str, value: &str) -> NamedNode {
        NamedNode::Prefixed {
            prefix: prefix.to_string(),
            value: value.to_string(),
            idx: 0, // ignored in PartialEq? No — but we only care about the string parts.
        }
    }

    /// Compare two NamedNodes ignoring offset fields.
    fn nn_eq(a: &NamedNode, b: &NamedNode) -> bool {
        match (a, b) {
            (NamedNode::Full(s1, _), NamedNode::Full(s2, _)) => s1 == s2,
            (
                NamedNode::Prefixed {
                    prefix: p1,
                    value: v1,
                    ..
                },
                NamedNode::Prefixed {
                    prefix: p2,
                    value: v2,
                    ..
                },
            ) => p1 == p2 && v1 == v2,
            (NamedNode::A(_), NamedNode::A(_)) => true,
            (NamedNode::Invalid, NamedNode::Invalid) => true,
            _ => false,
        }
    }

    fn term_nn(t: &Term) -> &NamedNode {
        match t {
            Term::NamedNode(nn) => nn,
            other => panic!("expected NamedNode, got {:?}", other),
        }
    }

    fn term_lit(t: &Term) -> &Literal {
        match t {
            Term::Literal(l) => l,
            other => panic!("expected Literal, got {:?}", other),
        }
    }

    fn term_bn(t: &Term) -> &BlankNode {
        match t {
            Term::BlankNode(bn) => bn,
            other => panic!("expected BlankNode, got {:?}", other),
        }
    }

    // ── prefix directive ─────────────────────────────────────────────────────

    #[test]
    fn test_prefix_directive() {
        let doc = parse("@prefix ex: <http://example.org/> .");
        assert_eq!(doc.prefixes.len(), 1);
        let p = doc.prefixes[0].value();
        assert_eq!(p.prefix.value(), "ex");
        assert!(nn_eq(
            p.value.value(),
            &NamedNode::Full("http://example.org/".to_string(), 0)
        ));
    }

    #[test]
    fn test_sparql_prefix_directive() {
        let doc = parse("PREFIX ex: <http://example.org/>");
        assert_eq!(doc.prefixes.len(), 1);
        assert_eq!(doc.prefixes[0].prefix.value(), "ex");
    }

    // ── base directive ────────────────────────────────────────────────────────

    #[test]
    fn test_base_directive() {
        let doc = parse("@base <http://example.org/> .");
        let base = doc.base.as_ref().expect("base should be present");
        assert!(nn_eq(
            base.value().1.value(),
            &NamedNode::Full("http://example.org/".to_string(), 0)
        ));
    }

    // ── simple triple with full IRI subject ───────────────────────────────────

    #[test]
    fn test_full_iri_subject_and_predicate() {
        let doc = parse(
            "<http://example.org/alice> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Person> .",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(
            term_nn(t.subject.value()),
            &NamedNode::Full("http://example.org/alice".to_string(), 0)
        ));
        assert_eq!(t.po.len(), 1);
        assert!(nn_eq(
            term_nn(t.po[0].predicate.value()),
            &NamedNode::Full(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string(),
                0
            )
        ));
        assert!(nn_eq(
            term_nn(t.po[0].object[0].value()),
            &NamedNode::Full("http://xmlns.com/foaf/0.1/Person".to_string(), 0)
        ));
    }

    // ── `a` shorthand ─────────────────────────────────────────────────────────

    #[test]
    fn test_a_shorthand() {
        let doc = parse(
            "@prefix foaf: <http://xmlns.com/foaf/0.1/> . <http://example.org/alice> a foaf:Person .",
        );
        let t = doc.triples[0].value();
        assert!(matches!(
            t.po[0].predicate.value(),
            Term::NamedNode(NamedNode::A(_))
        ));
        assert!(nn_eq(
            term_nn(t.po[0].object[0].value()),
            &prefixed("foaf", "Person")
        ));
    }

    // ── prefixed name ─────────────────────────────────────────────────────────

    #[test]
    fn test_prefixed_name_subject() {
        let doc = parse("@prefix ex: <http://example.org/> . ex:alice ex:knows ex:bob .");
        let t = doc.triples[0].value();
        assert!(nn_eq(term_nn(t.subject.value()), &prefixed("ex", "alice")));
        assert!(nn_eq(
            term_nn(t.po[0].object[0].value()),
            &prefixed("ex", "bob")
        ));
    }

    // ── multiple predicate-object pairs (`;`) ────────────────────────────────

    #[test]
    fn test_multiple_po_pairs() {
        let doc =
            parse("@prefix ex: <http://example.org/> . ex:alice ex:name \"Alice\" ; ex:age 30 .");
        let t = doc.triples[0].value();
        assert_eq!(t.po.len(), 2);
        assert!(nn_eq(
            term_nn(t.po[0].predicate.value()),
            &prefixed("ex", "name")
        ));
        assert!(nn_eq(
            term_nn(t.po[1].predicate.value()),
            &prefixed("ex", "age")
        ));
    }

    // ── multiple objects (`,`) ────────────────────────────────────────────────

    #[test]
    fn test_multiple_objects() {
        let doc =
            parse("@prefix ex: <http://example.org/> . ex:alice ex:knows ex:bob , ex:carol .");
        let t = doc.triples[0].value();
        assert_eq!(t.po[0].object.len(), 2);
        assert!(nn_eq(
            term_nn(t.po[0].object[0].value()),
            &prefixed("ex", "bob")
        ));
        assert!(nn_eq(
            term_nn(t.po[0].object[1].value()),
            &prefixed("ex", "carol")
        ));
    }

    // ── string literal ────────────────────────────────────────────────────────

    #[test]
    fn test_string_literal() {
        let doc = parse("@prefix ex: <http://example.org/> . ex:alice ex:name \"Alice\" .");
        let t = doc.triples[0].value();
        let lit = term_lit(t.po[0].object[0].value());
        assert_eq!(lit.plain_string(), "Alice");
    }

    #[test]
    fn test_string_literal_with_lang() {
        let doc = parse("@prefix ex: <http://example.org/> . ex:alice ex:name \"Alice\"@en .");
        let t = doc.triples[0].value();
        match term_lit(t.po[0].object[0].value()) {
            Literal::RDF(r) => {
                assert_eq!(r.value, "Alice");
                assert_eq!(r.lang.as_deref(), Some("en"));
            }
            other => panic!("expected RDF literal, got {:?}", other),
        }
    }

    #[test]
    fn test_string_literal_with_datatype() {
        let doc = parse(
            "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> . @prefix ex: <http://example.org/> . ex:alice ex:age \"30\"^^xsd:integer .",
        );
        let t = doc.triples[0].value();
        match term_lit(t.po[0].object[0].value()) {
            Literal::RDF(r) => {
                assert_eq!(r.value, "30");
                assert!(r.ty.is_some());
                assert!(nn_eq(r.ty.as_ref().unwrap(), &prefixed("xsd", "integer")));
            }
            other => panic!("expected RDF literal, got {:?}", other),
        }
    }

    // ── numeric literal ───────────────────────────────────────────────────────

    #[test]
    fn test_numeric_literal() {
        let doc = parse("@prefix ex: <http://example.org/> . ex:alice ex:age 30 .");
        let t = doc.triples[0].value();
        match term_lit(t.po[0].object[0].value()) {
            Literal::Numeric(n) => assert_eq!(n, "30"),
            other => panic!("expected Numeric literal, got {:?}", other),
        }
    }

    // ── boolean literal ───────────────────────────────────────────────────────

    #[test]
    fn test_boolean_literal() {
        let doc = parse("@prefix ex: <http://example.org/> . ex:alice ex:active true .");
        let t = doc.triples[0].value();
        assert_eq!(*term_lit(t.po[0].object[0].value()), Literal::Boolean(true));
    }

    // ── named blank node ──────────────────────────────────────────────────────

    #[test]
    fn test_named_blank_node() {
        let doc = parse("@prefix ex: <http://example.org/> . _:b0 ex:knows ex:alice .");
        let t = doc.triples[0].value();
        match term_bn(t.subject.value()) {
            BlankNode::Named(name, _) => assert_eq!(name, "b0"),
            other => panic!("expected Named blank node, got {:?}", other),
        }
    }

    // ── anonymous blank node property list ───────────────────────────────────

    #[test]
    fn test_anon_blank_node_property_list() {
        let doc =
            parse("@prefix ex: <http://example.org/> . ex:alice ex:knows [ ex:name \"Bob\" ] .");
        let t = doc.triples[0].value();
        match term_bn(t.po[0].object[0].value()) {
            BlankNode::Unnamed(pos, _, _) => {
                assert_eq!(pos.len(), 1);
                assert!(nn_eq(
                    term_nn(pos[0].predicate.value()),
                    &prefixed("ex", "name")
                ));
            }
            other => panic!("expected Unnamed blank node, got {:?}", other),
        }
    }

    // ── blank node as subject with property list ──────────────────────────────

    #[test]
    fn test_blank_node_subject_property_list() {
        let doc = parse("@prefix ex: <http://example.org/> . [ ex:name \"Alice\" ] ex:age 30 .");
        let t = doc.triples[0].value();
        assert!(matches!(
            t.subject.value(),
            Term::BlankNode(BlankNode::Unnamed(_, _, _))
        ));
        assert_eq!(t.po.len(), 1);
    }

    // ── collection ────────────────────────────────────────────────────────────

    #[test]
    fn test_collection() {
        let doc =
            parse("@prefix ex: <http://example.org/> . ex:alice ex:list ( ex:a ex:b ex:c ) .");
        let t = doc.triples[0].value();
        match t.po[0].object[0].value() {
            Term::Collection(items) => {
                assert_eq!(items.len(), 3);
                assert!(nn_eq(term_nn(items[0].value()), &prefixed("ex", "a")));
                assert!(nn_eq(term_nn(items[1].value()), &prefixed("ex", "b")));
                assert!(nn_eq(term_nn(items[2].value()), &prefixed("ex", "c")));
            }
            other => panic!("expected Collection, got {:?}", other),
        }
    }

    // ── fault-tolerant parsing ────────────────────────────────────────────────

    fn parse_raw(input: &str) -> crate::Parse {
        let (result, _) = crate_parse(lang::Rule::new(lang::SyntaxKind::TurtleDoc), input);
        result
    }

    #[test]
    fn test_valid_input_has_no_errors() {
        let p = parse_raw("@prefix ex: <http://example.org/> . ex:alice ex:age 30 .");
        assert_eq!(p.errors.len(), 0, "valid input should produce no errors");
    }

    #[test]
    fn test_missing_trailing_dot_reports_error() {
        let p = parse_raw("@prefix ex: <http://example.org/> . ex:alice ex:age 30");
        assert!(
            p.errors.len() > 0,
            "missing trailing dot should produce an error"
        );
        // The error should name the missing Stop (`.`) terminal
        assert!(
            p.errors.iter().any(|e| e.contains("Stop")),
            "expected an error mentioning Stop, got: {:?}",
            p.errors.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_missing_prefix_iri_reports_error() {
        // `@prefix ex:` with no IRI — missing the Iriref token
        let p = parse_raw("@prefix ex: .");
        assert!(
            p.errors.len() > 0,
            "missing prefix IRI should produce an error"
        );
    }

    #[test]
    fn test_recovery_missing_trailing_dot_still_extracts_triple() {
        // Even when the trailing `.` is absent the A* still builds a Statement
        // node with an Error child, and the converter should extract the triple.
        let doc = parse("@prefix ex: <http://example.org/> . ex:alice ex:age 30");
        assert_eq!(doc.triples.len(), 1);
        assert!(nn_eq(
            term_nn(doc.triples[0].subject.value()),
            &prefixed("ex", "alice")
        ));
        match term_lit(doc.triples[0].po[0].object[0].value()) {
            Literal::Numeric(n) => assert_eq!(n, "30"),
            other => panic!("expected Numeric, got {:?}", other),
        }
    }

    #[test]
    fn test_recovery_second_triple_after_missing_dot() {
        // The first triple is missing its `.`.  The A* should still produce two
        // Statement nodes — one with an error, one clean — giving us two triples.
        let input = "@prefix ex: <http://example.org/> .\n\
                     ex:alice ex:age 30\n\
                     ex:bob ex:age 25 .";
        let p = parse_raw(input);
        let root = p.syntax::<lang::Lang>();
        println!("Parse tree:\n{:#?}", root);
        assert!(p.errors.len() > 0, "should report at least one error");

        let doc = parse(input);
        assert_eq!(doc.triples.len(), 2, "both triples should be recovered");
        assert!(nn_eq(
            term_nn(doc.triples[0].subject.value()),
            &prefixed("ex", "alice")
        ));
        assert!(nn_eq(
            term_nn(doc.triples[1].subject.value()),
            &prefixed("ex", "bob")
        ));
    }

    // ── multiple triples ──────────────────────────────────────────────────────

    #[test]
    fn test_multiple_triples() {
        let doc =
            parse("@prefix ex: <http://example.org/> . ex:alice ex:age 30 . ex:bob ex:age 25 .");
        assert_eq!(doc.triples.len(), 2);
        assert!(nn_eq(
            term_nn(doc.triples[0].subject.value()),
            &prefixed("ex", "alice")
        ));
        assert!(nn_eq(
            term_nn(doc.triples[1].subject.value()),
            &prefixed("ex", "bob")
        ));
    }

    // ── incremental parsing ───────────────────────────────────────────────────

    use crate::{IncrementalBias, PrevParseInfo, TokenTrait, parse_incremental};

    fn prev_info(text: &str) -> PrevParseInfo {
        let (_, tokens) = crate_parse(lang::Rule::new(lang::SyntaxKind::TurtleDoc), text);
        let mut depth: i32 = 0;
        PrevParseInfo {
            tokens: tokens
                .iter()
                .map(|t| {
                    let d = depth.clamp(0, 255) as u8;
                    depth += t.kind.bracket_delta() as i32;
                    t.to_prev_token(d)
                })
                .collect(),
            had_errors: false,
        }
    }

    /// Going from `<b> <c> <d> .` to `<a> <b> <c> <d> .` (inserting `<a>`
    /// at the front), the incremental parser should produce two triples:
    ///
    ///   1. `<a>` as subject with error-recovery predicate/object (empty IRIs)
    ///   2. `<b> <c> <d>` — the original triple, roles preserved
    ///
    /// This works because `expect_as` enqueues a fallback element (with
    /// lower cost) whenever a token matches but conflicts with its old role,
    /// letting the A* explore paths where the conflicting token begins a new
    /// parse context (e.g. as Subject of the next statement).
    #[test]
    fn test_incremental_two_triples_from_inserted_token() {
        let prev = prev_info("<b> <c> <d> .");
        let bias = IncrementalBias::default();
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            "<a> <b> <c> <d> .",
            Some(&prev),
            bias,
        );
        let root = parse.syntax::<lang::Lang>();
        let doc = convert(&root);

        assert_eq!(doc.triples.len(), 2, "should produce two triples");

        // Triple 1: <a> as subject (error recovery — predicate/object are
        // error nodes with empty IRIs, representing the undefined slots)
        match doc.triples[0].value().subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "a"),
            other => panic!("expected <a> as first subject, got {:?}", other),
        }

        // Triple 2: <b> <c> <d> — original roles preserved
        let t2 = doc.triples[1].value();
        match t2.subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "b"),
            other => panic!("expected <b> as subject of second triple, got {:?}", other),
        }
        assert_eq!(t2.po.len(), 1);
        match t2.po[0].value().predicate.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "c"),
            other => panic!("expected <c> as predicate, got {:?}", other),
        }
        assert_eq!(t2.po[0].value().object.len(), 1);
        match t2.po[0].value().object[0].value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "d"),
            other => panic!("expected <d> as object, got {:?}", other),
        }
    }

    #[test]
    fn test_incremental_remove_object() {
        let prev = prev_info("<a> <b> <c> .");
        let bias = IncrementalBias::default();
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            "<a> <b> .",
            Some(&prev),
            bias,
        );
        let root = parse.syntax::<lang::Lang>();
        let doc = convert(&root);

        assert_eq!(doc.triples.len(), 1, "should produce two triples");

        // Triple 1: <a> as subject (error recovery — predicate/object are
        // error nodes with empty IRIs, representing the undefined slots)
        match doc.triples[0].value().subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "a"),
            other => panic!("expected <a> as first subject, got {:?}", other),
        }
    }

    #[test]
    fn test_incremental_remove_prefix() {
        let prev =
            prev_info("@prefix foaf: <http://xmlns.com/foaf/0.1/> .\n <a> foaf:knows ex:Bob .");
        let bias = IncrementalBias::default();
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            "<a> foaf:knows ex:Bob .",
            Some(&prev),
            bias,
        );
        let root = parse.syntax::<lang::Lang>();
        let doc = convert(&root);

        assert_eq!(doc.triples.len(), 1, "should produce one triples");

        // Triple 1: <a> as subject (error recovery — predicate/object are
        // error nodes with empty IRIs, representing the undefined slots)
        match doc.triples[0].value().subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "a"),
            other => panic!("expected <a> as first subject, got {:?}", other),
        }
    }

    #[test]
    fn test_suggest_correct_sq_close_location() {
        let before = r#"ex:Bob foaf: [
  foaf:knows [
    foaf:name "Alice"
  ];
  foaf:name "Bob";
 ]."#;

        let after = r#"ex:Bob foaf: [
  foaf:knows [
    foaf:name "Alice"
  ;
  foaf:name "Bob";
 ]."#;
        let prev = prev_info(before);
        let bias = IncrementalBias::default();
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            after,
            Some(&prev),
            bias,
        );
        let root = parse.syntax::<lang::Lang>();

        let first_location = before.find(']').unwrap_or_default();
        let error_span = root
            .descendants_with_tokens()
            .filter(|t| t.kind() == lang::SyntaxKind::Error)
            .map(|t| t.text_range())
            .next()
            .expect("has an error");

        println!("error span {:?} location {}", error_span, first_location);
        let loc = TextSize::new(first_location as u32);
        assert!(
            error_span.contains(loc) || error_span.start() == loc,
            "The error span should start at or include the location of the removed bracket"
        );
    }

    #[test]
    fn test_suggest_completing_triple() {
        let before = r#"ex:Alice a foaf:Person ;
    foaf:knows ex:Bob . "#;

        let after = r#"ex:Alice a foaf:Person ; <a>
    foaf:knows ex:Bob . "#;
        let prev = prev_info(before);
        let bias = IncrementalBias::default();
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            after,
            Some(&prev),
            bias,
        );

        let root = parse.syntax::<lang::Lang>();
        let doc = convert(&root);
        assert_eq!(doc.triples.len(), 1, "should produce one triples");
        assert_eq!(
            doc.triples[0].0.po.len(),
            3,
            "should produce three predicate objects"
        );

        let errors: Vec<_> = parse.errors.iter().collect();

        assert_eq!(errors.len(), 2, "expect a term and a semi colon");
    }

    #[test]
    fn test_suggest_completing_triple_2() {
        let before = r#"ex:Alice a foaf:Person ;
    foaf:knows ex:Bob . "#;

        let after_1 = r#"ex:Alice a foaf:Person ; <a>
    foaf:knows ex:Bob . "#;

        let after_2 = r#"ex:Alice a foaf:Person ; <a> <b>
    foaf:knows ex:Bob . "#;
        let prev = prev_info(before);
        let bias = IncrementalBias::default();
        let (_, prev) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            after_1,
            Some(&prev),
            bias,
        );
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            after_2,
            Some(&prev),
            bias,
        );

        let root = parse.syntax::<lang::Lang>();
        let doc = convert(&root);
        assert_eq!(doc.triples.len(), 1, "should produce one triples");
        assert_eq!(
            doc.triples[0].0.po.len(),
            3,
            "should produce three predicate objects"
        );

        let errors: Vec<_> = parse.errors.iter().collect();

        assert_eq!(errors.len(), 1, "expect a semi colon");
    }

    #[test]
    fn test_suggest_inside_blank_node() {
        let before = r#"<a> a <b> ;
    <knows> [  ] ."#;

        let after_1 = r#"<a> a <b> ;
    <knows> [  ] ."#;
        //     let after_1 = r#"<a> a <b> ;
        // <knows> [ <nam ] ."#;

        let after_2 = r#"<a> a <b> ;
    <knows> [ <name> ] ."#;

        let prev = prev_info(before);
        let bias = IncrementalBias::default();

        let (_, prev) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            after_1,
            Some(&prev),
            bias,
        );
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            after_2,
            Some(&prev),
            bias,
        );

        let root = parse.syntax::<lang::Lang>();
        let doc = convert(&root);
        assert_eq!(doc.triples.len(), 1, "should produce one triples");
        assert_eq!(
            doc.triples[0].0.po.len(),
            2,
            "should produce two predicate objects"
        );
    }

    #[test]
    fn test_changing_whitespace_does_not_influence_errors() {
        let before = r#"<a> a <b> . "#;

        let after_1 = r#"<a>  <b> . "#;
        let after_2 = r#"<a> <b> . "#;

        let prev = prev_info(before);
        let bias = IncrementalBias::default();

        let (parse, prev) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            after_1,
            Some(&prev),
            bias,
        );
        println!("First errors {:?}", parse.errors);
        assert!(
            parse
                .errors
                .iter()
                .any(|x| x.to_lowercase().contains("verb")),
            "expected a verb"
        );
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            after_2,
            Some(&prev),
            bias,
        );

        println!("Later errors {:?}", parse.errors);
        assert!(
            parse
                .errors
                .iter()
                .any(|x| x.to_lowercase().contains("verb")),
            "expected a verb"
        );
    }

    /// When `a` (the rdf:type shorthand verb) is removed from `<s> a <o> .`,
    /// the parser must report a missing verb.  The error should name the `Verb`
    /// grammar rule, not the low-level `Alit` terminal.
    #[test]
    fn test_remove_rdf_type_verb_error_names_verb_rule() {
        let prev = prev_info("<s> a <o> .");
        let bias = IncrementalBias::default();
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            "<s> <o> .",
            Some(&prev),
            bias,
        );

        let errors: Vec<_> = parse.errors.iter().collect();
        assert!(
            !errors.is_empty(),
            "removing the verb should produce an error, got none"
        );
        assert!(
            errors.iter().any(|e| e.to_lowercase().contains("verb")),
            "error should name the Verb grammar rule, got: {:?}",
            errors
        );
        assert!(
            !errors.iter().any(|e| e.to_lowercase().contains("alit")),
            "error should not expose the low-level Alit terminal, got: {:?}",
            errors
        );
    }

    /// Removing the `a` verb from `<b> a <c> .` incrementally should produce
    /// an error that names the Verb grammar rule.  The raw CST error node is
    /// zero-width, but `effective_error_span` widens it to cover the whitespace
    /// gap between `<b>` and `<c>` — a span of 1.
    #[test]
    fn test_remove_verb_from_iri_triple_reports_verb_error() {
        let prev = prev_info("<b> a <c> .");
        let bias = IncrementalBias::default();
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TurtleDoc),
            "<b> <c> .",
            Some(&prev),
            bias,
        );

        let errors: Vec<_> = parse.errors.iter().collect();
        assert!(
            errors.iter().any(|e| e.to_lowercase().contains("verb")),
            "error should name the Verb grammar rule, got: {:?}",
            errors
        );

        let root = parse.syntax::<lang::Lang>();
        let error_node = root
            .descendants()
            .find(|n| n.kind() == lang::SyntaxKind::Error)
            .expect("should have an error node");
        let effective = crate::effective_error_span::<lang::Lang>(&error_node);
        assert_eq!(
            effective.end - effective.start,
            1,
            "effective_error_span should cover the whitespace gap (1 byte), got {:?}",
            effective
        );
    }

    // ── fast (non-fault-tolerant) parsing ─────────────────────────────────────

    use crate::parse_fast;

    fn parse_fast_doc(input: &str) -> Option<Turtle> {
        let (result, _) = parse_fast(lang::Rule::new(lang::SyntaxKind::TurtleDoc), input)?;
        let root = result.syntax::<lang::Lang>();
        Some(convert(&root))
    }

    fn parse_fast_raw(input: &str) -> Option<crate::Parse> {
        let (result, _) = parse_fast(lang::Rule::new(lang::SyntaxKind::TurtleDoc), input)?;
        Some(result)
    }

    /// A well-formed document should succeed and produce no errors.
    #[test]
    fn test_fast_valid_simple_triple() {
        let doc = parse_fast_doc("<http://a> <http://b> <http://c> .")
            .expect("valid input should return Some");
        assert_eq!(doc.triples.len(), 1);
    }

    /// parse_fast must return None (not panic) when the document has errors.
    #[test]
    fn test_fast_returns_none_on_error() {
        // Missing trailing dot
        assert!(
            parse_fast_doc("<http://a> <http://b> <http://c>").is_none(),
            "document with missing dot should return None"
        );
    }

    /// Syntax errors mid-document (extra subject with no predicate) must yield None.
    #[test]
    fn test_fast_returns_none_on_mid_document_error() {
        // A valid triple followed by a subject with no predicate or object —
        // a genuine grammar-level error that can't be skipped at the lexer level.
        assert!(
            parse_fast_doc("<http://a> <http://b> <http://c> . <http://dangling>").is_none(),
            "incomplete trailing statement should return None"
        );
    }

    /// For a correct document, fast mode should produce no CST errors.
    #[test]
    fn test_fast_produces_no_errors() {
        let p = parse_fast_raw("@prefix ex: <http://example.org/> . ex:alice ex:age 30 .")
            .expect("valid input should return Some");
        assert_eq!(
            p.errors.len(),
            0,
            "fast parse of valid input should have no errors"
        );
    }

    /// Fast mode should correctly parse prefix declarations.
    #[test]
    fn test_fast_prefix_directive() {
        let doc = parse_fast_doc("@prefix ex: <http://example.org/> . ex:alice ex:knows ex:bob .")
            .expect("valid input should return Some");
        assert_eq!(doc.prefixes.len(), 1);
        assert_eq!(doc.triples.len(), 1);
        assert!(nn_eq(
            term_nn(doc.triples[0].subject.value()),
            &prefixed("ex", "alice")
        ));
    }

    /// Fast mode should correctly parse multiple triples.
    #[test]
    fn test_fast_multiple_triples() {
        let doc = parse_fast_doc(
            "@prefix ex: <http://example.org/> . ex:alice ex:age 30 . ex:bob ex:age 25 .",
        )
        .expect("valid input should return Some");
        assert_eq!(doc.triples.len(), 2);
        assert!(nn_eq(
            term_nn(doc.triples[0].subject.value()),
            &prefixed("ex", "alice")
        ));
        assert!(nn_eq(
            term_nn(doc.triples[1].subject.value()),
            &prefixed("ex", "bob")
        ));
    }

    /// Fast mode result should match regular parse for a correct document.
    #[test]
    fn test_fast_matches_regular_parse() {
        let input = "@prefix ex: <http://example.org/> . ex:alice ex:name \"Alice\" ; ex:age 30 .";
        let fast_doc = parse_fast_doc(input).expect("valid input should return Some");
        let regular_doc = parse(input);
        assert_eq!(fast_doc.triples.len(), regular_doc.triples.len());
        assert_eq!(fast_doc.prefixes.len(), regular_doc.prefixes.len());
    }

    /// Fast mode should handle blank node property lists.
    #[test]
    fn test_fast_blank_node_property_list() {
        let doc = parse_fast_doc(
            "@prefix ex: <http://example.org/> . ex:alice ex:knows [ ex:name \"Bob\" ] .",
        )
        .expect("valid input should return Some");
        assert_eq!(doc.triples.len(), 1);
    }

    /// Fast mode should handle collections.
    #[test]
    fn test_fast_collection() {
        let doc = parse_fast_doc(
            "@prefix ex: <http://example.org/> . ex:alice ex:list ( ex:a ex:b ex:c ) .",
        )
        .expect("valid input should return Some");
        assert_eq!(doc.triples.len(), 1);
    }
}
