use std::collections::HashMap;
use std::fs::read_to_string;

use ariadne::Span as _;
use turtle::parse_t;
use turtle::sparql as lang;
use turtle::util::{completion, print, print_ariadne};

// TODO:
//x1. errors -> spans
// 2. code cleanup (less in macro's more in implementation)
// 3. locational autocomplete
//   1. next token (at)
// 4. tests?
// 5. timing information?
// 6. context aware parsing!
fn main() {
    let sexps = read_to_string("./test.ttl").unwrap();
    println!("Parsing {}", sexps);
    let parse: turtle::Parse = parse_t::<lang::QueryUnit>(&sexps);

    let root = parse.syntax::<lang::Lang>();

    let mut errors: Vec<_> = parse.errors.iter().cloned().collect();
    errors.reverse();
    let error_nodes: Vec<_> = root
        .descendants()
        .filter(|x| x.kind() == lang::SyntaxKind::Error)
        .flat_map(|x| match root.token_at_offset(x.text_range().start()) {
            rowan::TokenAtOffset::None => None,
            rowan::TokenAtOffset::Single(x) => Some(x.text_range().into()),
            rowan::TokenAtOffset::Between(a, b) => {
                Some(a.text_range().start().into()..b.text_range().start().into())
            }
        })
        .zip(errors.iter())
        .collect();

    let mut es: &[String] = &errors;
    println!("All errors {:?}", error_nodes);
    let mut suggestion_per: HashMap<usize, Vec<String>> = HashMap::new();
    for (a, b) in parse.suggestions {
        suggestion_per.entry(b.start()).or_default().push(a);
    }
    let mut sugs: Vec<_> = suggestion_per.into_iter().collect();
    sugs.sort();
    for s in sugs {
        println!("{:?}", s);
    }
    print(&root, 0, &mut es);

    if let Some(r) = root.first_child() {
        let completions = completion(&r, 11);
        println!("Completions {:?}", completions);
    }

    print_ariadne(&error_nodes, &sexps);
}
