//! A clause keyword typed where it does not belong (`FILTER` in a SELECT
//! clause) is repaired by deleting it. That deletion used to cost 50, and
//! the search explores every cheaper partial parse first: in SPARQL's grammar
//! over a million of them, until it hit the iteration cap and deleted the
//! rest of the query.

use rdf_parsers::sparql::{Rule, SyntaxKind};
use rdf_parsers::{last_iterations, parse};

#[test]
fn stray_filter_in_select_is_deleted() {
    let text = "PREFIX ex: <http://example.org/>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>

SELECT ?person ?name FILTER
WHERE {
  ?person a foaf:Person ; foaf: [ ] ;
          foaf:name ?name
  OPTIONAL { ?person foaf:age ?age . }
  FILTER(?name != \"\")
}
ORDER BY ?name
";
    let (result, _) = parse(Rule::new(SyntaxKind::QueryUnit), text);
    let iterations = last_iterations();
    let errors: Vec<&String> = result.errors.iter().collect();
    assert_eq!(errors, ["Unexpected(FilterLit)"]);
    assert!(iterations < 200_000, "{iterations} search iterations");
}
