# A\* Error-Recovery Parser — Algorithm Description

## Overview

The parser is a **recursive-descent parser driven by A\* best-first search**.
Grammars are compiled to Rust state machines (`xtask codegen`); at runtime,
the A\* explores alternative parse paths in priority order, choosing the one
with the lowest total cost.  This makes it an **optimal error-repair** parser:
for any input (valid or broken), it finds the parse that requires the fewest /
cheapest error operations.

The design is adapted from Kim & Yi (2010) *"An algorithm for LR error repair
using A\* search"*, re-targeted from LR to recursive descent.

---

## Two-pass architecture

1. **Fast mode** — tries a zero-error parse.  No heuristic, no fallback
   tracking, no incremental bias.  Elements that acquire `has_error` are
   immediately dropped.  `state_dist > 0` also rejects outright (no
   insertions needed = terminal must be directly reachable).  Returns `None`
   on any error.

2. **FaultTolerant mode** — full A\* search with error recovery.  Activated
   only when the fast pass fails.  Explores insertions, deletions,
   sync-ahead, and maintains a best-at-EOF fallback.

`parse_incremental()` runs fast mode first; on failure, falls back to
fault-tolerant mode with incremental bias from the previous parse.

---

## Cost model

Every token match costs `max_error_value()`.  Every error insertion also
costs `max_error_value()`.  Differentiation comes from:

| Signal | Effect |
|---|---|
| Token **matches** normally | cost += `max_error_value()` |
| Token **inserted** (error) | cost += `max_error_value()`, `has_error = true` |
| Token **deleted** (skipped) | cost += `deletion_cost()` (= 5 × `max_error_value()`) |
| Incremental **agreement** | cost unchanged (bias = 0) |
| Incremental **role conflict** | cost += **50** (same depth) or `delta_change × max_error_value()` (depth shift) |
| Delete a token with **previous role** | cost += `bias.strength` extra (history penalty) |

The A\* priority is `f = cost + h` (lower = better).

---

## Magic numbers & tuning constants

### A\* search bounds

| Constant | Value | Purpose |
|---|---|---|
| `DEFAULT_MAX_ITERATIONS` | **1,000,000** | Maximum heap pops before timeout.  Returns best partial parse at EOF. |
| `DEFAULT_MAX_REPAIR_SPAN` | **15** | Maximum consecutive error operations (insertions + deletions) per element.  Elements exceeding this are dropped. |

### Error weights (per-grammar, in `=== error_value ===` sections)

| Token class | `max_error_value` | Rationale |
|---|---|---|
| Most terminals (IRI, STRING, …) | **1** (default) | Low-cost; misplacing one is a minor error |
| Brackets, commas, colons | **2** | Structural tokens — inserting/deleting has wider impact |
| Dot (`.`) | **3** (Turtle/TriG) | Statement terminator — wrong placement disrupts triples |
| Dot (`.`) in N-Triples | **8** | N-Triples has strict line structure; dot errors are severe |
| `@prefix`, `@base` | **100** | Directive keywords — the parser should almost never invent these |

### Deletion

| Constant | Value | Purpose |
|---|---|---|
| `deletion_cost()` | **5 × `max_error_value()`** | Deleting a token must be significantly more expensive than inserting one, so the parser prefers matching over skipping.  At 3× some tests failed (deleting 3 IRIs was cheaper than inserting a missing dot). |
| History penalty | **`bias.strength`** (default **1**) | Extra cost when deleting a token that had a role in the previous parse — discourages discarding tokens the user likely intended to keep. |

### Sync-ahead

| Constant | Value | Purpose |
|---|---|---|
| `SYNC_THRESHOLD` | **2** | Consecutive errors before sync-ahead scanning activates.  Paper's (R3S-n) is always available; 2 is a compromise to avoid scanning overhead on isolated single-token errors. |
| `MAX_SYNC_SCAN` | **50** | Maximum tokens to scan ahead looking for a sync point (an ancestor rule that can accept the scanned token). |

### Incremental bias

| Constant | Value | Purpose |
|---|---|---|
| `IncrementalBias.strength` | **1** (default) | Agreement discount / conflict penalty per token.  Accumulated over the suffix sum as the heuristic. |
| Role-conflict bias | **50** | When a token's fingerprint disagrees at the same bracket depth (genuine role conflict), the match branch pays +50 cost and a fallback insertion branch is also created.  This steers the parser toward reparsing tokens in their previous roles. |
| Depth-shift adoption cost | **`delta_change × max_error_value()`** | One-time penalty when a token's bracket depth has shifted since the previous parse.  Proportional to how far the depth moved. |

### Heuristic components (additive)

The element heuristic `h` is the sum of two independent, admissible components:

1. **Suffix-sum heuristic** (`heuristic[pos]`): sum of `max_error_value()` for
   all remaining tokens.  In incremental mode, each token with an `old_kind`
   contributes `-strength` (optimistic agreement discount).

2. **dist(q, a)**: precomputed minimum insertion cost from the current parser
   state to a state where the next input terminal can be matched.  Computed
   per `(rule_kind, state_number, terminal)` triple via fixed-point iteration
   over the grammar.

These are additive (not max'd) because they measure non-overlapping costs:
the suffix sum counts token-matching costs; dist counts insertion costs to
reach the next matchable state.

### dist(q, a) computation (codegen)

| Constant | Value | Purpose |
|---|---|---|
| Inner fixed-point cap | **100** iterations | Per-rule dist convergence.  Rarely needs more than ~10. |
| Outer (cross-rule) fixed-point cap | **100** iterations | Cross-rule dist propagation.  Large grammars (SPARQL, ~900 states) may need many iterations. |

### t counter (shifts_since_pop)

Tracks consecutive successful token matches (Bumps) since the last rule
completion (Pop).  Used to gate operations per Kim & Yi §3.3:

- **`shifts_since_pop > 1`**: skip `try_delete_token` and `try_sync_ahead`.
  The paper proves that after multiple shifts, the only useful recovery
  operation is insertion — deletions produce redundant configurations.
- **`shifts_since_pop > 0`** in `add_element`: dist(q, a) is known to be
  exact (not diluted by the conservative pop = 0 fallback), so the h-boost
  is tight.

### Fast-mode pruning

In Fast mode (zero-error parse), two guards eliminate dead branches early:

1. **`has_error` → reject**: any element that acquired an error step.
2. **`state_dist > 0` → reject**: the current state requires insertions to
   reach the next token — this element will inevitably produce errors.

Together these prevent the high branching factor of Either alternatives
(e.g. JSON-LD's `jsonValue` with 7 branches) from exploding the heap.

### Either branch pruning (codegen)

For Either alternatives whose children are single terminals, the codegen
emits `add_element_checked` instead of `add_element`.  This calls
`min_error_for_token(kind, current_token)` — if the token can't start the
alternative, the element is penalised:

- **cost** += `min_error_for_token` (positive when unreachable)
- **h** += `min_completion_cost` (cheapest insertion cost to complete the rule)
- **`has_error`** = true (gets rejected in Fast mode)

### Deduplication

The `done` map tracks the best cost seen for each `(fingerprint, token_pos,
parent_rule_at)` triple.  Elements with equal-or-worse cost are rejected on
insertion into the heap.  Stale heap entries (from a later better-cost
element for the same state) are skipped on pop.

---

## Operation flow (per heap pop)

```
pop element e from heap
  │
  ├─ if stale (done[state] < e.cost): skip
  ├─ if at EOF and parent depth == 1: return parse ✓
  ├─ if max_iterations exceeded: break to fallback
  │
  ├─ head.step(e, state)          ← grammar state machine:
  │     ├─ Pop (state 0)          → pop parent, reset shifts_since_pop = 0
  │     ├─ Expect(token)          → try match or insert error
  │     ├─ Push(rule)             → push sub-rule, descend fingerprint
  │     └─ Either(alts)           → enqueue all alternatives
  │
  └─ if FaultTolerant AND shifts_since_pop ≤ 1:
        ├─ try_delete_token(e)    → skip current token at deletion_cost
        └─ try_sync_ahead(e)      → if consecutive_errors ≥ 2, scan ahead
                                     for ancestor-acceptable token
```

---

## Summary of all tuning knobs

| # | Name | Value | File | Line | Why this value |
|---|---|---|---|---|---|
| 1 | `DEFAULT_MAX_ITERATIONS` | 1,000,000 | a_star.rs | 55 | Large enough for any practical document; prevents infinite loops |
| 2 | `DEFAULT_MAX_REPAIR_SPAN` | 15 | a_star.rs | 60 | Limits cascading errors; long enough for realistic multi-token gaps |
| 3 | `SYNC_THRESHOLD` | 2 | a_star.rs | 66 | Activates sync early (paper uses 0); avoids overhead on single errors |
| 4 | `MAX_SYNC_SCAN` | 50 | a_star.rs | 69 | Covers typical error spans; scanning further is unlikely to help |
| 5 | `deletion_cost` multiplier | 5× | lib.rs | 161 | 3× failed tests (NTriples); 5× reliably prefers matching over deletion |
| 6 | Role-conflict bias | 50 | a_star.rs | 431 | High enough to prefer correct roles; low enough to not block all reuse |
| 7 | `IncrementalBias.strength` | 1 | lib.rs | 271 | Gentle per-token nudge; accumulates over many tokens |
| 8 | dist fixed-point cap (inner) | 100 | xtask/lib.rs | 440 | Per-rule convergence; usually <10 iterations needed |
| 9 | dist fixed-point cap (outer) | 100 | xtask/lib.rs | 1159 | Cross-rule convergence for large grammars like SPARQL |
| 10 | `error_value` defaults | 1 / 2 / 3 / 8 / 100 | grammars/*.txt | varies | Per-grammar weights; see table above |
