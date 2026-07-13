# ADR-0002 — One worker runtime with role switching, not separate role runtimes

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Chief Architect · **Review:** after Phase A experiment E1 (v2-09)

## Context

v1 specified 24 worker role families and a scheduler that leases tasks to role-specialized workers, with all inter-role communication mediated as typed artifacts through the orchestrator. The gate review flagged (C5) that the empirical record for agentic coding favors a single strong agent loop with good tools over rigid multi-agent pipelines: published multi-role frameworks (MetaGPT-, ChatDev-style pipelines) underperform simpler single-loop agents (OpenHands/CodeAct, Claude-Code-style harnesses) on real-repository benchmarks such as SWE-bench, because every mediated handoff loses context, and context quality dominates outcome quality. Separate runtimes also require the fleet machinery (registry, leases, heartbeats, scheduler) before the first mission can run.

## Decision

**One worker runtime binary. Roles are declarative profiles. The Core spawns the runtime once per phase (or phase group) with the role profile, a phase-scoped sandbox envelope, and a phase-scoped context pack.** Role separation is achieved by three enforced isolations rather than by process multiplicity:

1. **Envelope isolation** — the reviewer phase's process has a read-only filesystem view; the builder phase's process can write only its worktree (v2-05).
2. **Context isolation** — the Core's Context Assembly builds each phase's context pack; the review phase receives the diff, brief, criteria, and test evidence but **not** the builder's reasoning transcript, eliminating self-review bias by construction (v2-04).
3. **Brain-profile isolation** — a phase may request a different brain profile (e.g., independent review by a second model) without any runtime change.

The Worker Contract (WWP, v2-07) retains lease/heartbeat/cancel semantics so the same protocol later supports separate, pooled, or remote runtimes without contract change. This is the **hybrid** posture: single runtime now, protocol-ready for a fleet.

## Reason

The org-chart-as-processes reading of v1 was an unvalidated bet with the worst cost/risk ratio in the plan. The properties the architecture actually needs from "specialized workers" — least privilege, independent review, auditable handoffs, replaceability — are all achievable per phase with one runtime, at a fraction of the cost.

## Benefits

Removes registry/scheduler/lease-arbitration from the MVP critical path (~30–40% of the v1 core effort); preserves every governance property; makes the orchestration-thesis experiment (Phase A, E1) cheap to run because both arms use the same runtime with different context/isolation settings.

## Trade-offs

No task parallelism in the MVP (tasks are sequential; parallelism returns with the fleet in V2 per v2-09). The "engineering organization" is realized as governed phases rather than concurrent processes; the narrative is less theatrical and the audit trail is identical.

## Migration impact

Fleet evolution is additive: introduce a worker registry and let Core dispatch `attempt.start` over a socket instead of stdio. WWP messages, envelopes, and context packs are unchanged. Roles never encode runtime identity, so no mission-domain change occurs.
