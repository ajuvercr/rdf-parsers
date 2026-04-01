use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::io::Cursor;
use std::ops::Range;

use ariadne::{ColorGenerator, Config, Label, Report, ReportBuilder, ReportKind, Source};
use rowan::{NodeOrToken, SyntaxElement};
use wasm_bindgen::prelude::*;

use turtle::{
    IncrementalBias, Parse, PrevParseInfo, TokenTrait,
    ntriples::parser::{
        Lang as NTriplesLang, Rule as NTriplesRule, SyntaxKind as NTriplesSyntaxKind,
    },
    parse_t_2_incremental,
    sparql::parser::{Lang as SparqlLang, Rule as SparqlRule, SyntaxKind as SparqlSyntaxKind},
    trig::parser::{Lang as TrigLang, Rule as TrigRule, SyntaxKind as TrigSyntaxKind},
    turtle::parser::{Lang, Rule, SyntaxKind},
};

thread_local! {
    static PREV_TURTLE:   RefCell<Option<PrevParseInfo<SyntaxKind>>>         = RefCell::new(None);
    static PREV_SPARQL:   RefCell<Option<PrevParseInfo<SparqlSyntaxKind>>>   = RefCell::new(None);
    static PREV_TRIG:     RefCell<Option<PrevParseInfo<TrigSyntaxKind>>>     = RefCell::new(None);
    static PREV_NTRIPLES: RefCell<Option<PrevParseInfo<NTriplesSyntaxKind>>> = RefCell::new(None);
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Walk a finished A* `Parse` and return `(byte_range, message)` pairs for
/// every Error node in source order.
fn astar_pairs_from_parse<L>(parse: &Parse, text: &str) -> Vec<(Range<usize>, String)>
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
            NodeOrToken::Token(t) if !t.kind().skips() => Some(usize::from(t.text_range().end())),
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

/// Recursively format a syntax node/token into `out`, annotating Error nodes
/// with their error message.  `error_msgs` maps a node's tree-order index
/// among all Error nodes to its message string.
fn format_element<L>(
    elem: SyntaxElement<L>,
    error_msgs: &HashMap<usize, String>,
    error_idx: &mut usize,
    depth: usize,
    out: &mut String,
) where
    L: rowan::Language,
    L::Kind: TokenTrait + std::fmt::Debug,
{
    let indent = "  ".repeat(depth);
    match elem {
        NodeOrToken::Node(n) => {
            let range = n.text_range();
            let is_error = n.kind() == L::Kind::ERROR;
            let _ = write!(
                out,
                "{}{:?}@{}..{}",
                indent,
                n.kind(),
                u32::from(range.start()),
                u32::from(range.end()),
            );
            if is_error {
                if let Some(msg) = error_msgs.get(error_idx) {
                    let _ = write!(out, "  // {}", msg);
                }
                *error_idx += 1;
            }
            out.push('\n');
            for child in n.children_with_tokens() {
                format_element(child, error_msgs, error_idx, depth + 1, out);
            }
        }
        NodeOrToken::Token(t) => {
            let range = t.text_range();
            let text = t.text();
            if text.len() > 40 {
                let _ = writeln!(
                    out,
                    "{}{:?}@{}..{}  \"{} ...\"",
                    indent,
                    t.kind(),
                    u32::from(range.start()),
                    u32::from(range.end()),
                    &text[..40],
                );
            } else {
                let _ = writeln!(
                    out,
                    "{}{:?}@{}..{}  {:?}",
                    indent,
                    t.kind(),
                    u32::from(range.start()),
                    u32::from(range.end()),
                    text,
                );
            }
        }
    }
}

fn render_ast<L>(parse: &Parse, pairs: &[(Range<usize>, String)]) -> String
where
    L: rowan::Language,
    L::Kind: TokenTrait + std::fmt::Debug,
{
    // pairs[i] corresponds to the i-th Error node in pre-order traversal.
    let error_msgs: HashMap<usize, String> = pairs
        .iter()
        .enumerate()
        .map(|(i, (_, msg))| (i, msg.clone()))
        .collect();

    let root = parse.syntax::<L>();
    let mut out = String::new();
    format_element(NodeOrToken::Node(root), &error_msgs, &mut 0, 0, &mut out);
    out
}

#[wasm_bindgen]
pub fn parse(language: &str, text: &str) -> String {
    let loc = "input";
    let bias = IncrementalBias::default();
    match language {
        "turtle" => {
            let (parse, tokens) = PREV_TURTLE.with(|prev| {
                let p = prev.borrow();
                parse_t_2_incremental(Rule::new(SyntaxKind::TurtleDoc), text, p.as_ref(), bias)
            });
            let pairs = astar_pairs_from_parse::<Lang>(&parse, text);
            PREV_TURTLE.with(|prev| {
                *prev.borrow_mut() = Some(PrevParseInfo { tokens });
            });
            render_ariadne(&pairs, text, loc)
        }
        "sparql" => {
            let (parse, tokens) = PREV_SPARQL.with(|prev| {
                let p = prev.borrow();
                parse_t_2_incremental(
                    SparqlRule::new(SparqlSyntaxKind::QueryUnit),
                    text,
                    p.as_ref(),
                    bias,
                )
            });
            let pairs = astar_pairs_from_parse::<SparqlLang>(&parse, text);
            PREV_SPARQL.with(|prev| {
                *prev.borrow_mut() = Some(PrevParseInfo { tokens });
            });
            render_ariadne(&pairs, text, loc)
        }
        "trig" => {
            let (parse, tokens) = PREV_TRIG.with(|prev| {
                let p = prev.borrow();
                parse_t_2_incremental(
                    TrigRule::new(TrigSyntaxKind::TrigDoc),
                    text,
                    p.as_ref(),
                    bias,
                )
            });
            let pairs = astar_pairs_from_parse::<TrigLang>(&parse, text);
            PREV_TRIG.with(|prev| {
                *prev.borrow_mut() = Some(PrevParseInfo { tokens });
            });
            render_ariadne(&pairs, text, loc)
        }
        "ntriples" => {
            let (parse, tokens) = PREV_NTRIPLES.with(|prev| {
                let p = prev.borrow();
                parse_t_2_incremental(
                    NTriplesRule::new(NTriplesSyntaxKind::NtriplesDoc),
                    text,
                    p.as_ref(),
                    bias,
                )
            });
            let pairs = astar_pairs_from_parse::<NTriplesLang>(&parse, text);
            PREV_NTRIPLES.with(|prev| {
                *prev.borrow_mut() = Some(PrevParseInfo { tokens });
            });
            render_ariadne(&pairs, text, loc)
        }
        _ => String::from("Unknown language"),
    }
}

#[wasm_bindgen]
pub fn parse_ast(language: &str, text: &str) -> String {
    let bias = IncrementalBias::default();
    match language {
        "turtle" => {
            let (parse, _) = PREV_TURTLE.with(|prev| {
                let p = prev.borrow();
                parse_t_2_incremental(Rule::new(SyntaxKind::TurtleDoc), text, p.as_ref(), bias)
            });
            let pairs = astar_pairs_from_parse::<Lang>(&parse, text);
            render_ast::<Lang>(&parse, &pairs)
        }
        "sparql" => {
            let (parse, _) = PREV_SPARQL.with(|prev| {
                let p = prev.borrow();
                parse_t_2_incremental(
                    SparqlRule::new(SparqlSyntaxKind::QueryUnit),
                    text,
                    p.as_ref(),
                    bias,
                )
            });
            let pairs = astar_pairs_from_parse::<SparqlLang>(&parse, text);
            render_ast::<SparqlLang>(&parse, &pairs)
        }
        "trig" => {
            let (parse, _) = PREV_TRIG.with(|prev| {
                let p = prev.borrow();
                parse_t_2_incremental(
                    TrigRule::new(TrigSyntaxKind::TrigDoc),
                    text,
                    p.as_ref(),
                    bias,
                )
            });
            let pairs = astar_pairs_from_parse::<TrigLang>(&parse, text);
            render_ast::<TrigLang>(&parse, &pairs)
        }
        "ntriples" => {
            let (parse, _) = PREV_NTRIPLES.with(|prev| {
                let p = prev.borrow();
                parse_t_2_incremental(
                    NTriplesRule::new(NTriplesSyntaxKind::NtriplesDoc),
                    text,
                    p.as_ref(),
                    bias,
                )
            });
            let pairs = astar_pairs_from_parse::<NTriplesLang>(&parse, text);
            render_ast::<NTriplesLang>(&parse, &pairs)
        }
        _ => String::from("Unknown language"),
    }
}
