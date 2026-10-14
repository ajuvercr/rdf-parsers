//! `parse_incremental` reuses the previous parse before and after an edit.
//! Whenever the edited document is valid, that must give exactly the tree a
//! fresh parse gives.  Edits are chained, so reuse of a parse that itself
//! reused an earlier one is covered too.

use rdf_parsers::{IncrementalBias, PrevParseInfo, TokenTrait, parse_fast, parse_incremental};

fn rng(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

fn files(dir: &str, ext: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![std::path::PathBuf::from(dir)];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&dir) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == ext)
                && let Ok(s) = std::fs::read_to_string(&p)
            {
                out.push(s);
            }
        }
    }
    out.sort();
    out
}

macro_rules! check {
    ($lang:ident, $unit:ident, $docs:expr) => {{
        use rdf_parsers::$lang::{Rule, SyntaxKind};
        let bias = IncrementalBias::default();
        let (mut checked, mut reused_valid) = (0, 0);
        for (n, doc) in $docs.into_iter().enumerate() {
            let mut seed = 0x9E3779B97F4A7C15u64 ^ (n as u64 + 1);
            let mut text = doc;
            let (_, mut prev): (_, PrevParseInfo) =
                parse_incremental(Rule::new(SyntaxKind::$unit), &text, None, bias);
            // The last valid document and its parse info.  An edit that breaks
            // the document gets one more edit on top (so reuse of a parse with
            // errors is covered too), then the chain goes back to this.
            let (mut good_text, mut good_prev) = (text.clone(), None);
            let mut broken = 0;
            for _ in 0..6 {
                if broken == 2 {
                    text = good_text.clone();
                    match good_prev.take() {
                        Some(p) => prev = p,
                        None => {
                            prev = parse_incremental(Rule::new(SyntaxKind::$unit), &text, None, bias).1
                        }
                    }
                    broken = 0;
                }
                let toks = rdf_parsers::tokenize::<SyntaxKind>(&text);
                let real: Vec<_> = toks.iter().filter(|t| !t.kind.skips()).collect();
                if real.is_empty() {
                    break;
                }
                let t = real[rng(&mut seed) as usize % real.len()];
                let r = t.range.clone();
                let tok = &text[r.clone()];
                let replacement = match rng(&mut seed) % 4 {
                    // Renames keep most documents valid.
                    0 | 1 if tok.starts_with('?') => format!("{tok}z"),
                    0 | 1 if tok.ends_with('>') => format!("{}z>", &tok[..tok.len() - 1]),
                    0 | 1 if tok.contains(':') => format!("{tok}z"),
                    2 => String::new(),
                    _ => format!("{tok} {tok}"),
                };
                text = format!("{}{}{}", &text[..r.start], replacement, &text[r.end..]);

                let (parse, next) =
                    parse_incremental(Rule::new(SyntaxKind::$unit), &text, Some(&prev), bias);
                assert_eq!(parse.syntax::<rdf_parsers::$lang::Lang>().to_string(), text);
                checked += 1;
                if let Some((fresh, _)) = parse_fast(Rule::new(SyntaxKind::$unit), &text) {
                    assert_eq!(
                        parse.green_node, fresh.green_node,
                        "incremental tree differs from a fresh parse of:\n{text}"
                    );
                    reused_valid += 1;
                    good_text = text.clone();
                    good_prev = None;
                    broken = 0;
                    prev = next;
                } else {
                    if broken == 0 {
                        good_prev = Some(std::mem::replace(&mut prev, next));
                    } else {
                        prev = next;
                    }
                    broken += 1;
                }
            }
        }
        eprintln!("{}: {checked} edits, {reused_valid} valid after the edit", stringify!($lang));
        assert!(reused_valid > 0);
    }};
}

fn sparql_doc(n: usize) -> String {
    let mut s = String::from("PREFIX foaf: <http://xmlns.com/foaf/0.1/>\nSELECT DISTINCT * WHERE {\n");
    for i in 0..n {
        s.push_str(&format!(
            "  ?s{i} foaf:name ?n{i} ;\n        foaf:knows ?k{i} .\n  OPTIONAL {{ ?s{i} foaf:mbox ?m{i} }}\n  FILTER(?n{i} != \"x{i}\" && STRLEN(?n{i}) > {i})\n"
        ));
    }
    s.push_str("}\nORDER BY ?n0\nLIMIT 10\n");
    s
}

fn turtle_doc(n: usize) -> String {
    let mut s = String::from(
        "@prefix ex: <http://example.org/> .\n@prefix foaf: <http://xmlns.com/foaf/0.1/> .\n",
    );
    for i in 0..n {
        s.push_str(&format!(
            "ex:s{i} a foaf:Person ;\n  foaf:name \"n{i}\" ;\n  foaf:knows [ foaf:name \"k{i}\" ], ex:s{} .\n",
            i + 1
        ));
    }
    s
}

/// Every fourth valid document, to keep the test quick.
fn valid(docs: Vec<String>, ok: impl Fn(&str) -> bool) -> Vec<String> {
    docs.into_iter().filter(|d| d.len() < 20_000 && ok(d)).step_by(4).collect()
}

#[test]
fn sparql_reuse_matches_fresh_parse() {
    use rdf_parsers::sparql::{Rule, SyntaxKind};
    let ok = |d: &str| parse_fast(Rule::new(SyntaxKind::QueryUnit), d).is_some();
    let mut docs = valid(files("conformance/rdf-tests/sparql", "rq"), ok);
    docs.extend((0..20).map(|i| sparql_doc(5 + i * 7)));
    check!(sparql, QueryUnit, docs);
}

#[test]
fn turtle_reuse_matches_fresh_parse() {
    use rdf_parsers::turtle::{Rule, SyntaxKind};
    let ok = |d: &str| parse_fast(Rule::new(SyntaxKind::TurtleDoc), d).is_some();
    let mut docs = valid(files("conformance/rdf-tests/rdf/rdf11/rdf-turtle", "ttl"), ok);
    docs.extend((0..20).map(|i| turtle_doc(5 + i * 7)));
    check!(turtle, TurtleDoc, docs);
}
