pub mod parser;

pub mod convert;
pub use parser::*;

#[cfg(test)]
mod format_tests {
    use crate::{
        parse,
        sparql::{Lang, Rule, SyntaxKind},
    };

    fn format_sparql(input: &str, width: usize) -> String {
        let rule = Rule::new(SyntaxKind::QueryUnit);
        let (result, _) = parse(rule, input);
        let root = result.syntax::<Lang>();
        crate::sparql::parser::format::format(&root, width)
    }

    #[test]
    fn format_simple_select() {
        let input = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";
        let out = format_sparql(input, 80);
        assert_eq!(out, "SELECT ?s ?p ?o\nWHERE {\n  ?s ?p ?o\n}\n");
    }

    #[test]
    fn format_select_with_prefix() {
        let input = "PREFIX foaf: <http://xmlns.com/foaf/0.1/> SELECT ?name WHERE { ?s foaf:name ?name }";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "PREFIX foaf: <http://xmlns.com/foaf/0.1/>\n\nSELECT ?name\nWHERE {\n  ?s foaf:name ?name\n}\n"
        );
    }

    #[test]
    fn format_select_with_semicolon() {
        let input = "SELECT ?s WHERE { ?s <a> <b>; <c> <d> }";
        let out = format_sparql(input, 80);
        assert_eq!(out, "SELECT ?s\nWHERE {\n  ?s <a> <b>; <c> <d>\n}\n");
    }

    #[test]
    fn format_select_with_filter() {
        let input = "SELECT ?s WHERE { ?s <a> ?o . FILTER(?o > 5) }";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "SELECT ?s\nWHERE {\n  ?s <a> ?o.\n  FILTER (?o>5)\n}\n"
        );
    }

    #[test]
    fn format_select_with_optional() {
        let input = "SELECT ?s ?o WHERE { ?s <a> <b> . OPTIONAL { ?s <c> ?o } }";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "SELECT ?s ?o\nWHERE {\n  ?s <a> <b>.\n  OPTIONAL {\n    ?s <c> ?o\n  }\n\n}\n"
        );
    }

    #[test]
    fn format_select_with_order_by() {
        let input = "SELECT ?s WHERE { ?s <a> <b> } ORDER BY ?s LIMIT 10";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "SELECT ?s\nWHERE {\n  ?s <a> <b>\n}\nORDER BY ?s\nLIMIT 10\n"
        );
    }

    #[test]
    fn format_select_with_group_having() {
        let input = "SELECT ?s (COUNT(?o) AS ?c) WHERE { ?s <a> ?o } GROUP BY ?s HAVING(?c > 1)";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "SELECT ?s (COUNT(?o) AS  ?c)\nWHERE {\n  ?s <a> ?o\n}\nGROUP BY ?s\nHAVING (?c>1)\n"
        );
    }

    #[test]
    fn format_select_with_union() {
        let input = "SELECT ?s WHERE { { ?s <a> <b> } UNION { ?s <c> <d> } }";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "SELECT ?s\nWHERE {\n  {\n    ?s <a> <b>\n  }\n\n  UNION\n  {\n    ?s <c> <d>\n  }\n\n}\n"
        );
    }

    #[test]
    fn format_construct_query() {
        let input = "CONSTRUCT { ?s <a> ?o } WHERE { ?s <b> ?o }";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "CONSTRUCT {\n  ?s <a> ?o\n}\n\nWHERE {\n  ?s <b> ?o\n}\n"
        );
    }

    #[test]
    fn format_select_breaking() {
        // At width 40, the SELECT line breaks but the WHERE clause triples still fit flat.
        let input = "SELECT ?subject ?predicate ?object WHERE { ?subject ?predicate ?object }";
        let out = format_sparql(input, 40);
        assert_eq!(
            out,
            "SELECT ?subject ?predicate ?object\nWHERE {\n  ?subject ?predicate ?object\n}\n"
        );
    }

    #[test]
    fn format_ask_query() {
        let input = "ASK WHERE { ?s <a> <b> }";
        let out = format_sparql(input, 80);
        assert_eq!(out, "ASK\nWHERE {\n  ?s <a> <b>\n}\n");
    }

    #[test]
    fn format_describe_query() {
        let input = "DESCRIBE <http://example.org/resource>";
        let out = format_sparql(input, 80);
        assert_eq!(out, "DESCRIBE <http://example.org/resource>\n");
    }

    #[test]
    fn format_select_distinct_star() {
        let input = "SELECT DISTINCT * WHERE { ?s ?p ?o }";
        let out = format_sparql(input, 80);
        assert_eq!(out, "SELECT DISTINCT *\nWHERE {\n  ?s ?p ?o\n}\n");
    }

    #[test]
    fn format_multiple_prefixes() {
        let input = "PREFIX foaf: <http://xmlns.com/foaf/0.1/> PREFIX dc: <http://purl.org/dc/elements/1.1/> SELECT ?name WHERE { ?s foaf:name ?name }";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "PREFIX foaf: <http://xmlns.com/foaf/0.1/>\nPREFIX dc: <http://purl.org/dc/elements/1.1/>\n\nSELECT ?name\nWHERE {\n  ?s foaf:name ?name\n}\n"
        );
    }

    #[test]
    fn format_order_by_desc() {
        let input = "SELECT ?s WHERE { ?s <a> ?o } ORDER BY DESC(?o) LIMIT 5 OFFSET 10";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "SELECT ?s\nWHERE {\n  ?s <a> ?o\n}\nORDER BY DESC(?o)\nLIMIT 5\nOFFSET 10\n"
        );
    }

    #[test]
    fn format_select_with_bind() {
        let input = "SELECT ?s ?label WHERE { ?s <a> ?o . BIND(CONCAT(?o, \"_suffix\") AS ?label) }";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "SELECT ?s ?label\nWHERE {\n  ?s <a> ?o.\n  BIND (CONCAT(?o, \"_suffix\") AS ?label)\n}\n"
        );
    }

    #[test]
    fn format_select_with_minus() {
        let input = "SELECT ?s WHERE { ?s <a> <b> . MINUS { ?s <c> <d> } }";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "SELECT ?s\nWHERE {\n  ?s <a> <b>.\n  MINUS {\n    ?s <c> <d>\n  }\n\n}\n"
        );
    }

    #[test]
    fn format_select_with_graph() {
        let input = "SELECT ?s WHERE { GRAPH <http://example.org/g> { ?s <a> <b> } }";
        let out = format_sparql(input, 80);
        assert_eq!(
            out,
            "SELECT ?s\nWHERE {\n  GRAPH <http://example.org/g> {\n    ?s <a> <b>\n  }\n\n}\n"
        );
    }

    #[test]
    fn format_property_list_breaking() {
        // Semicolons force a line break when the property list is too wide.
        let input = "SELECT ?s WHERE { ?s <very-long-predicate-one> <object-one>; <very-long-predicate-two> <object-two> }";
        let out = format_sparql(input, 40);
        assert_eq!(
            out,
            "SELECT ?s\nWHERE {\n  ?s <very-long-predicate-one> <object-one>;\n  <very-long-predicate-two> <object-two>\n}\n"
        );
    }

    #[test]
    fn format_multiple_objects_in_list() {
        let input = "SELECT ?s WHERE { ?s <a> <b>, <c>, <d> }";
        let out = format_sparql(input, 80);
        assert_eq!(out, "SELECT ?s\nWHERE {\n  ?s <a> <b>, <c>, <d>\n}\n");
    }

    #[test]
    fn format_multiple_objects_in_list_wrapping() {
        let input = "SELECT ?s WHERE { ?s <a> <b>, <c>, <d>, <e>, <f> }";
        let out = format_sparql(input, 20);
        assert_eq!(
            out,
            "SELECT ?s\nWHERE {\n  ?s <a> <b>,\n  <c>,\n  <d>,\n  <e>,\n  <f>\n}\n"
        );
    }
}
