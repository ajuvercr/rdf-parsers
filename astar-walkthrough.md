# A\* Error-Recovery Walkthrough: Incremental Token Insertion

This document walks through a concrete example that exercises almost every
mechanism in the incremental A\* parser: the heuristic, role agreement based
on previous parse history, role conflicts, token deletion with history-aware
penalties, error insertion as recovery, and the visited-state pruning that
ultimately decides which path survives.

---

## 1. Setup

### Previous document (valid)

```turtle
<b> <c> <d> .
```

Parsed successfully as one triple. Each token received a **role signature** — a
hash that encodes the grammatical context in which it was matched:

| Role      | Token | Role signature | Error weight |
|-----------|-------|----------------|--------------|
| Subject   | `<b>` | subject-iri    | 1            |
| Predicate | `<c>` | predicate-iri  | 1            |
| Object    | `<d>` | object-iri     | 1            |
| Stop      | `.`   | stop           | 3            |

The **error weight** is the grammar's measure of how costly it is to get a
token wrong — IRIs are cheap (1), while a missing dot is more serious (3).

### New document (invalid — one extra token)

```turtle
<a> <b> <c> <d> .
```

A diff between the old and new token streams transfers the previous role
signatures to tokens that were already present:

| Position | Token | Previous role?             | Error weight |
|----------|-------|----------------------------|--------------|
| 0        | `<a>` | **none** (new token)        | 1            |
| 2        | `<b>` | subject-iri                 | 1            |
| 4        | `<c>` | predicate-iri               | 1            |
| 6        | `<d>` | object-iri                  | 1            |
| 8        | `.`   | stop                        | 3            |

(Odd positions are whitespace tokens that the parser skips.)

### Cost model parameters

- **History bonus**: 1 (used to favour tokens that had a role in the previous parse)
- **Deletion base cost**: 5 × error weight
- **Role conflict penalty**: +50 (when a token's previous role contradicts its current assignment)

---

## 2. Heuristic Computation

The heuristic `h[i]` is a suffix sum of estimated remaining cost. Tokens that
had a role in the previous parse get a **discount** (the history bonus,
clamped to 0), because we expect them to match cheaply:

```
estimated cost per token:
  known token  →  max(error weight − history bonus, 0)
  new token    →  error weight
```

| Position | Token | Known? | Error weight | Estimated cost | h |
|----------|-------|--------|--------------|----------------|---|
| 8        | `.`   | yes    | 3            | max(3−1, 0) = 2 | **2** |
| 6        | `<d>` | yes    | 1            | max(1−1, 0) = 0 | 2 + 0 = **2** |
| 4        | `<c>` | yes    | 1            | 0              | 2 + 0 = **2** |
| 2        | `<b>` | yes    | 1            | 0              | 2 + 0 = **2** |
| 0        | `<a>` | **no** | 1            | 1              | 2 + 1 = **3** |

So `h[0] = 3`. The initial state starts with `cost = 0`, `h = 3`, giving
**`f = 3`**.

> **Key insight:** the heuristic is *optimistic* about paths that keep known
> tokens — it assumes they'll slot into their old roles cheaply. This is the
> A\* admissibility property, tuned for incremental re-parsing.

---

## 3. The Two Competing Deletion Paths

The document has five tokens but a triple only needs three IRIs plus a dot.
One IRI must be deleted. Two candidate paths are:

| Path            | Deletes | Resulting triple |
|-----------------|---------|------------------|
| **A** (desired) | `<a>`   | `<b> <c> <d> .`  |
| **B** (wrong)   | `<b>`   | `<a> <c> <d> .`  |

---

## 4. Step-by-Step: Path B (match `<a>`, delete `<b>`)

The A\* explores this path first because matching `<a>` is cheap.

### 4.1  Match `<a>` as Subject

The grammar expands: TurtleDoc → Statement → Triples → Subject → Iri → expect
Iriref.

Token `<a>` at position 0 **is** an Iriref — it matches.

Since `<a>` is a new token (no previous role), there is no agreement or
conflict — just a neutral match.

| field | value |
|-------|-------|
| cost  | 0 + 1 (error weight) + 0 (no conflict) = **1** |
| h     | h\[2\] = **2** |
| **f** | **3** |
| pos   | 2 (`<b>`) |

### 4.2  Expect predicate at `<b>` — role conflict!

The grammar now expects a predicate. Token `<b>` is an Iriref, so the type
matches. But the **role check** fails:

```
Previous role:  subject-iri   (was subject in the old parse)
Current role:   predicate-iri (grammar is asking for a predicate)
→ CONFLICT: these don't match — same depth, genuine role disagreement
```

This creates **two** alternatives:

| Alternative                | cost              | h | **f** | pos | note |
|----------------------------|-------------------|---|-------|-----|------|
| **Accept** (with conflict) | 1 + 1 + 50 = **52** | 2 | **54** | 4 | prohibitively expensive |
| **Skip & insert error**    | 1 + 1 = **2**    | 2 | **4** | 2   | stays at `<b>`, inserts a dummy predicate |

The accept option (f=54) sinks to the bottom of the priority queue.
Only the error-insertion alternative (f=4) continues.

### 4.3  Error-insertion cascade: fill in a dummy triple

The error-insertion branch (cost=2, pos=2) advances through the grammar
without consuming any tokens. At each missing piece, the parser inserts an
error placeholder:

| Inserted error for | Error weight | cost after | h | **f** | pos |
|--------------------|--------------|------------|---|-------|-----|
| Predicate (Iriref) | 1            | 2          | 2 | 4     | 2   |
| Object (Iriref)    | 1            | 3          | 2 | 5     | 2   |

After completing the first triple's skeleton, the grammar expects a **dot**.
Token `<b>` is not a dot, so that's another error:

| Inserted error for | Error weight | cost after | h | **f** | pos |
|--------------------|--------------|------------|---|-------|-----|
| Dot (`.`)          | 3            | **6**      | 2 | **8** | 2   |

### 4.4  Second triple: `<b> <c> <d> .`

After closing the first (dummy) triple, the grammar loops and begins a new
Statement → Triples → Subject.

Now `<b>` at position 2 is matched as Subject — and this time the **role
agrees** with its previous assignment:

```
Previous role:  subject-iri
Current role:   subject-iri
→ AGREEMENT ✓  — no penalty
```

| step                | cost | h | **f** |
|---------------------|------|---|-------|
| Match `<b>` subject | 7    | 2 | 9     |
| Match `<c>` pred    | 8    | 2 | 10    |
| Match `<d>` object  | 9    | 2 | 11    |
| Match `.` stop      | 12   | 0 | 12    |

**Path B total cost = 12.**

---

### 4.5  Meanwhile: the deletion of `<b>` inside the predicate slot

When the grammar first reaches the predicate slot at position 2 (step 4.2),
the A\* *also* considers **deleting** `<b>`:

```
deletion cost  = 5 × error weight + history penalty
               = 5 × 1            + 1                ← known token → penalised
               = 6
```

The history penalty applies because `<b>` had a role in the previous parse —
deleting a token with established history costs extra.

| field | value |
|-------|-------|
| cost  | 1 (from matching `<a>`) + 6 = **7** |
| h     | h\[4\] = **2** |
| **f** | **9** |
| pos   | 4 (`<c>`) |

After deleting `<b>`, the grammar still expects a predicate. Token `<c>` is
next, and its previous role was predicate — **agreement** ✓.

| step                   | cost | h | **f** |
|------------------------|------|---|-------|
| Delete `<b>` (penalty) | 7    | 2 | 9     |
| Match `<c>` predicate  | 8    | 2 | 10    |
| Match `<d>` object     | 9    | 2 | 11    |
| Match `.` stop         | 12   | 0 | 12    |

**This sub-path also totals cost = 12.**

---

## 5. Step-by-Step: Path A (delete `<a>`, match `<b> <c> <d> .`)

### 5.1  Deletion of `<a>`

Every time the A\* processes a state at position 0, it also offers a deletion
branch for `<a>`. The earliest opportunity is at the Subject → Iri level:

```
deletion cost  = 5 × error weight + history penalty
               = 5 × 1            + 0                ← new token → no penalty
               = 5
```

No history penalty because `<a>` is new — it has no established role to
preserve.

| field | value |
|-------|-------|
| cost  | 0 + 5 = **5** |
| h     | h\[2\] = **2** |
| **f** | **7** |
| pos   | 2 (`<b>`) |

### 5.2  Match `<b>` as Subject (agreement ✓)

After the deletion, the grammar is still at Subject → Iri — the deletion only
advanced the token pointer.

```
Previous role:  subject-iri
Current role:   subject-iri
→ AGREEMENT ✓
```

| field | value |
|-------|-------|
| cost  | 5 + 1 + 0 = **6** |
| h     | h\[4\] = **2** |
| **f** | **8** |

### 5.3  Match `<c>` as Predicate (agreement ✓)

```
Previous role:  predicate-iri
Current role:   predicate-iri
→ AGREEMENT ✓
```

| field | value |
|-------|-------|
| cost  | 6 + 1 = **7** |
| h     | h\[6\] = **2** |
| **f** | **9** |

### 5.4  Match `<d>` as Object (agreement ✓)

```
Previous role:  object-iri
Current role:   object-iri
→ AGREEMENT ✓
```

| field | value |
|-------|-------|
| cost  | 7 + 1 = **8** |
| h     | h\[8\] = **2** |
| **f** | **10** |

### 5.5  Match `.` as Stop (agreement ✓)

```
Previous role:  stop
Current role:   stop
→ AGREEMENT ✓
```

| field | value |
|-------|-------|
| cost  | 8 + 3 = **11** |
| h     | 0 |
| **f** | **11** |

**Path A total cost = 11.** ✓

---

## 6. How the A\* Picks the Winner

### 6.1  Exploration order

The A\* always processes the lowest-f element first:

```
f:  3 ···  4 ···  5 ···  7 ···  8 ···  9 ··· 10 ··· 11 ··· 12
    ↑            ↑       ↑      ↑      ↑      ↑      ↑      ↑
    match <a>  errors  errors  del <a> A:<b>  A:<c>  A:<d>  A:. → SOLUTION (cost 11)
    (Path B)                   (Path A)                      ↑
                                                     Path B arrives later (cost 12)
```

Path B starts strong (f=3) because matching `<a>` is cheap. But every step
hits either a **role conflict** (+50) or an **error insertion**, driving its
cost up. Path B cannot finish below cost 12.

Path A first appears at **f = 7** (the deletion of `<a>`). From there, every
token agrees with its previous role, so the cost grows by just the error
weight at each step — straight to a complete parse at cost 11.

### 6.2  The decisive convergence point

Both paths eventually reach the same grammar state: expecting a predicate at
position 4 (`<c>`). Their costs at that point:

| Path | Cost at `<c>` | How |
|------|---------------|-----|
| A    | **6**         | delete `<a>` (5) + match `<b>` (1) |
| B    | **7**         | match `<a>` (1) + delete `<b>` (5+1) |

The A\* tracks the best-known cost for each visited state (keyed by grammar
position, token position, and role signature). When both paths reach the same
state, **only the cheaper one survives** — Path A with cost 6. Path B (cost 7)
is pruned.

From that point on, only Path A continues. It matches `<c>`, `<d>`, `.` with
full role agreement and produces the solution at cost 11.

---

## 7. Summary: Why Each Mechanism Matters

| Mechanism | What it does in this example |
|-----------|------------------------------|
| **Heuristic discount for known tokens** | Reduces h\[0\] from 7 to 3. Path A's deletion gets f=7 instead of f=11, so it's explored early enough to matter. |
| **Role conflict penalty** (+50) | Makes accepting `<b>` as predicate (f=54) prohibitively expensive — the A\* avoids that dead end. |
| **Error insertion as fallback** | Offers a cheaper alternative (f=4) to the role conflict, letting Path B attempt a two-triple recovery — but at total cost 12. |
| **History penalty on deletion** (+1 for known tokens) | Deleting `<b>` costs 6 instead of 5. This 1-point gap is what separates the two paths at the convergence point. |
| **Visited-state pruning** | When both paths reach the same state (predicate at `<c>`), the cheaper one (Path A: 6) survives and the other (Path B: 7) is discarded. |
| **Role agreement** (no penalty) | In Path A, every token matches its previous role for free. Four consecutive agreements keep the total at just 11. |

### Without the fix

Without the heuristic discount **and** the deletion history penalty, both
paths would arrive at the convergence point with **cost = 6**. The A\* would
keep whichever arrived first — and since Path B has lower intermediate f
values, it would win. Result: `<a>` kept as subject, `<b>` deleted. ✗

### With the fix

The history penalty makes Path B cost 7 at the convergence point while Path A
costs 6. The heuristic discount ensures Path A is explored early enough to
register its lower cost first. Result: `<a>` deleted, `<b>` preserved as
subject. ✓
