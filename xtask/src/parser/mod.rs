mod context;
pub use context::{Context, context_parser};
mod expr;
use expr::parse_rules;
pub use expr::{Expr, Mark, Rule, Rules};

use chumsky::{extra::Err, prelude::*};

pub struct Config {
    pub rules: Rules,
    pub context: Context,
}

pub fn section_header<'src>(
    name: &'static str,
    padding: &'static str,
) -> impl Parser<'src, &'src str, (), Err<Rich<'src, char>>> {
    just(name)
        .delimited_by(just(padding).padded(), just(padding).padded())
        .to(())
}

pub fn parse<'a>(inp: &'a str) -> Result<Config, Vec<Rich<'a, char>>> {
    let parser = parse_rules()
        .then(context_parser())
        .map(|(rules, context)| Config { rules, context });
    let output = parser.parse(inp);
    output.into_result()
}
