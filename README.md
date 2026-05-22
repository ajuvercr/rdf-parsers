# rdf-parsers

An error-tolerant RDF/SPARQL parser using an A\* search algorithm to find the lowest-cost parse of potentially malformed input.

## Architecture

Grammars are defined in BNF files (`grammars/turtle.txt`, `grammars/sparql.txt`) and compiled into Rust source by the `xtask` code generator. The generated files (`src/turtle.rs`, `src/sparql.rs`) are checked in — run `cargo xtask codegen` after changing a grammar to regenerate them. Each grammar produces a single `Rule { kind: SyntaxKind, state: usize }` struct. The A\* search explores parse states in best-first order, assigning error costs to mismatched tokens.

The output is a [rowan](https://github.com/rust-analyzer/rowan) concrete syntax tree that preserves all tokens including whitespace and errors.

## Conformance

The `conformance` crate runs W3C conformance tests from the [rdf-tests](https://github.com/w3c/rdf-tests) repository. Run them with:

```
cargo test -p conformance --test turtle_conformance
cargo test -p conformance --test trig_conformance
cargo test -p conformance --test sparql_conformance
```

### Results

| Suite | Passing | Total | % |
|---|---|---|---|
| Turtle (RDF 1.1) | 280 | 296 | 95% |
| TriG (RDF 1.1) | 318 | 334 | 95% |
| SPARQL 1.0 / 1.1 | 319 | 372 | 86% |

### Why some tests fail

The failures fall into two categories.

**Post-lex semantic constraints not checked by the parser.**
The W3C test suites label these as "negative syntax tests", but they require validation steps beyond parsing:

- *Surrogate code points in string escapes* — `"\ud800"` is lexically valid (the `\uXXXX` regex matches any four hex digits) but U+D800–U+DFFF are Unicode surrogates and must be rejected. The parser does not validate the numeric range of `\u` escapes.
- *Invalid characters in IRI escapes* — `< >` passes the lexer (the escape is well-formed) but expands to a space character which is forbidden inside an IRI. Similarly for `<<>` (less-than) and `<{>` (brace).
- *Undeclared prefix used as subject* — `:s <p> <o> .` without a preceding `@prefix :` declaration is syntactically valid per the grammar; the check that every prefix has been declared is semantic.
- *Blank node reuse across graph boundaries* (SPARQL) — reusing `_:x` in two separate graph patterns is forbidden by the SPARQL spec but accepted by the grammar.
- *Blank nodes in DELETE templates* (SPARQL Update) — `DELETE { ?a :p [] }` is syntactically well-formed; the prohibition on anonymous blank nodes in DELETE is a semantic rule.
- *Nested aggregates / ungrouped variables* (SPARQL) — `COUNT(COUNT(?x))` or projecting a variable not in the GROUP BY clause is a semantic error, not a grammar error.

**Model limitation: only the last `@base` is stored.**
The `Turtle` struct records a single base IRI (the last `@base` directive seen). The test `turtle-subm-27` (and the equivalent trig test) changes the base URI several times within one document and expects each set of triples to be resolved against the base that was active at the time. Fixing this requires tracking base URI changes per-statement in the model.

## Known Limitations

### Fingerprint-based deduplication is approximate

The A\* search deduplicates parse states using a 128-bit fingerprint computed from the sequence of grammar rule *kinds* pushed onto the parse stack (`SyntaxKind` variant only), combined with the current token index and the top rule's internal state number.

This means two parse paths that arrive at the same top-of-stack (same rule kind + state, same token position) via different intermediate rule states deeper in the stack are treated as identical and merged. In an unambiguous grammar this is sound. For ambiguous grammars, valid parse paths can be silently discarded — only the highest-scoring path reaching any given state survives.
