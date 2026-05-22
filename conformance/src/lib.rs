use std::collections::HashMap;

use rdf_parsers::model::*;
use rdf_parsers::{parse as rdf_parse, parse_fast, tokenize, Spanned};
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

// ── public test runner entry points ─────────────────────────────────────────

pub fn run_turtle_positive_syntax(path: &str) {
    let src = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("cannot read {}", path));
    let no_lex_errors = !has_lex_error_turtle(&src);
    let parse_ok = parse_fast(
        turtle::parser::Rule::new(turtle::parser::SyntaxKind::TurtleDoc),
        &src,
    )
    .is_some();
    assert!(
        no_lex_errors && parse_ok,
        "expected valid Turtle syntax in {}",
        path
    );
}

pub fn run_turtle_negative_syntax(path: &str) {
    let src = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("cannot read {}", path));
    let has_lex_err = has_lex_error_turtle(&src);
    let parse_fails = parse_fast(
        turtle::parser::Rule::new(turtle::parser::SyntaxKind::TurtleDoc),
        &src,
    )
    .is_none();
    assert!(
        has_lex_err || parse_fails,
        "expected invalid Turtle syntax in {}",
        path
    );
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

    let mut norm = Normalizer::new(base_iri, &doc);
    let got = norm.normalize_turtle(&doc);
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
    let src = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("cannot read {}", path));
    let no_lex_errors = !has_lex_error_trig(&src);
    let parse_ok = parse_fast(
        trig::parser::Rule::new(trig::parser::SyntaxKind::TrigDoc),
        &src,
    )
    .is_some();
    assert!(
        no_lex_errors && parse_ok,
        "expected valid TriG syntax in {}",
        path
    );
}

pub fn run_trig_negative_syntax(path: &str) {
    let src = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("cannot read {}", path));
    let has_lex_err = has_lex_error_trig(&src);
    let parse_fails = parse_fast(
        trig::parser::Rule::new(trig::parser::SyntaxKind::TrigDoc),
        &src,
    )
    .is_none();
    assert!(
        has_lex_err || parse_fails,
        "expected invalid TriG syntax in {}",
        path
    );
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

    let mut norm = Normalizer::new(base_iri, &doc);
    let got = norm.normalize_turtle(&doc);
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

// ── normalizer ───────────────────────────────────────────────────────────────

struct Normalizer {
    prefixes: HashMap<String, String>,
    base: String,
    counter: usize,
}

impl Normalizer {
    fn new(file_base: &str, doc: &Turtle) -> Self {
        let mut prefixes = HashMap::new();
        for p in &doc.prefixes {
            let p = p.value();
            let name = p.prefix.value().clone();
            if let NamedNode::Full(iri, _) = p.value.value() {
                prefixes.insert(name, resolve_iri(&unescape_iri(iri), file_base));
            }
        }

        let base = if let Some(b) = &doc.base {
            if let NamedNode::Full(iri, _) = b.value().1.value() {
                resolve_iri(&unescape_iri(iri), file_base)
            } else {
                file_base.to_string()
            }
        } else {
            file_base.to_string()
        };

        Normalizer { prefixes, base, counter: 0 }
    }

    fn fresh_bnode(&mut self) -> String {
        let n = self.counter;
        self.counter += 1;
        format!("__gen_{}", n)
    }

    fn expand_named_node(&self, nn: &NamedNode) -> Option<String> {
        match nn {
            NamedNode::Full(iri, _) => Some(resolve_iri(&unescape_iri(iri), &self.base)),
            NamedNode::Prefixed { prefix, value, .. } => {
                let base_iri = self.prefixes.get(prefix)?;
                Some(format!("{}{}", base_iri, unescape_local_name(value)))
            }
            NamedNode::A(_) => {
                Some("http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string())
            }
            NamedNode::Invalid => None,
        }
    }

    fn expand_term(
        &mut self,
        term: &Term,
        extra: &mut Vec<NQuad>,
        graph: &Option<NTerm>,
    ) -> Option<NTerm> {
        match term {
            Term::NamedNode(nn) => self.expand_named_node(nn).map(NTerm::Iri),
            Term::BlankNode(bn) => self.expand_blank_node(bn, extra, graph),
            Term::Collection(items) => self.expand_collection(items, extra, graph),
            Term::Literal(lit) => self.expand_literal(lit),
            _ => None,
        }
    }

    fn expand_blank_node(
        &mut self,
        bn: &BlankNode,
        extra: &mut Vec<NQuad>,
        graph: &Option<NTerm>,
    ) -> Option<NTerm> {
        match bn {
            BlankNode::Named(name, _) => Some(NTerm::BlankNode(name.clone())),
            BlankNode::Unnamed(pos, _, _) => {
                let label = self.fresh_bnode();
                let node = NTerm::BlankNode(label);
                for po in pos {
                    let po = po.value();
                    let pred = self.expand_term(po.predicate.value(), extra, graph)?;
                    for obj in &po.object {
                        let obj_t = self.expand_term(obj.value(), extra, graph)?;
                        extra.push((node.clone(), pred.clone(), obj_t, graph.clone()));
                    }
                }
                Some(node)
            }
            BlankNode::Invalid => None,
        }
    }

    fn expand_collection(
        &mut self,
        items: &[Spanned<Term>],
        extra: &mut Vec<NQuad>,
        graph: &Option<NTerm>,
    ) -> Option<NTerm> {
        let rdf_first = NTerm::Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#first".to_string());
        let rdf_rest = NTerm::Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest".to_string());
        let rdf_nil = NTerm::Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil".to_string());

        if items.is_empty() {
            return Some(rdf_nil);
        }

        let bnodes: Vec<NTerm> = (0..items.len())
            .map(|_| NTerm::BlankNode(self.fresh_bnode()))
            .collect();

        for (i, item) in items.iter().enumerate() {
            let item_term = self.expand_term(item.value(), extra, graph)?;
            let next = if i + 1 < bnodes.len() {
                bnodes[i + 1].clone()
            } else {
                rdf_nil.clone()
            };
            extra.push((bnodes[i].clone(), rdf_first.clone(), item_term, graph.clone()));
            extra.push((bnodes[i].clone(), rdf_rest.clone(), next, graph.clone()));
        }

        Some(bnodes[0].clone())
    }

    fn expand_literal(&self, lit: &Literal) -> Option<NTerm> {
        match lit {
            Literal::RDF(rdf) => {
                let val = unescape(&rdf.value);
                let lang = rdf.lang.as_ref().map(|l| l.to_lowercase());
                let dt = if let Some(nn) = &rdf.ty {
                    self.expand_named_node(nn)
                } else if lang.is_some() {
                    Some(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".to_string(),
                    )
                } else {
                    Some("http://www.w3.org/2001/XMLSchema#string".to_string())
                };
                Some(NTerm::Literal(val, lang, dt))
            }
            Literal::Boolean(b) => Some(NTerm::Literal(
                b.to_string(),
                None,
                Some("http://www.w3.org/2001/XMLSchema#boolean".to_string()),
            )),
            Literal::Numeric(n) => {
                let n = n.trim().to_string();
                let dt = infer_xsd_type(&n).to_string();
                Some(NTerm::Literal(n, None, Some(dt)))
            }
        }
    }

    fn normalize_turtle(&mut self, doc: &Turtle) -> Vec<NQuad> {
        let mut out: Vec<NQuad> = Vec::new();

        for triple_sp in &doc.triples {
            let triple = triple_sp.value();

            let graph: Option<NTerm> = triple.graph.as_ref().and_then(|g| {
                let mut dummy: Vec<NQuad> = Vec::new();
                self.expand_term(g.value(), &mut dummy, &None)
            });

            let mut extra: Vec<NQuad> = Vec::new();
            let subj = match self.expand_term(triple.subject.value(), &mut extra, &graph) {
                Some(t) => t,
                None => continue,
            };
            out.extend(extra.drain(..));

            for po_sp in &triple.po {
                let po = po_sp.value();
                let mut extra2: Vec<NQuad> = Vec::new();
                let pred = match self.expand_term(po.predicate.value(), &mut extra2, &graph) {
                    Some(t) => t,
                    None => continue,
                };
                out.extend(extra2.drain(..));

                for obj_sp in &po.object {
                    let mut extra3: Vec<NQuad> = Vec::new();
                    let obj = match self.expand_term(obj_sp.value(), &mut extra3, &graph) {
                        Some(t) => t,
                        None => continue,
                    };
                    out.extend(extra3.drain(..));
                    out.push((subj.clone(), pred.clone(), obj, graph.clone()));
                }
            }
        }

        out
    }
}

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
            let dt = if let Some(NamedNode::Full(iri, _)) = &rdf.ty {
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

fn resolve_iri(r: &str, base: &str) -> String {
    if r.is_empty() {
        return base.to_string();
    }
    // Already has a scheme: first ':' must come before any '/' or '?'
    if let Some(colon) = r.find(':') {
        let before = &r[..colon];
        if !before.is_empty()
            && before.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '-' || c == '.')
        {
            return r.to_string();
        }
    }
    if r.starts_with("//") {
        let scheme = base.find(':').map(|i| &base[..i]).unwrap_or("https");
        return remove_dot_segments(&format!("{}:{}", scheme, r));
    }
    if r.starts_with('/') {
        let origin = base_origin(base);
        return remove_dot_segments(&format!("{}{}", origin, r));
    }
    if r.starts_with('?') {
        let base_path = base.split('?').next().unwrap_or(base);
        let base_path = base_path.split('#').next().unwrap_or(base_path);
        return format!("{}{}", base_path, r);
    }
    if r.starts_with('#') {
        let base_no_frag = base.split('#').next().unwrap_or(base);
        return format!("{}{}", base_no_frag, r);
    }
    // Relative path
    let base_dir = base_directory(base);
    remove_dot_segments(&format!("{}{}", base_dir, r))
}

fn base_origin(base: &str) -> &str {
    if let Some(p) = base.find("://") {
        let after_scheme = &base[p + 3..];
        let path_start = after_scheme
            .find('/')
            .map(|i| p + 3 + i)
            .unwrap_or(base.len());
        &base[..path_start]
    } else {
        ""
    }
}

fn base_directory(base: &str) -> &str {
    let no_qf = base.split('?').next().unwrap_or(base);
    let no_qf = no_qf.split('#').next().unwrap_or(no_qf);
    match no_qf.rfind('/') {
        Some(i) => &base[..=i],
        None => base,
    }
}

fn remove_dot_segments(input: &str) -> String {
    // Split into scheme+authority and path+rest
    let (prefix, rest) = if let Some(p) = input.find("://") {
        let after = &input[p + 3..];
        let path_start = after.find('/').map(|i| p + 3 + i).unwrap_or(input.len());
        (input[..path_start].to_string(), &input[path_start..])
    } else {
        (String::new(), input)
    };

    // Separate path from query/fragment
    let (path, suffix) = match rest.find(|c| c == '?' || c == '#') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, ""),
    };

    let mut out: Vec<&str> = Vec::new();
    let mut remaining = path;
    loop {
        if remaining.is_empty() {
            break;
        }
        if remaining == "." || remaining == ".." {
            break;
        }
        if remaining.starts_with("../") {
            remaining = &remaining[3..];
        } else if remaining.starts_with("./") {
            remaining = &remaining[2..];
        } else if remaining.starts_with("/./") || remaining == "/." {
            remaining = if remaining == "/." { "/" } else { &remaining[2..] };
        } else if remaining.starts_with("/../") || remaining == "/.." {
            remaining = if remaining == "/.." { "/" } else { &remaining[3..] };
            out.pop();
        } else {
            let seg_end = if remaining.starts_with('/') {
                remaining[1..].find('/').map(|i| i + 1).unwrap_or(remaining.len())
            } else {
                remaining.find('/').unwrap_or(remaining.len())
            };
            out.push(&remaining[..seg_end]);
            remaining = &remaining[seg_end..];
        }
    }

    format!("{}{}{}", prefix, out.join(""), suffix)
}

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

fn unescape_local_name(s: &str) -> String {
    if !s.contains('\\') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                out.push(next);
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn infer_xsd_type(n: &str) -> &'static str {
    if n.to_lowercase().contains('e') {
        "http://www.w3.org/2001/XMLSchema#double"
    } else if n.contains('.') {
        "http://www.w3.org/2001/XMLSchema#decimal"
    } else {
        "http://www.w3.org/2001/XMLSchema#integer"
    }
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
