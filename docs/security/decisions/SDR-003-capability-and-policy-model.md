# SDR-003 — Capability and Policy Model

- **Status:** Adopted (founder decision, 2026-07-19)
- **Prerequisites:** SDR-001, SDR-002

## Context

Denying ambient authority is only meaningful if a concrete authorization
model replaces it. Humans, agents, services, integrations, and devices
all need scoped authority; agents especially must be usable without
ever holding standing power.

## Decision

Adopt `../CAPABILITY_MODEL.md`: explicit, scoped, expiring, revocable
capabilities with the canonical schema (actor, identity type, action,
resource, scope, constraints, classification ceiling, budget, expiry,
binding, approval reference, revocation, audit level); the fixed policy
precedence — emergency controls, then organization policy, explicit
deny, resource policy, approval requirements, temporary capabilities,
role grants — with nothing overriding an active emergency freeze;
deterministic, explainable evaluation; per-operation checks; Work
Contracts (or approved templates) for every effectful agent run; and
ledger evidence for issuance, use, denial, expiry, and revocation.
`secret.reveal` is absent from the standard permission model.

## Rationale

Capabilities make authority enumerable, auditable, and revocable;
deterministic precedence makes every decision explainable to the user;
the freeze-wins rule gives incidents a reliable stop button.

## Consequences

Every future permission catalog, role design, and agent integration
builds on this schema; the CoWork authorization model (deferred to
S0-B) must specialize this model rather than invent another.

## Legal-review boundary

None.

## Supersession rules

Precedence or schema changes require a successor SDR linking here;
additive fields may be recorded as dated amendments.
