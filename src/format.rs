/// Wadler-Lindig pretty-printing combinators.
///
/// A `Doc` describes a document that can be rendered at a given line width.
/// `Group(d)` attempts to render `d` on a single line; if it doesn't fit
/// within the remaining width the group "breaks" and any `Line` inside
/// becomes a newline followed by the current indentation.
#[derive(Debug, Clone)]
pub enum Doc {
    Nil,
    Text(String),
    /// Soft line: a space when the enclosing group is flat, a newline +
    /// indentation when it is broken.
    Line,
    /// Hard line: always a newline + indentation regardless of group mode.
    HardLine,
    Concat(Vec<Doc>),
    /// Increase indentation for the inner document by `n` spaces.
    Nest(i32, Box<Doc>),
    /// Try to render the inner document flat; break if it exceeds the width.
    Group(Box<Doc>),
    /// Text that is only emitted when the enclosing group is broken.
    IfBreak(String),
}

impl Doc {
    pub fn nil() -> Self {
        Doc::Nil
    }

    pub fn text(s: impl Into<String>) -> Self {
        Doc::Text(s.into())
    }

    pub fn concat(docs: Vec<Doc>) -> Self {
        let flat: Vec<Doc> = docs
            .into_iter()
            .flat_map(|d| match d {
                Doc::Concat(inner) => inner,
                Doc::Nil => vec![],
                other => vec![other],
            })
            .collect();
        match flat.len() {
            0 => Doc::Nil,
            1 => flat.into_iter().next().unwrap(),
            _ => Doc::Concat(flat),
        }
    }

    pub fn nest(indent: i32, doc: Doc) -> Self {
        Doc::Nest(indent, Box::new(doc))
    }

    pub fn group(doc: Doc) -> Self {
        Doc::Group(Box::new(doc))
    }

    pub fn if_break(s: impl Into<String>) -> Self {
        Doc::IfBreak(s.into())
    }

    /// Whether rendering this doc always spans multiple lines.
    pub fn has_hardline(&self) -> bool {
        match self {
            Doc::HardLine => true,
            Doc::Concat(ds) => ds.iter().any(Doc::has_hardline),
            Doc::Nest(_, d) | Doc::Group(d) => d.has_hardline(),
            Doc::Nil | Doc::Text(_) | Doc::Line | Doc::IfBreak(_) => false,
        }
    }

    /// The first character this doc renders, if any. Line breaks count as
    /// `'\n'`; `IfBreak` text is ignored since it may not be rendered.
    pub fn first_char(&self) -> Option<char> {
        match self {
            Doc::Text(s) => s.chars().next(),
            Doc::Line | Doc::HardLine => Some('\n'),
            Doc::Concat(ds) => ds.iter().find_map(Doc::first_char),
            Doc::Nest(_, d) | Doc::Group(d) => d.first_char(),
            Doc::Nil | Doc::IfBreak(_) => None,
        }
    }

    /// The last character this doc renders, if any. See [`Doc::first_char`].
    pub fn last_char(&self) -> Option<char> {
        match self {
            Doc::Text(s) => s.chars().last(),
            Doc::Line | Doc::HardLine => Some('\n'),
            Doc::Concat(ds) => ds.iter().rev().find_map(Doc::last_char),
            Doc::Nest(_, d) | Doc::Group(d) => d.last_char(),
            Doc::Nil | Doc::IfBreak(_) => None,
        }
    }

    /// A comment runs to the end of the line, so it is followed by a hard
    /// line. This also forces all enclosing groups to break.
    pub fn comment(s: impl Into<String>) -> Self {
        Doc::Concat(vec![Doc::Text(s.into()), Doc::HardLine])
    }
}

/// Whether two tokens rendered as `left` directly followed by `right` must
/// be separated by a space: a word (keyword, prefixed name, number) would
/// otherwise run into the next word, IRI, string, variable or bracket, as in
/// `PREFIXex:` or `ex:<x>`. Punctuation like `(`, `.` or `@en` never gets one.
pub fn needs_space(left: Option<char>, right: Option<char>) -> bool {
    let (Some(l), Some(r)) = (left, right) else {
        return false;
    };
    let word = |c: char| c.is_alphanumeric() || c == '_' || c == ':';
    word(l) && (word(r) || matches!(r, '<' | '"' | '\'' | '?' | '$' | '[' | '{'))
}

/// Language-specific escape hatch for the generated formatters.
///
/// The grammar's `=== format ===` hints cover layout that only depends on a
/// node's kind and its parent's kind. Rules that depend on a node's content
/// (how many children it has, which tokens are redundant) go here.
pub trait FormatHook<L: rowan::Language> {
    /// Return `false` to leave this element out of the output.
    fn keep(&self, _element: &rowan::SyntaxElement<L>) -> bool {
        true
    }

    /// Replace the document for `node`.
    fn node(&self, _node: &rowan::SyntaxNode<L>, _fmt: &Formatter<'_, L>) -> Option<Doc> {
        None
    }
}

/// Gives a [`FormatHook`] access to the generated formatter.
pub struct Formatter<'a, L: rowan::Language> {
    child: &'a dyn Fn(&rowan::SyntaxNode<L>) -> Doc,
    default: &'a dyn Fn(&rowan::SyntaxNode<L>) -> Doc,
}

impl<'a, L: rowan::Language> Formatter<'a, L> {
    pub fn new(
        child: &'a dyn Fn(&rowan::SyntaxNode<L>) -> Doc,
        default: &'a dyn Fn(&rowan::SyntaxNode<L>) -> Doc,
    ) -> Self {
        Self { child, default }
    }

    /// Format a child node as its parent would: hook, hints and grouping.
    pub fn child(&self, node: &rowan::SyntaxNode<L>) -> Doc {
        (self.child)(node)
    }

    /// The hint-driven doc for `node` itself, without its own hook override
    /// or group wrapping. Its children still go through the hook.
    pub fn default(&self, node: &rowan::SyntaxNode<L>) -> Doc {
        (self.default)(node)
    }
}

/// Hook that keeps the grammar-driven defaults.
pub struct NoHook;

impl<L: rowan::Language> FormatHook<L> for NoHook {}

impl Doc {
    fn to_pretty(&self) -> pretty::RcDoc<'static, ()> {
        use pretty::RcDoc;
        match self {
            Doc::Nil => RcDoc::nil(),
            Doc::Text(s) => RcDoc::text(s.clone()),
            Doc::Line => RcDoc::line(),
            Doc::HardLine => RcDoc::hardline(),
            Doc::Concat(ds) => RcDoc::concat(ds.iter().map(Doc::to_pretty)),
            Doc::Nest(n, inner) => inner.to_pretty().nest(*n as isize),
            Doc::Group(inner) => inner.to_pretty().group(),
            Doc::IfBreak(s) => RcDoc::text(s.clone()).flat_alt(RcDoc::nil()),
        }
    }
}

/// Output sink that drops indentation on lines that stay empty.
///
/// `pretty` writes the indentation directly after a newline; it is held back
/// here until text follows, so blank lines carry no trailing whitespace.
struct TrimBlankLines {
    out: String,
    after_newline: bool,
    pending_indent: String,
}

impl std::fmt::Write for TrimBlankLines {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if s == "\n" {
            self.pending_indent.clear();
            self.out.push('\n');
            self.after_newline = true;
        } else if self.after_newline && s.bytes().all(|b| b == b' ') {
            self.pending_indent.push_str(s);
        } else {
            self.out.push_str(&self.pending_indent);
            self.pending_indent.clear();
            self.out.push_str(s);
            self.after_newline = false;
        }
        Ok(())
    }
}

/// Render a `Doc` to a `String` within the given line `width`.
///
/// Rendering is delegated to the `pretty` crate. A `HardLine` inside a group
/// makes that group break, so multi-line children break their parents.
pub fn render(doc: &Doc, width: usize) -> String {
    let mut out = TrimBlankLines {
        out: String::new(),
        after_newline: false,
        pending_indent: String::new(),
    };
    // Writing to a String cannot fail.
    let _ = doc.to_pretty().render_fmt(width, &mut out);
    out.out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_renders_literally() {
        let doc = Doc::text("hello");
        assert_eq!(render(&doc, 80), "hello");
    }

    #[test]
    fn hardline_breaks_enclosing_groups() {
        // A multi-line child forces its parent group to break, even though
        // the flat text would fit.
        let inner = Doc::concat(vec![Doc::text("a"), Doc::HardLine, Doc::text("b")]);
        let doc = Doc::group(Doc::concat(vec![
            Doc::text("("),
            Doc::nest(2, Doc::concat(vec![Doc::Line, inner])),
            Doc::Line,
            Doc::text(")"),
        ]));
        assert_eq!(render(&doc, 80), "(\n  a\n  b\n)");
    }

    #[test]
    fn blank_lines_have_no_indentation() {
        let doc = Doc::nest(
            2,
            Doc::concat(vec![
                Doc::text("a"),
                Doc::HardLine,
                Doc::HardLine,
                Doc::text("b"),
            ]),
        );
        assert_eq!(render(&doc, 80), "a\n\n  b");
    }

    #[test]
    fn if_break_only_renders_when_broken() {
        let doc = Doc::group(Doc::concat(vec![
            Doc::text("["),
            Doc::nest(
                2,
                Doc::concat(vec![Doc::Line, Doc::text("p o"), Doc::if_break(";")]),
            ),
            Doc::Line,
            Doc::text("]"),
        ]));
        assert_eq!(render(&doc, 80), "[ p o ]");
        assert_eq!(render(&doc, 5), "[\n  p o;\n]");
    }

    #[test]
    fn needs_space_between_words_only() {
        assert!(needs_space(Some('X'), Some('e'))); // PREFIX ex:
        assert!(needs_space(Some(':'), Some('<'))); // ex: <x>
        assert!(needs_space(Some('E'), Some('<'))); // BASE <x>
        assert!(!needs_space(Some('"'), Some('@'))); // "x"@en
        assert!(!needs_space(Some('D'), Some('('))); // BOUND(
        assert!(!needs_space(Some('a'), Some('.'))); // ex:a.
        assert!(!needs_space(Some(' '), Some('e')));
        assert!(!needs_space(None, Some('e')));
    }

    #[test]
    fn group_fits_on_one_line() {
        // ["a", "b", "c"] rendered with width=80 → fits flat → "a b c"
        let doc = Doc::group(Doc::concat(vec![
            Doc::text("a"),
            Doc::Line,
            Doc::text("b"),
            Doc::Line,
            Doc::text("c"),
        ]));
        assert_eq!(render(&doc, 80), "a b c");
    }

    #[test]
    fn group_breaks_when_too_wide() {
        // Same doc but width=4 → breaks → "a\nb\nc"
        let doc = Doc::group(Doc::concat(vec![
            Doc::text("a"),
            Doc::Line,
            Doc::text("b"),
            Doc::Line,
            Doc::text("c"),
        ]));
        assert_eq!(render(&doc, 4), "a\nb\nc");
    }

    #[test]
    fn nest_indents_broken_lines() {
        let doc = Doc::group(Doc::concat(vec![
            Doc::text("{"),
            Doc::nest(
                2,
                Doc::concat(vec![Doc::Line, Doc::text("x"), Doc::Line, Doc::text("y")]),
            ),
            Doc::Line,
            Doc::text("}"),
        ]));
        // Width=80 → fits flat
        assert_eq!(render(&doc, 80), "{ x y }");
        // Width=5 → breaks: "{\n  x\n  y\n}"
        assert_eq!(render(&doc, 5), "{\n  x\n  y\n}");
    }

    #[test]
    fn hardline_always_breaks() {
        let doc = Doc::group(Doc::concat(vec![
            Doc::text("a"),
            Doc::HardLine,
            Doc::text("b"),
        ]));
        // Even with large width, HardLine always breaks.
        assert_eq!(render(&doc, 999), "a\nb");
    }

    #[test]
    fn concat_flattens_nils() {
        let doc = Doc::concat(vec![Doc::Nil, Doc::text("x"), Doc::Nil]);
        assert_eq!(render(&doc, 80), "x");
    }

    #[test]
    fn nest_propagates_to_all_lines_in_concat() {
        // Regression: Nest(2, concat([Line, "a", ",", Line, "b"])) should
        // indent BOTH Lines, not just the first.
        let inner = Doc::concat(vec![
            Doc::Line,
            Doc::text("a"),
            Doc::text(","),
            Doc::Line,
            Doc::text("b"),
        ]);
        let doc = Doc::group(Doc::concat(vec![
            Doc::text("{"),
            Doc::nest(2, inner),
            Doc::Line,
            Doc::text("}"),
        ]));
        // Too narrow to fit flat → must break
        assert_eq!(render(&doc, 5), "{\n  a,\n  b\n}");
    }
}
