use benchmark::{Fixture, load_fixtures_ext};
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use turtle::{
    IncrementalBias, PrevParseInfo, TokenTrait as _, extract_prev_roles, parse_t_2,
    parse_t_2_incremental, tokenize,
    ntriples::parser::{Lang, Rule, SyntaxKind},
};

// ── helpers ───────────────────────────────────────────────────────────────────

fn ntriples_parse(text: &str) {
    parse_t_2(Rule::new(SyntaxKind::NtriplesDoc), text);
}

fn build_prev_info(text: &str) -> PrevParseInfo<SyntaxKind> {
    let parse = parse_t_2(Rule::new(SyntaxKind::NtriplesDoc), text);
    let root = parse.syntax::<Lang>();
    let tokens = tokenize::<SyntaxKind>(text);
    let prev_roles = extract_prev_roles::<Lang>(&root);
    PrevParseInfo { tokens, prev_roles }
}

// ── benchmark groups ──────────────────────────────────────────────────────────

/// Fresh parse of static N-Triples fixtures — no edit, pure throughput.
fn bench_fresh(c: &mut Criterion) {
    let fixtures_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");
    let fixtures = load_fixtures_ext(fixtures_dir, "nt");

    let mut group = c.benchmark_group("fresh/ntriples");
    for fix in &fixtures {
        group.bench_with_input(
            BenchmarkId::new("parse", &fix.name),
            fix,
            |b, fix| b.iter(|| ntriples_parse(&fix.after)),
        );
    }
    group.finish();
}

/// Incremental re-parse benchmarks — only for edit fixtures (those with markers).
fn bench_incremental(c: &mut Criterion) {
    let fixtures_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");
    let edit_fixtures: Vec<Fixture> = load_fixtures_ext(fixtures_dir, "nt")
        .into_iter()
        .filter(|f| !f.is_static)
        .collect();

    if edit_fixtures.is_empty() {
        return;
    }

    // "cold": fresh parse of the "after" text — baseline for the incremental case.
    let mut group = c.benchmark_group("incremental/cold_ntriples");
    for fix in &edit_fixtures {
        group.bench_with_input(
            BenchmarkId::new("fresh_after", &fix.name),
            fix,
            |b, fix| b.iter(|| ntriples_parse(&fix.after)),
        );
    }
    group.finish();

    // "warm": parse "after" with PrevParseInfo from "before" — the incremental path.
    let mut group = c.benchmark_group("incremental/warm_ntriples");
    let bias = IncrementalBias::default();
    for fix in &edit_fixtures {
        group.bench_with_input(
            BenchmarkId::new("incremental_after", &fix.name),
            fix,
            |b, fix| {
                b.iter_batched(
                    || build_prev_info(&fix.before),
                    |prev| {
                        parse_t_2_incremental(
                            Rule::new(SyntaxKind::NtriplesDoc),
                            &fix.after,
                            Some(&prev),
                            bias,
                        )
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
