use ariadne::{Report, ReportKind, Source};
use chumsky::error::Rich;
use proc_macro2::{Span, token_stream};
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Range;
use std::io::Cursor;
use syn::LitStr;

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

fn push_rule(
    next: usize,
    n: &proc_macro2::Ident,
    initial_state: usize,
) -> token_stream::TokenStream {
    quote! {
        state.add_element(
            element.pop_push(Rule { kind: self.kind, state: #next })
                .push(Rule { kind: SyntaxKind::#n, state: #initial_state })
        );
    }
}

/// Mirrors the state-counter allocation in `add_impl` without generating code,
/// returning the entry state for the given expression.
fn compute_entry_state(expr: &Expr, state_count: &mut usize, next: usize) -> usize {
    let id = *state_count;
    match expr {
        Expr::Marked(inner, Mark::Option) => {
            *state_count += 1;
            compute_entry_state(inner, state_count, next);
            id
        }
        Expr::Marked(inner, Mark::Plus) => {
            *state_count += 1;
            let done_once = *state_count;
            *state_count += 1;
            compute_entry_state(inner, state_count, done_once);
            id
        }
        Expr::Marked(inner, Mark::Star) => {
            *state_count += 1;
            compute_entry_state(inner, state_count, id);
            id
        }
        Expr::Either(exprs) => {
            *state_count += 1;
            for e in exprs {
                compute_entry_state(e, state_count, next);
            }
            id
        }
        Expr::Seq(exprs) => {
            let mut target = next;
            for e in exprs.iter().rev() {
                target = compute_entry_state(e, state_count, target);
            }
            target
        }
        Expr::Literal(_, _) | Expr::Reference(_) => {
            *state_count += 1;
            id
        }
    }
}

fn add_impl(
    expr: &Expr,
    ctx: &Config,
    terminals: &mut HashSet<Terminal>,
    state_count: &mut usize,
    next: usize,
    impls: &mut HashMap<usize, token_stream::TokenStream>,
    reachable: &mut HashSet<usize>,
    initial_states: &HashMap<String, usize>,
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
                state.add_element(element.pop_push(Rule { kind: self.kind, state: #x }));
            }
        }
    }
    let id = *state_count;
    let exp = match expr {
        Expr::Marked(expr, Mark::Option) => {
            *state_count += 1;
            let thing = add_impl(expr, ctx, terminals, state_count, next, impls, reachable, initial_states);
            let tt = as_tt(impls, thing);
            let next = as_tt(impls, next);

            quote! {
                #next
                #tt
            }
        }
        Expr::Marked(expr, Mark::Plus) => {
            *state_count += 1;
            let done_once = *state_count;
            *state_count += 1;
            let thing = add_impl(expr, ctx, terminals, state_count, done_once, impls, reachable, initial_states);
            let tt = as_tt(impls, thing);

            let next = as_tt(impls, next);

            impls.insert(
                done_once,
                quote! {
                    #tt
                    #next
                },
            );

            quote! {
                #tt
            }
        }
        Expr::Marked(expr, Mark::Star) => {
            *state_count += 1;
            let thing = add_impl(expr, ctx, terminals, state_count, id, impls, reachable, initial_states);
            let tt = as_tt(impls, thing);
            let next = as_tt(impls, next);
            quote! {
                #tt
                #next
            }
        }
        Expr::Either(exprs) => {
            *state_count += 1;
            let things: Vec<_> = exprs
                .iter()
                .map(|e| add_impl(e, ctx, terminals, state_count, next, impls, reachable, initial_states))
                .collect();

            let things = things.into_iter().map(|x| as_tt(impls, x));

            quote! {
                #( #things )*
            }
        }
        Expr::Seq(exprs) => {
            let mut target = next;
            for e in exprs.iter().rev() {
                target = add_impl(e, ctx, terminals, state_count, target, impls, reachable, initial_states);
            }

            return target;
        }
        Expr::Literal(_literal_type, f) => {
            *state_count += 1;
            terminals.insert(Terminal::Literal(f.clone()));
            let name = ctx.context.with(f);
            let n = ctx.context.ident_for(&name);
            reachable.insert(next);
            push_rule(next, &n, 0) // literals are always terminals; initial state is 0
        }
        Expr::Reference(re) => {
            *state_count += 1;
            let is_terminal = !ctx.rules.producing.iter().any(|r| &r.name == re);
            if is_terminal {
                terminals.insert(Terminal::Ref(re.clone()));
            }
            let n = ctx.context.ident_for(re);
            let entry = if is_terminal {
                0 // terminals always start at state 0
            } else {
                *initial_states.get(re).unwrap_or_else(|| panic!("unknown rule: {re}"))
            };
            reachable.insert(next);
            push_rule(next, &n, entry)
        }
    };

    impls.insert(id, exp);

    id
}

fn producing_rule_arms(
    rule: &Rule,
    ctx: &Config,
    terminals: &mut HashSet<Terminal>,
    initial_states: &HashMap<String, usize>,
) -> (token_stream::TokenStream, token_stream::TokenStream) {
    let n = ctx.context.ident_for(&rule.name);
    let mut state_count = 1;
    let mut impls = HashMap::new();
    impls.insert(
        0,
        quote! {
            if let Some(parent) = element.pop() {
                state.add_element(parent);
            }
        },
    );
    let mut reachable = HashSet::new();
    let imp = add_impl(
        &rule.expression,
        ctx,
        terminals,
        &mut state_count,
        0,
        &mut impls,
        &mut reachable,
        initial_states,
    );
    reachable.insert(imp); // entry state used by Rule::new

    let step_arms = impls
        .into_iter()
        .filter(|(k, _)| reachable.contains(k))
        .map(|(k, v)| quote! { (SyntaxKind::#n, #k) => { #v } });

    let new_arm = quote! { SyntaxKind::#n => Rule { kind, state: #imp }, };

    (quote! { #( #step_arms )* }, new_arm)
}

fn terminal_rule_arms(
    terminal: &str,
    is_kw: bool,
    ctx: &Config,
) -> (token_stream::TokenStream, token_stream::TokenStream) {
    let n = ctx.context.ident_for(terminal);
    let defa = if is_kw { 10 } else { 2 };
    let error = ctx
        .context
        .error_values
        .get(terminal)
        .copied()
        .unwrap_or(defa);

    let step_arm = quote! {
        (SyntaxKind::#n, _) => {
            let added = state.expect_as(element, SyntaxKind::#n, #error);
            if let Some(parent) = added.pop() {
                state.add_element(parent);
            }
        }
    };

    let new_arm = quote! { SyntaxKind::#n => Rule { kind, state: 0 }, };

    (step_arm, new_arm)
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

/// Generate Rust source code from a grammar file.
/// `path` is used only for error reporting; `contents` is the grammar text.
pub fn generate(path: &str, contents: &str) -> String {
    let config = match crate::parser::parse(contents) {
        Ok(r) => r,
        Err(es) => {
            let rendered = render_ariadne_reports(path, contents, &es);
            panic!("Grammar parse error in {}:\n{}", path, rendered);
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

    // Pre-compute the entry state for each producing rule so that push_rule can
    // emit `Rule { kind: X, state: N }` directly instead of `Rule::new(X)`.
    let initial_states: HashMap<String, usize> = config
        .rules
        .producing
        .iter()
        .map(|rule| {
            let entry = compute_entry_state(&rule.expression, &mut 1usize, 0);
            (rule.name.clone(), entry)
        })
        .collect();

    let mut terminals = HashSet::new();

    let (producing_step_arms, producing_new_arms): (Vec<_>, Vec<_>) = config
        .rules
        .producing
        .iter()
        .map(|value| producing_rule_arms(value, &config, &mut terminals, &initial_states))
        .unzip();

    let sub_pattersn = get_sub_patterns(&config.rules.terminals);

    let (terminal_step_arms, terminal_new_arms): (Vec<_>, Vec<_>) = terminals
        .iter()
        .map(|x| {
            let ident = x.ident(&config);
            let is_kw = match x {
                Terminal::Literal(_) => true,
                Terminal::Ref(_) => false,
            };
            terminal_rule_arms(&ident, is_kw, &config)
        })
        .unzip();

    let first_token_producing_arms: Vec<_> = first_items
        .iter()
        .filter(|(name, _)| config.rules.producing.iter().any(|r| &r.name == *name))
        .map(|(name, items)| {
            let n = config.context.ident_for(name);
            let toks: Vec<_> = items
                .iter()
                .filter_map(|item| match item {
                    Item::Terminal(t) => Some(config.context.ident_for(t)),
                    Item::NonTerminal(_) => None,
                })
                .collect();
            quote! { SyntaxKind::#n => &[#( SyntaxKind::#toks ),*], }
        })
        .collect();

    let first_token_terminal_arms: Vec<_> = terminals
        .iter()
        .map(|x| {
            let name = x.ident(&config);
            let n = config.context.ident_for(&name);
            quote! { SyntaxKind::#n => &[SyntaxKind::#n], }
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
        use super::*;

        #[derive(Debug, Clone, Copy)]
        pub struct Rule {
            pub kind: SyntaxKind,
            pub state: usize,
        }

        impl Rule {
            pub fn new(kind: SyntaxKind) -> Self {
                match kind {
                    #( #producing_new_arms )*
                    #( #terminal_new_arms )*
                    _ => panic!("Unknown rule kind {:?}", kind),
                }
            }
        }

        pub fn first_tokens(kind: SyntaxKind) -> &'static [SyntaxKind] {
            match kind {
                #( #first_token_producing_arms )*
                #( #first_token_terminal_arms )*
                _ => &[],
            }
        }

        impl crate::a_star::ParserTrait for Rule {
            type Kind = SyntaxKind;

            fn step(
                &self,
                element: &crate::a_star::Element<Self>,
                state: &mut crate::a_star::AStar<Self>,
            ) {
                match (self.kind, self.state) {
                    #( #producing_step_arms )*
                    #( #terminal_step_arms )*
                    _ => panic!("Aaaah unexpected state {:?}", self),
                }
            }

            fn at(&self) -> usize {
                self.state
            }

            fn element_kind(&self) -> SyntaxKind {
                self.kind
            }
        }
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
            &[]
        }

        fn ending_tokens(&self) -> &'static [SyntaxKind] {
            &[]
        }
    }


        };

    out.to_string()
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
