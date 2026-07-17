# ADR-0013 — Code time travel uses Git snapshot refs, not a parallel file store

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Core owner · **Review:** Phase B exit

## Context

Replay, forking, and forensics need the *workspace state at any ledger point*: what the code looked like when a phase started, what a reviewer actually reviewed, where a fork should restore from. v2 records `base_commit` per attempt and diffs as artifacts, which is enough to display changes but not enough to cheaply *materialize* an arbitrary historical workspace or bisect across it.

## Decision

The mission runtime commits the worktree state to a **hidden Git ref** at every phase boundary and recovery snapshot: `refs/wepld/<mission>/<attempt>/<phase>-<end|snap>`, created with a temporary index (never touching the user's branches, never appearing in normal `git log`). Each commit's hash is recorded in a `WorkspaceSnapshotRecorded` ledger entry. Chronicle materializes any point via `git worktree add --detach <ref>`; diffs between any two points are `git diff <refA> <refB>`; regression bisection (v2-14) walks these refs.

## Reason

Git already is a content-addressed, delta-compressed, battle-tested snapshot store sitting inside every project WePLD touches. A parallel file-snapshot store would duplicate gigabytes, reimplement deduplication badly, and desynchronize from source truth — violating the v1 principle that source code remains owned by Git.

## Benefits

Near-zero storage overhead (objects deduplicate; typical phase deltas are small); instant historical checkout for fork restore; `git diff` powers visual comparison for free; large-repository behavior inherits Git's own scaling.

## Trade-offs

Refs accumulate — bounded by a retention sweep (refs for missions past retention are deleted; the diffs-as-artifacts remain for display, so audit survives ref pruning while *materialization* expires). Untracked-but-relevant files (build outputs) are excluded by gitignore exactly as they are from user commits — snapshots capture source truth, not build state; gate logs already capture build outputs as artifacts.

## Migration impact

One new write-path step at phase boundaries (~milliseconds); one vocabulary entry; recovery (v2-02 §7) now records its snapshot as a ref + entry instead of an ad-hoc artifact — strictly simpler.
