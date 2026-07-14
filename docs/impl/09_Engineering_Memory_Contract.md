# Engineering Memory — Contract (Build Feature slice)

**Status:** implemented for the Build Feature reference experience only.
**Scope of this document:** what a *lesson* is, how it is identified, scoped,
and applied, and the durability/consistency guarantees around it. This is a
reliability-and-safety contract, not a roadmap for general memory.

Engineering Memory lets Hermes carry evidence-derived **lessons** from a
completed mission into future missions **on the same repository**. It closes the
loop the founder requires — a completed mission leaves the codebase, Hermes, and
the Engineering Memory better — without letting unverified model text become
durable "knowledge."

The narrow, defensible claim after this slice:

> For the Build Feature reference experience, WePLD can durably retain, safely
> scope, and reproducibly apply evidence-derived engineering lessons across
> mission and process boundaries.

---

## 1. Lesson identity

A lesson is a row in the `lessons` table (`crates/ledger/src/store.rs`) with an
opaque, durable, **derived** id: `lesson_{mission_id}`. Deriving the id from the
source mission makes recording **idempotent** — re-processing the same
acceptance cannot mint a second lesson.

Each lesson references:

| Field | Meaning |
|---|---|
| `lesson_id` | opaque durable identity (derived from the mission) |
| `repo` | project/repository identity — the scope key |
| `mission_id` | source mission |
| `spec_id` | source specification (nullable) |
| `gates_json` | reusable verification recipes learned: `(gate, command)` |
| `files_json` | files the mission's diff touched |
| `confidence` | evidence-derived confidence in `[0,1]` (see §4) |
| `status` | lifecycle state (see §2) |
| `created_at` | zero-padded millis, for deterministic ordering |
| `created_seq` | ledger sequence of the `InsightRecorded` fact that created it |

Provenance is complete: from any lesson you can reach the source mission, its
specification, its evidence (gates + diff files), and the exact immutable ledger
fact (`created_seq`) that recorded it. A language model never writes a lesson
directly — the Core extracts it from durable ledger facts at acceptance.

## 2. Lifecycle

Conceptual states: **Candidate → Active → Superseded → Invalidated → Archived**.

V0 implements a conservative subset: lessons are recorded with `status =
"candidate"` and are eligible for application. `Superseded`, `Invalidated`, and
`Archived` are defined here as the intended transitions but are **not** yet
mutated automatically — there is no autonomous rewriting or deletion of memory
in this slice.

**Not every accepted mission produces a lesson.** A valid outcome is
`No durable lesson extracted.` Recording is skipped when:

- the mission did not reach `accepted` (failed / cancelled / rejected / gate
  failing / missing evidence); or
- the mission carries no reusable evidence — **neither** a passed verification
  gate **nor** a changed file (`has_reusable_evidence`); or
- a lesson for that mission already exists (idempotent no-op).

This keeps memory conservative: it records verified success, not attempts, and
never manufactures filler to satisfy a metric.

## 3. Scope

Lessons are scoped by **exact repository identity** (`lessons_for_repo(repo)`).
A lesson learned in repository A is never supplied to repository B, and the
learned / applied / total counters are computed per scope, so they cannot
mislead across projects. Broader matching (path/subsystem, language/toolchain,
recipe, gate type) is deliberately **out of scope** for this slice.

## 4. Confidence vs. validity — do not conflate

Four different things are kept distinct and must not be collapsed into one
number:

1. **A check ran** — a `GateEvaluated` fact exists.
2. **A change succeeded** — the gate `passed`.
3. **The inferred lesson is correct** — *not asserted.* A lesson is labelled
   evidence, its `confidence` is the mission's evidence-derived confidence, and
   it is explicitly untrusted context when applied (§5).
4. **The lesson applies to a future mission** — *not asserted.* Applicability is
   the model's judgement over labelled context, never an automatic guarantee.

`confidence` on a lesson is (3)-adjacent only in that it reports the *evidence
strength of the source mission*, computed by `derive_confidence` from recorded
facts. It is never a claim that the lesson is universally true or applicable.

## 5. Application semantics

When memory informs a new specification (`specify_memory_entries` →
`reason_spec_from_request`), each entry is **labelled, provenance-carrying
context** placed only in the `engineering_memory` field of the specify pack.
WePLD constructs the pack; the model never does. Therefore memory can only ever
enter reasoning through this one clearly-delimited, untrusted channel.

Each entry carries `lesson_id`, `source_mission`, `provenance:
"evidence-derived"`, `trust: "untrusted-context"`, `confidence`, `title`, and a
size-bounded `body`. Guarantees:

- **Separated from authority.** Memory is never the `request`/`intent`, never a
  capability, never a policy input, and never a mission's acceptance criteria.
  Instruction-like or malicious text inside a lesson survives only as an escaped
  JSON *data* string, never as a directive (test:
  `injection_text_in_a_lesson_stays_contained_as_untrusted_data`).
- **Bounded.** At most `MAX_LESSONS_IN_PACK` (5) lessons, each body truncated to
  `MAX_LESSON_BODY_CHARS` (2000). Byte-identical bodies are de-duplicated.
- **Deterministic.** Selection is ordered by confidence, then recency, then id —
  stable across calls (stable cassette keys).
- **Observable & attributable.** The `BrainInvoked` fact for the specify step
  records `applied_lessons` (which lesson ids informed which mission). The
  reported `applied` count is the bounded selection actually sent — never the
  raw repo total — so the metric never overstates.

## 6. Transaction & event semantics

Invariant: **a lesson becomes visible as a durable row only when its metadata
and its ledger fact are mutually consistent.**

Achieved with the smallest mechanism consistent with the architecture — one
SQLite transaction (`Tx`, the single-writer surface) that:

1. appends the `InsightRecorded` ledger fact (hash-chained), then
2. stamps the row with that fact's `seq` (provenance) and inserts it.

Either both commit or neither does. A partial failure rolls back atomically
(tests: `lesson_and_event_roll_back_together_on_failure`,
`duplicate_lesson_id_leaves_no_orphan_event`).

The content-addressed **body** is written to the CAS *before* the transaction.
This is safe because the CAS is content-derived and write-once: an identical
body deduplicates, and an orphaned body (if the transaction never commits) is
harmless and never referenced. No lesson is ever visible without its committed
row **and** committed ledger fact. There is no distributed-transaction or
outbox machinery.

## 7. Workspace snapshot safety (adjacent)

Lesson extraction reads the mission's diff artifact, which is produced by a
workspace snapshot. Snapshots use a race-safe, self-cleaning temporary index
directory (`tempfile`, `crates/workspace/src/lib.rs`): unique creation, no
following of a pre-existing symlink, restrictive permissions where the OS
supports it, and removal on drop (success **and** error paths). Concurrent or
repeated snapshots never share an index path or collide on a stale lock (tests
in `crates/workspace/tests/workspace_tests.rs`).

## 8. Explicitly out of scope for this slice

Generalization to every recipe/mission type; embeddings or semantic retrieval;
a knowledge graph; cross-repository learning; model-driven lesson rewriting;
autonomous memory deletion; cloud sync. None of these are implemented, and none
are required for the claim in the header.
