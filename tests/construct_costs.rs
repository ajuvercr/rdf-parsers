//! The cost model: every inserted token costs 1, every deleted one
//! `CONSTRUCT_DELETE`, and a construct the repair invents costs what the
//! grammar's `=== construct_cost ===` says.

use rdf_parsers::turtle::{Lang, Rule, SyntaxKind};
use rdf_parsers::{ParseOptions, PrevParseInfo, last_iterations, parse_incremental_with};

/// Errors, the kinds around each probed token (nearest first), and iterations.
fn run(before: Option<&str>, after: &str, probe: &[&str]) -> (Vec<String>, Vec<String>, usize) {
    let options = ParseOptions::default();
    let rule = || Rule::new(SyntaxKind::TurtleDoc);
    let prev: Option<PrevParseInfo> =
        before.map(|b| parse_incremental_with(rule(), b, None, options).1);
    let (parse, _) = parse_incremental_with(rule(), after, prev.as_ref(), options);
    let iterations = last_iterations();
    let root = parse.syntax::<Lang>();
    let mut where_ = Vec::new();
    for t in root
        .descendants_with_tokens()
        .filter_map(|e| e.into_token())
    {
        if probe.contains(&t.text()) {
            let kinds: Vec<String> = t
                .parent_ancestors()
                .map(|n| format!("{:?}", n.kind()))
                .filter(|k| {
                    matches!(
                        k.as_str(),
                        "Subject"
                            | "Verb"
                            | "Object"
                            | "Error"
                            | "PrefixId"
                            | "BlankNodePropertyList"
                    )
                })
                .take(1)
                .collect();
            where_.push(format!("{}={}", t.text(), kinds.join("")));
        }
    }
    (parse.errors.iter().cloned().collect(), where_, iterations)
}

const P: &str = "@prefix x: <http://example.org/x#> .\n@prefix ex: <http://example.org/> .\n";

/// `x:` typed in front of the predicate: with history, it starts a new pair
/// and the old pair keeps its roles; without, a comma is the cheapest reading.
#[test]
fn typed_name_starts_a_new_pair() {
    let before = format!("{P}<a> <b> <c> .\n");
    let after = format!("{P}<a> x: <b> <c> .\n");
    let (errors, roles, _) = run(Some(&before), &after, &["<b>", "<c>"]);
    assert_eq!(errors, ["Expected(ObjectList)", "Expected(Colon)"]);
    assert_eq!(roles, ["<b>=Verb", "<c>=Object"]);
    let (errors, _, _) = run(None, &after, &[]);
    assert_eq!(errors, ["Expected(Comma)"]);
}

/// A `@prefix` typed on a new line is the start of a directive: three
/// insertions complete it, which is cheaper than deleting it.
#[test]
fn typed_directive_is_completed() {
    let before = format!("{P}ex:a ex:p ex:o .\n");
    let after = format!("{P}ex:a ex:p ex:o .\n@prefix\n");
    let (errors, roles, _) = run(Some(&before), &after, &["@prefix"]);
    assert_eq!(
        errors,
        ["Expected(PnameNs)", "Expected(Iriref)", "Expected(Stop)"]
    );
    assert_eq!(roles.last().map(String::as_str), Some("@prefix=PrefixId"));
}

/// A `@prefix` typed inside a statement is deleted: inventing a directive
/// around it costs more.  (Per-token weights, the earlier cost model, charged
/// 500 to delete it, and the parser invented seven tokens instead.)
#[test]
fn stray_directive_keyword_is_deleted() {
    let before = format!("{P}ex:a ex:p ex:o .\n");
    let after = format!("{P}ex:a ex:p @prefix ex:o .\n");
    let (errors, roles, _) = run(Some(&before), &after, &["ex:o"]);
    assert_eq!(errors, ["Unexpected(PrefixToken)"]);
    assert_eq!(roles, ["ex:o=Object"]);
}

#[test]
fn lone_bracket_is_closed() {
    let (errors, roles, _) = run(None, &format!("{P}<a> <p> [ .\n<b> <p> <o> .\n"), &["<b>"]);
    assert_eq!(errors, ["Expected(SqClose)"]);
    assert_eq!(roles, ["<b>=Subject"]);
}

/// Half an IRI typed in front of `SELECT`, one character at a time.  Once
/// the keyword is no longer glued to the junk (`<http:/`, `<http://`), the
/// junk is deleted as one run and `SELECT` is kept, rather than read as
/// operators in an invented projection.  That holds even after a broken parse
/// gave the junk such roles, because it never had a clean role.  And no
/// keystroke runs to the search's iteration cap.
#[test]
fn stray_iri_before_select_is_deleted_as_a_run() {
    use rdf_parsers::sparql::Rule as Q;
    use rdf_parsers::sparql::SyntaxKind as K;
    let options = ParseOptions::default();
    let query = "PREFIX foaf: <http://xmlns.com/foaf/0.1/>\nSELECT ?person ?name\nWHERE {\n  ?person a foaf:Person ;\n          foaf:name ?name .\n}\nORDER BY ?name\n";
    let at = query.find("SELECT").unwrap();
    let (_, mut prev) = parse_incremental_with(Q::new(K::QueryUnit), query, None, options);
    let mut text = query.to_string();
    for (i, c) in "<http://x/> ".char_indices() {
        text.insert(at + i, c);
        let (parse, next) = parse_incremental_with(Q::new(K::QueryUnit), &text, Some(&prev), options);
        let iterations = last_iterations();
        assert!(iterations < 200_000, "{iterations} iterations after typing {c:?}");
        let typed = &text[at..at + i + 1];
        let errors: Vec<&String> = parse.errors.iter().collect();
        if typed.ends_with('/') {
            assert!(
                errors.iter().all(|e| !e.starts_with("Expected(") && e.as_str() != "Unexpected(SelectLit)"),
                "{typed:?} should only delete junk, got {errors:?}"
            );
        }
        prev = next;
    }
}
