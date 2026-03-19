use std::ops::Range;

use ariadne::{ColorGenerator, Label, Report, ReportBuilder, ReportKind, Source};
use benchmark::Fixture;
use lang_turtle::lang::{
    context::{Context, TokenIdx},
    parser::parse_turtle as chumsky_parse_turtle,
    tokenizer::parse_tokens_str as chumsky_parse_tokens_str,
};
use lsp_core::util::Spanned as ChumskySpanned;
use lsp_core::util::token::Token as ChumskyToken;
use rowan::NodeOrToken;
use tower_lsp::lsp_types::Url;
use turtle::{
    IncrementalBias, Parse, PrevParseInfo, TokenTrait as _, extract_term_types, parse_t_2,
    parse_t_2_incremental, tokenize,
    turtle::parser::{Lang, Rule, SyntaxKind},
};

// ── ariadne output ────────────────────────────────────────────────────────────

fn print_ariadne(errors: &[(Range<usize>, String)], source: &str, loc: &str) {
    if errors.is_empty() {
        println!("  No errors.");
        return;
    }

    let s = errors.iter().map(|x| x.0.start).min().unwrap_or_default();
    let e = errors.iter().map(|x| x.0.end).max().unwrap_or_default();

    let mut colors = ColorGenerator::from_state([10000, 15000, 15000], 0.8);

    let report = errors.iter().rev().fold(
        Report::build(ReportKind::Error, (loc, s..e)),
        |report: ReportBuilder<(&str, Range<usize>)>, (span, msg): &(Range<usize>, String)| {
            let (s, e) = (span.start, span.end);
            report.with_label(
                Label::new((loc, s..e))
                    .with_message(msg)
                    .with_color(colors.next()),
            )
        },
    );

    report.finish().print((loc, Source::from(source))).unwrap();
}

// ── A* helpers ────────────────────────────────────────────────────────────────

fn astar_build_prev(text: &str) -> PrevParseInfo<SyntaxKind> {
    let parse = parse_t_2(Rule::new(SyntaxKind::TurtleDoc), text);
    let root = parse.syntax::<Lang>();
    let tokens = tokenize::<SyntaxKind>(text);
    let term_types = extract_term_types(&root, |k: SyntaxKind| k.term_type());
    PrevParseInfo { tokens, term_types }
}

/// Walk a finished A* `Parse` and return `(byte_range, message)` pairs for
/// every Error node in source order. Error coalescing (grouping per-rule rather
/// than per-token) already happened inside `Parse::from_steps`.
fn astar_pairs_from_parse(parse: Parse, text: &str) -> Vec<(Range<usize>, String)> {
    // List is prepend-ordered (newest first); reverse to match DFS order.
    let msgs: Vec<String> = parse
        .errors
        .iter()
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let root = parse.syntax::<Lang>();

    // Collect all non-whitespace token end positions, used to point the span
    // just after the last real token before each Error node.
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
        .filter(|n| n.kind() == SyntaxKind::Error)
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

// ── chumsky helpers ───────────────────────────────────────────────────────────

fn chumsky_build_prev(text: &str) -> (Vec<ChumskySpanned<ChumskyToken>>, Context) {
    let (raw, _) = chumsky_parse_tokens_str(text);
    let tokens: Vec<_> = raw.into_iter().filter(|t| !t.is_invalid()).collect();
    let url = Url::parse("file:///conflict-lint/input.ttl").unwrap();
    let mut ctx = Context::new();
    let (turtle, _) = chumsky_parse_turtle(&url, tokens.clone(), text.len(), ctx.ctx());
    ctx.clear();
    turtle.set_context(&mut ctx);
    (tokens, ctx)
}

fn chumsky_pairs(
    text: &str,
    prev: Option<(&Vec<ChumskySpanned<ChumskyToken>>, &mut Context)>,
) -> Vec<(Range<usize>, String)> {
    let url = Url::parse("file:///conflict-lint/input.ttl").unwrap();
    let (raw, _) = chumsky_parse_tokens_str(text);
    let tokens: Vec<_> = raw.into_iter().filter(|t| !t.is_invalid()).collect();
    let len = tokens.len();

    let ctx = if let Some((before_tokens, context)) = prev {
        context.setup_current_to_prev(
            TokenIdx { tokens: &tokens },
            len,
            TokenIdx {
                tokens: before_tokens,
            },
            before_tokens.len(),
        );
        context.ctx()
    } else {
        // Fresh parse: use a temporary empty context.
        let empty = Context::new();
        // SAFETY: the empty context lives long enough for the parse call below.
        // We extend the lifetime manually to avoid holding the borrow across
        // the parse call — the parse finishes before `empty` is dropped.
        unsafe { std::mem::transmute(empty.ctx()) }
    };

    let (_tree, errors) = chumsky_parse_turtle(&url, tokens, text.len(), ctx);
    errors
        .into_iter()
        .map(|e| (e.span(), e.to_string()))
        .collect()
}

// ── run ───────────────────────────────────────────────────────────────────────

fn run_astar(fixture: &Fixture, loc: &str) {
    let bias = IncrementalBias::default();

    if fixture.is_static {
        println!("=== (static) ===");
        let pairs = astar_pairs_from_parse(
            parse_t_2(Rule::new(SyntaxKind::TurtleDoc), &fixture.before),
            &fixture.before,
        );
        print_ariadne(&pairs, &fixture.before, loc);
        println!();
        return;
    }

    // before — fresh parse
    println!("=== before ===");
    let prev = astar_build_prev(&fixture.before);
    let before_pairs = astar_pairs_from_parse(
        parse_t_2(Rule::new(SyntaxKind::TurtleDoc), &fixture.before),
        &fixture.before,
    );
    print_ariadne(&before_pairs, &fixture.before, loc);
    println!();

    // after — incremental parse using before as context
    println!("=== after (incremental from before) ===");
    let after_parse = parse_t_2_incremental(
        Rule::new(SyntaxKind::TurtleDoc),
        &fixture.after,
        Some(&prev),
        bias,
    );
    let after_pairs = astar_pairs_from_parse(after_parse, &fixture.after);
    print_ariadne(&after_pairs, &fixture.after, loc);
    println!();
}

fn run_chumsky(fixture: &Fixture, loc: &str) {
    if fixture.is_static {
        println!("=== (static) ===");
        let pairs = chumsky_pairs(&fixture.before, None);
        print_ariadne(&pairs, &fixture.before, loc);
        println!();
        return;
    }

    // before — fresh parse, also builds context for "after"
    println!("=== before ===");
    let (before_tokens, mut ctx) = chumsky_build_prev(&fixture.before);
    let before_pairs = chumsky_pairs(&fixture.before, None);
    print_ariadne(&before_pairs, &fixture.before, loc);
    println!();

    // after — incremental parse using before context
    println!("=== after (incremental from before) ===");
    let after_pairs = chumsky_pairs(&fixture.after, Some((&before_tokens, &mut ctx)));
    print_ariadne(&after_pairs, &fixture.after, loc);
    println!();
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    let mut args = std::env::args().skip(1);

    let subcmd = args.next().unwrap_or_else(|| {
        eprintln!("Usage: conflict-lint <astar|chumsky> <file>");
        std::process::exit(1);
    });
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: conflict-lint <astar|chumsky> <file>");
        std::process::exit(1);
    });

    let content = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("Error reading {path}: {e}");
        std::process::exit(1);
    });

    let fixture = Fixture::parse(&path, &content);

    match subcmd.as_str() {
        "astar" => run_astar(&fixture, &path),
        "chumsky" => run_chumsky(&fixture, &path),
        _ => {
            eprintln!("Unknown subcommand: '{subcmd}'. Use 'astar' or 'chumsky'.");
            std::process::exit(1);
        }
    }
}
