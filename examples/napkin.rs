//! Napkin benchmark: how long does a parse take on documents of a few sizes,
//! valid and with one error in the middle?  Not rigorous — median of a few
//! runs — but enough to see how the search scales.
//!
//!     cargo run --release --example napkin [filter]

use std::time::{Duration, Instant};

use rdf_parsers::{
    IncrementalBias, PrevParseInfo, last_iterations, parse, parse_incremental, sparql, turtle,
};

/// One block of a generated document.  `edit` replaces the middle block.
type Block = fn(usize) -> String;

fn sparql_block(i: usize) -> String {
    format!(
        "  ?s{i} foaf:name ?n{i} ;\n        foaf:knows ?k{i} .\n  OPTIONAL {{ ?s{i} foaf:mbox ?m{i} }}\n  FILTER(?n{i} != \"x{i}\" && STRLEN(?n{i}) > {i})\n"
    )
}

fn sparql_doc(n: usize, edit: Option<Block>) -> String {
    let mut s = String::from(
        "PREFIX foaf: <http://xmlns.com/foaf/0.1/>\nSELECT DISTINCT * WHERE {\n",
    );
    for i in 0..n {
        match edit {
            Some(f) if i == n / 2 => s.push_str(&f(i)),
            _ => s.push_str(&sparql_block(i)),
        }
    }
    s.push_str("}\nORDER BY ?n0\nLIMIT 10\n");
    s
}

const SPARQL_EDITS: &[(&str, Block)] = &[
    ("rename", |i| sparql_block(i).replace("?k", "?friend")),
    ("missing-dot", |i| {
        format!("  ?s{i} foaf:name ?n{i} ;\n        foaf:knows ?k{i}\n  ?x{i} foaf:age ?a{i} .\n")
    }),
    ("missing-brace", |i| format!("  OPTIONAL {{ ?s{i} foaf:mbox ?m{i}\n")),
    ("keyword-typo", |i| format!("  FILTR(?n{i} != \"x{i}\")\n")),
    ("half-typed", |i| format!("  ?s{i} foaf:\n")),
    ("open-paren", |i| format!("  FILTER(?n{i} > {i}\n")),
];

fn turtle_block(i: usize) -> String {
    format!(
        "ex:s{i} a foaf:Person ;\n  foaf:name \"n{i}\" ;\n  foaf:knows [ foaf:name \"k{i}\" ], ex:s{} .\n",
        i + 1
    )
}

fn turtle_doc(n: usize, edit: Option<Block>) -> String {
    let mut s = String::from(
        "@prefix ex: <http://example.org/> .\n@prefix foaf: <http://xmlns.com/foaf/0.1/> .\n",
    );
    for i in 0..n {
        match edit {
            Some(f) if i == n / 2 => s.push_str(&f(i)),
            _ => s.push_str(&turtle_block(i)),
        }
    }
    s
}

const TURTLE_EDITS: &[(&str, Block)] = &[
    ("rename", |i| turtle_block(i).replace("foaf:knows", "foaf:friend")),
    ("missing-dot", |i| format!("ex:s{i} a foaf:Person ;\n  foaf:name \"n{i}\"\n")),
    ("missing-bracket", |i| format!("ex:s{i} foaf:knows [ foaf:name \"k{i}\" .\n")),
    ("half-typed", |i| format!("ex:s{i} foaf:\n")),
];

/// Median wall time of `f` and the heap pops of its last run.  Slow cases get
/// fewer runs so the whole thing stays napkin-sized.
fn time(mut f: impl FnMut()) -> (Duration, usize) {
    let start = Instant::now();
    f();
    let first = start.elapsed();
    let runs = if first > Duration::from_millis(500) { 1 } else { 7 };
    let mut times = vec![first];
    for _ in 1..runs {
        let start = Instant::now();
        f();
        times.push(start.elapsed());
    }
    times.sort();
    (times[times.len() / 2], last_iterations())
}

fn prev_info(parse: impl FnOnce() -> (rdf_parsers::Parse, PrevParseInfo)) -> PrevParseInfo {
    parse().1
}

macro_rules! lang {
    ($name:literal, $lang:ident, $unit:ident, $doc:ident, $edits:ident, $filter:expr) => {{
        use $lang::{Rule, SyntaxKind::$unit};
        let bias = IncrementalBias::default();
        for &n in &[10usize, 50, 200] {
            let valid = $doc(n, None);
            let cases = std::iter::once(("valid", None)).chain($edits.iter().map(|(k, f)| (*k, Some(*f))));
            for (case, edit) in cases {
                let label = format!("{} n={n} {case}", $name);
                if !label.contains($filter) {
                    continue;
                }
                let text = $doc(n, edit);
                let toks = rdf_parsers::tokenize::<$lang::SyntaxKind>(&text)
                    .iter()
                    .filter(|t| !rdf_parsers::TokenTrait::skips(&t.kind))
                    .count();
                let (fresh, fresh_it) = time(|| {
                    let _ = parse(Rule::new($unit), &text);
                });
                let errors = parse(Rule::new($unit), &text).0.errors.len();
                let prev = prev_info(|| parse_incremental(Rule::new($unit), &valid, None, bias));
                let (incr, incr_it) = time(|| {
                    let _ = parse_incremental(Rule::new($unit), &text, Some(&prev), bias);
                });
                println!(
                    "{label:<34} {toks:>6} {fresh:>11.2?} {fresh_it:>9} {incr:>11.2?} {incr_it:>9} {errors:>4}"
                );
            }
        }
    }};
}

fn main() {
    let filter = std::env::args().nth(1).unwrap_or_default();
    let filter = filter.as_str();
    println!(
        "{:<34} {:>6} {:>11} {:>9} {:>11} {:>9} {:>4}",
        "case", "tokens", "fresh", "pops", "incr", "pops", "errs"
    );
    lang!("turtle", turtle, TurtleDoc, turtle_doc, TURTLE_EDITS, filter);
    lang!("sparql", sparql, QueryUnit, sparql_doc, SPARQL_EDITS, filter);
}
