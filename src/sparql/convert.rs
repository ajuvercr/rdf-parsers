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

/// Recursively find all descendants of a given kind.
fn collect_descendants(node: &Node, kind: SyntaxKind, out: &mut Vec<Node>) {
    for c in node.children() {
        if c.kind() == kind {
            out.push(c.clone());
        }
        collect_descendants(&c, kind, out);
    }
}

pub fn convert(root: &Node) -> Turtle {
    let mut base = None;
    let mut prefixes = Vec::new();
    let mut triples = Vec::new();

    // Extract base and prefix declarations from any Prologue in the tree.
    let mut prologues = Vec::new();
    collect_descendants(root, SyntaxKind::Prologue, &mut prologues);
    for prologue in &prologues {
        for bd in children(prologue, SyntaxKind::BaseDecl) {
            base = Some(Spanned(convert_base_decl(&bd), text_range(&bd)));
        }
        for pd in children(prologue, SyntaxKind::PrefixDecl) {
            prefixes.push(Spanned(convert_prefix_decl(&pd), text_range(&pd)));
        }
    }

    // Collect all triple patterns from the entire tree.
    collect_triples(root, &mut triples);

    Turtle::new(base, prefixes, triples)
}

fn collect_triples(node: &Node, triples: &mut Vec<Spanned<Triple>>) {
    for child_node in node.children() {
        let kind = child_node.kind();
        match kind {
            SyntaxKind::TriplesSameSubject => {
                let range = text_range(&child_node);
                triples.push(Spanned(convert_triples_same_subject(&child_node), range));
            }
            SyntaxKind::TriplesSameSubjectPath => {
                let range = text_range(&child_node);
                triples.push(Spanned(
                    convert_triples_same_subject_path(&child_node),
                    range,
                ));
            }
            _ => {
                collect_triples(&child_node, triples);
            }
        }
    }
}

// ── prologue ─────────────────────────────────────────────────────────────────

fn convert_base_decl(node: &Node) -> Base {
    // BaseDecl ::= 'BASE' IRIREF
    let range = text_range(node);
    let nn = child(node, SyntaxKind::Iriref)
        .map(|n| iri_from_iriref_node(&n))
        .unwrap_or(NamedNode::Invalid);
    let nn_range = child(node, SyntaxKind::Iriref)
        .map(|n| text_range(&n))
        .unwrap_or(range.clone());
    Base(range, Spanned(nn, nn_range))
}

fn convert_prefix_decl(node: &Node) -> TurtlePrefix {
    // PrefixDecl ::= 'PREFIX' PNAME_NS IRIREF
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
        .unwrap_or_else(|| Spanned(NamedNode::Invalid, range.clone()));

    TurtlePrefix {
        span: range,
        prefix: prefix_text,
        value,
    }
}

// ── TriplesSameSubject (CONSTRUCT, TriplesTemplate) ──────────────────────────

fn convert_triples_same_subject(node: &Node) -> Triple {
    // TriplesSameSubject ::= VarOrTerm PropertyListNotEmpty | TriplesNode PropertyList
    let (subject, po_node) = if let Some(vot) = child(node, SyntaxKind::VarOrTerm) {
        let range = text_range(&vot);
        (
            Spanned(convert_var_or_term(&vot), range),
            child(node, SyntaxKind::PropertyListNotEmpty),
        )
    } else if let Some(tn) = child(node, SyntaxKind::TriplesNode) {
        let range = text_range(&tn);
        (
            Spanned(convert_triples_node(&tn), range),
            child(node, SyntaxKind::PropertyList)
                .and_then(|pl| child(&pl, SyntaxKind::PropertyListNotEmpty)),
        )
    } else {
        (Spanned(Term::Invalid, text_range(node)), None)
    };

    let po = po_node
        .map(|n| convert_property_list_not_empty(&n))
        .unwrap_or_default();

    Triple {
        subject,
        po,
        graph: None,
    }
}

// ── TriplesSameSubjectPath (WHERE clause TriplesBlock) ───────────────────────

fn convert_triples_same_subject_path(node: &Node) -> Triple {
    // TriplesSameSubjectPath ::= Subject1 PropertyListPathNotEmpty | Subject2 PropertyListPath
    // Subject1 ::= VarOrTerm
    // Subject2 ::= TriplesNodePath
    let (subject, po_node) = if let Some(s1) = child(node, SyntaxKind::Subject1) {
        let range = text_range(&s1);
        let term = child(&s1, SyntaxKind::VarOrTerm)
            .map(|v| convert_var_or_term(&v))
            .unwrap_or(Term::Invalid);
        (
            Spanned(term, range),
            child(node, SyntaxKind::PropertyListPathNotEmpty),
        )
    } else if let Some(s2) = child(node, SyntaxKind::Subject2) {
        let range = text_range(&s2);
        let term = child(&s2, SyntaxKind::TriplesNodePath)
            .map(|t| convert_triples_node_path(&t))
            .unwrap_or(Term::Invalid);
        (
            Spanned(term, range),
            child(node, SyntaxKind::PropertyListPath)
                .and_then(|pl| child(&pl, SyntaxKind::PropertyListPathNotEmpty)),
        )
    } else {
        (Spanned(Term::Invalid, text_range(node)), None)
    };

    let po = po_node
        .map(|n| convert_property_list_path_not_empty(&n))
        .unwrap_or_default();

    Triple {
        subject,
        po,
        graph: None,
    }
}

// ── property lists ───────────────────────────────────────────────────────────

fn convert_property_list_not_empty(node: &Node) -> Vec<Spanned<PO>> {
    // PropertyListNotEmpty ::= Verb ObjectList (';' (Verb ObjectList)?)*
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

fn convert_property_list_path_not_empty(node: &Node) -> Vec<Spanned<PO>> {
    // PropertyListPathNotEmpty ::=
    //   (VerbPath | VerbSimple) ObjectListPath
    //   (';' ((VerbPath | VerbSimple) ObjectList)?)*
    //
    // Walk children in order, pairing verbs with their object lists.
    let mut result = Vec::new();
    let mut current_verb: Option<(Term, std::ops::Range<usize>)> = None;

    for child_node in node.children() {
        let kind = child_node.kind();
        match kind {
            SyntaxKind::VerbPath => {
                current_verb = Some((convert_verb_path(&child_node), text_range(&child_node)));
            }
            SyntaxKind::VerbSimple => {
                current_verb = Some((convert_verb_simple(&child_node), text_range(&child_node)));
            }
            SyntaxKind::ObjectListPath => {
                if let Some((pred, pred_range)) = current_verb.take() {
                    let obj_range = text_range(&child_node);
                    let range = pred_range.start..obj_range.end;
                    let objects = convert_object_list_path(&child_node);
                    result.push(Spanned(
                        PO {
                            predicate: Spanned(pred, pred_range),
                            object: objects,
                        },
                        range,
                    ));
                }
            }
            SyntaxKind::ObjectList => {
                if let Some((pred, pred_range)) = current_verb.take() {
                    let obj_range = text_range(&child_node);
                    let range = pred_range.start..obj_range.end;
                    let objects = convert_object_list(&child_node);
                    result.push(Spanned(
                        PO {
                            predicate: Spanned(pred, pred_range),
                            object: objects,
                        },
                        range,
                    ));
                }
            }
            _ => {}
        }
    }

    result
}

// ── verbs ────────────────────────────────────────────────────────────────────

fn convert_verb(node: &Node) -> Term {
    // Verb ::= VarOrIri | 'a'
    if let Some(voi) = child(node, SyntaxKind::VarOrIri) {
        convert_var_or_iri(&voi)
    } else if let Some(alit) = child(node, SyntaxKind::Alit) {
        Term::NamedNode(NamedNode::A(alit.text_range().start().into()))
    } else {
        Term::Invalid
    }
}

fn convert_verb_path(node: &Node) -> Term {
    // VerbPath ::= Path
    // Walk: Path → PathAlternative → PathSequence → PathEltOrInverse → PathElt → PathPrimary
    child(node, SyntaxKind::Path)
        .and_then(|p| child(&p, SyntaxKind::PathAlternative))
        .and_then(|pa| child(&pa, SyntaxKind::PathSequence))
        .and_then(|ps| child(&ps, SyntaxKind::PathEltOrInverse))
        .and_then(|peoi| child(&peoi, SyntaxKind::PathElt))
        .and_then(|pe| child(&pe, SyntaxKind::PathPrimary))
        .map(|pp| convert_path_primary(&pp))
        .unwrap_or(Term::Invalid)
}

fn convert_path_primary(node: &Node) -> Term {
    // PathPrimary ::= iri | 'a' | '!' PathNegatedPropertySet | '(' Path ')'
    if let Some(iri) = child(node, SyntaxKind::Iri) {
        Term::NamedNode(convert_iri(&iri))
    } else if let Some(alit) = child(node, SyntaxKind::Alit) {
        Term::NamedNode(NamedNode::A(alit.text_range().start().into()))
    } else {
        Term::Invalid
    }
}

fn convert_verb_simple(node: &Node) -> Term {
    // VerbSimple ::= Var
    child(node, SyntaxKind::Var)
        .map(|v| convert_var(&v))
        .unwrap_or(Term::Invalid)
}

// ── object lists ─────────────────────────────────────────────────────────────

fn convert_object_list(node: &Node) -> Vec<Spanned<Term>> {
    // ObjectList ::= Object (',' Object)*
    // Object ::= GraphNode
    children(node, SyntaxKind::Object)
        .map(|o| {
            let range = text_range(&o);
            Spanned(convert_object(&o), range)
        })
        .collect()
}

fn convert_object(node: &Node) -> Term {
    // Object ::= GraphNode
    child(node, SyntaxKind::GraphNode)
        .map(|gn| convert_graph_node(&gn))
        .unwrap_or(Term::Invalid)
}

fn convert_object_list_path(node: &Node) -> Vec<Spanned<Term>> {
    // ObjectListPath ::= ObjectPath (',' ObjectPath)*
    children(node, SyntaxKind::ObjectPath)
        .map(|op| {
            let range = text_range(&op);
            Spanned(convert_object_path(&op), range)
        })
        .collect()
}

fn convert_object_path(node: &Node) -> Term {
    // ObjectPath ::= GraphNodePath
    child(node, SyntaxKind::GraphNodePath)
        .map(|gnp| convert_graph_node_path(&gnp))
        .unwrap_or(Term::Invalid)
}

// ── graph nodes ──────────────────────────────────────────────────────────────

fn convert_graph_node(node: &Node) -> Term {
    // GraphNode ::= VarOrTerm | TriplesNode
    if let Some(vot) = child(node, SyntaxKind::VarOrTerm) {
        convert_var_or_term(&vot)
    } else if let Some(tn) = child(node, SyntaxKind::TriplesNode) {
        convert_triples_node(&tn)
    } else {
        Term::Invalid
    }
}

fn convert_graph_node_path(node: &Node) -> Term {
    // GraphNodePath ::= VarOrTerm | TriplesNodePath
    if let Some(vot) = child(node, SyntaxKind::VarOrTerm) {
        convert_var_or_term(&vot)
    } else if let Some(tnp) = child(node, SyntaxKind::TriplesNodePath) {
        convert_triples_node_path(&tnp)
    } else {
        Term::Invalid
    }
}

// ── terms ────────────────────────────────────────────────────────────────────

fn convert_var_or_term(node: &Node) -> Term {
    // VarOrTerm ::= Var | GraphTerm
    if let Some(var) = child(node, SyntaxKind::Var) {
        convert_var(&var)
    } else if let Some(gt) = child(node, SyntaxKind::GraphTerm) {
        convert_graph_term(&gt)
    } else {
        Term::Invalid
    }
}

fn convert_var_or_iri(node: &Node) -> Term {
    // VarOrIri ::= Var | iri
    if let Some(var) = child(node, SyntaxKind::Var) {
        convert_var(&var)
    } else if let Some(iri) = child(node, SyntaxKind::Iri) {
        Term::NamedNode(convert_iri(&iri))
    } else {
        Term::Invalid
    }
}

fn convert_var(node: &Node) -> Term {
    // Var ::= VAR1 | VAR2
    let text = child(node, SyntaxKind::Var1)
        .or_else(|| child(node, SyntaxKind::Var2))
        .map(|n| terminal_text(&n));

    if let Some(text) = text {
        let offset: usize = node.text_range().start().into();
        let name = text
            .strip_prefix('?')
            .or_else(|| text.strip_prefix('$'))
            .unwrap_or(&text);
        Term::Variable(Variable(name.to_string(), offset))
    } else {
        Term::Invalid
    }
}

fn convert_graph_term(node: &Node) -> Term {
    // GraphTerm ::= iri | RDFLiteral | NumericLiteral | BooleanLiteral | BlankNode | NIL
    if let Some(iri) = child(node, SyntaxKind::Iri) {
        Term::NamedNode(convert_iri(&iri))
    } else if let Some(rdf) = child(node, SyntaxKind::Rdfliteral) {
        Term::Literal(Literal::RDF(convert_rdf_literal(&rdf)))
    } else if let Some(num) = child(node, SyntaxKind::NumericLiteral) {
        Term::Literal(convert_numeric_literal(&num))
    } else if let Some(b) = child(node, SyntaxKind::BooleanLiteral) {
        Term::Literal(Literal::Boolean(b.text().to_string().trim() == "true"))
    } else if let Some(bn) = child(node, SyntaxKind::BlankNode) {
        Term::BlankNode(convert_blank_node(&bn))
    } else if child(node, SyntaxKind::Nil).is_some() {
        Term::Collection(Vec::new())
    } else {
        Term::Invalid
    }
}

// ── compound nodes ───────────────────────────────────────────────────────────

fn convert_triples_node(node: &Node) -> Term {
    // TriplesNode ::= Collection | BlankNodePropertyList
    if let Some(coll) = child(node, SyntaxKind::Collection) {
        Term::Collection(convert_collection(&coll))
    } else if let Some(bpl) = child(node, SyntaxKind::BlankNodePropertyList) {
        convert_blank_node_property_list(&bpl)
    } else {
        Term::Invalid
    }
}

fn convert_triples_node_path(node: &Node) -> Term {
    // TriplesNodePath ::= CollectionPath | BlankNodePropertyListPath
    if let Some(coll) = child(node, SyntaxKind::CollectionPath) {
        Term::Collection(convert_collection_path(&coll))
    } else if let Some(bplp) = child(node, SyntaxKind::BlankNodePropertyListPath) {
        convert_blank_node_property_list_path(&bplp)
    } else {
        Term::Invalid
    }
}

fn convert_collection(node: &Node) -> Vec<Spanned<Term>> {
    // Collection ::= '(' GraphNode+ ')'
    children(node, SyntaxKind::GraphNode)
        .map(|gn| {
            let range = text_range(&gn);
            Spanned(convert_graph_node(&gn), range)
        })
        .collect()
}

fn convert_collection_path(node: &Node) -> Vec<Spanned<Term>> {
    // CollectionPath ::= '(' GraphNodePath+ ')'
    children(node, SyntaxKind::GraphNodePath)
        .map(|gnp| {
            let range = text_range(&gnp);
            Spanned(convert_graph_node_path(&gnp), range)
        })
        .collect()
}

fn convert_blank_node_property_list(node: &Node) -> Term {
    // BlankNodePropertyList ::= '[' PropertyListNotEmpty ']'
    let offset: usize = node.text_range().start().into();
    let end: usize = node.text_range().end().into();
    let po = child(node, SyntaxKind::PropertyListNotEmpty)
        .map(|n| convert_property_list_not_empty(&n))
        .unwrap_or_default();
    Term::BlankNode(BlankNode::Unnamed(po, offset, end))
}

fn convert_blank_node_property_list_path(node: &Node) -> Term {
    // BlankNodePropertyListPath ::= '[' PropertyListPathNotEmpty ']'
    let offset: usize = node.text_range().start().into();
    let end: usize = node.text_range().end().into();
    let po = child(node, SyntaxKind::PropertyListPathNotEmpty)
        .map(|n| convert_property_list_path_not_empty(&n))
        .unwrap_or_default();
    Term::BlankNode(BlankNode::Unnamed(po, offset, end))
}

// ── atomic terms ─────────────────────────────────────────────────────────────

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

fn convert_numeric_literal(node: &Node) -> Literal {
    // NumericLiteral ::= NumericLiteralUnsigned | NumericLiteralPositive | NumericLiteralNegative
    Literal::Numeric(node.text().to_string().trim().to_string())
}

fn convert_rdf_literal(node: &Node) -> RDFLiteral {
    // RDFLiteral ::= String (LANGTAG | '^^' iri)?
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
    use crate::{parse as crate_parse, sparql::parser as lang};

    fn parse(input: &str) -> Turtle {
        let (result, _) = crate_parse(lang::Rule::new(lang::SyntaxKind::QueryUnit), input);
        let root = result.syntax::<lang::Lang>();
        convert(&root)
    }

    fn parse_update(input: &str) -> Turtle {
        let (result, _) = crate_parse(lang::Rule::new(lang::SyntaxKind::UpdateUnit), input);
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

    fn term_var(t: &Term) -> &str {
        match t {
            Term::Variable(Variable(name, _)) => name.as_str(),
            other => panic!("expected Variable, got {:?}", other),
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

    // ── prologue ─────────────────────────────────────────────────────────────

    #[test]
    fn test_prefix_decl() {
        let doc = parse("PREFIX ex: <http://example.org/> SELECT * WHERE { ?s ?p ?o }");
        assert_eq!(doc.prefixes.len(), 1);
        assert_eq!(doc.prefixes[0].prefix.value(), "ex");
        assert!(nn_eq(
            doc.prefixes[0].value.value(),
            &NamedNode::Full("http://example.org/".to_string(), 0)
        ));
    }

    #[test]
    fn test_base_decl() {
        let doc = parse("BASE <http://example.org/> SELECT * WHERE { ?s ?p ?o }");
        let base = doc.base.as_ref().expect("base should be present");
        assert!(nn_eq(
            base.value().1.value(),
            &NamedNode::Full("http://example.org/".to_string(), 0)
        ));
    }

    #[test]
    fn test_multiple_prefixes() {
        let doc = parse(
            "PREFIX ex: <http://example.org/> PREFIX foaf: <http://xmlns.com/foaf/0.1/> SELECT * WHERE { ?s ?p ?o }",
        );
        assert_eq!(doc.prefixes.len(), 2);
        assert_eq!(doc.prefixes[0].prefix.value(), "ex");
        assert_eq!(doc.prefixes[1].prefix.value(), "foaf");
    }

    // ── basic triple pattern with variables ──────────────────────────────────

    #[test]
    fn test_select_basic_triple_pattern() {
        let doc = parse("SELECT * WHERE { ?s ?p ?o }");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert_eq!(term_var(t.subject.value()), "s");
        assert_eq!(term_var(t.po[0].predicate.value()), "p");
        assert_eq!(term_var(t.po[0].object[0].value()), "o");
    }

    // ── triple pattern with IRIs ─────────────────────────────────────────────

    #[test]
    fn test_triple_with_iris() {
        let doc =
            parse("PREFIX ex: <http://example.org/> SELECT * WHERE { ex:alice ex:knows ex:bob }");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(term_nn(t.subject.value()), &prefixed("ex", "alice")));
        assert!(nn_eq(
            term_nn(t.po[0].predicate.value()),
            &prefixed("ex", "knows")
        ));
        assert!(nn_eq(
            term_nn(t.po[0].object[0].value()),
            &prefixed("ex", "bob")
        ));
    }

    // ── `a` shorthand ────────────────────────────────────────────────────────

    #[test]
    fn test_a_shorthand() {
        let doc =
            parse("PREFIX foaf: <http://xmlns.com/foaf/0.1/> SELECT * WHERE { ?x a foaf:Person }");
        let t = doc.triples[0].value();
        assert!(matches!(
            t.po[0].predicate.value(),
            Term::NamedNode(NamedNode::A(_))
        ));
    }

    // ── multiple triple patterns ─────────────────────────────────────────────

    #[test]
    fn test_multiple_triple_patterns() {
        let doc = parse("SELECT * WHERE { ?s ?p ?o . ?a ?b ?c }");
        assert_eq!(doc.triples.len(), 2);
        assert_eq!(term_var(doc.triples[0].subject.value()), "s");
        assert_eq!(term_var(doc.triples[1].subject.value()), "a");
    }

    // ── semicolon for multiple predicates ────────────────────────────────────

    #[test]
    fn test_semicolon_multiple_po() {
        let doc = parse(
            "PREFIX ex: <http://example.org/> SELECT * WHERE { ?x ex:name ?name ; ex:age ?age }",
        );
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

    // ── comma for multiple objects ───────────────────────────────────────────

    #[test]
    fn test_comma_multiple_objects() {
        let doc = parse(
            "PREFIX ex: <http://example.org/> SELECT * WHERE { ?x ex:knows ex:alice , ex:bob }",
        );
        let t = doc.triples[0].value();
        assert_eq!(t.po[0].object.len(), 2);
    }

    // ── literals ─────────────────────────────────────────────────────────────

    #[test]
    fn test_string_literal() {
        let doc = parse("PREFIX ex: <http://example.org/> SELECT * WHERE { ?x ex:name \"Alice\" }");
        let t = doc.triples[0].value();
        assert_eq!(term_lit(t.po[0].object[0].value()).plain_string(), "Alice");
    }

    #[test]
    fn test_numeric_literal() {
        let doc = parse("PREFIX ex: <http://example.org/> SELECT * WHERE { ?x ex:age 30 }");
        let t = doc.triples[0].value();
        match term_lit(t.po[0].object[0].value()) {
            Literal::Numeric(n) => assert_eq!(n, "30"),
            other => panic!("expected Numeric, got {:?}", other),
        }
    }

    #[test]
    fn test_boolean_literal() {
        let doc = parse("PREFIX ex: <http://example.org/> SELECT * WHERE { ?x ex:active true }");
        let t = doc.triples[0].value();
        assert_eq!(*term_lit(t.po[0].object[0].value()), Literal::Boolean(true));
    }

    // ── blank nodes ──────────────────────────────────────────────────────────

    #[test]
    fn test_blank_node_property_list() {
        let doc = parse(
            "PREFIX ex: <http://example.org/> SELECT * WHERE { ?x ex:knows [ ex:name \"Bob\" ] }",
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

    // ── CONSTRUCT queries ────────────────────────────────────────────────────

    #[test]
    fn test_construct_template_triples() {
        let doc = parse(
            "PREFIX ex: <http://example.org/> CONSTRUCT { ?s ex:knows ?o } WHERE { ?s ex:knows ?o }",
        );
        // Both the CONSTRUCT template and WHERE clause produce triples
        assert!(doc.triples.len() >= 2);
    }

    // ── OPTIONAL / UNION (triples in nested patterns) ────────────────────────

    #[test]
    fn test_optional_triples_collected() {
        let doc = parse(
            "PREFIX ex: <http://example.org/> SELECT * WHERE { ?x a ex:Person . OPTIONAL { ?x ex:name ?name } }",
        );
        // Main triple + OPTIONAL triple
        assert_eq!(doc.triples.len(), 2);
    }

    #[test]
    fn test_union_triples_collected() {
        let doc = parse(
            "PREFIX ex: <http://example.org/> SELECT * WHERE { { ?x ex:name ?n } UNION { ?x ex:label ?n } }",
        );
        assert_eq!(doc.triples.len(), 2);
    }

    // ── ASK query ────────────────────────────────────────────────────────────

    #[test]
    fn test_ask_query() {
        let doc = parse("PREFIX ex: <http://example.org/> ASK { ex:alice ex:knows ex:bob }");
        assert_eq!(doc.triples.len(), 1);
    }

    // ── mixed variables and IRIs ─────────────────────────────────────────────

    #[test]
    fn test_mixed_variables_and_iris() {
        let doc =
            parse("PREFIX ex: <http://example.org/> SELECT ?name WHERE { ex:alice ex:name ?name }");
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(term_nn(t.subject.value()), &prefixed("ex", "alice")));
        assert_eq!(term_var(t.po[0].object[0].value()), "name");
    }

    // ── UPDATE queries ───────────────────────────────────────────────────────

    #[test]
    fn test_insert_data() {
        let doc = parse_update(
            "PREFIX ex: <http://example.org/> INSERT DATA { ex:alice ex:knows ex:bob }",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        assert!(nn_eq(term_nn(t.subject.value()), &prefixed("ex", "alice")));
    }

    // ── incremental parsing ───────────────────────────────────────────────────

    use crate::{IncrementalBias, PrevParseInfo, TokenTrait, parse_incremental};

    fn prev_info(text: &str) -> PrevParseInfo {
        let (_, tokens) = crate_parse(lang::Rule::new(lang::SyntaxKind::QueryUnit), text);
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
        }
    }

    /// Moving FILTER out of the top-level WHERE and into an OPTIONAL block.
    ///
    /// Before:
    /// ```sparql
    /// SELECT * WHERE {
    ///   OPTIONAL { ?person foaf:age ?age . }
    ///   FILTER(?name != "")
    /// }
    /// ```
    ///
    /// After:
    /// ```sparql
    /// SELECT * WHERE {
    ///   OPTIONAL { ?person foaf:age ?age .
    ///     FILTER(?name != "")
    ///   }
    /// }
    /// ```
    ///
    /// The after form is valid SPARQL. The test verifies the incremental parser
    /// does not get stuck preferring the old parse (where FILTER was a sibling
    /// of OPTIONAL at the top level) when the closing `}` of OPTIONAL now
    /// appears after FILTER.
    #[test]
    fn test_suggest_filter_moved_into_optional() {
        let before =
            "SELECT * WHERE {\n  OPTIONAL { ?person foaf:age ?age . }\n  FILTER(?name != \"\")\n}";
        let after = "SELECT * WHERE {\n  OPTIONAL { ?person foaf:age ?age .\n    FILTER(?name != \"\")\n  }\n}";

        let prev = prev_info(before);
        let bias = IncrementalBias::default();
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::QueryUnit),
            after,
            Some(&prev),
            bias,
        );

        let errors: Vec<_> = parse.errors.iter().collect();
        assert_eq!(
            errors.len(),
            0,
            "valid SPARQL should parse without errors; got: {:?}",
            errors
        );

        let root = parse.syntax::<lang::Lang>();
        let doc = convert(&root);
        assert_eq!(
            doc.triples.len(),
            1,
            "should produce one triple (?person foaf:age ?age)"
        );
    }
}

#[cfg(test)]
mod demo_sim_tests {
    use super::*;
    use crate::sparql::parser as lang;
    use crate::{IncrementalBias, PrevParseInfo, parse_incremental};

    // Simulate demo flow: use parse_incremental's returned prev (not crate_parse)
    #[test]
    fn test_suggest_filter_moved_demo_flow() {
        let before =
            "SELECT * WHERE {\n  OPTIONAL { ?person foaf:age ?age . }\n  FILTER(?name != \"\")\n}";
        let after = "SELECT * WHERE {\n  OPTIONAL { ?person foaf:age ?age .\n    FILTER(?name != \"\")\n  }\n}";

        // First parse: no prev (as in demo on initial load)
        let (_, prev) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::QueryUnit),
            before,
            None,
            IncrementalBias::default(),
        );

        // Second parse: use the prev from the first parse (as in demo on edit)
        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::QueryUnit),
            after,
            Some(&prev),
            IncrementalBias::default(),
        );

        let errors: Vec<_> = parse.errors.iter().collect();
        assert_eq!(
            errors.len(),
            0,
            "demo flow: valid SPARQL should parse without errors; got: {:?}",
            errors
        );
    }
}

#[cfg(test)]
mod real_demo_tests {
    use super::*;
    use crate::sparql::parser as lang;
    use crate::{IncrementalBias, parse_incremental};

    #[test]
    fn test_suggest_filter_moved_real_demo() {
        let before = "
PREFIX ex: <http://example.org/>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>

SELECT ?person ?name ?age
WHERE {
  ?person a foaf:Person ;
          foaf:name ?name .
  OPTIONAL { ?person foaf:age ?age . }
  FILTER(?name != \"\")
}
ORDER BY ?name";

        let after = "
PREFIX ex: <http://example.org/>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>

SELECT ?person ?name ?age
WHERE {
  ?person a foaf:Person ;
          foaf:name ?name .
  OPTIONAL { ?person foaf:age ?age . 
  FILTER(?name != \"\")}
}
ORDER BY ?name";

        // Simulate demo: first parse with no prev, then incremental
        let (_, prev) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::QueryUnit),
            before,
            None,
            IncrementalBias::default(),
        );

        let (parse, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::QueryUnit),
            after,
            Some(&prev),
            IncrementalBias::default(),
        );

        let errors: Vec<_> = parse.errors.iter().collect();
        assert_eq!(
            errors.len(),
            0,
            "real demo: valid SPARQL should parse without errors; got: {:?}",
            errors
        );
    }
}
