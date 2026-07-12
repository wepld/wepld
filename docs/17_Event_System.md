# 17 — Event System

## Purpose

The event system supplies durable workflow history, auditability, projections, recovery, and causal explanation. It records domain facts after valid commands—not raw chat, source-file contents, every log line, or unvalidated model thoughts. Event sourcing is applied deliberately to control-plane behavior, not as a universal storage religion.

## Event envelope

Every published event includes:

| Field | Requirement |
| --- | --- |
| Event identity | unique immutable identifier and event type |
| Aggregate | type, ID, and expected/current revision |
| Time | authoritative local sequence plus UTC occurrence/record time |
| Actor | human principal, worker/session, system component, or integration identity |
| Causation / correlation | parent command/event and mission/run trace IDs |
| Schema | event schema version and producer version |
| Payload | typed minimal fact; large/sensitive bodies use artifact reference/hash |
| Integrity | payload hash and storage integrity metadata |
| Delivery | idempotency key, classification, retention, and redaction policy |

The local sequence is the deterministic order for a single Core ledger. Timestamp order is useful for display but insufficient for causality. Cross-device synchronization is a future protocol and must not assume global clock ordering.

## Event families

| Family | Examples |
| --- | --- |
| Mission / Portfolio | MissionCreated, ScopeChanged, PriorityChanged, MissionPaused, MissionCompleted |
| Planning / Orchestration | PlanProposed, PlanApproved, TaskReady, TaskLeased, LeaseExpired, TaskRetried, RunCancelled |
| Policy / Decision | PolicyEvaluated, ActionDenied, ApprovalRequested, DecisionRecorded, ExceptionExpired |
| Worker / Brain | WorkerRegistered, AttemptStarted, BrainInvoked, BrainDegraded, AttemptSucceeded, AttemptFailed |
| Tool / Artifact | ToolActionRequested, ToolActionStarted, ToolActionCompleted, ArtifactRecorded, WorktreeCreated |
| Quality / Security | CheckCompleted, GatePassed, GateBlocked, FindingOpened, FindingDispositioned |
| Knowledge | KnowledgeIngestionQueued, ClaimPublished, DecisionLinked, RecordSuperseded |
| Messenger / Integration | ReportQueued, NotificationDelivered, InboundIntentReceived, ChannelFailed |
| Registry / Release | PackageInstalled, PackageRevoked, ReleaseProposed, ReleaseRolledBack |

Event names describe facts in past tense. Requests and commands are distinct records; a `ToolActionRequested` event does not claim the tool action has occurred.

## Command-to-effect flow

~~~mermaid
sequenceDiagram
  participant Caller
  participant Core
  participant Policy
  participant Effect as Worker / Tool
  participant Ledger
  Caller->>Core: idempotent command
  Core->>Policy: classify and evaluate
  Policy-->>Core: permit / deny / approval needed
  Core->>Ledger: command outcome + intent event
  Core->>Effect: leased action with capability
  Effect->>Ledger: started / evidence / completed or failed
  Ledger-->>Core: projection update
~~~

## Delivery and idempotency

Effects are at-least-once, not magically exactly-once. A command and tool action have idempotency keys. Consumers store their last processed sequence/key and tolerate replay. Non-idempotent actions require a recovery probe or human decision after a worker/daemon crash. The outbox pattern atomically records a committed domain event before delivery to projections/adapters. A delivery failure is retried and observable; it does not roll back an already-completed real-world effect.

## Projections and replay

Mission Control, Timeline, Executive reports, search indexes, and notification queues are projections. They can rebuild from retained events plus referenced artifacts. Projection code is versioned and replayed in an isolated environment before upgrades. An event’s immutable semantics never change; new fields are additive or represented by a new event type/version with an upcaster only when safe.

## Security, privacy, and retention

Events contain the minimum needed for audit. Sensitive bodies are referenced through access-controlled artifacts, and redacted projections are generated per audience. Event access is authorized like any other data. Retention follows classification; a deletion/tombstone event prevents retrieval and downstream indexing while preserving permitted audit proof. Event hashes help detect accidental or malicious alteration, but do not substitute for filesystem/OS security or backups.

## Observability relationship

Domain events explain business/workflow state. Logs explain component diagnostics. Traces follow a request across components. Metrics summarize rates and resources. They share correlation IDs but are not interchangeable. OpenTelemetry-style traces/metrics/logs can be exported; the event ledger remains the source for mission truth.

## Acceptance criteria

- A user can trace a visible task status to command, policy outcome, lease, evidence, and state transition.
- Reprocessing a delivered event does not repeat a protected tool effect.
- A provider/channel failure creates a durable event and visible degraded state.
- An audit export can be filtered by mission, actor, artifact, decision, and time without exposing unauthorized bodies.

See also: [03_System_Architecture.md](03_System_Architecture.md), [13_Mission_Control.md](13_Mission_Control.md), [16_Data_Model.md](16_Data_Model.md), and [28_Release_Strategy.md](28_Release_Strategy.md).

