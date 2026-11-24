//! The construct cost model (`Costs::Constructs`): every inserted token costs
//! 1, every deleted one `CONSTRUCT_DELETE`, and a construct the repair invents
//! costs what the grammar's `=== construct_cost ===` says.

use rdf_parsers::turtle::{Lang, Rule, SyntaxKind};
use rdf_parsers::{Costs, ParseOptions, PrevParseInfo, last_iterations, parse_incremental_with};

/// Errors, the kinds around each probed token (nearest first), and iterations.
fn run(
    costs: Costs,
    before: Option<&str>,
    after: &str,
    probe: &[&str],
) -> (Vec<String>, Vec<String>, usize) {
    let options = ParseOptions {
        costs,
        ..ParseOptions::default()
    };
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
    let (errors, roles, _) = run(Costs::Constructs, Some(&before), &after, &["<b>", "<c>"]);
    assert_eq!(errors, ["Expected(ObjectList)", "Expected(Colon)"]);
    assert_eq!(roles, ["<b>=Verb", "<c>=Object"]);
    let (errors, _, _) = run(Costs::Constructs, None, &after, &[]);
    assert_eq!(errors, ["Expected(Comma)"]);
}

/// A `@prefix` typed on a new line is the start of a directive: three
/// insertions complete it, which is cheaper than deleting it.
#[test]
fn typed_directive_is_completed() {
    let before = format!("{P}ex:a ex:p ex:o .\n");
    let after = format!("{P}ex:a ex:p ex:o .\n@prefix\n");
    let (errors, roles, _) = run(Costs::Constructs, Some(&before), &after, &["@prefix"]);
    assert_eq!(
        errors,
        ["Expected(PnameNs)", "Expected(Iriref)", "Expected(Stop)"]
    );
    assert_eq!(roles.last().map(String::as_str), Some("@prefix=PrefixId"));
}

/// A `@prefix` typed inside a statement is deleted: inventing a directive
/// around it costs more. With the grammar's token weights, deleting it costs
/// 500 and the parser invents seven tokens instead.
#[test]
fn stray_directive_keyword_is_deleted() {
    let before = format!("{P}ex:a ex:p ex:o .\n");
    let after = format!("{P}ex:a ex:p @prefix ex:o .\n");
    let (errors, roles, _) = run(Costs::Constructs, Some(&before), &after, &["ex:o"]);
    assert_eq!(errors, ["Unexpected(PrefixToken)"]);
    assert_eq!(roles, ["ex:o=Object"]);
}

#[test]
fn lone_bracket_is_closed() {
    let (errors, roles, _) = run(
        Costs::Constructs,
        None,
        &format!("{P}<a> <p> [ .\n<b> <p> <o> .\n"),
        &["<b>"],
    );
    assert_eq!(errors, ["Expected(SqClose)"]);
    assert_eq!(roles, ["<b>=Subject"]);
}
