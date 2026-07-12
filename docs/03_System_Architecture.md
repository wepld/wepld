# 03 — System Architecture

## Decision

WePLD begins as a **desktop-first modular control plane**: a Studio client communicates with a long-lived local Core Daemon over authenticated local RPC. The daemon is the authority for durable workflow state, policy decisions, worker leases, knowledge provenance, and audit. It can operate entirely locally except for explicitly selected brain providers and notification/integration adapters.

This avoids the operational and privacy cost of a cloud-first microservice fleet while preserving boundaries that can later become remote services.

## Five primary systems

| System | Purpose | Authority |
| --- | --- | --- |
| Studio | Presents workspaces, receives user intent, and renders live projections | No direct state mutation; sends commands |
| Orchestration Engine | Plans durable runs, owns task graph state, schedules work, applies policy gates | Sole writer for mission/run/task state |
| Worker System | Runs role-specialized, leased execution attempts in isolated environments | Scoped task artifacts and tool effects |
| Knowledge System | Stores and retrieves evidence-backed organizational memory | Knowledge records and retrieval indexes |
| Brain System | Normalizes reasoning providers and routes structured requests | Provider invocation and usage records |

Cross-cutting systems are Policy & Security, Tool Execution, Artifact/Git Management, Quality, Observability, and the Messenger/Integration Gateway. They are not optional add-ons; each is a bounded context with explicit ports.

## Context and trust-boundary diagram

~~~mermaid
flowchart LR
  Human["Human executive"] --> Studio["Studio workspaces"]
  Studio <--> RPC["authenticated local RPC"]
  Channels["Telegram • Slack • Email • etc."] <--> Messenger["Messenger + channel adapters"]
  Messenger <--> RPC
  RPC <--> Core["Local Core Daemon\nMission • Orchestration • Policy • Event Ledger"]
  Core <--> Knowledge["Knowledge + Evidence\nSQLite, files, indexes"]
  Core <--> Artifact["Git / Worktree + Artifact Manager"]
  Core <--> Scheduler["Worker Scheduler + Mailboxes"]
  Scheduler <--> Workers["Sandboxed worker hosts"]
  Workers <--> Tools["Capability-mediated tools"]
  Core <--> Brains["Brain Gateway / router"]
  Brains <--> Providers["Local or external providers"]
  Core --> Telemetry["Local telemetry + optional exporter"]
~~~

## Deployment topology: V1

| Process / store | Runs where | Trust boundary | Notes |
| --- | --- | --- | --- |
| Studio desktop shell | User desktop | User interface | Has no ambient filesystem, secret, or worker authority |
| Core Daemon | User desktop | Local control plane | Single writer to operational database |
| Worker host | Child process or supported sandbox | Untrusted execution boundary | Uses per-task capability token and isolated worktree |
| Brain adapter | Daemon-owned adapter | Data egress boundary | Receives only policy-approved context |
| SQLite operational store | User-controlled application data directory | Durable local state | Event ledger and projections; WAL only on local filesystem |
| Artifact store | User-controlled application data directory and Git worktrees | Content integrity boundary | Immutable hash-addressed evidence; source code remains in Git |

Remote workers, collaborative sync, organization policy servers, and hosted retention are future deployments. They must consume the same versioned contracts and never become a shortcut around the Core Daemon’s policy semantics.

## Control, data, and execution planes

- **Control plane:** commands, policy decisions, task transitions, leases, cancellation, budgets, and approvals. It is durable and auditable.
- **Data plane:** Git, task artifacts, logs, metrics, knowledge documents, and provider context. Large bodies are referenced by immutable content hashes rather than embedded in every event.
- **Execution plane:** sandboxed workers and tools. Effects are requested, approved, performed, then evidenced; at-least-once delivery is assumed.

## Canonical mission flow

1. Studio or Messenger submits a Mission command with success criteria, scope, autonomy mode, budget, and classification.
2. Core records the intent, evaluates policy, and produces a planning run.
3. A planning worker proposes a task DAG; Core validates dependencies, required gates, and capability envelopes.
4. Scheduler leases ready tasks to compatible workers. Workers obtain scoped artifacts and propose tool actions through the Tool Executor.
5. Quality, security, and review workers attach evidence. Core advances only valid state transitions.
6. Messenger reports material changes and sends a decision packet only when policy marks a strategic decision.
7. Completion requires acceptance evidence, gate results, no blocking findings, and budget/scope compliance. The timeline and knowledge graph are updated from the same event stream.

## Future evolution rules

The Core Daemon may later become a regional or organization control plane when real collaboration requires it. Before that split, all boundaries must be exercised through interfaces, database access remains private to the owning module, and no UI or adapter may depend on a storage schema. PostgreSQL/object storage, remote queues, and distributed workflow engines are scale-out candidates—not V1 dependencies.

## Architecture acceptance criteria

- A brain, worker, skill, or channel adapter can be replaced without changing mission-domain code.
- A stopped daemon can recover a mission without guessing task status.
- A worker cannot modify the primary worktree, access undeclared secrets, or communicate directly with a human.
- Every Mission Control view, Timeline entry, and Messenger report is derivable from durable events and evidence.

See also: [04_Component_Architecture.md](04_Component_Architecture.md), [16_Data_Model.md](16_Data_Model.md), [17_Event_System.md](17_Event_System.md), and [23_Technology_Evaluation.md](23_Technology_Evaluation.md).

