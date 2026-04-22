pub mod parser;

pub mod convert;
pub use parser::*;

#[cfg(test)]
mod format_tests {
    use crate::{
        parse,
        turtle::{Lang, Rule, SyntaxKind},
    };

    fn format_turtle(input: &str, width: usize) -> String {
        let rule = Rule::new(SyntaxKind::TurtleDoc);
        let (result, _) = parse(rule, input);
        let root = result.syntax::<Lang>();
        crate::turtle::parser::format::format(&root, width)
    }

    #[test]
    fn format_multiple_objects() {
        // Both inner and outer fit at wide width — both stay flat.
        let input = r#"<a> <b> <c>,<d>  ."#;
        let out = format_turtle(input, 80);
        assert_eq!(out, "<a> <b> <c>, <d>.\n");
    }
    #[test]
    fn format_multiple_predicate_objects() {
        // Both inner and outer fit at wide width — both stay flat.
        let input = r#"<a> <b> <c>; <d> <e> ."#;
        let out = format_turtle(input, 80);
        assert_eq!(out, "<a> <b> <c>; <d> <e>.\n");
    }

    #[test]
    fn format_multiple_predicate_objects_with_newlines() {
        // Too wide for one line: break after ';', indenting the second clause.
        let input = r#"<a> <b> <c>; <d> <e> ."#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "<a> <b> <c>;\n  <d> <e>.\n");

        let input = r#"<a> <b> <c>; <d> <e>; <f> <g> ."#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "<a> <b> <c>;\n  <d> <e>;\n  <f> <g>.\n");
    }

    #[test]
    fn format_multiple_predicate_objects_with_newlines_bn_subject() {
        let input = r#"[ ] <b> <c>; <d> <e>; <f> <g> ."#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "[ ] <b> <c>;\n  <d> <e>;\n  <f> <g>.\n");
    }

    #[test]
    fn format_multiple_triples_with_newlines() {
        let input = r#"<a> <b> <c>; <d> <e>; <f> <g> . <a> <b> <c>; <d> <e>; <f> <g> ."#;
        let out = format_turtle(input, 16);
        assert_eq!(
            out,
            "<a> <b> <c>;\n  <d> <e>;\n  <f> <g>.\n\n<a> <b> <c>;\n  <d> <e>;\n  <f> <g>.\n"
        );
    }

    #[test]
    fn format_directies() {
        // Too wide for one line: break after ';', indenting the second clause.
        let input = r#"@prefix foaf: <> ."#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "@prefix foaf: <>.\n");
    }

    #[test]
    fn format_multiple_triples() {
        // Too wide for one line: break after ';', indenting the second clause.
        let input = r#"<a> <b> <c>. <d> <e> <f> ."#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "<a> <b> <c>.\n\n<d> <e> <f>.\n");
    }
    #[test]
    fn format_multiple_directives() {
        // Too wide for one line: break after ';', indenting the second clause.
        let input = r#"@prefix foaf: <> . @prefix foaf2: <> . "#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "@prefix foaf: <>.\n@prefix foaf2: <>.\n");
    }

    #[test]
    fn format_directive_then_triple() {
        // Too wide for one line: break after ';', indenting the second clause.
        let input = r#"@prefix foaf: <> . <a> <b> <c>. "#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "@prefix foaf: <>.\n\n<a> <b> <c>.\n");
    }

    #[test]
    fn format_bn_triple() {
        // Too wide for one line: break after ';', indenting the second clause.
        let input = r#"[ <a> <b> ]."#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "[ <a> <b> ].\n");
    }

    #[test]
    fn format_bn_triple_big() {
        // Too wide for one line: break after ';', indenting the second clause.
        let input = r#"[ <a> <b>; <c> <d> ]."#;
        let out = format_turtle(input, 24);
        assert_eq!(out, "[ <a> <b>; <c> <d> ].\n");
    }

    #[test]
    fn format_bn_triple_big_multi_line() {
        // Too wide for one line: break after ';', indenting the second clause.
        let input = r#"[ <a> <b>; <c> <d>; <e> <f> ]."#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "[\n  <a> <b>;\n  <c> <d>;\n  <e> <f>\n].\n");
    }
}
