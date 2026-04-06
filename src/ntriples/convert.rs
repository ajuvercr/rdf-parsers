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
    let triples: Vec<Spanned<Triple>> = children(root, SyntaxKind::Triple)
        .map(|t| {
            let range = text_range(&t);
            Spanned(convert_triple(&t), range)
        })
        .collect();

    Turtle::new(None, Vec::new(), triples)
}

fn convert_triple(node: &Node) -> Triple {
    let subject = child(node, SyntaxKind::Subject)
        .map(|s| {
            let range = text_range(&s);
            Spanned(convert_subject(&s), range)
        })
        .unwrap_or_else(|| Spanned(Term::Invalid, text_range(node)));

    let predicate = child(node, SyntaxKind::Predicate)
        .map(|p| {
            let range = text_range(&p);
            Spanned(convert_predicate(&p), range)
        })
        .unwrap_or_else(|| Spanned(Term::Invalid, text_range(node)));

    let object = child(node, SyntaxKind::Object)
        .map(|o| {
            let range = text_range(&o);
            Spanned(convert_object(&o), range)
        })
        .unwrap_or_else(|| Spanned(Term::Invalid, text_range(node)));

    let po_range = predicate.span().start..object.span().end;
    let po = Spanned(
        PO {
            predicate,
            object: vec![object],
        },
        po_range,
    );

    Triple {
        subject,
        po: vec![po],
        graph: None,
    }
}

fn convert_subject(node: &Node) -> Term {
    if let Some(iriref) = child(node, SyntaxKind::Iriref) {
        Term::NamedNode(iri_from_iriref_node(&iriref))
    } else if let Some(bnode) = child(node, SyntaxKind::BlankNodeLabel) {
        Term::BlankNode(blank_node_from_label(&bnode))
    } else {
        Term::Invalid
    }
}

fn convert_predicate(node: &Node) -> Term {
    if let Some(iriref) = child(node, SyntaxKind::Iriref) {
        Term::NamedNode(iri_from_iriref_node(&iriref))
    } else {
        Term::Invalid
    }
}

fn convert_object(node: &Node) -> Term {
    if let Some(iriref) = child(node, SyntaxKind::Iriref) {
        Term::NamedNode(iri_from_iriref_node(&iriref))
    } else if let Some(bnode) = child(node, SyntaxKind::BlankNodeLabel) {
        Term::BlankNode(blank_node_from_label(&bnode))
    } else if let Some(lit) = child(node, SyntaxKind::Literal) {
        Term::Literal(convert_literal(&lit))
    } else {
        Term::Invalid
    }
}

fn iri_from_iriref_node(node: &Node) -> NamedNode {
    let text = terminal_text(node);
    let offset: usize = node.text_range().start().into();
    let inner = text.trim_start_matches('<').trim_end_matches('>');
    NamedNode::Full(inner.to_string(), offset)
}

fn blank_node_from_label(node: &Node) -> BlankNode {
    let text = terminal_text(node);
    let offset: usize = node.text_range().start().into();
    let name = text.strip_prefix("_:").unwrap_or(&text);
    BlankNode::Named(name.to_string(), offset)
}

fn convert_literal(node: &Node) -> Literal {
    let offset: usize = node.text_range().start().into();

    let (value, len) = if let Some(str_node) = child(node, SyntaxKind::StringLiteralQuote) {
        let text = terminal_text(&str_node);
        let inner = text
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(&text);
        (inner.to_string(), 1)
    } else {
        (String::new(), 0)
    };

    let lang = child(node, SyntaxKind::Langtag).map(|n| {
        let text = terminal_text(&n);
        text.strip_prefix('@').unwrap_or(&text).to_string()
    });

    let ty = child(node, SyntaxKind::Iriref).map(|n| iri_from_iriref_node(&n));

    Literal::RDF(RDFLiteral {
        value,
        quote_style: StringStyle::Double,
        lang,
        ty,
        idx: offset,
        len,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse as crate_parse, ntriples::parser as lang};

    fn parse(input: &str) -> Turtle {
        let (result, _) = crate_parse(lang::Rule::new(lang::SyntaxKind::NtriplesDoc), input);
        let root = result.syntax::<lang::Lang>();
        convert(&root)
    }

    fn nn_eq(a: &NamedNode, b: &NamedNode) -> bool {
        match (a, b) {
            (NamedNode::Full(s1, _), NamedNode::Full(s2, _)) => s1 == s2,
            _ => false,
        }
    }

    fn term_nn(t: &Term) -> &NamedNode {
        match t {
            Term::NamedNode(nn) => nn,
            other => panic!("expected NamedNode, got {:?}", other),
        }
    }

    fn term_bn(t: &Term) -> &BlankNode {
        match t {
            Term::BlankNode(bn) => bn,
            other => panic!("expected BlankNode, got {:?}", other),
        }
    }

    fn term_lit(t: &Term) -> &Literal {
        match t {
            Term::Literal(l) => l,
            other => panic!("expected Literal, got {:?}", other),
        }
    }

    #[test]
    fn simple_triple_with_iris() {
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

    #[test]
    fn blank_node_as_subject() {
        let doc = parse(
            "_:b1 <http://example.org/name> <http://example.org/value> .",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        let bn = term_bn(t.subject.value());
        assert!(matches!(bn, BlankNode::Named(name, _) if name == "b1"));
    }

    #[test]
    fn blank_node_as_object() {
        let doc = parse(
            "<http://example.org/alice> <http://example.org/knows> _:b2 .",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        let bn = term_bn(t.po[0].object[0].value());
        assert!(matches!(bn, BlankNode::Named(name, _) if name == "b2"));
    }

    #[test]
    fn string_literal_object() {
        let doc = parse(
            "<http://example.org/alice> <http://example.org/name> \"Alice\" .",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        let lit = term_lit(t.po[0].object[0].value());
        match lit {
            Literal::RDF(rdf) => {
                assert_eq!(rdf.value, "Alice");
                assert_eq!(rdf.quote_style, StringStyle::Double);
                assert!(rdf.lang.is_none());
                assert!(rdf.ty.is_none());
            }
            other => panic!("expected RDF literal, got {:?}", other),
        }
    }

    #[test]
    fn literal_with_language_tag() {
        let doc = parse(
            "<http://example.org/alice> <http://example.org/name> \"Alice\"@en .",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        let lit = term_lit(t.po[0].object[0].value());
        match lit {
            Literal::RDF(rdf) => {
                assert_eq!(rdf.value, "Alice");
                assert_eq!(rdf.lang.as_deref(), Some("en"));
                assert!(rdf.ty.is_none());
            }
            other => panic!("expected RDF literal, got {:?}", other),
        }
    }

    #[test]
    fn literal_with_datatype() {
        let doc = parse(
            "<http://example.org/alice> <http://example.org/age> \"30\"^^<http://www.w3.org/2001/XMLSchema#integer> .",
        );
        assert_eq!(doc.triples.len(), 1);
        let t = doc.triples[0].value();
        let lit = term_lit(t.po[0].object[0].value());
        match lit {
            Literal::RDF(rdf) => {
                assert_eq!(rdf.value, "30");
                assert!(rdf.lang.is_none());
                let ty = rdf.ty.as_ref().expect("datatype should be present");
                assert!(nn_eq(
                    ty,
                    &NamedNode::Full(
                        "http://www.w3.org/2001/XMLSchema#integer".to_string(),
                        0
                    )
                ));
            }
            other => panic!("expected RDF literal, got {:?}", other),
        }
    }

    #[test]
    fn multiple_triples() {
        let doc = parse(concat!(
            "<http://example.org/alice> <http://example.org/name> \"Alice\" .\n",
            "<http://example.org/bob> <http://example.org/name> \"Bob\" .\n",
        ));
        assert_eq!(doc.triples.len(), 2);

        let t0 = doc.triples[0].value();
        assert!(nn_eq(
            term_nn(t0.subject.value()),
            &NamedNode::Full("http://example.org/alice".to_string(), 0)
        ));

        let t1 = doc.triples[1].value();
        assert!(nn_eq(
            term_nn(t1.subject.value()),
            &NamedNode::Full("http://example.org/bob".to_string(), 0)
        ));
    }

    // ── fault-tolerant parsing ────────────────────────────────────────────────

    fn parse_raw(input: &str) -> crate::Parse {
        let (result, _) = crate_parse(lang::Rule::new(lang::SyntaxKind::NtriplesDoc), input);
        result
    }

    #[test]
    fn test_valid_input_has_no_errors() {
        let p = parse_raw(
            "<http://example.org/s> <http://example.org/p> <http://example.org/o> .",
        );
        assert_eq!(p.errors.len(), 0, "valid input should produce no errors");
    }

    #[test]
    fn test_missing_trailing_dot_reports_error() {
        let p = parse_raw(
            "<http://example.org/s> <http://example.org/p> <http://example.org/o>",
        );
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
    fn no_prefixes_and_no_base() {
        let doc = parse(
            "<http://example.org/s> <http://example.org/p> <http://example.org/o> .",
        );
        assert!(doc.base.is_none());
        assert!(doc.prefixes.is_empty());
    }

    // ── incremental re-parse tests ────────────────────────────────────────────
    //
    // These tests verify that the A* search finds the correct parse within its
    // iteration budget after a small edit.  They catch grammar weight errors:
    // if a token's error_value is set too high the heuristic becomes too
    // optimistic and the search exhausts its budget before finding the solution.

    use crate::{IncrementalBias, PrevParseInfo, TokenTrait, parse_incremental};

    fn prev_info_incr(text: &str) -> PrevParseInfo {
        let (_, tokens) = crate_parse(lang::Rule::new(lang::SyntaxKind::NtriplesDoc), text);
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
        let prev = prev_info_incr(before);
        let (result, _) = parse_incremental(
            lang::Rule::new(lang::SyntaxKind::NtriplesDoc),
            after,
            Some(&prev),
            IncrementalBias::default(),
        );
        let root = result.syntax::<lang::Lang>();
        convert(&root)
    }

    /// Remove the object: `<a> <b> <c> .` → `<a> <b> .`
    /// The parser should recover and produce one triple with the original subject.
    #[test]
    fn test_incremental_remove_object() {
        let doc = parse_incr(
            "<http://a> <http://b> <http://c> .",
            "<http://a> <http://b> .",
        );
        assert_eq!(doc.triples.len(), 1, "should produce one triple");
        match doc.triples[0].value().subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => {
                assert_eq!(iri, "http://a")
            }
            other => panic!("expected http://a as subject, got {:?}", other),
        }
    }

    /// Add a second triple: `<a> <b> <c> .` → `<a> <b> <c> .\n<d> <e> <f> .`
    /// The incremental parser should produce two triples with the correct subjects.
    #[test]
    fn test_incremental_add_triple() {
        let doc = parse_incr(
            "<http://a> <http://b> <http://c> .",
            "<http://a> <http://b> <http://c> .\n<http://d> <http://e> <http://f> .",
        );
        assert_eq!(doc.triples.len(), 2, "should produce two triples");
        match doc.triples[0].value().subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "http://a"),
            other => panic!("expected http://a as first subject, got {:?}", other),
        }
        match doc.triples[1].value().subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "http://d"),
            other => panic!("expected http://d as second subject, got {:?}", other),
        }
    }

    /// Change the subject: `<a> <b> <c> .` → `<x> <b> <c> .`
    /// Only one token changes; the incremental bias should preserve the other roles.
    #[test]
    fn test_incremental_change_subject() {
        let doc = parse_incr(
            "<http://a> <http://b> <http://c> .",
            "<http://x> <http://b> <http://c> .",
        );
        assert_eq!(doc.triples.len(), 1, "should produce one triple");
        match doc.triples[0].value().subject.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "http://x"),
            other => panic!("expected http://x as subject, got {:?}", other),
        }
        match doc.triples[0].value().po[0].predicate.value() {
            Term::NamedNode(NamedNode::Full(iri, _)) => assert_eq!(iri, "http://b"),
            other => panic!("expected http://b as predicate, got {:?}", other),
        }
    }
}
