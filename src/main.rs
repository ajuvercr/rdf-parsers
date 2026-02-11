use turtle::{SyntaxKind, SyntaxNode, parse};

fn print(n: &SyntaxNode, indent: usize, mut errors: &[String]) {
    for _ in 0..indent {
        eprint!(" ");
    }
    if n.kind() == SyntaxKind::ERROR {
        eprintln!("{:?} {}", n, errors[0]);
        errors = &errors[1..];
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
    let sexps = "a b c d .";
    println!("Parsing {}", sexps);
    let parse = parse(sexps);

    let root = parse.syntax();

    print(&root, 0, &parse.errors);
}
