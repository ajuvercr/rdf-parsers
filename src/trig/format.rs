//! TriG formatting.
//!
//! Layout that depends only on node kinds lives in the `=== format ===`
//! section of `grammars/trig.txt`. Rules that depend on a node's content are
//! shared with Turtle (`crate::rdf_format`); only statement separation is
//! TriG-specific, since TriG has no `statement` node and chains the triples
//! inside a graph through nested `triplesBlock`s.

use rowan::NodeOrToken;

use super::parser::{Lang, SyntaxKind, SyntaxNode};
use crate::format::{Doc, FormatHook, Formatter};

type SyntaxElement = rowan::SyntaxElement<Lang>;

/// Format a TriG document.
pub fn format(node: &SyntaxNode, width: usize) -> String {
    super::parser::format::format_with(node, width, &TrigHook)
}

/// Build the formatting document for a TriG document.
pub fn to_doc(node: &SyntaxNode) -> Doc {
    super::parser::format::to_doc_with(node, &TrigHook)
}

pub struct TrigHook;

impl FormatHook<Lang> for TrigHook {
    fn keep(&self, element: &SyntaxElement) -> bool {
        keep_element(element)
    }

    fn node(&self, node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
        match node.kind() {
            SyntaxKind::Directive | SyntaxKind::Block => top_level(node, fmt),
            SyntaxKind::TriplesBlock => triples_block(node, fmt),
            SyntaxKind::PredicateObjectList => predicate_object_list(node, fmt),
            SyntaxKind::ObjectList => object_list(node, fmt),
            _ => None,
        }
    }
}

turtle_like_format_rules!();

/// The directive or block directly before `node`, unless a comment sits in
/// between.
fn prev_top_level(node: &SyntaxNode) -> Option<SyntaxNode> {
    let mut prev = node.prev_sibling_or_token();
    while let Some(e) = prev {
        match e {
            NodeOrToken::Token(ref t) if t.kind() == SyntaxKind::WhiteSpace => {}
            NodeOrToken::Node(n)
                if matches!(n.kind(), SyntaxKind::Directive | SyntaxKind::Block) =>
            {
                return Some(n);
            }
            _ => return None,
        }
        prev = e.prev_sibling_or_token();
    }
    None
}

fn top_level(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
    let prev = prev_top_level(node)?;
    if newlines_before(node) > 1 {
        return None;
    }
    let is_directive = |n: &SyntaxNode| n.kind() == SyntaxKind::Directive;
    let doc = fmt.default(node);
    needs_blank_line(
        &fmt.default(&prev),
        is_directive(&prev),
        &doc,
        is_directive(node),
    )
    .then(|| Doc::concat(vec![Doc::HardLine, doc]))
}

/// `triplesBlock ::= triples ('.' triplesBlock?)?` — the nested block holds
/// the next statement in the graph. Separate it like top-level statements.
fn triples_block(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
    let parent = node
        .parent()
        .filter(|p| p.kind() == SyntaxKind::TriplesBlock)?;
    let prev = parent
        .children()
        .find(|c| c.kind() == SyntaxKind::Triples)?;
    let triples = node.children().find(|c| c.kind() == SyntaxKind::Triples)?;
    if newlines_before(node) > 1 {
        return None;
    }
    let doc = fmt.default(node);
    needs_blank_line(&fmt.child(&prev), false, &fmt.child(&triples), false)
        .then(|| Doc::concat(vec![Doc::HardLine, doc]))
}
