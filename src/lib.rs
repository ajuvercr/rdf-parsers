use std::borrow::Borrow;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use logos::{Lexer, Logos};
pub use parser::*;

mod a_star;
pub use a_star::Fingerprint;
pub use a_star::ParseMode;
pub use a_star::ParserTrait;
pub mod list;
pub mod model;
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

    /// The set of every terminal that can be consumed *anywhere* in a parse of
    /// this rule — including inside nested sub-rules at any depth.
    ///
    /// Used by the A* to prune parser elements whose current token cannot
    /// appear anywhere in the rule's subtree: if the set is non-empty and the
    /// current token is absent, the path must produce at least one error.
    ///
    /// Returns `&[]` (no pruning) by default; generated parsers override this.
    fn all_reachable_tokens(&self) -> &'static [Self] {
        &[]
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
    let mut tokens = Vec::new();
    while let Some(t) = lexer.next() {
        let kind = t.unwrap_or(K::ERROR);
        tokens.push(FatToken::new(
            kind,
            lexer.span(),
            text[lexer.span()].to_string(),
        ));
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
pub struct PrevParseInfo {
    pub tokens: Vec<PrevToken>,
    /// Whether the previous parse produced any errors.  When true, stored
    /// bracket depths are not used for conflict classification (they may be
    /// misleading due to error-recovery changing the grammar context), so
    /// fingerprint conflicts are treated as free-pass rather than +50 role
    /// conflicts.
    pub had_errors: bool,
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

/// Like `parse_t_2` but, when `prev` is provided, diffs the token stream
/// against the previous one and copies each old token's parse-time fingerprint
/// onto the matching new token via `FatToken::set_old_kind`.  The A* scorer
/// then uses `bias` to adjust scores for parses that agree or disagree with
/// the previous token positions in the grammar rule stack.
pub fn parse_incremental<'a, T: a_star::ParserTrait + 'static>(
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

    if let Some(prev) = prev {
        // Build text slices for the differ.
        let old_texts: Vec<&str> = prev.tokens.iter().map(|t| t.text.as_str()).collect();
        let new_texts: Vec<&str> = tokens.iter().map(|t| t.text()).collect();

        // Map new token index → old token index for Equal (unchanged) regions.
        use similar::{Algorithm, DiffOp, capture_diff_slices};
        let ops = capture_diff_slices(Algorithm::Myers, &old_texts, &new_texts);
        let mut new_to_old = std::collections::HashMap::<usize, usize>::new();
        for op in &ops {
            if let DiffOp::Equal {
                old_index,
                new_index,
                len,
            } = op
            {
                for i in 0..*len {
                    new_to_old.insert(new_index + i, old_index + i);
                }
            }
        }

        // Copy the old fingerprint and depth onto each unchanged new token.
        // When the previous parse had errors, skip copying depth: error-
        // recovery may have placed tokens at grammatically misleading depths,
        // and we don't want the +50 same-depth bias to fire on them.
        for (new_idx, tok) in tokens.iter_mut().enumerate() {
            if let Some(&old_idx) = new_to_old.get(&new_idx) {
                if let Some(prev_tok) = prev.tokens.get(old_idx) {
                    tok.set_old_kind(prev_tok.fingerprint);
                    if !prev.had_errors {
                        tok.set_old_depth(Some(prev_tok.depth));
                    }
                }
            }
        }
    }

    let list = a_star::a_star(root, &tokens, bias, a_star::DEFAULT_MAX_ITERATIONS);
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
    };
    (parse, prev)
}
