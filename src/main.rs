use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::Range,
};

use ariadne::Span;
use rowan::{NodeOrToken, SyntaxToken, TextSize};
use turtle::{
    parse_t,
    turtle::{self as lang, Lang, SyntaxKind, SyntaxNode},
};

fn completion(n: &SyntaxNode, at: usize) -> HashSet<SyntaxKind> {
    let at = TextSize::new(at as u32);

    let mut out = HashSet::new();
    if let Some(end) = n
        .descendants()
        .filter(|x| x.text_range().end() == at)
        .next()
    {
        println!("end {:?}", end);
        out.extend(lang::ending_tokens(end.kind()).iter());
    }

    if let Some(end) = n
        .descendants()
        .filter(|x| x.text_range().start() == at)
        .next()
    {
        println!("start {:?}", end);
        out.extend(lang::starting_tokens(end.kind()).iter());
    }

    if let Some(end) = n.token_at_offset(at).left_biased() {
        println!("sefl {:?}", end);
        out.extend(lang::ending_tokens(end.kind()).iter());
    }

    out
}

fn print(n: &SyntaxNode, indent: usize, errors: &mut &[String]) {
    for _ in 0..indent {
        eprint!(" ");
    }
    if n.kind() == lang::SyntaxKind::Error {
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
        kind == lang::SyntaxKind::Error || kind == lang::SyntaxKind::WhiteSpace
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
    let sexps = read_to_string("./test.ttl").unwrap();
    println!("Parsing {}", sexps);
    let parse: turtle::Parse = parse_t::<turtle::turtle::TurtleDoc>(&sexps);

    let root = parse.syntax();

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
