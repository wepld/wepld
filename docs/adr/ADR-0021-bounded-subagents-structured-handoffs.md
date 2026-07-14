# ADR-0021 — Subagents are bounded roles with structured handoffs, not a swarm

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H6 implementation authorization

## Context

Specialized exploration, implementation, testing, security, performance, documentation, and recovery can improve outcomes, but uncontrolled agent chat creates authority ambiguity, context leakage, conflicting writes, runaway cost, and unauditable completion claims.

## Decision

Hermes supervises a fixed, extensible set of specialized subagent roles. Each receives one objective, scoped Context Pack, skills, tools/capabilities, budget, deadline, output schema, evidence obligations, and stop/escalation rules.

Communication is `Subagent → structured finding/artifact/action proposal → Hermes → Core record when durable`. There is no peer swarm channel. Read-only research may run in bounded parallel; writable work uses isolated worktrees and conflict-controlled or proven-disjoint scopes. Subagents cannot approve plans, effects, exceptions, gates, or completion.

Independent review excludes builder rationalization by default and uses separate attempts/context plus risk-proportionate profile independence.

## Reason

The architecture needs specialized judgment and selective parallelism without delegating governance or losing causal evidence. Structured handoffs preserve audit, budget, context, and recovery boundaries.

## Benefits

- Bounded parallel research and conflict-safe writes.
- Independent review with explicit evidence.
- Replaceable specialist roles.
- Traceable cost, cancellation, and recovery.

## Trade-offs

- Less free-form collaboration.
- Supervisor and handoff schemas add overhead.
- Some context is intentionally omitted and must be cited as artifacts.
- Parallelism may be lower than an uncontrolled swarm.

## Migration

H6 evidence must prove bounded read-only parallelism; one-writer and conflict isolation; cancellation, timeout, and loss handling; structured findings; context separation; contradictory-review escalation; no peer side channel; and denial of subagent approval or completion attempts.

Draft PR #1's child-process worker protocol, attempt budgets, worktrees, and artifacts are candidate substrate. Its single V0 Hermes worker and tests do not establish a Subagent Supervisor or independent review chain, and this ADR does not authorize that expansion.
