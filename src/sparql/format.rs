//! SPARQL formatting.
//!
//! Layout that depends only on node kinds lives in the `=== format ===`
//! section of `grammars/sparql.txt`. The rules here depend on a node's
//! content, which the hints cannot express. They make blank nodes and
//! collections look like Turtle's: one predicate per line with a trailing `;`
//! when there are several, and `], [` kept together. A short top-level `;`
//! list may still share a line.

use rowan::NodeOrToken;

use super::parser::{Lang, SyntaxKind, SyntaxNode};
use crate::format::{Doc, FormatHook, Formatter};

type SyntaxElement = rowan::SyntaxElement<Lang>;

/// Format a SPARQL query or update.
pub fn format(node: &SyntaxNode, width: usize) -> String {
    super::parser::format::format_with(node, width, &SparqlHook)
}

/// Build the formatting document for a SPARQL query or update.
pub fn to_doc(node: &SyntaxNode) -> Doc {
    super::parser::format::to_doc_with(node, &SparqlHook)
}

pub struct SparqlHook;

impl FormatHook<Lang> for SparqlHook {
    fn keep(&self, element: &SyntaxElement) -> bool {
        keep_element(element)
    }

    fn node(&self, node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
        match node.kind() {
            SyntaxKind::PropertyListNotEmpty | SyntaxKind::PropertyListPathNotEmpty => {
                property_list(node, fmt)
            }
            SyntaxKind::ObjectList | SyntaxKind::ObjectListPath => object_list(node, fmt),
            _ => None,
        }
    }
}

fn is_verb(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::Verb | SyntaxKind::VerbPath | SyntaxKind::VerbSimple
    )
}

/// A `;` in a property list that is not followed by another predicate is
/// redundant: `?s ?p ?o;.` and `;;`. Multi-line blank nodes get their trailing
/// `;` back in `property_list`. (The `;` between updates is kept.)
fn keep_element(element: &SyntaxElement) -> bool {
    let in_property_list = element.parent().is_some_and(|p| {
        matches!(
            p.kind(),
            SyntaxKind::PropertyListNotEmpty | SyntaxKind::PropertyListPathNotEmpty
        )
    });
    if element.kind() == SyntaxKind::Colon && in_property_list {
        // Next to a parse error the `;` may belong elsewhere (between
        // updates), so it stays.
        return next_significant(element)
            .is_some_and(|e| is_verb(e.kind()) || e.kind() == SyntaxKind::Error);
    }
    true
}

fn next_significant(element: &SyntaxElement) -> Option<SyntaxElement> {
    let mut next = element.next_sibling_or_token();
    while let Some(e) = next {
        if !matches!(e.kind(), SyntaxKind::WhiteSpace | SyntaxKind::Comment) {
            return Some(e);
        }
        next = e.next_sibling_or_token();
    }
    None
}

/// Blank nodes with several predicates put each on its own line and end every
/// line with `;`, like Turtle.
fn property_list(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
    let in_bnode = node.parent().is_some_and(|p| {
        matches!(
            p.kind(),
            SyntaxKind::BlankNodePropertyList | SyntaxKind::BlankNodePropertyListPath
        )
    });
    let verbs = node.children().filter(|c| is_verb(c.kind())).count();
    (in_bnode && verbs > 1)
        .then(|| Doc::concat(vec![hard_breaks(fmt.default(node)), Doc::text(";")]))
}

/// `doc` with its own soft line breaks made hard. Breaks inside nested groups
/// (object lists, nested blank nodes) still decide for themselves.
fn hard_breaks(doc: Doc) -> Doc {
    match doc {
        Doc::Line => Doc::HardLine,
        Doc::Concat(ds) => Doc::Concat(ds.into_iter().map(hard_breaks).collect()),
        Doc::Nest(n, d) => Doc::Nest(n, Box::new(hard_breaks(*d))),
        other => other,
    }
}

/// Whether an object contains a blank node or collection.
fn is_nested(object: &SyntaxNode) -> bool {
    object.descendants().any(|d| {
        matches!(
            d.kind(),
            SyntaxKind::BlankNodePropertyList
                | SyntaxKind::BlankNodePropertyListPath
                | SyntaxKind::Collection
                | SyntaxKind::CollectionPath
        )
    })
}

/// Objects that lay themselves out over several lines stay together on the
/// predicate's line: `], [ ], "x"`. The blocks close at the indentation of
/// the line they open on, so this undoes the nest the grammar gives object
/// lists.
fn object_list(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
    if !node.children().any(|c| is_nested(&c)) {
        return None;
    }
    let mut docs = Vec::new();
    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Token(t) if t.kind() == SyntaxKind::Comment => {
                docs.push(Doc::text(" "));
                docs.push(Doc::comment(t.text().to_string()));
            }
            NodeOrToken::Token(_) => {}
            NodeOrToken::Node(n) if n.kind() == SyntaxKind::Comma => docs.push(Doc::text(", ")),
            NodeOrToken::Node(n)
                if matches!(n.kind(), SyntaxKind::Object | SyntaxKind::ObjectPath) =>
            {
                docs.push(fmt.child(&n))
            }
            NodeOrToken::Node(n) => docs.push(Doc::text(n.text().to_string())),
        }
    }
    // On the subject's line the property list's nest is undone too.
    let undo = if is_hanging(node) { -4 } else { -2 };
    Some(Doc::nest(undo, Doc::concat(docs)))
}

/// Whether this is the object list of the first predicate, which shares the
/// subject's line.
fn is_hanging(object_list: &SyntaxNode) -> bool {
    object_list.parent().is_some_and(|list| {
        list.parent().is_some_and(|p| {
            matches!(
                p.kind(),
                SyntaxKind::TriplesSameSubject | SyntaxKind::TriplesSameSubjectPath
            )
        }) && list
            .children()
            .find(|c| {
                matches!(
                    c.kind(),
                    SyntaxKind::ObjectList | SyntaxKind::ObjectListPath
                )
            })
            .as_ref()
            == Some(object_list)
    })
}
