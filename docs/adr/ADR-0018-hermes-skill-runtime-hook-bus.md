# ADR-0018 — Hermes uses executable Skill Runtime and governed Hook Bus contracts

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H3 implementation authorization

## Context

The current Skills plan defines instructions, templates, checks, and permissions but not a complete executable procedure contract. Lifecycle hooks are absent. Ad hoc prompt skills or plugins could bypass policy, duplicate workflow logic, or turn callbacks into hidden effects.

## Decision

Adopt the Skill Runtime and Hook Bus in [32_Hermes_Engineering_Intelligence_Runtime.md](../32_Hermes_Engineering_Intelligence_Runtime.md).

A skill declares applicability, context/tool requirements, requested capabilities, procedure, verification, failure modes, typed output/evidence, compatibility, signing/trust, and evaluation. Hermes resolves exact versions; Core grants only an allowed subset of requested capability.

Hooks are typed as observational, validating, blocking, or effect-producing. Every hook is versioned, scoped, timed, idempotent, evidence-producing, and policy-registered. An effect-producing hook may only propose a new action that re-enters the Effect Firewall.

H3 also establishes a versioned **internal** Hermes lifecycle/control-event port for the Agent Kernel, Skill Runtime and Hook Bus. Messages carry exact action/attempt/capability correlation, schema version, idempotency, ordering and backpressure semantics; reconnect rebuilds an operational projection and never creates authority. This decision does not expose a public SDK or authorize ACP, MCP, editor, terminal or other external client adapters.

Package lifecycle at H3 is limited to skills and hooks. The Registry may verify and stage an exact hash, but only Core may atomically activate, deactivate or revoke it for an approved scope. Provider, worker, LSP/retrieval, protocol, UI and other package types repeat the package experiment and architecture gate at their own milestones.

## Reason

Engineering expertise becomes testable and reproducible only when its procedure and proof obligations are executable contracts. Typed hooks enable extension and instrumentation without an ungoverned plugin escape path.

## Benefits

- Deterministic, hash-pinned skill routing.
- Repeatable procedures and reusable verification.
- Observable, bounded, and revocable hooks.
- Explicit capability and evidence contracts.
- Deterministic internal control/event correlation and reconnect behavior.
- Atomic, reversible skill/hook package activation without implying a marketplace.

## Trade-offs

- Prompt-only “skills” must be upgraded or treated as untrusted context.
- Hook ordering, reentrancy, timeout, and failure semantics add runtime complexity.
- Internal protocol compatibility and staged package state add schemas and failure cases.
- An open marketplace remains deferred.

## Migration

H3 evidence must prove hash-pinned resolution; compatibility/capability denial; typed outputs; skill failure and cleanup; evidence contracts; required lifecycle hooks; timeout, idempotency, and reentrancy guards; fail-closed blocking behavior; denial of an attempted hook side channel; internal message schema/version/correlation integrity; duplicate, reorder, reconnect and backpressure handling; and atomic stage/activate/rollback/revoke behavior for skill and hook packages. External protocols and non-skill/hook packages are explicitly outside this migration.

Draft PR #1's candidate `SkillPin`, role profile, worker packet, and empty skill list are seams, not an implemented Skill Runtime. No branch-local skill or hook gains trust through this proposal, and this ADR does not authorize candidate implementation.
