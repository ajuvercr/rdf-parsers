use std::collections::HashMap;

use chumsky::prelude::*;

use crate::parser::section_header;

#[derive(Clone, Debug)]
pub struct Rule {
    id: String,
    pub name: String,
    pub expression: Expr,
}

#[derive(Clone, Debug)]
pub enum Mark {
    Option,
    Star,
    Plus,
}

#[derive(Clone, Debug)]
pub enum LiteralType {
    Single,
    Double,
}
#[derive(Clone, Debug)]
pub enum Expr {
    Marked(Box<Self>, Mark),
    Either(Vec<Self>),
    Seq(Vec<Self>),
    Literal(LiteralType, String),
    Reference(String),
}

fn ident<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> + Clone {
    any()
        .filter(|x| char::is_ascii_alphabetic(x) || *x == '_' || char::is_ascii_digit(x))
        .repeated()
        .at_least(1)
        .collect()
}

fn padded<'src>(
    c: char,
) -> impl Parser<'src, &'src str, char, extra::Err<Rich<'src, char>>> + Clone {
    let unit = none_of('*').or(just('*').ignore_then(none_of('/')));
    let start = just("/*");
    let end = just("*/");
    let comment = unit.repeated().delimited_by(start, end);
    just(c).padded().padded_by(comment.padded().repeated())
}

fn rule<'src>() -> impl Parser<'src, &'src str, Rule, extra::Err<Rich<'src, char>>> {
    let n = none_of("]")
        .repeated()
        .collect()
        .delimited_by(padded('['), padded(']'));

    let name = ident().padded();

    n.then(name)
        .then_ignore(just("::=").padded())
        .then(expr())
        .map(|((id, name), expr)| Rule {
            id,
            name: name.to_string(),
            expression: expr,
        })
}

fn expr<'src>() -> impl Parser<'src, &'src str, Expr, extra::Err<Rich<'src, char>>> {
    let mark = choice((
        just('*').to(Mark::Star),
        just('?').to(Mark::Option),
        just('+').to(Mark::Plus),
    ))
    .padded();

    recursive(|a_parser| {
        let grouped = a_parser.clone().delimited_by(padded('('), padded(')'));
        let literal = none_of('\'')
            .repeated()
            .collect()
            .delimited_by(padded('\''), padded('\''))
            .map(|x| Expr::Literal(LiteralType::Single, x))
            .padded();
        let literal2 = none_of('"')
            .repeated()
            .collect()
            .delimited_by(padded('"'), padded('"'))
            .map(|x| Expr::Literal(LiteralType::Double, x))
            .padded();
        let reference = ident().map(Expr::Reference).padded();

        let main = choice((grouped, literal, literal2, reference));

        let main = main
            .clone()
            .then(mark)
            .map(|(e, m)| Expr::Marked(Box::new(e), m))
            .or(main);

        let main = main
            .clone()
            .repeated()
            .at_least(2)
            .collect()
            .map(Expr::Seq)
            .or(main);

        let main = main
            .clone()
            .separated_by(padded('|'))
            .at_least(2)
            .collect()
            .map(Expr::Either)
            .or(main);

        main
    })
}

#[derive(Debug, Clone, Default)]
pub struct Rules {
    pub producing: Vec<Rule>,
    pub terminals: HashMap<String, String>,
}

enum Block {
    Producing(Vec<Rule>),
    Terminals(Vec<(String, String)>),
}

pub fn parse_rules<'src>() -> impl Parser<'src, &'src str, Rules, extra::Err<Rich<'src, char>>> {
    let producing = section_header("producing", "===")
        .ignore_then(rule().repeated().collect())
        .map(Block::Producing);

    let terminal = ident().then_ignore(just("::=")).then(ident());
    let terminals = section_header("terminals", "===")
        .ignore_then(terminal.repeated().collect())
        .map(Block::Terminals);

    let block = producing.or(terminals).repeated().collect();

    section_header("rules", "==")
        .ignore_then(block)
        .map(|x: Vec<_>| {
            let mut ctx = Rules::default();
            for block in x {
                match block {
                    Block::Producing(rules) => ctx.producing.extend(rules),
                    Block::Terminals(items) => ctx.terminals.extend(items),
                }
            }
            ctx
        })
}
