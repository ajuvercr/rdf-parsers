use ariadne::{Report, ReportKind, Source};
use chumsky::container::Seq;
use chumsky::error::Rich;
use proc_macro::TokenStream;
use proc_macro2::{Span, token_stream};
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Range;
use std::{io::Cursor, path::PathBuf};
use syn::{LitStr, parse_macro_input};

use crate::parser::{Config, Expr, Mark, Rule};
use crate::regex::{order_rules_by_references, to_regex};

mod parser;
mod regex;

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd)]
enum Terminal {
    Literal(String),
    Ref(String),
}
impl Terminal {
    fn ident<'a, 'b: 'a>(&'a self, config: &'b Config) -> String {
        match self {
            Terminal::Ref(x) => x.to_string(),
            Terminal::Literal(x) => config.context.with(x),
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
                let func = |parser: &mut crate::Parser<SyntaxKind>| {
                    #imp
                };

                parser.option(func);
            }
        }
        Expr::Marked(expr, Mark::Star) => {
            let imp = to_impl(expr, ctx, terminals);
            quote! {
                let func = |parser: &mut crate::Parser<SyntaxKind>| {
                    #imp
                };

                parser.star(func);
            }
        }
        Expr::Marked(expr, Mark::Plus) => {
            let imp = to_impl(expr, ctx, terminals);
            quote! {
                let func = |parser: &mut crate::Parser<SyntaxKind>| {
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
            let name = ctx.context.with(f);
            let n = ctx.context.ident_for(&name);
            quote! {
                #n::parse(parser, context)
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

fn add_impl(
    expr: &Expr,
    ctx: &Config,
    terminals: &mut HashSet<Terminal>,
    state_count: &mut usize,
    next: usize,
    impls: &mut HashMap<usize, token_stream::TokenStream>,
) -> usize {
    fn as_tt(
        impls: &mut HashMap<usize, proc_macro2::TokenStream>,
        x: usize,
    ) -> proc_macro2::TokenStream {
        if let Some(tt) = impls.get(&x) {
            quote! {
                #tt
            }
        } else {
            quote! {
                state.add_element(element.pop_push(Self { state: #x  }));
            }
        }
    }
    let id = *state_count;
    let comment = format!(" {:?}", expr);
    let exp = match expr {
        Expr::Marked(expr, Mark::Option) => {
            *state_count += 1;
            let thing = add_impl(expr, ctx, terminals, state_count, next, impls);
            let tt = as_tt(impls, thing);
            let next = as_tt(impls, next);

            let once = format!(" Execute ({:?}) at once", expr);
            let never = format!(" Never execute ({:?})", expr);
            quote! {
                #[doc = #comment]
                #[doc = #never]
                #next
                #[doc = #once]
                #tt
            }
        }
        Expr::Marked(expr, Mark::Plus) => {
            *state_count += 1;
            let done_once = *state_count;
            *state_count += 1;
            let thing = add_impl(expr, ctx, terminals, state_count, done_once, impls);
            let tt = as_tt(impls, thing);

            let next = as_tt(impls, next);

            let once_more = format!(" Execute ({:?}) once more", expr);
            let at_least_once = format!(" Execute ({:?}) at least once", expr);

            impls.insert(
                done_once,
                quote! {
                    #[doc = #comment]
                    #[doc = #once_more]
                    #tt
                    #[doc = " Break, goto next"]
                    #next
                },
            );

            quote! {
                #[doc = #comment]
                #[doc = #at_least_once]
                #tt
            }
        }
        Expr::Marked(expr, Mark::Star) => {
            *state_count += 1;
            let thing = add_impl(expr, ctx, terminals, state_count, id, impls);
            let tt = as_tt(impls, thing);
            let next = as_tt(impls, next);
            let once_more = format!(" Execute ({:?}) once more", expr);
            quote! {
                #[doc = #comment]
                #[doc = #once_more]
                #tt
                #[doc = " Break, goto next"]
                #next
            }
        }
        Expr::Either(exprs) => {
            *state_count += 1;
            let things: Vec<_> = exprs
                .iter()
                .map(|e| add_impl(e, ctx, terminals, state_count, next, impls))
                .collect();

            let things = things.into_iter().map(|x| as_tt(impls, x));

            quote! {
                #[doc = #comment]
                #( #things )*
            }
        }
        Expr::Seq(exprs) => {
            let mut target = next;
            for e in exprs.iter().rev() {
                target = add_impl(e, ctx, terminals, state_count, target, impls);
            }

            return target;
        }
        Expr::Literal(_literal_type, f) => {
            *state_count += 1;
            terminals.insert(Terminal::Literal(f.clone()));
            let name = ctx.context.with(f);
            let n = ctx.context.ident_for(&name);

            quote! {
                #[doc = #comment]
                state.add_element(
                    element.pop_push(Self { state: #next }).push(#n::new())
                );
            }
        }
        Expr::Reference(re) => {
            *state_count += 1;
            if !ctx.rules.producing.iter().any(|r| &r.name == re) {
                terminals.insert(Terminal::Ref(re.clone()));
            }

            let n = ctx.context.ident_for(re);

            quote! {
                #[doc = #comment]
                state.add_element(
                    element.pop_push(Self { state: #next }).push(#n::new())
                );
            }
        }
    };

    impls.insert(id, exp);

    id
}

fn producing_trait_impl(
    rule: &Rule,
    ctx: &Config,
    terminals: &mut HashSet<Terminal>,
) -> token_stream::TokenStream {
    let n = ctx.context.ident_for(&rule.name);
    let mut state_count = 1;
    let mut impls = HashMap::new();
    // This implementation should be something like
    // If no tokens were consumed, and I shouldn't be empty
    // update steps with expected me
    impls.insert(
        0,
        quote! {
            if let Some(parent) = element.pop() {
                state.add_element(parent);
            }
        },
    );
    let imp = add_impl(
        &rule.expression,
        ctx,
        terminals,
        &mut state_count,
        0,
        &mut impls,
    );

    let impls = impls.into_iter().map(|(k, v)| quote! { #k => { #v } });

    // const FIRST_ITEMS: &'static [SyntaxKind] = &[ #( #fi, )* ];
    // const LAST_ITEMS: &'static [SyntaxKind] = &[ #( #li, )* ];
    quote! {
        #[derive(Debug)]
        pub struct #n {
            state: usize
        }

        impl crate::a_star::ParserTrait for #n {
            type Kind = SyntaxKind;

            fn step(&self, element: &crate::a_star::Element<Self::Kind>, state: &mut crate::a_star::AStar<Self::Kind>)
            {
                match self.state {
                    #( #impls )*
                    _ => panic!("Aaaah unexpected state"),
                }
            }

            fn at(&self) -> usize {
                self.state
            }
        }
        impl crate::a_star::ParserTraitConsts for #n {
            const ELEMENT: SyntaxKind = SyntaxKind::#n;

            fn new() -> Self {
                Self {
                    state: #imp
                }
            }
        }
    }
}

fn terminal_trait_impl(terminal: &str, is_kw: bool, ctx: &Config) -> token_stream::TokenStream {
    let n = ctx.context.ident_for(&terminal);
    let defa = if is_kw { 10 } else { 2 };
    let error = ctx
        .context
        .error_values
        .get(terminal)
        .copied()
        .unwrap_or(defa);
    quote! {
        #[derive(Debug)]
        pub struct #n;

        impl crate::a_star::ParserTrait for #n {
            type Kind = SyntaxKind;

            /// terminal
            fn step(&self, element: &crate::a_star::Element<Self::Kind>, state: &mut crate::a_star::AStar<Self::Kind>)
            {
                let added = state.expect_as(element, SyntaxKind::#n, #error);
                if let Some(parent) = added.pop() {
                    state.add_element(parent);
                } else {
                    println!("Failed to add parent {:?}", self);
                }
            }
            fn at(&self) -> usize {
                0
            }
        }


        impl crate::a_star::ParserTraitConsts for #n {
            const ELEMENT: SyntaxKind = SyntaxKind::#n;

            fn new() -> Self {
                Self
            }
        }
    }
}

#[derive(Hash, PartialEq, PartialOrd, Debug, Clone, Eq)]
enum Item {
    Terminal(String),
    NonTerminal(String),
}
impl Item {
    fn terminal(s: impl Into<String>) -> Self {
        Self::Terminal(s.into())
    }

    fn non_terminal(s: impl Into<String>) -> Self {
        Self::NonTerminal(s.into())
    }
}

fn set_first_possible_item(
    name: &str,
    ctx: &Config,
    map: &mut HashMap<String, HashSet<Item>>,
    can_be_empty: &mut HashMap<String, bool>,
) {
    if map.contains_key(name) {
        return;
    }

    let mut set = HashSet::new();
    if let Some(rule) = ctx.rules.producing.iter().find(|x| &x.name == name) {
        expr_first_items(&rule.expression, ctx, map, &mut set, can_be_empty);
    }
    map.insert(name.to_string(), set);
}

fn set_last_possible_item(
    name: &str,
    ctx: &Config,
    map: &mut HashMap<String, HashSet<Item>>,
    can_be_empty: &mut HashMap<String, bool>,
    at: usize,
) {
    if map.contains_key(name) {
        return;
    }

    let mut set = HashSet::new();
    if let Some(rule) = ctx.rules.producing.iter().find(|x| &x.name == name) {
        expr_last_items(&rule.expression, ctx, map, &mut set, can_be_empty, at + 1);
    }
    map.insert(name.to_string(), set);
}

fn expr_first_items(
    expr: &Expr,
    ctx: &Config,
    map: &mut HashMap<String, HashSet<Item>>,
    set: &mut HashSet<Item>,
    can_be_empty: &mut HashMap<String, bool>,
) {
    match expr {
        Expr::Marked(e, _) => {
            expr_first_items(e, ctx, map, set, can_be_empty);
        }
        Expr::Either(exprs) => {
            for e in exprs {
                expr_first_items(e, ctx, map, set, can_be_empty);
            }
        }
        Expr::Seq(exprs) => {
            for e in exprs {
                expr_first_items(e, ctx, map, set, can_be_empty);
                if !rule_can_be_empty(e, ctx, can_be_empty) {
                    break;
                }
            }
        }
        Expr::Literal(_literal_type, s) => {
            set.insert(Item::terminal(ctx.context.with(s)));
        }
        Expr::Reference(s) => {
            set.insert(Item::non_terminal(s));
            set_first_possible_item(&s, ctx, map, can_be_empty);
            if let Some(new_set) = map.get(s) {
                set.extend(new_set.iter().cloned());
            } else {
                eprintln!("Hmm this should be present");
            }
        }
    }
}

fn expr_last_items(
    expr: &Expr,
    ctx: &Config,
    map: &mut HashMap<String, HashSet<Item>>,
    set: &mut HashSet<Item>,
    can_be_empty: &mut HashMap<String, bool>,
    done: usize,
) {
    if done > 100 {
        return;
    }
    match expr {
        Expr::Marked(e, _) => {
            expr_last_items(e, ctx, map, set, can_be_empty, done + 1);
        }
        Expr::Either(exprs) => {
            for e in exprs {
                expr_last_items(e, ctx, map, set, can_be_empty, done + 1);
                if !rule_can_be_empty(e, ctx, can_be_empty) {
                    break;
                }
            }
        }
        Expr::Seq(exprs) => {
            for e in exprs.iter().rev() {
                expr_last_items(e, ctx, map, set, can_be_empty, done + 1);
                if !rule_can_be_empty(e, ctx, can_be_empty) {
                    break;
                }
            }
        }
        Expr::Literal(_literal_type, s) => {
            set.insert(Item::terminal(ctx.context.with(s)));
        }
        Expr::Reference(s) => {
            set.insert(Item::non_terminal(s));
            set_last_possible_item(&s, ctx, map, can_be_empty, done + 1);
            if let Some(new_set) = map.get(s) {
                set.extend(new_set.iter().cloned());
            } else {
                eprintln!("Hmm this should be present");
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
    let mut first_items = HashMap::new();
    let mut last_items = HashMap::new();

    for r in config.rules.producing.iter() {
        set_can_be_empty(&r.name, &config, &mut can_be_empty);
    }

    for r in config.rules.producing.iter() {
        set_first_possible_item(&r.name, &config, &mut first_items, &mut can_be_empty);
        set_last_possible_item(&r.name, &config, &mut last_items, &mut can_be_empty, 0);
    }

    let mut terminals = HashSet::new();

    let definitions: Vec<_> = config
        .rules
        .producing
        .iter()
        .map(|value| producing_trait_impl(value, &config, &mut terminals))
        .collect();

    let sub_pattersn = get_sub_patterns(&config.rules.terminals);

    let terminal_definitions: Vec<_> = terminals
        .iter()
        .map(|x| {
            let ident = x.ident(&config);
            let is_kw = match x {
                Terminal::Literal(_) => true,
                Terminal::Ref(_) => false,
            };
            terminal_trait_impl(&ident, is_kw, &config)
        })
        .collect();

    let mut producing: Vec<_> = config.rules.producing.iter().map(|x| &x.name).collect();
    producing.sort();

    let terminals: Vec<_> = terminals
        .iter()
        .flat_map(|x| match x {
            Terminal::Literal(x) => {
                let name = config.context.with(x);
                let ident = config.context.ident_for(&name);

                quote! {
                    #[token(#x)]
                    #ident,
                }
            }
            Terminal::Ref(n) => {
                let regex = format!("(?&{})", n);
                let ident = config.context.ident_for(n);
                quote! {
                    #[regex(#regex)]
                    #ident,
                }
            }
        })
        .collect();

    let producing: Vec<_> = producing
        .into_iter()
        .map(|x| config.context.ident_for(x))
        .collect();

    let enum_definition = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, logos::Logos)]
        #(#sub_pattersn)*
        #[repr(u16)]
        pub enum SyntaxKind {
            Eof = 0,
            #[regex(r"[ \t\n]+")]
            WhiteSpace,
            #[regex(r"\\#[^\n]+", allow_greedy=true)]
            Comment,
            /// producings
            #( #producing ,)*
            /// terminals
            #( #terminals )*
            Error,
            ROOT,    // top-level node: a list of s-expressions
        }
    };

    let out = quote! {

    use crate::TokenTrait;
    pub type SyntaxNode = rowan::SyntaxNode<Lang>;

    #enum_definition

    impl From<SyntaxKind> for rowan::SyntaxKind {
        fn from(kind: SyntaxKind) -> Self {
            Self(kind as u16)
        }
    }

    impl From<rowan::SyntaxKind> for SyntaxKind {
        fn from(value: rowan::SyntaxKind) -> Self {
            assert!(value.0 <= SyntaxKind::ROOT as u16);
            unsafe { std::mem::transmute::<u16, SyntaxKind>(value.0) }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Lang {}
    impl rowan::Language for Lang {
        type Kind = SyntaxKind;
        fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
            assert!(raw.0 <= SyntaxKind::ROOT as u16);
            unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
        }
        fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
            kind.into()
        }
    }

    mod definitions {
        use crate::a_star::ParserTraitConsts as _;
        use super::*;
        #(
            #definitions
        )*

        #(
            #terminal_definitions
        )*
    }
    pub use definitions::*;

    impl TokenTrait for SyntaxKind {
        const ERROR: Self = SyntaxKind::Error;
        const ROOT: Self = SyntaxKind::ROOT;

        fn branch(&self) -> u32 {
            *self as u32
        }

        fn skips(&self) -> bool {
            match self {
                SyntaxKind::WhiteSpace => true,
                SyntaxKind::Error => true,
                SyntaxKind::Comment => true,
                _ => false,
            }
        }

        fn starting_tokens(&self) -> &'static [SyntaxKind] {
            use crate::ParserTrait as _;
            match self {
                _ => &[],
            }
        }

        fn ending_tokens(&self) -> &'static [SyntaxKind] {
            use crate::ParserTrait as _;
            match self {
                _ => &[],
            }
        }
    }


        };

    out.into()
}

fn get_sub_patterns(rules: &[Rule]) -> Vec<proc_macro2::TokenStream> {
    let ordered_rules = order_rules_by_references(rules).unwrap();
    let sub_pattersn: Vec<_> = ordered_rules
        .iter()
        .map(|x| {
            let pattern = to_regex(&x.expression);
            let ident = syn::Ident::new(&x.name, Span::call_site());

            let pat_lit: LitStr = match syn::parse_str(&format!("r#\"{pattern}\"#")) {
                Ok(x) => x,
                Err(e) => {
                    let error = e.to_string();
                    eprintln!("error {} {} {} {:?}", x.name, error, pattern, pattern);
                    return quote! {
                        compiler_error!(#error);
                    };
                }
            };

            quote!(
                #[logos(subpattern #ident = #pat_lit)]
            )
        })
        .collect();
    sub_pattersn
}
