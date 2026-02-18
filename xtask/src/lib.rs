use ariadne::{Report, ReportKind, Source};
use chumsky::container::Seq;
use chumsky::error::Rich;
use proc_macro::TokenStream;
use proc_macro2::token_stream;
use quote::quote;
use std::collections::{HashMap, HashSet};
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
                let func = |parser: &mut crate::Parser| {
                    #imp
                };

                parser.option(func);
            }
        }
        Expr::Marked(expr, Mark::Star) => {
            let imp = to_impl(expr, ctx, terminals);
            quote! {
                let func = |parser: &mut crate::Parser| {
                    #imp
                };

                parser.star(func);
            }
        }
        Expr::Marked(expr, Mark::Plus) => {
            let imp = to_impl(expr, ctx, terminals);
            quote! {
                let func = |parser: &mut crate::Parser| {
                    #imp
                };

                parser.plus(func);
            }
        }
        Expr::Either(exprs) => {
            let parts: Vec<_> = exprs.iter().map(|e| to_impl(e, ctx, terminals)).collect();
            quote! {
                let mut checker = crate::Checker::new(parser);

                #(
                    { #parts };
                    checker.update(parser);
                )*

                *parser = checker.get();
            }
        }
        Expr::Seq(exprs) => {
            let parts = exprs.iter().map(|e| to_impl(e, ctx, terminals));
            quote! {
                #(
                    {#parts};
                )*
            }
        }
        Expr::Literal(_literal_type, f) => {
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
    can_be_empty: &HashMap<String, bool>,
) -> token_stream::TokenStream {
    let n = ctx.context.ident_for(&rule.name);
    let imp = to_impl(&rule.expression, ctx, terminals);
    let can_be_emtpy = *can_be_empty.get(&rule.name).unwrap();

    quote! {
        #[derive(Debug)]
        pub struct #n;

        impl crate::ParserTrait for #n {
            const KIND: SyntaxKind = SyntaxKind::#n;
            const CAN_BE_EMPTY: bool = #can_be_emtpy;

            fn parse(parser: &mut crate::Parser, context: &mut crate::Context)  {
                let mut func = |parser: &mut crate::Parser| {
                    #imp
                };

                parser.producing_rule::<Self>(func);
            }
        }
    }
}

fn terminal_trait_impl(terminal: &str, ctx: &Config) -> token_stream::TokenStream {
    let n = ctx.context.ident_for(&terminal);
    let error = ctx.context.error_values.get(terminal).copied().unwrap_or(1);
    quote! {
        #[derive(Debug)]
        pub struct #n;

        impl crate::ParserTrait for #n {
            const KIND: SyntaxKind = SyntaxKind::#n;
            const CAN_BE_EMPTY: bool = false;

            fn parse(parser: &mut crate::Parser, context: &mut crate::Context) {
                if parser.res.error_value > 10 {
                    return;
                }

                parser.expect_as::<Self>(#error)
            }
        }
    }
}

fn set_can_be_empty(name: &str, ctx: &Config, map: &mut HashMap<String, bool>) -> bool {
    if let Some(r) = map.get(name) {
        return *r;
    }

    let result = if let Some(rule) = ctx.rules.producing.iter().find(|x| &x.name == name) {
        rule_can_be_empty(&rule.expression, ctx, map)
    } else {
        false
    };
    // This is a terminal
    map.insert(name.to_string(), result);
    result
}

fn rule_can_be_empty(expr: &Expr, ctx: &Config, map: &mut HashMap<String, bool>) -> bool {
    match expr {
        Expr::Marked(_, Mark::Star) => true,
        Expr::Marked(_, Mark::Option) => true,
        Expr::Marked(expr, Mark::Plus) => rule_can_be_empty(expr, ctx, map),
        Expr::Either(exprs) => exprs.iter().any(|e| rule_can_be_empty(e, ctx, map)),
        Expr::Seq(exprs) => exprs.iter().all(|e| rule_can_be_empty(e, ctx, map)),
        Expr::Literal(_, _) => false,
        Expr::Reference(s) => set_can_be_empty(&s, ctx, map),
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

    let mut can_be_empty = HashMap::new();

    for r in config.rules.producing.iter() {
        set_can_be_empty(&r.name, &config, &mut can_be_empty);
    }

    println!("Can Be Empty {:?}", can_be_empty);

    let mut terminals = HashSet::new();
    let definitions: Vec<_> = config
        .rules
        .producing
        .iter()
        .map(|value| producing_trait_impl(value, &config, &mut terminals, &can_be_empty))
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

    out.into()
}
