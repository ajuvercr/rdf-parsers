//! The engines side by side. They search the same graph with the same costs
//! and differ in how much of it they explore: `Global` prunes, `Exact`
//! doesn't, `Local` commits to a repair once it has held for a window of
//! tokens.

use rdf_parsers::turtle::{Rule, SyntaxKind};
use rdf_parsers::{
    DEFAULT_LOCAL_WINDOW, Engine, ParseOptions, last_commits, parse_incremental_with,
};

const LOCAL: Engine = Engine::Local {
    window: DEFAULT_LOCAL_WINDOW,
};

/// The errors of a cold parse, and the commits it made.
fn errors(text: &str, engine: Engine) -> (Vec<String>, usize) {
    let options = ParseOptions {
        engine,
        ..ParseOptions::default()
    };
    let (parse, _) = parse_incremental_with(Rule::new(SyntaxKind::TurtleDoc), text, None, options);
    (parse.errors.iter().cloned().collect(), last_commits())
}

/// A document with a missing `,` near the top and a missing object near the
/// end, twenty statements apart.
fn two_distant_errors() -> String {
    let mut text = String::from("@prefix ex: <http://example.org/> .\nex:a ex:p ex:b ex:c .\n");
    for i in 0..20 {
        text.push_str(&format!("ex:s{i} ex:p ex:o{i} .\n"));
    }
    text.push_str("ex:x ex:p .\n");
    text
}

#[test]
fn every_engine_repairs_both_errors() {
    let text = two_distant_errors();
    let (global, _) = errors(&text, Engine::Global);
    assert_eq!(global.len(), 2, "{global:?}");
    for engine in [Engine::Exact, LOCAL] {
        assert_eq!(errors(&text, engine).0, global, "{engine:?}");
    }
}

#[test]
fn local_engine_commits_between_errors() {
    let text = two_distant_errors();
    assert_eq!(errors(&text, Engine::Global).1, 0);
    assert!(errors(&text, LOCAL).1 >= 1);
}

#[test]
fn valid_documents_need_no_engine() {
    let text = "@prefix ex: <http://example.org/> .\nex:a ex:p [ ex:q ( 1 2 ) ] .\n";
    for engine in [Engine::Global, Engine::Exact, LOCAL] {
        assert_eq!(errors(text, engine).0, Vec::<String>::new(), "{engine:?}");
    }
}
