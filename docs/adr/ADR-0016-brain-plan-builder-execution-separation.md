# ADR-0016 — Separate Brain planning from builder execution authority

**Status:** Accepted
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H2 implementation authorization

## Context

The current documents use Brain, planner, worker, and Hermes inconsistently. A provider model is sometimes conflated with a planning role, while an execution runtime can appear to own plan or mission outcomes. That ambiguity creates self-approval and authority-leak risk.

## Decision

Define five independent contracts:

1. The **Brain Agent** is a Core-governed planner, architect, risk analyst, and replanner using replaceable brain profiles. It proposes structured specifications and `PlanProposal` records and performs no effects.
2. **Plan qualification** is a separate Core-governed pipeline: `Brain Agent → PlanProposal → deterministic compiler/normalization → candidate DeliveryPlan → structural validation → initial PlanAssessment → independent review when policy requires → finalized Ready PlanAssessment → authenticated PlanDecision → approved DeliveryPlan`. Compilation may normalize representation but cannot invent strategy. The initial assessment covers specification/outcome coverage, acceptance/evidence, DAG, architecture, proportionality, risk, budget/WIP, rollback/recovery, assumptions/uncertainty, alternatives, blockers, and readiness. When policy requires reviews, it remains `ReviewRequired`; reviewers create separate immutable records, and Core finalizes a new `Ready` assessment version bound to exact reviewer identity/independence and record IDs/versions/hashes.
3. **Hermes** is the Engineering Intelligence Runtime and Supervisor. It turns approved PhasePlans into bounded TaskPackets, routes execution, and proposes effects and completion; it owns no durable governance truth.
4. A **builder model/profile** consumes one TaskPacket and returns typed actions, artifacts, and evidence; it cannot redefine scope, policy, criteria, or approval.
5. **Core and tool boundaries** remain separate: Core alone commits state and authorization; tool boundaries alone perform Core-authorized effects.

For low-risk plans, deterministic validation plus an authenticated decision by an authorized user may satisfy policy. Medium/high risk requires the independent architecture, quality and security review set named by policy. The proposal producer never approves its own plan and cannot be the sole acceptance-critical reviewer. Model voting can expose disagreement but is never decision authority. Core alone authenticates and records the immutable `PlanDecision`. Multiple plans are not mandatory; alternatives are requested only when risk, uncertainty, a material architectural choice or failed assessment makes comparison proportionate. A builder or Hermes result never completes a phase or mission without Core validation and the required human authority.

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

H2 evidence must prove schema-valid PlanProposal production; deterministic and reproducible candidate compilation; structural rejection of invalid candidates; complete PlanAssessment fields; risk-tier review selection and reviewer-independence enforcement; separate authenticated Core-recorded PlanDecision bound to the exact policy version, risk-tier decision, and every required review record ID/version/hash; rejection of stale, missing, forged, or substituted bindings; packet derivation from the exact approved version; denial of scope/criteria mutation; model-vote non-authority; alternatives triggered only by recorded risk/uncertainty policy; no direct model tool access; and safe brain or builder profile replacement without domain-contract changes. Qualification evaluation must retain `EvaluationCase`, `TreatmentArm`, `EvaluationRun`, frozen `RunManifest`, `MetricObservation`, `ProtocolDeviation`, and `EvaluationResult` provenance so the gate can be reproduced.

Draft PR #1's provider gateway, worker protocol, staged approval, and Core-owned ledger are candidate foundations. Its branch-local Hermes V0 remains a narrow worker and is not evidence that the full authority separation or Supervisor runtime exists. This ADR does not ratify or authorize the candidate baseline.

## Acceptance disposition (2026-07-18)

Accepted by founder and Mastermind with the following V0 scope. This
acceptance records architecture only and does not authorize implementation.

The canonical V0 pipeline is: `PlanProposal` → deterministic compilation →
initial deterministic `PlanAssessment` → required Consulting assessment →
finalized Ready `PlanAssessment` → authenticated `PlanDecision` → versioned
`DeliveryPlan`.

1. Initial `PlanAssessment` fields are deterministic-Core-owned; the initial
   assessment is deterministic only.
2. Model-generated annotations may be attached as advisory content only and
   never populate assessment fields.
3. Consulting review is required for every Native Delivery V0 mission.
4. The producer cannot provide the only acceptance-critical review.
5. Role, session, and context independence is mandatory for the reviewing
   invocation.
6. Provider/model-family independence is preferred, not mandatory, in V0.
7. A same-provider fresh invocation is permitted only with an explicit
   `ProviderNotIndependent` disclosure carried into the completion proposal.
8. When structural independence cannot be established, the honest outcome is
   `IndependentReviewUnavailable`; it is never silently downgraded.
9. Review-independence assurance levels are: `FullyIndependent`,
   `RoleIndependentOnly`, `NotIndependent` (disclosed), `Unavailable`.
10. `shared_context_detected` is structurally derived from invocation
    identity and ContextPack lineage hashes, never self-attested.
11. Historical `PlanApproved` events remain readable and hash-verifiable and
    acquire no new authority retroactively.
12. The legacy single-approval command cannot create new production missions
    after reconciliation activation; it is closed as an operational bypass
    while its historical facts remain valid evidence.
