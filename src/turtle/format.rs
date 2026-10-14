//! Turtle formatting.
//!
//! Layout that depends only on node kinds lives in the `=== format ===`
//! section of `grammars/turtle.txt`. The rules here depend on a node's
//! content or its neighbours, which the hints cannot express. Rules shared
//! with TriG live in `crate::rdf_format`.

use rowan::NodeOrToken;

use super::parser::{Lang, SyntaxKind, SyntaxNode};
use crate::format::{Doc, FormatHook, Formatter};

type SyntaxElement = rowan::SyntaxElement<Lang>;

/// Format a Turtle document.
pub fn format(node: &SyntaxNode, width: usize) -> String {
    super::parser::format::format_with(node, width, &TurtleHook)
}

/// Build the formatting document for a Turtle document.
pub fn to_doc(node: &SyntaxNode) -> Doc {
    super::parser::format::to_doc_with(node, &TurtleHook)
}

pub struct TurtleHook;

impl FormatHook<Lang> for TurtleHook {
    fn keep(&self, element: &SyntaxElement) -> bool {
        keep_element(element)
    }

    fn node(&self, node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
        match node.kind() {
            SyntaxKind::Statement => statement(node, fmt),
            SyntaxKind::PredicateObjectList => predicate_object_list(node, fmt),
            SyntaxKind::ObjectList => object_list(node, fmt),
            _ => None,
        }
    }
}

turtle_like_format_rules! {
    bnode_lists: [BlankNodePropertyList, BlankNodePropertyList2],
    blocks: [BlankNodePropertyList2, Collection],
}

fn is_directive(statement: &SyntaxNode) -> bool {
    statement
        .children()
        .any(|c| c.kind() == SyntaxKind::Directive)
}

/// Blank lines from the source are kept by the generated formatter already.
fn statement(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
    let prev = prev_statement(node, |k| k == SyntaxKind::Statement, |_| false)?;
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
    .then(|| Doc::concat(vec![Doc::HardLine, doc]))
}
