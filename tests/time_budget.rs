//! `parse_incremental_with` and its time budget: the search stops in time and
//! still returns a parse that covers the whole text.

use std::time::{Duration, Instant};

use rdf_parsers::turtle::{Lang, Rule, SyntaxKind};
use rdf_parsers::{ParseOptions, parse_incremental_with};

/// Text the Turtle 1.1 grammar cannot parse anywhere after the first
/// statement (RDF 1.2 reified triples, unclosed brackets): without a budget
/// the fault-tolerant search runs into its iteration limit (about 0.5 s).
fn hostile() -> String {
    let mut s = String::from("<first> <p> <o> .\n");
    for i in 0..400 {
        s.push_str(&format!(
            "<< <s{i}> <p> <o{i}> ~ <r{i}> >> <q> {{| <a> <b> |}} .\n"
        ));
    }
    for i in 0..400 {
        s.push_str(&format!("<x{i}> <p> [ <q> ( <a> <b>\n"));
    }
    s
}

#[test]
fn search_stops_within_its_budget() {
    let text = hostile();
    let options = ParseOptions {
        time_budget: Some(Duration::from_millis(20)),
        ..Default::default()
    };
    let start = Instant::now();
    let (parse, _) = parse_incremental_with(Rule::new(SyntaxKind::TurtleDoc), &text, None, options);
    let took = start.elapsed();
    // the budget is checked every few hundred expansions, plus building the tree
    assert!(took < Duration::from_millis(400), "took {took:?}");

    let root = parse.syntax::<Lang>();
    assert_eq!(
        root.text().to_string(),
        text,
        "the tree covers the whole text"
    );
    assert!(parse.errors.len() > 0);

    // the statement before the damage is parsed as a triple
    let first = root
        .descendants()
        .find(|n| n.kind() == SyntaxKind::Subject)
        .expect("a subject");
    assert_eq!(first.text().to_string().trim(), "<first>");
}

#[test]
fn without_budget_results_match_parse_incremental() {
    let text = "<a> <b> <c> .\n<d> <e> .\n<f> <g> <h> .\n";
    let (a, _) = rdf_parsers::parse_incremental(
        Rule::new(SyntaxKind::TurtleDoc),
        text,
        None,
        rdf_parsers::IncrementalBias::default(),
    );
    let (b, _) = parse_incremental_with(
        Rule::new(SyntaxKind::TurtleDoc),
        text,
        None,
        ParseOptions::default(),
    );
    assert_eq!(
        format!("{:#?}", a.syntax::<Lang>()),
        format!("{:#?}", b.syntax::<Lang>())
    );
}
