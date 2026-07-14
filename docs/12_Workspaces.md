# 12 — Workspaces

## Principle and sequencing

Workspaces are tailored, authorization-filtered lenses over the same structured mission, specification, plan, phase, evidence, and policy state. The IDE is important but deliberately not the product center. A person can lead an engineering portfolio in WePLD without reading source code or operating an external project-management method.

Core alone commits durable governance and workflow truth. A workspace submits typed commands and renders Core projections; it cannot approve on behalf of a principal, issue capabilities, mutate a specification, bypass WIP, perform a protected effect, or decide completion. Broad workspace delivery belongs to **H9 Product Surfaces**, after the governed-delivery and Hermes runtime contracts have passed their preceding gates.

## Workspace catalog

| Workspace / surface | Primary content | Allowed interaction |
| --- | --- | --- |
| Mission Control | operational health, active phases, WIP, alerts, decisions | inspect, triage, submit approved interventions |
| Missions | charter, status, outcome, owners, budgets, phase graph | describe, clarify, pause, return, defer, cancel through commands |
| Spec Review | versioned Engineering Specification, exclusions, assumptions, criteria, verification bindings | comment, request clarification, approve or reject when authorized |
| Plan Review | Delivery Plan, Phase Plans, traceability, dependencies, estimates, stop conditions | review, request change, approve or reject when authorized |
| Kanban | phase-local task flow, WIP, leases, blocked and protected-effect queues | reprioritize or transition only through policy-valid commands |
| Decisions | evidence-linked Decision Requests and affected dependents | decide, delegate, expire, or request evidence within authority |
| Risks | Risk Items, assumptions, triggers, mitigations, owners, residual state | assess, assign, escalate, accept only through protected authority |
| Evidence | requirements, bundles, checks, findings, gates, provenance | inspect, validate, challenge, export where permitted |
| Change Requests | Specification or Plan Change Requests and impact analysis | submit, review, approve, reject, withdraw, supersede |
| Completion Review | Completion Proposal, outcome trace, unresolved risk, evidence completeness | accept, return, defer, or cancel when authorized |
| Executive | portfolio outcomes, roadmap, risk, cost, capacity, reports | set priorities and make authorized strategic decisions |
| Architecture | declared/observed architecture, APIs, data flow, policy boundaries, ADRs | propose and review architecture decisions |
| Timeline | causal events, approvals, model/skill/tool versions, effects, evidence | inspect, filter, export permitted evidence |
| Knowledge | verified engineering memory, sources, contradictions, freshness | search, challenge, propose Memory Candidates |
| IDE | explorer, editor, terminal, Git, diff, diagnostics, task context | review or work directly only under scoped policy |
| Settings / Registry | policy, brains, workers, skills, plugins, tools, integrations | configure or request activation within administrative authority |

These may be tabs or contextual panels rather than sixteen unrelated top-level destinations. The contract is that each required review and decision is first-class, not hidden inside chat.

## Mission, specification, and plan workspaces

The Mission Workspace guides the native flow from `MissionCharter` through clarification, approved `EngineeringSpecification` and `OutcomeContract`, validated and approved `DeliveryPlan`, approved `PhasePlan` records, bounded `TaskPacket` execution, and completion review. It always displays the governing version chain:

`Policy > approved EngineeringSpecification > approved OutcomeContract > approved DeliveryPlan > approved PhasePlan > authorized TaskPacket > ToolAction`

No lower layer can silently redefine a higher one. Editing an approved specification starts a Specification Change Request and new version. Changing only sequencing, tools, estimates, or delivery strategy starts a Plan Change Request. The original approved records remain immutable and visible.

The Brain Agent may populate proposed specification or planning artifacts and validation findings, but never renders its proposal as approved. Hermes may recommend phase or task transitions; Core validates and records them. Builder and subagent output appears as proposed actions, artifacts, findings, and evidence, not governance truth.

## Executive Workspace

The Executive Workspace shows no code by default. It focuses on outcome confidence, specification and plan status, phase flow, budget, current risks and assumptions, architecture impact, decision latency, evidence completeness, completion proposals, and digest reports. A summary links to the authoritative contract and evidence behind it. “Approve strategy” is never a vague super-action: each affordance names the artifact, version, consequence, and authority being exercised.

## Kanban and phase workspace

Each active phase exposes the Core-recorded task states `Backlog`, `Ready`, `InProgress`, `Review`, `Verification`, and `Done`, plus `Blocked`, `NeedsClarification`, `NeedsApproval`, `Returned`, `Deferred`, `Uncertain`, and `Cancelled`. The board shows which transition rule and evidence requirement applies. Client drag-and-drop submits an expected-revision command; the card moves only after Core records the transition.

WIP limits are policy-configurable runtime constraints, not display preferences. The view distinguishes one writable implementation task per isolated worktree, bounded parallel read-only exploration, unresolved-decision limits, and pending protected-effect limits. Hermes schedules within those limits; Core owns the counters and rejects invalid admission.

## IDE Workspace

The IDE is a review and focused-intervention environment. Autonomous changes appear in isolated worktrees or diffs with a provenance banner identifying specification, phase, task packet, builder profile, skills, tool effects, and evidence. The default is never to write the user’s primary worktree. Terminal, Git, network, dependency, database, push, pull-request, merge, and deployment actions pass through the same Effect Firewall as every other surface.

LSP diagnostics, affected symbols/tests, retrieved context, and model output are evidence sources with provenance and trust labels; none independently overrides an approved specification, policy, or exact repository fact.

## Architecture and Timeline workspaces

The Architecture Workspace distinguishes declared architecture from observed evidence and links important edges to applicable policies, approved specifications, ADRs, artifacts, and validation status. It flags undocumented dependencies and shows the impact of a proposed change before approval.

The Timeline is a causally navigable engineering record. A user can travel from intent to specification requirement, plan approval, phase transition, task lease, brain/skill invocation, tool intent, observed effect, test result, completion decision, and memory candidate without reconstructing chat history. Event references retain identity and integrity even when policy later redacts or expires an authorized body.

## Knowledge and memory workspace

The Knowledge Workspace separates Governance Memory from working, mission, engineering, skill, and provider/model performance memory. Users may correct sources, challenge claims, or propose a `MemoryCandidate`; they do not directly rewrite verified Engineering Memory. Consolidation shows Memory Judge validation, contradictions, freshness, scope, supersession, and provenance.

## Permissions and data handling

Core evaluates identity, artifact authority, classification, and field-level disclosure before returning a projection. Workspaces do not receive hidden fields and merely conceal them in the client. Sensitive logs, provider traces, context items, or secret-related findings may be summarized for an executive while protected evidence remains available only to authorized roles. Exports are auditable artifacts governed by classification, retention, and sharing policy.

## Cross-workspace continuity

Stable trace links preserve both context and authority: an alert opens its Risk Item or phase; a decision opens its evidence and affected task; a diff opens its Task Packet and checks; a completion criterion opens its evidence bundle; an accepted lesson opens the completion decision and source chain. Navigation may preserve filters and time range but cannot create a second mutable copy of state.

## H9 delivery boundary

Before H9, minimal CLI flows and narrow Mission Control projections may validate real runtime behavior. H9 delivers the coherent Spec Review, Plan Review, Kanban, Decisions, Risks, Evidence, Change Requests, Completion Review, Executive, Architecture, Knowledge, Timeline, and review-oriented IDE experience. Editor parity, decorative agent animation, and general chat breadth remain non-goals until the operational contracts are trustworthy.

See [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [07_Messenger_Agent.md](07_Messenger_Agent.md), [11_UI_UX_Architecture.md](11_UI_UX_Architecture.md), [13_Mission_Control.md](13_Mission_Control.md), and [29_Future_Vision.md](29_Future_Vision.md).
