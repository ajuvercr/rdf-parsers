use benchmark::{Fixture, load_fixtures_ext};
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use rdf_parsers::{
    IncrementalBias, PrevParseInfo, TokenTrait, parse, parse_incremental,
    sparql::parser::{Rule, SyntaxKind},
};

// ── helpers ───────────────────────────────────────────────────────────────────

fn sparql_parse(text: &str) {
    let _ = parse(Rule::new(SyntaxKind::QueryUnit), text);
}

fn build_prev_info(text: &str) -> PrevParseInfo {
    let (parse, tokens) = parse(Rule::new(SyntaxKind::QueryUnit), text);
    let had_errors = parse.errors.len() > 0;
    let mut depth: i32 = 0;
    PrevParseInfo {
        tokens: tokens
            .iter()
            .map(|t| {
                let d = depth.clamp(0, 255) as u8;
                depth += t.kind.bracket_delta() as i32;
                t.to_prev_token(d)
            })
            .collect(),
        had_errors,
    }
}

// ── benchmark groups ──────────────────────────────────────────────────────────

/// Fresh parse of static SPARQL fixtures — no edit, pure throughput.
fn bench_fresh(c: &mut Criterion) {
    let fixtures_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");
    let fixtures = load_fixtures_ext(fixtures_dir, "rq");

    let mut group = c.benchmark_group("fresh/sparql");
    for fix in &fixtures {
        group.bench_with_input(BenchmarkId::new("parse", &fix.name), fix, |b, fix| {
            b.iter(|| sparql_parse(&fix.after))
        });
    }
    group.finish();
}

/// Incremental re-parse benchmarks — only for edit fixtures (those with markers).
fn bench_incremental(c: &mut Criterion) {
    let fixtures_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");
    let edit_fixtures: Vec<Fixture> = load_fixtures_ext(fixtures_dir, "rq")
        .into_iter()
        .filter(|f| !f.is_static)
        .collect();

    if edit_fixtures.is_empty() {
        return;
    }

    // "cold": fresh parse of the "after" text — baseline for the incremental case.
    let mut group = c.benchmark_group("incremental/cold_sparql");
    for fix in &edit_fixtures {
        group.bench_with_input(BenchmarkId::new("fresh_after", &fix.name), fix, |b, fix| {
            b.iter(|| sparql_parse(&fix.after))
        });
    }
    group.finish();

    // "warm": parse "after" with PrevParseInfo from "before" — the incremental path.
    let mut group = c.benchmark_group("incremental/warm_sparql");
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
                            Rule::new(SyntaxKind::QueryUnit),
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
}

criterion_group!(benches, bench_fresh, bench_incremental);
criterion_main!(benches);
