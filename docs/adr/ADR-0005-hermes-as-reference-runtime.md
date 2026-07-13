# ADR-0005 — "Hermes" is WePLD's first-party reference worker runtime, not a dependency

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Chief Architect · **Review:** Phase A contract freeze

## Context

v1 referenced "Hermes-compatible workers" seven times as the V1 worker adapter — and once as a *hosted brain adapter* — without ever defining, linking, or evaluating it. The gate review (C4) flagged an undefined, load-bearing dependency that also blurred the brain/worker separation the architecture treats as foundational.

## Decision

1. The worker execution contract is the **WePLD Worker Protocol (WWP)** — a WePLD-owned, versioned contract specified in [v2-07 §Worker Contract](../v2/07_Contracts.md) and [v2-03](../v2/03_Worker_Runtime_and_Hermes.md). WePLD depends only on WWP.
2. **Hermes** is the codename of WePLD's **first-party reference implementation** of WWP: the single-runtime, role-switching worker of ADR-0002. It has no privileged status; it is replaceable by any conformant runtime and must pass the same WWP conformance suite required of third-party runtimes.
3. The name "Hermes" is **removed from the brain layer**. Brain adapters are named by provider family only.

## Reason

A contract the company owns cannot be hostage to an unspecified external protocol, and a reviewer must be able to inspect every load-bearing element. Naming the reference implementation preserves whatever internal history the codename carries while making the dependency direction explicit: runtimes conform to WWP; WePLD never conforms to a runtime.

## Benefits

Eliminates the specification hole; gives principle 3 (replaceable worker runtime) a concrete mechanism (WWP + conformance suite); prevents the reference runtime from accreting private side channels, because it may only use WWP messages.

## Trade-offs

WePLD must author and maintain the protocol spec and conformance fixtures itself (Phase A deliverable) rather than borrowing an existing one. Accepted: no suitable open worker-lease protocol with envelope and evidence semantics exists.

## Migration impact

All v1 references to "Hermes-compatible" now read "WWP-conformant." No other change; v1's worker lifecycle semantics map 1:1 onto WWP messages.
