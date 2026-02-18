use std::ops::Range;

use ariadne::Span;
use rowan::{NodeOrToken, SyntaxToken};
use turtle::{Lang, SyntaxNode, parse_t, testing};

fn print(n: &SyntaxNode, indent: usize, errors: &mut &[String]) {
    for _ in 0..indent {
        eprint!(" ");
    }
    if n.kind() == turtle::testing::SyntaxKind::Error {
        eprintln!("{:?} {:?}", n, errors.first());
        if errors.len() > 0 {
            *errors = &errors[1..];
        }
    } else {
        eprintln!("{:?}", n);
    }
    for node in n.children_with_tokens() {
        match node {
            NodeOrToken::Node(n) => print(&n, indent + 2, errors),
            NodeOrToken::Token(t) => {
                for _ in 0..indent {
                    eprint!(" ");
                }
                eprintln!("  {:?} (token)", t);
            }
        }
    }
}

fn find_range(token: SyntaxNode) -> std::ops::Range<usize> {
    let mut start = token.text_range();

    let is_empty = |token: &NodeOrToken<rowan::SyntaxNode<Lang>, SyntaxToken<Lang>>| {
        let kind = token.kind();
        kind == testing::SyntaxKind::Error || kind == testing::SyntaxKind::WhiteSpace
    };

    if let Some(mut start_find) = token.prev_sibling_or_token() {
        while is_empty(&start_find) {
            start = start_find.text_range();
            if let Some(pr) = start_find.prev_sibling_or_token() {
                start_find = pr;
            } else {
                break;
            }
        }
    }
    let mut end = token.text_range();

    if let Some(mut end_find) = token.next_sibling_or_token() {
        while is_empty(&end_find) {
            end = end_find.text_range();
            if let Some(pr) = end_find.next_sibling_or_token() {
                end_find = pr;
            } else {
                break;
            }
        }
    }

    let e: usize = end.end().into();
    start.start().into()..e + 1
}

fn print_ariadne(errors: &[(Range<usize>, &String)], source: &str) {
    if errors.is_empty() {
        return;
    }

    use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
    let loc = "sample.ttl";

    let s = errors.iter().map(|x| x.0.start()).min().unwrap_or_default();
    let e = errors.iter().map(|x| x.0.end()).max().unwrap_or_default();

    let mut colors = ColorGenerator::from_state([10000, 15000, 15000], 0.8);

    let report = errors.iter().rev().fold(
        Report::build(ReportKind::Error, (loc, s..e)),
        |report, (span, e)| {
            report.with_label(
                Label::new((loc, span.clone()))
                    .with_message(e)
                    .with_color(colors.next()),
            )
        },
    );
    println!("report {:?}", report);

    report.finish().print((loc, Source::from(source))).unwrap();
}

// TODO:
//x1. errors -> spans
// 2. code cleanup (less in macro's more in implementation)
// 3. locational autocomplete
//   1. next token (at)
// 4. tests?
// 5. timing information?
// 6. context aware parsing!
fn main() {
    let s: &'static str = "";

    println!("Got: {}", s);
    let sexps = "   [    ] . ";
    println!("Parsing {}", sexps);
    let parse: turtle::Parse = parse_t::<turtle::testing::TurtleDoc>(sexps);

    let root = parse.syntax();

    let mut errors: Vec<_> = parse.errors.iter().cloned().collect();
    errors.reverse();
    let error_nodes: Vec<_> = root
        .descendants()
        .filter(|x| x.kind() == testing::SyntaxKind::Error)
        .flat_map(|x| match root.token_at_offset(x.text_range().start()) {
            rowan::TokenAtOffset::None => None,
            rowan::TokenAtOffset::Single(x) => Some(x.text_range().into()),
            rowan::TokenAtOffset::Between(a, b) => {
                Some(a.text_range().start().into()..b.text_range().start().into())
            }
        })
        .zip(errors.iter())
        .collect();

    print_ariadne(&error_nodes, sexps);

    let mut es: &[String] = &errors;
    println!("All errors {:?}", error_nodes);
    print(&root, 0, &mut es);
}
