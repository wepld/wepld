# ADR-0026 — Governed Engineering Committee deliberation protocol

**Status:** Proposed
**Date:** 2026-07-18
**Owner:** Architecture Review Board
**Review:** Before any Committee V0 implementation authorization

## Context

Single-reviewer assessment concentrates model-specific blind spots, and naive
multi-model "voting" imports a worse failure: agreement among models being read
as engineering truth. WePLD already rejects model votes, score rankings, and
appearance preferences as authority (ADR-0023, ADR-0025). The founder has
approved an Engineering Committee product direction: three or more
independently reasoning members — Hermes profiles, local models, supported
remote APIs, or enterprise gateways — reviewing a frozen subject in bounded
rounds and producing one evidence-linked advisory report.

## Decision

Adopt the Governed Multi-Model Deliberation Protocol in
[36_Engineering_Committee.md](../36_Engineering_Committee.md) as Proposed
architecture. The Committee is advisory and sits outside the authority chain
(Governance Policy → Approved Specification → Outcome Contract → Approved
DeliveryPlan → Phase Plan → TaskPacket → Tool Action). Committee agreement is
never engineering truth: no vote approves a plan, accepts a mission,
authorizes an effect, grants capability, closes a security finding, promotes a
Skill, or overrides deterministic evidence. The only durable-change path is
`CommitteeReport` → Mastermind review → optional `PlanChangeProposal` (a
Committee-originated `PlanProposal`) → deterministic structural validation →
independent Consulting assessment where policy requires → authenticated
`PlanDecision` → new `DeliveryPlan` version, with exact history preserved.

Sessions are finite: one frozen, hashed `CommitteePack`; per-member hashed
projections under explicit data-egress policy; an independent first opinion
round; bounded challenge rounds; Chair synthesis that preserves verbatim
minority reports; and exactly one durable disposition (`ReportReady`,
`QuorumNotMet`, `MoreEvidenceRequired`, `MemberFailure`, `BudgetExhausted`,
`DeadlineExceeded`, `PolicyBlocked`, `Cancelled`, `NonConvergent`). Members
bind provider, model/profile, deployment, perspective, invocation identity,
context scope, skills, capability boundary, egress policy, cost limit,
timeout, provenance, and evaluation status. Consumer chat subscriptions are
not programmatic access: WePLD uses supported APIs, local runtimes, enterprise
gateways, or official integrations only, and must not capture browser cookies,
automate consumer chat sessions, or circumvent provider usage restrictions.
Credentials stay behind the Effect Firewall and Secret Manager.

Admission is falsifiable: the compared arms, metrics, and rejection criteria
of [37_Committee_Evaluation_Protocol.md](../37_Committee_Evaluation_Protocol.md)
must run on the ADR-0024 evaluation spine before any V0 authorization. V0, if
authorized, is user-triggered only, exactly three members, one challenge
round, and no automatic plan mutation.

## Reason

Deliberation can surface defects a single reviewer misses, but only if
independence is structural (frozen first round), disagreement is preserved
(verbatim minority reports), budgets are hard, and authority is untouched.
Encoding those properties as protocol — rather than as model behavior we hope
for — keeps multi-model review inside WePLD's existing authority, evidence,
and provider-neutrality invariants.

## Benefits

- Multi-perspective review without a new authority surface.
- Provider-neutral membership: local-only, hybrid, or remote Committees.
- Dissent preserved as first-class evidence; imitation measurable.
- Deterministic budgets and durable failure dispositions.
- A falsifiable admission path with explicit rejection criteria.

## Trade-offs

- Real cost per session; value must be proven per the evaluation protocol.
- Diversity reporting can mislead if read as independence; it is advisory.
- Bounded rounds may end `NonConvergent`; that is accepted as honest output.
- Additional typed artifacts and projections to maintain.

## Migration

No implementation is authorized by this record. Gate evidence: acceptance of
this ADR after independent review; EC-A1–EC-A3 results per document 37 with no
rejection criterion fired (earliest gate H6, after bounded-subagent and
structured-handoff foundations under ADR-0021); Critical Review Board and
automatic triggers additionally require EC-A5–EC-A8 evidence and policy
admission. Compatibility with Draft PR #1 is preserved: nothing here modifies
the Build Feature slice, its governance, or its security boundaries; the
Committee reads evidence those systems produce and adds none of its own
authority over them.
