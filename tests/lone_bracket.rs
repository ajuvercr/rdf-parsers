//! A `[` that is not closed yet, as when it has just been typed. A complete
//! `[]` is one ANON token, so the empty `'[' ']'` form of a property list
//! never occurs in valid text; recovery can still use it, and then a lone `[`
//! costs one missing `]` instead of a whole invented predicate-object list.

use rdf_parsers::{IncrementalBias, PrevParseInfo, parse, parse_incremental};

#[test]
fn turtle_lone_bracket_misses_only_its_close() {
    use rdf_parsers::turtle::{Rule, SyntaxKind};
    let (result, _) = parse(
        Rule::new(SyntaxKind::TurtleDoc),
        "<a> <p> [ .\n<b> <p> <o> .\n",
    );
    let errors: Vec<&String> = result.errors.iter().collect();
    assert_eq!(errors, ["Expected(SqClose)"]);
}

/// `foaf:foaf [` typed on a new line above `foaf:name ?name .`, one
/// keystroke at a time: the old line keeps its tokens.
#[test]
fn sparql_typed_bracket_deletes_nothing() {
    use rdf_parsers::sparql::{Rule, SyntaxKind};
    let before = "PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?person ?name
WHERE {
  ?person a foaf:Person ;
          foaf:name ?name .
}
";
    let run = |text: &str, prev: Option<&PrevParseInfo>| {
        parse_incremental(
            Rule::new(SyntaxKind::QueryUnit),
            text,
            prev,
            IncrementalBias::default(),
        )
    };
    let (_, mut prev) = run(before, None);
    let at = before.find("          foaf:name").unwrap();
    let mut text = before.to_string();
    let mut errors = Vec::new();
    for (i, c) in "          foaf:foaf [\n".char_indices() {
        text.insert(at + i, c);
        let (result, info) = run(&text, Some(&prev));
        errors = result.errors.iter().cloned().collect();
        prev = info;
    }
    assert!(
        errors.iter().all(|e| e.starts_with("Expected")),
        "{errors:?}"
    );
}
