//! Turtle formatting.
//!
//! Layout that depends only on node kinds lives in the `=== format ===`
//! section of `grammars/turtle.txt`. The rules here depend on a node's
//! content or its neighbours, which the hints cannot express.

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
        // A `;` that is not followed by another predicate is redundant:
        // `<a> <b> <c>;.` and `;;`. Multi-line blank nodes get their trailing
        // `;` back below.
        if element.kind() == SyntaxKind::Colon {
            return next_significant(element).map(|e| e.kind()) == Some(SyntaxKind::Verb);
        }
        true
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

/// Newlines in the whitespace directly before `node`.
fn newlines_before(node: &SyntaxNode) -> usize {
    match node.prev_sibling_or_token() {
        Some(NodeOrToken::Token(t)) if t.kind() == SyntaxKind::WhiteSpace => {
            t.text().matches('\n').count()
        }
        _ => 0,
    }
}

/// The statement directly before `node`, unless a comment sits in between.
fn prev_statement(node: &SyntaxNode) -> Option<SyntaxNode> {
    let mut prev = node.prev_sibling_or_token();
    while let Some(e) = prev {
        match e {
            NodeOrToken::Token(ref t) if t.kind() == SyntaxKind::WhiteSpace => {}
            NodeOrToken::Node(n) if n.kind() == SyntaxKind::Statement => return Some(n),
            _ => return None,
        }
        prev = e.prev_sibling_or_token();
    }
    None
}

fn is_directive(statement: &SyntaxNode) -> bool {
    statement.children().any(|c| c.kind() == SyntaxKind::Directive)
}

/// Separate statements with a blank line when going from directives to
/// triples, or when either statement spans multiple lines. Blank lines from
/// the source are kept by the generated formatter already.
fn statement(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
    let prev = prev_statement(node)?;
    if newlines_before(node) > 1 {
        return None;
    }
    let doc = fmt.default(node);
    let blank = (is_directive(&prev) && !is_directive(node))
        || doc.has_hardline()
        || fmt.default(&prev).has_hardline();
    blank.then(|| Doc::concat(vec![Doc::HardLine, doc]))
}

fn verb_count(node: &SyntaxNode) -> usize {
    node.children().filter(|c| c.kind() == SyntaxKind::Verb).count()
}

/// Blank nodes with several predicates end every line with `;`.
fn predicate_object_list(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
    let in_bnode = node.parent().is_some_and(|p| {
        matches!(
            p.kind(),
            SyntaxKind::BlankNodePropertyList | SyntaxKind::BlankNodePropertyList2
        )
    });
    (in_bnode && verb_count(node) > 1).then(|| Doc::concat(vec![fmt.default(node), Doc::text(";")]))
}

/// A blank node or collection with content.
fn is_nested(object: &SyntaxNode) -> bool {
    object.children().any(|c| match c.kind() {
        SyntaxKind::BlankNodePropertyList2 => c
            .children()
            .any(|c| c.kind() == SyntaxKind::PredicateObjectList),
        SyntaxKind::Collection => c.children().any(|c| c.kind() == SyntaxKind::Object),
        _ => false,
    })
}

/// The first object list of a triple shares the subject's line. Blocks
/// opened there close at the subject's indentation, not one level deeper.
fn is_hanging(object_list: &SyntaxNode) -> bool {
    object_list.parent().is_some_and(|pol| {
        pol.parent().is_some_and(|p| p.kind() == SyntaxKind::Triples)
            && pol.children().find(|c| c.kind() == SyntaxKind::ObjectList).as_ref()
                == Some(object_list)
    })
}

/// Simple objects go all on one line or each on their own indented line.
/// Nested structures lay themselves out, so `], [` stays together.
fn object_list(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
    let doc = if node.children().any(|c| is_nested(&c)) {
        nested_object_list(node, fmt)
    } else {
        Doc::nest(2, fmt.default(node))
    };
    Some(if is_hanging(node) { Doc::nest(-2, doc) } else { doc })
}

fn nested_object_list(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Doc {
    let mut docs = Vec::new();
    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Token(t) if t.kind() == SyntaxKind::Comment => {
                docs.push(Doc::text(" "));
                docs.push(Doc::comment(t.text().to_string()));
            }
            NodeOrToken::Token(_) => {}
            NodeOrToken::Node(n) if n.kind() == SyntaxKind::Comma => docs.push(Doc::text(", ")),
            NodeOrToken::Node(n) if n.kind() == SyntaxKind::Object => docs.push(fmt.child(&n)),
            NodeOrToken::Node(n) => docs.push(Doc::text(n.text().to_string())),
        }
    }
    Doc::concat(docs)
}
