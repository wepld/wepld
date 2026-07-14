# 07 — Messenger Agent

## Mandate

Messenger is the sole **agent persona** permitted to initiate or conduct human-facing communication. It is an interaction and reporting adapter over Core, not a privileged executive or governance authority. Brain Agent, Hermes, builders, and subagents never contact users directly. Messenger never grants privileges, approves on a user's behalf, changes a project, or mutates durable state.

This rule does not make Messenger a mandatory hop for every product interaction. Studio, CLI, MCP, and APIs are authenticated command/query surfaces over the same Core workflow. Whether intent arrives through a form, command, API, or conversation, Core applies identical identity, policy, artifact, transition, and approval semantics.

## Responsibilities

- Help a user describe an outcome and present Brain Agent clarification questions without inventing answers.
- Present versioned EngineeringSpecification, OutcomeContract, DeliveryPlan, and PhasePlan review projections.
- Collect authenticated approval, rejection, return, defer, cancel, and change-request commands.
- Produce phase/Kanban updates, risk summaries, budget forecasts, evidence status, retrospectives, and project-health reports from durable projections.
- Present DecisionRequests with the minimum context needed for the named authority.
- Maintain channel delivery, consent, identity/thread mapping, notification preferences, receipts, and acknowledgment state.
- Keep unrelated work moving while a decision is pending unless dependency or WIP policy requires a pause.

Messenger summarizes; it does not become the source of specification, plan, evidence, or completion truth.

## Native user interaction flow

| User step | Messenger / surface responsibility | Core result |
| --- | --- | --- |
| Describe | capture outcome, constraints, priorities, and known exclusions | versioned MissionCharter |
| Clarify | present unresolved questions, assumptions, and consequences | resolved or explicitly open items |
| Review specification | show WHAT, acceptance criteria, verification bindings, risks, and evidence needs | approved, returned, deferred, or cancelled specification version |
| Review plan | show phases, dependencies, scope, WIP, budget, risks, gates, and decisions | approved or returned DeliveryPlan/PhasePlan version |
| Observe execution | render phase and Kanban state, actual effects, evidence, cost, and uncertainty | no authority change |
| Decide/change | present typed DecisionRequest or ChangeRequest | authorized versioned transition |
| Review completion | show OutcomeContract trace, gate evidence, unresolved risk, and retrospective | CompletionDecision: accept, return, defer, or cancel |
| Consolidate memory | show policy-selected MemoryCandidates where human review is required | approved, rejected, or deferred consolidation |

## DecisionRequest contract

A `DecisionRequest` is a versioned Core artifact containing:

| Field | Requirement |
| --- | --- |
| Decision | one clear question and permitted options, including defer where safe |
| Why now | triggering evidence, deadline, blocked dependencies, and WIP effect |
| Governing context | policy and exact specification/contract/plan/phase versions |
| Recommendation | proposing role, rationale, confidence, assumptions, and dissenting evidence |
| Consequences | outcome, scope, cost, security, schedule, reversibility, and evidence impact per option |
| Evidence | cited artifacts, checks, findings, and known uncertainty |
| Authority | policy rule, authenticated principal/role, and quorum where applicable |
| Resolution | signed response, timestamp, rationale, resulting command, and supersession links |

Messenger may collect the response; Core authenticates the principal, verifies authority, records the decision, and determines the transition.

## Change and completion interactions

A request changing WHAT creates a `ChangeRequest(kind=SpecificationChange)` and a new specification version if approved. A request changing only HOW creates a `ChangeRequest(kind=PlanChange)` and affected plan versions. Messenger must show which requirements, phases, tasks, evidence, budget, and completed work are invalidated; a conversational “small change” never edits an approved artifact in place.

A completion notification is a `CompletionProposal`, not a success declaration. Messenger presents evidence completeness and unresolved risks. Only an authorized `CompletionDecision` can accept, return, defer, or cancel. Hermes, builders, reviewers, and Messenger cannot issue that decision.

## Communication model

~~~mermaid
sequenceDiagram
  participant Core as WePLD Core
  participant Msg as Messenger
  participant Channel as Channel Adapter
  participant Human as Authorized Human
  Core->>Msg: projection or DecisionRequested event
  Msg->>Msg: summarize, redact, apply preferences
  Msg->>Channel: queued report / review / decision packet
  Channel->>Human: human-facing message
  Human->>Channel: intent / approval / change / completion decision
  Channel->>Msg: authenticated normalized input
  Msg->>Core: typed command with principal identity
  Core->>Core: authority + policy + validation + durable event
  Core-->>Msg: resulting projection or rejection
~~~

## Interruption policy

Messenger interrupts when configured materiality requires it: specification/plan approval, a required human decision, safety incident, invalid governing artifact, critical gate failure, imminent budget/time boundary, protected effect, completion proposal, or user-selected digest. Routine loop iterations and worker steps stay in projections and summaries. A decision blocks only dependent work unless policy requires a broader stop.

## Channels and adapters

Studio inbox is the initial conversational adapter. Later adapters may include Telegram, Discord, Slack, WhatsApp, email, and push. Every adapter supplies identity verification, delivery receipt, rate limiting, thread mapping, formatting, inbound command parsing, retention, and opt-out semantics. A channel is untrusted input and never tool authority.

Channel breadth follows stable Core and Hermes contracts; it is not a prerequisite for governed delivery.

## Identity, privacy, and disclosure

Every inbound command binds to an authenticated principal and project scope. Messenger applies data classification and channel policy before disclosure; sensitive artifacts may be redacted or restricted to Studio. Conversation transcripts are sources, not governance records or automatically consolidated memory. No third party receives project content merely because its channel is connected.

## Operating modes

| Mode | Messenger behavior |
| --- | --- |
| Manual | presents all configured material effects and every required artifact approval |
| Limited Approval | requests declared gated effects and strategic changes; routine execution reports asynchronously |
| Full Autonomous | reports operation inside the approved envelope while preserving specification, plan, authority, and hard-effect gates |
| Enterprise Policy | applies centrally defined identity, retention, quorum, escalation, and notification rules |

No mode permits Brain Agent, Hermes, a builder, reviewer, or Messenger to approve its own proposal.

## Acceptance criteria

- No non-Messenger agent has a user identity, channel credential, or outbound transport.
- All product surfaces produce the same typed Core commands and semantic outcomes.
- Messenger reports only authorized durable projections and cited evidence.
- Approval and completion commands identify an authenticated, policy-authorized principal.
- A pending decision does not stop unrelated ready work.

See also: [03_System_Architecture.md](03_System_Architecture.md), [12_Workspaces.md](12_Workspaces.md), [13_Mission_Control.md](13_Mission_Control.md), [18_API_Architecture.md](18_API_Architecture.md), and [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md).
