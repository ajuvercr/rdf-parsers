use std::io::Cursor;
use std::ops::Range;

use ariadne::{ColorGenerator, Config, Label, Report, ReportBuilder, ReportKind, Source};
use rowan::NodeOrToken;
use wasm_bindgen::prelude::*;

use turtle::{
    Parse, TokenTrait, parse_t_2,
    ntriples::parser::{Lang as NTriplesLang, Rule as NTriplesRule, SyntaxKind as NTriplesSyntaxKind},
    sparql::parser::{Lang as SparqlLang, Rule as SparqlRule, SyntaxKind as SparqlSyntaxKind},
    trig::parser::{Lang as TrigLang, Rule as TrigRule, SyntaxKind as TrigSyntaxKind},
    turtle::parser::{Lang, Rule, SyntaxKind},
};

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Walk a finished A* `Parse` and return `(byte_range, message)` pairs for
/// every Error node in source order.
fn astar_pairs_from_parse<L>(parse: Parse, text: &str) -> Vec<(Range<usize>, String)>
where
    L: rowan::Language,
    L::Kind: TokenTrait,
{
    let msgs: Vec<String> = parse
        .errors
        .iter()
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let root = parse.syntax::<L>();

    let mut token_ends: Vec<usize> = root
        .descendants_with_tokens()
        .filter_map(|nt| match nt {
            NodeOrToken::Token(t) if !t.kind().skips() => {
                Some(usize::from(t.text_range().end()))
            }
            _ => None,
        })
        .collect();
    token_ends.sort_unstable();

    let ranges: Vec<Range<usize>> = root
        .descendants()
        .filter(|n| n.kind() == L::Kind::ERROR)
        .map(|n| {
            let pos = usize::from(n.text_range().start());
            let prev_end = token_ends.partition_point(|&e| e <= pos);
            if prev_end > 0 {
                let end = token_ends[prev_end - 1].min(text.len());
                let next_char_end = text[end..]
                    .char_indices()
                    .next()
                    .map(|(i, c)| end + i + c.len_utf8())
                    .unwrap_or(end + 1)
                    .min(text.len());
                end..next_char_end
            } else {
                let end = text[pos..]
                    .char_indices()
                    .next()
                    .map(|(i, c)| pos + i + c.len_utf8())
                    .unwrap_or(pos + 1)
                    .min(text.len());
                pos..end
            }
        })
        .collect();

    ranges.into_iter().zip(msgs).collect()
}

fn render_ariadne(errors: &[(Range<usize>, String)], source: &str, loc: &str) -> String {
    if errors.is_empty() {
        return String::from("No errors \u{2713}");
    }

    // ariadne with auto-color calls concolor::get(Stream::Stderr).color() to decide
    // whether to emit ANSI codes — not Config::with_color and not yansi's global state.
    // Force it on via concolor's api feature.
    concolor::set(concolor::ColorChoice::Always);

    let s = errors.iter().map(|x| x.0.start).min().unwrap_or_default();
    let e = errors.iter().map(|x| x.0.end).max().unwrap_or_default();

    let mut colors = ColorGenerator::from_state([10000, 15000, 15000], 0.8);

    let report = errors.iter().rev().fold(
        Report::build(ReportKind::Error, (loc, s..e))
            .with_config(Config::default().with_color(true)),
        |report: ReportBuilder<(&str, Range<usize>)>, (span, msg)| {
            report.with_label(
                Label::new((loc, span.clone()))
                    .with_message(msg)
                    .with_color(colors.next()),
            )
        },
    );

    let mut out = Cursor::new(Vec::<u8>::new());
    let _ = report.finish().write((loc, Source::from(source)), &mut out);
    String::from_utf8(out.into_inner()).unwrap_or_default()
}

#[wasm_bindgen]
pub fn parse(language: &str, text: &str) -> String {
    let loc = "input";
    match language {
        "turtle" => {
            let parse = parse_t_2(Rule::new(SyntaxKind::TurtleDoc), text);
            let pairs = astar_pairs_from_parse::<Lang>(parse, text);
            render_ariadne(&pairs, text, loc)
        }
        "sparql" => {
            let parse = parse_t_2(SparqlRule::new(SparqlSyntaxKind::QueryUnit), text);
            let pairs = astar_pairs_from_parse::<SparqlLang>(parse, text);
            render_ariadne(&pairs, text, loc)
        }
        "trig" => {
            let parse = parse_t_2(TrigRule::new(TrigSyntaxKind::TrigDoc), text);
            let pairs = astar_pairs_from_parse::<TrigLang>(parse, text);
            render_ariadne(&pairs, text, loc)
        }
        "ntriples" => {
            let parse = parse_t_2(NTriplesRule::new(NTriplesSyntaxKind::NtriplesDoc), text);
            let pairs = astar_pairs_from_parse::<NTriplesLang>(parse, text);
            render_ariadne(&pairs, text, loc)
        }
        _ => String::from("Unknown language"),
    }
}
