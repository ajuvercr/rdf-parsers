use chumsky::{extra::Err, prelude::*};
use proc_macro2::Span;
use std::collections::HashMap;
use syn::Ident;

use crate::parser::section_header;

fn uniform_ident(ident: &str) -> String {
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
    pub with: HashMap<String, String>,
}

impl Context {
    pub fn ident_for(&self, inp: &str) -> Ident {
        let mapped = self.rename.get(inp).map(|x| x.as_str()).unwrap_or(inp);

        let st = uniform_ident(mapped);

        Ident::new(&st, Span::call_site())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CtxBlock {
    Rename(Vec<(String, String)>),
    With(Vec<(String, String)>),
}

pub fn context_parser<'src>() -> impl Parser<'src, &'src str, Context, Err<Rich<'src, char>>> {
    // Read the rest of the line, trim it.
    let line_text = none_of("\n\r -><")
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

    let rename = section_header("rename", "===")
        .ignore_then(mapping)
        .map(CtxBlock::Rename);

    let with = section_header("with", "===")
        .ignore_then(mapping)
        .map(CtxBlock::With);

    let block = rename.or(with);

    section_header("context", "==")
        .labelled("context header")
        .ignore_then(block.repeated().collect())
        .map(|blocks: Vec<_>| {
            let mut ctx = Context::default();
            for b in blocks {
                match b {
                    CtxBlock::Rename(entries) => ctx.rename.extend(entries),
                    CtxBlock::With(entries) => ctx.with.extend(entries),
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
