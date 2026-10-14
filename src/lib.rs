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
pub use a_star::last_iterations;
pub mod list;
pub mod model;
pub mod format;
#[macro_use]
mod rdf_format;
pub mod jsonld;
pub mod n3;
pub mod ntriples;
mod parser;
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

    /// Returns the minimum error cost this rule must incur when `tok` is the
    /// current token.  Returns 0 if `tok` is reachable anywhere inside the
    /// rule or the rule is nullable (can match zero tokens).  Returns a
    /// positive value otherwise — the rule cannot make progress without
    /// producing at least that much error cost.
    ///
    /// Used by `add_element_checked` to pre-charge elements before adding
    /// them to the A* search.  Defaults to 0 (no pre-charging).
    fn min_error_for_token(&self, _tok: &Self) -> isize {
        0
    }


    /// Maximum `error_value` for this token kind across all grammar rules that
    /// may match it.
    ///
    /// Must always return > 0: the A* cost model adds `error_value` on a miss
    /// and nothing on a match, so matching is strictly cheaper when
    /// `error_value > 0`.
    /// Defaults to 2, matching the minimum terminal weight used in codegen.
    fn max_error_value(&self) -> isize {
        2
    }

    /// Minimum cost to complete this rule via insertions.  This is the sum of
    /// error_values for all required terminals on the cheapest path through the
    /// rule.  Used by `add_element_checked` to pre-charge elements that push
    /// a rule the current token cannot start.  Defaults to `max_error_value()`.
    fn min_completion_cost(&self) -> isize {
        self.max_error_value()
    }

    /// Cost of deleting (skipping) this token during error recovery.
    ///
    /// Defaults to `5 × max_error_value()`.  Must be high enough that
    /// matching tokens is always cheaper than deleting them, while still
    /// cheaper than long cascades of error insertions.
    fn deletion_cost(&self) -> isize {
        self.max_error_value() * 5
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
}

impl PrevParseInfo {
    /// Previous-parse info carrying only token roles, without the steps and
    /// checkpoints `parse_incremental` records for reusing unchanged parts.
    pub fn new(tokens: Vec<PrevToken>, had_errors: bool) -> Self {
        Self {
            tokens,
            had_errors,
            memo: Default::default(),
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
    let mut tokens = tokenize::<T::Kind>(text);
    let old_texts: Vec<&str> = prev
        .map(|p| p.tokens.iter().map(|t| t.text.as_str()).collect())
        .unwrap_or_default();
    let new_texts: Vec<&str> = tokens.iter().map(|t| t.text()).collect();
    let reuse =
        prev.and_then(|p| incremental::Reuse::plan(&p.memo, &old_texts, &new_texts));

    let found = a_star::search(
        root.clone(),
        &tokens,
        ParseMode::Fast,
        IncrementalBias { strength: 0 },
        usize::MAX,
        reuse.as_ref(),
    );
    let (list, memo) = match found {
        Some(found) => found.finish(reuse.as_ref()),
        None => {
            if let Some(prev) = prev {
                let unchanged = unchanged_tokens(&old_texts, &new_texts);
                for (old, new) in unchanged {
                    let prev_tok = &prev.tokens[old];
                    tokens[new].set_old_kind(prev_tok.fingerprint);
                    tokens[new].set_old_depth(Some(prev_tok.depth));
                }
            }
            a_star::search(
                root,
                &tokens,
                ParseMode::FaultTolerant,
                bias,
                a_star::DEFAULT_MAX_ITERATIONS,
                reuse.as_ref(),
            )
            .map(|found| found.finish(reuse.as_ref()))
            .unwrap_or_default()
        }
    };
    drop(reuse);

    let parse = Parse::from_steps(&mut tokens, list);
    let had_errors = parse.errors.len() > 0;
    // Compute bracket depths for the new token sequence to populate PrevParseInfo.
    let mut depth: i32 = 0;
    let prev = PrevParseInfo {
        tokens: tokens
            .iter()
            .map(|t| {
                let d = depth.clamp(0, 255) as u8;
                depth += t.kind.bracket_delta() as i32;
                t.to_prev_token(d)
            })
            .collect(),
        had_errors,
        memo,
    };
    (parse, prev)
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
