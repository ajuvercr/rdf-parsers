use ariadne::{Report, ReportKind, Source};
use chumsky::error::Rich;
use proc_macro2::{Span, token_stream};
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::Cursor;
use std::ops::Range;
use syn::LitStr;

use crate::parser::{
    Config, Expr, FormatEntry, FormatHint, FormatPosition, LiteralType, Mark, Rule,
};
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

// ── State-machine code generation ────────────────────────────────────────────
//
// Each producing rule is compiled to a state machine.  State 0 is always
// "rule finished — pop frame".  Higher states correspond to positions inside
// the rule's grammar expression.
//
// TWO-PASS APPROACH
// ─────────────────
// The grammar may have cycles (e.g. collection ↔ object), so rules cannot be
// processed in topological order.  Instead two passes are used:
//
//   Pass 1 — compute_entry_state():
//     Walks each rule's expression counting state IDs in exactly the same order
//     as RuleCodegen::emit(), but generates no code.  Produces a map of
//     rule_name → entry_state for every rule upfront.
//
//   Pass 2 — RuleCodegen::emit():
//     The real code generation.  Allocates the same state IDs as pass 1 and
//     fills `impls` with the token streams for each state.  Can reference any
//     rule's entry state via `initial_states` because pass 1 already computed them.
//
// IMPORTANT: compute_entry_state() and RuleCodegen::emit() MUST mirror each
// other's state-allocation logic exactly.  If you change the structure of one,
// change the other.

/// Mirrors the state-counter allocation in `RuleCodegen::emit` without generating code,
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

// ── dist(q, a) transition analysis ────────────────────────────────────────────
//
// To compute the paper's dist(q, a) heuristic, we need a structural
// representation of each parser state's transitions.  The following builds
// that representation by walking the grammar expressions in the same order
// as compute_entry_state() / RuleCodegen::emit().

/// A transition from one parser state.
#[derive(Debug, Clone)]
enum Transition {
    /// State expects terminal `ident`, then transitions to `next`.
    Expect { ident: String, next: usize },
    /// State pushes sub-rule `rule` (at its `entry` state), then continues
    /// at `next` after the rule completes.
    Push {
        rule: String,
        entry: usize,
        next: usize,
    },
    /// State can pop (return to parent).  Terminates the rule.
    Pop,
    /// State branches to one of several alternative states.
    Goto(Vec<usize>),
}

/// Build transition info for `expr`, mirroring compute_entry_state's allocation.
fn build_transitions(
    expr: &Expr,
    ctx: &Config,
    initial_states: &HashMap<String, usize>,
    state_count: &mut usize,
    next: usize,
    transitions: &mut HashMap<usize, Vec<Transition>>,
) -> usize {
    match expr {
        Expr::Seq(exprs) => {
            let mut target = next;
            for e in exprs.iter().rev() {
                target =
                    build_transitions(e, ctx, initial_states, state_count, target, transitions);
            }
            target
        }
        Expr::Marked(inner, Mark::Option) => {
            let id = *state_count;
            *state_count += 1;
            let thing =
                build_transitions(inner, ctx, initial_states, state_count, next, transitions);
            transitions
                .entry(id)
                .or_default()
                .push(Transition::Goto(vec![thing, next]));
            id
        }
        Expr::Marked(inner, Mark::Plus) => {
            let id = *state_count;
            *state_count += 1;
            let done_once = *state_count;
            *state_count += 1;
            let thing = build_transitions(
                inner,
                ctx,
                initial_states,
                state_count,
                done_once,
                transitions,
            );
            transitions
                .entry(done_once)
                .or_default()
                .push(Transition::Goto(vec![thing, next]));
            transitions
                .entry(id)
                .or_default()
                .push(Transition::Goto(vec![thing]));
            id
        }
        Expr::Marked(inner, Mark::Star) => {
            let id = *state_count;
            *state_count += 1;
            let thing = build_transitions(inner, ctx, initial_states, state_count, id, transitions);
            transitions
                .entry(id)
                .or_default()
                .push(Transition::Goto(vec![thing, next]));
            id
        }
        Expr::Either(exprs) => {
            let id = *state_count;
            *state_count += 1;
            let targets: Vec<usize> = exprs
                .iter()
                .map(|e| build_transitions(e, ctx, initial_states, state_count, next, transitions))
                .collect();
            transitions
                .entry(id)
                .or_default()
                .push(Transition::Goto(targets));
            id
        }
        Expr::Literal(lt, f) => {
            let id = *state_count;
            *state_count += 1;
            let ignore_case = if lt == &LiteralType::Double {
                IgnoreCase::True
            } else {
                IgnoreCase::False
            };
            let ident = Terminal::Literal(f.clone(), ignore_case).ident(ctx);
            transitions
                .entry(id)
                .or_default()
                .push(Transition::Expect { ident, next });
            id
        }
        Expr::Reference(re) => {
            let id = *state_count;
            *state_count += 1;
            let is_terminal = !ctx.rules.producing.iter().any(|r| &r.name == re);
            if is_terminal {
                let ident = Terminal::Ref(re.clone()).ident(ctx);
                transitions
                    .entry(id)
                    .or_default()
                    .push(Transition::Expect { ident, next });
            } else {
                let entry = *initial_states.get(re.as_str()).unwrap_or(&0);
                transitions.entry(id).or_default().push(Transition::Push {
                    rule: re.clone(),
                    entry,
                    next,
                });
            }
            id
        }
    }
}

/// Compute `dist(state, terminal)` for all states in a rule.
///
/// Returns a map from `(state_id, terminal_ident)` to minimum insertion cost.
/// Missing entries default to 0 (admissible lower bound — the parent context
/// might accept the terminal after a pop).
fn compute_dist_for_rule(
    transitions: &HashMap<usize, Vec<Transition>>,
    state_count: usize,
    terminal_idents: &[String],
    error_values: &HashMap<String, isize>,
    min_completion_costs: &HashMap<String, isize>,
    initial_states: &HashMap<String, usize>,
    // dist table for all rules computed so far (rule_name → per-rule dist map)
    all_dists: &HashMap<String, HashMap<(usize, String), isize>>,
) -> HashMap<(usize, String), isize> {
    let mut dist: HashMap<(usize, String), isize> = HashMap::new();

    // Fixed-point iteration.
    let mut changed = true;
    let mut iterations = 0;
    while changed && iterations < 100 {
        changed = false;
        iterations += 1;

        for state_id in 0..state_count {
            let actions = match transitions.get(&state_id) {
                Some(a) => a.clone(),
                None => continue,
            };

            // State 0 is always Pop — dist = 0 for all terminals (default).
            if state_id == 0 {
                continue;
            }

            for terminal in terminal_idents {
                let ev = *error_values
                    .get(terminal.as_str())
                    .unwrap_or(&DEFAULT_TOKEN_WEIGHT);
                let key = (state_id, terminal.clone());

                let mut best = dist.get(&key).copied().unwrap_or(isize::MAX);

                for action in &actions {
                    let cost = match action {
                        Transition::Pop => 0,
                        Transition::Expect { ident, next } => {
                            if ident == terminal {
                                0
                            } else {
                                let expect_ev = *error_values
                                    .get(ident.as_str())
                                    .unwrap_or(&DEFAULT_TOKEN_WEIGHT);
                                let next_dist =
                                    dist.get(&(*next, terminal.clone())).copied().unwrap_or(0);
                                expect_ev.saturating_add(next_dist)
                            }
                        }
                        Transition::Push { rule, entry, next } => {
                            // Option 1: terminal is matched inside the pushed rule.
                            let inside = all_dists
                                .get(rule.as_str())
                                .and_then(|d| d.get(&(*entry, terminal.clone())))
                                .copied()
                                .unwrap_or(0);

                            // Option 2: pushed rule completes, terminal matched after.
                            let completion = *min_completion_costs.get(rule.as_str()).unwrap_or(&0);
                            let after = dist.get(&(*next, terminal.clone())).copied().unwrap_or(0);
                            let outside = completion.saturating_add(after);

                            inside.min(outside)
                        }
                        Transition::Goto(targets) => targets
                            .iter()
                            .map(|t| dist.get(&(*t, terminal.clone())).copied().unwrap_or(0))
                            .min()
                            .unwrap_or(0),
                    };

                    best = best.min(cost);
                }

                if best < isize::MAX && best != dist.get(&key).copied().unwrap_or(0) {
                    dist.insert(key, best);
                    changed = true;
                }
            }
        }
    }

    dist
}

/// Holds the mutable state for generating the state machine of one grammar rule.
///
/// State 0 is reserved for "rule finished — pop frame and return to parent".
/// All other states are allocated by `emit`.
struct RuleCodegen<'a> {
    ctx: &'a Config,
    terminals: &'a mut HashSet<Terminal>,
    /// Entry state for every producing rule that has already been code-generated.
    /// Needed so that `push_rule` can embed a rule's entry state directly.
    initial_states: &'a HashMap<String, usize>,
    state_count: usize,
    /// Maps state ID → token stream that runs when the A* reaches that state.
    impls: HashMap<usize, token_stream::TokenStream>,
    /// Body deduplication: maps serialised token stream → first state that used it.
    /// When a later state would emit identical code, it instead redirects (pop_push)
    /// to the canonical state, letting the A* dedup logic collapse both paths.
    body_to_state: HashMap<String, usize>,
    /// State IDs referenced by at least one other state or by `Rule::new`.
    /// Unreachable states are pruned before output.
    reachable: HashSet<usize>,
}

impl<'a> RuleCodegen<'a> {
    fn new(
        ctx: &'a Config,
        terminals: &'a mut HashSet<Terminal>,
        initial_states: &'a HashMap<String, usize>,
    ) -> Self {
        let mut impls = HashMap::new();
        impls.insert(
            0,
            quote! {
                if let Some(parent) = element.pop() {
                    state.add_element(parent);
                }
            },
        );
        Self {
            ctx,
            terminals,
            initial_states,
            state_count: 1,
            impls,
            body_to_state: HashMap::new(),
            reachable: HashSet::new(),
        }
    }

    fn alloc(&mut self) -> usize {
        let id = self.state_count;
        self.state_count += 1;
        id
    }

    /// Emit a pop_push to `target` and mark it reachable.
    fn goto(&mut self, target: usize) -> token_stream::TokenStream {
        self.reachable.insert(target);
        quote! {
            state.add_element(element.pop_push(Rule { kind: self.kind, state: #target }));
        }
    }

    /// Like `goto` but wraps the transition in `add_element_checked`, pruning
    /// branches where the current token can't match `terminal_kind`.
    fn goto_checked(
        &mut self,
        target: usize,
        terminal_kind: &proc_macro2::Ident,
    ) -> token_stream::TokenStream {
        self.reachable.insert(target);
        quote! {
            state.add_element_checked(
                element.pop_push(Rule { kind: self.kind, state: #target }),
                SyntaxKind::#terminal_kind,
            );
        }
    }

    /// If `expr` is a single terminal (literal or terminal reference), return
    /// its SyntaxKind ident.  Used by `Either` to emit checked gotos.
    fn terminal_kind_of(&self, expr: &Expr) -> Option<proc_macro2::Ident> {
        match expr {
            Expr::Literal(lt, f) => {
                let ignore_case = if lt == &LiteralType::Double {
                    IgnoreCase::True
                } else {
                    IgnoreCase::False
                };
                let name = Terminal::Literal(f.clone(), ignore_case).ident(self.ctx);
                Some(self.ctx.context.ident_for(&name))
            }
            Expr::Reference(re) => {
                let is_terminal = !self.ctx.rules.producing.iter().any(|r| &r.name == re);
                if is_terminal {
                    Some(self.ctx.context.ident_for(re))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Store `body` under `id`, or redirect to the canonical state if a duplicate body
    /// was already registered.
    fn register(&mut self, id: usize, body: token_stream::TokenStream) {
        let key = body.to_string();
        if let Some(&canonical) = self.body_to_state.get(&key) {
            // Redirect: pop_push to the canonical state so the A* dedup
            // collapses both paths at `canonical` on the next iteration.
            let redirect = quote! {
                state.add_element(element.pop_push(Rule { kind: self.kind, state: #canonical }));
            };
            self.impls.insert(id, redirect);
        } else {
            self.body_to_state.insert(key, id);
            self.impls.insert(id, body);
        }
    }

    /// Recursively emit the state machine for `expr`, where `next` is the state to
    /// transition to after `expr` is fully matched.  Returns the entry state for `expr`.
    ///
    /// IMPORTANT: state allocation here must mirror `compute_entry_state` exactly.
    fn emit(&mut self, expr: &Expr, next: usize) -> usize {
        match expr {
            Expr::Seq(exprs) => {
                let mut target = next;
                for e in exprs.iter().rev() {
                    target = self.emit(e, target);
                }
                target
            }
            Expr::Marked(inner, Mark::Option) => {
                let id = self.alloc();
                let thing = self.emit(inner, next);
                let tt = self.goto(thing);
                let next_tt = self.goto(next);
                self.impls.insert(id, quote! { #next_tt #tt });
                id
            }
            Expr::Marked(inner, Mark::Plus) => {
                let id = self.alloc();
                let done_once = self.alloc();
                let thing = self.emit(inner, done_once);
                let tt = self.goto(thing);
                let next_tt = self.goto(next);
                self.impls.insert(done_once, quote! { #tt #next_tt });
                self.impls.insert(id, quote! { #tt });
                id
            }
            Expr::Marked(inner, Mark::Star) => {
                let id = self.alloc();
                let thing = self.emit(inner, id);
                let tt = self.goto(thing);
                let next_tt = self.goto(next);
                self.impls.insert(id, quote! { #tt #next_tt });
                id
            }
            Expr::Either(exprs) => {
                let id = self.alloc();
                let things: Vec<_> = exprs.iter().map(|e| self.emit(e, next)).collect();
                // For each alternative, determine whether it starts with a single
                // terminal.  If so, use add_element_checked to prune branches where
                // the current token can't possibly match.  This avoids the
                // combinatorial explosion that e.g. jsonString (24 keyword
                // alternatives) would cause.
                let gotos: Vec<_> = exprs
                    .iter()
                    .zip(things.iter())
                    .map(|(expr, &target)| {
                        if let Some(terminal_kind) = self.terminal_kind_of(expr) {
                            self.goto_checked(target, &terminal_kind)
                        } else {
                            self.goto(target)
                        }
                    })
                    .collect();
                self.impls.insert(id, quote! { #( #gotos )* });
                id
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
                let id = self.alloc();
                self.terminals
                    .insert(Terminal::Literal(f.clone(), ignore_case));
                let name = self.ctx.context.with(f);
                let n = self.ctx.context.ident_for(&name);
                self.reachable.insert(next);
                let exp = inline_terminal_rule(next, &n);
                self.register(id, exp);
                id
            }
            Expr::Reference(re) => {
                let id = self.alloc();
                let is_terminal = !self.ctx.rules.producing.iter().any(|r| &r.name == re);
                if is_terminal {
                    self.terminals.insert(Terminal::Ref(re.clone()));
                }
                let n = self.ctx.context.ident_for(re);
                self.reachable.insert(next);
                let exp = if is_terminal {
                    inline_terminal_rule(next, &n)
                } else {
                    let entry = *self
                        .initial_states
                        .get(re)
                        .unwrap_or_else(|| panic!("unknown rule: {re}"));
                    push_rule(next, &n, entry)
                };
                self.register(id, exp);
                id
            }
        }
    }

    /// Create a `RuleCodegen`, emit the state machine for `expr`, then assemble and
    /// return `(entry_state, step_arms, new_arm)`.
    fn generate(
        rule_name: &str,
        expr: &Expr,
        ctx: &'a Config,
        terminals: &'a mut HashSet<Terminal>,
        initial_states: &'a HashMap<String, usize>,
    ) -> (usize, token_stream::TokenStream, token_stream::TokenStream) {
        let mut cg = Self::new(ctx, terminals, initial_states);
        let entry = cg.emit(expr, 0);
        cg.reachable.insert(entry); // entry state used by Rule::new

        let n = ctx.context.ident_for(rule_name);
        let reachable = cg.reachable;
        let mut step_arms_vec: Vec<_> = cg
            .impls
            .into_iter()
            .filter(|(k, _)| reachable.contains(k))
            .collect();
        step_arms_vec.sort_by_key(|(k, _)| *k);
        let step_arms = step_arms_vec
            .into_iter()
            .map(|(k, v)| quote! { (SyntaxKind::#n, #k) => { #v } });

        let new_arm = quote! { SyntaxKind::#n => Rule { kind, state: #entry }, };

        (entry, quote! { #( #step_arms )* }, new_arm)
    }
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

// ── Minimum completion cost ───────────────────────────────────────────────────

/// Compute the minimum insertion cost to complete `expr` along its cheapest path.
/// Returns `None` for recursive references not yet resolved (treated as 0).
fn expr_min_completion_cost(expr: &Expr, ctx: &Config, known: &HashMap<String, isize>) -> isize {
    match expr {
        Expr::Literal(lt, f) => {
            let ignore_case = if lt == &LiteralType::Double {
                IgnoreCase::True
            } else {
                IgnoreCase::False
            };
            let ident = Terminal::Literal(f.clone(), ignore_case).ident(ctx);
            *ctx.context
                .error_values
                .get(ident.as_str())
                .unwrap_or(&DEFAULT_TOKEN_WEIGHT)
        }
        Expr::Reference(name) => {
            let is_terminal = !ctx.rules.producing.iter().any(|r| &r.name == name);
            if is_terminal {
                let ident = Terminal::Ref(name.clone()).ident(ctx);
                *ctx.context
                    .error_values
                    .get(ident.as_str())
                    .unwrap_or(&DEFAULT_TOKEN_WEIGHT)
            } else {
                // Non-terminal: use previously computed value, or 0 for cycles.
                known.get(name).copied().unwrap_or(0)
            }
        }
        Expr::Seq(exprs) => exprs
            .iter()
            .map(|e| expr_min_completion_cost(e, ctx, known))
            .sum(),
        Expr::Either(exprs) => exprs
            .iter()
            .map(|e| expr_min_completion_cost(e, ctx, known))
            .min()
            .unwrap_or(0),
        Expr::Marked(inner, Mark::Star | Mark::Option) => 0, // nullable — can skip
        Expr::Marked(inner, Mark::Plus) => expr_min_completion_cost(inner, ctx, known),
    }
}

/// Compute `min_completion_cost(rule_name)` for every producing rule.
/// Uses fixed-point iteration to handle mutual recursion.
fn compute_min_completion_costs(config: &Config) -> HashMap<String, isize> {
    let mut result: HashMap<String, isize> = HashMap::new();
    let mut changed = true;
    while changed {
        changed = false;
        for rule in &config.rules.producing {
            let new_cost = expr_min_completion_cost(&rule.expression, config, &result);
            let prev = result.get(&rule.name).copied().unwrap_or(0);
            if new_cost != prev {
                result.insert(rule.name.clone(), new_cost);
                changed = true;
            }
        }
    }
    result
}

/// Collect all terminal idents from `expr` into `set` (non-recursive into sub-rules).
fn expr_reachable_terminals_flat(expr: &Expr, ctx: &Config, set: &mut HashSet<String>) {
    match expr {
        Expr::Literal(lt, f) => {
            let ignore_case = if lt == &LiteralType::Double {
                IgnoreCase::True
            } else {
                IgnoreCase::False
            };
            set.insert(Terminal::Literal(f.clone(), ignore_case).ident(ctx));
        }
        Expr::Reference(name) => {
            let is_terminal = !ctx.rules.producing.iter().any(|r| &r.name == name);
            if is_terminal {
                set.insert(Terminal::Ref(name.clone()).ident(ctx));
            }
            // Don't recurse into producing rules — the outer loop handles all rules.
        }
        Expr::Seq(exprs) | Expr::Either(exprs) => {
            for e in exprs {
                expr_reachable_terminals_flat(e, ctx, set);
            }
        }
        Expr::Marked(inner, _) => {
            expr_reachable_terminals_flat(inner, ctx, set);
        }
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
fn compute_reachable_terminals(config: &Config) -> HashMap<String, HashSet<String>> {
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
    // ── Phase 1: Parse grammar ──
    let config = match crate::parser::parse(contents) {
        Ok(r) => r,
        Err(es) => {
            let rendered = render_ariadne_reports(path, contents, &es);
            panic!("Grammar parse error in {}:\n{}", path, rendered);
        }
    };

    // ── Phase 2: Analyse rule properties ──
    let mut can_be_empty = HashMap::new();
    let mut first_items = HashMap::new();

    for r in config.rules.producing.iter() {
        set_can_be_empty(&r.name, &config, &mut can_be_empty);
    }

    for r in config.rules.producing.iter() {
        set_first_possible_item(&r.name, &config, &mut first_items, &mut can_be_empty);
    }

    // Compute reachable terminal sets for all_tokens() pruning.
    let reachable_terminals = compute_reachable_terminals(&config);

    // Compute minimum completion cost for each producing rule.
    let min_completion_costs = compute_min_completion_costs(&config);

    // Compact each rule's expression before code generation.
    let compacted: Vec<Expr> = config
        .rules
        .producing
        .iter()
        .map(|r| compact(r.expression.clone()))
        .collect();

    // ── Phase 3: Pre-compute entry states (pass 1 of the two-pass codegen) ──
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

    // ── Phase 3b: Build transition graphs and compute dist(state, terminal) ──
    // Collect all terminal idents for the dist computation.
    let all_terminal_idents: Vec<String> = {
        let mut t = HashSet::new();
        for rule in &config.rules.producing {
            expr_reachable_terminals_flat(&rule.expression, &config, &mut t);
        }
        let mut v: Vec<_> = t.into_iter().collect();
        v.sort();
        v
    };

    // Build transitions for each producing rule and compute dist tables.
    let dist_tables: HashMap<String, HashMap<(usize, String), isize>> = {
        // Collect error_values for terminals.
        let error_values: HashMap<String, isize> = config
            .context
            .error_values
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        let mut all_dists: HashMap<String, HashMap<(usize, String), isize>> = HashMap::new();

        // Fixed-point: iterate until all cross-rule references converge.
        // Large grammars (e.g. SPARQL, ~900 states) may need many iterations
        // when deeply nested rules depend on each other.
        let mut changed = true;
        let mut outer_iters = 0;
        while changed && outer_iters < 100 {
            changed = false;
            outer_iters += 1;

            for (rule, expr) in config.rules.producing.iter().zip(compacted.iter()) {
                let mut transitions = HashMap::new();
                let mut sc = 1usize; // state 0 = pop
                transitions.insert(0usize, vec![Transition::Pop]);
                build_transitions(expr, &config, &initial_states, &mut sc, 0, &mut transitions);

                let new_dist = compute_dist_for_rule(
                    &transitions,
                    sc,
                    &all_terminal_idents,
                    &error_values,
                    &min_completion_costs,
                    &initial_states,
                    &all_dists,
                );

                if all_dists.get(&rule.name) != Some(&new_dist) {
                    all_dists.insert(rule.name.clone(), new_dist);
                    changed = true;
                }
            }
        }
        all_dists
    };

    // ── Phase 4: Generate state-machine code (pass 2) ──
    let mut terminals = HashSet::new();

    let (producing_step_arms, producing_new_arms): (Vec<_>, Vec<_>) = config
        .rules
        .producing
        .iter()
        .zip(compacted.iter())
        .map(|(rule, expr)| {
            let (_entry, step_arms, new_arm) =
                RuleCodegen::generate(&rule.name, expr, &config, &mut terminals, &initial_states);
            (step_arms, new_arm)
        })
        .unzip();

    let sub_pattersn = get_sub_patterns(&config.rules.terminals);

    let mut sorted_terminals: Vec<_> = terminals.iter().collect();
    sorted_terminals.sort();

    let (terminal_step_arms, terminal_new_arms): (Vec<_>, Vec<_>) = sorted_terminals
        .iter()
        .map(|x| {
            let ident = x.ident(&config);
            let n = config.context.ident_for(&ident);
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
        })
        .unzip();

    // ── Phase 5: Build token enum and trait impls ──

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
            let mut tok_idents: Vec<_> = toks.iter().map(|t| config.context.ident_for(t)).collect();
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

    // Build min_completion_cost arms for producing rules.
    let min_completion_cost_arms: Vec<_> = {
        let mut sorted: Vec<_> = min_completion_costs.iter().collect();
        sorted.sort_by_key(|(name, _)| name.as_str());
        sorted
            .iter()
            .filter(|&&(_, cost)| *cost > DEFAULT_TOKEN_WEIGHT) // only emit non-default
            .map(|&(name, cost)| {
                let n = config.context.ident_for(name);
                let c = *cost;
                quote! { SyntaxKind::#n => #c, }
            })
            .collect()
    };

    // Build state_dist match arms from the precomputed dist tables.
    // Group by (kind, state) and emit inner match on terminal.
    // Optimisation: when all terminals share the same dist, emit a single
    // wildcard arm instead of enumerating every terminal.
    let state_dist_arms: Vec<_> = {
        let mut arms = Vec::new();
        let mut sorted_rules: Vec<_> = config.rules.producing.iter().map(|r| &r.name).collect();
        sorted_rules.sort();

        for rule_name in &sorted_rules {
            if let Some(dist_map) = dist_tables.get(*rule_name) {
                // Group entries by state_id.
                let mut by_state: HashMap<usize, Vec<(&String, &isize)>> = HashMap::new();
                for ((state_id, terminal), cost) in dist_map {
                    if *cost > 0 {
                        by_state
                            .entry(*state_id)
                            .or_default()
                            .push((terminal, cost));
                    }
                }

                let kind_ident = config.context.ident_for(rule_name);

                let mut by_state_sorted: Vec<_> = by_state.into_iter().collect();
                by_state_sorted.sort_by_key(|(state_id, _)| *state_id);
                for (state_id, mut entries) in by_state_sorted {
                    if entries.is_empty() {
                        continue;
                    }
                    entries.sort_by_key(|(t, _)| t.to_string());
                    let st = state_id;

                    // Check if all entries share the same cost value.
                    let first_cost = *entries[0].1;
                    let all_same = entries.iter().all(|(_, c)| **c == first_cost);

                    if all_same && entries.len() > 3 {
                        // Emit a compact wildcard arm with exceptions for 0-cost terminals.
                        let zero_terminals: Vec<_> = all_terminal_idents
                            .iter()
                            .filter(|t| !entries.iter().any(|(et, _)| et == t))
                            .map(|t| config.context.ident_for(t))
                            .collect();
                        let c = first_cost;
                        if zero_terminals.is_empty() {
                            arms.push(quote! {
                                (SyntaxKind::#kind_ident, #st, _) => #c,
                            });
                        } else {
                            arms.push(quote! {
                                (SyntaxKind::#kind_ident, #st, _) => match terminal {
                                    #( SyntaxKind::#zero_terminals )|* => 0,
                                    _ => #c,
                                },
                            });
                        }
                    } else {
                        // Emit individual terminal arms.
                        let terminal_arms: Vec<_> = entries
                            .iter()
                            .map(|(terminal, cost)| {
                                let t_ident = config.context.ident_for(terminal);
                                let c = **cost;
                                quote! { SyntaxKind::#t_ident => #c, }
                            })
                            .collect();
                        arms.push(quote! {
                            (SyntaxKind::#kind_ident, #st, _) => match terminal {
                                #( #terminal_arms )*
                                _ => 0,
                            },
                        });
                    }
                }
            }
        }
        arms
    };

    // ── Format module codegen ────────────────────────────────────────────────

    let format_mod = if config.context.format_groups.is_empty()
        && config.context.format_hints.is_empty()
    {
        quote! {}
    } else {
        // Build format_hints() arms.
        // Scoped arms (ruleName.token) come first so they take precedence.
        let scoped: Vec<_> = config
            .context
            .format_hints
            .iter()
            .filter(|e| e.scope.is_some() && e.token != "default")
            .map(|e| format_hint_arm(e, &config))
            .collect();
        let global: Vec<_> = config
            .context
            .format_hints
            .iter()
            .filter(|e| e.scope.is_none() && e.token != "default")
            .map(|e| format_hint_arm(e, &config))
            .collect();
        // The `default` entry (if any) provides hints for unmatched *terminals*.
        // Non-terminals always fall through to (Hints::default(), Hints::default()).
        let default_after_hints: proc_macro2::TokenStream = config
            .context
            .format_hints
            .iter()
            .find(|e| e.token == "default")
            .map(|e| format_hint_arm_default_tuple(e))
            .unwrap_or_else(|| quote! { (Hints::default(), Hints::default()) });

        // Build is_group() arms.
        let group_idents: Vec<_> = config
            .context
            .format_groups
            .iter()
            .map(|name| config.context.ident_for(name))
            .collect();

        let is_group_body = if group_idents.is_empty() {
            quote! { false }
        } else {
            quote! { matches!(kind, #( SyntaxKind::#group_idents )|*) }
        };

        quote! {
            pub mod format {
                use super::{SyntaxKind, SyntaxNode};
                use rowan::NodeOrToken;
                use crate::TokenTrait;
                use crate::format::Doc;

                #[derive(Default)]
                struct Hints {
                    space: bool,
                    line: bool,
                    hardline: bool,
                    blankline: bool,
                    indent: bool,
                    dedent: bool,
                }

                impl Hints {
                    fn to_docs(&self) -> Vec<Doc> {
                        let mut v = Vec::new();
                        if self.indent { v.push(Doc::nil()); } // handled by caller
                        if self.dedent { v.push(Doc::nil()); } // handled by caller
                        if self.blankline { v.push(Doc::HardLine); v.push(Doc::HardLine); }
                        else if self.hardline { v.push(Doc::HardLine); }
                        else if self.line { v.push(Doc::Line); }
                        else if self.space { v.push(Doc::text(" ")); }
                        v
                    }
                }

                fn format_hints(parent: SyntaxKind, token: SyntaxKind) -> (Hints, Hints) {
                    match (parent, token) {
                        #( #scoped )*
                        #( #global )*
                        _ => #default_after_hints,
                    }
                }

                fn is_group(kind: SyntaxKind) -> bool {
                    #is_group_body
                }

                pub fn to_doc(node: &SyntaxNode) -> Doc {
                    let mut parts: Vec<Vec<Doc>> = vec![vec![]];

                    for child in node.children_with_tokens() {
                        match child {
                            NodeOrToken::Token(t) => {
                                if t.kind() == SyntaxKind::WhiteSpace {
                                    continue;
                                }
                                if t.kind() == SyntaxKind::Comment {
                                    // Preserve comments verbatim.
                                    parts.last_mut().unwrap().push(Doc::text(t.text().to_string()));
                                    continue;
                                }
                                let (before, after) = format_hints(node.kind(), t.kind());

                                // Before hints: dedent first, then line/space.
                                if before.dedent && parts.len() > 1 {
                                    let nested = parts.pop().unwrap_or_default();
                                    let indent_doc = Doc::nest(2, Doc::concat(nested));
                                    parts.last_mut().unwrap().push(indent_doc);
                                }
                                parts.last_mut().unwrap().extend(before.to_docs().into_iter().filter(|d| !matches!(d, Doc::Nil)));

                                parts.last_mut().unwrap().push(Doc::text(t.text().to_string()));

                                // After hints: if indent, the line break goes
                                // INSIDE the new nest level (as its first item),
                                // so it is indented in break mode.
                                let after_line: Vec<Doc> = after.to_docs().into_iter().filter(|d| !matches!(d, Doc::Nil)).collect();
                                if after.indent {
                                    parts.push(after_line); // line is first item inside the nest
                                } else {
                                    parts.last_mut().unwrap().extend(after_line);
                                }
                            }
                            NodeOrToken::Node(n) => {
                                if n.kind() == SyntaxKind::ERROR {
                                    // Preserve error nodes verbatim.
                                    parts.last_mut().unwrap().push(Doc::text(n.text().to_string()));
                                    continue;
                                }
                                // Detect terminal wrapper nodes (a single non-whitespace
                                // token child, no sub-nodes). For these we apply format
                                // hints in the *parent's* context so that indent/dedent
                                // correctly share the same `parts` stack.
                                let is_terminal_wrapper = {
                                    let mut it = n.children_with_tokens().filter(|c| match c {
                                        NodeOrToken::Token(t) => t.kind() != SyntaxKind::WhiteSpace,
                                        NodeOrToken::Node(_) => true,
                                    });
                                    matches!(it.next(), Some(NodeOrToken::Token(_))) && it.next().is_none()
                                };
                                if is_terminal_wrapper {
                                    let token_text = n.text().to_string();
                                    let (before, after) = format_hints(node.kind(), n.kind());

                                    if before.dedent && parts.len() > 1 {
                                        let nested = parts.pop().unwrap_or_default();
                                        let indent_doc = Doc::nest(2, Doc::concat(nested));
                                        parts.last_mut().unwrap().push(indent_doc);
                                    }
                                    parts.last_mut().unwrap().extend(before.to_docs().into_iter().filter(|d| !matches!(d, Doc::Nil)));

                                    parts.last_mut().unwrap().push(Doc::text(token_text));

                                    let after_line: Vec<Doc> = after.to_docs().into_iter().filter(|d| !matches!(d, Doc::Nil)).collect();
                                    if after.indent {
                                        parts.push(after_line);
                                    } else {
                                        parts.last_mut().unwrap().extend(after_line);
                                    }
                                } else {
                                    // Non-terminal node: apply before/after hints
                                    // from the parent's perspective, then recurse.
                                    let (before, after) = format_hints(node.kind(), n.kind());

                                    if before.dedent && parts.len() > 1 {
                                        let nested = parts.pop().unwrap_or_default();
                                        let indent_doc = Doc::nest(2, Doc::concat(nested));
                                        parts.last_mut().unwrap().push(indent_doc);
                                    }
                                    parts.last_mut().unwrap().extend(before.to_docs().into_iter().filter(|d| !matches!(d, Doc::Nil)));

                                    let child_doc = to_doc(&n);
                                    let child_doc = if is_group(n.kind()) {
                                        Doc::group(child_doc)
                                    } else {
                                        child_doc
                                    };
                                    parts.last_mut().unwrap().push(child_doc);

                                    let after_line: Vec<Doc> = after.to_docs().into_iter().filter(|d| !matches!(d, Doc::Nil)).collect();
                                    if after.indent {
                                        parts.push(after_line);
                                    } else {
                                        parts.last_mut().unwrap().extend(after_line);
                                    }
                                }
                            }
                        }
                    }

                    // Drain any unclosed indent levels.
                    while parts.len() > 1 {
                        let nested = parts.pop().unwrap();
                        parts.last_mut().unwrap().push(Doc::nest(2, Doc::concat(nested)));
                    }

                    Doc::concat(parts.pop().unwrap_or_default())
                }

                pub fn format(node: &SyntaxNode, width: usize) -> String {
                    let doc = to_doc(node);
                    let s = crate::format::render(&doc, width);
                    let s = s.trim_start_matches('\n').trim_end_matches('\n');
                    format!("{s}\n")
                }
            }
        }
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

        /// Precomputed dist(q, a): minimum insertion cost to reach a point
        /// where terminal `terminal` can be matched, starting from parser
        /// state `(kind, state)`.  Returns 0 as the conservative default
        /// (admissible — parent context might accept the terminal after a pop).
        pub fn state_dist(kind: SyntaxKind, state: usize, terminal: SyntaxKind) -> isize {
            match (kind, state, terminal) {
                #( #state_dist_arms )*
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

            fn state_dist(&self, terminal: &SyntaxKind) -> isize {
                state_dist(self.kind, self.state, *terminal)
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

        fn min_completion_cost(&self) -> isize {
            match self {
                #( #min_completion_cost_arms )*
                _ => Self::max_error_value(self),
            }
        }
    }

    #format_mod

        };

    out.to_string()
}

/// Build one match arm for the generated `format_hints()` function.
fn format_hint_arm(entry: &FormatEntry, config: &Config) -> proc_macro2::TokenStream {
    let tok_ident = config.context.ident_for(&entry.token);

    let parent_pat: proc_macro2::TokenStream = match &entry.scope {
        Some(scope) => {
            let s = config.context.ident_for(scope);
            quote! { SyntaxKind::#s }
        }
        None => quote! { _ },
    };

    let mut before = quote! {
        Hints { space: false, line: false, hardline: false, blankline: false, indent: false, dedent: false }
    };
    let mut after = quote! {
        Hints { space: false, line: false, hardline: false, blankline: false, indent: false, dedent: false }
    };

    let hints_to_struct = |hints: &[FormatHint]| -> proc_macro2::TokenStream {
        let space = hints.contains(&FormatHint::Space);
        let line = hints.contains(&FormatHint::Line);
        let hardline = hints.contains(&FormatHint::HardLine);
        let blankline = hints.contains(&FormatHint::BlankLine);
        let indent = hints.contains(&FormatHint::Indent);
        let dedent = hints.contains(&FormatHint::Dedent);
        quote! {
            Hints { space: #space, line: #line, hardline: #hardline, blankline: #blankline, indent: #indent, dedent: #dedent }
        }
    };

    for (p, hints) in &entry.items {
        match p {
            FormatPosition::Before => {
                before = hints_to_struct(&hints);
            }
            FormatPosition::After => {
                after = hints_to_struct(&hints);
            }
        }
    }

    quote! {
        (#parent_pat, SyntaxKind::#tok_ident) => (#before, #after),
    }
}

/// Build just the hints tuple `(before, after)` from a `default` format entry,
/// for use in the `_ => if is_terminal { ... } else { ... }` fallback arm.
fn format_hint_arm_default_tuple(entry: &FormatEntry) -> proc_macro2::TokenStream {
    let hints_to_struct = |hints: &[FormatHint]| -> proc_macro2::TokenStream {
        let space = hints.contains(&FormatHint::Space);
        let line = hints.contains(&FormatHint::Line);
        let hardline = hints.contains(&FormatHint::HardLine);
        let blankline = hints.contains(&FormatHint::BlankLine);
        let indent = hints.contains(&FormatHint::Indent);
        let dedent = hints.contains(&FormatHint::Dedent);
        quote! {
            Hints { space: #space, line: #line, hardline: #hardline, blankline: #blankline, indent: #indent, dedent: #dedent }
        }
    };

    let mut before = quote! {
        Hints { space: false, line: false, hardline: false, blankline: false, indent: false, dedent: false }
    };
    let mut after = quote! {
        Hints { space: false, line: false, hardline: false, blankline: false, indent: false, dedent: false }
    };

    for (p, hints) in &entry.items {
        match p {
            FormatPosition::Before => {
                before = hints_to_struct(&hints);
            }
            FormatPosition::After => {
                after = hints_to_struct(&hints);
            }
        }
    }

    quote! { (#before, #after) }
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
