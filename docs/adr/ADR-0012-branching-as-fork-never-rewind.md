# ADR-0012 — Mission branching is fork-as-new-mission; history is never rewound

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Chief Architect · **Review:** after first cohort branch usage data

## Context

Users must return to any point of a mission, inspect it, edit a decision, and continue — without destroying history (v2-15). Two families of design exist: (a) mutable time travel — rewind the mission's own state and re-run; (b) immutable forking — every continuation is a new mission with recorded lineage.

## Decision

**Forking only.** `ForkMission(mission, at_seq)` creates a *new mission* whose state is reconstructed at `at_seq` (ledger fold + workspace snapshot ref), with lineage recorded on both sides (`MissionForked` in the parent's ledger; `forked_from{mission, seq}` in the child's `MissionCreated`). The parent mission is untouched — not paused, not mutated, not annotated in place. Decision editing (v2-15 §3) is a specialization: fork immediately before a `DecisionResolved`, record `DecisionRevised{revises, new_option, rationale}` in the child, invalidate the revised decision's downstream causal cone, re-plan, continue.

**Mission merge is adoption, not history merging.** Concluding a branch exploration means: adopt one branch's outcome (its worktree branch merges through Git exactly as any mission completion does), record `MissionSuperseded{by}` on the others, and optionally carry knowledge records across by supersession. Automatic semantic merging of two mission *histories* is rejected (Future at most): mission histories are causal narratives, not mergeable text.

## Reason

Rewind-in-place cannot coexist with a hash-chained append-only ledger, with audit trust, or with the "history is never destroyed" requirement — a rewound mission is a mission whose past is a lie. Forking maps 1:1 onto machinery that already exists (mission creation, state fold, Git worktrees) and makes "what would happen if…" an *honest* operation: a real branch, really executed, really costed — not a simulation with invented physics.

## Benefits

Immutability by construction; branches are first-class missions (all gates, budgets, ledger, replay work unchanged); the branch graph is a simple lineage tree; comparisons reduce to the existing two-mission diff; a live parent and its branch can run concurrently or the user can pause the parent — their choice, not the architecture's.

## Trade-offs

Post-fork-point work in the parent is not automatically reused in the child. MVP accepts recomputation (correct and simple). V1 adds **salvage**: the child's planner is told which parent artifacts are causally independent of the revised decision and may adopt them by hash, with gates re-run to validate them in the new context. Storage cost of many branches is bounded by content addressing (identical artifacts and Git objects deduplicate).

## Migration impact

Adds a `lineage` table and three vocabulary entries; no change to mission state machine semantics — a fork is `MissionCreated` with provenance.
