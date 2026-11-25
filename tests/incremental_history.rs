//! `parse_incremental` on documents that stay broken across edits.
//!
//! 1. An error earlier in the document does not make an edit further down
//!    expensive: the fault-tolerant search resumes after it.
//! 2. While a statement is broken, its unchanged tokens keep the history of
//!    the last version in which the statement parsed cleanly, not the roles
//!    the latest recovery happened to give them.
//! 3. History records the bracket depth each token had in the parse, the
//!    depth the next search compares against.

use rdf_parsers::turtle::{Rule, SyntaxKind};
use rdf_parsers::{IncrementalBias, PrevParseInfo, last_iterations, parse_incremental};

fn parse(text: &str, prev: Option<&PrevParseInfo>) -> (rdf_parsers::Parse, PrevParseInfo) {
    parse_incremental(
        Rule::new(SyntaxKind::TurtleDoc),
        text,
        prev,
        IncrementalBias::default(),
    )
}

fn doc(n: usize) -> String {
    let mut s = String::from(
        "@prefix ex: <http://example.org/> .\n@prefix foaf: <http://xmlns.com/foaf/0.1/> .\n",
    );
    for i in 0..n {
        s.push_str(&format!(
            "ex:s{i} a foaf:Person ;\n  foaf:name \"n{i}\" ;\n  foaf:knows ex:s{} .\n",
            i + 1
        ));
    }
    s
}

/// Deletes the first occurrence of `what` at or after byte `from`.
fn delete(text: &str, what: &str, from: usize) -> String {
    let at = from + text[from..].find(what).unwrap();
    format!("{}{}", &text[..at], &text[at + what.len()..])
}

#[test]
fn test_edits_break_what_they_should() {
    use rdf_parsers::parse_fast;
    let ok = |t: &str| parse_fast(Rule::new(SyntaxKind::TurtleDoc), t).is_some();
    let v1 = "@prefix ex: <http://example.org/> .\nex:a ex:p ex:b .\nex:c ex:q ex:d , ex:e .\nex:f ex:r ex:g .\n";
    let v2 = v1.replacen(" ex:q", "", 1);
    let v3 = v2.replacen(" ex:d", "", 1);
    let v4 = v3.replacen("ex:c ,", "ex:c ex:s ex:t ,", 1);
    assert!(ok(v1) && !ok(&v2) && !ok(&v3) && ok(&v4));
}

#[test]
fn edit_below_an_earlier_error_is_parsed_locally() {
    let n = 200;
    let valid = doc(n);
    // An unrelated error near the top: a statement lost its `.`.
    let broken = valid.replacen("ex:s2 .", "ex:s2", 1);
    let (_, prev) = parse(&broken, None);
    // Delete an object near the end.
    let edited = delete(&broken, &format!("\"n{}\"", n - 5), 0);

    let (inc, _) = parse(&edited, Some(&prev));
    let inc_iterations = last_iterations();
    let (fresh, _) = parse(&edited, None);
    let fresh_iterations = last_iterations();

    assert_eq!(
        inc.syntax::<rdf_parsers::turtle::Lang>().to_string(),
        edited
    );
    // Both errors are still reported.
    assert!(inc.errors.len() >= 2, "{:?}", inc.errors);
    assert_eq!(inc.errors.len(), fresh.errors.len());
    // The search started near the edit, not at the first error.
    assert!(
        inc_iterations * 10 < fresh_iterations,
        "incremental {inc_iterations} iterations, fresh {fresh_iterations}"
    );
}

/// Token texts and history fingerprints of a parse, trivia included.
fn history(prev: &PrevParseInfo) -> Vec<(String, Option<rdf_parsers::Fingerprint>)> {
    prev.tokens
        .iter()
        .map(|t| (t.text.clone(), t.fingerprint))
        .collect()
}

#[test]
fn broken_statement_keeps_history_of_its_last_clean_version() {
    let v1 = "@prefix ex: <http://example.org/> .\nex:a ex:p ex:b .\nex:c ex:q ex:d , ex:e .\nex:f ex:r ex:g .\n";
    let (_, p1) = parse(v1, None);
    let clean = history(&p1);

    // v2: the second statement loses its predicate; v3: then its first
    // object too.  It stays broken; the other statements stay clean.
    let v2 = v1.replacen(" ex:q", "", 1);
    let (_, p2) = parse(&v2, Some(&p1));
    let v3 = v2.replacen(" ex:d", "", 1);
    let (_, p3) = parse(&v3, Some(&p2));

    for (version, prev, removed) in [(&v2, &p2, 2), (&v3, &p3, 4)] {
        let h = history(prev);
        assert_eq!(h.len(), clean.len() - removed);
        // Every token outside the edited spot is unchanged: before it at the
        // same index, after it shifted by the removed tokens.  All of them,
        // in the broken statement too, carry v1's fingerprint.
        let first_change = clean.iter().zip(&h).take_while(|(a, b)| a.0 == b.0).count();
        for (i, (text, fp)) in h.iter().enumerate() {
            let j = if i < first_change { i } else { i + removed };
            assert_eq!(*text, clean[j].0, "token {i} of:\n{version}");
            assert_eq!(*fp, clean[j].1, "history of {text:?} in:\n{version}");
        }
    }

    // v4: the statement is whole again; its history is this parse's own.
    let v4 = v3.replacen("ex:c ,", "ex:c ex:s ex:t ,", 1);
    let (_, p4) = parse(&v4, Some(&p3));
    let (_, fresh) = parse(&v4, None);
    assert_eq!(history(&p4), history(&fresh));
}

/// The case from the public dev seed `catalog.ttl` where carrying the last
/// clean history matters.  v1 types `( ` before a language tag, and the
/// recovery for that version reads the following statement wrongly.  v2
/// deletes an object; without the clean history, v2 starts from v1's
/// reading, and the damage spreads into the next statement.
#[test]
fn damage_from_one_broken_keystroke_does_not_carry_over() {
    use rdf_parsers::turtle::Lang;
    let v0 = r#"@prefix dcat: <http://www.w3.org/ns/dcat#> .
@prefix dct: <http://purl.org/dc/terms/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@base <http://data.example.org/catalog/> .

<> a dcat:Catalog ;
    dct:title "Open transport data"@en ;
    dcat:dataset <bus-stops>, <timetables> .

<bus-stops> a dcat:Dataset ;
    dct:title "Bus stops"@en, "Bushaltes"@nl ;
    dct:issued "2024-03-01"^^xsd:date ;
    dcat:keyword "transport", "bus", "stops" .
"#;
    let next = "<bus-stops> a dcat:Dataset ;\n    dct:title \"Bus stops\"@en, \"Bushaltes\"@nl ;\n    dct:issued \"2024-03-01\"^^xsd:date ;\n    dcat:keyword \"transport\", \"bus\", \"stops\" .";
    let v1 = v0.replacen("data\"@en", "data\"( @en", 1);
    let v2 = v1.replacen("<bus-stops>, ", ", ", 1);
    let (_, p0) = parse(v0, None);
    let (_, p1) = parse(&v1, Some(&p0));
    let (p2, _) = parse(&v2, Some(&p1));

    // The statement after the edited one is untouched by both edits: it must
    // come out as one clean statement of its own.
    let root = p2.syntax::<Lang>();
    let found = root.children().any(|st| {
        st.text().to_string().trim() == next
            && !st.descendants().any(|n| n.kind() == SyntaxKind::Error)
    });
    assert!(
        found,
        "the next statement was pulled into the broken one:\n{root:#?}"
    );
}

/// While a `[` is still open, the repair closes it, and the statements after
/// it are parsed at depth 0.  History used to record the text's depth (1)
/// instead.  The next search then saw those statements moved a level up, and
/// adopting that shift skips the role check: a subject there could be read
/// as an object without a role conflict.
#[test]
fn history_records_the_depth_the_parse_gave_a_token() {
    let text = "@prefix ex: <http://example.org/> .\n\
                ex:a ex:p [ a ex:D .\n\
                ex:b ex:q ex:c .\n";
    let (parse, info) = parse(text, None);
    assert_eq!(
        parse.errors.iter().collect::<Vec<_>>(),
        ["Expected(SqClose)"],
        "expected the repair to close the bracket before the `.`"
    );
    for (i, t) in info.tokens.iter().enumerate().skip_while(|(_, t)| t.text != "ex:D").skip(1) {
        if !t.text.trim().is_empty() {
            assert_eq!(t.depth, 0, "depth of {:?} (token {i}) after the closed bracket", t.text);
        }
    }
}

/// Types `pair` one character at a time in front of `next_pair` in `before`
/// (after a `;` and a line break), parsing each version with the history of
/// the one before.  Once `name` is typed whole it must be a predicate, and
/// `next_pair` must stay in the statement of `subject` throughout.
fn type_pair_before(before: &str, next_pair: &str, pair: &str, name: &str, subject: &str) {
    use rdf_parsers::turtle::Lang;
    let at = before.find(next_pair).unwrap();
    let (_, mut prev) = parse(before, None);
    let mut text = before.to_string();
    for (i, c) in pair.char_indices() {
        text.insert(at + i, c);
        let (p, next) = parse(&text, Some(&prev));
        let root = p.syntax::<Lang>();
        let token = |word: &str| {
            root.descendants_with_tokens()
                .filter_map(|e| e.into_token())
                .find(|t| t.text() == word)
        };
        let typed = &text[at..at + i + c.len_utf8()];
        if typed.contains(name) {
            let role = token(name).and_then(|t| {
                t.parent_ancestors()
                    .map(|n| format!("{:?}", n.kind()))
                    .find(|k| matches!(k.as_str(), "Subject" | "Verb" | "Object" | "Error"))
            });
            assert_eq!(role.as_deref(), Some("Verb"), "{name} after typing {typed:?}");
        }
        let first = next_pair.split_whitespace().next().unwrap();
        let statement = token(first)
            .and_then(|t| t.parent_ancestors().find(|n| format!("{:?}", n.kind()) == "Statement"))
            .map(|n| n.text().to_string())
            .unwrap_or_default();
        assert!(
            statement.contains(subject),
            "after typing {typed:?}, {first} left {subject}'s statement: {statement:?}"
        );
        prev = next;
    }
}

/// A new pair typed after a `;`, before the next pair, the way an editor sees
/// it: Enter after the `;`, then `foaf:age 42 ;` one character at a time.
/// Until its own `;` is typed, the half-typed pair could also be read as a
/// statement of its own, `foaf:age foaf:knows ex:Bob .`, with a `.` inserted
/// after Alice's `;`.  That repair is cheaper, but it moves `foaf:knows
/// ex:Bob` to another subject, and the editor would suggest predicates where
/// an object is wanted.  So the new name stays a predicate of Alice, and
/// `foaf:knows` stays in her statement.
#[test]
fn new_pair_typed_before_the_next_one_stays_in_the_statement() {
    let before = "@prefix foaf: <http://xmlns.com/foaf/0.1/> .\n\
                  @prefix ex: <http://example.org/> .\n\
                  ex:Alice a foaf:Person ;\n    foaf:knows ex:Bob .\n";
    type_pair_before(before, "    foaf:knows", "    foaf:age 42 ;\n", "foaf:age", "ex:Alice");
}

/// The same in a statement that never parses cleanly (`--x` is not a token),
/// so its history comes from the last keystroke's parse, in which the next
/// pair followed the half-typed name.  That name changes with every
/// keystroke, so the next pair still belongs after the `;`.
#[test]
fn new_pair_typed_into_a_broken_statement_stays_in_it() {
    let before = "@prefix : <http://example.org/> .\n\
                  :dave :name \"Dave\"@en--x ;\n    :nick ( \"dave\" ) ;\n    :mbox <mailto:dave@example.org> .\n";
    type_pair_before(before, "    :nick", "    :since 2019 ;\n", ":since", ":dave");
}
