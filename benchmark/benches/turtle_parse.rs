use benchmark::{Fixture, load_fixtures};
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use lsp_types::Url;
use oxttl::TurtleParser;
use swls_lang_turtle::lang::{
    context::{Context, TokenIdx},
    parse_source as lang_turtle_parse_source,
    parser::parse_turtle as chumsky_parse_turtle,
    tokenizer::parse_tokens_str as chumsky_parse_tokens_str,
};
use turtle::{
    IncrementalBias, PrevParseInfo, parse, parse_incremental,
    turtle::parser::{Rule, SyntaxKind},
};

// ── helpers ──────────────────────────────────────────────────────────────────

fn chumsky_turtle_parse(url: &Url, text: &str) {
    lang_turtle_parse_source(url, text);
}

fn turtle_parse(text: &str) {
    let _ = parse(Rule::new(SyntaxKind::TurtleDoc), text);
}

fn oxttl_parse(text: &str) {
    TurtleParser::new()
        .for_slice(text.as_bytes())
        .for_each(|r| {
            let _ = r;
        });
}

fn build_prev_info(text: &str) -> PrevParseInfo {
    let (_, tokens) = parse(Rule::new(SyntaxKind::TurtleDoc), text);
    PrevParseInfo {
        tokens: tokens.iter().map(|t| t.to_prev_token()).collect(),
    }
}

// ── benchmark groups ──────────────────────────────────────────────────────────

/// Fresh parse of static fixtures — no edit, pure throughput.
fn bench_fresh(c: &mut Criterion) {
    let fixtures_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");
    let fixtures = load_fixtures(fixtures_dir);
    let url = Url::parse("file:///benchmark/fixture.ttl").unwrap();

    let mut group = c.benchmark_group("fresh/turtle");
    for fix in &fixtures {
        group.bench_with_input(BenchmarkId::new("parse", &fix.name), fix, |b, fix| {
            b.iter(|| turtle_parse(&fix.after))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("fresh/chumsky_turtle");
    for fix in &fixtures {
        group.bench_with_input(BenchmarkId::new("parse", &fix.name), fix, |b, fix| {
            b.iter(|| chumsky_turtle_parse(&url, &fix.after))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("fresh/oxttl");
    for fix in &fixtures {
        group.bench_with_input(BenchmarkId::new("parse", &fix.name), fix, |b, fix| {
            b.iter(|| oxttl_parse(&fix.after))
        });
    }
    group.finish();
}

/// Incremental re-parse benchmarks — only for edit fixtures (those with markers).
fn bench_incremental(c: &mut Criterion) {
    let fixtures_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");
    let edit_fixtures: Vec<Fixture> = load_fixtures(fixtures_dir)
        .into_iter()
        .filter(|f| !f.is_static)
        .collect();

    if edit_fixtures.is_empty() {
        return;
    }

    // "cold": fresh parse of the "after" text — baseline for the incremental case.
    let mut group = c.benchmark_group("incremental/cold_turtle");
    for fix in &edit_fixtures {
        group.bench_with_input(BenchmarkId::new("fresh_after", &fix.name), fix, |b, fix| {
            b.iter(|| turtle_parse(&fix.after))
        });
    }
    group.finish();

    // "warm": parse "after" with PrevParseInfo from "before" — the incremental path.
    let mut group = c.benchmark_group("incremental/warm_turtle");
    let bias = IncrementalBias::default();
    for fix in &edit_fixtures {
        group.bench_with_input(
            BenchmarkId::new("incremental_after", &fix.name),
            fix,
            |b, fix| {
                b.iter_batched(
                    || build_prev_info(&fix.before),
                    |prev| {
                        let _ = parse_incremental(
                            Rule::new(SyntaxKind::TurtleDoc),
                            &fix.after,
                            Some(&prev),
                            bias,
                        );
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();

    // swls_lang_turtle as non-incremental reference for edit fixtures.
    let url = Url::parse("file:///benchmark/fixture.ttl").unwrap();
    let mut group = c.benchmark_group("incremental/cold_chumsky_turtle");
    for fix in &edit_fixtures {
        group.bench_with_input(BenchmarkId::new("fresh_after", &fix.name), fix, |b, fix| {
            b.iter(|| chumsky_turtle_parse(&url, &fix.after))
        });
    }
    group.finish();

    // "warm": parse "after" with chumsky context from "before" parse.
    let mut group = c.benchmark_group("incremental/warm_chumsky_turtle");
    for fix in &edit_fixtures {
        group.bench_with_input(
            BenchmarkId::new("incremental_after", &fix.name),
            fix,
            |b, fix| {
                b.iter_batched(
                    || {
                        let (raw_tokens, _) = chumsky_parse_tokens_str(&fix.before);
                        let tokens: Vec<_> =
                            raw_tokens.into_iter().filter(|x| !x.is_invalid()).collect();
                        let mut context = Context::new();
                        let (turtle, _) = chumsky_parse_turtle(
                            &url,
                            tokens.clone(),
                            fix.before.len(),
                            context.ctx(),
                        );
                        context.clear();
                        turtle.set_context(&mut context);
                        (tokens, context)
                    },
                    |(before_tokens, mut context)| {
                        let (raw_tokens, _) = chumsky_parse_tokens_str(&fix.after);
                        let after_tokens: Vec<_> =
                            raw_tokens.into_iter().filter(|x| !x.is_invalid()).collect();
                        let len = after_tokens.len();
                        context.setup_current_to_prev(
                            TokenIdx {
                                tokens: &after_tokens,
                            },
                            len,
                            TokenIdx {
                                tokens: &before_tokens,
                            },
                            before_tokens.len(),
                        );
                        chumsky_parse_turtle(&url, after_tokens, fix.after.len(), context.ctx())
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();

    // oxttl as the non-incremental reference for edit fixtures.
    let mut group = c.benchmark_group("incremental/cold_oxttl");
    for fix in &edit_fixtures {
        group.bench_with_input(BenchmarkId::new("fresh_after", &fix.name), fix, |b, fix| {
            b.iter(|| oxttl_parse(&fix.after))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fresh, bench_incremental);
criterion_main!(benches);
