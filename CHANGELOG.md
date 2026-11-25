# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.5.0 (2026-11-25)

### New Features

 - <csr-id-ac8327b7d1ef191468b75af53bc7ebfba58b35ee/> under construct costs, delete a run of junk as one mistake
   Typing an IRI in front of `SELECT` gives fragments like `< http: / /`, each
   a valid token (less-than, a prefix, division). At 4 per deletion, deleting
   them cost more than reading them as operators in an invented projection
   expression, which then also deleted the real `SELECT`.
   
   Now the first deleted token costs `CONSTRUCT_DELETE` (4) and each token
   deleted right after it `CONSTRUCT_DELETE_MORE` (1). The discount only goes
   to tokens without a clean role: new text, or text that has not been part of
   a statement that parsed (anchored history). Valid text keeps its full price.
   A version that discounted every token deleted the valid `PREFIX` line,
   because the per-token deletion cost is also what protects valid text.
   Using clean roles rather than the previous parse's roles matters as well:
   once a broken parse reads the junk as operators, it has roles, but still no
   clean one. There's no discount without a previous version.
   
   Whether a path is in a run is part of the state key, since it changes what
   the next deletion costs.
   
   Typing `<http://x/> ` before `SELECT`, keystroke by keystroke: `<http:/` and
   `<http://` now delete the junk and keep `SELECT` (1.1k and 1.8k search
   iterations, against 29k and 38k for the expression reading). The worst
   keystroke takes 13.5k iterations, against 49k, about what the token weights
   need (12.7k). The other probe cases are unchanged.
 - <csr-id-2472b7a73831289fe005cd2500789dd46e74bbd3/> construct costs for SPARQL, N3 and JSON-LD, with a distance table
   `=== construct_cost ===` sections for the other grammars, on Turtle's scale:
   
   - 20 for optional top-level constructs: SPARQL's BASE/PREFIX declarations
     and update operations, N3's directives.
   - 10 for keyword-led clauses inside a query: OPTIONAL, FILTER, BIND,
     VALUES, GRAPH, SERVICE, MINUS, ORDER/GROUP BY, HAVING, LIMIT, OFFSET,
     EXISTS, aggregates and built-in calls, sub-selects, FROM/USING.
   - 3 for inventing brackets: SPARQL groups, blank nodes and collections, N3
     formulas, blank nodes and collections, JSON objects and arrays.
   - 3 for SPARQL's query forms. A query must have exactly one, so a penalty
     there never decides between readings. It only made the unavoidable repair
     expensive when a `SELECT` gets mangled (`<h` typed against it lexes as
     `hSELECT`), which sent the search to its iteration cap.
   
   The generator now also computes the distance heuristic (`state_dist`) in
   construct costs: inserted tokens count 1, and rules invented on the way add
   their penalty. Before, construct costs ran without a distance heuristic.
   Typing `FILTER` into a SELECT clause takes 1.2k search iterations on its
   worst keystroke, against 18.8k with the token weights.
 - <csr-id-f65f7bdc3b3f63a81667595b6a56439d9fd8005e/> add a construct cost model for recovery
   `ParseOptions.costs` chooses what a repair costs. `Costs::Tokens` (the
   default) keeps the grammar's per-token error values. `Costs::Constructs`
   prices a repair by what it does to the document:
   
   - Every inserted token costs 1, and every deleted token `CONSTRUCT_DELETE`
     (4). So the parser invents up to three missing tokens to keep a token the
     user typed, and deletes a stray one without searching far.
   - A construct the repair invents (a rule entered with its first token
     inserted, not typed) adds the penalty the grammar gives it in a new
     `=== construct_cost ===` section. Turtle and TriG penalise inventing a
     blank node or collection (3) and a directive (20); TriG also a graph
     block (3). A construct that starts after deleted tokens is not penalised.
   - History (role conflicts) stays as it is.
   
   The grammar then says how unlikely a construct is, instead of how much each
   token weighs. Under the token weights, deleting a `@prefix` typed inside a
   statement costs 500, so the parser invents seven tokens around it. The
   construct model deletes it. `tests/construct_costs.rs` covers that, a typed
   `x:` that starts a new pair, a typed directive that gets completed, and a
   lone `[` that gets closed.

### Bug Fixes

 - <csr-id-5297f15595e986c988588518299d318f95c1d082/> find a token's statement neighbour past tokens edited away
   The split check links each unchanged token to the token before it in its
   statement in the history. In a statement that never parses cleanly (here
   `"دَيْف"@ar--rtl`, RDF 1.2 that the Turtle 1.1 lexer rejects), that history
   comes from the previous keystroke's parse. While a new pair is typed after a
   `;`, that parse has the next pair right after the half-typed name (`:`),
   which the next keystroke turns into `:since`. The link then pointed at a
   token that no longer exists, so the split went unnoticed and `:since`
   became the subject of the next pair.
   
   The link now follows the history's chain of predecessors past tokens that
   were edited away, to the nearest one still there.
   
   Found with the `add-pair` traces added to the public study-3 dev kit. Over
   all six layouts, the typed terms of those traces (the ones an editor
   completes) are right in 100% of the cases (288/288), against 85.4% before
   this fix, 77.4% without the split check, and 95.8% with token weights.
   Near-edit roles 93.5% (92.8% with token weights); the other families do
   not change.
 - <csr-id-4d74ec148b68ec59728c661e443dc229749dcc49/> don't let a repair split a statement of the previous parse
   Typing a new pair after a `;`, before the next pair:
   
       ex:Alice a foaf:Person ;
           foaf:age
           foaf:knows ex:Bob .
   
   Until the pair's own `;` is typed, a repair can read `foaf:age` as a new
   predicate (insert an object and a `;`) or end Alice's statement after the
   `;` (insert one `.`) and make `foaf:age foaf:knows ex:Bob .` a statement of
   its own. The second is cheaper, and role agreement does not object: every
   old token keeps its role, but `foaf:knows ex:Bob` moves to another subject.
   An editor then suggests predicates where an object is wanted, and history
   keeps that reading for the rest of the pair.
   
   History now records, per token, the token before it in its top-level unit
   (a statement), anchored like the roles. A repair that starts a new unit
   right after an insertion or deletion, between two unchanged tokens that
   directly followed each other in one unit, splits that unit and costs as
   much as a role conflict. A split the text makes (a typed `.`) follows a
   match and stays free, and so does a new unit next to tokens that were not
   together before, such as after a string that swallowed the next statement
   while it was being typed.
   
   The Turtle completion tests now build their history with a real parse, so
   they test what an editor gets. A new test types `foaf:age 42 ;` one
   character at a time and checks every parse.
   
   Public study-3 kit, with history: no change under construct costs except
   one trace in the packed layout of the RDF 1.2 document; under token
   weights, that document's trace loses 2 roles in every layout (flicker 20 ->
   23 of 12192). p95 latency 2.9 -> 2.1 ms (construct costs), 9.0 -> 7.9 ms
   (token weights).
 - <csr-id-9012eb3dc8d9fa683f421646f5df89b6636f6ddf/> record the bracket depth a token had in the parse
   History stored each token's bracket depth from the text, counting only the
   brackets the user typed. The search compares that with its own depth, which
   also counts brackets the repair inserted. While a `[` is still open, the
   repair closes it, so every token after it was stored one level deeper than
   it was parsed. The next search then saw a block moved up a level, and
   adopting that shift skips the role check: the next statement's subject
   could become an object without a role conflict.
   
   Now `from_steps` counts inserted brackets, as the search does, and history
   stores the depth the parse gave each token. Tokens the parse did not
   consume keep the text's depth.
   
   Public study-3 kit, editing traces with history, token weights: broken-state
   roles 92.9% -> 93.5%, near-edit roles 94.4% -> 95.2% (retype-token) and
   89.6% -> 92.3% (retype-char); states all-right and flicker unchanged.
 - <csr-id-20e41d6f9754a2a223721436319208ef83cc4a07/> give sync-ahead states the right fingerprint
   When sync-ahead skips tokens, it climbs the parser stack to a rule that
   accepts the next token and keeps that rule on top. But it set the state's
   fingerprint to that rule's `saved` value, which is the fingerprint from
   before the rule was entered: the state of the rule below it. Every state
   sync-ahead produced was labelled one level too low.
   
   The search identifies states by that fingerprint, both to drop duplicates
   and to splice in the previous parse. So a mislabelled state could be taken
   for a different one. In the case that found this (an IRI typed in front of
   `SELECT`, character by character), the finished parse was dropped as a
   duplicate of its own parent state, and the search ran to its iteration cap.
   
   A debug assertion in `add_element` now checks that every state's fingerprint
   matches its stack.
   
   On the edits of the incremental reuse test (release build), SPARQL needs
   8.5M search iterations instead of 25.5M (26.3 s to 8.2 s), and none of its
   14 edits that hit the 1M cap still do. Turtle is unchanged (480 edits,
   340k to 318k iterations).
 - <csr-id-1c1b1898da820fc82df9271d76bfa1b177add26f/> make a stray clause keyword cheap to delete
   Deleting a token costs 5× its error value unless the grammar sets its own
   deletion value, so deleting `FILTER`, `WHERE`, `OPTIONAL`, `ORDER`, ...
   cost 50. The search explores every partial parse cheaper than the parse it
   returns, so one of these keywords typed where it does not belong
   (`SELECT ?person ?name FILTER`) made it explore over a million states in
   SPARQL's grammar. It then hit the iteration cap and deleted everything
   after the furthest point it had reached, reporting errors on `WHERE`,
   `ORDER`, `BY` and the rest. These keywords now cost 20 to delete, as
   `SELECT`, `ASK` and the update keywords already did.
   
   That query now parses in 19522 search iterations (18 ms, native) instead of
   1029928 (1.0 s), with one error: the stray `FILTER`.
   `tests/stray_keyword.rs` checks it.
 - <csr-id-a9d65acf6b4782bfdec45252fbec73fad781acf7/> let recovery close a lone `[` with one missing `]`
   Since `[]` is one ANON token, a `[` that is not closed yet could only start
   a property list, so recovery had to invent a whole predicate-object list
   as well as the `]`: two errors, and a large search. The list inside `[ ]`
   is now optional (Turtle and TriG `blankNodePropertyList`, SPARQL
   `BlankNodePropertyList` and `BlankNodePropertyListPath`). Valid text never
   has the empty form, because the lexer reads `[ ]` as ANON, so only
   recovery uses it: a lone `[` costs one missing `]`, as it did before ANON.
   
   Typing `foaf:foaf [` above `foaf:name ?name .` in a SPARQL WHERE block now
   gives `Expected(SqClose)` and `Expected(Colon)`, and keeps
   `foaf:name ?name` as they were, instead of inventing a list and deleting
   them. The search takes 7293 iterations instead of 38773 (8 ms instead of
   41 ms, native). The SyntaxKinds and the trees of valid documents do not
   change.
   
   SPARQL's ANON does not take comments, so in SPARQL `[ # note` + newline +
   `]` now parses as an empty property list instead of failing.
 - <csr-id-f13f5637a1e6577e02c1c839dcc16d083bb1ffc7/> keep the tree aligned with the tokens after a sync-ahead recovery
   When the search recovered by skipping ahead to a token that an enclosing
   rule accepts, the skipped tokens got no Delete steps. The steps then
   consumed fewer tokens than the text has, so the tree put every later token
   in the node meant for the token after it (in SPARQL, a `?name` in a `.`
   node, `OPTIONAL` in a `{` node, ...) and left the last tokens over as
   `Unparsed(...)` errors. The next incremental parse reused those steps, so
   the shift carried over to later edits.
   
   Each skipped token is now a Delete step: an error node in the rule that
   takes over.
   
   Typing snippets like `[`, ` ;`, `(` or `"ab` one keystroke at a time into
   the W3C Turtle test files gave a misaligned tree in 1693 of 16484 parses
   with 0.4.0 (1253 with 0.3.0), and in none with this fix.
   `tests/cst_alignment.rs` checks every parse while typing an unclosed `[`
   into a SPARQL WHERE block, and while typing snippets at random places of a
   Turtle document.

### Performance

 - <csr-id-ea4f0e1973a2de22e6378e189af698369d7e72da/> build sync-ahead steps for the chosen candidate only
   For every candidate in its 50-token window, the sync-ahead scan re-summed
   the cost of the skipped tokens and built a full step list (End steps for
   the popped rules, Delete steps for the skipped tokens), although it keeps
   only the cheapest candidate. It now keeps running totals while scanning and
   builds the steps once, for the chosen candidate. The choice is the same,
   ties included.
   
   Typing an unclosed `[` into a SPARQL WHERE block took 209 ms for that
   keystroke with 0.4.0 and takes 75 ms now (native, release build, the same
   38773 search iterations). `tests/incremental_reuse.rs` runs in 110 s
   instead of 272 s in a debug build.

### New Features (BREAKING)

 - <csr-id-2ad87db085a13020e1f1f8016e8e23b230a267fc/> construct costs are the only cost model
   Recovery prices repairs one way only: inserting a token costs 1, deleting
   one `CONSTRUCT_DELETE` (4; `CONSTRUCT_DELETE_MORE`, 1, for each further
   token of a run of junk), and a construct the repair invents adds the
   penalty in the grammar's `=== construct_cost ===` section. Role conflicts
   and statement splits cost 50, as before.
   
   Everything for per-token weights is gone:
   
   - `Costs` and `ParseOptions::costs`;
   - the grammars' `=== error_value ===` and `=== deletion_value ===`
     sections, and their parsing in xtask;
   - `TokenTrait::max_error_value`, `min_completion_cost` and
     `deletion_cost`;
   - the token-weight distance table: `ParserTrait::state_dist` is now the
     construct-cost table, which was `state_dist_constructs`.
   
   `min_error_for_token` only ever mattered as zero or not, and is now
   `can_start_with`, a bool.
   
   Every token a parse consumes, matched or deleted, costs the same constant,
   which the heuristic counts for every token ahead. It cancels out between
   paths, as the token weights did before under construct costs, so recovery
   is unchanged from `Costs::Constructs`. The public study-3 kit gives
   identical results, and the regenerated distance tables are identical in
   all six grammars. The generated parsers shrink by about 52k lines.
   
   Against the token weights (the old default), public study-3 kit, with
   history:
   
   | | token weights | construct costs |
   |-|-|-|
   | roles, valid / broken states | 93.7 / 93.5 | 94.8 / 94.9 |
   | near-edit roles, retype-token / retype-char | 95.2 / 92.3 | 97.8 / 94.9 |
   | states all-right, retype-token / retype-char | 104/186, 174/388 | 104/186, 174/388 |
   | flicker, retype-token | 20 / 12192 | 0 / 12192 |
   | latency p50 / p95 / max | 1.17 / 9.02 / 42.8 ms | 0.77 / 2.07 / 3.47 ms |
 - <csr-id-6829c6c5f10eb2a363f2b8f8277abe262b14eed1/> choose the recovery engine: global, exact or local
   `ParseOptions` has a new field `engine`. All engines search the same graph
   (match a token, insert a missing one, delete one, skip ahead to a token an
   enclosing rule accepts) with the same costs, role agreement included. They
   differ in how much of it they explore:
   
   - `Engine::Global`, the default: the search as before, pruned to stay fast
     (dominance window, a budget per token position, a cap on repairs in a
     row; pruned paths come back if everything else dead-ends).
   - `Engine::Exact`: the same search without that pruning, so the cheapest
     parse under the cost model. It shows what the pruning costs, and can be
     much slower on input with many errors.
   - `Engine::Local { window }`: once a path has repaired something (or given
     a token another role than it had before) and then parsed `window` token
     positions cleanly, and nothing cheaper is left, the search commits to it
     and drops every other path; later errors are repaired from there.
     `DEFAULT_LOCAL_WINDOW` is 24 positions, twice the dominance window:
     shorter windows commit in the middle of a cluster of errors and repair
     the rest of the cluster from there.
   
   `last_commits()` reports the commits of the last search, next to
   `last_iterations()`. `tests/cst_alignment.rs` now checks every engine, and
   `tests/engines.rs` compares them on documents with distant errors.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Find a token's statement neighbour past tokens edited away ([`5297f15`](https://github.com/ajuvercr/rdf-parsers/commit/5297f15595e986c988588518299d318f95c1d082))
    - Construct costs are the only cost model ([`2ad87db`](https://github.com/ajuvercr/rdf-parsers/commit/2ad87db085a13020e1f1f8016e8e23b230a267fc))
    - Don't let a repair split a statement of the previous parse ([`4d74ec1`](https://github.com/ajuvercr/rdf-parsers/commit/4d74ec148b68ec59728c661e443dc229749dcc49))
    - Under construct costs, delete a run of junk as one mistake ([`ac8327b`](https://github.com/ajuvercr/rdf-parsers/commit/ac8327b7d1ef191468b75af53bc7ebfba58b35ee))
    - Construct costs for SPARQL, N3 and JSON-LD, with a distance table ([`2472b7a`](https://github.com/ajuvercr/rdf-parsers/commit/2472b7a73831289fe005cd2500789dd46e74bbd3))
    - Add a construct cost model for recovery ([`f65f7bd`](https://github.com/ajuvercr/rdf-parsers/commit/f65f7bdc3b3f63a81667595b6a56439d9fd8005e))
    - Record the bracket depth a token had in the parse ([`9012eb3`](https://github.com/ajuvercr/rdf-parsers/commit/9012eb3dc8d9fa683f421646f5df89b6636f6ddf))
    - Give sync-ahead states the right fingerprint ([`20e41d6`](https://github.com/ajuvercr/rdf-parsers/commit/20e41d6f9754a2a223721436319208ef83cc4a07))
    - Make a stray clause keyword cheap to delete ([`1c1b189`](https://github.com/ajuvercr/rdf-parsers/commit/1c1b1898da820fc82df9271d76bfa1b177add26f))
    - Choose the recovery engine: global, exact or local ([`6829c6c`](https://github.com/ajuvercr/rdf-parsers/commit/6829c6c5f10eb2a363f2b8f8277abe262b14eed1))
    - Let recovery close a lone `[` with one missing `]` ([`a9d65ac`](https://github.com/ajuvercr/rdf-parsers/commit/a9d65acf6b4782bfdec45252fbec73fad781acf7))
    - Build sync-ahead steps for the chosen candidate only ([`ea4f0e1`](https://github.com/ajuvercr/rdf-parsers/commit/ea4f0e1973a2de22e6378e189af698369d7e72da))
    - Keep the tree aligned with the tokens after a sync-ahead recovery ([`f13f563`](https://github.com/ajuvercr/rdf-parsers/commit/f13f5637a1e6577e02c1c839dcc16d083bb1ffc7))
</details>

## v0.4.0 (2026-11-24)

### New Features

 - <csr-id-f5cb636514ba8b5bf3dab34d66f54b6b952ac22b/> improve incremental parsing

### New Features (BREAKING)

 - <csr-id-c151a3cb075095b5c0c865a0d7071d4c06f4f4eb/> parse an empty blank node as one ANON token in Turtle and TriG
   `[]`, with white space or comments inside, is one `ANON` token again, as in
   the W3C grammars, and `BlankNode ::= BLANK_NODE_LABEL | ANON` wherever a
   blank node can stand: subject, object and, in TriG, graph name. `object`
   follows the W3C grammar: `iri | BlankNode | collection |
   blankNodePropertyList | literal`.
   
   - An empty blank node is found the same way as `_:b`: a token of kind
   `Anon` in a `BlankNode`. It used to be a `[` `]` pair, under `BlankNode`
   as a subject but under an empty `BlankNodePropertyList2` as an object.
   - Typing inside `[ ]` makes the new token a predicate. Turtle used to read
   `[ <q> ] <p> <o> .` as `[]` with `<q>` skipped, and TriG did the same
   for objects. A `[` that is not closed yet, or holds a half-typed term,
   starts a property list; it is no longer an empty blank node.
   - Comments are allowed inside `[ ]`, as the W3C grammar's comments-as-white-
   space rule suggests although its `ANON ::= '[' WS* ']'` leaves them out.
   - The formatter prints `[]`, `[  ]` and a `[ ]` across lines as `[ ]`, and
   keeps comments inside as comments, with `]` on a line of its own.
   - W3C suites unchanged: Turtle 296/296, TriG 334/334.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 4 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Parse an empty blank node as one ANON token in Turtle and TriG ([`c151a3c`](https://github.com/ajuvercr/rdf-parsers/commit/c151a3cb075095b5c0c865a0d7071d4c06f4f4eb))
    - Improve incremental parsing ([`f5cb636`](https://github.com/ajuvercr/rdf-parsers/commit/f5cb636514ba8b5bf3dab34d66f54b6b952ac22b))
</details>

## v0.3.0 (2026-11-19)

### Bug Fixes

 - <csr-id-d3fcdfebdfb33bfed83197da102f4f174658dda7/> stack overflow by better list structure

### New Features (BREAKING)

 - <csr-id-3a454f484bced763f63e3104ece0a62fbbac0cd9/> add the RDF 1.2 output model (triple terms, reifiers, annotations)
 - <csr-id-78d2163e0db6867cca9e65856419c81420a19fb4/> resolve documents to RDF, back depth shifts by the text, add a time budget
   Model and resolver:
   - `Turtle.bases` keeps every @base/BASE in document order (`base` stays the
   last one); the Turtle, TriG and N3 converters fill it.
   - New `resolve` module: a converted document to quads with absolute IRIs,
   unescaped strings and fresh blank nodes, directives applied in document
   order. Reports invalid escapes (surrogates), IRIs that escape characters
   an IRI may not contain, and undefined prefixes, with spans.
   - The conformance crate uses it for Turtle and TriG (eval and negative
   tests). Its isomorphism check failed when both graphs used the same blank
   node labels; each side now gets its own namespace. The old normaliser is
   removed. Turtle 280/296 -> 296/296, TriG 318/334 -> 334/334.
   
   A* search:
   - A token at a different bracket depth than in the previous parse used to
   make later role conflicts free after a one-time cost of 2, so an unclosed
   `[` swallowed the next statement. The shift is now cheap only when the
   text's own brackets back it (no unmatched opener before it, or closer
   after it); otherwise it costs as much as a role conflict.
   - `parse_incremental_with` and `ParseOptions { bias, time_budget,
   max_iterations }`; `parse_incremental` is unchanged.
   - A search stopped before any path reached the end finishes the furthest
   path by deleting the rest, instead of returning an empty parse.
   - No more "Max iterations reached" on stderr.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Stack overflow by better list structure ([`d3fcdfe`](https://github.com/ajuvercr/rdf-parsers/commit/d3fcdfebdfb33bfed83197da102f4f174658dda7))
    - Add the RDF 1.2 output model (triple terms, reifiers, annotations) ([`3a454f4`](https://github.com/ajuvercr/rdf-parsers/commit/3a454f484bced763f63e3104ece0a62fbbac0cd9))
    - Resolve documents to RDF, back depth shifts by the text, add a time budget ([`78d2163`](https://github.com/ajuvercr/rdf-parsers/commit/78d2163e0db6867cca9e65856419c81420a19fb4))
</details>

## v0.2.2 (2026-10-14)

### New Features (BREAKING)

 - <csr-id-3a454f484bced763f63e3104ece0a62fbbac0cd9/> add the RDF 1.2 output model (triple terms, reifiers, annotations)
 - <csr-id-78d2163e0db6867cca9e65856419c81420a19fb4/> resolve documents to RDF, back depth shifts by the text, add a time budget
   Model and resolver:
   - `Turtle.bases` keeps every @base/BASE in document order (`base` stays the
   last one); the Turtle, TriG and N3 converters fill it.
   - New `resolve` module: a converted document to quads with absolute IRIs,
   unescaped strings and fresh blank nodes, directives applied in document
   order. Reports invalid escapes (surrogates), IRIs that escape characters
   an IRI may not contain, and undefined prefixes, with spans.
   - The conformance crate uses it for Turtle and TriG (eval and negative
   tests). Its isomorphism check failed when both graphs used the same blank
   node labels; each side now gets its own namespace. The old normaliser is
   removed. Turtle 280/296 -> 296/296, TriG 318/334 -> 334/334.
   
   A* search:
   - A token at a different bracket depth than in the previous parse used to
   make later role conflicts free after a one-time cost of 2, so an unclosed
   `[` swallowed the next statement. The shift is now cheap only when the
   text's own brackets back it (no unmatched opener before it, or closer
   after it); otherwise it costs as much as a role conflict.
   - `parse_incremental_with` and `ParseOptions { bias, time_budget,
   max_iterations }`; `parse_incremental` is unchanged.
   - A search stopped before any path reached the end finishes the furthest
   path by deleting the rest, instead of returning an empty parse.
   - No more "Max iterations reached" on stderr.

### Bug Fixes

 - <csr-id-dc9b09d4ce8be9609f3dca43efa4ce12d3f11324/> format blank nodes and collections like Turtle
   - Collections and blank nodes are groups: when a collection does not fit,
   each item goes on its own line, like Turtle.
   - New `sparql::format` hook with Turtle's content rules: redundant `;` in
   property lists is dropped (the `;` between updates and next to parse
   errors is kept), blank nodes with several predicates put one per line
   and end with `;`, and object lists holding blocks stay together
   (`], [ ], "x"`) with blocks closing at the indentation they open on.
   - `sparql::format::format` now goes through the hook.
 - <csr-id-d3fcdfebdfb33bfed83197da102f4f174658dda7/> stack overflow by better list structure

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Format blank nodes and collections like Turtle ([`dc9b09d`](https://github.com/ajuvercr/rdf-parsers/commit/dc9b09d4ce8be9609f3dca43efa4ce12d3f11324))
</details>

## v0.2.1 (2026-10-14)

### Bug Fixes

 - <csr-id-0f7f51f3edf972abfa978bdbaed3523a891a07c1/> indent wrapped objects past their predicate
   When an object list breaks, its objects now go one level deeper than
   the predicates instead of lining up with them, so they no longer read
   as new predicates.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Indent wrapped objects past their predicate ([`0f7f51f`](https://github.com/ajuvercr/rdf-parsers/commit/0f7f51f3edf972abfa978bdbaed3523a891a07c1))
</details>

## v0.2.0 (2026-10-14)

### New Features

 - <csr-id-eed8071fd9f22600f11c58b119f70e2d4b0ce9af/> format SPARQL updates; add `glue` hint
   - New `glue` format hint attaches a token to the previous one, dropping
   the line breaks before it (never after a comment).
   - SPARQL updates start after a blank line, like queries, and `;` between
   updates is glued to the closing brace: `};`.
   - Drop a dead assignment in the generated comment handling.
 - <csr-id-2e4c12c462869bcefc80b48005ceaf9c28cb3187/> tuned formatters for N3 and JSON-LD
   - N3 uses the shared Turtle rules (now parameterised by blank-node and
   block kinds, looking through expression wrappers) plus formulas laid
   out like TriG graphs, staying on one line when they fit.
   - JSON-LD moves from indent/dedent to `nest`, so empty containers render
   as `{ }` / `[ ]` instead of `{  }`.
   - Format hints can be scoped to the document with `ROOT.<rule>`.
   - Statement separation skips trailing comments of the previous statement,
   so `@prefix ... . # note` still gets a blank line before the triples
   (Turtle, TriG, N3).
 - <csr-id-ad13100d9d7a79d1bed52bc959076069bc93c528/> never glue words together; drop keyword spacing hints
   Generated formatters insert a space wherever a word character (keyword,
   prefixed name, number) would run into another word, IRI, string, variable
   or bracket. This guarantees the output lexes the same as the input and
   replaces the per-keyword `after: space` hints in the turtle and trig
   grammars. Punctuation (`(`, `.`, `@en`) is never separated.
 - <csr-id-cd9f411b0843238491b446b6fccb59f8e8e6f485/> generated formatter with the Turtle rules; comments follow their element
   - trig grammar uses `nest` hints; `trig::format` hooks in the rules shared
   with Turtle (moved to `rdf_format`), plus statement separation for
   top-level blocks and the triples inside a graph. `GRAPH`, bare labels,
   default-graph braces and empty graphs are kept as written.
   - Generated formatters emit a comment on its own line together with the
   next element, so it gets that element's indentation (e.g. the first
   line inside a graph).
   - Update trig tests to the new policy; add graph-form and comment tests.
 - <csr-id-0d834cf8fea506159a5907ae029256c6da39fa39/> generated formatter on par with the swls one
   - turtle grammar uses `nest` instead of indent/dedent pairs; every
   predicate after the first goes on its own line.
   - `turtle::format` is a hand-written FormatHook for content-dependent
   rules: drop redundant `;`, trailing `;` in multi-predicate blank nodes,
   all-or-nothing breaking for simple object lists, hanging first object
   list, and blank lines around multi-line statements and after directives.
   - Generated formatters keep at most one blank line from the source and
   keep trailing comments on their line. The newline after a comment is
   emitted inside the following nest so the next line is indented.
   - Port the swls turtle formatter tests and add an idempotence test.
 - <csr-id-2b97a81b22544d2b7f2a2d4c2ba836d8c12781b6/> `nest`/`ifbreak` hints and a FormatHook escape hatch
   - New format hints: `nest` wraps a child (and its `before` hints) in one
   indentation level; `ifbreak(<text>)` emits text only when the enclosing
   group is broken. Legacy `indent`/`dedent` keep working.
   - Generated formatters gain `format_with`/`to_doc_with` taking a
   `FormatHook`, which can drop elements or replace a node's doc for rules
   that depend on content rather than node kinds.
   - The first line break after a comment is dropped, since the comment
   already ended the line.
   - The generated `to_doc` handles tokens, terminal wrappers and nodes in a
   single path instead of three copies.
 - <csr-id-53beaaaedcc4dd0e831f8da2da313fe54a060d05/> render docs with `pretty`, end comments with a hard line
   - Render `Doc` through the `pretty` crate. A hard line inside a group now
   makes the group break, so multi-line children break their parents, and
   widths are counted correctly. Blank lines keep no indentation.
   - Generated formatters emit a hard line after each comment. Previously
   `<c>; # x\n<d> <e>.` became `# x<d> <e>.`, commenting out code.
   - trig grammar: drop `after: hardline` on `}`, which kept every wrapped
   graph from rendering flat; `block -> before: hardline` covers it.
   - trig test: use a width where the expected output is actually valid.

### Bug Fixes

 - <csr-id-5dfd9a013cb2ef9a1b939d175835d6d5ffcbc6a7/> robustness fixes found by running the W3C test suites
   Running every formatter over the rdf-tests corpus (Turtle, N-Triples,
   TriG, N3, JSON-LD, SPARQL Update) turned up:
   
   - Stack overflow on large documents: the conversion to `pretty` built an
   append chain as deep as the document; build a balanced tree instead.
   - Tokens merging when printed side by side (`"x"@en--ltr .` became
   `"x"@en--ltr.`): the generated lexer now checks every pair of touching
   tokens and a space is kept where they would lex differently.
   - Unlexable error tokens keep their source spacing, like error nodes.
   - Line breaks combine across node boundaries too, so blank lines no
   longer grow on every run; statement hooks ask for the total they want.
   
   After these, valid files format without errors and stably, and no
   invalid file gets more errors. The SPARQL query suite was not fully run.
 - <csr-id-6989b4ac20044a02496e32c36facf21b5ce9336b/> stable output and safe error regions; tidy SPARQL layout
   Generator (all languages):
   - Line breaks from neighbouring hints combine instead of adding up, so
   output no longer grows blank lines on every run (`}\n\nWHERE`).
   - Error regions keep the source's separation; when the parser gives up,
   tokens were glued together (`?s ex:p` -> `?sex:p`).
   - Trailing comments go before queued line breaks and stay on their line;
   own-line comments sit after all leading breaks, directly above their
   element.
   - No double spaces, no spaces at line ends.
   
   SPARQL grammar: nest property lists under their subject, space binary
   operators, drop 19 keyword hints now covered by automatic word spacing.

### Performance

 - <csr-id-b1e34e0b7050b03c7d9e2345ca53f17ba137f954/> bounded error recovery and incremental reuse
   Error recovery no longer explodes on SPARQL: one typo in a 5k-token query
   went from hitting the 1M-step budget (whole document unparsed) to ~100ms,
   and incremental re-parses take a few ms regardless of document size.
   
   Search (a_star.rs):
   - Frontier-dominance pruning: drop a path once a strictly cheaper path is
   12 tokens ahead and parsed them cleanly. Repairs only happen near where
   the cheaper parse got stuck, instead of everywhere before the error.
   - Per-token-position budget of 4096 expansions, popped cheapest-first.
   - Pruned paths are parked, not dropped, and resumed if all live paths
   dead-end; an empty queue returns the best partial parse.
   - Zero-cost moves at the same token skip the heap (eager stack), and the
   state map uses an Fx-style hasher.
   
   Incremental parsing (incremental.rs, parse_incremental):
   - A parse records its steps and parser checkpoints every ~32 tokens as
   plain data in PrevParseInfo (still Send + Sync, no language types).
   - The next parse resumes from a checkpoint before the edit and splices in
   the old steps once it reaches the old parser state after the edit.
   - ParserTrait gains from_parts (generated); PrevParseInfo gains a private
   field, Default and PrevParseInfo::new.
   
   Role bias:
   - Tokens remember a role fingerprint that collapses rule cycles (SPARQL's
   right-recursive TriplesBlock no longer changes every later token's role),
   and agreement also requires the same bracket depth.
   - Inserted (missing) brackets count toward bracket depth.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 99 days passed between releases.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bounded error recovery and incremental reuse ([`b1e34e0`](https://github.com/ajuvercr/rdf-parsers/commit/b1e34e0b7050b03c7d9e2345ca53f17ba137f954))
    - Robustness fixes found by running the W3C test suites ([`5dfd9a0`](https://github.com/ajuvercr/rdf-parsers/commit/5dfd9a013cb2ef9a1b939d175835d6d5ffcbc6a7))
    - Format SPARQL updates; add `glue` hint ([`eed8071`](https://github.com/ajuvercr/rdf-parsers/commit/eed8071fd9f22600f11c58b119f70e2d4b0ce9af))
    - Tuned formatters for N3 and JSON-LD ([`2e4c12c`](https://github.com/ajuvercr/rdf-parsers/commit/2e4c12c462869bcefc80b48005ceaf9c28cb3187))
    - Stable output and safe error regions; tidy SPARQL layout ([`6989b4a`](https://github.com/ajuvercr/rdf-parsers/commit/6989b4ac20044a02496e32c36facf21b5ce9336b))
    - Never glue words together; drop keyword spacing hints ([`ad13100`](https://github.com/ajuvercr/rdf-parsers/commit/ad13100d9d7a79d1bed52bc959076069bc93c528))
    - Generated formatter with the Turtle rules; comments follow their element ([`cd9f411`](https://github.com/ajuvercr/rdf-parsers/commit/cd9f411b0843238491b446b6fccb59f8e8e6f485))
    - Generated formatter on par with the swls one ([`0d834cf`](https://github.com/ajuvercr/rdf-parsers/commit/0d834cf8fea506159a5907ae029256c6da39fa39))
    - `nest`/`ifbreak` hints and a FormatHook escape hatch ([`2b97a81`](https://github.com/ajuvercr/rdf-parsers/commit/2b97a81b22544d2b7f2a2d4c2ba836d8c12781b6))
    - Render docs with `pretty`, end comments with a hard line ([`53beaaa`](https://github.com/ajuvercr/rdf-parsers/commit/53beaaaedcc4dd0e831f8da2da313fe54a060d05))
</details>

## v0.1.16 (2026-07-07)

<csr-id-6e1c60fbb2c1787d7eb53ee385009825ef4abdf6/>

### New Features

 - <csr-id-7b29b4d72e75f1fc8e49cfdb07e20859d149da3c/> span literal lang/datatype and carry computed IRIs for JSON-LD
   - RDFLiteral.lang / .ty now carry source spans (Spanned<_>) across every
   parser (Turtle, TriG, N3, N-Triples, SPARQL, JSON-LD).
   - NamedNode::Prefixed gains a `computed` field so JSON-LD keeps the compact
   source form while carrying the context-resolved absolute IRI (whole-term
   precedence, scoped contexts, @vocab). The conformance normalizer prefers
   it, and Display renders whole-term compact IRIs without a spurious colon.
   - Tune A* recovery: SYNC_THRESHOLD 5->2 (matches its comment),
   DEFAULT_MAX_REPAIR_SPAN 15->10.
   - Add remove_graph_iri.trig incremental-parse benchmark fixture.
   - Bump to 0.1.16.
 - <csr-id-408f577ce2ba8f15251c49a9b66da11875e0b01e/> allow empty comment string
 - <csr-id-160b93d1a1e3fe4846df0f42a0494dedaae87ca5/> change deletion cost from find power, so we can control it better. Deleteing SELECT should be quite cheap but finding it should point to parser to parsring the correct subtree
 - <csr-id-db5093a8eafbab8b4a0d98df38541525cba70ab3/> improve sparql behaviour

### Other

 - <csr-id-6e1c60fbb2c1787d7eb53ee385009825ef4abdf6/> fix wrong json-ld spans

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 8 calendar days.
 - 56 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Span literal lang/datatype and carry computed IRIs for JSON-LD ([`7b29b4d`](https://github.com/ajuvercr/rdf-parsers/commit/7b29b4d72e75f1fc8e49cfdb07e20859d149da3c))
    - Allow empty comment string ([`408f577`](https://github.com/ajuvercr/rdf-parsers/commit/408f577ce2ba8f15251c49a9b66da11875e0b01e))
    - Fix wrong json-ld spans ([`6e1c60f`](https://github.com/ajuvercr/rdf-parsers/commit/6e1c60fbb2c1787d7eb53ee385009825ef4abdf6))
    - Change deletion cost from find power, so we can control it better. Deleteing SELECT should be quite cheap but finding it should point to parser to parsring the correct subtree ([`160b93d`](https://github.com/ajuvercr/rdf-parsers/commit/160b93d1a1e3fe4846df0f42a0494dedaae87ca5))
    - Improve sparql behaviour ([`db5093a`](https://github.com/ajuvercr/rdf-parsers/commit/db5093a8eafbab8b4a0d98df38541525cba70ab3))
</details>

## v0.1.15 (2026-06-29)

<csr-id-7be406b2850a8cfc26ed89e81616ccfae2e7927a/>

### New Features

 - <csr-id-799d44d48f0dadaf4c0c404b6fb47fa4bf180e47/> allow empty comment string
 - <csr-id-e3e9e2c5414258cdd74ac4487f8bb114d5e2c80f/> change deletion cost from find power, so we can control it better. Deleteing SELECT should be quite cheap but finding it should point to parser to parsring the correct subtree
 - <csr-id-e3ab85da22257e424e7c09c4f755076edbba38f6/> improve sparql behaviour

### Other

 - <csr-id-7be406b2850a8cfc26ed89e81616ccfae2e7927a/> fix wrong json-ld spans

## v0.1.12 (2026-05-11)

<csr-id-f0267ddd995ec2706f9ffdd4ff3020a6df2da9c3/>
<csr-id-4516878063e9f923b6c4218f62dcf89cd51d0256/>
<csr-id-4966dd3c251b0a1e15727d8baed20564ccbbdae9/>
<csr-id-2fd00eed48b5b01ba4c6f32ef1a506e69441ed24/>
<csr-id-5f470c00cb6c20ba20ce9e6f280fdb248173605f/>
<csr-id-e9a54c86b8e814abaa84d77fd9a763db1db1b4e1/>
<csr-id-6075177965a174b2ab6b8cc7782d82458c760483/>
<csr-id-277cf8e382b71179c580b39335930c606ed16400/>
<csr-id-4f83ba0bac5b9523cd6e70e07d54c458c98be669/>
<csr-id-a39452b9c883e509d1859e1bda48338402ea1018/>
<csr-id-1c92db0ca8f8bd20c9bbca3575de6790cdad292a/>
<csr-id-7f9dcc4b50e7625262b2c3102c4282ace8f2b63c/>

### Chore

 - <csr-id-f0267ddd995ec2706f9ffdd4ff3020a6df2da9c3/> regenerate parsers
 - <csr-id-4516878063e9f923b6c4218f62dcf89cd51d0256/> cargo fmt + change crate name to rdf-parsers
 - <csr-id-4966dd3c251b0a1e15727d8baed20564ccbbdae9/> remove unused code
 - <csr-id-2fd00eed48b5b01ba4c6f32ef1a506e69441ed24/> remove spurious term_type config

### New Features

 - <csr-id-a09a5011eee13402f72b89779e454e3ac08269fe/> better incorporate dist(a, b) in heuristic, hopefully it is admissable
 - <csr-id-6d36a6f476ff03a9b3441b9bfb2d6adf309ed219/> try to add formatter things, but things are very difficult:
 - <csr-id-902ae28efc548965f51916504810301e45b96bc4/> try to add trig formatter
 - <csr-id-a74a1e2f1525c7105017ab03028af677824395c2/> add formatting directives + improve JSON-LD structures
 - <csr-id-0cc8b5533fdc432fd3d5ccbcf22f40426c91edd8/> add format generation
 - <csr-id-a7574ba3968fb27d6189f050d1d74fc60fe15342/> group error tokens together during lexing
 - <csr-id-cd4a6b440e10e446b63f0c0bb63cb10bef2fab22/> better context resolution for json-ld
 - <csr-id-9b345a4733ddba6bfef75cbd063b160129f32ea6/> take spans more seriously in json-ld + publish
 - <csr-id-9a1881c22b29d22b323c0fee57c3c0b07932387e/> improve context loader trait and add spans to predicates
 - <csr-id-8d49710b6ded0ab5142cd2c931cddf7601d1074a/> cache contexts in demo
 - <csr-id-7afdb5662ab24994222e7a268ba504c2cbacd394/> add remote context example to demo
 - <csr-id-95c011d19d6cb35342cdb76b6c26020e71528b0e/> add JSON-LD
 - <csr-id-bee77e1884916fa74bfa7b4b21546afc653958a0/> remove delete tokens
 - <csr-id-bef42cb76d5db607d60009017ff3a408376aa0d3/> allow disable deletion
 - <csr-id-0091016c68606737ea0bb622c8db61afdffaf30e/> track possible tokens which allows pruning subtrees when the next token can never be present in the sub tree
 - <csr-id-c6dc249927f02664701b2aff9acd2dca4c40473b/> add delete mechanism
 - <csr-id-e92d5e18a3550e01992fd880b1e09bed105a026d/> first try to parse the document fast (without being fault-tolerant) then if there are errors use the prev_info to execute the fault tolerant parser
 - <csr-id-22bc6a5632003671e792cf89b7824402a805307c/> coalesce error to improve errors
 - <csr-id-b8987aabe3105e6399513741155b177a78926f8a/> add effective_error_span function
 - <csr-id-fdf19af921b45745854c03282222eb83f6dd365a/> implement depth offset for error values
 - <csr-id-c6df35740b405927fc55c666e9d840642a54e61a/> update grammars to fix parsing issues related to anon + move sparql tokens to case-insensitive
 - <csr-id-e7c436989e2e4cd9e146086cbfc0cd859c087428/> better weights for turtle
 - <csr-id-73d81c77000cecc37c808c07a50498ae4af29b0a/> port back functionality from swls
 - <csr-id-934bd722f6523f528e795a6cfa958e673c4b4f01/> show derived triples in demo
 - <csr-id-70e0b5577f64ce710b71703f89b8896cba5e4e0e/> add converters to triples for other languages
 - <csr-id-c0f07544270ecbdc860d3cb6d09ea5550b6d6038/> convert trig ast to triples
 - <csr-id-032af6a4b29aad7646c1b86fd317f8518359c912/> simplity parsing types
 - <csr-id-d49d4cdd0c4e473a6718ac00a9feb62c814c34a9/> add n3 to demo + make the parser work with comments
 - <csr-id-5dd4cb7f171fdc7b2fdb0672fd55bd4f4eb4934f/> use fingerprint to push parser into the direction of the previous parse
 - <csr-id-a66b746fd3b255a2c90f68457eaedea7724a9a1a/> cleanup algorithm which succeeds all tests
 - <csr-id-39c24decbfa646193d591bedc361cb1a1e0315fe/> make weights easier and IncrementalBias
 - <csr-id-68f32e47f91f0139f67c713926f923f3646393b6/> move to cost minimization
 - <csr-id-0b20fea7ff686e22d1dc88a8a25aef14f8b967ee/> add demo
 - <csr-id-b2725ae24f6c9208d2aa39dbb7f249fa03cd275e/> coalesce error nodes
 - <csr-id-386d8e10c676c86b51efc4fa36612d558d4d8db1/> deterministic generation
 - <csr-id-9d5a5cf4fe91b930d9228e967e76e74b66b7e1d6/> print parsing differences between chumsky and a star
 - <csr-id-982322f3e092d8b377011e41b5ec8669361901ba/> bias towards previously parsed roles
 - <csr-id-51de8fc58f6b4444aaabf415e5e7a3f2c6dc7a71/> collapse parser expressions before generating the code

### Bug Fixes

 - <csr-id-284acc029f3cbe009a1b77838ea2c53eadff60e8/> turtle formatter
 - <csr-id-a8b9da9988c7d82bc46217c1911aaafe148f7528/> restrict error pre-charging to Fast mode in add_element_checked
   In FaultTolerant mode, pre-charging error cost when tokens are not reachable
   according to min_error_for_token prevents the parser from exploring lower-cost
   alternatives. This causes incorrect error recovery choices. For example, for
   input '<a> <b> .', the parser should parse '<a>' as subject and '<b>' as
   predicate (expecting ObjectList), but with pre-charging in FaultTolerant mode,
   it incorrectly reports an error at Verb level instead.
   
   The optimization should only apply in Fast mode, where it helps prune branches
   early. In FaultTolerant mode, we need to explore all branches without
   pre-charging to allow the A* search to find the lowest-cost parse.
 - <csr-id-b6db53a87cb87f3e1200e41ea2b8b3da8170a3e8/> also use the optimization when running in normal mode
 - <csr-id-5a92e674557aa4fac7013a5c0c55d06fa9ad534e/> bias A* deletion and heuristic toward preserving old token roles
   When the A* offers a deletion branch, tokens with an established role
   from the previous parse (old_kind is Some) now receive a small cost
   penalty (bias.strength).  This breaks the cost tie between deleting a
   new/unknown token vs deleting one that already had a parse-time
   fingerprint, ensuring the incremental structure is preserved.
   
   The admissible heuristic is also adjusted: tokens with old_kind get a
   discounted offset of max(mev − strength, 0) instead of mev.  This
   makes paths that keep old tokens in their previous roles look cheaper,
   so the A* explores them first.
   
   Together, these two changes ensure that inserting <a> before <b> <c> <d>
   correctly removes <a> (the new token) rather than <b> (the old subject).
 - <csr-id-14a0a493f1bee744f334f9e9b9ba2d7a9f9ac6e1/> error spans in demo
 - <csr-id-eb8dbc3c689e468dadcf13f1ac349313f503e53b/> fix trig token weights
 - <csr-id-f56c5f3946ca11e9e5a77237f4ad8051e783bf7c/> only update  tracked depth when the parse is clean
 - <csr-id-123c8eb73e4abd5c843838d4b922c487d3e59906/> fix forgotten incremental parse

### Other

 - <csr-id-5f470c00cb6c20ba20ce9e6f280fdb248173605f/> remove spurious println!
 - <csr-id-e9a54c86b8e814abaa84d77fd9a763db1db1b4e1/> allow carriage return
 - <csr-id-6075177965a174b2ab6b8cc7782d82458c760483/> add verb is verb test
 - <csr-id-277cf8e382b71179c580b39335930c606ed16400/> element heuristic was not admissable, cardinal sin!

### Test

 - <csr-id-4f83ba0bac5b9523cd6e70e07d54c458c98be669/> add failing test
 - <csr-id-a39452b9c883e509d1859e1bda48338402ea1018/> add breaking sparql test to be sure
 - <csr-id-1c92db0ca8f8bd20c9bbca3575de6790cdad292a/> add failing sparql test
 - <csr-id-7f9dcc4b50e7625262b2c3102c4282ace8f2b63c/> create failing test

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 91 commits contributed to the release.
 - 58 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Remove spurious println! ([`5f470c0`](https://github.com/ajuvercr/rdf-parsers/commit/5f470c00cb6c20ba20ce9e6f280fdb248173605f))
    - Allow carriage return ([`e9a54c8`](https://github.com/ajuvercr/rdf-parsers/commit/e9a54c86b8e814abaa84d77fd9a763db1db1b4e1))
    - Better incorporate dist(a, b) in heuristic, hopefully it is admissable ([`a09a501`](https://github.com/ajuvercr/rdf-parsers/commit/a09a5011eee13402f72b89779e454e3ac08269fe))
    - Add verb is verb test ([`6075177`](https://github.com/ajuvercr/rdf-parsers/commit/6075177965a174b2ab6b8cc7782d82458c760483))
    - Try to add formatter things, but things are very difficult: ([`6d36a6f`](https://github.com/ajuvercr/rdf-parsers/commit/6d36a6f476ff03a9b3441b9bfb2d6adf309ed219))
    - Try to add trig formatter ([`902ae28`](https://github.com/ajuvercr/rdf-parsers/commit/902ae28efc548965f51916504810301e45b96bc4))
    - Turtle formatter ([`284acc0`](https://github.com/ajuvercr/rdf-parsers/commit/284acc029f3cbe009a1b77838ea2c53eadff60e8))
    - Add formatting directives + improve JSON-LD structures ([`a74a1e2`](https://github.com/ajuvercr/rdf-parsers/commit/a74a1e2f1525c7105017ab03028af677824395c2))
    - Add format generation ([`0cc8b55`](https://github.com/ajuvercr/rdf-parsers/commit/0cc8b5533fdc432fd3d5ccbcf22f40426c91edd8))
    - Group error tokens together during lexing ([`a7574ba`](https://github.com/ajuvercr/rdf-parsers/commit/a7574ba3968fb27d6189f050d1d74fc60fe15342))
    - Better context resolution for json-ld ([`cd4a6b4`](https://github.com/ajuvercr/rdf-parsers/commit/cd4a6b440e10e446b63f0c0bb63cb10bef2fab22))
    - Take spans more seriously in json-ld + publish ([`9b345a4`](https://github.com/ajuvercr/rdf-parsers/commit/9b345a4733ddba6bfef75cbd063b160129f32ea6))
    - Improve context loader trait and add spans to predicates ([`9a1881c`](https://github.com/ajuvercr/rdf-parsers/commit/9a1881c22b29d22b323c0fee57c3c0b07932387e))
    - Make codegen idempotent ([`d8960e4`](https://github.com/ajuvercr/rdf-parsers/commit/d8960e4e14ca6d96b25443a201e7c1b107d75996))
    - Regenerate parsers ([`f0267dd`](https://github.com/ajuvercr/rdf-parsers/commit/f0267ddd995ec2706f9ffdd4ff3020a6df2da9c3))
    - Gate deletions/sync on shifts_since_pop (paper's t counter) ([`2483a72`](https://github.com/ajuvercr/rdf-parsers/commit/2483a7210cad114e2f0645616f7e6d457b866c58))
    - Report lexer error tokens (InvalidToken) in error output ([`690961b`](https://github.com/ajuvercr/rdf-parsers/commit/690961b1036197eb74ddad53fbfe2cda17b9ce2e))
    - Fix error span for non-zero-width Error nodes (deleted/unparsed tokens) ([`4512fa0`](https://github.com/ajuvercr/rdf-parsers/commit/4512fa01235dc96a4eb3e19b716a0fc929172269))
    - Fix reversed error messages and report unparsed tokens ([`df6015a`](https://github.com/ajuvercr/rdf-parsers/commit/df6015a2506f48017a127f612139295c846445c6))
    - Apply dist(q,a) pruning in Fast mode: reject elements with d > 0 ([`28e3698`](https://github.com/ajuvercr/rdf-parsers/commit/28e3698f80eacf1a0a3385cfc4d0a2e13f030f7a))
    - Element heuristic was not admissable, cardinal sin! ([`277cf8e`](https://github.com/ajuvercr/rdf-parsers/commit/277cf8e382b71179c580b39335930c606ed16400))
    - Fix heuristic composition, lower sync threshold, add t counter ([`68f0287`](https://github.com/ajuvercr/rdf-parsers/commit/68f0287b224540e2983e7451a5aed8304df50d3e))
    - Simplify JSON-LD grammar: collapse jsonString to single terminal ([`92e48cf`](https://github.com/ajuvercr/rdf-parsers/commit/92e48cf48fa4ca0a155f40c4838f9d036e4f71d4))
    - Prune Either alternatives with add_element_checked ([`cc148f7`](https://github.com/ajuvercr/rdf-parsers/commit/cc148f79cb2f64528a8b941a13a55343ad5abce4))
    - Implement per-state dist(q, a) heuristic table from Kim & Yi ([`c9435f6`](https://github.com/ajuvercr/rdf-parsers/commit/c9435f6243ddbef159ee31ca889613b5de0ddaff))
    - Use min_completion_cost as heuristic boost in add_element_checked ([`25e6aa6`](https://github.com/ajuvercr/rdf-parsers/commit/25e6aa68ae703f8cd9ae1edad2254ca9179c4284))
    - Implement state-of-the-art A* error recovery optimizations ([`c4d633b`](https://github.com/ajuvercr/rdf-parsers/commit/c4d633bc0cb766b6aaf9cd08e5b88a1ea698819c))
    - Cache contexts in demo ([`8d49710`](https://github.com/ajuvercr/rdf-parsers/commit/8d49710b6ded0ab5142cd2c931cddf7601d1074a))
    - Add remote context example to demo ([`7afdb56`](https://github.com/ajuvercr/rdf-parsers/commit/7afdb5662ab24994222e7a268ba504c2cbacd394))
    - Add JSON-LD ([`95c011d`](https://github.com/ajuvercr/rdf-parsers/commit/95c011d19d6cb35342cdb76b6c26020e71528b0e))
    - Remove delete tokens ([`bee77e1`](https://github.com/ajuvercr/rdf-parsers/commit/bee77e1884916fa74bfa7b4b21546afc653958a0))
    - Allow disable deletion ([`bef42cb`](https://github.com/ajuvercr/rdf-parsers/commit/bef42cb76d5db607d60009017ff3a408376aa0d3))
    - Restrict error pre-charging to Fast mode in add_element_checked ([`a8b9da9`](https://github.com/ajuvercr/rdf-parsers/commit/a8b9da9988c7d82bc46217c1911aaafe148f7528))
    - Also use the optimization when running in normal mode ([`b6db53a`](https://github.com/ajuvercr/rdf-parsers/commit/b6db53a87cb87f3e1200e41ea2b8b3da8170a3e8))
    - Track possible tokens which allows pruning subtrees when the next token can never be present in the sub tree ([`0091016`](https://github.com/ajuvercr/rdf-parsers/commit/0091016c68606737ea0bb622c8db61afdffaf30e))
    - Cargo fmt + change crate name to rdf-parsers ([`4516878`](https://github.com/ajuvercr/rdf-parsers/commit/4516878063e9f923b6c4218f62dcf89cd51d0256))
    - Add delete mechanism ([`c6dc249`](https://github.com/ajuvercr/rdf-parsers/commit/c6dc249927f02664701b2aff9acd2dca4c40473b))
    - Bias A* deletion and heuristic toward preserving old token roles ([`5a92e67`](https://github.com/ajuvercr/rdf-parsers/commit/5a92e674557aa4fac7013a5c0c55d06fa9ad534e))
    - First try to parse the document fast (without being fault-tolerant) then if there are errors use the prev_info to execute the fault tolerant parser ([`e92d5e1`](https://github.com/ajuvercr/rdf-parsers/commit/e92d5e18a3550e01992fd880b1e09bed105a026d))
    - Add failing test ([`4f83ba0`](https://github.com/ajuvercr/rdf-parsers/commit/4f83ba0bac5b9523cd6e70e07d54c458c98be669))
    - Error spans in demo ([`14a0a49`](https://github.com/ajuvercr/rdf-parsers/commit/14a0a493f1bee744f334f9e9b9ba2d7a9f9ac6e1))
    - Coalesce error to improve errors ([`22bc6a5`](https://github.com/ajuvercr/rdf-parsers/commit/22bc6a5632003671e792cf89b7824402a805307c))
    - Add effective_error_span function ([`b8987aa`](https://github.com/ajuvercr/rdf-parsers/commit/b8987aabe3105e6399513741155b177a78926f8a))
    - Fix trig token weights ([`eb8dbc3`](https://github.com/ajuvercr/rdf-parsers/commit/eb8dbc3c689e468dadcf13f1ac349313f503e53b))
    - Only update  tracked depth when the parse is clean ([`f56c5f3`](https://github.com/ajuvercr/rdf-parsers/commit/f56c5f3946ca11e9e5a77237f4ad8051e783bf7c))
    - Add breaking sparql test to be sure ([`a39452b`](https://github.com/ajuvercr/rdf-parsers/commit/a39452b9c883e509d1859e1bda48338402ea1018))
    - Implement depth offset for error values ([`fdf19af`](https://github.com/ajuvercr/rdf-parsers/commit/fdf19af921b45745854c03282222eb83f6dd365a))
    - Add failing sparql test ([`1c92db0`](https://github.com/ajuvercr/rdf-parsers/commit/1c92db0ca8f8bd20c9bbca3575de6790cdad292a))
    - Update grammars to fix parsing issues related to anon + move sparql tokens to case-insensitive ([`c6df357`](https://github.com/ajuvercr/rdf-parsers/commit/c6df35740b405927fc55c666e9d840642a54e61a))
    - Better weights for turtle ([`e7c4369`](https://github.com/ajuvercr/rdf-parsers/commit/e7c436989e2e4cd9e146086cbfc0cd859c087428))
    - Port back functionality from swls ([`73d81c7`](https://github.com/ajuvercr/rdf-parsers/commit/73d81c77000cecc37c808c07a50498ae4af29b0a))
    - Show derived triples in demo ([`934bd72`](https://github.com/ajuvercr/rdf-parsers/commit/934bd722f6523f528e795a6cfa958e673c4b4f01))
    - Add converters to triples for other languages ([`70e0b55`](https://github.com/ajuvercr/rdf-parsers/commit/70e0b5577f64ce710b71703f89b8896cba5e4e0e))
    - Convert trig ast to triples ([`c0f0754`](https://github.com/ajuvercr/rdf-parsers/commit/c0f07544270ecbdc860d3cb6d09ea5550b6d6038))
    - Simplity parsing types ([`032af6a`](https://github.com/ajuvercr/rdf-parsers/commit/032af6a4b29aad7646c1b86fd317f8518359c912))
    - Add n3 to demo + make the parser work with comments ([`d49d4cd`](https://github.com/ajuvercr/rdf-parsers/commit/d49d4cdd0c4e473a6718ac00a9feb62c814c34a9))
    - Remove unused code ([`4966dd3`](https://github.com/ajuvercr/rdf-parsers/commit/4966dd3c251b0a1e15727d8baed20564ccbbdae9))
    - Remove spurious term_type config ([`2fd00ee`](https://github.com/ajuvercr/rdf-parsers/commit/2fd00eed48b5b01ba4c6f32ef1a506e69441ed24))
    - Use fingerprint to push parser into the direction of the previous parse ([`5dd4cb7`](https://github.com/ajuvercr/rdf-parsers/commit/5dd4cb7f171fdc7b2fdb0672fd55bd4f4eb4934f))
    - Create failing test ([`7f9dcc4`](https://github.com/ajuvercr/rdf-parsers/commit/7f9dcc4b50e7625262b2c3102c4282ace8f2b63c))
    - Cleanup algorithm which succeeds all tests ([`a66b746`](https://github.com/ajuvercr/rdf-parsers/commit/a66b746fd3b255a2c90f68457eaedea7724a9a1a))
    - Make weights easier and IncrementalBias ([`39c24de`](https://github.com/ajuvercr/rdf-parsers/commit/39c24decbfa646193d591bedc361cb1a1e0315fe))
    - Bump ([`3f46a9a`](https://github.com/ajuvercr/rdf-parsers/commit/3f46a9a0c3052578d49924bfe55f9b41eab2a3d5))
    - Move to cost minimization ([`68f32e4`](https://github.com/ajuvercr/rdf-parsers/commit/68f32e47f91f0139f67c713926f923f3646393b6))
    - Fix forgotten incremental parse ([`123c8eb`](https://github.com/ajuvercr/rdf-parsers/commit/123c8eb73e4abd5c843838d4b922c487d3e59906))
    - Add demo ([`0b20fea`](https://github.com/ajuvercr/rdf-parsers/commit/0b20fea7ff686e22d1dc88a8a25aef14f8b967ee))
    - Add trig and ntriples ([`c5ea380`](https://github.com/ajuvercr/rdf-parsers/commit/c5ea3804429bcb59a404a20b9c6143a36c4073f1))
    - Coalesce error nodes ([`b2725ae`](https://github.com/ajuvercr/rdf-parsers/commit/b2725ae24f6c9208d2aa39dbb7f249fa03cd275e))
    - Fix some inifinite loop somewhere ([`8792bed`](https://github.com/ajuvercr/rdf-parsers/commit/8792bede2efc362a212d41d2e2047fa7dc41058e))
    - Deterministic generation ([`386d8e1`](https://github.com/ajuvercr/rdf-parsers/commit/386d8e10c676c86b51efc4fa36612d558d4d8db1))
    - Fix error spans ([`e62b05c`](https://github.com/ajuvercr/rdf-parsers/commit/e62b05cd845669c6a39e000c8d9f0078700bf49a))
    - Print parsing differences between chumsky and a star ([`9d5a5cf`](https://github.com/ajuvercr/rdf-parsers/commit/9d5a5cf4fe91b930d9228e967e76e74b66b7e1d6))
    - Add benchmark + try to optimize the code ([`267b4c1`](https://github.com/ajuvercr/rdf-parsers/commit/267b4c17abfa5962493cc587c8f092dac65e42f1))
    - Bias towards previously parsed roles ([`982322f`](https://github.com/ajuvercr/rdf-parsers/commit/982322f3e092d8b377011e41b5ec8669361901ba))
    - Collapse parser expressions before generating the code ([`51de8fc`](https://github.com/ajuvercr/rdf-parsers/commit/51de8fc58f6b4444aaabf415e5e7a3f2c6dc7a71))
    - Add tests, move xtask to binary and inline known inlines in xtask generation ([`de39d3e`](https://github.com/ajuvercr/rdf-parsers/commit/de39d3e5681b597ec6b9a871ac0d66510bb88825))
    - Add A* algorithm ([`8a9df16`](https://github.com/ajuvercr/rdf-parsers/commit/8a9df1643ca51e69b9a13c563488df15039388c9))
    - Remove ParseRes ([`4cd5e57`](https://github.com/ajuvercr/rdf-parsers/commit/4cd5e57ece994bee595b6f99a6a3a56246a1b3b8))
    - Add slow sparql ([`5b867f6`](https://github.com/ajuvercr/rdf-parsers/commit/5b867f6616269d7d9e142262ef11d1373600c4a1))
    - Valid logos :o ([`ef53b98`](https://github.com/ajuvercr/rdf-parsers/commit/ef53b9813b4b28c7a0ce70e011b76bc57b3582ff))
    - Valid logos :o ([`499747a`](https://github.com/ajuvercr/rdf-parsers/commit/499747a6f353b9abcf4b4927777bd685c89dd5d2))
    - Parse expressions better, let's try ([`d892a0e`](https://github.com/ajuvercr/rdf-parsers/commit/d892a0e68b02db609cd2ac8810f4ff40c8926a4f))
    - Things are not working ([`917d9c4`](https://github.com/ajuvercr/rdf-parsers/commit/917d9c4b7642071691fafa040f708c04d8802ef4))
    - Things ([`156a90a`](https://github.com/ajuvercr/rdf-parsers/commit/156a90a73b0fc7394b70ced6e4727db00494b700))
    - Handle whitespace better ([`a966867`](https://github.com/ajuvercr/rdf-parsers/commit/a9668672e7637c046f21db67935b4eeb32817257))
    - Make jolly report ([`52955b3`](https://github.com/ajuvercr/rdf-parsers/commit/52955b34cc7bcb0a6fe55c8186aff10f61299270))
    - This is getting interesting ([`167f574`](https://github.com/ajuvercr/rdf-parsers/commit/167f5749ff21359f1640ededc17fb3afb4c9f915))
    - Something that smells like parsing ([`8f10dcd`](https://github.com/ajuvercr/rdf-parsers/commit/8f10dcda88de4d169415ac60f3848c77c4489c5b))
    - Things ([`a4be7ef`](https://github.com/ajuvercr/rdf-parsers/commit/a4be7ef27359190fe86b1379d779730297617872))
    - This is difficult, let's add gates ([`88c6716`](https://github.com/ajuvercr/rdf-parsers/commit/88c6716d9563004aa8d25d9e1db96cbacbd55a47))
    - Add many macros ([`51995ba`](https://github.com/ajuvercr/rdf-parsers/commit/51995ba6717c432a59235af1f658d59e99227fb8))
</details>

