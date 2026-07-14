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

## Reason

Engineering expertise becomes testable and reproducible only when its procedure and proof obligations are executable contracts. Typed hooks enable extension and instrumentation without an ungoverned plugin escape path.

## Benefits

- Deterministic, hash-pinned skill routing.
- Repeatable procedures and reusable verification.
- Observable, bounded, and revocable hooks.
- Explicit capability and evidence contracts.

## Trade-offs

- Prompt-only “skills” must be upgraded or treated as untrusted context.
- Hook ordering, reentrancy, timeout, and failure semantics add runtime complexity.
- An open marketplace remains deferred.

## Migration

H3 evidence must prove hash-pinned resolution; compatibility/capability denial; typed outputs; skill failure and cleanup; evidence contracts; required lifecycle hooks; timeout, idempotency, and reentrancy guards; fail-closed blocking behavior; and denial of an attempted hook side channel.

Draft PR #1's candidate `SkillPin`, role profile, worker packet, and empty skill list are seams, not an implemented Skill Runtime. No branch-local skill or hook gains trust through this proposal, and this ADR does not authorize candidate implementation.
