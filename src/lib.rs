use std::hash::Hash;
use std::ops::Range;
use std::{collections::HashSet, fmt::Debug};

use crate::a_star::Element;
use crate::list::{Inner, List};
use logos::{Lexer, Logos};
pub use parser::*;

mod a_star;
mod impls;
pub use impls::sparql;
pub use impls::turtle;
mod list;
mod parser;
pub mod util;

pub trait ParserTrait {
    type Kind: 'static + TokenTrait;
    const KIND: Self::Kind;
    const CAN_BE_EMPTY: bool;
    const FIRST_ITEMS: &'static [Self::Kind];
    const LAST_ITEMS: &'static [Self::Kind];
    fn parse(parser: &mut crate::Parser<Self::Kind>, context: &mut Context);
}

pub trait TokenTrait: Debug + Clone + Into<rowan::SyntaxKind> + PartialEq + Hash + 'static {
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

pub fn parse_t<'a, T: ParserTrait>(text: &'a str) -> Parse
where
    T::Kind: Logos<'a, Source = str>,
    <<T as ParserTrait>::Kind as Logos<'a>>::Extras: Default,
{
    let mut lexer: Lexer<'a, T::Kind> = Lexer::new(text);

    let mut tokens: Vec<FatToken<T::Kind>> = Vec::default();

    while let Some(t) = lexer.next() {
        let kind = t.unwrap_or(T::Kind::ERROR);

        let s = text[lexer.span()].to_string();
        tokens.push(FatToken::new(kind, lexer.span(), s));
    }

    let ts: Vec<_> = tokens.iter().map(|x| &x.kind).collect();
    println!("tokens {:?}", ts);

    let tokens = tokens
        .into_iter()
        .rfold(Inner::new(), |acc, b| acc.prepend(b));

    // let tokens = lex(text);
    Parser::new(tokens).parse_item::<T>()
}

pub fn parse_t_2<'a, T: a_star::ParserTraitConsts>(text: &'a str) -> Parse
where
    T::Kind: Logos<'a, Source = str>,
    <<T as a_star::ParserTrait>::Kind as Logos<'a>>::Extras: Default,
{
    let mut lexer: Lexer<'a, T::Kind> = Lexer::new(text);

    let mut tokens: Vec<FatToken<T::Kind>> = Vec::default();

    while let Some(t) = lexer.next() {
        let kind = t.unwrap_or(T::Kind::ERROR);

        let s = text[lexer.span()].to_string();
        tokens.push(FatToken::new(kind, lexer.span(), s));
    }

    let ts: Vec<_> = tokens.iter().map(|x| &x.kind).collect();
    println!("tokens {:?}", ts);

    let list = a_star::a_star(T::new(), &tokens);

    Parse::from_steps(&tokens, list)
}
