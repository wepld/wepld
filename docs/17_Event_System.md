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
| Authority | governing policy/specification/outcome/plan/phase/task versions and approval/capability references where applicable |
| Time/order | authoritative local ledger sequence plus UTC occurrence and record times |
| Actor/source | authenticated principal, Core component, Hermes/worker session, tool boundary, or integration |
| Causation/correlation | parent command/event, mission/phase/task/attempt/evaluation trace IDs |
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
| Delivery / Phase planning | `DeliveryPlanProposed`, `DeliveryPlanValidated`, `DeliveryPlanApproved`, `PhasePlanApproved`, `PhaseBecameReady`, `PhaseActivated`, `PhaseReturned`, `PhaseClosed` |
| Task / Kanban / WIP | `TaskPacketIssued`, `TaskBecameReady`, `TaskEnteredReview`, `TaskEnteredVerification`, `TaskDone`, `WipAdmissionDenied`, `WipLimitChanged` |
| Change / Decision / Risk | `ChangeRequestSubmitted`, `SpecificationChangeApproved`, `PlanChangeApproved`, `DecisionRequested`, `DecisionRecorded`, `RiskEscalated`, `AssumptionInvalidated` |
| Hermes / Brain / Skill | `HermesSessionStarted`, `BrainPlanProposed`, `SkillRouteSelected`, `SkillInvocationCompleted`, `ContextPackCompiled`, `LspEvidenceObserved`, `LoopNoProgressDetected` |
| Worker / Subagent | `TaskLeased`, `LeaseExpired`, `AttemptStarted`, `SubagentFindingReported`, `AttemptResultReported`, `AttemptBecameUncertain` |
| Hook | `HookInvocationStarted`, `HookValidationReported`, `HookBlockedProgress`, `HookEffectProposed`, `HookInvocationFailed` |
| Tool / Effect / Artifact | `ToolActionProposed`, `EffectDenied`, `EffectApprovalRequested`, `EffectIntentRecorded`, `EffectStarted`, `EffectResultObserved`, `EffectBecameUncertain`, `ArtifactRecorded` |
| Quality / Security | `CheckCompleted`, `EvidenceBundleSubmitted`, `EvidenceBundleValidated`, `GatePassed`, `GateBlocked`, `FindingOpened`, `FindingDispositioned` |
| Completion | `CompletionProposed`, `CompletionReturned`, `CompletionDeferred`, `CompletionAccepted`, `MissionCancelled` |
| Memory / Retrospective | `MemoryCandidateSubmitted`, `MemoryCandidateRejected`, `EngineeringMemoryConsolidated`, `MemoryContradictionDetected`, `MemorySuperseded`, `RetrospectiveClosed` |
| Messenger / Integration | `InboundIntentReceived`, `ReportQueued`, `NotificationDelivered`, `ChannelFailed` |
| Registry / Release | `PackageInstalled`, `PackageActivated`, `PackageQuarantined`, `PackageRevoked`, `ReleaseProposed`, `ReleaseRolledBack` |
| Evaluation / Ablation | `EvaluationRunRegistered`, `AblationCellStarted`, `EvaluationMetricRecorded`, `OutcomeEquivalenceAssessed`, `ProfileCertificationChanged` |

Event names describe facts. A proposal or intent event does not claim execution; an observed effect does not claim evidence validation; `TaskDone` or `PhaseClosed` does not claim mission acceptance. `CompletionAccepted` is emitted only after Core records an authorized `CompletionDecision` against the exact validated proposal.

## Governed artifact and transition semantics

Approval events bind exact artifact versions, decision authority, policy version, validation result, and any expiry/conditions. Approved artifacts are never edited in place; replacement approval and supersession are separate events.

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

Immutable event meaning never changes. Additive schema evolution is preferred; semantic changes require a new type/version. Upcasters preserve original identity and are used only when transformation cannot misstate authority, evidence, or effect history. Deprecated consumers receive an explicit resync/migration signal.

## Security, privacy, and retention

Events contain the minimum audit fact. Sensitive bodies reside in access-controlled artifacts; audience-specific redacted projections preserve event identity and permitted proof. Event access is authorized like any other data. Tombstone/redaction/supersession events can prevent ordinary body retrieval and downstream indexing while preserving policy-allowed audit identity. Hashes detect alteration but do not replace OS security, backups, signing, or access control.

## Observability relationship

Domain events explain governed state. Logs explain component diagnostics. Traces follow requests. Metrics summarize rates/resources. Loop iteration records explain hypotheses and confidence change; evaluation records compare controlled runs. These may share correlation IDs but are not interchangeable, and model chain-of-thought is neither required nor treated as evidence. OpenTelemetry-style data may be exported; Core’s ledger remains the source for mission workflow truth.

## Acceptance criteria

- A user can trace visible state through command/proposal, authority, policy, capability, approval, effect probe, evidence, and Core transition.
- No worker, tool, hook, provider, or integration can append a domain event directly.
- Reprocessing an event cannot repeat a protected effect.
- Spec vs plan changes, phase/Kanban flow, WIP denial, evidence validation, completion, memory consolidation, and evaluation are typed facts.
- Provider/channel/boundary failure creates an explicit degraded or uncertain state.
- An audit export can filter by mission, artifact version, phase, task, actor, effect, decision, evidence, and time without exposing unauthorized bodies.

See [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [03_System_Architecture.md](03_System_Architecture.md), [13_Mission_Control.md](13_Mission_Control.md), [16_Data_Model.md](16_Data_Model.md), and [28_Release_Strategy.md](28_Release_Strategy.md). Proposed ADRs 0015–0024 govern these schemas and remain non-authorizing while Proposed.
