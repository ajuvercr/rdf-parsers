use std::fmt::Debug;

use chumsky::prelude::*;

use crate::parser::section_header;

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Rule {
    id: String,
    pub name: String,
    pub expression: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mark {
    Option,
    Star,
    Plus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiteralType {
    Single,
    Double,
    Regex,
    Hex,
}
#[derive(Clone, PartialEq, Eq)]
pub enum Expr {
    Marked(Box<Self>, Mark),
    Either(Vec<Self>),
    Seq(Vec<Self>),
    Literal(LiteralType, String),
    Reference(String),
}
impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Marked(arg0, Mark::Option) => write!(f, "({:?})?", arg0),
            Self::Marked(arg0, Mark::Star) => write!(f, "({:?})*", arg0),
            Self::Marked(arg0, Mark::Plus) => write!(f, "({:?})+", arg0),
            Self::Either(arg0) => {
                write!(f, "( {:?}", arg0[0])?;
                for a in &arg0[1..] {
                    write!(f, " | {:?}", a)?;
                }

                write!(f, " )")?;
                Ok(())
            }
            Self::Seq(arg0) => {
                write!(f, "{:?}", arg0[0])?;
                for a in &arg0[1..] {
                    write!(f, " {:?}", a)?;
                }
                Ok(())
            }
            Self::Literal(_, arg1) => write!(f, "\"{}\"", arg1),
            Self::Reference(arg0) => write!(f, "{}", arg0),
        }
    }
}

fn hex<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> + Clone {
    just("#x").ignore_then(one_of("0123456789ABCDEF").repeated().collect())
}

fn ws<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Rich<'src, char>>> + Clone {
    one_of(" \t").repeated().ignored()
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
    just(c)
        .padded_by(ws())
        .padded_by(comment.padded_by(ws()).repeated())
}

fn rule<'src>() -> impl Parser<'src, &'src str, Rule, extra::Err<Rich<'src, char>>> {
    let n = none_of("]")
        .repeated()
        .collect()
        .delimited_by(just('\n').or_not().ignore_then(just('[')), just(']'));

    let name = ident().padded_by(ws());

    n.then(name)
        .then_ignore(just("::=").padded_by(ws()))
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
    .padded_by(ws());

    recursive(|a_parser| {
        let grouped = a_parser.clone().delimited_by(padded('('), padded(')'));
        let literal = none_of('\'')
            .repeated()
            .collect()
            .delimited_by(padded('\''), padded('\''))
            .map(|x| Expr::Literal(LiteralType::Single, x))
            .padded_by(ws());
        let literal2 = none_of('"')
            .repeated()
            .collect()
            .delimited_by(padded('"'), padded('"'))
            .map(|x| Expr::Literal(LiteralType::Double, x))
            .padded_by(ws());

        let literal3 = none_of(']')
            .repeated()
            .collect()
            .delimited_by(padded('['), padded(']'))
            .map(|x| Expr::Literal(LiteralType::Regex, x))
            .padded_by(ws());

        let literal4 = hex()
            .map(|x| Expr::Literal(LiteralType::Hex, x))
            .padded_by(ws());
        let reference = ident().map(Expr::Reference).padded_by(ws());

        let main = choice((grouped, literal, literal2, literal3, literal4, reference));

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
            .separated_by(just('|').padded_by(one_of(" \t\n\r").repeated()))
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
    pub terminals: Vec<Rule>,
}

enum Block {
    Producing(Vec<Rule>),
    Terminals(Vec<Rule>),
}

pub fn parse_rules<'src>() -> impl Parser<'src, &'src str, Rules, extra::Err<Rich<'src, char>>> {
    let producing = section_header("producing", "===")
        .ignore_then(rule().repeated().collect())
        .map(Block::Producing);

    let terminals = section_header("terminals", "===")
        .ignore_then(rule().repeated().collect())
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
