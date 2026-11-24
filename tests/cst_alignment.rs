//! The tree of a recovered parse holds every token once, in order, each in a
//! node of its own kind (or in an error node).
//!
//! A sync-ahead recovery used to skip tokens without recording them, so every
//! later token landed in the node meant for the token after it, and the last
//! ones were left over as `Unparsed`. These tests type into documents one
//! keystroke at a time, as an editor does, and check every parse.

use rdf_parsers::{IncrementalBias, PrevParseInfo, parse_incremental};

/// Ok, or the first token that sits alone in a node of another kind, or an
/// `Unparsed` error.
macro_rules! alignment {
    ($lang:ident, $root:ident, $text:expr, $prev:expr) => {{
        use rdf_parsers::$lang::{Lang, Rule, SyntaxKind};
        let (parse, info) = parse_incremental(
            Rule::new(SyntaxKind::$root),
            $text,
            $prev,
            IncrementalBias::default(),
        );
        let mut problem = parse
            .errors
            .iter()
            .find(|e| e.starts_with("Unparsed"))
            .map(|e| format!("{e} in {:?}", $text));
        let root = parse.syntax::<Lang>();
        let other = |k: SyntaxKind| {
            matches!(
                k,
                SyntaxKind::WhiteSpace | SyntaxKind::Comment | SyntaxKind::Error
            )
        };
        for t in root
            .descendants_with_tokens()
            .filter_map(|e| e.into_token())
        {
            let Some(node) = t.parent() else { continue };
            let alone = node
                .children_with_tokens()
                .filter(|c| !other(c.kind()))
                .count()
                == 1;
            if problem.is_none()
                && !other(t.kind())
                && alone
                && node.parent().is_some()
                && node.kind() != SyntaxKind::Error
                && node.kind() != t.kind()
            {
                problem = Some(format!(
                    "token {:?} {:?} in a {:?} node, in {:?}",
                    t.kind(),
                    t.text(),
                    node.kind(),
                    $text
                ));
            }
        }
        (problem, info)
    }};
}

/// The demo's query, with `foaf:foaf [` typed on a new line above
/// `foaf:name ?name .`.
#[test]
fn sparql_unclosed_bracket_typed_in() {
    let before = "PREFIX ex: <http://example.org/>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>

SELECT ?person ?name ?age
WHERE {
  ?person a foaf:Person ;
          foaf:name ?name .
  OPTIONAL { ?person foaf:age ?age . }
  FILTER(?name != \"\")
}
ORDER BY ?name
";
    let (problem, mut prev): (Option<String>, PrevParseInfo) =
        alignment!(sparql, QueryUnit, before, None);
    assert_eq!(problem, None);
    let at = before.find("          foaf:name").unwrap();
    let mut text = before.to_string();
    for (i, c) in "          foaf:foaf [\n".char_indices() {
        text.insert(at + i, c);
        let (problem, info) = alignment!(sparql, QueryUnit, &text, Some(&prev));
        assert_eq!(problem, None);
        prev = info;
    }
}

/// Snippets typed at pseudo-random places of a Turtle document.
#[test]
fn turtle_random_typing() {
    let base = "@prefix ex: <http://example.org/> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .

ex:alice a foaf:Person ;
    foaf:name \"Alice\"@en ;
    foaf:knows [ foaf:name \"Bob\" ; foaf:age 42 ] , ex:carol ;
    ex:likes ( ex:tea \"coffee\" 1.5 ) .

[] ex:says \"hello; world.\" ;
    ex:about _:b1 .
_:b1 ex:rank -3 ; ex:ok true .
";
    let snippets = [
        "[",
        " ]",
        " ;",
        " .",
        "(",
        " )",
        "<x> ",
        "ex:p ",
        "\"ab",
        " [ ex:q 1",
        "_:b ",
        "a ",
    ];
    let mut seed: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next = |n: usize| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % n as u64) as usize
    };
    for _ in 0..8 {
        let (_, mut prev) = alignment!(turtle, TurtleDoc, base, None);
        let mut text = base.to_string();
        for _ in 0..3 {
            let mut at = next(text.len() + 1);
            while !text.is_char_boundary(at) {
                at -= 1;
            }
            for c in snippets[next(snippets.len())].chars() {
                text.insert(at, c);
                at += c.len_utf8();
                let (problem, info) = alignment!(turtle, TurtleDoc, &text, Some(&prev));
                assert_eq!(problem, None);
                prev = info;
            }
        }
    }
}
