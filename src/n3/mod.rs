pub mod parser;

pub mod convert;
pub mod format;
pub use parser::*;
#[cfg(test)]
mod format_tests {
    use crate::{
        n3::{Lang, Rule, SyntaxKind},
        parse,
    };

    fn format_n3(input: &str, width: usize) -> String {
        let (result, _) = parse(Rule::new(SyntaxKind::N3Doc), input);
        crate::n3::format::format(&result.syntax::<Lang>(), width)
    }

    /// Formats `input` and checks that formatting the output changes nothing.
    fn check(input: &str, expected: &str) {
        let out = format_n3(input, 80);
        assert_eq!(out, expected);
        assert_eq!(format_n3(&out, 80), out, "formatting is not idempotent");
    }

    #[test]
    fn format_short_rule_stays_on_one_line() {
        check(
            "@prefix ex: <http://ex/>. # rules\n{ ?x ex:parent ?y . ?y ex:parent ?z } => { ?x ex:grandparent ?z } .",
            "@prefix ex: <http://ex/>. # rules\n\n{ ?x ex:parent ?y. ?y ex:parent ?z } => { ?x ex:grandparent ?z }.\n",
        );
    }

    #[test]
    fn format_long_rule_breaks_formulas() {
        check(
            "PREFIX ex: <http://ex/>\n\n# long rule\n{ ?person ex:livesIn ?city . ?city ex:locatedIn ?country . ?country ex:continent ex:Europe } => { ?person a ex:European ; ex:region ?country } .",
            "PREFIX ex: <http://ex/>

# long rule
{
  ?person ex:livesIn ?city.
  ?city ex:locatedIn ?country.
  ?country ex:continent ex:Europe
} => {
  ?person a ex:European;
    ex:region ?country
}.
",
        );
    }

    #[test]
    fn format_turtle_layout() {
        check(
            "ex:a ex:p ex:b; ex:q [ ex:r ex:s; ex:t ( 1 2 ) ], ex:c .",
            "ex:a ex:p ex:b;
  ex:q [
    ex:r ex:s;
    ex:t ( 1 2 );
  ], ex:c.
",
        );
    }

    #[test]
    fn format_n3_verbs_and_paths() {
        check(
            "ex:a!ex:p^ex:q ex:r ex:s ; <- ex:s2 ex:o ; = ex:t ; has ex:u ex:v ; is ex:w of ex:x .",
            "ex:a!ex:p^ex:q ex:r ex:s;
  <- ex:s2 ex:o;
  = ex:t;
  has ex:u ex:v;
  is ex:w of ex:x.
",
        );
    }

    #[test]
    fn format_iri_property_list() {
        check(
            "[ id ex:b ex:c ex:d ; ex:e ex:f ] ex:g ex:h .",
            "[ id ex:b\n  ex:c ex:d;\n  ex:e ex:f;\n] ex:g ex:h.\n",
        );
    }
}
