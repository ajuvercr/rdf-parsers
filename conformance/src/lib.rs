use std::collections::HashMap;

use rdf_parsers::model::*;
use rdf_parsers::{parse as rdf_parse, parse_fast, tokenize};
use rdf_parsers::{ntriples, sparql, trig, turtle};

fn has_lex_error_turtle(src: &str) -> bool {
    tokenize::<turtle::parser::SyntaxKind>(src)
        .iter()
        .any(|t| t.kind == turtle::parser::SyntaxKind::Error)
}

fn has_lex_error_trig(src: &str) -> bool {
    tokenize::<trig::parser::SyntaxKind>(src)
        .iter()
        .any(|t| t.kind == trig::parser::SyntaxKind::Error)
}

fn has_lex_error_sparql(src: &str) -> bool {
    tokenize::<sparql::parser::SyntaxKind>(src)
        .iter()
        .any(|t| t.kind == sparql::parser::SyntaxKind::Error)
}

// ── RDF via the library's resolver ──────────────────────────────────────────

/// The quads of a converted document, or the resolver's errors (undefined
/// prefixes, invalid escapes, invalid IRIs).
fn resolved_quads(doc: &Turtle, base: &str) -> Result<Vec<NQuad>, Vec<rdf_parsers::resolve::Error>> {
    use rdf_parsers::resolve::{Node, resolve};
    let r = resolve(doc, Some(base));
    if !r.errors.is_empty() {
        return Err(r.errors);
    }
    let term = |n: Node| match n {
        Node::Iri(i) => NTerm::Iri(i),
        Node::Blank(b) => NTerm::BlankNode(b),
        Node::Literal { lexical, datatype, language } => NTerm::Literal(lexical, language, Some(datatype)),
    };
    Ok(r.quads
        .into_iter()
        .map(|q| (term(q.subject), term(q.predicate), term(q.object), q.graph.map(term)))
        .collect())
}

fn turtle_doc(src: &str) -> Option<Turtle> {
    let (parse, _) = parse_fast(turtle::parser::Rule::new(turtle::parser::SyntaxKind::TurtleDoc), src)?;
    Some(turtle::convert::convert(&parse.syntax::<turtle::parser::Lang>()))
}

fn trig_doc(src: &str) -> Option<Turtle> {
    let (parse, _) = parse_fast(trig::parser::Rule::new(trig::parser::SyntaxKind::TrigDoc), src)?;
    Some(trig::convert::convert(&parse.syntax::<trig::parser::Lang>()))
}

/// Valid RDF: lexes, parses, and resolves without errors.
fn turtle_is_valid(src: &str) -> bool {
    !has_lex_error_turtle(src) && turtle_doc(src).is_some_and(|d| resolved_quads(&d, "http://example/base/").is_ok())
}

fn trig_is_valid(src: &str) -> bool {
    !has_lex_error_trig(src) && trig_doc(src).is_some_and(|d| resolved_quads(&d, "http://example/base/").is_ok())
}

// ── public test runner entry points ─────────────────────────────────────────

pub fn run_turtle_positive_syntax(path: &str) {
    let src = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("cannot read {}", path));
    assert!(turtle_is_valid(&src), "expected valid Turtle in {}", path);
}

pub fn run_turtle_negative_syntax(path: &str) {
    let src = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("cannot read {}", path));
    assert!(!turtle_is_valid(&src), "expected invalid Turtle in {}", path);
}

pub fn run_turtle_eval(action: &str, result: &str, base_iri: &str) {
    let src = std::fs::read_to_string(action)
        .unwrap_or_else(|_| panic!("cannot read {}", action));
    assert!(
        !has_lex_error_turtle(&src),
        "lexer errors in {}",
        action
    );
    let (parse, _) = rdf_parse(
        turtle::parser::Rule::new(turtle::parser::SyntaxKind::TurtleDoc),
        &src,
    );
    assert!(
        parse.errors.len() == 0,
        "parse errors in {}: {:?}",
        action,
        parse.errors
    );
    let root = parse.syntax::<turtle::parser::Lang>();
    let doc = turtle::convert::convert(&root);

    let got = resolved_quads(&doc, base_iri)
        .unwrap_or_else(|e| panic!("resolve errors in {}: {:?}", action, e));
    let expected = parse_nt_ref(result);

    assert!(
        graphs_isomorphic(got.clone(), expected.clone()),
        "eval mismatch for {}\n  got {} triples: {:?}\n  expected {} triples: {:?}",
        action,
        got.len(),
        got,
        expected.len(),
        expected
    );
}

pub fn run_trig_positive_syntax(path: &str) {
    let src = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("cannot read {}", path));
    assert!(trig_is_valid(&src), "expected valid TriG in {}", path);
}

pub fn run_trig_negative_syntax(path: &str) {
    let src = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("cannot read {}", path));
    assert!(!trig_is_valid(&src), "expected invalid TriG in {}", path);
}

pub fn run_trig_eval(action: &str, result: &str, base_iri: &str) {
    let src = std::fs::read_to_string(action)
        .unwrap_or_else(|_| panic!("cannot read {}", action));
    assert!(
        !has_lex_error_trig(&src),
        "lexer errors in {}",
        action
    );
    let (parse, _) = rdf_parse(
        trig::parser::Rule::new(trig::parser::SyntaxKind::TrigDoc),
        &src,
    );
    assert!(
        parse.errors.len() == 0,
        "parse errors in {}: {:?}",
        action,
        parse.errors
    );
    let root = parse.syntax::<trig::parser::Lang>();
    let doc = trig::convert::convert(&root);

    let got = resolved_quads(&doc, base_iri)
        .unwrap_or_else(|e| panic!("resolve errors in {}: {:?}", action, e));
    let expected = parse_nq_ref(result);

    assert!(
        graphs_isomorphic(got.clone(), expected.clone()),
        "eval mismatch for {}\n  got {} quads: {:?}\n  expected {} quads: {:?}",
        action,
        got.len(),
        got,
        expected.len(),
        expected
    );
}

// ── SPARQL runners ───────────────────────────────────────────────────────────

pub fn run_sparql_positive_syntax(path: &str) {
    let src = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("cannot read {}", path));
    let no_lex_errors = !has_lex_error_sparql(&src);
    let parse_ok = if path.ends_with(".ru") {
        parse_fast(
            sparql::parser::Rule::new(sparql::parser::SyntaxKind::UpdateUnit),
            &src,
        )
        .is_some()
    } else {
        parse_fast(
            sparql::parser::Rule::new(sparql::parser::SyntaxKind::QueryUnit),
            &src,
        )
        .is_some()
    };
    assert!(
        no_lex_errors && parse_ok,
        "expected valid SPARQL syntax in {}",
        path
    );
}

pub fn run_sparql_negative_syntax(path: &str) {
    let src = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("cannot read {}", path));
    let has_lex_err = has_lex_error_sparql(&src);
    let parse_fails = if path.ends_with(".ru") {
        parse_fast(
            sparql::parser::Rule::new(sparql::parser::SyntaxKind::UpdateUnit),
            &src,
        )
        .is_none()
    } else {
        parse_fast(
            sparql::parser::Rule::new(sparql::parser::SyntaxKind::QueryUnit),
            &src,
        )
        .is_none()
    };
    assert!(
        has_lex_err || parse_fails,
        "expected invalid SPARQL syntax in {}",
        path
    );
}

// ── normalized term / quad ───────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NTerm {
    Iri(String),
    Literal(String, Option<String>, Option<String>), // value, lang, datatype-iri
    BlankNode(String),
}

pub type NQuad = (NTerm, NTerm, NTerm, Option<NTerm>);

// ── reference file parsers ───────────────────────────────────────────────────

pub fn parse_nt_ref(path: &str) -> Vec<NQuad> {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("cannot read {}", path));
    let (parse, _) = rdf_parse(
        ntriples::parser::Rule::new(ntriples::parser::SyntaxKind::NtriplesDoc),
        &content,
    );
    let root = parse.syntax::<ntriples::parser::Lang>();
    let doc = ntriples::convert::convert(&root);

    doc.triples
        .iter()
        .filter_map(|t| {
            let t = t.value();
            let subj = nt_term_to_nterm(t.subject.value())?;
            let po = t.po.first()?;
            let po = po.value();
            let pred = nt_term_to_nterm(po.predicate.value())?;
            let obj = nt_term_to_nterm(po.object.first()?.value())?;
            Some((subj, pred, obj, None))
        })
        .collect()
}

pub fn parse_nq_ref(path: &str) -> Vec<NQuad> {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("cannot read {}", path));
    content
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
        .filter_map(|l| parse_nquad_line(l.trim()))
        .collect()
}

fn nt_term_to_nterm(term: &Term) -> Option<NTerm> {
    match term {
        Term::NamedNode(NamedNode::Full(iri, _)) => Some(NTerm::Iri(unescape_iri(iri))),
        Term::BlankNode(BlankNode::Named(name, _)) => Some(NTerm::BlankNode(name.clone())),
        Term::Literal(Literal::RDF(rdf)) => {
            let val = unescape(&rdf.value);
            let lang = rdf.lang.as_ref().map(|l| l.to_lowercase());
            let dt = if let Some(NamedNode::Full(iri, _)) = rdf.ty.as_ref().map(|s| s.value()) {
                Some(iri.clone())
            } else if lang.is_some() {
                Some(
                    "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".to_string(),
                )
            } else {
                Some("http://www.w3.org/2001/XMLSchema#string".to_string())
            };
            Some(NTerm::Literal(val, lang, dt))
        }
        _ => None,
    }
}

fn parse_nquad_line(line: &str) -> Option<NQuad> {
    let line = line.trim_end_matches('.').trim_end();
    let mut pos = 0;
    let subj = next_term(line, &mut pos)?;
    let pred = next_term(line, &mut pos)?;
    let obj = next_term(line, &mut pos)?;
    skip_ws(line, &mut pos);
    let graph = if pos < line.len() { Some(next_term(line, &mut pos)?) } else { None };
    Some((subj, pred, obj, graph))
}

fn skip_ws(s: &str, pos: &mut usize) {
    while *pos < s.len() && (s.as_bytes()[*pos] == b' ' || s.as_bytes()[*pos] == b'\t') {
        *pos += 1;
    }
}

fn next_term(s: &str, pos: &mut usize) -> Option<NTerm> {
    skip_ws(s, pos);
    let (term, consumed) = parse_term_at(&s[*pos..])?;
    *pos += consumed;
    Some(term)
}

fn parse_term_at(s: &str) -> Option<(NTerm, usize)> {
    if s.starts_with('<') {
        let end = s.find('>')?;
        let iri = unescape(&s[1..end]);
        Some((NTerm::Iri(iri), end + 1))
    } else if s.starts_with("_:") {
        let end = s[2..]
            .find(|c: char| c.is_ascii_whitespace())
            .map(|i| i + 2)
            .unwrap_or(s.len());
        Some((NTerm::BlankNode(s[2..end].to_string()), end))
    } else if s.starts_with('"') {
        let mut i = 1;
        let b = s.as_bytes();
        while i < b.len() {
            match b[i] {
                b'\\' => i += 2,
                b'"' => break,
                _ => i += 1,
            }
        }
        let value = unescape(&s[1..i]);
        let rest = &s[i + 1..];
        let (lang, dt, extra) = if rest.starts_with('@') {
            let end = rest[1..]
                .find(|c: char| c.is_ascii_whitespace())
                .map(|x| x + 1)
                .unwrap_or(rest.len());
            (
                Some(rest[1..end].to_lowercase()),
                Some("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".to_string()),
                end,
            )
        } else if rest.starts_with("^^<") {
            let r2 = &rest[3..];
            let end = r2.find('>')?;
            (None, Some(unescape(&r2[..end])), 3 + end + 1)
        } else {
            (None, Some("http://www.w3.org/2001/XMLSchema#string".to_string()), 0)
        };
        Some((NTerm::Literal(value, lang, dt), i + 1 + extra))
    } else {
        None
    }
}

// ── IRI resolution (RFC 3986 §5.2.2) ─────────────────────────────────────────

// ── escape sequence processing ───────────────────────────────────────────────

fn unescape(s: &str) -> String {
    if !s.contains('\\') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('b') => out.push('\u{0008}'),
            Some('f') => out.push('\u{000C}'),
            Some('"') => out.push('"'),
            Some('\'') => out.push('\''),
            Some('\\') => out.push('\\'),
            Some('u') => {
                let hex: String = chars.by_ref().take(4).collect();
                match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                    Some(c) => out.push(c),
                    None => {
                        out.push('\\');
                        out.push('u');
                        out.push_str(&hex);
                    }
                }
            }
            Some('U') => {
                let hex: String = chars.by_ref().take(8).collect();
                match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                    Some(c) => out.push(c),
                    None => {
                        out.push('\\');
                        out.push('U');
                        out.push_str(&hex);
                    }
                }
            }
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

fn unescape_iri(iri: &str) -> String {
    if !iri.contains('\\') {
        return iri.to_string();
    }
    let mut out = String::with_capacity(iri.len());
    let mut chars = iri.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.peek() {
            Some('u') => {
                chars.next();
                let hex: String = chars.by_ref().take(4).collect();
                match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                    Some(ch) => out.push(ch),
                    None => {
                        out.push('\\');
                        out.push('u');
                        out.push_str(&hex);
                    }
                }
            }
            Some('U') => {
                chars.next();
                let hex: String = chars.by_ref().take(8).collect();
                match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                    Some(ch) => out.push(ch),
                    None => {
                        out.push('\\');
                        out.push('U');
                        out.push_str(&hex);
                    }
                }
            }
            _ => out.push(c),
        }
    }
    out
}

// ── graph isomorphism ────────────────────────────────────────────────────────

fn has_bnode(q: &NQuad) -> bool {
    is_bnode(&q.0)
        || is_bnode(&q.1)
        || is_bnode(&q.2)
        || q.3.as_ref().map(is_bnode).unwrap_or(false)
}

fn is_bnode(t: &NTerm) -> bool {
    matches!(t, NTerm::BlankNode(_))
}

fn apply_mapping(q: &NQuad, m: &HashMap<String, String>) -> NQuad {
    let mt = |t: &NTerm| -> NTerm {
        if let NTerm::BlankNode(name) = t {
            if let Some(mapped) = m.get(name) {
                return NTerm::BlankNode(mapped.clone());
            }
        }
        t.clone()
    };
    (mt(&q.0), mt(&q.1), mt(&q.2), q.3.as_ref().map(mt))
}

/// The same quads with every blank node label prefixed by `ns` (`#` cannot
/// occur in a label, so the namespaces are disjoint).
fn namespaced(quads: Vec<NQuad>, ns: &str) -> Vec<NQuad> {
    let t = |t: NTerm| match t {
        NTerm::BlankNode(n) => NTerm::BlankNode(format!("{ns}{n}")),
        t => t,
    };
    quads.into_iter().map(|(s, p, o, g)| (t(s), t(p), t(o), g.map(t))).collect()
}

fn try_iso(
    a: &[NQuad],
    b: &[NQuad],
    mapping: &HashMap<String, String>,
    a_bnodes: &[String],
    b_bnodes: &[String],
) -> bool {
    if a.is_empty() {
        return b.is_empty();
    }

    let first = apply_mapping(&a[0], mapping);

    for idx in 0..b.len() {
        let mut new_mapping = mapping.clone();
        if quads_can_match(&first, &b[idx], &mut new_mapping, a_bnodes, b_bnodes) {
            let remaining: Vec<NQuad> = b
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, q)| q.clone())
                .collect();
            if try_iso(&a[1..], &remaining, &new_mapping, a_bnodes, b_bnodes) {
                return true;
            }
        }
    }
    false
}

fn quads_can_match(
    a: &NQuad,
    b: &NQuad,
    mapping: &mut HashMap<String, String>,
    a_bnodes: &[String],
    b_bnodes: &[String],
) -> bool {
    term_can_match(&a.0, &b.0, mapping, a_bnodes, b_bnodes)
        && term_can_match(&a.1, &b.1, mapping, a_bnodes, b_bnodes)
        && term_can_match(&a.2, &b.2, mapping, a_bnodes, b_bnodes)
        && match (&a.3, &b.3) {
            (None, None) => true,
            (Some(at), Some(bt)) => term_can_match(at, bt, mapping, a_bnodes, b_bnodes),
            _ => false,
        }
}

fn term_can_match(
    a: &NTerm,
    b: &NTerm,
    mapping: &mut HashMap<String, String>,
    a_bnodes: &[String],
    b_bnodes: &[String],
) -> bool {
    match (a, b) {
        (NTerm::BlankNode(an), NTerm::BlankNode(bn)) => {
            if let Some(mapped) = mapping.get(an) {
                // `an` is already mapped; verify it maps to bn
                mapped == bn
            } else if !a_bnodes.contains(an) {
                // `an` came from a prior substitution; compare literally
                an == bn
            } else if b_bnodes.contains(bn) && !mapping.values().any(|v| v == bn) {
                // `bn` is a free b-variable; bind an → bn
                mapping.insert(an.clone(), bn.clone());
                true
            } else {
                false
            }
        }
        _ => a == b,
    }
}

pub fn graphs_isomorphic(a: Vec<NQuad>, b: Vec<NQuad>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    // The matcher substitutes mapped labels of `a` by labels of `b`, so the
    // two sides must not share labels (`_:b1` in both used to break it).
    let (a, b) = (namespaced(a, "A#"), namespaced(b, "B#"));

    let (mut a_ground, a_nonground): (Vec<_>, Vec<_>) =
        a.into_iter().partition(|q| !has_bnode(q));
    let (mut b_ground, b_nonground): (Vec<_>, Vec<_>) =
        b.into_iter().partition(|q| !has_bnode(q));

    a_ground.sort();
    b_ground.sort();
    if a_ground != b_ground {
        return false;
    }

    if a_nonground.is_empty() && b_nonground.is_empty() {
        return true;
    }

    let mut a_bnodes: Vec<String> = Vec::new();
    let mut b_bnodes: Vec<String> = Vec::new();

    for q in &a_nonground {
        for t in [&q.0, &q.1, &q.2] {
            if let NTerm::BlankNode(n) = t {
                if !a_bnodes.contains(n) {
                    a_bnodes.push(n.clone());
                }
            }
        }
        if let Some(g) = &q.3 {
            if let NTerm::BlankNode(n) = g {
                if !a_bnodes.contains(n) {
                    a_bnodes.push(n.clone());
                }
            }
        }
    }
    for q in &b_nonground {
        for t in [&q.0, &q.1, &q.2] {
            if let NTerm::BlankNode(n) = t {
                if !b_bnodes.contains(n) {
                    b_bnodes.push(n.clone());
                }
            }
        }
        if let Some(g) = &q.3 {
            if let NTerm::BlankNode(n) = g {
                if !b_bnodes.contains(n) {
                    b_bnodes.push(n.clone());
                }
            }
        }
    }

    try_iso(&a_nonground, &b_nonground, &HashMap::new(), &a_bnodes, &b_bnodes)
}
