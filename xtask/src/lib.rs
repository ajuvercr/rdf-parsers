use ariadne::{Report, ReportKind, Source};
use chumsky::error::Rich;
use proc_macro2::{Span, token_stream};
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::Cursor;
use std::ops::Range;
use syn::LitStr;

use crate::parser::{Config, Expr, LiteralType, Mark, Rule};
use crate::regex::{order_rules_by_references, to_regex};

mod parser;
mod regex;

/// Default `error_value` for all tokens (terminal references and keyword literals).
///
/// Must be > 0: the A* cost model requires `error_value > 0` so that matching
/// a token is strictly cheaper than skipping it.  Tokens that need different
/// recovery behaviour should be assigned explicit weights in the grammar's
/// `=== error_value ===` section.
const DEFAULT_TOKEN_WEIGHT: isize = 1;

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum IgnoreCase {
    True,
    False,
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Terminal {
    Literal(String, IgnoreCase),
    Ref(String),
}
impl Terminal {
    fn ident<'a, 'b: 'a>(&'a self, config: &'b Config) -> String {
        match self {
            Terminal::Ref(x) => x.to_string(),
            Terminal::Literal(x, _) => config.context.with(x),
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

// ── Expression compaction ─────────────────────────────────────────────────────

/// Build a Seq, flattening any nested Seqs and collapsing single-element results.
fn make_seq(parts: Vec<Expr>) -> Expr {
    let flat: Vec<Expr> = parts
        .into_iter()
        .flat_map(|e| if let Expr::Seq(p) = e { p } else { vec![e] })
        .collect();
    match flat.len() {
        0 => panic!("make_seq: empty"),
        1 => flat.into_iter().next().unwrap(),
        _ => Expr::Seq(flat),
    }
}

/// Build an Either, flattening nested Eithers and collapsing single-element results.
fn make_either(alts: Vec<Expr>) -> Expr {
    let flat: Vec<Expr> = alts
        .into_iter()
        .flat_map(|e| if let Expr::Either(a) = e { a } else { vec![e] })
        .collect();
    match flat.len() {
        0 => panic!("make_either: empty"),
        1 => flat.into_iter().next().unwrap(),
        _ => Expr::Either(flat),
    }
}

/// Unwrap a Seq into its parts, or wrap a non-Seq in a single-element Vec.
fn seq_parts(e: Expr) -> Vec<Expr> {
    match e {
        Expr::Seq(p) => p,
        other => vec![other],
    }
}

/// Compact an expression by recursively factoring common prefixes and suffixes
/// out of `Either` alternatives.
pub fn compact(expr: Expr) -> Expr {
    match expr {
        Expr::Marked(inner, mark) => Expr::Marked(Box::new(compact(*inner)), mark),
        Expr::Seq(parts) => make_seq(parts.into_iter().map(compact).collect()),
        Expr::Either(alts) => {
            // Compact children first, then flatten nested Eithers.
            let alts: Vec<Expr> = alts
                .into_iter()
                .map(compact)
                .flat_map(|e| if let Expr::Either(a) = e { a } else { vec![e] })
                .collect();
            if alts.len() == 1 {
                return alts.into_iter().next().unwrap();
            }
            // Normalise each alternative to a flat sequence and factor.
            let seqs: Vec<Vec<Expr>> = alts
                .into_iter()
                .map(|e| if let Expr::Seq(p) = e { p } else { vec![e] })
                .collect();
            factor_seqs(seqs)
        }
        other => other,
    }
}

/// Given a list of sequences that are alternatives of an Either, apply
/// suffix-factoring then prefix-factoring to reduce duplication.
fn factor_seqs(seqs: Vec<Vec<Expr>>) -> Expr {
    if seqs.len() == 1 {
        return make_seq(seqs.into_iter().next().unwrap());
    }

    // ── Suffix factoring: group by last element ───────────────────────────────
    // Only factor a group when *all* of its prefixes are non-empty; otherwise
    // a single-token alternative like [A] would produce an unrepresentable ε
    // prefix after removing A.
    let mut suffix_groups: Vec<(Expr, Vec<Vec<Expr>>)> = Vec::new();
    for seq in &seqs {
        if seq.is_empty() {
            // ε alternative — skip suffix factoring entirely for this call.
            return make_either(seqs.into_iter().map(make_seq).collect());
        }
        let last = seq.last().unwrap().clone();
        let prefix = seq[..seq.len() - 1].to_vec();
        match suffix_groups.iter_mut().find(|(k, _)| k == &last) {
            Some(g) => g.1.push(prefix),
            None => suffix_groups.push((last, vec![prefix])),
        }
    }
    let can_suffix = suffix_groups
        .iter()
        .any(|(_, ps)| ps.len() > 1 && ps.iter().all(|p| !p.is_empty()));
    if can_suffix {
        let parts: Vec<Expr> = suffix_groups
            .into_iter()
            .flat_map(|(last, prefixes)| -> Vec<Expr> {
                if prefixes.len() > 1 && prefixes.iter().all(|p| !p.is_empty()) {
                    // Factor: compact the shared prefixes, then append the common last.
                    let inner = factor_seqs(prefixes);
                    let mut parts = seq_parts(inner);
                    parts.push(last);
                    vec![make_seq(parts)]
                } else {
                    // Cannot factor this group — restore the original sequences.
                    prefixes
                        .into_iter()
                        .map(|mut p| {
                            p.push(last.clone());
                            make_seq(p)
                        })
                        .collect()
                }
            })
            .collect();
        return make_either(parts);
    }

    // ── Prefix factoring: group by first element ──────────────────────────────
    let mut prefix_groups: Vec<(Expr, Vec<Vec<Expr>>)> = Vec::new();
    for seq in &seqs {
        let first = seq[0].clone();
        let tail = seq[1..].to_vec();
        match prefix_groups.iter_mut().find(|(k, _)| k == &first) {
            Some(g) => g.1.push(tail),
            None => prefix_groups.push((first, vec![tail])),
        }
    }
    let can_prefix = prefix_groups
        .iter()
        .any(|(_, ts)| ts.len() > 1 && ts.iter().all(|t| !t.is_empty()));
    if can_prefix {
        let parts: Vec<Expr> = prefix_groups
            .into_iter()
            .flat_map(|(first, tails)| -> Vec<Expr> {
                if tails.len() > 1 && tails.iter().all(|t| !t.is_empty()) {
                    let inner = factor_seqs(tails);
                    let mut parts = vec![first];
                    parts.extend(seq_parts(inner));
                    vec![make_seq(parts)]
                } else {
                    // Cannot factor this group — restore the original sequences.
                    tails
                        .into_iter()
                        .map(|t| {
                            let mut s = vec![first.clone()];
                            s.extend(t);
                            make_seq(s)
                        })
                        .collect()
                }
            })
            .collect();
        return make_either(parts);
    }

    // No factoring possible.
    make_either(seqs.into_iter().map(make_seq).collect())
}

// ─────────────────────────────────────────────────────────────────────────────

fn push_rule(
    next: usize,
    n: &proc_macro2::Ident,
    initial_state: usize,
) -> token_stream::TokenStream {
    quote! {
        state.add_element_checked(
            element.pop_push(Rule { kind: self.kind, state: #next })
                .push(Rule { kind: SyntaxKind::#n, state: #initial_state }),
            SyntaxKind::#n,
        );
    }
}

/// Generate an inlined terminal match: matches the token and wraps it in CST
/// Start/End nodes in one call, without pushing a separate rule onto the stack.
fn inline_terminal_rule(next: usize, n: &proc_macro2::Ident) -> token_stream::TokenStream {
    quote! {
        let (matched, fb) = state.expect_as_inline(element, SyntaxKind::#n);
        state.add_element(matched.pop_push(Rule { kind: self.kind, state: #next }));
        if let Some(fb) = fb {
            state.add_element(fb.pop_push(Rule { kind: self.kind, state: #next }));
        }
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
    body_to_state: &mut HashMap<String, usize>,
    reachable: &mut HashSet<usize>,
    initial_states: &HashMap<String, usize>,
) -> usize {
    // Never inline: always emit pop_push(state:x).  This keeps every state as
    // an explicit match arm so redirect states are always reachable.
    fn as_tt(reachable: &mut HashSet<usize>, x: usize) -> proc_macro2::TokenStream {
        reachable.insert(x);
        quote! {
            state.add_element(element.pop_push(Rule { kind: self.kind, state: #x }));
        }
    }
    let id = *state_count;
    let exp = match expr {
        Expr::Marked(expr, Mark::Option) => {
            *state_count += 1;
            let thing = add_impl(
                expr,
                ctx,
                terminals,
                state_count,
                next,
                impls,
                body_to_state,
                reachable,
                initial_states,
            );
            let tt = as_tt(reachable, thing);
            let next = as_tt(reachable, next);

            quote! {
                #next
                #tt
            }
        }
        Expr::Marked(expr, Mark::Plus) => {
            *state_count += 1;
            let done_once = *state_count;
            *state_count += 1;
            let thing = add_impl(
                expr,
                ctx,
                terminals,
                state_count,
                done_once,
                impls,
                body_to_state,
                reachable,
                initial_states,
            );
            let tt = as_tt(reachable, thing);

            let next = as_tt(reachable, next);

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
            let thing = add_impl(
                expr,
                ctx,
                terminals,
                state_count,
                id,
                impls,
                body_to_state,
                reachable,
                initial_states,
            );
            let tt = as_tt(reachable, thing);
            let next = as_tt(reachable, next);
            quote! {
                #tt
                #next
            }
        }
        Expr::Either(exprs) => {
            *state_count += 1;
            let things: Vec<_> = exprs
                .iter()
                .map(|e| {
                    add_impl(
                        e,
                        ctx,
                        terminals,
                        state_count,
                        next,
                        impls,
                        body_to_state,
                        reachable,
                        initial_states,
                    )
                })
                .collect();

            let things = things.into_iter().map(|x| as_tt(reachable, x));

            quote! {
                #( #things )*
            }
        }
        Expr::Seq(exprs) => {
            let mut target = next;
            for e in exprs.iter().rev() {
                target = add_impl(
                    e,
                    ctx,
                    terminals,
                    state_count,
                    target,
                    impls,
                    body_to_state,
                    reachable,
                    initial_states,
                );
            }

            return target;
        }
        // For leaf nodes (Literal, Reference) the generated body does NOT
        // reference the newly-allocated state ID.  If an identical body was
        // already emitted for a previous state, emit a one-line redirect arm
        // instead of duplicating the body.  This keeps state numbering in sync
        // with compute_entry_state while still letting the A* dedup converge
        // both paths at the canonical state on the next step.
        Expr::Literal(lt, f) => {
            let ignore_case = if lt == &LiteralType::Double {
                IgnoreCase::True
            } else {
                IgnoreCase::False
            };
            *state_count += 1;
            terminals.insert(Terminal::Literal(f.clone(), ignore_case));
            let name = ctx.context.with(f);
            let n = ctx.context.ident_for(&name);
            reachable.insert(next);
            let exp = inline_terminal_rule(next, &n);
            let key = exp.to_string();
            if let Some(&canonical) = body_to_state.get(&key) {
                // Redirect: pop_push to the canonical state so the A* dedup
                // collapses both paths at `canonical` on the next iteration.
                let redirect = quote! {
                    state.add_element(element.pop_push(Rule { kind: self.kind, state: #canonical }));
                };
                impls.insert(id, redirect);
            } else {
                body_to_state.insert(key, id);
                impls.insert(id, exp);
            }
            return id;
        }
        Expr::Reference(re) => {
            *state_count += 1;
            let is_terminal = !ctx.rules.producing.iter().any(|r| &r.name == re);
            if is_terminal {
                terminals.insert(Terminal::Ref(re.clone()));
            }
            let n = ctx.context.ident_for(re);
            reachable.insert(next);
            let exp = if is_terminal {
                inline_terminal_rule(next, &n)
            } else {
                let entry = *initial_states
                    .get(re)
                    .unwrap_or_else(|| panic!("unknown rule: {re}"));
                push_rule(next, &n, entry)
            };
            let key = exp.to_string();
            if let Some(&canonical) = body_to_state.get(&key) {
                let redirect = quote! {
                    state.add_element(element.pop_push(Rule { kind: self.kind, state: #canonical }));
                };
                impls.insert(id, redirect);
            } else {
                body_to_state.insert(key, id);
                impls.insert(id, exp);
            }
            return id;
        }
    };

    impls.insert(id, exp);

    id
}

fn producing_rule_arms(
    rule: &Rule,
    expr: &Expr,
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
    let mut body_to_state: HashMap<String, usize> = HashMap::new();
    let imp = add_impl(
        expr,
        ctx,
        terminals,
        &mut state_count,
        0,
        &mut impls,
        &mut body_to_state,
        &mut reachable,
        initial_states,
    );
    reachable.insert(imp); // entry state used by Rule::new

    let mut step_arms_vec: Vec<_> = impls
        .into_iter()
        .filter(|(k, _)| reachable.contains(k))
        .collect();
    step_arms_vec.sort_by_key(|(k, _)| *k);
    let step_arms = step_arms_vec
        .into_iter()
        .map(|(k, v)| quote! { (SyntaxKind::#n, #k) => { #v } });

    let new_arm = quote! { SyntaxKind::#n => Rule { kind, state: #imp }, };

    (quote! { #( #step_arms )* }, new_arm)
}

fn terminal_rule_arms(
    terminal: &str,
    _is_kw: bool,
    ctx: &Config,
) -> (token_stream::TokenStream, token_stream::TokenStream) {
    let n = ctx.context.ident_for(terminal);
    let step_arm = quote! {
        (SyntaxKind::#n, _) => {
            let added = state.expect_as(element, SyntaxKind::#n);
            if let Some(parent) = added.pop() {
                state.add_element(parent);
            }
        }
    };

    let new_arm = quote! { SyntaxKind::#n => Rule { kind, state: 0 }, };

    (step_arm, new_arm)
}

#[derive(Hash, PartialEq, PartialOrd, Debug, Clone, Eq, Ord)]
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

/// Collect all terminal names reachable from `expr` into `set`, using already-
/// computed results from `known` for non-terminal references.
fn expr_reachable_terminals(
    expr: &Expr,
    ctx: &Config,
    known: &HashMap<String, HashSet<String>>,
    set: &mut HashSet<String>,
) {
    match expr {
        Expr::Literal(lt, f) => {
            let ignore_case = if lt == &LiteralType::Double {
                IgnoreCase::True
            } else {
                IgnoreCase::False
            };
            let name = ctx.context.with(f);
            set.insert(Terminal::Literal(f.clone(), ignore_case).ident(ctx));
            let _ = name; // used via Terminal::Literal above
        }
        Expr::Reference(name) => {
            let is_terminal = !ctx.rules.producing.iter().any(|r| &r.name == name);
            if is_terminal {
                set.insert(Terminal::Ref(name.clone()).ident(ctx));
            } else if let Some(sub) = known.get(name) {
                set.extend(sub.iter().cloned());
            }
        }
        Expr::Seq(exprs) | Expr::Either(exprs) => {
            for e in exprs {
                expr_reachable_terminals(e, ctx, known, set);
            }
        }
        Expr::Marked(inner, _) => {
            expr_reachable_terminals(inner, ctx, known, set);
        }
    }
}

/// Compute the set of all terminal names that can be consumed *anywhere* in a
/// parse of each producing rule.  Uses fixed-point iteration to handle mutual
/// recursion.
fn compute_reachable_terminals(
    config: &Config,
) -> HashMap<String, HashSet<String>> {
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();

    let mut changed = true;
    while changed {
        changed = false;
        for rule in &config.rules.producing {
            let prev_len = result.get(&rule.name).map(|s| s.len()).unwrap_or(0);
            let snapshot = result.clone();
            let entry = result.entry(rule.name.clone()).or_default();
            expr_reachable_terminals(&rule.expression, config, &snapshot, entry);
            if result[&rule.name].len() > prev_len {
                changed = true;
            }
        }
    }

    result
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

    // Compute reachable terminal sets for all_tokens() pruning.
    let reachable_terminals = compute_reachable_terminals(&config);

    // Compact each rule's expression before code generation.
    let compacted: Vec<Expr> = config
        .rules
        .producing
        .iter()
        .map(|r| compact(r.expression.clone()))
        .collect();

    // Pre-compute the entry state for each producing rule so that push_rule can
    // emit `Rule { kind: X, state: N }` directly instead of `Rule::new(X)`.
    let initial_states: HashMap<String, usize> = config
        .rules
        .producing
        .iter()
        .zip(compacted.iter())
        .map(|(rule, expr)| {
            let entry = compute_entry_state(expr, &mut 1usize, 0);
            (rule.name.clone(), entry)
        })
        .collect();

    let mut terminals = HashSet::new();

    let (producing_step_arms, producing_new_arms): (Vec<_>, Vec<_>) = config
        .rules
        .producing
        .iter()
        .zip(compacted.iter())
        .map(|(value, expr)| {
            producing_rule_arms(value, expr, &config, &mut terminals, &initial_states)
        })
        .unzip();

    let sub_pattersn = get_sub_patterns(&config.rules.terminals);

    let mut sorted_terminals: Vec<_> = terminals.iter().collect();
    sorted_terminals.sort();

    let (terminal_step_arms, terminal_new_arms): (Vec<_>, Vec<_>) = sorted_terminals
        .iter()
        .map(|x| {
            let ident = x.ident(&config);
            let is_kw = match x {
                Terminal::Literal(_, _) => true,
                Terminal::Ref(_) => false,
            };
            terminal_rule_arms(&ident, is_kw, &config)
        })
        .unzip();

    let mut sorted_first_items: Vec<_> = first_items
        .iter()
        .filter(|(name, _)| config.rules.producing.iter().any(|r| &r.name == *name))
        .collect();
    sorted_first_items.sort_by_key(|(name, _)| name.as_str());

    let first_token_producing_arms: Vec<_> = sorted_first_items
        .iter()
        .map(|(name, items)| {
            let n = config.context.ident_for(name);
            let mut toks: Vec<_> = items
                .iter()
                .filter_map(|item| match item {
                    Item::Terminal(t) => Some(config.context.ident_for(t)),
                    Item::NonTerminal(_) => None,
                })
                .collect();
            toks.sort_by_key(|id| id.to_string());
            quote! { SyntaxKind::#n => &[#( SyntaxKind::#toks ),*], }
        })
        .collect();

    let first_token_terminal_arms: Vec<_> = sorted_terminals
        .iter()
        .map(|x| {
            let name = x.ident(&config);
            let n = config.context.ident_for(&name);
            quote! { SyntaxKind::#n => &[SyntaxKind::#n], }
        })
        .collect();

    // Build sorted_reachable: terminal sets for each producing rule, used by min_error_for_token.
    let mut sorted_reachable: Vec<_> = reachable_terminals
        .iter()
        .filter(|(name, _)| config.rules.producing.iter().any(|r| &r.name == *name))
        .collect();
    sorted_reachable.sort_by_key(|(name, _)| name.as_str());

    let mut producing: Vec<_> = config.rules.producing.iter().map(|x| &x.name).collect();
    producing.sort();

    // Build min_error_for_token arms for each non-nullable producing rule:
    // returns 0 if tok is in the reachable set, max_error_value() otherwise.
    // Nullable rules are omitted (fall through to the default arm that returns 0).
    let min_error_producing_arms: Vec<_> = sorted_reachable
        .iter()
        .filter(|(name, _)| !can_be_empty.get(*name).copied().unwrap_or(false))
        .map(|(name, toks)| {
            let n = config.context.ident_for(name);
            let mut tok_idents: Vec<_> = toks
                .iter()
                .map(|t| config.context.ident_for(t))
                .collect();
            tok_idents.sort_by_key(|id| id.to_string());
            tok_idents.dedup_by_key(|id| id.to_string());
            quote! {
                SyntaxKind::#n => match tok {
                    #( SyntaxKind::#tok_idents )|* => 0,
                    _ => kind.max_error_value(),
                },
            }
        })
        .collect();
    // Terminal rules: they match exactly one token kind; return 0 for that
    // token and max_error_value() for everything else.
    let min_error_terminal_arms: Vec<_> = sorted_terminals
        .iter()
        .map(|x| {
            let name = x.ident(&config);
            let n = config.context.ident_for(&name);
            quote! {
                SyntaxKind::#n => match tok {
                    SyntaxKind::#n => 0,
                    _ => kind.max_error_value(),
                },
            }
        })
        .collect();

    let terminals: Vec<_> = sorted_terminals
        .iter()
        .flat_map(|x| match x {
            Terminal::Literal(x, case) => {
                let name = config.context.with(x);
                let ident = config.context.ident_for(&name);

                if case == &IgnoreCase::False {
                    quote! {
                        #[token(#x)]
                        #ident,
                    }
                } else {
                    quote! {
                        #[token(#x, ignore(case))]
                        #ident,
                    }
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

    // Build max_error_value arms: for each terminal, emit the error_value used
    // by grammar rules that match it.  Only emit arms that differ from the
    // wildcard default (2), so the match stays compact.
    let max_error_value_arms: Vec<_> = sorted_terminals
        .iter()
        .filter_map(|x| {
            let ident_str = x.ident(&config);
            let ev = config
                .context
                .error_values
                .get(ident_str.as_str())
                .copied()?;

            let n = config.context.ident_for(&ident_str);
            Some(quote! { SyntaxKind::#n => #ev, })
        })
        .collect();

    // Build bracket_delta arms for openers (+1) and closers (-1).
    let bracket_delta_arms: Vec<_> = {
        let openers = config.context.bracket_openers.iter().map(|name| {
            let n = config.context.ident_for(name);
            quote! { SyntaxKind::#n => 1, }
        });
        let closers = config.context.bracket_closers.iter().map(|name| {
            let n = config.context.ident_for(name);
            quote! { SyntaxKind::#n => -1, }
        });
        openers.chain(closers).collect()
    };

    let enum_definition = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, logos::Logos)]
        #(#sub_pattersn)*
        #[repr(u16)]
        pub enum SyntaxKind {
            Eof = 0,
            #[regex(r"[ \t\n]+")]
            WhiteSpace,
            #[regex(r"#[^\n]+", allow_greedy=true)]
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

        /// Returns the minimum error cost that `kind` must incur when `tok`
        /// is the current token.  0 means the token is reachable (or the rule
        /// is nullable); positive means the rule cannot make progress without
        /// at least that much error cost.
        pub fn min_error_for_token(kind: SyntaxKind, tok: SyntaxKind) -> isize {
            match kind {
                #( #min_error_producing_arms )*
                #( #min_error_terminal_arms )*
                _ => 0,
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

        fn min_error_for_token(&self, tok: &SyntaxKind) -> isize {
            min_error_for_token(*self, *tok)
        }


        fn ending_tokens(&self) -> &'static [SyntaxKind] {
            &[]
        }

        fn max_error_value(&self) -> isize {
            match self {
                #( #max_error_value_arms )*
                _ => #DEFAULT_TOKEN_WEIGHT,
            }
        }

        fn bracket_delta(&self) -> i8 {
            match self {
                #( #bracket_delta_arms )*
                _ => 0,
            }
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
