pub mod parser;

pub mod convert;
pub mod format;
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
        crate::turtle::format::format(&root, width)
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
        // Every predicate after the first goes on its own line, even when
        // everything would fit.
        let input = r#"<a> <b> <c>; <d> <e> ."#;
        let out = format_turtle(input, 80);
        assert_eq!(out, "<a> <b> <c>;\n  <d> <e>.\n");
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
    fn format_multiple_objects_with_newlines() {
        let input = r#"<a> <b> <c>, <d>, <e>, <f>, <g> . "#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "<a> <b> <c>,\n  <d>,\n  <e>,\n  <f>,\n  <g>.\n");
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
        assert_eq!(out, "<a> <b> <c>.\n<d> <e> <f>.\n");
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
        assert_eq!(out, "[\n  <a> <b>;\n  <c> <d>;\n].\n");
    }

    #[test]
    fn format_bn_triple_big_multi_line() {
        // Too wide for one line: break after ';', indenting the second clause.
        let input = r#"[ <a> <b>; <c> <d>; <e> <f> ]."#;
        let out = format_turtle(input, 16);
        assert_eq!(out, "[\n  <a> <b>;\n  <c> <d>;\n  <e> <f>;\n].\n");
    }

    #[test]
    fn format_comments_end_their_line() {
        // Code after a comment on the same line would become part of the
        // comment, changing the data.
        let input = "# header\n<a> <b> <c>; # trailing\n  <d> <e>.\n<f> <g> [ # in bnode\n <h> <i> ].\n";
        let out = format_turtle(input, 80);
        for comment in ["# header", "# trailing", "# in bnode"] {
            assert!(
                out.lines().any(|l| l.ends_with(comment)),
                "{comment:?} is not at the end of a line in:\n{out}"
            );
        }
        assert_eq!(
            out,
            "# header\n<a> <b> <c>; # trailing\n  <d> <e>.\n\n<f> <g> [ # in bnode\n  <h> <i>\n].\n"
        );
    }

    #[test]
    fn format_keeps_one_blank_line() {
        let input = "<a> <b> <c>.\n\n\n\n<d> <e> <f>.\n<g> <h> <i>.\n";
        let out = format_turtle(input, 80);
        assert_eq!(out, "<a> <b> <c>.\n\n<d> <e> <f>.\n<g> <h> <i>.\n");
    }

    #[test]
    fn format_blank_line_around_multi_line_statements() {
        let input = "@prefix a: <x>. <a> <b> <c>. <d> <e> <f>; <g> <h>. <i> <j> <k>.";
        let out = format_turtle(input, 80);
        assert_eq!(
            out,
            "@prefix a: <x>.\n\n<a> <b> <c>.\n\n<d> <e> <f>;\n  <g> <h>.\n\n<i> <j> <k>.\n"
        );
    }

    #[test]
    fn format_drops_redundant_semicolons() {
        let input = "<a> <b> <c>;; <d> <e>;.";
        let out = format_turtle(input, 80);
        assert_eq!(out, "<a> <b> <c>;\n  <d> <e>.\n");
    }

    // The cases below are ported from the hand-written formatter in swls
    // (lang-turtle/src/lang/formatter.rs).

    #[test]
    fn swls_harder_format_pos() {
        let input = "\n[] a foaf:Name;\n   foaf:knows <abc>; foaf:knows2 <abc>.\n\n";
        let expected = "[ ] a foaf:Name;\n  foaf:knows <abc>;\n  foaf:knows2 <abc>.\n";
        assert_eq!(format_turtle(input, 80), expected);
    }

    #[test]
    fn swls_format_blanknodes() {
        let input = "
        [ <a> foaf:Person; foaf:knows <abc>; foaf:knows <def> ] foaf:knows [
        a foaf:Person;
        foaf:knows <abc>;
        foaf:knows <def>;
        ] .
";
        let expected = "[
  <a> foaf:Person;
  foaf:knows <abc>;
  foaf:knows <def>;
] foaf:knows [
  a foaf:Person;
  foaf:knows <abc>;
  foaf:knows <def>;
].
";
        assert_eq!(format_turtle(input, 80), expected);
    }

    #[test]
    fn swls_long_objectlist() {
        let input = "<abc> a <something-long>, <something-longer-still>, <something-longer>, <something-tes>, <soemthing-eeeellssee>.";
        let expected = "<abc> a <something-long>,
  <something-longer-still>,
  <something-longer>,
  <something-tes>,
  <soemthing-eeeellssee>.
";
        assert_eq!(format_turtle(input, 80), expected);
    }

    #[test]
    fn swls_short_collection() {
        let input = "<abc> a (), (<abc> <def>).";
        assert_eq!(format_turtle(input, 80), "<abc> a ( ), ( <abc> <def> ).\n");
    }

    #[test]
    fn swls_long_collection() {
        let input = "<abc> a (), (<somevery-very-very-long-item> <and-othersss> <and-ottteeehs> <wheeeeeeeeeeeee>).";
        let expected = "<abc> a ( ), (
  <somevery-very-very-long-item>
  <and-othersss>
  <and-ottteeehs>
  <wheeeeeeeeeeeee>
).
";
        assert_eq!(format_turtle(input, 80), expected);
    }

    #[test]
    fn swls_hard_comments() {
        let input = "

[] a foaf:Name; # Nested comment
   foaf:knows <abc>;     # Another comment!
   foaf:knows2 <abc>.

   #trailing comments
";
        // Trailing comments stay on their line (swls moved them to the next).
        let expected = "[ ] a foaf:Name; # Nested comment
  foaf:knows <abc>; # Another comment!
  foaf:knows2 <abc>.

#trailing comments
";
        assert_eq!(format_turtle(input, 80), expected);
    }

    #[test]
    fn swls_bug_1() {
        let input = r#"
[] a sh:NodeShape;
  sh:targetClass js:Echo;
  sh:property [
    sh:class :ReaderChannel;
    sh:path js:input;
    sh:name "Input Channel"
  ], [
    sh:class :WriterChannel;
    sh:path js:output;
    sh:name "Output Channel"
  ].
"#;
        let expected = r#"[ ] a sh:NodeShape;
  sh:targetClass js:Echo;
  sh:property [
    sh:class :ReaderChannel;
    sh:path js:input;
    sh:name "Input Channel";
  ], [
    sh:class :WriterChannel;
    sh:path js:output;
    sh:name "Output Channel";
  ].
"#;
        assert_eq!(format_turtle(input, 80), expected);
    }

    #[test]
    fn swls_format_nested_bnodes() {
        let input = "[] sh:or ( [ sh:property [
    sh:path crm:P4_has_time-span;
    sh:minCount 1;
  ] ] [ sh:property [
    sh:path crm:P7_took_place_at;
    sh:minCount 1;
  ] ] ).
";
        let expected = "[ ] sh:or (
  [
    sh:property [
      sh:path crm:P4_has_time-span;
      sh:minCount 1;
    ]
  ]
  [
    sh:property [
      sh:path crm:P7_took_place_at;
      sh:minCount 1;
    ]
  ]
).
";
        assert_eq!(format_turtle(input, 80), expected);
    }

    #[test]
    fn format_is_idempotent() {
        let input = "@prefix sh: <http://www.w3.org/ns/shacl#>.
# shapes
[] a sh:NodeShape; sh:property [ sh:path <p>; sh:minCount 1 ], [ sh:path <q> ];
  sh:or ( [ sh:path <r> ] ( <a> <b> ) ). # done
";
        let once = format_turtle(input, 40);
        assert_eq!(format_turtle(&once, 40), once);
    }

    #[test]
    fn format_keywords_are_separated() {
        let input = "@base <http://ex/>. BASE <http://ex/> @prefix ex: <http://ex/>. PREFIX ex2: <http://ex2/>";
        let expected = "@base <http://ex/>.\nBASE <http://ex/>\n@prefix ex: <http://ex/>.\nPREFIX ex2: <http://ex2/>\n";
        assert_eq!(format_turtle(input, 80), expected);
    }
}
