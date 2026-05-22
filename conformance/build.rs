use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    // ── Turtle ────────────────────────────────────────────────────────────────
    let turtle_dir = format!("{}/rdf-tests/rdf/rdf11/rdf-turtle", manifest_dir);
    let turtle_manifest =
        fs::read_to_string(format!("{}/manifest.ttl", turtle_dir)).expect("turtle manifest");
    let turtle_tests = parse_rdf_manifest(
        &turtle_manifest,
        "Turtle",
        &turtle_dir,
        "https://w3c.github.io/rdf-tests/rdf/rdf11/rdf-turtle/",
    );
    fs::write(Path::new(&out_dir).join("turtle_tests.rs"), generate_tests(&turtle_tests, "turtle"))
        .unwrap();
    println!("cargo:rerun-if-changed={}/manifest.ttl", turtle_dir);

    // ── TriG ─────────────────────────────────────────────────────────────────
    let trig_dir = format!("{}/rdf-tests/rdf/rdf11/rdf-trig", manifest_dir);
    let trig_manifest =
        fs::read_to_string(format!("{}/manifest.ttl", trig_dir)).expect("trig manifest");
    let trig_tests = parse_rdf_manifest(
        &trig_manifest,
        "Trig",
        &trig_dir,
        "https://w3c.github.io/rdf-tests/rdf/rdf11/rdf-trig/",
    );
    fs::write(Path::new(&out_dir).join("trig_tests.rs"), generate_tests(&trig_tests, "trig"))
        .unwrap();
    println!("cargo:rerun-if-changed={}/manifest.ttl", trig_dir);

    // ── SPARQL ────────────────────────────────────────────────────────────────
    let sparql_base = format!("{}/rdf-tests/sparql", manifest_dir);
    let sparql_dirs: &[(&str, &str)] = &[
        // sparql 1.0
        ("sparql10/syntax-sparql1", "https://www.w3.org/2001/sw/DataAccess/tests/data-r2/syntax-sparql1/"),
        ("sparql10/syntax-sparql2", "https://www.w3.org/2001/sw/DataAccess/tests/data-r2/syntax-sparql2/"),
        ("sparql10/syntax-sparql3", "https://www.w3.org/2001/sw/DataAccess/tests/data-r2/syntax-sparql3/"),
        ("sparql10/syntax-sparql4", "https://www.w3.org/2001/sw/DataAccess/tests/data-r2/syntax-sparql4/"),
        ("sparql10/syntax-sparql5", "https://www.w3.org/2001/sw/DataAccess/tests/data-r2/syntax-sparql5/"),
        // sparql 1.1
        ("sparql11/syntax-query",   "https://www.w3.org/2009/sparql/docs/tests/data-sparql11/syntax-query/"),
        ("sparql11/syntax-update-1","https://www.w3.org/2009/sparql/docs/tests/data-sparql11/syntax-update-1/"),
        ("sparql11/syntax-update-2","https://www.w3.org/2009/sparql/docs/tests/data-sparql11/syntax-update-2/"),
        ("sparql11/syntax-fed",     "https://www.w3.org/2009/sparql/docs/tests/data-sparql11/syntax-fed/"),
        ("sparql11/aggregates",     "https://www.w3.org/2009/sparql/docs/tests/data-sparql11/aggregates/"),
        ("sparql11/construct",      "https://www.w3.org/2009/sparql/docs/tests/data-sparql11/construct/"),
        ("sparql11/delete-insert",  "https://www.w3.org/2009/sparql/docs/tests/data-sparql11/delete-insert/"),
        ("sparql11/grouping",       "https://www.w3.org/2009/sparql/docs/tests/data-sparql11/grouping/"),
    ];

    let mut all_sparql: Vec<TestEntry> = Vec::new();
    for (rel_dir, assumed_base) in sparql_dirs {
        let dir = format!("{}/{}", sparql_base, rel_dir);
        let manifest_path = format!("{}/manifest.ttl", dir);
        // dir key: "sparql10_syntax_sparql3" etc. from "sparql10/syntax-sparql3"
        let dir_key = rel_dir.replace('/', "_").replace('-', "_");
        if let Ok(content) = fs::read_to_string(&manifest_path) {
            let mut tests = parse_sparql_manifest(&content, &dir, assumed_base, &dir_key);
            all_sparql.append(&mut tests);
            println!("cargo:rerun-if-changed={}", manifest_path);
        }
    }

    fs::write(
        Path::new(&out_dir).join("sparql_tests.rs"),
        generate_tests(&all_sparql, "sparql"),
    )
    .unwrap();
}

// ── shared types ──────────────────────────────────────────────────────────────

#[derive(Debug)]
enum TestType {
    PositiveSyntax,
    NegativeSyntax,
    NegativeEval,
    Eval,
}

struct TestEntry {
    name: String,
    test_type: TestType,
    action: String,
    result: Option<String>,
    base_iri: String,
}

fn extract_angle_value(line: &str) -> Option<String> {
    let start = line.find('<')? + 1;
    let end = start + line[start..].find('>')?;
    Some(line[start..end].to_string())
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect()
}

// ── RDF (Turtle / TriG) manifest parser ───────────────────────────────────────

fn parse_rdf_manifest(
    content: &str,
    format: &str,
    base_dir: &str,
    assumed_base: &str,
) -> Vec<TestEntry> {
    let positive = format!("Test{}PositiveSyntax", format);
    let negative = format!("Test{}NegativeSyntax", format);
    let neg_eval = format!("Test{}NegativeEval", format);
    let eval = format!("Test{}Eval", format);

    let mut entries: Vec<TestEntry> = Vec::new();
    let mut cur_name: Option<String> = None;
    let mut cur_type: Option<TestType> = None;
    let mut cur_action: Option<String> = None;
    let mut cur_result: Option<String> = None;

    let flush = |name: Option<String>,
                 ty: Option<TestType>,
                 action: Option<String>,
                 result: Option<String>,
                 entries: &mut Vec<TestEntry>| {
        if let (Some(name), Some(ty), Some(action)) = (name, ty, action) {
            let file_name = Path::new(&action)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            let base_iri = format!("{}{}", assumed_base, file_name);
            entries.push(TestEntry { name, test_type: ty, action, result, base_iri });
        }
    };

    for line in content.lines() {
        let t = line.trim();

        if t.starts_with("<#") && t.contains("rdf:type rdft:") {
            flush(cur_name.take(), cur_type.take(), cur_action.take(), cur_result.take(), &mut entries);
            if let Some(name_end) = t.find('>') {
                let name = t[2..name_end].to_string();
                let test_type = if t.contains(&positive) {
                    Some(TestType::PositiveSyntax)
                } else if t.contains(&neg_eval) {
                    Some(TestType::NegativeEval)
                } else if t.contains(&negative) {
                    Some(TestType::NegativeSyntax)
                } else if t.contains(&eval) {
                    Some(TestType::Eval)
                } else {
                    None
                };
                if let Some(ty) = test_type {
                    cur_name = Some(name);
                    cur_type = Some(ty);
                }
            }
        }

        if cur_name.is_some() {
            if t.starts_with("mf:action") {
                if let Some(file) = extract_angle_value(t) {
                    cur_action = Some(format!("{}/{}", base_dir, file));
                }
            } else if t.starts_with("mf:result") {
                if let Some(file) = extract_angle_value(t) {
                    cur_result = Some(format!("{}/{}", base_dir, file));
                }
            } else if t == "." {
                flush(cur_name.take(), cur_type.take(), cur_action.take(), cur_result.take(), &mut entries);
            }
        }
    }
    flush(cur_name, cur_type, cur_action, cur_result, &mut entries);
    entries
}

// ── SPARQL manifest parser ────────────────────────────────────────────────────
//
// SPARQL manifests use several format variations across sparql10/11/12:
//   ":name rdf:type mf:TestType ;"      (name + type on same line)
//   ":name a mf:TestType ;"             (a-shorthand on same line)
//   ":name\n    a mf:TestType ;"        (name alone, type on next line)
//   ":name mf:name "..." ;\n  rdf:type" (name + mf:name first, type later)
//
// Strategy: sliding-window scan — for each `mf:action` line, look back up to
// 15 lines to find the test type and local name.

fn parse_sparql_manifest(
    content: &str,
    base_dir: &str,
    assumed_base: &str,
    dir_key: &str,
) -> Vec<TestEntry> {
    let lines: Vec<&str> = content.lines().collect();

    let pos_markers = [
        "mf:PositiveSyntaxTest11",
        "mf:PositiveUpdateSyntaxTest11",
        "mf:PositiveSyntaxTest",
    ];
    let neg_markers = [
        "mf:NegativeSyntaxTest11",
        "mf:NegativeUpdateSyntaxTest11",
        "mf:NegativeSyntaxTest",
    ];

    let mut entries: Vec<TestEntry> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        if !t.starts_with("mf:action") {
            continue;
        }
        let file = match extract_angle_value(t) {
            Some(f) => f,
            None => continue,
        };
        // Only consider .rq and .ru files (query/update)
        if !file.ends_with(".rq") && !file.ends_with(".ru") {
            continue;
        }

        let window_start = i.saturating_sub(15);
        let window = &lines[window_start..=i];

        let mut test_type: Option<TestType> = None;
        let mut test_name: Option<String> = None;

        for wline in window.iter() {
            let wt = wline.trim();

            if pos_markers.iter().any(|&m| wt.contains(m)) {
                test_type = Some(TestType::PositiveSyntax);
            } else if neg_markers.iter().any(|&m| wt.contains(m)) {
                test_type = Some(TestType::NegativeSyntax);
            }

            // Local name: line starts with optional whitespace then ':identifier'
            // (not '://' which would be a URL, not a predicate like 'rdf:type')
            let stripped = wline.trim_start();
            if stripped.starts_with(':') && !stripped.starts_with("://") {
                let rest = &stripped[1..];
                let end = rest
                    .find(|c: char| c.is_whitespace())
                    .unwrap_or(rest.len());
                let candidate = &rest[..end];
                // Must look like an identifier (no slashes, not just punctuation)
                if !candidate.is_empty()
                    && !candidate.contains('/')
                    && candidate.chars().next().map(|c| c.is_alphanumeric() || c == '_' || c == '-').unwrap_or(false)
                {
                    test_name = Some(candidate.to_string());
                }
            }
        }

        let test_type = match test_type {
            Some(t) => t,
            None => continue,
        };
        // Build unique name: dir_key + local name (or filename)
        let local = test_name.unwrap_or_else(|| {
            file.trim_end_matches(".rq")
                .trim_end_matches(".ru")
                .to_string()
        });
        let name = format!("{}_{}", dir_key, local);

        let action = format!("{}/{}", base_dir, file);
        let base_iri = format!("{}{}", assumed_base, file);

        entries.push(TestEntry {
            name,
            test_type,
            action,
            result: None,
            base_iri,
        });
    }

    entries
}

// ── code generator ────────────────────────────────────────────────────────────

fn generate_tests(entries: &[TestEntry], format: &str) -> String {
    let mut out = String::new();
    for e in entries {
        let suf = sanitize(&e.name).to_lowercase();
        let fn_name = match e.test_type {
            TestType::PositiveSyntax => format!("{}_positive_{}", format, suf),
            TestType::NegativeSyntax => format!("{}_negative_{}", format, suf),
            TestType::NegativeEval => format!("{}_negative_{}", format, suf),
            TestType::Eval => format!("{}_eval_{}", format, suf),
        };

        let body = match &e.test_type {
            TestType::PositiveSyntax => {
                format!("    conformance::run_{}_positive_syntax(\"{}\");", format, e.action)
            }
            TestType::NegativeSyntax | TestType::NegativeEval => {
                format!("    conformance::run_{}_negative_syntax(\"{}\");", format, e.action)
            }
            TestType::Eval => match &e.result {
                Some(result) => format!(
                    "    conformance::run_{}_eval(\"{}\", \"{}\", \"{}\");",
                    format, e.action, result, e.base_iri
                ),
                None => format!(
                    "    conformance::run_{}_positive_syntax(\"{}\");",
                    format, e.action
                ),
            },
        };

        out.push_str(&format!("#[test]\nfn {}() {{\n{}\n}}\n\n", fn_name, body));
    }
    out
}
