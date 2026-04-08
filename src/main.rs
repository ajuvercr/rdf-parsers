use std::fs::read_to_string;

use rdf_parsers::parse;
use rdf_parsers::turtle as lang;
use rdf_parsers::turtle::convert::convert;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./test.ttl".to_string());
    let input = read_to_string(&path).unwrap();

    println!("=== Input ===");
    println!("{}", input);

    let (result, _) = parse(lang::Rule::new(lang::SyntaxKind::TurtleDoc), &input);
    let root = result.syntax::<lang::Lang>();
    let doc = convert(&root);

    println!("=== Triples ===");
    for triple in &doc.triples {
        println!("{}", triple.value());
    }
}
