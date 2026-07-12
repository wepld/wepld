# 16 — Data Model

## Purpose and scope

This document defines the canonical domain records that support a durable, explainable engineering organization. It is normative for boundary semantics, not a physical database schema. The V1 implementation may use relational tables and document/artifact stores, but no adapter may alter the meanings, ownership, or identities below.

## Identity and common metadata

Every domain record has a globally unique opaque identifier, a type, creation/update timestamps, owner/project scope, schema version, classification, and optimistic-concurrency revision where it is mutable. Records that affect audit also have actor, correlation, and causation identifiers. Display names are mutable aliases, never foreign keys.

## Core entities

| Entity | Purpose | Owner | Key relationships |
| --- | --- | --- | --- |
| Organization / User / Principal | identity and authorization scope | Identity / Policy | authorizes commands and decisions |
| Project | repository and policy boundary | Mission & Portfolio | contains missions, artifacts, configurations |
| Mission | executive outcome with scope, success, mode, budget | Mission & Portfolio | has plans, runs, decisions, milestones |
| Plan | versioned decomposition and acceptance matrix | Orchestration | proposes task DAG for a mission |
| Task | bounded unit of work and dependency node | Orchestration | has attempts, input/output artifacts, gates |
| Run / Attempt / Lease | execution history and worker allocation | Orchestration | binds task, worker, capability, sandbox |
| Worker Profile / Session | role policy and live compatible executor | Worker Registry | selected for attempts |
| Brain Profile / Invocation | provider-neutral reasoning configuration and evidence | Brain Gateway | used by worker attempt |
| Skill / Plugin Package | versioned reusable capability | Registry | resolved into profile/task |
| Policy / Decision | rule set and evaluated allow/deny/approval outcome | Policy & Security | guards command/action/egress |
| Tool Action | requested and observed effect | Tool & Workspace | has capability token, idempotency and evidence |
| Artifact | immutable content-addressed body or Git reference | Artifact Manager | supports evidence and handoff |
| Check / Finding / Gate | quality, test, security, review, benchmark result | Quality / Security | gates task/mission progression |
| Knowledge Record / Claim / Link | source-backed organizational memory | Knowledge | references artifacts and decisions |
| Conversation / Delivery | Messenger interaction and channel receipt | Messenger | normalizes user intent |
| Event | immutable domain fact | Event Ledger | source for projections and audit |

## Key relationships

~~~mermaid
erDiagram
  PROJECT ||--o{ MISSION : contains
  MISSION ||--o{ PLAN : has
  PLAN ||--o{ TASK : decomposes_to
  TASK ||--o{ ATTEMPT : executes_as
  ATTEMPT }o--|| WORKER_PROFILE : uses
  ATTEMPT }o--|| BRAIN_PROFILE : requests
  ATTEMPT ||--o{ TOOL_ACTION : proposes
  ATTEMPT ||--o{ ARTIFACT : produces
  TASK ||--o{ GATE : requires
  GATE ||--o{ CHECK : evidenced_by
  ARTIFACT ||--o{ KNOWLEDGE_RECORD : sources
  MISSION ||--o{ DECISION : requires
  EVENT }o--|| MISSION : relates_to
  EVENT }o--|| TASK : relates_to
~~~

## Record invariants

- A Mission declares executive outcome, scope boundary, acceptance criteria, autonomy mode, budget, data classification, priority, and owner. Scope changes create a revision and may require a decision.
- A Task has one owning mission/plan, typed inputs/outputs, dependency edges, acceptance criteria, required gates, and an allowed capability envelope. A completed task never becomes mutable; a replacement task supersedes it.
- An Attempt has at most one active lease. A lease has expiry and heartbeat policy. Multiple attempts may exist for retry, but their causation and idempotency relationship are explicit.
- An Artifact has a content hash, media/type schema, producer, provenance, classification, retention, and immutable body or a stable external/Git reference.
- A Policy Decision binds subject, action, resource, context, rule version, outcome, conditions, authority, expiry, and evidence. It is not merely a boolean.
- A Check is reproducible enough to identify command/toolchain/environment, artifact version, status, measurement, threshold, and raw evidence reference.
- A Knowledge Claim cannot be canonical without at least one source artifact, freshness/validity status, confidence, and access classification.

## Data ownership and access

Contexts own writes to their records. A projection may duplicate read models but cannot establish a new source of truth. The UI, brain adapters, workers, plugins, and integrations access data only through scoped query/artifact ports. Database tables, local files, and secrets are never public extension APIs.

## Storage strategy

V1 uses a local transactional operational database for domain records/event ledger, a content-addressed local artifact store for larger immutable bodies, Git for source-history truth, and derived indexes for full-text/semantic knowledge retrieval. The database serves one local Core writer; WAL is a local concurrency optimization, not a network-shared storage design. Raw telemetry and large logs are retained separately with bounded policy, rather than event-sourcing every byte.

## Concurrency and deletion

Commands carry expected revisions or idempotency keys. Conflicts create a visible rejection/retry flow; only document-like content may later use CRDTs. Workflow state is not CRDT-merged. Retention uses tombstone/supersession references so a removed or corrected record is excluded from normal retrieval without destroying allowed audit evidence. Legal holds and enterprise retention policy override ordinary expiration.

## Data classifications

The baseline taxonomy is Public, Internal, Confidential, Restricted, and Secret. Classifications travel with artifacts, knowledge, mission context, and provider/tool requests. Policy maps classifications to allowed storage, brain profiles, channels, retention, export, and redaction behavior. Secret is reference-only by default and must not appear in ordinary artifacts or prompts.

See also: [08_Knowledge_System.md](08_Knowledge_System.md), [14_Security_Architecture.md](14_Security_Architecture.md), [17_Event_System.md](17_Event_System.md), and [18_API_Architecture.md](18_API_Architecture.md).

