# ADR-0018 — Hermes uses executable Skill Runtime and governed Hook Bus contracts

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H3 implementation authorization

## Context

The current Skills plan defines instructions, templates, checks, and permissions but not a complete executable procedure contract. Lifecycle hooks are absent. Ad hoc prompt skills or plugins could bypass policy, duplicate workflow logic, or turn callbacks into hidden effects.

## Decision

Adopt the Skill Runtime and Hook Bus in [32_Hermes_Engineering_Intelligence_Runtime.md](../32_Hermes_Engineering_Intelligence_Runtime.md).

A skill declares applicability, context/tool requirements, requested capabilities, procedure, verification, failure modes, typed output/evidence, compatibility, provenance/trust, and evaluation. Hermes resolves exact versions; Core grants only an allowed subset of requested capability.

H3.1 contains only repository-owned skills described by static, versioned manifests and built-in lifecycle hooks. The manifest binds an exact content hash, typed input/output schemas, declared context/tool/capability requirements, verification/evidence obligations, bounded failures, and compatibility. A normal repository review plus exact revision/hash is its provenance boundary; H3.1 has no external install, registry, marketplace, third-party executable hook, dependency resolver, or general publisher-signing infrastructure.

Built-in hooks are typed as observational, validating, blocking, or effect-producing. Every hook is versioned, scoped, timed, idempotent, evidence-producing, and policy-registered. An effect-producing hook may only propose a new action that re-enters the Effect Firewall.

H3.1 also establishes a versioned **internal** Hermes lifecycle/control-event port for the Agent Kernel, Skill Runtime and built-in Hook Bus. Messages carry exact action/attempt/capability correlation, schema version, idempotency, ordering and backpressure semantics; reconnect rebuilds an operational projection and never creates authority. This decision does not expose a public SDK or authorize ACP, MCP, editor, terminal or other external client adapters.

For each attempt, Core derives a `CapabilityProjectedToolCatalog` by intersecting the approved packet, policy, issued capabilities, role, scope, classification, availability, compatibility, and context budget. Hermes receives only eligible versioned tool schemas with exact hashes and selection reasons; omitted entries and reasons remain in the attempt manifest. Tool/skill and future MCP catalogue descriptions have explicit item/token budgets. H3.1 projects repository-owned built-ins only. Any later catalogue discovery is bounded and provenance-labelled and cannot auto-connect, auto-install, auto-trust, or grant ambient package authority.

Projection reduces irrelevant tool-schema context but is not an enforcement boundary. Every invocation still re-enters Core policy, capability, approval, idempotency, and effect checks. A visible schema grants nothing, and a hidden schema does not substitute for denial at the tool boundary.

H3.2 is a separate, conditional packaging slice. It may begin only after H3.1 shows measured skill/hook value and RS-19 demonstrates the required supply-chain and governance controls. H3.2 may then stage exact skill/hook packages with provenance/signature checks; only Core may atomically activate, roll back, deactivate, or revoke an approved scope. If the benefit or governance proof fails, WePLD keeps built-in skills/hooks and does not create a package surface. Provider, worker, LSP/retrieval, protocol, UI and other package types repeat the package experiment and architecture gate at their own milestones.

## Reason

Engineering expertise becomes testable and reproducible only when its procedure and proof obligations are executable contracts. Typed hooks enable extension and instrumentation without an ungoverned plugin escape path.

## Benefits

- Deterministic, hash-pinned skill routing.
- Repeatable procedures and reusable verification.
- Observable, bounded, and revocable hooks.
- Explicit capability and evidence contracts.
- Deterministic internal control/event correlation and reconnect behavior.
- Capability-projected, hash-pinned tool schemas with explicit catalogue budgets.
- A small built-in H3.1 surface that can ship without package-distribution infrastructure.
- Conditional, atomic, reversible H3.2 skill/hook package activation without implying a marketplace.

## Trade-offs

- Prompt-only “skills” must be upgraded or treated as untrusted context.
- Hook ordering, reentrancy, timeout, and failure semantics add runtime complexity.
- Internal protocol compatibility adds schemas and failure cases; H3.2 staged package state adds more only if separately admitted.
- An open marketplace remains deferred.

## Migration

H3.1 evidence must prove repository-owned hash-pinned resolution; manifest and compatibility/capability denial; typed outputs; skill failure and cleanup; evidence contracts; required built-in lifecycle hooks; timeout, idempotency, and reentrancy guards; fail-closed blocking behavior; denial of an attempted hook side channel; internal message schema/version/correlation integrity under duplicate, reorder, reconnect and backpressure cases; deterministic capability-projected schema sets and hashes; catalogue item/token-budget enforcement; omitted-tool audit completeness; and repeat denial at the actual tool boundary. The acceptance suite must also prove that external installation, registry lookup, arbitrary executable hooks, auto-trusted MCP/package resources, ambient package authority, and undeclared resource loading are absent.

H3.2 requires a separate decision at its gate and must prove provenance/signature handling, isolated staging, atomic activation, rollback, revocation, active-mission impact, and every applicable RS-19 threshold. External protocols and non-skill/hook packages remain explicitly outside this migration.

Draft PR #1's candidate `SkillPin`, role profile, worker packet, and empty skill list are seams, not an implemented Skill Runtime. No branch-local skill or hook gains trust through this proposal, and this ADR does not authorize candidate implementation.
