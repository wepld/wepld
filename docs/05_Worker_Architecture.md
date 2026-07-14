# 05 — Worker Architecture

## Definition and organization

A worker is an executable organizational role, not a language model. A worker attempt is composed of:

**role policy + selected brain/builder profile + pinned skills + compiled context + Core-issued TaskPacket + scoped capabilities + durable lease + isolated workspace.**

Hermes is the **Engineering Intelligence Runtime and Supervisor** for these attempts. It maintains bounded operational state, routes work, and coordinates evidence, but it is not a provider, does not own governance truth, cannot issue capabilities, and cannot approve plans, effects, or completion. If Hermes restarts, it reconstructs its bounded state from Core-issued artifacts and durable events.

The Brain Agent is a separate planning role: planner, architect, risk analyst, and replanner. It proposes governed delivery artifacts through replaceable brain profiles but never performs effects or approves its own plan. Builder models consume bounded TaskPackets and return artifacts, uncertainty, and typed proposed actions. Only mediated tool boundaries perform Core-authorized effects.

## Initial role taxonomy

| Role | Primary output | Default authority |
| --- | --- | --- |
| Brain Agent | clarification, specification/plan proposal, risk analysis, controlled replan | read approved context; propose only |
| Repository Explorer | cited repository map, affected artifacts, unknowns | bounded read-only exploration |
| Architecture Analyst | constraint and dependency findings, design options | read-only; no plan approval |
| Builder / Implementer | isolated change artifact and implementation evidence | one TaskPacket, one writable worktree, approved tools |
| Test Engineer / QA | test plan, results, reproducible gate evidence | read/test sandbox; no completion authority |
| Reviewer | independent findings and dispositions required | read artifact/worktree; no effect or completion approval |
| Security Reviewer | threat and policy findings, remediation proposal | read-only unless separately leased remediation exists |
| Performance Reviewer | baseline, measurements, regression findings | bounded benchmark environment |
| Documentation Agent | traceable documentation artifact | scoped writable documentation packet |
| Recovery Investigator | observed state, uncertainty classification, recovery recommendation | read/probe capabilities only by default |

Additional language, platform, compliance, accessibility, UX, database, release, and research roles are declarative profiles over the same contract, not independent authorities or binaries.

## TaskPacket contract

Every execution attempt is governed by a Core-issued, versioned `TaskPacket` containing at least:

| Field | Requirement |
| --- | --- |
| Identity and trace | mission, specification, OutcomeContract, DeliveryPlan, PhasePlan, task, attempt, correlation, and causation identifiers |
| Objective | one bounded deliverable stated without redefining higher-level intent |
| Inputs | immutable artifact/context references with provenance and trust labels |
| Scope | explicit writable and readable paths plus forbidden scope |
| Skills and tools | pinned versions, compatibility, allowed tools, and capability ceiling |
| Budget | time, tokens, spend, tool calls, retries, and deadline |
| Dependencies | satisfied prerequisites and declared consumers |
| Acceptance and evidence | requirement bindings, verification procedure, evidence schema, and quality/security gates |
| Risk controls | risk class, effect classifications, approval points, rollback/snapshot requirements |
| Stop and escalation | uncertainty, repeated failure, budget, invalid plan, authority, and safety conditions |

A packet that conflicts with policy or an approved higher artifact is invalid. Hermes and a builder may request a corrected packet or controlled plan change; they may not reinterpret it.

## Worker contract

Every worker adapter supports this lifecycle:

1. **Advertise:** register runtime version, roles, skill/tool/sandbox support, limits, and health.
2. **Lease:** accept only a signed, expiring lease with a valid TaskPacket, declared inputs, cancellation channel, and Core-issued capabilities.
3. **Compile context:** Hermes assembles a minimal, provenance-labelled context pack and records why each item was selected.
4. **Plan actions:** the builder or subagent emits schema-valid reasoning, artifacts, uncertainty, and typed proposed actions; none is an effect.
5. **Authorize and execute:** Core classifies the proposed action, applies policy/capability/budget/approval checks, records durable intent, and dispatches an authorized action to the tool boundary.
6. **Probe and report:** the tool boundary observes the actual result; the worker links logs, diagnostics, hashes, measurements, and citations.
7. **Finish or fail:** return a terminal attempt outcome, unresolved uncertainty, and evidence; release sandbox resources and lease.

Hermes-compatible protocol messages are normalized into this contract. “Hermes-compatible” describes protocol compatibility, not governance ownership or a model provider.

## Attempt lifecycle and recovery

~~~mermaid
stateDiagram-v2
  [*] --> Registered
  Registered --> Eligible: role + policy + compatibility
  Eligible --> Leased: Core issues packet + capability
  Leased --> Running: heartbeat accepted
  Running --> AwaitingAuthority: classified action or decision required
  AwaitingAuthority --> Running: Core permits
  Running --> Succeeded: artifacts + required evidence returned
  Running --> Failed: classified error
  Running --> Uncertain: actual effect state cannot yet be proved
  Running --> Cancelled: cancellation acknowledged
  Leased --> Expired: heartbeat timeout
  Expired --> Uncertain: inspect observable state
  Uncertain --> Eligible: Core authorizes safe retry/reassignment
  Succeeded --> [*]
  Failed --> [*]
  Cancelled --> [*]
~~~

Worker-attempt state is not task Kanban state and never closes a phase or mission. A lost worker is not assumed to have failed cleanly. Core records uncertainty, uses Recovery Investigator/tool probes to inspect observable state, and alone decides retry, reassignment, return, or escalation.

## Isolation, capabilities, and writable concurrency

Each writable attempt receives an isolated Git worktree or equivalent snapshot, filesystem allowlists, CPU/memory/time quota, network policy, and only the secrets required by its packet. Untrusted code has no network by default, never sees the user home directory, and cannot write the primary worktree. Platform sandbox limitations remain visible risk evidence.

Initial WIP rules are:

- one writable implementation task per isolated worktree;
- bounded parallel read-only exploration with non-overlapping objectives;
- writable tasks that touch shared artifacts are dependency-ordered or isolated before reviewed integration;
- bounded unresolved DecisionRequests and pending protected effects;
- Core-enforced limits, never display-only Kanban metadata.

## Skill routing and scheduling

Hermes proposes a route using role compatibility, required skills, data classification, tool support, locality, health, historical reliability, model/skill evaluation, budget, task risk, and verification level. The route selects a pinned skill set, brain or builder profile, subagent role, context recipe, tools, and budget allocation. Core verifies the route against the TaskPacket and issues only the allowed subset.

Routing is evidence-driven and policy-replaceable. A cheaper or weaker builder does not receive a weaker OutcomeContract or quality threshold. If it cannot converge inside the envelope, Hermes follows the escalation ladder rather than manufacturing success.

## Structured subagent handoffs

Subagents receive one objective, scoped context, skills, tools, capabilities, budget, deadline, output schema, and evidence requirements. They do not chat freely or approve one another. Their communication path is:

`subagent → typed finding/artifact → Hermes Supervisor → Core-recorded result when durably relevant`.

Read-only exploration may run in bounded parallel. Writable work remains isolated and conflict-controlled. Independent review should not rely only on the same context or model that produced the implementation.

## Completion and escalation

A worker can report an attempt outcome; Hermes can assemble a `CompletionProposal`; neither can mark a task, phase, or mission complete. Core evaluates evidence and transition rules. An authorized `CompletionDecision` accepts, returns, defers, or cancels the mission.

Escalation is required for scope ambiguity, new requirements, invalid governing versions, permissions, dependencies, confidential-data exposure, budget breach, repeated/no-progress loops, contradictory evidence, protected Git effects, deployment, or any action requiring human authority.

## Acceptance criteria

- Worker and model adapters are replaceable without changing TaskPacket or mission semantics.
- Every effect traces to a packet, attempt, policy decision, capability, durable intent, actual-result probe, and evidence.
- No worker, model, subagent, or Hermes component has a direct user channel, durable mission authority, or ambient project access.
- A killed worker can be recovered without silently duplicating a non-idempotent action.
- Writable parallelism and uncertainty remain explicit and Core-governed.

See also: [06_Brain_Architecture.md](06_Brain_Architecture.md), [10_Loop_Engineering.md](10_Loop_Engineering.md), [14_Security_Architecture.md](14_Security_Architecture.md), [17_Event_System.md](17_Event_System.md), and [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md).
