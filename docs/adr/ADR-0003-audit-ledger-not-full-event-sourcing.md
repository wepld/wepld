# ADR-0003 — Transactional state plus append-only audit ledger, not full event sourcing

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Core owner · **Review:** before any V2 multi-writer or sync work

## Context

v1 (docs 16, 17) specified event-sourced workflow state with CQRS projections, an outbox, upcasters, and projection replay/versioning. The gate review (M1) found this the heaviest pattern load on the menu for a single-writer local SQLite application, consuming core engineering on schema evolution and replay testing while the product thesis is still unproven. The properties actually required by the non-negotiable principles are: durable recovery, complete attributable audit, timeline/replay rendering, and observability — not state-rebuild-from-events purity.

## Decision

The Core stores **workflow state in ordinary transactional tables** (missions, plans, tasks, attempts, decisions) guarded by an explicit state machine, and writes an **append-only audit ledger** in the same SQLite transaction as every state change. Ledger entries carry the full v1 event envelope (actor, causation, correlation, schema version, payload hash, previous-entry hash chain). The ledger is the source of truth for *history*; the tables are the source of truth for *current state*. Timeline, replay, Messenger reports, and audit exports read the ledger. Mechanics in [v2-06](../v2/06_State_and_Ledger.md).

## Reason

Same-transaction table+ledger writes give crash consistency without projection infrastructure. Every user-visible v1 promise (traceability, replayable explanation, rebuildable timeline, tamper evidence) is a ledger read. What is given up — deriving current state by folding events — pays for itself only with multiple writers or offline sync, both explicitly deferred.

## Benefits

Roughly half the core-daemon effort of the v1 design; no upcaster/projection-versioning discipline needed while contracts are still moving; recovery logic reads one table (`attempts` in non-terminal states) plus the worktree, instead of replaying streams.

## Trade-offs

State-transition bugs can produce a current state that disagrees with the ledger narrative; mitigated by (a) a single transition function that is the only writer of both, and (b) a consistency checker that folds the ledger and diffs against tables in CI and on startup. If full ES is later justified, history before the migration point is the existing ledger — no data loss, but pre-migration state cannot be *re-derived*, only attested.

## Migration impact

The ledger schema is deliberately ES-compatible (aggregate id/type, sequence, envelope fields). Promotion path: declare ledger authoritative per aggregate, generate tables as projections, add an outbox. Contracts in v2-07 do not change.
