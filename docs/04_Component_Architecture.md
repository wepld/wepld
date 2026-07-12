# 04 — Component Architecture

## Architectural style

WePLD uses domain-oriented, hexagonal boundaries inside a modular monolith. The domain layer defines stable types, invariants, commands, events, and ports. Adapters implement providers, databases, UI transport, operating-system execution, Git, notification channels, and plugin runtimes. Dependencies point inward: domain code must not import a vendor SDK, desktop framework, or database driver.

## Bounded contexts

| Context | Owns | Public port | Cannot own |
| --- | --- | --- | --- |
| Mission & Portfolio | missions, goals, priority, scope, decision requests | Mission command/query port | task execution or provider calls |
| Orchestration | runs, task DAGs, transitions, leases, retries, cancellation | Scheduler and run port | raw filesystem effects |
| Policy & Security | rules, risk classification, approvals, capabilities, exceptions | Policy decision port | UI conversation or tool execution |
| Worker Registry | roles, worker profiles, sessions, health, compatibility | Worker lease port | mission state mutation |
| Brain Gateway | profiles, provider capability, routing, invocation record | Reasoning port | tools, project writes, secrets |
| Tool & Workspace | worktrees, sandboxed commands, artifacts, patches | Tool action port | policy choice or task graph state |
| Quality | checks, reviews, benchmarks, release readiness | Gate evaluation port | final strategic approval |
| Knowledge & Evidence | documents, claims, links, indexes, retention | Retrieval and ingest port | canonical workflow state |
| Skills & Plugins | packages, dependencies, trust, lifecycle, compatibility | Capability resolution port | unrestricted in-process code |
| Messenger & Integrations | conversations, reports, channel queues, delivery receipts | Human interaction port | privilege grants |
| Observability | traces, metrics, logs, health projections | Telemetry port | business workflow decisions |

## Dependency rules

1. Only the Orchestration context may commit mission, run, or task transition state.
2. A context accesses another context through a port or published event, never by reading its tables.
3. The Tool & Workspace context enforces a capability token issued after a Policy decision; it does not trust a worker request alone.
4. Quality and Security supply gate evidence. The Orchestrator interprets their required/optional status from the mission policy.
5. Knowledge consumes durable events and artifacts, but failure to index a document must not corrupt mission state. It records deferred ingestion and retries.

## Component responsibilities

~~~mermaid
flowchart TB
  Command["Command adapters\nStudio / Messenger / API"] --> Mission
  Mission["Mission & Portfolio"] --> Orchestrator["Orchestration"]
  Orchestrator --> Policy["Policy & Security"]
  Orchestrator --> Registry["Worker Registry"]
  Registry --> Worker["Worker Host"]
  Worker --> Brain["Brain Gateway"]
  Worker --> Tool["Tool & Workspace"]
  Tool --> Evidence["Artifacts + Git"]
  Orchestrator --> Quality["Quality / Review"]
  Evidence --> Knowledge["Knowledge & Evidence"]
  Orchestrator --> Ledger["Event Ledger"]
  Ledger --> Projections["Mission Control / Timeline / Messenger"]
~~~

## Lifecycle ownership

| State | Owner | Valid entry condition |
| --- | --- | --- |
| Draft mission | Mission & Portfolio | Human or authorized system creates it |
| Planned | Orchestration | A policy-approved, reviewable task graph exists |
| Running | Orchestration | At least one nonterminal task has a valid lease or is ready |
| Waiting for decision | Orchestration | A classified strategic decision packet exists |
| Verifying | Quality / Orchestration | Required build, test, review, security, and benchmark work is scheduled |
| Completed | Orchestration | All acceptance criteria and mandatory gates have evidence |
| Cancelled / failed | Orchestration | Reason, partial artifacts, and recovery path are recorded |

## Artifact handoff contract

Workers do not pass prose-only messages to each other. A handoff contains a typed artifact reference, its immutable hash, provenance, task/run identifiers, schema version, intended consumer role, and a concise statement of limits or uncertainties. The receiving worker retrieves the artifact through Core-controlled access and records whether it accepted, rejected, or superseded it.

## Architectural decisions

- **No peer-to-peer worker messaging:** it defeats scheduling, audit, and policy control.
- **No direct database access from UI or plugins:** it couples presentation and extensions to mutable internals.
- **No “god agent”:** executive, planner, builder, reviewer, and security roles are distinct policies over a shared worker contract.
- **No distributed services before collaboration demand:** network failures should not be a V1 feature.

See also: [05_Worker_Architecture.md](05_Worker_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [18_API_Architecture.md](18_API_Architecture.md), and [24_Repository_Structure.md](24_Repository_Structure.md).

