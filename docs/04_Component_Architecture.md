# 04 — Component Architecture

## Architectural style

WePLD uses domain-oriented, hexagonal boundaries inside a desktop-first modular monolith. Domain modules define stable types, invariants, commands, events, and ports. Adapters implement models, databases, UI transport, operating-system execution, Git, notification channels, LSPs, retrieval indexes, and plugin runtimes. Dependencies point inward: governance code never imports a vendor SDK, desktop framework, or database driver.

All modules execute under one constitutional rule: Core alone owns durable truth, policy, approvals, capabilities, budgets, transitions, completion, and recovery. Logical ownership by a Core module does not confer authority on its adapters, models, Hermes, or workers.

## Bounded contexts

| Context | Owns inside Core | Public port | Cannot own |
| --- | --- | --- | --- |
| Mission & Portfolio | MissionCharters, goals, priorities, principals | Mission command/query port | specifications, task execution, or provider calls |
| Specification & Outcomes | EngineeringSpecifications, OutcomeContracts, requirements, exclusions, verification bindings | Specification review/change port | implementation strategy or self-approval |
| Delivery Control | DeliveryPlans, PhasePlans, TaskPackets, phase/task state, WIP, leases, retries, cancellation | Plan, phase, Kanban, and scheduler ports | raw filesystem effects or model-specific reasoning |
| Decisions & Change | DecisionRequests, ChangeRequests, approvals, CompletionProposals, CompletionDecisions | Authority and review port | unrecorded approval or policy bypass |
| Policy & Security | rules, risk classification, capability policy, exceptions | Policy decision port | effect execution or UI conversation |
| Worker Registry | role profiles, compatibility, sessions, health, attempt records | Worker lease port | governed mission or plan transitions |
| Brain Gateway | profiles, provider capabilities, routing candidates, invocation evidence | Provider-neutral reasoning port | tools, governance truth, project writes, or secrets |
| Hermes Runtime | transient agent-kernel state, skill routing, context compilation, loop and subagent supervision | Governed execution port | durable mission truth, approvals, capabilities, or effects |
| Tool & Workspace | worktrees, sandboxes, commands, artifacts, patches, result probes | Tool action port | policy choice, scope invention, or task-state mutation |
| Quality | checks, reviews, benchmarks, evidence evaluation | Gate evaluation port | final acceptance or strategic approval |
| Knowledge & Memory | artifacts, claims, typed memory, provenance, indexes, retention | Retrieval, Memory Judge, and ingest ports | canonical governance/workflow state |
| Skills & Plugins | packages, procedures, trust, compatibility, lifecycle | Skill resolution port | unrestricted in-process code or policy bypass |
| Messenger & Integrations | reports, conversations, channel queues, receipts | Human interaction port | privilege grants or direct project mutation |
| Observability | traces, metrics, logs, health projections | Telemetry port | business workflow decisions |

## Dependency and authority rules

1. Core command handlers are the sole writers of governed state. Mission, Specification, Delivery, Decision, and Policy modules validate their own invariants inside that boundary.
2. The authority hierarchy is policy → approved EngineeringSpecification → OutcomeContract → approved DeliveryPlan → approved PhasePlan → TaskPacket → ToolAction.
3. A context accesses another context through a port or published event, never by reading its tables.
4. Brain Agent output, Hermes output, model output, worker findings, and hook output are proposals or evidence until a Core command validates and records them.
5. Tool & Workspace requires a Core-issued, scoped, expiring capability tied to durable effect intent. It probes the actual result rather than trusting a worker report.
6. Quality and Security supply gate evidence. Core evaluates transition eligibility from the active policy and OutcomeContract.
7. Knowledge consumes events and artifacts asynchronously. Failure to index or embed content cannot corrupt or replace authoritative Core state.
8. Governance retrieval resolves exact approved versions from Core before derived Engineering Memory or semantic similarity.

## Component responsibilities

~~~mermaid
flowchart TB
  Commands["Studio / CLI / MCP / API / Messenger"] --> Core["Core command + transition boundary"]
  Core --> Spec["Specification & Outcomes"]
  Core --> Delivery["Delivery Control"]
  Delivery --> BrainAgent["Brain Agent proposals"]
  BrainAgent --> Gateway["Brain Gateway"]
  Delivery --> Hermes["Hermes Runtime"]
  Hermes --> Gateway
  Hermes --> Worker["Builder / bounded subagent"]
  Worker --> Proposal["Artifact, finding, proposed action"]
  Proposal --> Core
  Core --> Policy["Policy & capability decision"]
  Policy --> Tool["Tool & Workspace boundary"]
  Tool --> Evidence["Probed evidence + artifacts"]
  Evidence --> Core
  Evidence --> Knowledge["Knowledge & typed memory"]
  Core --> Ledger["Event ledger"]
  Ledger --> Projections["Mission Control / Kanban / Timeline / Messenger"]
~~~

## Governed artifact ownership

| Artifact | May propose | Approval / decision authority | Durable owner and rule |
| --- | --- | --- | --- |
| `MissionCharter` | User or authenticated product surface | User / designated authority where required | Core; versions the desired outcome and envelope |
| `EngineeringSpecification` | Brain Agent, incorporating user clarification | User / designated authority | Core; approved version is immutable |
| `OutcomeContract` | Brain Agent with Quality/Security input | Same authority as its specification unless policy adds approvers | Core; binds acceptance criteria to verification and evidence |
| `PlanProposal` | Brain Agent or authorized architect | None; proposal is never approval | Core; records exact proposal/context provenance for deterministic compilation |
| candidate `DeliveryPlan` | Deterministic Plan Compiler from one exact `PlanProposal` | None until assessment and decision | Core; normalized candidate traces every phase to approved requirements |
| `PlanAssessment` | Deterministic validators create the initial assessment; independent reviewers create separate immutable records; Core finalizes the Ready assessment | Evidence only; cannot approve | Core; binds exact candidate, policy/risk tier, review record IDs/versions/hashes, blockers, readiness, and independence |
| `PlanDecision` / approved `DeliveryPlan` | Authorized plan-decision principal over one exact candidate/assessment/review set | User / designated authority under policy | Core; immutable authenticated decision, with model votes carrying no authority |
| `PhasePlan` | Brain Agent or controlled replan | Designated authority under policy | Core; declares gates, WIP, budget, capabilities, scope, and evidence |
| `TaskPacket` | Hermes derives and proposes it from an approved PhasePlan | Core validates and issues; no worker approval | Core; must fit one approved PhasePlan |
| `ToolAction` | Builder, subagent, or Hermes runtime | Core policy/capability/approval decision | Core records intent; tool boundary executes |
| `DecisionRequest` / `ChangeRequest` | Any role through a typed proposal | Named human or policy authority | Core; result creates explicit versioned transitions |
| `CompletionProposal` | Hermes or Delivery Control from evidence | No acceptance authority | Core records proposal and verified eligibility |
| `CompletionDecision` | Authorized user / principal | Authorized user / principal | Core; accept, return, defer, or cancel |
| `MemoryCandidate` | Worker, subagent, Hermes, or retrospective | Memory Judge plus policy-designated review | Core/Knowledge; remains non-authoritative unless consolidated |

The complete schemas, lifecycle states, provenance, and trace rules are defined in [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md).

## Lifecycle separation

Mission, specification, delivery plan, phase, task Kanban, worker attempt, effect, and completion each have separate typed state machines. Reusing labels such as `Active`, `Review`, or `Cancelled` never permits a transition between aggregate types. Core records causation across them.

Typical phase states are `Pending`, `Ready`, `Active`, `Blocked`, `Review`, `Verification`, `Closed`, `Returned`, `Deferred`, `Uncertain`, and `Cancelled`. Task Kanban states are `Backlog`, `Ready`, `InProgress`, `Review`, `Verification`, and `Done`, with explicit exception states including `Blocked`, `NeedsClarification`, `NeedsApproval`, `Returned`, `Deferred`, `Uncertain`, and `Cancelled`.

## Artifact handoff and traceability

Agents and workers do not pass prose-only messages as durable handoffs. A handoff carries a typed artifact reference, immutable hash, schema version, provenance, mission/phase/task/attempt identity, intended consumer, governing artifact versions, evidence obligations, and explicit limits or uncertainty. Core records acceptance, rejection, or supersession.

Traceability is continuous:

`user intent → specification requirement → OutcomeContract criterion → DeliveryPlan phase → PhasePlan → TaskPacket → required evidence → CompletionDecision → MemoryCandidate`.

## Architectural decisions

- **No peer-to-peer worker swarm:** Hermes supervises bounded work; durable relevance returns through typed Core-recorded results.
- **No direct database access from UI, models, skills, hooks, or plugins:** adapters cannot couple to mutable internals.
- **No god agent:** Brain Agent, Hermes, builder, reviewer, security, policy, Core, and tool boundary have separate authority.
- **No self-approval:** proposal and authorization are different commands and principals.
- **No distributed services before collaboration demand:** network failures are not a first-release feature.

See also: [05_Worker_Architecture.md](05_Worker_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [18_API_Architecture.md](18_API_Architecture.md), [24_Repository_Structure.md](24_Repository_Structure.md), and [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md).
