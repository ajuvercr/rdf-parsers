use chumsky::{extra::Err, prelude::*};
use proc_macro2::Span;
use std::collections::HashMap;
use syn::Ident;

use crate::parser::section_header;

fn ident<'src>() -> impl Parser<'src, &'src str, String, Err<Rich<'src, char>>> + Clone {
    any()
        .filter(|x: &char| x.is_ascii_alphanumeric() || *x == '_')
        .repeated()
        .at_least(1)
        .collect()
}

pub(crate) fn uniform_ident(ident: &str) -> String {
    let mut out = String::new();

    let mut should_uppercase = true;
    let mut is_uppercase = false;
    for c in ident.chars() {
        if !is_uppercase && c.is_ascii_uppercase() {
            should_uppercase = true;
        }

        if c.is_alphanumeric() {
            if should_uppercase {
                should_uppercase = false;
                out.push(c.to_ascii_uppercase());
            } else {
                out.push(c.to_ascii_lowercase());
            }
        } else {
            should_uppercase = true;
        }

        is_uppercase = c.is_ascii_uppercase();
    }

    out
}
// ── Format annotation types ────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatPosition {
    Before,
    After,
}

/// A single formatting hint keyword.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatHint {
    Space,
    Line,
    HardLine,
    /// Two consecutive hard newlines — produces a blank line between nodes.
    BlankLine,
    Indent,
    Dedent,
}

/// One entry from the `=== format ===` section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatEntry {
    /// Rule name this hint is scoped to (e.g. `jsonObject` in
    /// `jsonObject.curly_open`).  `None` means "any parent rule".
    pub scope: Option<String>,
    /// Token name (must match a key in `=== with ===` or a terminal name).
    pub token: String,
    pub items: Vec<(FormatPosition, Vec<FormatHint>)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Context {
    pub rename: HashMap<String, String>,
    with: HashMap<String, String>,
    pub error_values: HashMap<String, isize>,
    /// Tokens whose `bracket_delta()` is +1 (openers: `{`, `[`, `(`).
    pub bracket_openers: Vec<String>,
    /// Tokens whose `bracket_delta()` is -1 (closers: `}`, `]`, `)`).
    pub bracket_closers: Vec<String>,
    /// Producing rule names that should be wrapped in a `Doc::Group`.
    pub format_groups: Vec<String>,
    /// Per-token formatting hints from the `=== format ===` section.
    pub format_hints: Vec<FormatEntry>,
}

impl Context {
    pub fn ident_for(&self, inp: &str) -> Ident {
        let mapped = self.rename.get(inp).map(|x| x.as_str()).unwrap_or(inp);

        let st = uniform_ident(mapped);

        Ident::new(&st, Span::call_site())
    }

    pub fn with(&self, literal: &str) -> String {
        if let Some(n) = self.with.get(literal) {
            n.clone()
        } else {
            let good = literal.chars().next().unwrap().is_alphabetic()
                && literal
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == ' ' || c == '_');

            if good {
                uniform_ident(&format!("{}_Lit", literal))
            } else {
                eprintln!("Failed to find with for {}", literal);
                panic!("aah")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CtxBlock {
    Rename(Vec<(String, String)>),
    With(Vec<(String, String)>),
    ErrorValues(Vec<(String, isize)>),
    BracketOpeners(Vec<String>),
    BracketClosers(Vec<String>),
    Format(Vec<String>, Vec<FormatEntry>),
}

fn format_section_parser<'src>() -> impl Parser<'src, &'src str, CtxBlock, Err<Rich<'src, char>>> {
    // Parses the `=== format ===` subsection.
    //
    // Each line is one of:
    //   group <ident>
    //   [<scope>.]<token> -> before|after: <hint>+
    //
    // Hints: space | line | hardline | indent | dedent

    let hint = choice((
        just("blankline").to(FormatHint::BlankLine),
        just("hardline").to(FormatHint::HardLine),
        just("space").to(FormatHint::Space),
        just("line").to(FormatHint::Line),
        just("indent").to(FormatHint::Indent),
        just("dedent").to(FormatHint::Dedent),
    ))
    .padded_by(one_of(" \t").repeated());

    let position = choice((
        just("before").to(FormatPosition::Before),
        just("after").to(FormatPosition::After),
    ));

    // `[<scope>.]<token>` — dot-separated optional scope + mandatory token
    let scoped_token = ident()
        .then(just('.').ignore_then(ident()).or_not())
        .map(|(a, b)| match b {
            Some(tok) => (Some(a), tok),
            None => (None, a),
        })
        .padded_by(one_of(" \t").repeated());

    let item = position
        .then_ignore(just(':'))
        .padded_by(one_of(" \t").repeated())
        .then(hint.repeated().at_least(1).collect::<Vec<_>>())
        .padded_by(one_of(" \t").repeated());

    let hint_line = scoped_token
        .then_ignore(just("->").padded_by(one_of(" \t").repeated()))
        .then(item.separated_by(just(',')).at_least(1).collect::<Vec<_>>())
        .then_ignore(none_of("\n").repeated()) // ignore rest of line
        .then_ignore(just('\n').or_not())
        .map(|((scope, token), items)| {
            CtxLine::Hint(FormatEntry {
                scope,
                token,
                items,
            })
        });

    let group_line = just("group")
        .padded_by(one_of(" \t").repeated())
        .ignore_then(ident().padded_by(one_of(" \t").repeated()))
        .then_ignore(none_of("\n").repeated())
        .then_ignore(just('\n').or_not())
        .map(CtxLine::Group);

    let line = group_line.or(hint_line);

    section_header("format", "===")
        .ignore_then(line.repeated().collect::<Vec<_>>())
        .map(|lines| {
            let mut groups = Vec::new();
            let mut hints = Vec::new();
            for l in lines {
                match l {
                    CtxLine::Group(name) => groups.push(name),
                    CtxLine::Hint(entry) => hints.push(entry),
                }
            }
            CtxBlock::Format(groups, hints)
        })
}

#[derive(Debug, Clone)]
enum CtxLine {
    Group(String),
    Hint(FormatEntry),
}

pub fn context_parser<'src>() -> impl Parser<'src, &'src str, Context, Err<Rich<'src, char>>> {
    // Read the rest of the line, trim it.
    let line_text = none_of(" \n")
        .repeated()
        .collect::<String>()
        .map(|s| s.to_string())
        .padded();

    // key -> value
    let mapping = line_text
        .then_ignore(just("->"))
        .then(line_text)
        .repeated()
        .collect();

    // plain list of names (one per non-empty line); stops at a new section header
    let name_list = none_of(" \n=")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .padded()
        .repeated()
        .collect::<Vec<_>>();

    let rename = section_header("rename", "===")
        .ignore_then(mapping)
        .map(CtxBlock::Rename);

    let with = section_header("with", "===")
        .ignore_then(mapping)
        .map(CtxBlock::With);

    let error_values = section_header("error_value", "===")
        .ignore_then(mapping)
        .map(|vs| {
            vs.into_iter()
                .flat_map(|(x, y)| isize::from_str_radix(&y, 10).map(|y| (x, y)))
                .collect::<Vec<_>>()
        })
        .map(CtxBlock::ErrorValues);

    let bracket_openers = section_header("bracket_open", "===")
        .ignore_then(name_list.clone())
        .map(CtxBlock::BracketOpeners);

    let bracket_closers = section_header("bracket_close", "===")
        .ignore_then(name_list)
        .map(CtxBlock::BracketClosers);

    let block = rename
        .or(with)
        .or(error_values)
        .or(bracket_openers)
        .or(bracket_closers)
        .or(format_section_parser());

    section_header("context", "==")
        .labelled("context header")
        .ignore_then(block.repeated().collect())
        .map(|blocks: Vec<_>| {
            let mut ctx = Context::default();
            for b in blocks {
                match b {
                    CtxBlock::Rename(entries) => ctx.rename.extend(entries),
                    CtxBlock::With(entries) => ctx.with.extend(entries),
                    CtxBlock::ErrorValues(items) => ctx.error_values.extend(items),
                    CtxBlock::BracketOpeners(names) => ctx.bracket_openers.extend(names),
                    CtxBlock::BracketClosers(names) => ctx.bracket_closers.extend(names),
                    CtxBlock::Format(groups, hints) => {
                        ctx.format_groups.extend(groups);
                        ctx.format_hints.extend(hints);
                    }
                }
            }
            ctx
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let input = r#"== context ==
=== rename ===
String -> myString
double -> myDouble
=== with ===
subject -> mySubject
"#;

        let res = context_parser().parse(input).into_result();
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.rename.len(), 2);
        assert_eq!(res.with.len(), 1);
    }

    #[test]
    fn parses_format_section() {
        let input = r#"== context ==
=== format ===
group jsonObject
group jsonArray
curly_open -> after: indent line
curly_close -> before: dedent line
comma -> after: line
colon -> after: space
"#;

        let res = context_parser().parse(input).into_result();
        assert!(res.is_ok(), "{:?}", res.unwrap_err());
        let res = res.unwrap();
        assert_eq!(res.format_groups, vec!["jsonObject", "jsonArray"]);
        assert_eq!(res.format_hints.len(), 4);

        let h = &res.format_hints[0];
        assert_eq!(h.scope, None);
        assert_eq!(h.token, "curly_open");
        assert_eq!(h.items.len(), 1);
        assert_eq!(h.items[0].0, FormatPosition::After);
        assert_eq!(h.items[0].1, vec![FormatHint::Indent, FormatHint::Line]);

        let h = &res.format_hints[1];
        assert_eq!(h.token, "curly_close");
        assert_eq!(h.items.len(), 1);
        assert_eq!(h.items[0].0, FormatPosition::Before);
        assert_eq!(h.items[0].1, vec![FormatHint::Dedent, FormatHint::Line]);
    }

    #[test]
    fn parses_scoped_format_entry() {
        let input = r#"== context ==
=== format ===
jsonObject.curly_open -> after: indent line
"#;

        let res = context_parser().parse(input).into_result();
        assert!(res.is_ok(), "{:?}", res.unwrap_err());
        let res = res.unwrap();
        let h = &res.format_hints[0];
        assert_eq!(h.scope, Some("jsonObject".to_string()));
        assert_eq!(h.token, "curly_open");
        assert_eq!(h.items.len(), 1);
        assert_eq!(h.items[0].0, FormatPosition::After);
    }

    #[test]
    fn test_uniform_sparql_base() {
        assert_eq!(&uniform_ident(""), "");
        assert_eq!(&uniform_ident("prefixID"), "PrefixId");
        assert_eq!(&uniform_ident("iri"), "Iri");
        assert_eq!(
            &uniform_ident("blankNodePropertyList"),
            "BlankNodePropertyList"
        );

        assert_eq!(&uniform_ident("PN_CHARS_BASE"), "PnCharsBase");
    }

    //     #[test]
    //     fn parse_it() {
    //         let o = parse(
    //             r#" /* testing comment */
    // [1]  	QueryUnit	  ::=  	Query
    // [2]  	Query	  ::=  	Prologue ( SelectQuery | ConstructQuery | DescribeQuery | AskQuery ) ValuesClause
    // [3]  	UpdateUnit	  ::=  	Update
    // [4]  	Prologue	  ::=  	( BaseDecl | PrefixDecl )*
    // [5]  	BaseDecl	  ::=  	'BASE' IRIREF
    // [6]  	PrefixDecl	  ::=  	'PREFIX' PNAME_NS IRIREF
    // [7]  	SelectQuery	  ::=  	SelectClause DatasetClause* WhereClause SolutionModifier
    // [8]  	SubSelect	  ::=  	SelectClause WhereClause SolutionModifier ValuesClause
    //         "#,
    //         );
    //
    //         println!("{:?}", o);
    //         assert!(o.is_ok());
    //     }
}
