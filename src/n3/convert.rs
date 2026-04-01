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

fn terminal_text(node: &Node) -> String {
    node.text().to_string()
}

pub fn convert(root: &Node) -> Turtle {
    let mut base = None;
    let mut prefixes = Vec::new();
    let mut triples = Vec::new();

    // Walk the entire tree to collect triples at all nesting levels (including
    // inside formulas), and collect directives from top-level statements.
    for child_node in root.children() {
        let kind = child_node.kind();
        match kind {
            SyntaxKind::N3Statement => {
                if let Some(dir) = child(&child_node, SyntaxKind::N3Directive) {
                    if let Some(b) = child(&dir, SyntaxKind::Base) {
                        base = Some(Spanned(convert_base(&b), text_range(&b)));
                    } else if let Some(p) = child(&dir, SyntaxKind::PrefixId) {
                        prefixes.push(Spanned(convert_prefix(&p), text_range(&p)));
                    }
                } else if let Some(t) = child(&child_node, SyntaxKind::Triples) {
                    // Collect triples nested inside formulas first
                    collect_nested_triples(&t, &mut triples);
                    // Convert the top-level triple itself, but skip if subject
                    // is invalid (e.g. N3 formula — inner triples already collected)
                    let range = text_range(&child_node);
                    let triple = convert_triples(&t);
                    if !matches!(triple.subject.value(), Term::Invalid) {
                        triples.push(Spanned(triple, range));
                    }
                }
            }
            SyntaxKind::SparqlDirective => {
                if let Some(b) = child(&child_node, SyntaxKind::SparqlBase) {
                    base = Some(Spanned(convert_base(&b), text_range(&b)));
                } else if let Some(p) = child(&child_node, SyntaxKind::SparqlPrefix) {
                    prefixes.push(Spanned(convert_prefix(&p), text_range(&p)));
                }
            }
            _ => {}
        }
    }

    Turtle::new(base, prefixes, triples)
}

/// Recursively find all Formula descendants and collect Triples within them.
fn collect_nested_triples(node: &Node, triples: &mut Vec<Spanned<Triple>>) {
    for c in node.children() {
        let kind = c.kind();
        if kind == SyntaxKind::Formula || kind == SyntaxKind::FormulaContent {
            collect_all_triples(&c, triples);
        } else {
            collect_nested_triples(&c, triples);
        }
    }
}

/// Recursively collect ALL Triples nodes from a subtree.
fn collect_all_triples(node: &Node, triples: &mut Vec<Spanned<Triple>>) {
    for c in node.children() {
        if c.kind() == SyntaxKind::Triples {
            let range = text_range(&c);
            triples.push(Spanned(convert_triples(&c), range));
        }
        // Always recurse to find deeper triples (e.g., formulas within formulas)
        collect_all_triples(&c, triples);
    }
}

// ── directive conversion ─────────────────────────────────────────────────────

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

// ── triples conversion ───────────────────────────────────────────────────────

fn convert_triples(node: &Node) -> Triple {
    // triples ::= subject predicateObjectList?
    let subject = if let Some(subj) = child(node, SyntaxKind::Subject) {
        let range = text_range(&subj);
        Spanned(convert_subject(&subj), range)
    } else {
        Spanned(Term::Invalid, text_range(node))
    };

    let po = child(node, SyntaxKind::PredicateObjectList)
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
    // verb ::= predicate | "a" | ("has" expression) | ("is" expression "of") | "=" | "<=" | "=>"
    if let Some(pred) = child(node, SyntaxKind::Predicate) {
        convert_predicate(&pred)
    } else if let Some(alit) = child(node, SyntaxKind::Alit) {
        Term::NamedNode(NamedNode::A(alit.text_range().start().into()))
    } else if child(node, SyntaxKind::HasLit).is_some() {
        // "has" expression — use the expression as the predicate term
        if let Some(expr) = child(node, SyntaxKind::Expression) {
            convert_expression(&expr)
        } else {
            Term::Invalid
        }
    } else if child(node, SyntaxKind::IsLit).is_some() {
        // "is" expression "of" — use the expression as the predicate term
        if let Some(expr) = child(node, SyntaxKind::Expression) {
            convert_expression(&expr)
        } else {
            Term::Invalid
        }
    } else if child(node, SyntaxKind::Eq).is_some() {
        Term::NamedNode(NamedNode::Full(
            "http://www.w3.org/2002/07/owl#sameAs".to_string(),
            node.text_range().start().into(),
        ))
    } else if child(node, SyntaxKind::ImplyLeft).is_some() {
        Term::NamedNode(NamedNode::Full(
            "http://www.w3.org/2000/10/swap/log#impliedBy".to_string(),
            node.text_range().start().into(),
        ))
    } else if child(node, SyntaxKind::ImplyRight).is_some() {
        Term::NamedNode(NamedNode::Full(
            "http://www.w3.org/2000/10/swap/log#implies".to_string(),
            node.text_range().start().into(),
        ))
    } else {
        Term::Invalid
    }
}

fn convert_predicate(node: &Node) -> Term {
    // predicate ::= expression | ("<-" expression)
    // In both cases, extract the expression as the term.
    if let Some(expr) = child(node, SyntaxKind::Expression) {
        convert_expression(&expr)
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
    // subject ::= expression
    if let Some(expr) = child(node, SyntaxKind::Expression) {
        convert_expression(&expr)
    } else {
        Term::Invalid
    }
}

fn convert_object(node: &Node) -> Term {
    // object ::= expression
    if let Some(expr) = child(node, SyntaxKind::Expression) {
        convert_expression(&expr)
    } else {
        Term::Invalid
    }
}

// ── expression / path / pathItem chain ───────────────────────────────────────

fn convert_expression(node: &Node) -> Term {
    child(node, SyntaxKind::Path)
        .map(|p| convert_path(&p))
        .unwrap_or(Term::Invalid)
}

fn convert_path(node: &Node) -> Term {
    // path ::= pathItem (("!" path) | ("^" path))?
    // Extract just the PathItem term.
    child(node, SyntaxKind::PathItem)
        .map(|pi| convert_path_item(&pi))
        .unwrap_or(Term::Invalid)
}

fn convert_path_item(node: &Node) -> Term {
    if let Some(iri) = child(node, SyntaxKind::Iri) {
        Term::NamedNode(convert_iri(&iri))
    } else if let Some(bn) = child(node, SyntaxKind::BlankNode) {
        Term::BlankNode(convert_blank_node(&bn))
    } else if let Some(qv) = child(node, SyntaxKind::QuickVar) {
        convert_quick_var(&qv)
    } else if let Some(coll) = child(node, SyntaxKind::Collection) {
        Term::Collection(convert_collection(&coll))
    } else if let Some(bpl) = child(node, SyntaxKind::BlankNodePropertyList) {
        convert_blank_node_property_list(&bpl)
    } else if let Some(ipl) = child(node, SyntaxKind::IriPropertyList) {
        convert_iri_property_list(&ipl)
    } else if let Some(lit) = child(node, SyntaxKind::Literal) {
        Term::Literal(convert_literal(&lit))
    } else if child(node, SyntaxKind::Formula).is_some() {
        // Formulas as terms are N3-specific; triples inside are collected separately
        Term::Invalid
    } else {
        Term::Invalid
    }
}

// ── term converters ──────────────────────────────────────────────────────────

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

fn convert_iri_property_list(node: &Node) -> Term {
    // iriPropertyList ::= IPLSTART iri predicateObjectList "]"
    // Use the Iri child for the term representation.
    if let Some(iri) = child(node, SyntaxKind::Iri) {
        Term::NamedNode(convert_iri(&iri))
    } else {
        Term::Invalid
    }
}

fn convert_quick_var(node: &Node) -> Term {
    // quickVar ::= QUICK_VAR_NAME
    if let Some(qv) = child(node, SyntaxKind::QuickVarName) {
        let text = terminal_text(&qv);
        let offset: usize = qv.text_range().start().into();
        let name = text.strip_prefix('?').unwrap_or(&text);
        Term::Variable(Variable(name.to_string(), offset))
    } else {
        Term::Invalid
    }
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
    if let Some(rdf) = child(node, SyntaxKind::RdfLiteral) {
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
    // rdfLiteral ::= STRING (LANGTAG | ("^^" iri))?
    // In N3, String is a single terminal (not wrapped in MyString).
    let offset: usize = node.text_range().start().into();

    let (value, quote_style, len) = if let Some(str_node) = child(node, SyntaxKind::String) {
        let text = terminal_text(&str_node);
        let (inner, style) = strip_string_delimiters(&text);
        (inner.to_string(), style, 1)
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
    use crate::{parse as crate_parse, n3::parser as lang};

    fn parse(input: &str) -> Turtle {
        let (result, _) = crate_parse(lang::Rule::new(lang::SyntaxKind::N3Doc), input);
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

    fn term_var(t: &Term) -> &Variable {
        match t {
            Term::Variable(v) => v,
            other => panic!("expected Variable, got {:?}", other),
        }
    }

    // ── prefix directives ────────────────────────────────────────────────────

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
        let doc = parse("PREFIX ex: <http://example.org/>\nex:alice ex:knows ex:bob .");
        assert_eq!(doc.prefixes.len(), 1);
        assert_eq!(doc.prefixes[0].value().prefix.value(), "ex");
    }

    // ── base directives ──────────────────────────────────────────────────────

    #[test]
    fn test_base_directive() {
        let doc = parse("@base <http://example.org/> .");
        let base = doc.base.as_ref().expect("base should be present");
        assert!(nn_eq(
            base.value().1.value(),
            &NamedNode::Full("http://example.org/".to_string(), 0)
        ));
    }

    #[test]
    fn test_sparql_base_directive() {
        let doc = parse("BASE <http://example.org/>\n<alice> <knows> <bob> .");
        let base = doc.base.as_ref().expect("base should be present");
        assert!(nn_eq(
            base.value().1.value(),
            &NamedNode::Full("http://example.org/".to_string(), 0)
        ));
    }

    // ── simple triple ────────────────────────────────────────────────────────

    #[test]
    fn test_simple_triple() {
        let doc = parse("@prefix ex: <http://example.org/> .\nex:alice ex:knows ex:bob .");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(term_nn(t.subject.value()), &prefixed("ex", "alice")));
        assert_eq!(t.po.len(), 1);
        assert!(nn_eq(
            term_nn(t.po[0].predicate.value()),
            &prefixed("ex", "knows")
        ));
        assert!(nn_eq(
            term_nn(t.po[0].object[0].value()),
            &prefixed("ex", "bob")
        ));
    }

    // ── full IRI triple ──────────────────────────────────────────────────────

    #[test]
    fn test_full_iri_triple() {
        let doc = parse("<http://example.org/alice> <http://example.org/knows> <http://example.org/bob> .");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(
            term_nn(t.subject.value()),
            &NamedNode::Full("http://example.org/alice".to_string(), 0)
        ));
        assert!(nn_eq(
            term_nn(t.po[0].predicate.value()),
            &NamedNode::Full("http://example.org/knows".to_string(), 0)
        ));
        assert!(nn_eq(
            term_nn(t.po[0].object[0].value()),
            &NamedNode::Full("http://example.org/bob".to_string(), 0)
        ));
    }

    // ── `a` shorthand ────────────────────────────────────────────────────────

    #[test]
    fn test_a_shorthand() {
        let doc = parse(
            "@prefix foaf: <http://xmlns.com/foaf/0.1/> .\n<http://example.org/alice> a foaf:Person .",
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

    // ── multiple PO pairs with `;` ───────────────────────────────────────────

    #[test]
    fn test_multiple_po_pairs() {
        let doc =
            parse("@prefix ex: <http://example.org/> .\nex:alice ex:name \"Alice\" ; ex:age 30 .");
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

    // ── multiple objects with `,` ────────────────────────────────────────────

    #[test]
    fn test_multiple_objects() {
        let doc =
            parse("@prefix ex: <http://example.org/> .\nex:alice ex:knows ex:bob , ex:carol .");
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

    // ── string literal ───────────────────────────────────────────────────────

    #[test]
    fn test_string_literal() {
        let doc = parse("@prefix ex: <http://example.org/> .\nex:alice ex:name \"Alice\" .");
        let t = doc.triples[0].value();
        let lit = term_lit(t.po[0].object[0].value());
        assert_eq!(lit.plain_string(), "Alice");
    }

    // ── numeric literal ──────────────────────────────────────────────────────

    #[test]
    fn test_numeric_literal() {
        let doc = parse("@prefix ex: <http://example.org/> .\nex:alice ex:age 30 .");
        let t = doc.triples[0].value();
        match term_lit(t.po[0].object[0].value()) {
            Literal::Numeric(n) => assert_eq!(n, "30"),
            other => panic!("expected Numeric literal, got {:?}", other),
        }
    }

    // ── boolean literal ──────────────────────────────────────────────────────

    #[test]
    fn test_boolean_literal() {
        let doc = parse("@prefix ex: <http://example.org/> .\nex:alice ex:active true .");
        let t = doc.triples[0].value();
        assert_eq!(
            *term_lit(t.po[0].object[0].value()),
            Literal::Boolean(true)
        );
    }

    // ── named blank node ─────────────────────────────────────────────────────

    #[test]
    fn test_named_blank_node() {
        let doc = parse("@prefix ex: <http://example.org/> .\n_:b0 ex:knows ex:alice .");
        let t = doc.triples[0].value();
        match term_bn(t.subject.value()) {
            BlankNode::Named(name, _) => assert_eq!(name, "b0"),
            other => panic!("expected Named blank node, got {:?}", other),
        }
    }

    // ── anonymous blank node property list ───────────────────────────────────

    #[test]
    fn test_anon_blank_node_property_list() {
        let doc = parse(
            "@prefix ex: <http://example.org/> .\nex:alice ex:knows [ ex:name \"Bob\" ] .",
        );
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

    // ── quick variables ──────────────────────────────────────────────────────

    #[test]
    fn test_quick_variables() {
        let doc = parse("?x ?y ?z .");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        let v = term_var(t.subject.value());
        assert_eq!(v.0, "x");
        let pred_v = term_var(t.po[0].predicate.value());
        assert_eq!(pred_v.0, "y");
        let obj_v = term_var(t.po[0].object[0].value());
        assert_eq!(obj_v.0, "z");
    }

    // ── collection ───────────────────────────────────────────────────────────

    #[test]
    fn test_collection() {
        let doc =
            parse("@prefix ex: <http://example.org/> .\nex:alice ex:list ( ex:a ex:b ex:c ) .");
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

    // ── formula with nested triples ──────────────────────────────────────────

    #[test]
    fn test_formula_nested_triples() {
        let doc = parse(
            "@prefix ex: <http://example.org/> .\n{ ex:a ex:b ex:c } ex:implies ex:d .",
        );
        // The top-level triple has a formula as subject (Term::Invalid), so it's
        // skipped. Only the nested triple inside the formula is collected.
        assert_eq!(doc.triples.len(), 1);

        // The nested triple from inside the formula
        let nested = &doc.triples[0];
        assert!(
            matches!(nested.value().subject.value(), Term::NamedNode(nn) if nn_eq(nn, &prefixed("ex", "a")))
        );
    }

    // ── `has` verb ───────────────────────────────────────────────────────────

    #[test]
    fn test_has_verb() {
        let doc = parse(
            "@prefix ex: <http://example.org/> .\nex:alice has ex:friend ex:bob .",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(
            term_nn(t.subject.value()),
            &prefixed("ex", "alice")
        ));
        // The predicate from "has ex:friend" should be ex:friend
        assert!(nn_eq(
            term_nn(t.po[0].predicate.value()),
            &prefixed("ex", "friend")
        ));
        assert!(nn_eq(
            term_nn(t.po[0].object[0].value()),
            &prefixed("ex", "bob")
        ));
    }

    // ── `is ... of` verb ─────────────────────────────────────────────────────

    #[test]
    fn test_is_of_verb() {
        let doc = parse(
            "@prefix ex: <http://example.org/> .\nex:bob is ex:friend of ex:alice .",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        // The predicate from "is ex:friend of" should be ex:friend
        assert!(nn_eq(
            term_nn(t.po[0].predicate.value()),
            &prefixed("ex", "friend")
        ));
    }

    #[test]
    fn test_n3_rule_formulas() {
        let input = "@prefix : <http://example.org/> .\n\n\
            { ?x a :Human . } => { ?x a :Mortal . } .\n\n\
            :Socrates a :Human .\n";
        let doc = parse(input);
        // The formula rule `{ } => { }` is skipped (subject is a formula),
        // but the inner triples are extracted, plus the regular triple.
        assert_eq!(doc.triples.len(), 3);
        // Inner formula triples
        assert_eq!(format!("{}", doc.triples[0].value().subject.value()), "?x");
        assert_eq!(format!("{}", doc.triples[1].value().subject.value()), "?x");
        // Regular triple
        assert!(nn_eq(
            term_nn(doc.triples[2].value().subject.value()),
            &prefixed("", "Socrates")
        ));
    }
}
