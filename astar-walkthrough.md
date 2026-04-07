# A\* Error-Recovery Walkthrough: Incremental Token Insertion

This document walks through a concrete example that exercises almost every
mechanism in the incremental A\* parser: the heuristic, fingerprint-based
role agreement, role conflict with bias, token deletion with role-aware
penalty, error insertion fallbacks, and the `done`-map deduplication that
ultimately decides which path survives.

---

## 1. Setup

### Previous document (valid)

```turtle
<b> <c> <d> .
```

Parsed successfully as one triple:

| Role      | Token | Fingerprint (symbolic) | max\_error\_value |
|-----------|-------|------------------------|-------------------|
| Subject   | `<b>` | `fp_subj_iri`          | 1                 |
| Predicate | `<c>` | `fp_pred_iri`          | 1                 |
| Object    | `<d>` | `fp_obj_iri`           | 1                 |
| Stop      | `.`   | `fp_stop`              | 3                 |

These fingerprints and roles are stored in a `PrevParseInfo`.

### New document (invalid — one extra token)

```turtle
<a> <b> <c> <d> .
```

A Myers diff between the old and new token streams yields:

| Position | Token | Kind   | old\_kind (from diff) | mev |
|----------|-------|--------|-----------------------|-----|
| 0        | `<a>` | Iriref | **None** (new token)  | 1   |
| 2        | `<b>` | Iriref | `fp_subj_iri`         | 1   |
| 4        | `<c>` | Iriref | `fp_pred_iri`         | 1   |
| 6        | `<d>` | Iriref | `fp_obj_iri`          | 1   |
| 8        | `.`   | Stop   | `fp_stop`             | 3   |

(Odd positions are whitespace tokens that are skipped by `get_actual_index`.)

### Parameters

- `IncrementalBias { strength: 1 }` (the default)
- Deletion base cost: `5 × mev`
- Role-conflict bias: `+50`

---

## 2. Heuristic Computation

The heuristic `h[i]` is a suffix sum over `max_error_value`, but tokens with
`old_kind` get a **discount** of `strength` (clamped to 0):

```
offset(token) = if old_kind.is_some() { max(mev − strength, 0) } else { mev }
```

| Position | Token | old\_kind? | mev | offset         | h\[i\]                |
|----------|-------|-----------|-----|----------------|-----------------------|
| 8        | `.`   | yes       | 3   | max(3−1, 0) = 2 | **2**               |
| 6        | `<d>` | yes       | 1   | max(1−1, 0) = 0 | 2 + 0 = **2**       |
| 4        | `<c>` | yes       | 1   | 0              | 2 + 0 = **2**        |
| 2        | `<b>` | yes       | 1   | 0              | 2 + 0 = **2**        |
| 0        | `<a>` | **no**    | 1   | 1              | 2 + 1 = **3**        |

So `h[0] = 3`.  The initial element starts with `cost = 0`, `h = 3`, giving
**`f = 3`**.

> **Key insight:** because all the "old" tokens have discounted offsets, the
> heuristic is *optimistic* about paths that keep them — exactly the A\*
> admissibility property, but tuned to favour incremental agreement.

---

## 3. The Two Competing Deletion Paths

The document has five non-whitespace tokens but a triple only needs three IRIs
plus a dot.  One IRI must be deleted.  Two candidate paths are:

| Path            | Deletes | Resulting triple        |
|-----------------|---------|-------------------------|
| **A** (desired) | `<a>`   | `<b> <c> <d> .`         |
| **B** (wrong)   | `<b>`   | `<a> <c_pred> <d_obj> .`|

---

## 4. Step-by-Step: Path B (match `<a>`, delete `<b>`)

The A\* explores this path first because matching `<a>` is cheap.

### 4.1  Match `<a>` as Subject → Iri → Iriref

The grammar expands: TurtleDoc → Statement → Triples → Subject → Iri → expect
`Iriref`.

Token `<a>` at position 0 **is** an `Iriref` — it matches.

```
old_kind = None  →  IsExpectedElement::Unknown  →  bias = 0
```

| field | value |
|-------|-------|
| cost  | 0 + mev + bias = **1** |
| h     | h\[2\] = **2** |
| **f** | **3** |
| pos   | 2 (`<b>`) |

> No conflict, no fallback created.  The A\* continues expanding the grammar.

### 4.2  Expect predicate: Verb → Predicate → Iri → expect `Iriref` at `<b>`

Token `<b>` matches `Iriref`.  Fingerprint check:

```
old_kind = fp_subj_iri   (was Subject in previous parse)
state.0  = fp_pred_iri   (current position is Predicate → Iri)
fp_subj_iri ≠ fp_pred_iri  →  IsExpectedElement::False
depth_delta = 0, committed_delta = 0  →  genuine role conflict
```

This creates **two** elements:

| Element                     | cost            | h   | **f** | pos | note                               |
|-----------------------------|-----------------|-----|-------|-----|--------------------------------------|
| **Match** (with conflict)   | 1 + 1 + 50 = **52** | 2 | **54** | 4 | prohibitively expensive            |
| **Fallback** (insert error) | 1 + 1 = **2**  | 2   | **4** | 2  | stays at `<b>`, inserts error pred |

The match element (f=54) sinks to the bottom of the heap.  Only the fallback
(f=4) continues.

### 4.3  Fallback cascade: insert errors for verb, object, and dot

The fallback element (cost=2, pos=2) pops through the grammar rules without
consuming any tokens.  At each missing production the parser inserts an error:

| Inserted error for | mev of expected token | cost after | h   | **f** | pos |
|--------------------|-----------------------|------------|-----|-------|-----|
| Verb (Iriref)      | 1                     | 2          | 2   | 4     | 2   |
| Object (Iriref)    | 1                     | 3          | 2   | 5     | 2   |

After popping ObjectList → PredicateObjectList → Triples → Statement, the
grammar expects a **Stop** (`.`).  Token `<b>` is not a dot:

| Inserted error for | mev | cost after | h   | **f** | pos |
|--------------------|-----|------------|-----|-------|-----|
| Stop               | 3   | **6**      | 2   | **8** | 2   |

### 4.4  Second triple: `<b> <c> <d> .`

After Statement pops, TurtleDoc loops and begins a new Statement → Triples →
Subject → Iri → Iriref.

Now `<b>` at position 2 is matched as Subject:

```
old_kind = fp_subj_iri  ==  state.0 (= fp_subj_iri)
→  IsExpectedElement::True  →  bias = 0  ✓ agreement!
```

| step                | cost | h | **f** |
|---------------------|------|---|-------|
| Match `<b>` subject | 7    | 2 | 9     |
| Match `<c>` pred    | 8    | 2 | 10    |
| Match `<d>` object  | 9    | 2 | 11    |
| Match `.` stop      | 12   | 0 | 12    |

**Path B total cost = 12, f = 12.**

---

### 4.5  Meanwhile: the deletion of `<b>` inside Iri

When the grammar first reaches Iri state=1 for Predicate at position 2 (step
4.2), the main loop *also* offers a **deletion branch** for `<b>`:

```
delete_cost = 5 × mev + role_penalty
            = 5 × 1   + 1              ← old_kind is Some → penalty = strength
            = 6
```

| field | value |
|-------|-------|
| cost  | 1 (from matching `<a>`) + 6 = **7** |
| h     | h\[4\] = **2** |
| **f** | **9** |
| pos   | 4 (`<c>`) |

After this deletion the grammar is still inside Iri (for Predicate).  Token
`<c>` is next:

```
old_kind = fp_pred_iri  ==  state.0 (= fp_pred_iri, same context)
→  agreement, bias = 0  ✓
```

| step                   | cost | h | **f** |
|------------------------|------|---|-------|
| Delete `<b>` (penalty) | 7    | 2 | 9     |
| Match `<c>` predicate  | 8    | 2 | 10    |
| Match `<d>` object     | 9    | 2 | 11    |
| Match `.` stop         | 12   | 0 | 12    |

**Path B-delete total cost = 12, f = 12.**

---

## 5. Step-by-Step: Path A (delete `<a>`, match `<b> <c> <d> .`)

### 5.1  Deletion of `<a>`

Every time an element at position 0 is popped from the heap, the main loop
offers a deletion branch for `<a>`.  The earliest such element is at the
Subject → Iri level:

```
delete_cost = 5 × mev + role_penalty
            = 5 × 1   + 0              ← old_kind is None → no penalty
            = 5
```

| field | value |
|-------|-------|
| cost  | 0 + 5 = **5** |
| h     | h\[2\] = **2** |
| **f** | **7** |
| pos   | 2 (`<b>`) |

### 5.2  Match `<b>` as Subject → Iri → Iriref  (agreement ✓)

The grammar context is still Subject → Iri (unchanged by the deletion, which
only advanced the token pointer).

```
old_kind = fp_subj_iri  ==  state.0 (= fp_subj_iri)
→  agreement, bias = 0
```

| field | value |
|-------|-------|
| cost  | 5 + 1 + 0 = **6** |
| h     | h\[4\] = **2** |
| **f** | **8** |

### 5.3  Match `<c>` as Predicate → Iri → Iriref  (agreement ✓)

```
old_kind = fp_pred_iri  ==  state.0 (= fp_pred_iri)
→  agreement, bias = 0
```

| field | value |
|-------|-------|
| cost  | 6 + 1 = **7** |
| h     | h\[6\] = **2** |
| **f** | **9** |

### 5.4  Match `<d>` as Object → Iri → Iriref  (agreement ✓)

```
old_kind = fp_obj_iri  ==  state.0 (= fp_obj_iri)
→  agreement, bias = 0
```

| field | value |
|-------|-------|
| cost  | 7 + 1 = **8** |
| h     | h\[8\] = **2** |
| **f** | **10** |

### 5.5  Match `.` as Stop  (agreement ✓)

```
old_kind = fp_stop  ==  state.0 (= fp_stop)
→  agreement, bias = 0
```

| field | value |
|-------|-------|
| cost  | 8 + 3 = **11** |
| h     | 0 |
| **f** | **11** |

**Path A total cost = 11, f = 11.** ✓

---

## 6. How the A\* Picks the Winner

### 6.1  Exploration order (f-values over time)

```
f:  3 ···  4 ···  5 ···  7 ···  8 ···  9 ··· 10 ··· 11 ··· 12
    ↑            ↑       ↑      ↑      ↑      ↑      ↑      ↑
    match <a>    fb      fb    del <a> A:<b>  A:<c>  A:<d>  A:. → SOLUTION (cost 11)
    (Path B)  (inserts)       (Path A)                       ↑
                                                     Path B reaches f=12 later
```

Path B's intermediate elements (f = 3, 4, 5, …) are explored first because
matching `<a>` is cheap.  But every step down that path either hits a **role
conflict** (+50) or an **error insertion** (+mev).  Path B cannot reach a
complete parse below f = 12.

Path A's first element appears at **f = 7** (the deletion of `<a>`).  From
there, every token agrees with its previous role, so f advances by small
increments (8, 9, 10, 11) straight to a complete parse.

### 6.2  The `done`-map convergence

Both paths eventually reach the same grammar state: Predicate → Iri, at
position 4 (`<c>`).  Their costs at that point:

| Path | cost at `<c>` | how                                   |
|------|---------------|---------------------------------------|
| A    | **6**         | delete `<a>` (5) + match `<b>` (1)    |
| B    | **7**         | match `<a>` (1) + delete `<b>` (5+1)  |

The `done` map keeps **the element with the lowest cost** for each state key
`(fingerprint, position, grammar_state)`.  Path A arrives with cost 6; Path B
arrives with cost 7.  **Path B is rejected.**

From that point on, only Path A continues.  It matches `<c>`, `<d>`, `.` with
full agreement and produces the solution at cost 11.

---

## 7. Summary: Why Each Mechanism Matters

| Mechanism | What it does in this example |
|-----------|------------------------------|
| **Heuristic discount** (`mev − strength` for old tokens) | Makes h\[0\]=3 instead of 7. Path A's deletion element gets f=7 instead of f=11, so it's explored earlier. |
| **Role-conflict bias** (+50) | Makes matching `<b>` as predicate (f=54) prohibitively expensive — the A\* never goes down that path. |
| **Error-insertion fallback** | Provides a cheaper alternative (f=4) to the role conflict, letting Path B attempt a two-triple recovery — but at cost 12. |
| **Deletion with role penalty** (`+strength` for old tokens) | Deleting `<b>` costs 6 instead of 5. This 1-point difference is what separates the paths at the `done`-map convergence point. |
| **`done`-map deduplication** | When both paths reach the same state (Predicate → Iri at `<c>`), the one with lower cost (Path A: 6) survives and the other (Path B: 7) is pruned. |
| **Fingerprint agreement** (bias = 0) | In Path A, every remaining token matches its old role for free.  Four consecutive agreements is what keeps f at 11. |

### Without the fix

Without the heuristic discount **and** the deletion penalty, both paths would
arrive at the convergence point with **cost = 6**.  The `done` map would keep
whichever arrived first — and since Path B is explored at lower intermediate f
values, it would win.  Result: `<a>` kept as subject, `<b>` deleted.

### With the fix

The deletion penalty makes Path B cost 7 at the convergence point while Path A
costs 6.  The heuristic discount ensures Path A is explored early enough to
register its lower cost first.  Result: `<a>` deleted, `<b>` preserved as
subject.  ✓
