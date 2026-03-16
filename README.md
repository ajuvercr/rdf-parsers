# rdf-parsers

An error-tolerant RDF/SPARQL parser using an A\* search algorithm to find the lowest-cost parse of potentially malformed input.

## Architecture

Grammars are defined in BNF files (`grammars/turtle.txt`, `grammars/sparql.txt`) and compiled into Rust source by the `xtask` code generator. The generated files (`src/turtle.rs`, `src/sparql.rs`) are checked in — run `cargo xtask codegen` after changing a grammar to regenerate them. Each grammar produces a single `Rule { kind: SyntaxKind, state: usize }` struct. The A\* search explores parse states in best-first order, assigning error costs to mismatched tokens.

The output is a [rowan](https://github.com/rust-analyzer/rowan) concrete syntax tree that preserves all tokens including whitespace and errors.

## Known Limitations

### Fingerprint-based deduplication is approximate

The A\* search deduplicates parse states using a 128-bit fingerprint computed from the sequence of grammar rule *kinds* pushed onto the parse stack (`SyntaxKind` variant only), combined with the current token index and the top rule's internal state number.

This means two parse paths that arrive at the same top-of-stack (same rule kind + state, same token position) via different intermediate rule states deeper in the stack are treated as identical and merged. In an unambiguous grammar this is sound. For ambiguous grammars, valid parse paths can be silently discarded — only the highest-scoring path reaching any given state survives.
