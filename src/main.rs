use std::fs::read_to_string;

use turtle::parse_t_2;
use turtle::turtle as lang;
use turtle::turtle::convert::convert;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./test.ttl".to_string());
    let input = read_to_string(&path).unwrap();

    println!("=== Input ===");
    println!("{}", input);

    let (parse, _) = parse_t_2(lang::Rule::new(lang::SyntaxKind::TurtleDoc), &input);
    let root = parse.syntax::<lang::Lang>();
    let doc = convert(&root);

    println!("=== Triples ===");
    for triple in &doc.triples {
        println!("{}", triple.value());
    }
}
