# v2-15 — Mission Branching and Decision Editing

History is never destroyed and never rewound; every continuation is a fork (ADR-0012). This document specifies the mechanics.

## 1. The snapshot triple

A **Mission Snapshot** at ledger sequence S is not a stored object — it is a *reconstructible coordinate* of three existing things:

| Component | Source | Materialization |
| --- | --- | --- |
| Control state | `state_at(mission, S)` — checkpoint + fold (v2-12) | milliseconds |
| Workspace state | nearest `WorkspaceSnapshotRecorded` ≤ S → Git ref (ADR-0013) | `git worktree add --detach`, seconds |
| Informational state | artifacts/packs/decisions referenced by entries ≤ S | already content-addressed |

Because snapshots are coordinates, every historical point of every mission is a potential fork origin at zero storage cost.

## 2. Forking

`ForkMission{mission, at_seq, reason, mode?}` — a Core command, one transaction:

1. Reconstruct the snapshot triple at `at_seq`.
2. Create mission B: `MissionCreated{forked_from:{mission:A, seq}, reason}` — B's brief inherits A's (classification, budget fresh by default; user may edit before launch).
3. Write `MissionForked{child:B, at_seq}` into **A's** ledger (A's history now records that it was studied and branched — nothing else about A changes).
4. Insert `lineage(B, A, at_seq, reason)`.
5. Create B's worktree branching from the snapshot ref: `git branch wepld/misB <snap-ref>`.
6. B's opening context carries the **fork preamble** (a T1 pack section): "this mission continues A from seq S; A's subsequent history is reference, not instruction."

B is a full mission: same gates, same envelope discipline, same ledger, fully replayable — Chronicle needs no special cases for branches because branches are just missions with lineage.

**Lineage lattice** (all derived views over `lineage` + `revised`/`carried` edges): Branch Lineage (the fork tree), Decision Lineage (a decision and its revisions across branches), Evidence Lineage (an artifact and its carried/superseded successors), Replay Lineage (which replay/forensic session motivated a fork — the fork `reason` records the provenance breadcrumb, so *even navigation history is causal*).

## 3. Decision editing — the full sequence

Scenario contract (the Part-5 case): mission A is at step 42; at step 18 the human approved PostgreSQL; they now want SQLite.

1. **Locate.** In Cinema, the user opens the Decision lens, seeks to the beat frame for `dec_2` (seq 18), clicks **Revise**.
2. **Restore.** Core executes `ForkMission{A, at_seq: seq(dec_2) − 1}` — the snapshot immediately *before* the resolution: the packet is open again, the world is as it was.
3. **Reconstruct context.** The child's pack for the decision point is the *original captured pack* (hash-identical — this is why packs are captured) plus the fork preamble.
4. **Revise.** The user resolves the packet differently: `DecisionRevised{revises: dec_2, option: "sqlite", rationale}` — authority checked by Core exactly as the original was; a revision is a decision.
5. **Invalidate.** Core computes the **impact cone** of `dec_2` in A (v2-14): every task, artifact, gate result, and knowledge record downstream of the PostgreSQL choice. The cone is presented as the **invalidation report**: "these 14 outcomes of A depended on the revised decision and will not carry into B." Work in A *after* the fork point but *outside* the cone is listed as salvageable (V1) or simply not carried (MVP).
6. **Re-plan.** B's planning phase runs with a `DecisionDelta` T1 section: old choice, new choice, rationale, invalidated set, salvage list. The planner produces plan v2 — plan approval applies per B's autonomy mode.
7. **Continue.** B executes normally to its own completion proposal.
8. **Explain.** Chronicle auto-generates the A↔B comparison (§6) — "exactly what changed": decisions (1 revised), plan (tasks Δ), files (git diff of outcomes), cost, duration, evidence. Rendered under claims discipline.
9. **Preserve.** A remains exactly as it was (running, paused, or completed — the user chooses whether to pause/cancel A, and adoption is explicit: `MissionSuperseded{by:B}` on A only when the user says so). Both histories permanent, both replayable, lineage navigable in the Branch Graph.

MVP ships steps 1–9 with salvage stubbed (nothing carried); V1 ships salvage: adopting a parent artifact writes a `carried` edge and re-runs the gates that depended on it in B's context — carried evidence is *re-proven*, not assumed.

## 4. Branch lifecycle and metadata

`lineage` rows carry: fork seq, reason, motive class (`decision_revision | what_if | recovery | learning`), creator, and outcome (`open | adopted | superseded | archived`). Branch Graph (V1) renders the tree with outcomes; archived branches remain replayable forever (they are missions; retention policy applies uniformly).

Concurrency: A live parent and its branches are independent missions; the MVP's one-live-mission-at-a-time constraint (v2-01) means launching B prompts to pause A — the constraint is scheduling, not architecture, and lifts with the V2 fleet.

## 5. Mission merge = adoption (ADR-0012)

"Merging branch B" means: B completes → its worktree branch merges via Git at acceptance (the normal completion hard gate) → `MissionSuperseded{by:B}` recorded on rejected alternatives → optionally, knowledge records from superseded branches are carried or superseded explicitly. No mission-history merge exists; the Branch Graph *is* the merged narrative.

## 6. Comparison — the visual diff contract

`Compare{a:{mission,seq}, b:{mission,seq}, facets[]}` returns a facet-diff document (contract in v2-17):

| Facet | Computed from |
| --- | --- |
| Files/code | `git diff` between the two workspace refs |
| Decisions | packet sets: revised / added / removed, with rationales side-by-side |
| Reasoning | brain invocations aligned by (task, phase, intent): provider/model deltas, pack hash deltas → *pack section diff* (which context differed) |
| Context | selection-manifest diff: files included/omitted differently |
| Evidence | gate and criteria matrices side-by-side |
| Policy | envelope grants/denials deltas; tier deltas |
| State | `state_at` folds diffed field-by-field |
| Worker behavior | phase durations, retries, escalation counts |
| Outputs | final artifacts, cost, duration, interrupt count |

MVP renders this as the two-column comparison report; V2 upgrades it to synchronized split-view playback (v2-13 §4) over the same data. Alignment rule everywhere: pre-fork by sequence, post-fork by logical clock (task index, phase).
