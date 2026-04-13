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
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Flat,
    Break,
}

/// Returns `true` if `docs` (in the current work list) fit within `width`
/// remaining characters when rendered flat.
fn fits(mut width: isize, docs: &[(&Doc, i32, Mode)]) -> bool {
    let mut i = docs.len();
    loop {
        if width < 0 {
            return false;
        }
        if i == 0 {
            return true;
        }
        i -= 1;
        let (doc, indent, mode) = docs[i];
        match doc {
            Doc::Nil => {}
            Doc::Text(s) => width -= s.len() as isize,
            Doc::Line => {
                if mode == Mode::Flat {
                    width -= 1; // space
                } else {
                    return true; // a break always fits "the rest"
                }
            }
            Doc::HardLine => return true,
            Doc::Concat(ds) => {
                // Push children in reverse so we process them in order
                // We need a local copy to extend the slice view — handled
                // by the iterative approach below via a local vec.
                //
                // Instead, recurse with an extended slice.
                let mut extended: Vec<(&Doc, i32, Mode)> = docs[..i].to_vec();
                for d in ds.iter().rev() {
                    extended.push((d, indent, mode));
                }
                return fits(width, &extended);
            }
            Doc::Nest(n, inner) => {
                let mut extended: Vec<(&Doc, i32, Mode)> = docs[..i].to_vec();
                extended.push((inner, indent + n, mode));
                return fits(width, &extended);
            }
            Doc::Group(inner) => {
                let mut extended: Vec<(&Doc, i32, Mode)> = docs[..i].to_vec();
                extended.push((inner, indent, Mode::Flat));
                return fits(width, &extended);
            }
        }
    }
}

/// Render a `Doc` to a `String` within the given line `width`.
pub fn render(doc: &Doc, width: usize) -> String {
    let mut out = String::new();
    let mut col: i32 = 0;

    // Work list: (doc, indent, mode). Processed front-to-back.
    let mut work: Vec<(&Doc, i32, Mode)> = vec![(doc, 0, Mode::Break)];

    while let Some((doc, indent, mode)) = work.pop() {
        match doc {
            Doc::Nil => {}
            Doc::Text(s) => {
                out.push_str(s);
                col += s.len() as i32;
            }
            Doc::Line => {
                if mode == Mode::Flat {
                    out.push(' ');
                    col += 1;
                } else {
                    out.push('\n');
                    for _ in 0..indent {
                        out.push(' ');
                    }
                    col = indent;
                }
            }
            Doc::HardLine => {
                out.push('\n');
                for _ in 0..indent {
                    out.push(' ');
                }
                col = indent;
            }
            Doc::Concat(ds) => {
                // Push in reverse so the first element is processed first.
                for d in ds.iter().rev() {
                    work.push((d, indent, mode));
                }
            }
            Doc::Nest(n, inner) => {
                work.push((inner, indent + n, mode));
            }
            Doc::Group(inner) => {
                // Build a temporary slice to pass to `fits`.
                let remaining = width as isize - col as isize;
                // Collect current work list plus the inner doc so fits() can
                // measure the whole upcoming context.
                let mut probe: Vec<(&Doc, i32, Mode)> = work.clone();
                probe.push((inner, indent, Mode::Flat));
                let chosen = if fits(remaining, &probe) {
                    Mode::Flat
                } else {
                    Mode::Break
                };
                work.push((inner, indent, chosen));
            }
        }
    }

    out
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
