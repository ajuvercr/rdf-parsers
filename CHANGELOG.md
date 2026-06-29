# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.15 (2026-06-29)

### New Features

 - <csr-id-799d44d48f0dadaf4c0c404b6fb47fa4bf180e47/> allow empty comment string
 - <csr-id-e3e9e2c5414258cdd74ac4487f8bb114d5e2c80f/> change deletion cost from find power, so we can control it better. Deleteing SELECT should be quite cheap but finding it should point to parser to parsring the correct subtree
 - <csr-id-e3ab85da22257e424e7c09c4f755076edbba38f6/> improve sparql behaviour

### Other

 - <csr-id-7be406b2850a8cfc26ed89e81616ccfae2e7927a/> fix wrong json-ld spans

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 39 calendar days.
 - 48 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Allow empty comment string ([`799d44d`](https://github.com/ajuvercr/rdf-parsers/commit/799d44d48f0dadaf4c0c404b6fb47fa4bf180e47))
    - Fix wrong json-ld spans ([`7be406b`](https://github.com/ajuvercr/rdf-parsers/commit/7be406b2850a8cfc26ed89e81616ccfae2e7927a))
    - Change deletion cost from find power, so we can control it better. Deleteing SELECT should be quite cheap but finding it should point to parser to parsring the correct subtree ([`e3e9e2c`](https://github.com/ajuvercr/rdf-parsers/commit/e3e9e2c5414258cdd74ac4487f8bb114d5e2c80f))
    - Improve sparql behaviour ([`e3ab85d`](https://github.com/ajuvercr/rdf-parsers/commit/e3ab85da22257e424e7c09c4f755076edbba38f6))
</details>

## v0.1.12 (2026-05-11)

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

