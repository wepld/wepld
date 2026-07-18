# ADR-0017 — Phase, Kanban, and controlled change are durable Core semantics

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H2 implementation authorization

## Context

The existing plan has mission, task, and attempt states and roadmap “phases,” but no delivery Phase entity, PhasePlan, enforced Kanban flow, WIP limits, or typed distinction between changing WHAT and changing HOW. UI-only boards or silent replanning would not provide governance.

## Decision

Make Phase the primary delivery unit. An approved, versioned `PhasePlan` binds objective, entry/exit conditions, dependencies, skills/tools, writable/forbidden scope, task set, WIP, budget, controls, evidence, gate, and escalation.

Use the namespaced phase lifecycle and task Kanban states in [31_Governed_Specification_Workflow.md](../31_Governed_Specification_Workflow.md). Core enforces transitions and policy-configured WIP; Hermes schedules within them. Specification changes create new WHAT versions; plan changes create new HOW versions. Both compute an impact cone and stale affected descendants.

After plan approval, a deterministic SOP Compiler projects exact approved `DeliveryPlan`, `PhasePlan`, and `TaskPacket` versions into a candidate `SOPGraph`. It contains typed `RoleNode`, `ActionContract`, authorized `InputSubscription`, `OutputContract`, dependency edges, evidence obligations, and stop/escalation rules. Core validates the graph and projects only assignment-authorized events/artifacts. Roles cannot self-subscribe, observe a free shared environment, or create a peer-chat authority path; the graph coordinates execution but never replaces its parent artifacts.

## Reason

Phase gates make incremental adaptation reviewable. Durable Kanban and WIP prevent uncontrolled swarm behavior and expose blocked decisions and effects. Typed change control preserves approved meaning while permitting evidence-driven replanning.

## Benefits

- Traceable incremental delivery and phase-by-phase authorization.
- Bounded concurrency and conflict-controlled writes.
- Honest blocked, returned, deferred, and uncertain states.
- Observable change impact and descendant invalidation.

## Trade-offs

- Additional state machines and version links.
- Parallelism requires reliable scope analysis.
- Low-risk missions need tailored compact phases rather than bypasses.

## Migration

H2 evidence must exercise tailored phase graphs; stable SOP compilation from exact parents; missing/stale/forged parent rejection; unauthorized subscriptions, shared-environment observation, peer broadcast, and free-chat denial; role input revocation/replay; invalid entry/exit transitions; WIP saturation; one writer per worktree; parallel read-only research; decision/effect backlog limits; phase return, defer, uncertainty, and recovery; and exact invalidation from specification-versus-plan changes. RS-21 and RS-22 must meet their controlled benefit and zero-authority-leak thresholds before the projection is admitted.

Draft PR #1's candidate `PlanDoc`, `TaskSpec`, attempts, envelopes, gates, and transitions can seed migration. Its single task and non-durable phase labels are V0 limitations. No H2 implementation begins before candidate-baseline disposition and ADR review, and this ADR does not authorize the PR.
