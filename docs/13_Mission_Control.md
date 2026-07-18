# 13 — Mission Control

## Purpose and authority

Mission Control is the live operational dashboard for WePLD’s governed engineering organization. It is not a decorative agent monitor and it owns no workflow state. It derives from Core-recorded contracts, events, leases, evidence, resource telemetry, WIP counters, budgets, and policy state to answer: what is approved, what is running, what is at risk, what is waiting, and which authorized decision is required?

Core is the sole authority for durable policy, approvals, capabilities, budgets, transitions, effect intent and result, completion, and recovery. The Brain Agent proposes governed plans; Hermes supervises execution within approved phase and WIP envelopes; builders and subagents produce actions, artifacts, findings, and evidence; tool boundaries perform authorized effects. Mission Control renders those facts and submits typed commands.

## Required views

| View | Required signals | Interpretation rule |
| --- | --- | --- |
| Governing contracts | charter, approved specification/outcome/plan versions, pending changes | no lower artifact may redefine a higher contract |
| Current mission | outcome, owner, status, active phase, autonomy mode, budget, classification | mission state is a versioned Core projection |
| Phase graph | objective, entry/exit conditions, dependencies, WIP, budget, gate | only approved Phase Plans may become active |
| Kanban | task state, Task Packet version, dependencies, lease, blocked reason | movement occurs only after a Core-recorded transition |
| Worker board | Hermes session, builders/subagents, role, task, heartbeat, resources | “running” requires an active lease and recent heartbeat |
| Execution Console | correlated plan/task/worker/tool status, terminal/process excerpts, budgets, retries, traces, effect/evidence links | operational events and telemetry are views; Core ledger/evidence remains truth |
| Quality and evidence | requirements, bundles, checks, review, tests, security/accessibility/benchmark gates | status is per binding and gate, never one unverifiable score |
| Resource and cost | CPU, memory, disk, provider/model usage, spend vs budget | measured, estimated, reserved, and unavailable values remain distinct |
| Decisions | Decision Requests, authority, options, deadline, blocked dependents | a decision is valid only when Core records an authorized response |
| Risks and assumptions | trigger, exposure, mitigation, owner, residual state, review date | accepted risk does not erase the underlying evidence |
| Change requests | specification vs plan kind, impact, status, replacement versions | approved work stays bound to old versions until change approval |
| Effects | proposed, awaiting policy/approval, intended, executing, probed, evidenced | intent is not result; result is not acceptance |
| Completion | outcome trace, unresolved risk, evidence completeness, proposal/decision | task Done and phase Closed do not mean mission Accepted |
| Timeline | causal state changes, approvals, actions, outputs, evidence | uses immutable event identity and authorized artifact references |

## Phase and Kanban flow

A phase uses `Pending`, `Ready`, `Active`, `Blocked`, `Review`, `Verification`, `Closed`, `Returned`, `Deferred`, `Uncertain`, or `Cancelled`. Within it, task flow is:

`Backlog → Ready → InProgress → Review → Verification → Done`

with exceptional states `Blocked`, `NeedsClarification`, `NeedsApproval`, `Returned`, `Deferred`, `Uncertain`, and `Cancelled`.

Core enforces policy-configured WIP and readiness. Initial defaults are one writable implementation task per isolated worktree, bounded parallel read-only exploration, bounded unresolved Decision Requests, and bounded pending protected effects. Hermes may select and schedule eligible work but cannot override counters, admit an invalid task, or record a transition. A card never enters `Done` solely because a worker says it finished; required evidence must be validated and Core must accept the transition.

## Health model

Project health is a transparent vector rather than an opaque score:

- **Governance:** specification/plan approval, traceability gaps, pending change requests, decision latency.
- **Delivery:** phase and critical-path progress, blocked duration, WIP pressure, schedule and budget variance.
- **Quality:** mandatory evidence and gate status, test reliability, review backlog, regression state.
- **Security:** finding severity, policy exceptions, protected effects, dependency/secret/supply-chain posture.
- **Operations:** worker health, loop progress, sandbox integrity, CPU/memory/disk pressure, provider availability.
- **Convergence:** supported-profile outcome-equivalence status, attempts, escalation, and honest non-convergence.
- **Knowledge:** applicable governance records and verified memory freshness, contradiction, and source status.

An optional summary is the worst applicable dimension with an explicit rationale and confidence. A green summary is impossible when a required specification decision, evidence binding, high/critical finding, protected-effect approval, or completion criterion remains unresolved. “No data” is never healthy.

## Alert and escalation policy

Alerts are typed by contract and materiality. Examples include: WIP admission denied; phase entry condition lost; repeated/no-progress loop detected; invalid plan or stale Task Packet; worker heartbeat delay; uncertain real-world effect after a crash; evidence conflict; context or memory provenance failure; model non-convergence; secret detection; budget exhaustion; or a Decision Request nearing expiry.

Deduplication, escalation, acknowledgment, suppression reason, and resolution are Core-recorded. Messenger may route a redacted notice, but neither Messenger nor a worker can resolve the underlying state. Suppression requires authorized rationale and expiry.

## Data freshness and correctness

Every widget displays the governing record version, last domain-event time, projection update time, source availability, and any expired cursor or evidence. Telemetry may be eventually consistent, but contract, phase, task, policy, budget, capability, decision, effect, and completion states use Core’s durable ledger and records. A projection rebuild must reproduce the same authorized state from retained events, snapshots, and referenced artifacts, or show an explicit recovery gap.

## Control actions

Mission Control exposes named commands such as pause/resume/cancel, request clarification, reprioritize within approved bounds, submit a Specification or Plan Change Request, acknowledge an alert, propose a budget change, answer a Decision Request, return a phase, request a report, challenge evidence, or decide a Completion Proposal when authorized.

It cannot directly terminate a worker process, rewrite an approved artifact, force a Kanban state, delete evidence, increase a budget, issue a capability, bypass a release gate, merge, deploy, or mark a mission complete. Those operations follow their own policy-controlled Core path and Effect Firewall.

## Operational acceptance criteria

- Every material status links to its Core projection, governing contract version, and evidence inputs.
- A user can explain why a phase or task is blocked and identify the dependency, decision, policy, WIP limit, or missing evidence.
- A user can distinguish a proposed plan, approved plan, executing phase, completed task, Completion Proposal, and accepted Completion Decision.
- Resource and cost data differentiates measurement, estimate, reservation, and unavailable state.
- Losing the Studio does not stop Core or transfer authority to Hermes; reconnecting cannot lose or invent authoritative state.
- The same state and commands are observable through the CLI, Studio, MCP adapter, and API subject to authorization.
- Execution Console plan/task/tool/trace rows resolve to Core correlation and artifact/evidence identities; a missing, sampled, stale, or contradictory telemetry signal is labeled and cannot override the ledger.

See [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [03_System_Architecture.md](03_System_Architecture.md), [07_Messenger_Agent.md](07_Messenger_Agent.md), [10_Loop_Engineering.md](10_Loop_Engineering.md), and [27_Performance_Goals.md](27_Performance_Goals.md).
