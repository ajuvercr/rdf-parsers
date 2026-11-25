use std::borrow::Borrow;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use logos::{Lexer, Logos};
pub use parser::*;

mod a_star;
mod incremental;
pub use a_star::Fingerprint;
pub use a_star::ParseMode;
pub use a_star::ParserTrait;
#[doc(hidden)]
pub use a_star::{
    CONSTRUCT_DELETE, CONSTRUCT_DELETE_MORE, CONSTRUCT_INSERT, DEFAULT_LOCAL_WINDOW, Engine, last_commits,
    last_iterations,
};
pub mod list;
pub mod model;
pub mod format;
#[macro_use]
mod rdf_format;
pub mod jsonld;
pub mod n3;
pub mod ntriples;
mod parser;
pub mod resolve;
pub mod sparql;
pub mod trig;
pub mod turtle;
pub mod util;

#[derive(Debug, Clone)]
pub struct Spanned<T>(pub T, pub std::ops::Range<usize>);
impl<T> Default for Spanned<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(T::default(), 0..1)
    }
}
impl Borrow<str> for Spanned<String> {
    #[inline]
    fn borrow(&self) -> &str {
        &self[..]
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T: std::hash::Hash> std::hash::Hash for Spanned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
impl<T: PartialEq> Eq for Spanned<T> {}

impl<T> Spanned<T> {
    pub fn into_value(self) -> T {
        self.0
    }
    pub fn into_span(self) -> std::ops::Range<usize> {
        self.1
    }
    pub fn value(&self) -> &T {
        &self.0
    }
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.0
    }
    pub fn span(&self) -> &std::ops::Range<usize> {
        &self.1
    }

    pub fn map<O>(self, f: impl Fn(T) -> O) -> Spanned<O> {
        Spanned(f(self.0), self.1)
    }
    pub fn map_ref<'a, O: 'a>(&'a self, f: impl Fn(&'a T) -> O) -> Spanned<O> {
        Spanned(f(&self.0), self.1.clone())
    }
    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned(&self.0, self.1.clone())
    }
    pub fn try_map_ref<'a, O>(&'a self, f: impl FnOnce(&'a T) -> Option<O>) -> Option<Spanned<O>> {
        f(&self.0).map(|v| Spanned(v, self.1.clone()))
    }
    pub fn try_map<O>(self, f: impl FnOnce(T) -> Option<O>) -> Option<Spanned<O>> {
        f(self.0).map(|v| Spanned(v, self.1))
    }
}

impl<T> Spanned<Option<T>> {
    pub fn transpose(self) -> Option<Spanned<T>> {
        self.0.map(|inner| Spanned(inner, self.1))
    }
}

pub fn spanned<T>(t: T, span: std::ops::Range<usize>) -> Spanned<T> {
    Spanned(t, span)
}

pub trait TokenTrait:
    Debug + Clone + Into<rowan::SyntaxKind> + From<rowan::SyntaxKind> + PartialEq + Eq + Hash + 'static
{
    const ERROR: Self;
    const ROOT: Self;

    fn branch(&self) -> u32;

    fn skips(&self) -> bool;

    fn starting_tokens(&self) -> &'static [Self];
    fn ending_tokens(&self) -> &'static [Self];

    /// Whether this rule can start with `tok`, or match nothing: whether it
    /// can make progress without an error when `tok` is the current token.
    /// Lets fast mode drop a branch as soon as it is pushed.  Defaults to
    /// `true`.
    fn can_start_with(&self, _tok: &Self) -> bool {
        true
    }


    /// Penalty for inventing this construct during recovery: entering the
    /// rule and inserting its first token instead of matching one.  Set per
    /// rule in the grammar's `=== construct_cost ===` section; only the
    /// search charges it (see `CONSTRUCT_INSERT`).
    fn construct_cost(&self) -> isize {
        0
    }

    /// Bracket nesting delta for this token kind.
    ///
    /// Returns +1 for opening brackets (`{`, `[`, `(`), -1 for closing
    /// brackets (`}`, `]`, `)`), and 0 for all other tokens.  Used by the
    /// incremental A* to track the bracket nesting depth at which each token
    /// was parsed, enabling a one-time "context-shift" cost when a block of
    /// tokens moves to a different nesting level between edits.
    fn bracket_delta(&self) -> i8 {
        0
    }
}

pub fn tokenize<'a, K>(text: &'a str) -> Vec<FatToken<K>>
where
    K: TokenTrait + Logos<'a, Source = str>,
    <K as Logos<'a>>::Extras: Default,
{
    let mut lexer: Lexer<'a, K> = Lexer::new(text);
    let mut tokens: Vec<FatToken<K>> = Vec::new();
    while let Some(t) = lexer.next() {
        let kind = t.unwrap_or(K::ERROR);
        let span = lexer.span();
        if kind == K::ERROR {
            if let Some(last) = tokens.last_mut() {
                if last.kind == K::ERROR {
                    let start = last.range.start;
                    last.extend_to(span.end, text[start..span.end].to_string());
                    continue;
                }
            }
        }
        tokens.push(FatToken::new(kind, span.clone(), text[span].to_string()));
    }
    tokens
}

pub fn parse<'a, T: a_star::ParserTrait + 'static>(
    root: T,
    text: &'a str,
) -> (Parse, Vec<FatToken<T::Kind>>)
where
    T::Kind: Logos<'a, Source = str>,
    <<T as a_star::ParserTrait>::Kind as Logos<'a>>::Extras: Default,
{
    let mut tokens = tokenize::<T::Kind>(text);
    let list = a_star::a_star(
        root,
        &tokens,
        IncrementalBias::default(),
        a_star::DEFAULT_MAX_ITERATIONS,
    );
    let parse = Parse::from_steps(&mut tokens, list);
    (parse, tokens)
}

/// Parses `text` in non-fault-tolerant fast mode.
///
/// Returns `Some((parse, tokens))` when the document is error-free, or `None`
/// if any token could not be matched.  Use this when you know the document is
/// correct and want maximum throughput — no error-recovery branches are
/// explored, no prevInfo fingerprints are tracked, and no heuristic is
/// precomputed.
pub fn parse_fast<'a, T: a_star::ParserTrait + 'static>(
    root: T,
    text: &'a str,
) -> Option<(Parse, Vec<FatToken<T::Kind>>)>
where
    T::Kind: Logos<'a, Source = str>,
    <<T as a_star::ParserTrait>::Kind as Logos<'a>>::Extras: Default,
{
    let mut tokens = tokenize::<T::Kind>(text);
    let list = a_star::a_star_fast(root, &tokens)?;
    let parse = Parse::from_steps(&mut tokens, list);
    Some((parse, tokens))
}

/// A single token from a previous parse, carrying the text, fingerprint, and
/// bracket nesting depth needed for incremental re-parsing.
pub struct PrevToken {
    pub text: String,
    pub fingerprint: Option<Fingerprint>,
    /// Bracket nesting depth at which this token was parsed (0 = top level).
    /// Used by the incremental A* to detect context-shifts (a block of tokens
    /// moving to a different nesting level) and charge for them only once.
    pub depth: u8,
}

/// Information from a previous parse needed for incremental re-parsing.
#[derive(Default)]
pub struct PrevParseInfo {
    pub tokens: Vec<PrevToken>,
    /// Whether the previous parse produced any errors.  When true, stored
    /// bracket depths are not used for conflict classification (they may be
    /// misleading due to error-recovery changing the grammar context), so
    /// fingerprint conflicts are treated as free-pass rather than +50 role
    /// conflicts.
    pub had_errors: bool,
    /// The previous parse's steps and parser checkpoints, so the next parse
    /// can reuse the parts before and after an edit.
    memo: incremental::Memo,
    /// Per token: whether its fingerprint comes from a parse in which its
    /// top-level unit had no errors (see `anchor_history`).  Empty when not
    /// known.
    anchored: Vec<bool>,
    /// Per token: the token before it in its top-level unit, if any (see
    /// `unit_predecessors`), anchored like the roles.  Empty when not known.
    unit_pred: Vec<Option<u32>>,
}

impl PrevParseInfo {
    /// Previous-parse info carrying only token roles, without the steps and
    /// checkpoints `parse_incremental` records for reusing unchanged parts.
    pub fn new(tokens: Vec<PrevToken>, had_errors: bool) -> Self {
        Self {
            tokens,
            had_errors,
            memo: Default::default(),
            anchored: Vec::new(),
            unit_pred: Vec::new(),
        }
    }
}

/// Role-preservation bias applied in the A* search during incremental
/// re-parsing.  When a token's previous `TermType` (Subject/Predicate/Object)
/// agrees with the current parse context, cost is reduced by `strength`
/// (a discount on the otherwise free match); when it conflicts, cost is
/// increased by `strength` (a penalty).
#[derive(Debug, Clone, Copy)]
pub struct IncrementalBias {
    /// Magnitude of the role-agreement bonus and role-conflict penalty.
    /// Agreement: cost -= strength.  Conflict: cost += strength.
    /// Must be > 0; use 0 to disable incremental bias entirely.
    pub strength: isize,
}

impl Default for IncrementalBias {
    fn default() -> Self {
        Self { strength: 1 }
    }
}

/// Search limits and bias for [`parse_incremental_with`].
#[derive(Debug, Clone, Copy)]
pub struct ParseOptions {
    pub bias: IncrementalBias,
    /// Wall-clock budget for the fault-tolerant search.  When it runs out the
    /// search stops and returns the best parse it has: one that reached the
    /// end of the input if any did, otherwise the path that got furthest,
    /// with the remaining tokens as errors.  `None`: no time limit.
    pub time_budget: Option<std::time::Duration>,
    /// Heap pops before the fault-tolerant search stops the same way.
    pub max_iterations: usize,
    /// How the fault-tolerant search recovers from errors: one pruned search
    /// for the cheapest parse (`Engine::Global`, the default), the same
    /// search without pruning (`Engine::Exact`), or a repair per error
    /// (`Engine::Local`).
    pub engine: Engine,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            bias: IncrementalBias::default(),
            time_budget: None,
            max_iterations: a_star::DEFAULT_MAX_ITERATIONS,
            engine: Engine::Global,
        }
    }
}

/// Re-parses `text`, reusing what it can of the previous parse `prev`.
///
/// The parts of the document before and after the edit are not parsed again:
/// the search resumes from a parser checkpoint shortly before the first
/// changed token, and once it reaches the state the previous parse was in at
/// a token after the last change, it completes with the previous parse's
/// steps.  So the work scales with the size of the edit, not the document.
///
/// The edited region is first parsed in fast (non-fault-tolerant) mode; only
/// if that fails does the fault-tolerant search run.  In that case unchanged
/// tokens carry their fingerprint from `prev`, and `bias` steers the search
/// towards parses where they keep their previous role.
pub fn parse_incremental<'a, T: a_star::ParserTrait + Clone + 'static>(
    root: T,
    text: &'a str,
    prev: Option<&PrevParseInfo>,
    bias: IncrementalBias,
) -> (Parse, PrevParseInfo)
where
    T::Kind: Logos<'a, Source = str>,
    <<T as a_star::ParserTrait>::Kind as Logos<'a>>::Extras: Default,
{
    parse_incremental_with(root, text, prev, ParseOptions { bias, ..Default::default() })
}

/// [`parse_incremental`] with search limits: see [`ParseOptions`].
pub fn parse_incremental_with<'a, T: a_star::ParserTrait + Clone + 'static>(
    root: T,
    text: &'a str,
    prev: Option<&PrevParseInfo>,
    options: ParseOptions,
) -> (Parse, PrevParseInfo)
where
    T::Kind: Logos<'a, Source = str>,
    <<T as a_star::ParserTrait>::Kind as Logos<'a>>::Extras: Default,
{
    let deadline = options.time_budget.map(|b| std::time::Instant::now() + b);
    let bias = options.bias;
    let mut tokens = tokenize::<T::Kind>(text);
    let old_texts: Vec<&str> = prev
        .map(|p| p.tokens.iter().map(|t| t.text.as_str()).collect())
        .unwrap_or_default();
    // Borrowed from `text`, not `tokens`, which the search below mutates.
    let new_texts: Vec<&str> = tokens.iter().map(|t| &text[t.range.clone()]).collect();
    let reuse =
        prev.and_then(|p| incremental::Reuse::plan(&p.memo, &old_texts, &new_texts, false));

    let found = a_star::search(
        root.clone(),
        &tokens,
        ParseMode::Fast,
        IncrementalBias { strength: 0 },
        usize::MAX,
        reuse.as_ref(),
        None,
        Engine::Global,
    );
    let (list, memo) = match found {
        Some(found) => found.finish(reuse.as_ref()),
        None => {
            // The fault-tolerant search may also resume after an earlier,
            // unchanged error, so an edit below it is not parsed from there.
            let reuse =
                prev.and_then(|p| incremental::Reuse::plan(&p.memo, &old_texts, &new_texts, true));
            if let Some(prev) = prev {
                let unchanged = unchanged_tokens(&old_texts, &new_texts);
                // The last unchanged non-trivia token so far (old index).
                let mut last_old: Option<usize> = None;
                for (old, new) in unchanged {
                    if !tokens[new].kind.skips() {
                        let pred = prev.unit_pred.get(old).copied().flatten();
                        tokens[new].set_unit_linked(pred.is_some() && pred.map(|p| p as usize) == last_old);
                        last_old = Some(old);
                    }
                    let prev_tok = &prev.tokens[old];
                    tokens[new].set_old_kind(prev_tok.fingerprint);
                    tokens[new].set_old_depth(Some(prev_tok.depth));
                    // Unknown (no anchoring info): count the role as clean,
                    // which keeps the token's full deletion cost.
                    tokens[new].set_clean_role(prev.anchored.get(old).copied().unwrap_or(true));
                }
            }
            a_star::search(
                root,
                &tokens,
                ParseMode::FaultTolerant,
                bias,
                options.max_iterations,
                reuse.as_ref(),
                deadline,
                options.engine,
            )
            .map(|found| found.finish(reuse.as_ref()))
            .unwrap_or_default()
        }
    };
    drop(reuse);

    let parse = Parse::from_steps(&mut tokens, list);
    let had_errors = parse.errors.len() > 0;
    // Each token's bracket depth in this parse (`from_steps` records it for
    // every token the parse consumed), which is what the next search
    // compares against: a bracket the repair inserted counts, a deleted one
    // does not.  The text's own depth stands in for tokens the parse did
    // not consume.
    let mut depth: i32 = 0;
    let mut prev_tokens: Vec<PrevToken> = tokens
        .iter()
        .map(|t| {
            let d = t.old_depth().unwrap_or(depth.clamp(0, 255) as u8);
            depth += t.kind.bracket_delta() as i32;
            t.to_prev_token(d)
        })
        .collect();
    let mut unit_pred = unit_predecessors::<T::Kind>(&parse.green_node, tokens.len()).unwrap_or_default();
    let anchored = match settled_tokens::<T::Kind>(&parse.green_node, tokens.len()) {
        Some(settled) => anchor_history(
            prev,
            &old_texts,
            &new_texts,
            settled,
            &mut prev_tokens,
            &mut unit_pred,
        ),
        None => Vec::new(),
    };
    let prev = PrevParseInfo {
        tokens: prev_tokens,
        had_errors,
        memo,
        anchored,
        unit_pred,
    };
    (parse, prev)
}

/// Per token, trivia included: whether the top-level unit it belongs to (a
/// child of the root, or of the root's only child node, e.g. a Turtle
/// statement) parsed without errors.  `None` if the tree does not hold
/// exactly `n_tokens` tokens.
fn settled_tokens<K: TokenTrait>(green: &rowan::GreenNode, n_tokens: usize) -> Option<Vec<bool>> {
    use rowan::{GreenNodeData, NodeOrToken};
    let error: rowan::SyntaxKind = K::ERROR.into();
    fn has_error(n: &GreenNodeData, error: rowan::SyntaxKind) -> bool {
        n.kind() == error
            || n.children().any(|c| match c {
                NodeOrToken::Node(n) => has_error(n, error),
                NodeOrToken::Token(t) => t.kind() == error,
            })
    }
    fn leaves(n: &GreenNodeData, ok: bool, out: &mut Vec<bool>) {
        for c in n.children() {
            match c {
                NodeOrToken::Node(n) => leaves(n, ok, out),
                NodeOrToken::Token(_) => out.push(ok),
            }
        }
    }
    let mut out = Vec::with_capacity(n_tokens);
    let mut nodes = green.children().filter_map(|c| c.into_node());
    let wrapper = match (nodes.next(), nodes.next()) {
        (Some(only), None) if only.kind() != error => Some(only),
        _ => None,
    };
    for c in green.children() {
        match c {
            NodeOrToken::Node(n) if wrapper.is_some_and(|w| std::ptr::eq(w, n)) => {
                for u in n.children() {
                    match u {
                        NodeOrToken::Node(u) => leaves(u, !has_error(u, error), &mut out),
                        NodeOrToken::Token(t) => out.push(t.kind() != error),
                    }
                }
            }
            NodeOrToken::Node(n) => leaves(n, !has_error(n, error), &mut out),
            NodeOrToken::Token(t) => out.push(t.kind() != error),
        }
    }
    (out.len() == n_tokens).then_some(out)
}

/// Per token, trivia included: the index of the token before it in its
/// top-level unit (a child node of the root, such as a Turtle statement),
/// trivia skipped.  `None` for trivia, for the first token of a unit and for
/// tokens outside any unit (deleted between units).  `None` overall if the
/// tree does not hold exactly `n_tokens` tokens.
fn unit_predecessors<K: TokenTrait>(
    green: &rowan::GreenNode,
    n_tokens: usize,
) -> Option<Vec<Option<u32>>> {
    use rowan::{GreenNodeData, NodeOrToken};
    let error: rowan::SyntaxKind = K::ERROR.into();
    fn leaves<K: TokenTrait>(n: &GreenNodeData, last: &mut Option<u32>, out: &mut Vec<Option<u32>>) {
        for c in n.children() {
            match c {
                NodeOrToken::Node(n) => leaves::<K>(n, last, out),
                NodeOrToken::Token(t) if K::from(t.kind()).skips() => out.push(None),
                NodeOrToken::Token(_) => {
                    out.push(*last);
                    *last = Some(out.len() as u32 - 1);
                }
            }
        }
    }
    let mut out = Vec::with_capacity(n_tokens);
    for c in green.children() {
        match c {
            NodeOrToken::Node(n) if n.kind() != error => leaves::<K>(n, &mut None, &mut out),
            NodeOrToken::Node(n) => out.extend(std::iter::repeat_n(None, count_tokens(n))),
            NodeOrToken::Token(_) => out.push(None),
        }
    }
    (out.len() == n_tokens).then_some(out)
}

fn count_tokens(n: &rowan::GreenNodeData) -> usize {
    n.children()
        .map(|c| match c {
            rowan::NodeOrToken::Node(n) => count_tokens(n),
            rowan::NodeOrToken::Token(_) => 1,
        })
        .sum()
}

/// History that holds up while a statement is broken.  A token whose
/// top-level unit parsed cleanly takes its role from this parse.  A token in
/// a broken unit that is unchanged since the previous version keeps the role
/// it was last given by a clean parse of its unit, if it ever had one;
/// otherwise it takes its role from this parse, as before.  So a bad recovery
/// in one keystroke is not carried into the next ones, and the roles of a
/// statement being edited keep pointing at the last version in which it was
/// whole.  Returns, per token, whether its role is such a clean one.
fn anchor_history(
    prev: Option<&PrevParseInfo>,
    old_texts: &[&str],
    new_texts: &[&str],
    mut anchored: Vec<bool>,
    out: &mut [PrevToken],
    unit_pred: &mut [Option<u32>],
) -> Vec<bool> {
    let Some(prev) = prev.filter(|p| p.anchored.len() == p.tokens.len()) else {
        return anchored;
    };
    let (n, o) = (new_texts.len(), old_texts.len());
    let prefix = old_texts.iter().zip(new_texts).take_while(|(a, b)| a == b).count();
    let suffix = old_texts
        .iter()
        .rev()
        .zip(new_texts.iter().rev())
        .take(o.min(n) - prefix)
        .take_while(|(a, b)| a == b)
        .count();
    for i in 0..n {
        if anchored[i] {
            continue;
        }
        let j = if i < prefix {
            i
        } else if i >= n - suffix {
            i + o - n
        } else {
            continue;
        };
        if prev.anchored[j] {
            out[i].fingerprint = prev.tokens[j].fingerprint;
            out[i].depth = prev.tokens[j].depth;
            // The predecessor keeps its place if it is unchanged too (before
            // or after the edit); otherwise it is gone.
            if let Some(u) = unit_pred.get_mut(i) {
                *u = prev.unit_pred.get(j).copied().flatten().and_then(|p| {
                    let p = p as usize;
                    if p < prefix {
                        Some(p as u32)
                    } else {
                        (p >= o - suffix).then(|| (p + n - o) as u32)
                    }
                });
            }
            anchored[i] = true;
        }
    }
    anchored
}

/// `(old index, new index)` of each token that is unchanged between the
/// previous and the new token list.  Unchanged tokens carry their old
/// fingerprint and depth into the fault-tolerant search, which then prefers
/// parses where they keep their previous role.
fn unchanged_tokens(old_texts: &[&str], new_texts: &[&str]) -> Vec<(usize, usize)> {
    use similar::{Algorithm, DiffOp, capture_diff_slices};
    let mut out = Vec::new();
    for op in capture_diff_slices(Algorithm::Myers, old_texts, new_texts) {
        if let DiffOp::Equal {
            old_index,
            new_index,
            len,
        } = op
        {
            out.extend((0..len).map(|i| (old_index + i, new_index + i)));
        }
    }
    out
}
