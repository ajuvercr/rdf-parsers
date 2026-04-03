use chumsky::{extra::Err, prelude::*};
use proc_macro2::Span;
use std::collections::HashMap;
use syn::Ident;

use crate::parser::section_header;

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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Context {
    pub rename: HashMap<String, String>,
    with: HashMap<String, String>,
    pub error_values: HashMap<String, isize>,
    /// Tokens whose `bracket_delta()` is +1 (openers: `{`, `[`, `(`).
    pub bracket_openers: Vec<String>,
    /// Tokens whose `bracket_delta()` is -1 (closers: `}`, `]`, `)`).
    pub bracket_closers: Vec<String>,
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

    let block = rename.or(with).or(error_values).or(bracket_openers).or(bracket_closers);

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
