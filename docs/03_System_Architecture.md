# 03 — System Architecture

## Decision

WePLD begins as a **desktop-first modular control plane**. Studio, CLI, MCP, APIs, and Messenger communicate with a long-lived local Core Daemon through authenticated commands and queries. These surfaces render or transport the same workflow; none is an alternate state authority.

Core alone owns durable mission truth, governance policy, artifact validation, approvals, capabilities, budget truth, transitions, completion, and recovery. The Brain Agent proposes governed plans. Hermes supervises engineering execution. Replaceable brain and builder models reason within typed contracts. Tool boundaries alone perform Core-authorized effects.

This preserves local privacy and recovery while allowing explicit external model or notification adapters. It also preserves boundaries that may later become remote services without changing mission semantics.

## Primary systems and authority

| System | Purpose | Authority boundary |
| --- | --- | --- |
| Product surfaces | Describe outcomes, review specifications/plans, show Kanban/evidence, and collect decisions | Authenticated commands and projections only |
| Core control plane | Govern specifications, plans, phases, tasks, policy, approvals, capabilities, budgets, events, completion, and recovery | Sole durable workflow and authorization authority |
| Brain Agent | Clarify, architect, analyze risk, emit `PlanProposal`/`PhasePlan` proposals, and replan | Structured proposals only; never compilation, assessment, approval, or effects authority |
| Hermes Engineering Intelligence Runtime | Compile context, route skills/models/subagents, run bounded loops, and supervise evidence production | Bounded operational state only; never governance truth |
| Brain Gateway | Normalize and route replaceable brain and builder model calls | Emits invocation evidence through Core; no tools or domain authority |
| Worker and Tool System | Host builders/subagents in isolation and execute mediated actions | Workers propose; tool boundaries execute only Core-authorized effects |
| Knowledge and Evidence System | Preserve artifacts, typed memory, provenance, and retrieval indexes | Derived and verified knowledge; authoritative governance remains in Core |

Policy & Security, Quality, Artifact/Git Management, Observability, Messenger, Skills, LSP, retrieval, and integrations are bounded modules with explicit ports. A module may own its internal records while Core remains the sole authority for governed state and transitions.

## Context and trust-boundary diagram

~~~mermaid
flowchart LR
  Human["User / designated authority"] --> Surfaces["Studio • CLI • MCP • API"]
  Human <--> Messenger["Messenger + channel adapters"]
  Surfaces <--> Core["Core Daemon\npolicy • approvals • durable state • budgets • transitions"]
  Messenger <--> Core
  Core --> BrainAgent["Brain Agent\nplanner • architect • risk analyst • replanner"]
  BrainAgent <--> Gateway["Brain Gateway\nreplaceable profiles"]
  Gateway <--> Providers["Local or approved external models"]
  Core --> Hermes["Hermes Intelligence Runtime\nskills • context • LSP/RAG • loops • subagents"]
  Hermes <--> Gateway
  Hermes --> Workers["Builder + bounded subagents\nisolated worktrees"]
  Workers --> Proposed["Typed proposed actions + artifacts"]
  Proposed --> Core
  Core --> Tools["Capability-mediated tool boundaries"]
  Tools --> Probe["Actual-result probes + evidence"]
  Probe --> Core
  Core <--> Knowledge["Evidence + typed memory\nSQLite • files • indexes"]
~~~

Hermes is never a provider in this diagram. It chooses an allowed brain or builder profile through the Gateway under the active TaskPacket and policy; it cannot change the approved envelope.

## Deployment topology: first governed release

| Process / store | Runs where | Trust boundary | Notes |
| --- | --- | --- | --- |
| Studio / CLI | User desktop | User interface | No ambient filesystem, secret, policy, or worker authority |
| Core Daemon | User desktop | Local control plane | Single durable writer and transition authority |
| Hermes runtime | Daemon-managed process/module | Untrusted intelligence boundary | Holds bounded operational state; recovers from Core truth |
| Worker host | Child process or supported sandbox | Untrusted execution boundary | Per-attempt lease, TaskPacket, capability, quota, and isolated worktree |
| Brain adapter | Core-controlled adapter | Data-egress boundary | Receives only approved, provenance-labelled context |
| SQLite operational store | User-controlled application data directory | Durable local state | Event ledger and projections; WAL only on local filesystems |
| Artifact store | Application data directory and Git worktrees | Content-integrity boundary | Immutable hash-addressed evidence; source remains in Git |

Remote workers, collaboration sync, hosted policy, and cloud retention are later deployments. They must consume the same versioned contracts and cannot bypass Core semantics.

## Authority hierarchy

Core validates every proposed transition against:

1. WePLD governance policy
2. approved `EngineeringSpecification`
3. approved `OutcomeContract`
4. approved `DeliveryPlan`
5. approved `PhasePlan`
6. Core-issued `TaskPacket`
7. proposed and Core-authorized `ToolAction`

Each lower artifact cites the exact higher versions. A mismatch is an invalid plan or packet, not an invitation for a model to reinterpret intent.

## Control, data, intelligence, and execution planes

- **Control plane:** governed artifacts, commands, policy decisions, approvals, transitions, leases, cancellation, WIP, budgets, capabilities, change requests, and completion decisions. It is durable and auditable.
- **Data plane:** Git, artifacts, diagnostics, logs, metrics, specifications, plans, evidence, and memory. Large bodies use immutable content references.
- **Intelligence plane:** Brain Agent proposals and Hermes context compilation, skill routing, LSP, hybrid retrieval, model calls, controlled loops, and bounded subagents. It has no independent effect authority.
- **Execution plane:** isolated worker hosts and mediated tools. Every effect is proposed, classified, authorized, durably intended, performed, probed, and evidenced; at-least-once delivery is assumed.

## Canonical governed mission flow

1. A product surface records a `MissionCharter` from the user's described outcome.
2. The Brain Agent proposes clarification questions and a versioned `EngineeringSpecification` defining WHAT success means, plus an `OutcomeContract` binding acceptance to verification and evidence.
3. Core validates the proposal; an authorized user reviews and approves it. An approved version is immutable.
4. The Brain Agent emits a traceable `PlanProposal` and tailored phase graph. A deterministic Plan Compiler normalizes it into a candidate `DeliveryPlan`; structural and policy/risk-tier assessment plus required independent reviews produce an exact `PlanAssessment`; only an authenticated `PlanDecision` can approve that candidate. The producing Brain cannot approve or provide the only acceptance-critical review.
5. Before a phase starts, an approved `PhasePlan` declares entry/exit conditions, WIP, budget, risks, skills, tools, writable/forbidden scope, tasks, evidence, and escalation conditions.
6. Core issues bounded `TaskPackets`. Hermes compiles cited context and supervises builders/subagents. Tasks flow through Core-enforced Kanban states.
7. Builders propose typed actions. Core applies policy, capability, budget, and approval checks; a tool boundary executes and records the actual result.
8. Evidence may trigger retry, replan, a `DecisionRequest`, a `ChangeRequest(kind=SpecificationChange)`, or a `ChangeRequest(kind=PlanChange)`. No approved artifact is silently edited.
9. Independent review, QA, security, and applicable performance checks produce evidence. Hermes may assemble a `CompletionProposal`; it cannot decide completion.
10. Core verifies the evidence and records the authorized `CompletionDecision`: accept, return, defer, or cancel.
11. A Memory Judge evaluates evidence-linked `MemoryCandidates`; only approved lessons enter scoped Engineering or Skill Memory.

The detailed artifact and lifecycle contracts are in [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md).

## Phase and flow semantics

Phase is the primary delivery unit. A typical graph may include Discovery, Specification, Architecture and Contract Design, Implementation, Verification, and Delivery, but the Brain Agent may tailor it within governance rules. Phase state and task Kanban state are separate typed lifecycles. Core enforces WIP, including one writable implementation task per isolated worktree, bounded read-only research, bounded unresolved decisions, and bounded pending protected effects.

## Future evolution rules

Core may later become a regional or organization control plane when collaboration requires it. Before any split, modules communicate through interfaces and typed events; database access stays private; no UI, model adapter, skill, hook, or plugin depends on a mutable storage schema. PostgreSQL, object storage, remote queues, and distributed workflow engines are scale-out candidates, not first-release dependencies.

## Architecture acceptance criteria

- A brain, builder, skill, worker, channel, or product surface can be replaced without changing mission-domain semantics.
- A stopped daemon or Hermes process recovers from Core truth without guessing phase, task, budget, approval, or effect status.
- No model or worker can modify the primary worktree, access undeclared secrets, authorize itself, or contact a human directly.
- Every view and report is derivable from durable events, governed artifacts, and evidence.
- Different supported profiles may be accepted only when they satisfy the same OutcomeContract and fixed quality gates.

See also: [04_Component_Architecture.md](04_Component_Architecture.md), [16_Data_Model.md](16_Data_Model.md), [17_Event_System.md](17_Event_System.md), [23_Technology_Evaluation.md](23_Technology_Evaluation.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), and [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md).
