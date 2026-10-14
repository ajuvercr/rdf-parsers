//! Formatting rules shared by the Turtle-like grammars (Turtle, TriG).
//!
//! The languages have distinct `SyntaxKind` enums with the same variant
//! names, so the rules are shared as a macro expanded in each language's
//! `format` module, where `Lang`, `SyntaxKind`, `SyntaxNode`,
//! `SyntaxElement`, `Doc`, `Formatter` and `NodeOrToken` are in scope.

macro_rules! turtle_like_format_rules {
    () => {
        /// A `;` that is not followed by another predicate is redundant:
        /// `<a> <b> <c>;.` and `;;`. Multi-line blank nodes get their trailing
        /// `;` back in `predicate_object_list`.
        fn keep_element(element: &SyntaxElement) -> bool {
            if element.kind() == SyntaxKind::Colon {
                return next_significant(element).map(|e| e.kind()) == Some(SyntaxKind::Verb);
            }
            true
        }

        /// Whether two neighbouring statements get a blank line between them:
        /// when going from directives to triples, or when either spans
        /// multiple lines.
        fn needs_blank_line(
            prev: &Doc,
            prev_is_directive: bool,
            doc: &Doc,
            is_directive: bool,
        ) -> bool {
            (prev_is_directive && !is_directive) || prev.has_hardline() || doc.has_hardline()
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

        fn verb_count(node: &SyntaxNode) -> usize {
            node.children()
                .filter(|c| c.kind() == SyntaxKind::Verb)
                .count()
        }

        /// Blank nodes with several predicates end every line with `;`.
        fn predicate_object_list(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
            let in_bnode = node.parent().is_some_and(|p| {
                matches!(
                    p.kind(),
                    SyntaxKind::BlankNodePropertyList | SyntaxKind::BlankNodePropertyList2
                )
            });
            (in_bnode && verb_count(node) > 1)
                .then(|| Doc::concat(vec![fmt.default(node), Doc::text(";")]))
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
                pol.parent()
                    .is_some_and(|p| p.kind() == SyntaxKind::Triples)
                    && pol
                        .children()
                        .find(|c| c.kind() == SyntaxKind::ObjectList)
                        .as_ref()
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
            Some(if is_hanging(node) {
                Doc::nest(-2, doc)
            } else {
                doc
            })
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
                    NodeOrToken::Node(n) if n.kind() == SyntaxKind::Comma => {
                        docs.push(Doc::text(", "))
                    }
                    NodeOrToken::Node(n) if n.kind() == SyntaxKind::Object => {
                        docs.push(fmt.child(&n))
                    }
                    NodeOrToken::Node(n) => docs.push(Doc::text(n.text().to_string())),
                }
            }
            Doc::concat(docs)
        }
    };
}
