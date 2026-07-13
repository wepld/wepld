# ADR-0011 — Replay is a projection over the ledger, never a second recording

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Chief Architect · **Review:** Chronicle V1 gate

## Context

WePLD adds Chronicle: the replay / cinema / forensics / branching / intelligence pillar (v2-11…v2-18). The naive design records a parallel "replay stream" (screen states, player frames, causal annotations) at execution time. Parallel recordings drift from the truth, double the write path, and double the storage and privacy surface — and a solo founder cannot maintain two sources of history.

## Decision

Chronicle has **exactly one recording substrate: the existing audit ledger plus its referenced artifacts** (context packs, brain invocation records, diffs, gate logs, workspace snapshots per ADR-0013). Everything Chronicle shows — frames, scenes, lenses, cameras, causal chains, comparisons, heatmaps, insights — is a **derived, versioned, rebuildable projection**. Derived stores (`frames`, `checkpoints`, `causal_edges`, `insights`) may be dropped and regenerated from the ledger at any time; none is ever authoritative.

Corollary: if a Chronicle capability needs data the ledger does not contain, the fix is a **ledger vocabulary extension** (a versioned Event Contract change per v2-07 §5), never a side channel. Chronicle's MVP additions to the vocabulary: `WorkspaceSnapshotRecorded`, `MissionForked`, `DecisionRevised`, `MissionSuperseded`; V1 additions: `InsightRecorded`, `AnnotationRecorded`, `ReplayExported`.

## Reason

The ledger is already complete for the questions Chronicle answers: every state change, decision, gate, brain call (with the exact context pack hash), envelope crossing, and artifact is a causally linked fact. A projection cannot lie about history it did not write, cannot drift, and inherits the ledger's hash-chain integrity and classification enforcement for free.

## Benefits

Zero new write-path complexity in the mission runtime; replay of *any* historical mission works retroactively, including missions run before Chronicle shipped; stepping backward and arbitrary seeking are free (render a different frame — no reverse execution problem); consistency recovery is `DROP TABLE` + re-derive; the privacy model is unchanged because Chronicle reads through the same classification-filtered query layer as the Studio.

## Trade-offs

Projection generation costs compute at read time — bounded by checkpointing and caching (v2-12 §Performance); anything genuinely unrecorded (e.g., wall-clock UI gestures of the user) is invisible to Chronicle unless promoted into the vocabulary. Accepted: if it isn't worth a ledger fact, it isn't worth replaying.

## Migration impact

None to existing contracts except the versioned vocabulary additions. The mission runtime gains one small write-path duty: workspace snapshot refs at phase boundaries (ADR-0013).
