//! N3 formatting.
//!
//! Layout that depends only on node kinds lives in the `=== format ===`
//! section of `grammars/n3.txt`. Rules that depend on a node's content are
//! shared with Turtle (`crate::rdf_format`); only statement separation is
//! N3-specific, since top-level statements sit directly under the root with
//! their `.` as a sibling.

use rowan::NodeOrToken;

use super::parser::{Lang, SyntaxKind, SyntaxNode};
use crate::format::{Doc, FormatHook, Formatter};

type SyntaxElement = rowan::SyntaxElement<Lang>;

/// Format an N3 document.
pub fn format(node: &SyntaxNode, width: usize) -> String {
    super::parser::format::format_with(node, width, &N3Hook)
}

/// Build the formatting document for an N3 document.
pub fn to_doc(node: &SyntaxNode) -> Doc {
    super::parser::format::to_doc_with(node, &N3Hook)
}

pub struct N3Hook;

impl FormatHook<Lang> for N3Hook {
    fn keep(&self, element: &SyntaxElement) -> bool {
        keep_element(element)
    }

    fn node(&self, node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
        match node.kind() {
            SyntaxKind::N3Statement | SyntaxKind::SparqlDirective => top_level(node, fmt),
            SyntaxKind::PredicateObjectList => predicate_object_list(node, fmt),
            SyntaxKind::ObjectList => object_list(node, fmt),
            _ => None,
        }
    }
}

turtle_like_format_rules! {
    bnode_lists: [BlankNodePropertyList, IriPropertyList],
    blocks: [BlankNodePropertyList, IriPropertyList, Collection, Formula],
}

fn is_directive(node: &SyntaxNode) -> bool {
    node.kind() == SyntaxKind::SparqlDirective
        || node.children().any(|c| c.kind() == SyntaxKind::N3Directive)
}

/// Separate top-level statements like Turtle does; statements inside
/// formulas are left to the grammar hints.
fn top_level(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
    if node.parent()?.kind() != SyntaxKind::ROOT {
        return None;
    }
    let is_top_level = |k| matches!(k, SyntaxKind::N3Statement | SyntaxKind::SparqlDirective);
    // Skip the statement's own `.`, a sibling at this level.
    let prev = prev_statement(node, is_top_level, |k| k == SyntaxKind::Stop)?;
    if newlines_before(node) > 1 {
        return None;
    }
    let doc = fmt.default(node);
    needs_blank_line(
        &fmt.default(&prev),
        is_directive(&prev),
        &doc,
        is_directive(node),
    )
    .then(|| after_blank_line(doc))
}
