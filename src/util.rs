use std::collections::HashSet;

use rowan::{NodeOrToken, SyntaxToken, TextSize};

use crate::TokenTrait;

pub fn completion<L: rowan::Language>(n: &rowan::SyntaxNode<L>, at: usize) -> HashSet<L::Kind>
where
    L::Kind: TokenTrait + 'static,
{
    let at = TextSize::new(at as u32);

    let mut out = HashSet::new();
    if let Some(end) = n
        .descendants()
        .filter(|x| x.text_range().end() == at)
        .next()
    {
        println!("end {:?}", end);
        out.extend(end.kind().ending_tokens().iter());
    }

    if let Some(end) = n
        .descendants()
        .filter(|x| x.text_range().start() == at)
        .next()
    {
        println!("start {:?}", end);

        out.extend(end.kind().starting_tokens().iter());
    }

    if let Some(end) = n.token_at_offset(at).left_biased() {
        println!("self {:?}", end);
        out.extend(end.kind().ending_tokens().iter());
    }

    out
}

pub fn print<L: rowan::Language>(n: &rowan::SyntaxNode<L>, indent: usize, errors: &mut &[String])
where
    L::Kind: TokenTrait + 'static,
{
    for _ in 0..indent {
        eprint!(" ");
    }
    if n.kind() == L::Kind::ERROR {
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

pub fn find_range<L: rowan::Language>(token: &rowan::SyntaxNode<L>) -> std::ops::Range<usize>
where
    L::Kind: TokenTrait + 'static,
{
    let mut start = token.text_range();

    let is_empty = |token: &NodeOrToken<rowan::SyntaxNode<L>, SyntaxToken<L>>| {
        let kind = token.kind();
        kind.skips()
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
