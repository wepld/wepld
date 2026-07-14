# ADR-0016 — Separate Brain planning from builder execution authority

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H2 implementation authorization

## Context

The current documents use Brain, planner, worker, and Hermes inconsistently. A provider model is sometimes conflated with a planning role, while an execution runtime can appear to own plan or mission outcomes. That ambiguity creates self-approval and authority-leak risk.

## Decision

Define four independent contracts:

1. The **Brain Agent** is a Core-governed planner, architect, risk analyst, and replanner using replaceable brain profiles. It proposes structured specifications and plans and performs no effects.
2. **Hermes** is the Engineering Intelligence Runtime and Supervisor. It turns approved PhasePlans into bounded TaskPackets, routes execution, and proposes effects and completion; it owns no durable governance truth.
3. A **builder model/profile** consumes one TaskPacket and returns typed actions, artifacts, and evidence; it cannot redefine scope, policy, criteria, or approval.
4. **Core and tool boundaries** remain separate: Core alone commits state and authorization; tool boundaries alone perform Core-authorized effects.

The Brain Agent never approves its own plan. A builder or Hermes result never completes a phase or mission without Core validation and the required human authority.

## Reason

Replaceable reasoning and runtime implementations are safe only when authority remains invariant under replacement. Clear separation also permits different brain and builder profiles while preserving the same approved contract and gates.

## Benefits

- No model or provider gains ambient authority.
- Independent review and model/profile replacement remain possible.
- Routing changes do not alter mission semantics.
- The worker protocol remains a replaceable runtime boundary.

## Trade-offs

- More structured handoffs and validation boundaries.
- Planners cannot “just fix” execution.
- Even small governed tasks retain plan, packet, and effect separation.

## Migration

H2 evidence must prove separate authenticated plan approval; packet derivation from approved versions; denial of scope/criteria mutation; no direct model tool access; and safe brain or builder profile replacement without domain-contract changes.

Draft PR #1's provider gateway, worker protocol, staged approval, and Core-owned ledger are candidate foundations. Its branch-local Hermes V0 remains a narrow worker and is not evidence that the full authority separation or Supervisor runtime exists. This ADR does not ratify or authorize the candidate baseline.
