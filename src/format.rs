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

/// The text a doc starts (or ends) with, if no line break or space comes
/// first: the token that would touch a neighbouring doc.
pub fn edge_text(docs: &[Doc], from_end: bool) -> Option<&str> {
    enum Edge<'a> {
        Text(&'a str),
        Break,
        Empty,
    }
    fn walk(doc: &Doc, from_end: bool) -> Edge<'_> {
        match doc {
            Doc::Text(t) if t.is_empty() => Edge::Empty,
            Doc::Text(t) if t.trim().is_empty() => Edge::Break,
            Doc::Text(t) => Edge::Text(t),
            Doc::Line | Doc::HardLine => Edge::Break,
            Doc::Nil | Doc::IfBreak(_) => Edge::Empty,
            Doc::Nest(_, d) | Doc::Group(d) => walk(d, from_end),
            Doc::Concat(ds) => walk_all(ds, from_end),
        }
    }
    fn walk_all(docs: &[Doc], from_end: bool) -> Edge<'_> {
        let mut iter: Box<dyn Iterator<Item = &Doc>> = if from_end {
            Box::new(docs.iter().rev())
        } else {
            Box::new(docs.iter())
        };
        iter.find_map(|d| match walk(d, from_end) {
            Edge::Empty => None,
            edge => Some(edge),
        })
        .unwrap_or(Edge::Empty)
    }
    match walk_all(docs, from_end) {
        Edge::Text(t) => Some(t),
        Edge::Break | Edge::Empty => None,
    }
}

/// Whether `left` directly followed by `right` lexes differently from the
/// two separately: `tokens` yields the token spans of a text. Then a space
/// must separate them, e.g. `"x"@en--ltr` followed by `.`.
pub fn glues(
    left: &str,
    right: &str,
    tokens: impl Fn(&str) -> Vec<std::ops::Range<usize>>,
) -> bool {
    let joined = format!("{left}{right}");
    let at = left.len();
    tokens(&joined)
        .iter()
        .any(|span| span.start < at && span.end > at)
}

/// Number of hard lines that `docs` ends with (looking through nested docs).
pub fn trailing_hardlines(docs: &[Doc]) -> usize {
    // Returns whether `doc` consists only of hard lines (and empty docs).
    fn walk(doc: &Doc, n: &mut usize) -> bool {
        match doc {
            Doc::HardLine => {
                *n += 1;
                true
            }
            Doc::Nil | Doc::IfBreak(_) => true,
            Doc::Concat(ds) => ds.iter().rev().all(|d| walk(d, n)),
            Doc::Nest(_, d) | Doc::Group(d) => walk(d, n),
            Doc::Text(_) | Doc::Line => false,
        }
    }
    let mut n = 0;
    for doc in docs.iter().rev() {
        if !walk(doc, &mut n) {
            break;
        }
    }
    n
}

/// Number of hard lines `doc` starts with (looking through nested docs).
pub fn leading_hardlines(doc: &Doc) -> usize {
    // Returns whether `doc` consists only of hard lines (and empty docs).
    fn walk(doc: &Doc, n: &mut usize) -> bool {
        match doc {
            Doc::HardLine => {
                *n += 1;
                true
            }
            Doc::Nil | Doc::IfBreak(_) => true,
            Doc::Concat(ds) => ds.iter().all(|d| walk(d, n)),
            Doc::Nest(_, d) | Doc::Group(d) => walk(d, n),
            Doc::Text(_) | Doc::Line => false,
        }
    }
    let mut n = 0;
    walk(doc, &mut n);
    n
}

/// Remove up to `n` hard lines that `docs` ends with (looking through nested
/// docs) and return how many went. Used when the next doc starts with line
/// breaks of its own: those carry the right indentation for what follows.
pub fn remove_trailing_hardlines(docs: &mut Vec<Doc>, n: usize) -> usize {
    let mut removed = 0;
    while removed < n {
        let Some(last) = docs.last_mut() else { break };
        match last {
            Doc::HardLine => {
                docs.pop();
                removed += 1;
            }
            Doc::Nil => {
                docs.pop();
            }
            Doc::Concat(ds) => {
                removed += remove_trailing_hardlines(ds, n - removed);
                if !ds.is_empty() {
                    break;
                }
                docs.pop();
            }
            Doc::Nest(_, d) | Doc::Group(d) => {
                let mut inner = vec![std::mem::replace(&mut **d, Doc::Nil)];
                removed += remove_trailing_hardlines(&mut inner, n - removed);
                let empty = inner.is_empty();
                **d = Doc::concat(inner);
                if !empty {
                    break;
                }
                docs.pop();
            }
            Doc::Text(_) | Doc::Line | Doc::IfBreak(_) => break,
        }
    }
    removed
}

/// Remove the line breaks and spaces `docs` ends with (looking through nested
/// docs), so the next token attaches to the previous one. Breaks that end a
/// comment are kept: anything glued to a comment would become part of it.
pub fn strip_trailing_breaks(docs: &mut Vec<Doc>) {
    fn last_text(docs: &[Doc]) -> Option<&str> {
        docs.iter().rev().find_map(|d| match d {
            Doc::Text(t) if t != " " => Some(t.as_str()),
            Doc::Concat(ds) => last_text(ds),
            Doc::Nest(_, d) | Doc::Group(d) => last_text(std::slice::from_ref(d.as_ref())),
            _ => None,
        })
    }
    // Returns whether `doc` was a break that should be removed entirely.
    fn strip(doc: &mut Doc) -> bool {
        match doc {
            Doc::HardLine | Doc::Line | Doc::Nil => true,
            Doc::Text(t) => t == " ",
            Doc::Concat(ds) => {
                strip_trailing_breaks(ds);
                ds.is_empty()
            }
            Doc::Nest(_, d) | Doc::Group(d) => strip(d),
            Doc::IfBreak(_) => false,
        }
    }
    if last_text(docs).is_some_and(|t| t.starts_with('#')) {
        return;
    }
    while let Some(last) = docs.last_mut() {
        if !strip(last) {
            break;
        }
        docs.pop();
    }
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
            Doc::Concat(ds) => concat_balanced(ds),
            Doc::Nest(n, inner) => inner.to_pretty().nest(*n as isize),
            Doc::Group(inner) => inner.to_pretty().group(),
            Doc::IfBreak(s) => RcDoc::text(s.clone()).flat_alt(RcDoc::nil()),
        }
    }
}

/// `RcDoc::concat` builds a chain of appends as deep as the list is long,
/// which `pretty` renders and drops recursively: a document with tens of
/// thousands of statements overflows the stack. Splitting in halves keeps
/// the depth logarithmic.
fn concat_balanced(docs: &[Doc]) -> pretty::RcDoc<'static, ()> {
    match docs {
        [] => pretty::RcDoc::nil(),
        [doc] => doc.to_pretty(),
        _ => {
            let (left, right) = docs.split_at(docs.len() / 2);
            concat_balanced(left).append(concat_balanced(right))
        }
    }
}

/// Output sink that drops indentation on lines that stay empty and spaces at
/// the end of a line.
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
            // No token ends in a space, so trailing spaces are layout only.
            let trimmed = self.out.trim_end_matches(' ').len();
            self.out.truncate(trimmed);
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
    fn strip_trailing_breaks_glues_but_keeps_comment_ends() {
        let mut docs = vec![Doc::concat(vec![Doc::text("}"), Doc::HardLine]), Doc::Line];
        strip_trailing_breaks(&mut docs);
        assert_eq!(render(&Doc::concat(docs), 80), "}");

        let mut docs = vec![Doc::text("# note"), Doc::HardLine];
        strip_trailing_breaks(&mut docs);
        assert_eq!(render(&Doc::concat(docs), 80), "# note\n");
    }

    #[test]
    fn long_documents_do_not_overflow_the_stack() {
        // One part per statement, as the generated formatters produce.
        let parts: Vec<Doc> = (0..200_000)
            .flat_map(|i| [Doc::text(format!("<s{i}> <p> <o>.")), Doc::HardLine])
            .collect();
        let out = render(&Doc::concat(parts), 80);
        assert_eq!(out.lines().count(), 200_000);
    }

    #[test]
    fn hard_lines_combine_across_docs() {
        let mut part = vec![Doc::concat(vec![Doc::text("# c"), Doc::HardLine])];
        let next = Doc::nest(
            2,
            Doc::concat(vec![Doc::HardLine, Doc::HardLine, Doc::text("x")]),
        );
        let lead = leading_hardlines(&next);
        assert_eq!(lead, 2);
        assert_eq!(remove_trailing_hardlines(&mut part, lead), 1);
        part.push(next);
        // One blank line between, as `next` asked; `x` keeps its indentation.
        assert_eq!(render(&Doc::concat(part), 80), "# c\n\n  x");
    }

    #[test]
    fn lines_have_no_trailing_spaces() {
        let doc = Doc::concat(vec![Doc::text("a "), Doc::HardLine, Doc::text("b")]);
        assert_eq!(render(&doc, 80), "a\nb");
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
