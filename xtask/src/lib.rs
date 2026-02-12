use ariadne::{Report, ReportKind, Source};
use chumsky::error::Rich;
use proc_macro::TokenStream;
use proc_macro2::token_stream;
use quote::quote;
use std::collections::HashSet;
use std::ops::Range;
use std::{io::Cursor, path::PathBuf};
use syn::{LitStr, parse_macro_input};

use crate::parser::{Config, Expr, Mark, Rule};

mod parser;

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd)]
enum Terminal {
    Literal(String),
    Ref(String),
}
impl Terminal {
    fn ident<'a, 'b: 'a>(&'a self, config: &'b Config) -> Option<&'a str> {
        match self {
            Terminal::Ref(x) => Some(x.as_str()),
            Terminal::Literal(x) => config.context.with.get(x).map(|x| x.as_str()),
        }
    }
}

fn render_ariadne_reports(file_id: &str, src: &str, errs: &[Rich<char>]) -> String {
    let mut out = Cursor::new(Vec::<u8>::new());

    for e in errs {
        let span = (file_id, e.span().into_range()); // Range<usize> into `src`
        let mut report = Report::build(ReportKind::Error, span.clone()).with_message("parse error");

        report = report.with_label(ariadne::Label::new(span.clone()).with_message(e.to_string()));
        let report: Report<'_, (&str, Range<usize>)> = report.finish();

        let source = Source::from(src);
        let _ = report.write((file_id, source), &mut out);
        out.get_mut().push(b'\n');
    }

    unsafe { String::from_utf8_unchecked(out.into_inner()) }
}

fn to_impl(
    expr: &Expr,
    ctx: &Config,
    terminals: &mut HashSet<Terminal>,
) -> token_stream::TokenStream {
    let imp = match expr {
        Expr::Marked(expr, Mark::Option) => {
            let imp = to_impl(expr, ctx, terminals);
            quote! {
                #imp;
                crate::ParseRes::Ok
            }
        }
        Expr::Marked(expr, Mark::Star) => {
            let imp = to_impl(expr, ctx, terminals);
            quote! {
                let mut output = crate::ParseRes::Ok;
                let mut checkpoint = parser.checkpoint();

                while output == crate::ParseRes::Ok {
                    checkpoint = parser.checkpoint();
                    output = { #imp };
                    if !parser.consumed_since(&checkpoint) {
                        return crate::ParseRes::IncorrectTermType(crate::TermType::Subject);
                    }
                }
                parser.reset(&checkpoint);
                if output == crate::ParseRes::Eof {
                    crate::ParseRes::Eof
                } else {
                    crate::ParseRes::Ok
                }
            }
        }
        Expr::Marked(expr, Mark::Plus) => {
            let imp = to_impl(expr, ctx, terminals);
            quote! {
                let mut output = { #imp };
                let mut checkpoint = parser.checkpoint();
                while output == crate::ParseRes::Ok {
                    output = { #imp };
                    if !parser.consumed_since(&checkpoint) {
                        return crate::ParseRes::IncorrectTermType(crate::TermType::Subject);
                    }
                }
                parser.reset(&checkpoint);
                if output == crate::ParseRes::Eof {
                    crate::ParseRes::Eof
                } else {
                    crate::ParseRes::Ok
                }
            }
        }
        Expr::Either(exprs) => {
            let parts: Vec<_> = exprs.iter().map(|e| to_impl(e, ctx, terminals)).collect();
            let first = &parts[0];
            let others = &parts[1..];
            quote! {
                let checkpoint = parser.checkpoint();
                let mut out = {#first};
                if out == crate::ParseRes::Ok {
                    return out;
                }

                #(
                    {
                        parser.reset(&checkpoint);
                        let o =  { #others };

                        if o == crate::ParseRes::Ok {
                            return o;
                        }

                        out = out.combine_or(o);
                    }
                )*

                out
            }
        }
        Expr::Seq(exprs) => {
            let parts = exprs.iter().map(|e| to_impl(e, ctx, terminals));
            quote! {
                let mut out = crate::ParseRes::Ok;
                #(
                    out = out.combine_and({#parts});
                    // This is kinda anoying, we want Unexected(STOP) to be seen later on
                    // So we will implement more functions!
                    if out == crate::ParseRes::Eof  {
                        return out;
                    }
                )*
                out
            }
        }
        Expr::Literal(literal_type, f) => {
            terminals.insert(Terminal::Literal(f.clone()));
            if let Some(name) = ctx.context.with.get(f) {
                let n = ctx.context.ident_for(name);
                quote! {
                    #n::parse(parser, context)
                }
            } else {
                let error = format!("Expected with literal {} ", f);
                quote! {
                    compile_error!(#error)
                }
            }
        }
        Expr::Reference(re) => {
            if !ctx.rules.producing.iter().any(|r| &r.name == re) {
                terminals.insert(Terminal::Ref(re.clone()));
            }

            let n = ctx.context.ident_for(re);
            quote! {
                #n::parse(parser, context)
            }
        }
    };
    imp
}

fn producing_trait_impl(
    rule: &Rule,
    ctx: &Config,
    terminals: &mut HashSet<Terminal>,
) -> token_stream::TokenStream {
    let n = ctx.context.ident_for(&rule.name);
    let imp = to_impl(&rule.expression, ctx, terminals);
    quote! {
        pub struct #n;

        impl crate::ParserTrait for #n {
            const KIND: SyntaxKind = SyntaxKind::#n;

            fn parse(parser: &mut crate::Parser, context: &mut crate::Context) -> crate::ParseRes {
                println!("Parsing {:?}", Self::KIND);
                let mut func = || {
                    #imp
                };
                let o = func() ;
                println!("Finished {:?} {:?}", Self::KIND, o);
                o
            }
        }
    }
}

fn terminal_trait_impl(terminal: &str, ctx: &Config) -> token_stream::TokenStream {
    let n = ctx.context.ident_for(&terminal);
    quote! {
        pub struct #n;

        impl crate::ParserTrait for #n {
            const KIND: SyntaxKind = SyntaxKind::#n;

            fn parse(parser: &mut crate::Parser, context: &mut crate::Context) -> crate::ParseRes {
                parser.expect_as(Self::KIND)
            }
        }
    }
}

#[proc_macro]
pub fn include_path_code(input: TokenStream) -> TokenStream {
    // 1) Parse input tokens as a string literal: "some/path"
    let lit = parse_macro_input!(input as LitStr);
    let rel_path = lit.value();

    // 2) Resolve path relative to the crate that *invokes* the macro.
    // CARGO_MANIFEST_DIR here refers to the *current compilation unit*;
    // in a proc-macro, std::env::var points to the caller crate's env at expansion time.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let full_path: PathBuf = PathBuf::from(manifest_dir).join(&rel_path);

    // 3) Do “some parsing” / processing. Here: read file contents at compile time.
    let contents = std::fs::read_to_string(&full_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", full_path.display()));

    let config = match crate::parser::parse(&contents) {
        Ok(r) => r,
        Err(es) => {
            let rendered = render_ariadne_reports(&full_path.display().to_string(), &contents, &es);

            // Make the compiler show the pretty report.
            // (Yes: compile_error! is the usual stable approach for proc-macros.)
            return quote! {
                compile_error!(#rendered);
            }
            .into();
        }
    };

    let mut terminals = HashSet::new();
    let definitions: Vec<_> = config
        .rules
        .producing
        .iter()
        .map(|value| producing_trait_impl(value, &config, &mut terminals))
        .collect();

    let terminal_definitions: Vec<_> = terminals
        .iter()
        .flat_map(|x| x.ident(&config))
        .map(|x| terminal_trait_impl(x, &config))
        .collect();

    let mut producing: Vec<_> = config.rules.producing.iter().map(|x| &x.name).collect();
    producing.sort();

    let mut terminals: Vec<_> = terminals.iter().flat_map(|x| x.ident(&config)).collect();
    terminals.sort();

    let producing: Vec<_> = producing
        .into_iter()
        .map(|x| config.context.ident_for(x))
        .collect();
    let terminals: Vec<_> = terminals
        .into_iter()
        .map(|x| config.context.ident_for(x))
        .collect();

    let enum_definition = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(u16)]
        pub enum SyntaxKind {
            Eof = 0,
            WhiteSpace,
            Comment,
            /// producings
            #( #producing ,)*
            /// terminals
            #( #terminals ,)*
            Error,
            ROOT,    // top-level node: a list of s-expressions
        }
        pub use SyntaxKind::*;
    };

    let out = quote! {
    #enum_definition

    #(
        #definitions
    )*

    #(
        #terminal_definitions
    )*
        };

    println!("{}", out);
    out.into()
}
