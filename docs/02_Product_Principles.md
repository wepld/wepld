# 02 — Product Principles

## Purpose

These principles are binding decision rules. A feature, provider, skill, or shortcut that conflicts with one requires an explicit architecture decision; no lower-level artifact or model may silently redefine the rule.

| Principle | Operational rule |
| --- | --- |
| WePLD provides the method | The user supplies an outcome; Core supplies governed specification, planning, phase flow, change control, verification, and completion semantics. |
| Core is the authority | Core alone owns durable truth, policy, approvals, capabilities, budgets, transitions, completion, and recovery. |
| Proposals are not authority | Brain Agent plans, Hermes supervises, and builders act within packets; none can approve its own proposal or perform an unmediated effect. |
| Local whenever possible | A core mission must not require a hosted WePLD control plane; external model and notification calls are explicit dependencies. |
| Brains are replaceable | Domain code sees a stable reasoning contract and named brain profile, never a vendor SDK. |
| Builders are replaceable | A builder profile is a measured execution choice, not a change to scope, policy, or the quality bar. |
| Skills are executable and reusable | Repeatable expertise is a versioned, testable procedure with capabilities, outputs, failure modes, verification, and evidence contracts. |
| Knowledge accumulates with evidence | Retained lessons carry provenance, scope, confidence, freshness, classification, and supersession; unverified claims do not become engineering memory. |
| Governance outranks memory | Approved policy, specifications, contracts, plans, and ADRs are authoritative Core records, not optional retrieval advice. |
| One control plane | Only Core changes governed mission, specification, plan, phase, task, approval, budget, and completion state. |
| One human-facing agent | Only Messenger acts as an agent communicating with people; CLI, Studio, MCP, and APIs remain authenticated Core command surfaces. |
| Evidence before assertion | Completion, quality, risk, and memory claims require linked, reproducible evidence. |
| Same engineering truth | Supported models may differ in route and effort, but accepted outputs satisfy the same OutcomeContract and gates. |
| Least privilege by default | Tools, paths, network, secrets, integrations, and model egress are scoped Core-issued capabilities, not ambient authority. |
| Observable by construction | Each effect records actor, intent, classification, policy outcome, capability, approval, actual result, evidence, and trace identity. |
| Reversible by design | Prefer worktrees, preview, snapshots, versioning, rollback, and staged release over irreversible change. |
| Bounded flow | Phase WIP, writable concurrency, unresolved decisions, retries, protected effects, and budgets are policy-enforced limits. |
| Extensibility without bypass | Skills, hooks, plugins, and integrations use typed contracts and cannot bypass policy, audit, or tool boundaries. |

## Foundational authority hierarchy

The governing precedence is:

1. WePLD governance policy
2. Approved `EngineeringSpecification`
3. Approved `OutcomeContract`
4. Approved `DeliveryPlan`
5. Approved `PhasePlan`
6. Core-issued `TaskPacket`
7. Core-authorized `ToolAction`

Every artifact records its upstream version and trace links. A lower layer may refine HOW within its envelope, but may not change WHAT, acceptance criteria, exclusions, policy, or authorized scope. Conflict makes the lower artifact invalid and forces clarification, change control, or escalation.

## Separation of responsibilities

| Component or role | May do | Must not do |
| --- | --- | --- |
| User / designated authority | state outcomes; approve specifications and plans; decide protected changes and completion | bypass non-waivable policy or treat a model claim as evidence |
| Core | validate and persist governance artifacts; issue capabilities; authorize effects; enforce budgets, transitions, gates, completion, and recovery | delegate governance truth to a model, plugin, or UI |
| Brain Agent | clarify, architect, plan, decompose, analyze risk, and propose replans | approve its own plan, execute tools, or mutate durable state |
| Brain profile / model | produce provider-neutral structured reasoning results | gain tools, mission authority, or provider-specific domain semantics |
| Hermes Supervisor | route skills, models, subagents, context, loops, and verification within an approved envelope | become a provider, redefine approved artifacts, authorize effects, or claim completion |
| Builder model | implement a bounded TaskPacket and propose typed actions | redefine scope or acceptance, contact a user, or execute an effect directly |
| Explorer / reviewer / QA / security subagent | return scoped artifacts, findings, and evidence | approve plans, effects, exceptions, or completion |
| Policy module | evaluate rules and propose/record Core policy decisions | perform an effect |
| Tool boundary | perform an authorized scoped effect and probe its actual result | invent scope, approve itself, or infer success from model text |
| Messenger | report projections and collect authenticated intent and decisions | grant privileges or mutate project state directly |

## Change and approval invariant

An approved specification is immutable. A change to WHAT is required creates a `ChangeRequest(kind=SpecificationChange)` and, if approved, a new `EngineeringSpecification` and affected contracts. A change only to HOW creates a `ChangeRequest(kind=PlanChange)` and new affected plan versions. Evidence may challenge a plan, but cannot silently rewrite it.

The Brain Agent may propose specification, outcome, delivery, phase, risk, decision-request, and change artifacts for which it is qualified. Hermes may recommend operational adaptations. Core validates and records the proposal; only the designated authority may supply an approval when policy requires one. No autonomy mode converts self-proposal into self-approval.

## Product non-goals

- Simulating a company through agent chat transcripts or an uncontrolled swarm.
- Treating raw model output as a command, test result, governance decision, or source of completion truth.
- Byte-identical output across models or claims that weak and strong models have equal capability.
- Lowering acceptance gates because a selected model is cheaper, slower, or less capable.
- Sending proprietary context externally without a recorded policy decision.
- Giving every worker or skill every capability for convenience.
- Autonomous production deployment, an open marketplace, or a cloud-first control plane in the first increment.
- Building a full graphical IDE before operational contracts are proven.
- Retention that overrides deletion, contractual, legal, or incident-response obligations.

## Decision and uncertainty rules

Safety, legal, compliance, and non-bypassable policy remain absolute. Within those constraints, the authority hierarchy determines which artifact wins. Conflicting evidence is preserved and escalated; it is never resolved by selecting the most confident model statement. If Core cannot evaluate a policy decision, it pauses the affected action rather than unrelated work.

Uncertainty is a first-class state. A supported profile that cannot converge retries only under a named hypothesis and fixed budget, then specializes, splits, seeks independent review, replans, switches profile, requests authority, or stops honestly.

## Autonomy invariant

Every mode permits policy-compliant reading, planning, and internal analysis. Every mode retains hard gates for secret access, protected branches, external transfer, destructive operations, scope expansion, dependency installation, production deployment, and other classified effects. “Full Autonomous” means autonomous operation inside a declared envelope; it never means unbounded authority.

## Adoption test

A proposal is acceptable only if it identifies: user outcome; owning Core context; governing artifact version; required capabilities and data classification; phase and TaskPacket scope; emitted events and evidence; WIP and budget effect; failure and recovery behavior; and how it can be replaced without altering mission semantics or acceptance truth.

See also: [14_Security_Architecture.md](14_Security_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), [25_Development_Guidelines.md](25_Development_Guidelines.md), [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), and [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md).
