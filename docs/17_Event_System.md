# 17 — Event System

## Purpose and write authority

The event system supplies durable workflow history, auditability, projections, recovery, and causal explanation. It records validated domain facts—not raw chat, source contents, every log line, hidden chain-of-thought, or an executor’s unsupported assertion. Event sourcing is deliberate for control-plane behavior, not universal storage.

Core is the only ledger writer. Brain Agent, Hermes, builders, subagents, tool boundaries, hooks, providers, integrations, and validators return typed proposals or observations through authenticated ports. Core validates identity, schema, capability, governing versions, expected revision, and evidence before recording a fact. A boundary never appends an event or advances a projection directly.

## Event envelope

Every recorded event includes:

| Field | Requirement |
| --- | --- |
| Identity | unique immutable event ID and past-tense event type |
| Aggregate | type, ID, prior revision, committed result revision |
| Authority | governing policy/specification/outcome/plan/phase/task versions plus SOPGraph/subscription/approval/capability references where applicable |
| Time/order | authoritative local ledger sequence plus UTC occurrence and record times |
| Actor/source | authenticated principal, Core component, Hermes/worker session, tool boundary, or integration |
| Causation/correlation | parent command/event, mission/phase/task/attempt/exploration/race/evaluation trace IDs |
| Schema | event schema and producer/adapter versions |
| Payload | minimal typed fact; large/sensitive bodies use authorized artifact/hash references |
| Integrity | payload hash, artifact identities, and storage-integrity metadata |
| Delivery/data | idempotency key, classification, retention, redaction, and audience policy |

Expected revision belongs to the command. The event records the revision transition Core actually committed. Local sequence is deterministic for one Core ledger; timestamps aid display but never establish causality. Future synchronization must not assume a global clock.

## Event families

| Family | Representative facts |
| --- | --- |
| Charter / Specification | `MissionCharterSubmitted`, `ClarificationRequested`, `EngineeringSpecificationReadyForReview`, `EngineeringSpecificationApproved`, `SpecificationSuperseded` |
| Outcome / Evidence binding | `OutcomeContractValidated`, `OutcomeContractApproved`, `EvidenceRequirementBound`, `VerificationBindingChanged` |
| Plan qualification / Phase planning | `PlanProposalSubmitted`, `PlanCandidateCompiled`, `PlanStructuralValidationFailed`, `PlanAssessmentRecorded`, `PlanIndependentReviewRecorded`, `PlanDecisionRecorded`, `DeliveryPlanApproved`, `PhasePlanApproved`, `PhaseBecameReady`, `PhaseActivated`, `PhaseReturned`, `PhaseClosed` |
| Task / Kanban / WIP | `TaskPacketIssued`, `TaskBecameReady`, `TaskEnteredReview`, `TaskEnteredVerification`, `TaskDone`, `WipAdmissionDenied`, `WipLimitChanged` |
| SOP / Role projection | `SOPGraphCompiled`, `SOPGraphValidationFailed`, `SOPGraphActivated`, `SOPGraphRevoked`, `RoleNodeActivated`, `InputSubscriptionAuthorized`, `InputSubscriptionRevoked`, `RoleInputProjected`, `RoleOutputSubmitted`, `RoleOutputRejected`, `SOPControlEdgeSatisfied`, `SOPControlEdgeTriggered` |
| Exploration / Compaction | `MissionExplorationBranchAuthorized`, `ExplorationFindingsSubmitted`, `ExplorationContributionAccepted`, `ExplorationContributionRejected`, `CompactionRecorded`, `CompactionRejected`, `AuthorityRehydrated` |
| Change / Decision / Risk | `ChangeRequestSubmitted`, `SpecificationChangeApproved`, `PlanChangeApproved`, `DecisionRequested`, `DecisionRecorded`, `RiskEscalated`, `AssumptionInvalidated` |
| Hermes / Brain / Skill | `HermesSessionStarted`, `BrainPlanProposalProduced`, `SkillRouteSelected`, `SkillInvocationCompleted`, `ContextPackCompiled`, `LspEvidenceObserved`, `LoopNoProgressDetected` |
| Worker / Subagent | `TaskLeased`, `LeaseExpired`, `AttemptStarted`, `SubagentFindingReported`, `AttemptResultReported`, `AttemptBecameUncertain` |
| Hook | `HookInvocationStarted`, `HookValidationReported`, `HookBlockedProgress`, `HookEffectProposed`, `HookInvocationFailed` |
| Tool / Effect / Artifact | `ToolCatalogManifestProjected`, `ToolCatalogManifestRevoked`, `ToolActionProposed`, `EffectDenied`, `EffectApprovalRequested`, `EffectIntentRecorded`, `EffectStarted`, `BoundedToolResultReported`, `ToolOutputArtifactRecorded`, `SandboxFailureReported`, `EffectResultObserved`, `EffectBecameUncertain`, `ArtifactRecorded` |
| Quality / Security / Visual | `CheckCompleted`, `VisualEvidenceCaptured`, `VisualComparisonSubmitted`, `VisualComparisonValidated`, `VisualComparisonRejected`, `EvidenceBundleSubmitted`, `EvidenceBundleValidated`, `GatePassed`, `GateBlocked`, `FindingOpened`, `FindingDispositioned` |
| Completion | `CompletionProposed`, `CompletionReturned`, `CompletionDeferred`, `CompletionAccepted`, `MissionCancelled` |
| Committee (advisory) | `CommitteeRequested`, `CommitteePackFrozen`, `CommitteeProjectionDelivered`, `MemberOpinionFrozen`, `MemberCritiqueFrozen`, `EvidenceClarificationRequested`, `CommitteeSynthesisProduced`, `MinorityReportPreserved`, `CommitteeDispositionRecorded`, `CommitteeBudgetExhausted`, `CommitteeMemberFailed`, `CommitteeCancelled` — facts about deliberation; none asserts approval, acceptance, or plan change ([36_Engineering_Committee.md](36_Engineering_Committee.md)) |
| Memory / Retrospective | `MemoryCandidateSubmitted`, `MemoryCandidateRejected`, `EngineeringMemoryConsolidated`, `MemoryContradictionDetected`, `MemorySuperseded`, `RetrospectiveClosed` |
| Messenger / Integration | `InboundIntentReceived`, `ReportQueued`, `NotificationDelivered`, `ChannelFailed` |
| Registry / Release | `PackageInstalled`, `PackageActivated`, `PackageQuarantined`, `PackageRevoked`, `ReleaseProposed`, `ReleaseRolledBack` |
| Evaluation / Ablation | `EvaluationCaseApproved`, `ControlledMultiRouteRaceAuthorized`, `RaceRouteStarted`, `RaceRouteResultReported`, `RaceRouteCancelled`, `RaceCandidateSelected`, `ControlledMultiRouteRaceAssessed`, `TreatmentArmFrozen`, `EvaluationRunRegistered`, `RunManifestFrozen`, `EvaluationRunStarted`, `MetricObservationRecorded`, `MetricObservationValidated`, `ProtocolDeviationRecorded`, `ProtocolDeviationDispositioned`, `EvaluationRunCompleted`, `EvaluationRunFailed`, `EvaluationRunAborted`, `EvaluationRunAssessed`, `EvaluationResultFinalized`, `OutcomeEquivalenceAssessed`, `ProfileCertificationChanged` |

Event names describe facts. A proposal or intent event does not claim execution; an observed effect does not claim evidence validation; `TaskDone` or `PhaseClosed` does not claim mission acceptance. `CompletionAccepted` is emitted only after Core records an authorized `CompletionDecision` against the exact validated proposal.

## Governed artifact and transition semantics

Approval events bind exact artifact versions, decision authority, policy version, validation result, and any expiry/conditions. Approved artifacts are never edited in place; replacement approval and supersession are separate events.

Plan qualification facts preserve the complete chain `PlanProposal → candidate DeliveryPlan → initial PlanAssessment → exact independent reviews when required → finalized Ready PlanAssessment → PlanDecision`. `PlanCandidateCompiled` identifies the deterministic compiler/schema version and input/output hashes. The initial `PlanAssessmentRecorded` may enter `ReviewRequired`; each `PlanIndependentReviewRecorded` is a separate immutable fact; a later versioned `PlanAssessmentRecorded` finalizes `Ready` only after Core verifies and binds the exact required records. Assessment and review facts bind coverage, evidence, DAG, architecture, proportionality, risk, budget/WIP, recovery, assumptions/uncertainty, alternatives, blockers, readiness, reviewer identity/role and independence. The final assessment and `PlanDecisionRecorded` bind the exact policy/risk-tier version and each required independent-review record ID/version/hash. Core records the decision only after authenticating the named authority and verifying those exact reviews. The proposal producer cannot approve or be the sole acceptance-critical reviewer; model votes can be recorded as findings but never as authority.

Evaluation facts preserve `EvaluationCase → TreatmentArm + RunManifest → EvaluationRun → MetricObservation/ProtocolDeviation → EvaluationResult`. A frozen manifest binds exact fixture/source and repository hashes, governing artifact versions, profile/provider/model/adapter/settings, prompt/context/skill/tool versions, environment, seed where supported, budgets, and manifest creation/freeze/allocation timestamps. `EvaluationRunStarted`/terminal facts record observed run start/end. A result cannot become final while a required observation is unvalidated or a protocol deviation lacks disposition. Baseline and regression comparisons cite the exact eligible run and result versions.

SOP facts preserve `exact approved DeliveryPlan/PhasePlan/TaskPacket → deterministic SOPGraph → RoleNode/contracts/edges → role projection/output`. `SOPGraphCompiled` binds exact parent hashes, compiler/schema version and canonical graph hash; the same inputs and compiler must reproduce it. Core alone records activation, subscription state, role input batches and control-edge state. `RoleInputProjected` cites the authorized `InputSubscription`, cursor/range, source event/artifact identities and hashes, redactions and omissions. No event asserted by a role can self-subscribe it, widen its projection, create peer broadcast/free chat, or observe an ambient shared environment.

Exploration and compaction facts preserve context without inventing authority. A `MissionExplorationBranch` event chain binds parent, objective, ContextPack hash, read-only permissions, budget, findings/evidence and explicit accepted-or-rejected contribution. `AuthorityRehydrated` records the current policy/specification/outcome/plan/phase/task/decision/capability/SOP versions restored after a `CompactionRecord`, its source hash and declared omissions; compaction text alone cannot authorize a transition.

Tool facts distinguish discovery, execution, output and failure. `ToolCatalogManifestProjected` is a Core-derived view of active capabilities and ActionContract bounds, not a capability. `BoundedToolResultReported` states truncation and output-artifact refs; `ToolOutputArtifactRecorded` records immutable large/binary content; `SandboxFailureReported` preserves denial/violation/timeout/crash/resource exhaustion, possible partial effects, probe and uncertainty. None claims evidence satisfaction.

Controlled-race events bind fixed task/outcome/context, isolated routes, budgets, cancellation and the predeclared selection metric to the evaluation spine. `RaceCandidateSelected` is a comparison fact, not model voting, plan/task approval or effect authority. Visual events bind build/commit, UI state, display/capture conditions, exact artifacts, comparison criterion and reviewer independence; a capture or diff score does not emit `GatePassed` by itself.

A Specification Change Request changes WHAT and produces a new Engineering Specification/Outcome Contract version. A Plan Change Request changes only HOW and replaces Delivery Plan/Phase Plan versions. Core rejects a change transition when its declared kind does not match impact.

Phase and Kanban transitions include previous/new state, reason, expected/committed revision, WIP counter snapshot, governing Task Packet or Phase Plan, gate/evidence references, and actor. Hermes may request a transition, but only Core can record it.

## Command-to-effect flow

~~~mermaid
sequenceDiagram
  participant Caller
  participant Core
  participant Boundary as Tool / Provider Boundary
  participant Projection
  Caller->>Core: typed idempotent proposal
  Core->>Core: classify + policy + capability + approval check
  Core->>Core: record denial or durable effect intent
  Core->>Boundary: dispatch exact intent + active capability
  Boundary->>Boundary: execute + probe postcondition
  Boundary->>Core: typed observed result + evidence references
  Core->>Core: validate and record result / uncertain state
  Core-->>Projection: committed event stream
~~~

This preserves the Effect Firewall order: propose → classify → policy → capability → approval → durable intent → execute → probe → evidence. A capability that needs approval is not usable before the approval is recorded. No event delivery to a boundary is itself authority to perform a different effect.

## Delivery, idempotency, and uncertain effects

Delivery is at-least-once, never “magically exactly once.” Commands, invocations, and effects carry idempotency keys. Consumers retain cursor/key state and tolerate replay. The outbox atomically records a committed Core fact before delivery to projections or adapters.

Non-idempotent effects require a postcondition probe. After crash, timeout, lost heartbeat, or delivery ambiguity, Core records `Uncertain`, blocks unsafe replay, and selects recovery probe, authorized retry, replanning, or human decision. A delivery failure is observable and does not roll back an already-performed real-world effect.

## Projections, replay, and schema evolution

Mission Control, Spec Review, Plan Review, Kanban, Timeline, reports, indexes, notification queues, and evaluation dashboards are projections. They rebuild from retained events, versioned snapshots, and authorized referenced artifacts. Projection code is versioned and dry-run/replayed before upgrade. If retention prevents full reconstruction, the projection exposes the gap rather than inventing state.

`RoleInputProjection` is a security boundary, not a client-maintained view. Core materializes it only from an active authorized `InputSubscription`, records exact source refs and redactions, and stops delivery on revocation or stale parent/capability state. Replay cannot exceed the authorized cursor/range, and a role cannot convert a projection cursor into ledger or artifact-store query access.

Immutable event meaning never changes. Additive schema evolution is preferred; semantic changes require a new type/version. Upcasters preserve original identity and are used only when transformation cannot misstate authority, evidence, or effect history. Deprecated consumers receive an explicit resync/migration signal.

## Security, privacy, and retention

Events contain the minimum audit fact. Sensitive bodies reside in access-controlled artifacts; audience-specific redacted projections preserve event identity and permitted proof. Event access is authorized like any other data. Tombstone/redaction/supersession events can prevent ordinary body retrieval and downstream indexing while preserving policy-allowed audit identity. Hashes detect alteration but do not replace OS security, backups, signing, or access control.

## Observability relationship

Domain events explain governed state. Logs explain component diagnostics. Traces follow requests. Metrics summarize rates/resources. Loop iteration records explain hypotheses and confidence change; evaluation records compare controlled runs. These may share correlation IDs but are not interchangeable, and model chain-of-thought is neither required nor treated as evidence. OpenTelemetry-style data may be exported; Core’s ledger remains the source for mission workflow truth.

## Acceptance criteria

- A user can trace visible state through command/proposal, authority, policy, capability, approval, effect probe, evidence, and Core transition.
- No worker, tool, hook, provider, or integration can append a domain event directly.
- Identical approved parent hashes plus SOP compiler/schema reproduce the same graph; stale/forged parents fail, and only Core can activate the graph, authorize subscriptions, project role input, or evaluate control edges.
- No role can self-subscribe, observe shared ambient state, broadcast/free-chat with peers, or treat a peer output as an authoritative event.
- Reprocessing an event cannot repeat a protected effect.
- Exploration contributions are explicitly accepted or rejected; compacted sessions record omissions and rehydrate current authority before progress.
- Tool catalog projection, bounded result/output, sandbox failure/uncertainty, visual capture/comparison, and controlled multi-route race selection are distinct typed facts with evaluation/evidence links.
- Spec vs plan changes, plan qualification/decision, phase/Kanban flow, WIP denial, evidence validation, completion, memory consolidation, and evaluation are typed facts.
- Provider/channel/boundary failure creates an explicit degraded or uncertain state.
- An audit export can filter by mission, artifact version, phase, task, actor, effect, decision, evidence, and time without exposing unauthorized bodies.

See [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [03_System_Architecture.md](03_System_Architecture.md), [13_Mission_Control.md](13_Mission_Control.md), [16_Data_Model.md](16_Data_Model.md), and [28_Release_Strategy.md](28_Release_Strategy.md). Proposed ADRs 0015–0025 govern these schemas and remain non-authorizing while Proposed.
