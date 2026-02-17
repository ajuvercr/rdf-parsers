use turtle::{SyntaxNode, parse_t};

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
            rowan::NodeOrToken::Node(n) => print(&n, indent + 2, errors),
            rowan::NodeOrToken::Token(t) => {
                for _ in 0..indent {
                    eprint!(" ");
                }
                eprintln!("  {:?}", t);
            }
        }
    }
}

fn main() {
    let s: &'static str = "";

    println!("Got: {}", s);
    let sexps = "a  [ b ;   d .";
    println!("Parsing {}", sexps);
    let parse: turtle::Parse = parse_t::<turtle::testing::TurtleDoc>(sexps);

    let root = parse.syntax();

    let mut errors: Vec<_> = parse.errors.iter().cloned().collect();
    errors.reverse();
    let mut es: &[String] = &errors;
    println!("All errors {:?}", errors);
    print(&root, 0, &mut es);
}
