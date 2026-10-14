//! Formatting rules shared by the Turtle-like grammars (Turtle, TriG, N3).
//!
//! The languages have distinct `SyntaxKind` enums with the same variant
//! names, so the rules are shared as a macro expanded in each language's
//! `format` module, where `Lang`, `SyntaxKind`, `SyntaxNode`,
//! `SyntaxElement`, `Doc`, `Formatter` and `NodeOrToken` are in scope.
//!
//! - `bnode_lists`: kinds whose predicate list ends with `;` when it has
//!   several predicates (`[ ... ]`).
//! - `blocks`: bracketed kinds that lay themselves out over several lines
//!   when they have content (blank nodes, collections, formulas).

macro_rules! turtle_like_format_rules {
    (bnode_lists: [$($bnode:ident),*], blocks: [$($block:ident),*] $(,)?) => {
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

        /// The statement directly before `node`: the nearest preceding sibling
        /// for which `is_statement` holds, skipping whitespace, trailing
        /// comments of that statement and siblings for which `skip` holds. An
        /// own-line comment in between belongs to `node`, so then there is none.
        fn prev_statement(
            node: &SyntaxNode,
            is_statement: impl Fn(SyntaxKind) -> bool,
            skip: impl Fn(SyntaxKind) -> bool,
        ) -> Option<SyntaxNode> {
            let mut prev = node.prev_sibling_or_token();
            while let Some(e) = prev {
                match &e {
                    NodeOrToken::Token(t) if t.kind() == SyntaxKind::WhiteSpace => {}
                    NodeOrToken::Token(t) if t.kind() == SyntaxKind::Comment => {
                        let own_line = match t.prev_sibling_or_token() {
                            Some(NodeOrToken::Token(ws)) if ws.kind() == SyntaxKind::WhiteSpace => {
                                ws.text().contains('\n')
                            }
                            Some(_) => false,
                            None => true,
                        };
                        if own_line {
                            return None;
                        }
                    }
                    NodeOrToken::Node(n) if is_statement(n.kind()) => return Some(n.clone()),
                    NodeOrToken::Node(n) if skip(n.kind()) => {}
                    _ => return None,
                }
                prev = e.prev_sibling_or_token();
            }
            None
        }

        fn verb_count(node: &SyntaxNode) -> usize {
            node.children()
                .filter(|c| c.kind() == SyntaxKind::Verb)
                .count()
        }

        /// Blank nodes with several predicates end every line with `;`.
        fn predicate_object_list(node: &SyntaxNode, fmt: &Formatter<'_, Lang>) -> Option<Doc> {
            let in_bnode = node
                .parent()
                .is_some_and(|p| matches!(p.kind(), $(SyntaxKind::$bnode)|*));
            (in_bnode && verb_count(node) > 1)
                .then(|| Doc::concat(vec![fmt.default(node), Doc::text(";")]))
        }

        /// Whether an object contains a block with content (a blank node,
        /// collection or formula), possibly wrapped in expression nodes.
        fn is_nested(object: &SyntaxNode) -> bool {
            object.descendants().any(|d| {
                matches!(d.kind(), $(SyntaxKind::$block)|*)
                    // More than just the opening and closing bracket.
                    && d.children().count() > 2
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
