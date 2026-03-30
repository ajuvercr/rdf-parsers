use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::{Deref, DerefMut, Range};
use std::{collections::HashSet, fmt::Debug};

use crate::list::Inner;
use logos::{Lexer, Logos};
pub use parser::*;

mod a_star;
mod list;
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
}

pub trait ParserTrait {
    type Kind: 'static + TokenTrait;
    const KIND: Self::Kind;
    const CAN_BE_EMPTY: bool;
    const FIRST_ITEMS: &'static [Self::Kind];
    const LAST_ITEMS: &'static [Self::Kind];
    fn parse(parser: &mut crate::Parser<Self::Kind>, context: &mut Context);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermType {
    Subject,
    Predicate,
    Object,
}

pub trait TokenTrait:
    Debug + Clone + Into<rowan::SyntaxKind> + PartialEq + Eq + Hash + 'static
{
    const ERROR: Self;
    const ROOT: Self;

    fn branch(&self) -> u32;

    fn skips(&self) -> bool;

    fn starting_tokens(&self) -> &'static [Self];
    fn ending_tokens(&self) -> &'static [Self];

    fn is_significant(&self) -> bool {
        self.term_type().is_some()
    }

    fn term_type(&self) -> Option<TermType>;

    /// Maximum `error_value` for this token kind across all grammar rules that
    /// may match it.  Used to compute the A* suffix-sum heuristic.
    ///
    /// Must always return > 0: the A* cost model requires `error_value > 0`
    /// so that matching is strictly cheaper than skipping.
    /// Defaults to 2, matching the minimum terminal weight used in codegen.
    fn max_error_value(&self) -> isize {
        2
    }
}

pub struct Context {
    suggestions: HashSet<(String, Range<usize>)>,
}

/// Second, implementing the `Language` trait teaches rowan to convert between
/// these two SyntaxKind types, allowing for a nicer SyntaxNode API where
/// "kinds" are values from our `enum SyntaxKind`, instead of plain u16 values.

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

pub fn parse_t<'a, T: ParserTrait>(text: &'a str) -> Parse
where
    T::Kind: Logos<'a, Source = str>,
    <<T as ParserTrait>::Kind as Logos<'a>>::Extras: Default,
{
    let tokens = tokenize::<T::Kind>(text);
    let tokens = tokens
        .into_iter()
        .rfold(Inner::new(), |acc, b| acc.prepend(b));
    Parser::new(tokens).parse_item::<T>()
}

pub fn parse_t_2<'a, T: a_star::ParserTrait + 'static>(root: T, text: &'a str) -> Parse
where
    T::Kind: Logos<'a, Source = str>,
    <<T as a_star::ParserTrait>::Kind as Logos<'a>>::Extras: Default,
{
    let tokens = tokenize::<T::Kind>(text);
    let list = a_star::a_star(root, &tokens, IncrementalBias::default());
    Parse::from_steps(&tokens, list)
}

/// Information from a previous parse needed for incremental re-parsing.
pub struct PrevParseInfo<K: TokenTrait> {
    pub tokens: Vec<FatToken<K>>,
    pub prev_roles: Vec<Option<TermType>>,
}

/// Role-preservation bias applied in the A* search during incremental
/// re-parsing.  When a token's previous `TermType` (Subject/Predicate/Object)
/// agrees with the current parse context, cost is reduced by `strength`;
/// when it conflicts, cost is increased by `strength`.
///
/// `strength` must be > 0 for the bias to have any effect, and should exceed
/// the default token weight (2) so that a single role agreement outweighs a
/// single missing token.  The default of 5 means one agreeing node saves more
/// than twice the cost of any missing token at the default weight.
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
/// against the previous one and copies each old token's `TermType` onto the
/// matching new token via `FatToken::set_old_kind`.  The A* scorer then uses
/// `bias` to adjust scores for parses that agree or disagree with the
/// previous token roles.
pub fn parse_t_2_incremental<'a, T: a_star::ParserTrait + 'static>(
    root: T,
    text: &'a str,
    prev: Option<&PrevParseInfo<T::Kind>>,
    bias: IncrementalBias,
) -> Parse
where
    T::Kind: Logos<'a, Source = str>,
    <<T as a_star::ParserTrait>::Kind as Logos<'a>>::Extras: Default,
{
    let mut tokens = tokenize::<T::Kind>(text);

    if let Some(prev) = prev {
        // Build text slices for the differ.
        let old_texts: Vec<&str> = prev.tokens.iter().map(|t| t.text()).collect();
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

        // Copy the old TermType onto each unchanged new token.
        for (new_idx, tok) in tokens.iter_mut().enumerate() {
            if let Some(&old_idx) = new_to_old.get(&new_idx) {
                tok.set_old_kind(prev.prev_roles.get(old_idx).cloned().flatten());
            }
        }
    }

    let list = a_star::a_star(root, &tokens, bias);
    Parse::from_steps(&tokens, list)
}
