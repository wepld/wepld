# 11 — UI/UX Architecture

## Design thesis

WePLD should feel like entering an engineering studio: calm, legible, and operationally credible. The user provides a desired outcome; WePLD supplies the governed engineering method. The interface privileges specifications, plans, phase flow, decisions, evidence, risk, and accepted outcomes over token streams, model theater, or editor chrome.

The Studio is never an authority boundary. WePLD Core alone commits durable governance and workflow truth: policy, approvals, capability issuance, budget accounting, transitions, effect intent and observed result, completion, and recovery. The Brain Agent proposes specifications and plans, Hermes supervises approved delivery, builder models and bounded subagents propose actions and produce artifacts or evidence, and tool boundaries perform only authorized effects.

## Native user journey

The product makes the delivery method explicit without requiring an external project-management or specification system:

1. **Describe** the desired outcome in a `MissionCharter`.
2. **Clarify** assumptions, open questions, exclusions, risks, and constraints.
3. **Review and approve the specification** and its `OutcomeContract`.
4. **Review and decide the qualified delivery-plan candidate** compiled from the Brain Agent's `PlanProposal`, with its `PlanAssessment`, required independent reviews, risks, alternatives, and exact versions visible.
5. **Execute phase by phase** under approved `PhasePlan` contracts.
6. **Observe Kanban flow, WIP, budgets, and evidence** without managing low-level commands.
7. **Resolve decisions and change requests** through their protected authority paths.
8. **Review verified completion**, including gaps and unresolved risk.
9. **Accept, return, defer, or cancel** the completion proposal.
10. **Consolidate approved engineering memory** from evidence-derived candidates.

An approved specification is never silently edited. A change to **what** is required creates a Specification Change Request and a new specification version. A change only to **how** approved work is delivered creates a Plan Change Request. Until Core accepts the applicable request and records replacement approvals, execution remains bound to the preceding approved versions.

## Information architecture

Workspace views are authorization-filtered projections over the same Core state; no screen owns a separate workflow or governance record.

| Area | User question it answers |
| --- | --- |
| Home / Mission Control | What needs attention and how healthy is delivery? |
| Missions | Which desired outcomes are being clarified, planned, delivered, reviewed, or closed? |
| Spec Review | What must be true, what remains unclear, and which version may be approved? |
| Plan Review | How will the approved outcome be delivered, and is the plan valid and traceable? |
| Kanban | Which phase tasks are ready, active, blocked, under review, verified, or done, and are WIP limits respected? |
| Decisions | Which authorized decision is required, by whom, by when, and with what evidence? |
| Risks | Which assumptions or risks threaten the outcome, controls, budget, or evidence bar? |
| Evidence | What proves each requirement, gate, effect, and claimed result? |
| Change Requests | Is the requested change to specification truth or only to delivery method? |
| Completion Review | Does the completion proposal satisfy the approved outcome contract, and who may decide? |
| Executive | Are outcomes, roadmap, risk, cost, and capacity aligned with strategy? |
| Architecture | What system shape, policy boundary, and dependency impact does this work have? |
| Timeline | What happened, why, under which authority, and with what evidence? |
| Knowledge | What verified engineering memory is applicable, fresh, and authorized? |
| IDE | What does the project contain and what isolated change is under review? |
| Settings / Registry | Which brains, workers, skills, plugins, policies, and integrations are approved? |

Broad Studio surface work is an **H9 Product Surfaces** concern. Earlier milestones may use a minimal CLI and narrow read projections to prove runtime contracts, but they must not create a convincing interface over incomplete governance truth.

## Primary interaction patterns

- **Governed contract review:** specification, outcome, plan, and phase views show version, status, provenance, validation, approval authority, and supersession.
- **Traceability rail:** a requirement links through outcome criterion, delivery phase, task packet, evidence requirement, evidence bundle, completion decision, and any memory candidate.
- **Decision queue:** compact, evidence-linked `DecisionRequest` packets expose options, authority, deadline, affected dependents, and the effect of no decision.
- **Phase and Kanban control:** Core-enforced WIP limits, readiness, protected-effect queues, and blocked reasons are visible; dragging a card submits a command, not a client-side state mutation.
- **Progress narrative:** summaries cite approved contract versions, Core state, artifacts, and gates; routine worker activity is collapsed without hiding uncertainty.
- **Evidence drawer:** every material claim exposes source, hash/version, producer, freshness, validation, classification, and policy outcome.
- **Safe intervention:** pause, return, defer, cancel, reprioritize, constrain, or request change through auditable commands with a visible blast radius.
- **Completion review:** “task done,” “phase closed,” “completion proposed,” and “mission accepted” are distinct states. No worker, subagent, brain, or Hermes component can accept its own output.

## Live-state design

The Studio subscribes to versioned query projections and event deltas from Core. It renders an explicit synchronization state: live, delayed, offline snapshot, reconnecting, or resync required. A client action is displayed separately as queued locally, submitted, accepted, rejected, awaiting approval, deferred, or reflected in durable state.

Offline clients may retain an untrusted intent for later submission, but they cannot declare it valid, reserve WIP, issue a capability, consume authoritative budget, or perform an effect. Core authenticates, checks expected revisions and policy, and accepts or rejects the command after reconnection.

High-volume worker and hook activity is summarized first and progressively expanded. The forensic Timeline retains causal links to approved contracts, task packets, model and skill versions, tool effects, probes, and evidence.

## Trust, explainability, and model independence

Every consequential affordance exposes:

| Display | Example |
| --- | --- |
| Actor and role | “Builder model profile B7 through Hermes Supervisor” |
| Governing authority | “Specification v3 / Outcome Contract v2 / Phase Plan v4” |
| Policy and approval | “Permitted by policy P12; approval D41 recorded by Core” |
| Capability and effect scope | “May write only task worktree T-42; network denied” |
| Evidence | “Targeted tests passed; independent review pending” |
| Uncertainty | “LSP impact incomplete; phase cannot close” |
| Model convergence | “Profile met the same outcome contract after two retries” |
| Time and freshness | Start, last heartbeat, occurrence time, projection time, and staleness |

“Different brains. Same engineering truth.” means supported models may take different implementation paths while facing the same approved contract and gates. The UI never implies byte-identical output or equal model capability. Cost, time, retries, context needs, and escalation frequency remain visible; the accepted quality bar does not change with model choice. Non-convergence is an honest blocked or escalated state, never a softened green result.

## Accessibility and inclusive design

The Studio targets keyboard-complete operation, semantic structure, visible focus, scalable typography, high-contrast themes, screen-reader labels for dynamic status, non-color-only risk indicators, reduced-motion support, localization-ready strings, and time-zone-aware timelines. Live announcements are batched and user-configurable so screen readers are not flooded by worker churn. Accessibility checks are a required quality gate.

## Error and degraded-mode UX

If a provider, worker, sandbox, integration, LSP adapter, retriever, memory index, or evidence validator is unavailable, the interface shows the affected contract, consequence, fallback, and next safe action. Missing data is not healthy data. Failure of a reporting channel never implies task or mission completion. Invalid plans, exhausted budgets, repeated no-progress loops, and unresolved uncertainty remain explicit until Core records an authorized transition.

## Design-system boundaries

Semantic tokens cover authority, approval, phase, Kanban, risk, policy, quality, freshness, evidence, convergence, and execution state. Theme or UI plugins may change presentation but cannot hide mandatory disclosures, relabel authoritative states, submit an effect outside the command API, or alter the meaning of a gate.

Detailed workflow and runtime contracts live in [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), and [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md). See also [12_Workspaces.md](12_Workspaces.md), [13_Mission_Control.md](13_Mission_Control.md), [17_Event_System.md](17_Event_System.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).
