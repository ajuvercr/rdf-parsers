use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::{Deref, DerefMut, Range};
use std::{collections::HashSet, fmt::Debug};

use crate::list::Inner;
use logos::{Lexer, Logos};
pub use parser::*;

mod a_star;
mod list;
mod parser;
pub mod sparql;
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

pub trait TokenTrait:
    Debug + Clone + Into<rowan::SyntaxKind> + PartialEq + Eq + Hash + 'static
{
    const ERROR: Self;
    const ROOT: Self;

    fn branch(&self) -> u32;

    fn skips(&self) -> bool;

    fn starting_tokens(&self) -> &'static [Self];
    fn ending_tokens(&self) -> &'static [Self];
}

pub struct Context {
    suggestions: HashSet<(String, Range<usize>)>,
}

/// Second, implementing the `Language` trait teaches rowan to convert between
/// these two SyntaxKind types, allowing for a nicer SyntaxNode API where
/// "kinds" are values from our `enum SyntaxKind`, instead of plain u16 values.

fn tokenize<'a, K>(text: &'a str) -> Vec<FatToken<K>>
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
    let list = a_star::a_star(root, &tokens);
    Parse::from_steps(&tokens, list)
}
