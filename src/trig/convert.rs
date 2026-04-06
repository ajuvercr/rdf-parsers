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
    // TrigDoc ::= (directive | block)*
    // Children of the root are Directive and Block nodes directly.
    let mut base = None;
    let mut prefixes = Vec::new();
    let mut triples = Vec::new();

    for child_node in root.children() {
        let kind = child_node.kind();
        if kind == SyntaxKind::Directive {
            if let Some(b) = child(&child_node, SyntaxKind::Base)
                .or_else(|| child(&child_node, SyntaxKind::SparqlBase))
            {
                base = Some(Spanned(convert_base(&b), text_range(&b)));
            } else if let Some(p) = child(&child_node, SyntaxKind::PrefixId)
                .or_else(|| child(&child_node, SyntaxKind::SparqlPrefix))
            {
                prefixes.push(Spanned(convert_prefix(&p), text_range(&p)));
            }
        } else if kind == SyntaxKind::Block {
            convert_block(&child_node, &mut triples);
        }
    }

    Turtle::new(base, prefixes, triples)
}

// ── trig-specific conversion ─────────────────────────────────────────────────

fn convert_block(block: &Node, triples: &mut Vec<Spanned<Triple>>) {
    // block ::= graphBlock | triples2 | "GRAPH" labelOrSubject wrappedGraph | triplesOrGraph
    // Check GraphToken first: the "GRAPH" variant also has a WrappedGraph child.
    if child(block, SyntaxKind::GraphToken).is_some() {
        // "GRAPH" labelOrSubject wrappedGraph
        let graph_term = child(block, SyntaxKind::LabelOrSubject).map(|los| {
            let range = text_range(&los);
            Spanned(convert_label_or_subject(&los), range)
        });
        if let Some(wg) = child(block, SyntaxKind::WrappedGraph) {
            convert_wrapped_graph(&wg, graph_term, triples);
        }
    } else if let Some(gb) = child(block, SyntaxKind::GraphBlock) {
        convert_graph_block(&gb, triples);
    } else if let Some(tog) = child(block, SyntaxKind::TriplesOrGraph) {
        convert_triples_or_graph(&tog, triples);
    } else if let Some(t2) = child(block, SyntaxKind::Triples2) {
        let range = text_range(block);
        triples.push(Spanned(convert_triples2(&t2), range));
    }
}

fn convert_graph_block(node: &Node, triples: &mut Vec<Spanned<Triple>>) {
    // graphBlock ::= labelOrSubject? wrappedGraph
    // The optional labelOrSubject is the named-graph IRI; absent means default graph.
    let graph_term = child(node, SyntaxKind::LabelOrSubject).map(|los| {
        let range = text_range(&los);
        Spanned(convert_label_or_subject(&los), range)
    });
    if let Some(wg) = child(node, SyntaxKind::WrappedGraph) {
        convert_wrapped_graph(&wg, graph_term, triples);
    }
}

fn convert_triples_or_graph(node: &Node, triples: &mut Vec<Spanned<Triple>>) {
    // triplesOrGraph ::= labelOrSubject predicateObjectList '.'
    let subject = child(node, SyntaxKind::LabelOrSubject)
        .map(|l| {
            let range = text_range(&l);
            Spanned(convert_label_or_subject(&l), range)
        })
        .unwrap_or_else(|| Spanned(Term::Invalid, text_range(node)));

    let po = child(node, SyntaxKind::PredicateObjectList)
        .map(|n| convert_predicate_object_list(&n))
        .unwrap_or_default();

    let range = text_range(node);
    triples.push(Spanned(
        Triple {
            subject,
            po,
            graph: None,
        },
        range,
    ));
}

fn convert_wrapped_graph(
    node: &Node,
    graph: Option<Spanned<Term>>,
    triples: &mut Vec<Spanned<Triple>>,
) {
    // wrappedGraph ::= '{' triplesBlock? '}'
    if let Some(tb) = child(node, SyntaxKind::TriplesBlock) {
        convert_triples_block(&tb, &graph, triples);
    }
}

fn convert_triples_block(
    node: &Node,
    graph: &Option<Spanned<Term>>,
    triples: &mut Vec<Spanned<Triple>>,
) {
    // triplesBlock ::= triples ('.' triplesBlock?)?
    if let Some(t) = child(node, SyntaxKind::Triples) {
        let range = text_range(&t);
        let mut triple = convert_triples(&t);
        triple.graph = graph.clone();
        triples.push(Spanned(triple, range));
    }
    // Recurse into nested triplesBlock
    if let Some(tb) = child(node, SyntaxKind::TriplesBlock) {
        convert_triples_block(&tb, graph, triples);
    }
}

fn convert_triples2(node: &Node) -> Triple {
    // triples2 ::= blankNodePropertyList predicateObjectList? '.' | collection predicateObjectList '.'
    let (subject, po_node) = if let Some(bpl) = child(node, SyntaxKind::BlankNodePropertyList) {
        let range = text_range(&bpl);
        let subject = convert_blank_node_property_list(&bpl);
        (
            Spanned(subject, range),
            child(node, SyntaxKind::PredicateObjectList),
        )
    } else if let Some(coll) = child(node, SyntaxKind::Collection) {
        let range = text_range(&coll);
        (
            Spanned(Term::Collection(convert_collection(&coll)), range),
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

fn convert_label_or_subject(node: &Node) -> Term {
    // labelOrSubject ::= iri | BlankNode
    if let Some(iri) = child(node, SyntaxKind::Iri) {
        Term::NamedNode(convert_iri(&iri))
    } else if let Some(bn) = child(node, SyntaxKind::BlankNode) {
        Term::BlankNode(convert_blank_node(&bn))
    } else {
        Term::Invalid
    }
}

// ── shared conversion (same rules as turtle, different SyntaxKind type) ──────

fn convert_base(node: &Node) -> Base {
    let range = text_range(node);
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

    let prefix_text = child(node, SyntaxKind::PnameNs)
        .map(|n| {
            let text = terminal_text(&n);
            let tr = text_range(&n);
            Spanned(text.trim_end_matches(':').to_string(), tr)
        })
        .unwrap_or_else(|| Spanned(String::new(), range.clone()));

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
    // triples ::= subject predicateObjectList | blankNodePropertyList predicateObjectList?
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
    // In trig: subject ::= iri | blank
    // blank ::= BlankNode | collection
    if let Some(iri) = child(node, SyntaxKind::Iri) {
        Term::NamedNode(convert_iri(&iri))
    } else if let Some(blank) = child(node, SyntaxKind::Blank) {
        convert_blank(&blank)
    } else {
        Term::Invalid
    }
}

fn convert_blank(node: &Node) -> Term {
    // blank ::= BlankNode | collection
    if let Some(bn) = child(node, SyntaxKind::BlankNode) {
        Term::BlankNode(convert_blank_node(&bn))
    } else if let Some(coll) = child(node, SyntaxKind::Collection) {
        Term::Collection(convert_collection(&coll))
    } else {
        Term::Invalid
    }
}

fn convert_object(node: &Node) -> Term {
    // In trig: object ::= iri | blank | blankNodePropertyList | literal
    if let Some(iri) = child(node, SyntaxKind::Iri) {
        Term::NamedNode(convert_iri(&iri))
    } else if let Some(blank) = child(node, SyntaxKind::Blank) {
        convert_blank(&blank)
    } else if let Some(bpl) = child(node, SyntaxKind::BlankNodePropertyList) {
        convert_blank_node_property_list(&bpl)
    } else if let Some(lit) = child(node, SyntaxKind::Literal) {
        Term::Literal(convert_literal(&lit))
    } else {
        Term::Invalid
    }
}

fn convert_iri(node: &Node) -> NamedNode {
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
    if let Some(label) = child(node, SyntaxKind::BlankNodeLabel) {
        let text = terminal_text(&label);
        let offset: usize = label.text_range().start().into();
        let name = text.strip_prefix("_:").unwrap_or(&text);
        BlankNode::Named(name.to_string(), offset)
    } else {
        let offset: usize = node.text_range().start().into();
        BlankNode::Unnamed(Vec::new(), offset, offset)
    }
}

fn convert_blank_node_property_list(node: &Node) -> Term {
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
    let offset: usize = node.text_range().start().into();

    let (value, quote_style, len) = if let Some(str_node) = child(node, SyntaxKind::MyString) {
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
    use super::*;
    use crate::{
        IncrementalBias, PrevParseInfo, TokenTrait, parse as crate_parse, parse_incremental,
        trig::parser as lang,
    };

    fn parse(input: &str) -> Turtle {
        let (result, _) = crate_parse(lang::Rule::new(lang::SyntaxKind::TrigDoc), input);
        let root = result.syntax::<lang::Lang>();
        convert(&root)
    }

    fn prev_info(text: &str) -> PrevParseInfo {
        let (_, tokens) = crate_parse(lang::Rule::new(lang::SyntaxKind::TrigDoc), text);
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

    fn parse_incr(before: &str, after: &str) -> Turtle {
        let prev = prev_info(before);
        let bias = IncrementalBias::default();
        let (result, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TrigDoc),
            after,
            Some(&prev),
            bias,
        );
        let root = result.syntax::<lang::Lang>();
        convert(&root)
    }

    // ── helpers ──────────────────────────────────────────────────────────────

    fn prefixed(prefix: &str, value: &str) -> NamedNode {
        NamedNode::Prefixed {
            prefix: prefix.to_string(),
            value: value.to_string(),
            idx: 0,
        }
    }

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

    // ── directives ───────────────────────────────────────────────────────────

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
    fn test_base_directive() {
        let doc = parse("@base <http://example.org/> .");
        let base = doc.base.as_ref().expect("base should be present");
        assert!(nn_eq(
            base.value().1.value(),
            &NamedNode::Full("http://example.org/".to_string(), 0)
        ));
    }

    // ── default graph triples (triplesOrGraph with predicateObjectList) ──────

    #[test]
    fn test_simple_triple() {
        let doc = parse("@prefix ex: <http://example.org/> . ex:alice ex:knows ex:bob .");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(term_nn(t.subject.value()), &prefixed("ex", "alice")));
        assert!(nn_eq(
            term_nn(t.po[0].object[0].value()),
            &prefixed("ex", "bob")
        ));
        assert!(t.graph.is_none());
    }

    #[test]
    fn test_full_iri_triple() {
        let doc = parse(
            "<http://example.org/alice> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Person> .",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(
            term_nn(t.subject.value()),
            &NamedNode::Full("http://example.org/alice".to_string(), 0)
        ));
    }

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
    }

    #[test]
    fn test_multiple_po_pairs() {
        let doc =
            parse("@prefix ex: <http://example.org/> . ex:alice ex:name \"Alice\" ; ex:age 30 .");
        let t = doc.triples[0].value();
        assert_eq!(t.po.len(), 2);
    }

    #[test]
    fn test_multiple_objects() {
        let doc =
            parse("@prefix ex: <http://example.org/> . ex:alice ex:knows ex:bob , ex:carol .");
        let t = doc.triples[0].value();
        assert_eq!(t.po[0].object.len(), 2);
    }

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
    fn test_numeric_literal() {
        let doc = parse("@prefix ex: <http://example.org/> . ex:alice ex:age 30 .");
        let t = doc.triples[0].value();
        match term_lit(t.po[0].object[0].value()) {
            Literal::Numeric(n) => assert_eq!(n, "30"),
            other => panic!("expected Numeric literal, got {:?}", other),
        }
    }

    #[test]
    fn test_boolean_literal() {
        let doc = parse("@prefix ex: <http://example.org/> . ex:alice ex:active true .");
        let t = doc.triples[0].value();
        assert_eq!(*term_lit(t.po[0].object[0].value()), Literal::Boolean(true));
    }

    #[test]
    fn test_named_blank_node() {
        let doc = parse("@prefix ex: <http://example.org/> . _:b0 ex:knows ex:alice .");
        let t = doc.triples[0].value();
        match term_bn(t.subject.value()) {
            BlankNode::Named(name, _) => assert_eq!(name, "b0"),
            other => panic!("expected Named blank node, got {:?}", other),
        }
    }

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

    // ── default graph wrapped in braces ──────────────────────────────────────

    #[test]
    fn test_default_graph_wrapped() {
        let doc = parse("@prefix ex: <http://example.org/> . { ex:alice ex:knows ex:bob . }");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(term_nn(t.subject.value()), &prefixed("ex", "alice")));
        assert!(t.graph.is_none());
    }

    #[test]
    fn test_default_graph_multiple_triples() {
        let doc = parse(
            "@prefix ex: <http://example.org/> . { ex:alice ex:knows ex:bob . ex:bob ex:knows ex:carol . }",
        );
        assert_eq!(doc.triples.len(), 2);
        assert!(doc.triples[0].value().graph.is_none());
        assert!(doc.triples[1].value().graph.is_none());
    }

    // ── named graph with GRAPH keyword ───────────────────────────────────────

    #[test]
    fn test_named_graph_with_keyword() {
        let doc =
            parse("@prefix ex: <http://example.org/> . GRAPH ex:g1 { ex:alice ex:knows ex:bob . }");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(term_nn(t.subject.value()), &prefixed("ex", "alice")));
        let graph = t.graph.as_ref().expect("should have graph");
        assert!(nn_eq(term_nn(graph.value()), &prefixed("ex", "g1")));
    }

    #[test]
    fn test_named_graph_multiple_triples() {
        let doc = parse(
            "@prefix ex: <http://example.org/> . GRAPH ex:g1 { ex:alice ex:knows ex:bob . ex:bob ex:knows ex:carol . }",
        );
        assert_eq!(doc.triples.len(), 2);
        for t in &doc.triples {
            let graph = t.value().graph.as_ref().expect("should have graph");
            assert!(nn_eq(term_nn(graph.value()), &prefixed("ex", "g1")));
        }
    }

    #[test]
    fn test_named_graph_full_iri() {
        let doc = parse(
            "GRAPH <http://example.org/g1> { <http://example.org/alice> <http://example.org/knows> <http://example.org/bob> . }",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        let graph = t.graph.as_ref().expect("should have graph");
        assert!(nn_eq(
            term_nn(graph.value()),
            &NamedNode::Full("http://example.org/g1".to_string(), 0)
        ));
    }

    // ── named graph via triplesOrGraph (iri followed by wrappedGraph) ────────

    #[test]
    fn test_named_graph_without_keyword() {
        let doc = parse("@prefix ex: <http://example.org/> . ex:g1 { ex:alice ex:knows ex:bob . }");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(term_nn(t.subject.value()), &prefixed("ex", "alice")));
        let graph = t.graph.as_ref().expect("should have graph");
        assert!(nn_eq(term_nn(graph.value()), &prefixed("ex", "g1")));
    }

    // ── triples2 (blankNodePropertyList as subject at block level) ───────────

    #[test]
    fn test_triples2_blank_node_property_list() {
        let doc = parse("@prefix ex: <http://example.org/> . [ ex:name \"Alice\" ] ex:age 30 .");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(matches!(
            t.subject.value(),
            Term::BlankNode(BlankNode::Unnamed(_, _, _))
        ));
        assert_eq!(t.po.len(), 1);
    }

    // ── sparql-style directives ──────────────────────────────────────────────

    #[test]
    fn test_sparql_prefix_directive() {
        let doc = parse("PREFIX ex: <http://example.org/> ex:alice ex:knows ex:bob .");
        assert_eq!(doc.prefixes.len(), 1);
        assert_eq!(doc.prefixes[0].value().prefix.value(), "ex");
    }

    #[test]
    fn test_sparql_base_directive() {
        let doc = parse("BASE <http://example.org/> <alice> <knows> <bob> .");
        let base = doc.base.as_ref().expect("base should be present");
        assert!(nn_eq(
            base.value().1.value(),
            &NamedNode::Full("http://example.org/".to_string(), 0)
        ));
    }

    // ── string literal with datatype ─────────────────────────────────────────

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

    // ── multiple triples (no graph) ──────────────────────────────────────────

    #[test]
    fn test_multiple_triples() {
        let doc =
            parse("@prefix ex: <http://example.org/> . ex:alice ex:age 30 . ex:bob ex:age 25 .");
        assert_eq!(doc.triples.len(), 2);
        assert!(nn_eq(
            term_nn(doc.triples[0].value().subject.value()),
            &prefixed("ex", "alice")
        ));
        assert!(nn_eq(
            term_nn(doc.triples[1].value().subject.value()),
            &prefixed("ex", "bob")
        ));
    }

    // ── fault-tolerant parsing ────────────────────────────────────────────────

    fn parse_raw(input: &str) -> crate::Parse {
        let (result, _) = crate_parse(lang::Rule::new(lang::SyntaxKind::TrigDoc), input);
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
        assert!(
            p.errors.iter().any(|e| e.contains("Stop")),
            "expected an error mentioning Stop, got: {:?}",
            p.errors.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_missing_prefix_iri_reports_error() {
        let p = parse_raw("@prefix ex: .");
        assert!(
            p.errors.len() > 0,
            "missing prefix IRI should produce an error"
        );
    }

    // ── mixed content ────────────────────────────────────────────────────────

    #[test]
    fn test_mixed_default_and_named() {
        let doc = parse(
            "@prefix ex: <http://example.org/> .\n\
             ex:s1 ex:p1 ex:o1 .\n\
             GRAPH ex:g1 { ex:s2 ex:p2 ex:o2 . }",
        );
        assert_eq!(doc.triples.len(), 2);
        assert!(doc.triples[0].value().graph.is_none());
        assert!(doc.triples[1].value().graph.is_some());
    }

    #[test]
    fn test_empty_wrapped_graph() {
        let doc = parse("@prefix ex: <http://example.org/> . GRAPH ex:g1 { }");
        assert_eq!(doc.triples.len(), 0);
    }

    #[test]
    fn test_empty_default_graph() {
        let doc = parse("{ }");
        assert_eq!(doc.triples.len(), 0);
    }

    #[test]
    fn test_named_graph_blank_node_graph_name() {
        let doc =
            parse("@prefix ex: <http://example.org/> . GRAPH _:g1 { ex:alice ex:knows ex:bob . }");
        assert_eq!(doc.triples.len(), 1);
        let graph = doc.triples[0]
            .value()
            .graph
            .as_ref()
            .expect("should have graph");
        match graph.value() {
            Term::BlankNode(BlankNode::Named(name, _)) => assert_eq!(name, "g1"),
            other => panic!("expected Named blank node graph, got {:?}", other),
        }
    }

    // ── incremental: remove graph IRI ────────────────────────────────────────

    /// Going from `{ <b> <c> <d> }` to `<b> <c> <d> .` (removing the wrapping
    /// curly braces), the incremental parser should produce one triple in the
    /// default graph with no errors.  The inner tokens shift from bracket depth
    /// 1 to depth 0; the depth-delta mechanism pays the adoption cost once and
    /// gives subsequent tokens a free pass.
    #[test]
    fn test_incremental_remove_curly_braces() {
        let before = "{ <b> <c> <d> }";
        let after = "<b> <c> <d> }";

        let (result, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::TrigDoc),
            after,
            Some(&prev_info(before)),
            IncrementalBias::default(),
        );
        assert!(
            result.errors.len() == 1,
            "expected one parse errors, got: {:?}",
            result.errors.iter().collect::<Vec<_>>()
        );

        let root = result.syntax::<lang::Lang>();
        println!("{:#?}", root);
        let doc = convert(&root);

        assert_eq!(doc.triples.len(), 1, "should produce exactly one triple");

        let t = doc.triples[0].value();
        assert!(t.graph.is_none(), "triple should be in the default graph");

        match t.subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "b"),
            other => panic!("expected <b> as subject, got {:?}", other),
        }
        match t.po[0].value().predicate.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "c"),
            other => panic!("expected <c> as predicate, got {:?}", other),
        }
        match t.po[0].value().object[0].value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "d"),
            other => panic!("expected <d> as object, got {:?}", other),
        }
    }

    /// Going from `<a> { <b> <c> <d> }` to `{ <b> <c> <d> }` (dropping the
    /// named-graph IRI), the incremental parser should produce one triple in
    /// the default graph with subject <b>, predicate <c>, object <d>.
    #[test]
    fn test_incremental_remove_graph_iri() {
        let before = "<a> { <b> <c> <d> }";
        let after = "{ <b> <c> <d> }";

        let doc = parse_incr(before, after);

        assert_eq!(doc.triples.len(), 1, "should produce exactly one triple");

        let t = doc.triples[0].value();
        assert!(t.graph.is_none(), "triple should be in the default graph");

        match t.subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "b"),
            other => panic!("expected <b> as subject, got {:?}", other),
        }
        match t.po[0].value().predicate.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "c"),
            other => panic!("expected <c> as predicate, got {:?}", other),
        }
        match t.po[0].value().object[0].value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "d"),
            other => panic!("expected <d> as object, got {:?}", other),
        }
    }

    /// Add a named-graph IRI: `{ <b> <c> <d> }` → `<g> { <b> <c> <d> }`
    /// The tokens inside the graph shift context; the A* must find the named-graph
    /// parse within budget, exercising graph_token and curly-brace weights.
    #[test]
    fn test_incremental_add_graph_iri() {
        let doc = parse_incr("{ <b> <c> <d> }", "<g> { <b> <c> <d> }");

        assert_eq!(doc.triples.len(), 1, "should produce exactly one triple");

        let t = doc.triples[0].value();
        match t.graph.as_ref().map(|g| g.value()) {
            Some(Term::NamedNode(NamedNode::Full(iri, _))) => assert_eq!(iri, "g"),
            other => panic!("expected <g> as graph name, got {:?}", other),
        }
        match t.subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "b"),
            other => panic!("expected <b> as subject, got {:?}", other),
        }
    }

    /// Change the object inside a default-graph block:
    /// `{ <b> <c> <d> }` → `{ <b> <c> <e> }`.  Only one token changes; the
    /// curly-brace tokens are unchanged, exercising depth-delta stability.
    #[test]
    fn test_incremental_change_object_inside_graph() {
        let doc = parse_incr("{ <b> <c> <d> }", "{ <b> <c> <e> }");

        assert_eq!(doc.triples.len(), 1, "should produce exactly one triple");

        let t = doc.triples[0].value();
        assert!(t.graph.is_none(), "triple should be in the default graph");
        match t.po[0].value().object[0].value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "e"),
            other => panic!("expected <e> as object, got {:?}", other),
        }
    }

    /// Add a triple inside a named graph:
    /// `<g> { <b> <c> <d> }` → `<g> { <b> <c> <d> . <e> <f> <h> }`
    /// The parser must handle the depth shift for the new tokens while keeping
    /// both triples in the named graph.
    #[test]
    fn test_incremental_add_triple_inside_named_graph() {
        let doc = parse_incr(
            "<g> { <b> <c> <d> }",
            "<g> { <b> <c> <d> . <e> <f> <h> }",
        );

        assert_eq!(doc.triples.len(), 2, "should produce two triples");

        for t in doc.triples.iter() {
            match t.value().graph.as_ref().map(|g| g.value()) {
                Some(Term::NamedNode(NamedNode::Full(iri, _))) => assert_eq!(iri, "g"),
                other => panic!("expected <g> as graph for both triples, got {:?}", other),
            }
        }
        match doc.triples[0].value().subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "b"),
            other => panic!("expected <b> as first subject, got {:?}", other),
        }
        match doc.triples[1].value().subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "e"),
            other => panic!("expected <e> as second subject, got {:?}", other),
        }
    }

    /// Remove a triple from a named graph:
    /// `<g> { <b> <c> <d> . <e> <f> <h> }` → `<g> { <b> <c> <d> }`
    /// After removal the parser should produce one triple still in graph <g>.
    #[test]
    fn test_incremental_remove_triple_inside_named_graph() {
        let doc = parse_incr(
            "<g> { <b> <c> <d> . <e> <f> <h> }",
            "<g> { <b> <c> <d> }",
        );

        assert_eq!(doc.triples.len(), 1, "should produce exactly one triple");

        let t = doc.triples[0].value();
        match t.graph.as_ref().map(|g| g.value()) {
            Some(Term::NamedNode(NamedNode::Full(iri, _))) => assert_eq!(iri, "g"),
            other => panic!("expected <g> as graph, got {:?}", other),
        }
        match t.subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "b"),
            other => panic!("expected <b> as subject, got {:?}", other),
        }
    }
}
