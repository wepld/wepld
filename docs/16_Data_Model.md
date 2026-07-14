# 16 — Data Model

## Purpose, source of truth, and authority

This document defines canonical domain records for durable, explainable engineering delivery. It is normative for boundary semantics, not a physical database schema. The detailed governed-artifact schemas and transitions live in [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md).

Structured Core state is authoritative. Markdown, diagrams, reports, Git notes, and chat are projections, exports, or source artifacts; none is the sole source of specification, approval, workflow, budget, effect, or completion truth. Git is authoritative for the source snapshot/history it records, not for WePLD governance state.

Core alone commits durable governance/workflow truth, policy and approval records, capability issuance, budget accounting, transitions, effect intent/result, completion, and recovery. Other bounded contexts own domain interpretation and may submit commands or observations, but they do not write authoritative records directly.

## Governing hierarchy

Every executable instruction resolves this precedence chain and exact versions:

1. WePLD governance policy
2. Approved `EngineeringSpecification`
3. Approved `OutcomeContract`
4. Approved `DeliveryPlan`
5. Approved `PhasePlan`
6. `TaskPacket`
7. `ToolAction`

A lower record may narrow a higher envelope but cannot redefine requirements, acceptance, scope, policy, or authority. Core rejects or quarantines a contradictory record.

## Identity and common contract metadata

Every domain record has an opaque globally unique ID, type, schema version, creation/record/update times, organization/project/mission scope, classification, provenance, actor, correlation and causation IDs, and optimistic-concurrency revision where mutable. Display names are mutable aliases, never keys.

Every governed artifact additionally declares purpose, authority/trust class, required fields, lifecycle state, artifact version, validation status, proposer, approval authority and decision where applicable, exact upstream/downstream trace links, supersession, retention, and content hash or immutable body reference. “Authoritative” means Core recorded the required approval or observation under policy; “derived” and “untrusted” remain visibly distinct.

## Required governed artifacts

| Artifact | Purpose and minimum entity content | Lifecycle and authority | Required trace |
| --- | --- | --- | --- |
| `MissionCharter` | user outcome, boundaries, owner, priority, classification, initial budget/deadline | Draft → ClarificationRequired → ReadyForReview → Approved → Superseded/Cancelled; user/Brain may draft, only sponsor authority approves | principal/project → specification |
| `EngineeringSpecification` | **what** success means: functional/non-functional requirements, constraints, exclusions, assumptions, questions, acceptance criteria, verification bindings, risks, evidence needs | Draft → ClarificationRequired → ReadyForReview → Approved → Superseded → Completed/Cancelled; Brain/user may propose, only authorized principal approves; approved versions immutable | charter → outcome contract, plans, change request |
| `OutcomeContract` | contract-equivalence dimensions, acceptance thresholds, public/architecture/security constraints, regression and unresolved-risk limits | Draft → ReadyForReview → Approved → Active → Satisfied/Unsatisfied → Superseded/Cancelled; planning/quality may propose, required specification/quality/security authority approves | specification requirements → evidence requirements/completion |
| `DeliveryPlan` | Brain Agent delivery strategy, tailored phase graph, dependencies, requirement allocation, risks, estimates, budgets, verification, stop/escalation | Draft → ValidationFailed/ReadyForReview → Approved → Active → ChangePending → Superseded → Closed/Cancelled; Brain proposes and never approves; Core validates, authorized principal approves | specification/outcome → phase plans |
| `PhasePlan` | phase objective, entry/exit, dependencies, skills/tools, writable/forbidden scope, tasks, WIP, budget, controls, evidence, gate, escalation | Draft → Ready → Approved → Active → GateReview → Closed/Returned/Deferred/Cancelled; Brain or Hermes may propose; Core records delegated/human approval under policy | delivery plan → Task Packets/evidence/gate |
| `TaskPacket` | bounded builder/subagent objective, inputs/outputs, dependencies, acceptance/evidence, context references, capabilities, scopes, model/skill constraints, budget, stop rules | Draft → Validated → Ready → Leased → Active → Review → Verification → Done, with explicit exceptional states; Hermes derives/proposes from an approved Phase Plan, Core validates/authorizes, executor cannot redefine it | phase/requirements → attempts/actions/evidence |
| `RiskItem` | threat/opportunity, likelihood, impact, trigger, exposure, mitigation, contingency, owner, review date, residual decision | Proposed → Open → Mitigating → Monitoring → Accepted/Closed/Expired; revisions preserve history and high/critical residual acceptance needs named authority | charter/spec/plan/phase/evidence/decision |
| `Assumption` | explicit proposition, rationale, confidence, validation method, owner, expiry and consequence if false | Proposed → Validating → Confirmed/Invalidated/AcceptedRisk → Superseded; never silently treated as fact | specification/plan/risk/change |
| `DecisionRequest` | question, options, evidence, recommendation, authority, deadline, default/no-decision consequence, blocked dependents | Draft → Pending → Answered/Deferred/Expired/Cancelled; components request, Core opens/records, authorized principal answers | affected artifact/version → decision event/transitions |
| `ChangeRequest` | kind (`SpecificationChange` or `PlanChange`), rationale, requested delta, impact, risk, migration/rollback, replacement versions | Proposed → ImpactAnalysis → PendingApproval → Approved/Rejected/Withdrawn → Applied; WHAT changes create new specification/outcome versions, HOW-only changes replace plan/phase | old contracts → decision → replacements/tasks |
| `EvidenceRequirement` | claim/criterion, method, source type, independence, environment, threshold, freshness, validator, gate effect | Draft → Bound → Active → Satisfied/Failed/Waived → Superseded; authoritative when bound by approved outcome/plan/policy | requirement/outcome/phase/task → bundles/gates |
| `EvidenceBundle` | referenced artifacts, checks, observations, environment/tool/model versions, hashes, producer, confidence, gaps, validation | Collecting → Submitted → Validating → Validated/Rejected/Stale → Superseded; producer output is untrusted until validated; Core records satisfaction | requirement/action/task → completion proposal |
| `CompletionProposal` | scope/results summary, requirement-evidence matrix, gate state, residual risk, deviations, budget, recommended disposition | Draft → ReadyForReview → Submitted → Returned/Withdrawn/Decided; Hermes/Core completion evaluator may assemble, no executor may accept | specification/outcome/plan/phases/evidence → decision |
| `CompletionDecision` | exact proposal/version, decision (`Accept`, `Return`, `Defer`, `Cancel`), authority, rationale, conditions and follow-up | Recorded once then immutable; superseding decisions are new records; only authorized principal decides and Core records completion | proposal → mission state/memory candidates |
| `MemoryCandidate` | evidence-derived lesson, type/scope, sources, confidence, contradiction/freshness/expiry, proposed retrieval terms | Proposed → Judging → Approved/Rejected/Quarantined → Consolidated/Superseded/Expired; untrusted until Memory Judge and policy admit it | evidence/decision/retrospective → typed memory |
| `Retrospective` | outcome vs plan, delivery/evidence metrics, incidents, decisions, loop and model/skill performance, lessons and actions | Draft → Reviewed → Finalized → Archived/Superseded; derived analysis, not governance authority | completed/cancelled mission → risks, evaluations, memory candidates |

## Supporting execution and intelligence entities

| Entity | Purpose | Authority relationship |
| --- | --- | --- |
| Organization / Principal / Project | identity, repository and policy scope | authenticates commands and approval authority |
| Phase Runtime | current phase state, WIP, budget, gate and active plan version | Core projection of an approved Phase Plan |
| Task / Kanban Record | operational task state and dependency node | Core transitions under Task Packet and WIP rules |
| Attempt / Lease / Hermes Session | execution history, supervisor and worker allocation | Core issues/revokes lease; participants report observations |
| Brain / Builder / Subagent Invocation | selected profile, request/context/output schema, usage and result | proposal/evidence only; never approval |
| Skill / Hook / Plugin Package | versioned executable capability and trust evidence | Registry stages; Core approves/activates/revokes |
| Policy Rule / Policy Decision / Approval | rule, evaluation, and authorized human/enterprise decision | separate immutable records; not a combined boolean |
| Capability | scoped permission for one subject/action/resource/envelope | issued and revoked only by Core |
| `ToolAction` / Effect Result | proposed action, durable intent, observed postcondition and evidence | boundary executes; Core records intent/result |
| Artifact | immutable content-addressed body or stable Git/external reference | payload/evidence carrier, not acceptance by itself |
| Check / Finding / Gate | reproducible validation, issue and transition constraint | validators report; Core derives/records gate state |
| Typed Memory Record | Working, Mission, Engineering, Skill, Provider/Model Performance, Governance | trust, scope, freshness and authority differ by type |
| Context Pack / Retrieval Item | minimal task context and per-item provenance/trust/ranking | derived, reproducible input; cannot override governance |
| Loop Iteration / Hook Invocation | hypothesis/action/result/confidence and typed lifecycle hook | bounded operational evidence, selectively durable |
| Evaluation Run / Ablation Cell | fixed fixture/configuration, varied component, metrics and outcome | controlled evidence for profile/harness certification |
| Event | immutable typed domain fact recorded by Core | ledger source for projections and audit |

## Phase, Kanban, and WIP model

Phase lifecycle is `Pending`, `Ready`, `Active`, `Blocked`, `Review`, `Verification`, `Closed`, `Returned`, `Deferred`, `Uncertain`, or `Cancelled`. Task flow is `Backlog → Ready → InProgress → Review → Verification → Done`, with `Blocked`, `NeedsClarification`, `NeedsApproval`, `Returned`, `Deferred`, `Uncertain`, and `Cancelled` exceptional states.

State transitions carry expected revision, actor, governing contract versions, policy/approval references, gate/evidence references, and reason. `Done` means the Task Packet’s required evidence passed; it does not accept a phase or mission. `Closed` means phase exit conditions and gate passed; it does not create a Completion Decision.

WIP policies define scope, counter, limit, admission rules, expiry, and exception authority. Initial constraints are at most one writable implementation task in an isolated worktree unless Core proves disjoint writable scopes, bounded read-only research, bounded unresolved decisions, and bounded pending protected effects. Core owns counters and admission; Hermes schedules eligible work within them.

## Record invariants

- An approved specification, outcome, delivery plan, or phase plan is immutable; a change creates a typed request and replacement version.
- A Specification Change Request changes WHAT; a Plan Change Request changes only HOW. Core rejects a mislabeled request whose impact crosses that boundary.
- Brain Agent proposals require validation and independent approval. Hermes, builders, workers, subagents, tools, and model providers never approve plans, effects, evidence, or completion.
- A Task Packet has one owning phase/version, bounded inputs/outputs, capabilities, evidence requirements, and writable/forbidden scope. An executor may return it as invalid but cannot rewrite it.
- An attempt has at most one active lease. Retries preserve causation, hypothesis, budget and idempotency relationships.
- An artifact records producer, provenance, classification, retention and content identity. An Evidence Bundle additionally binds artifacts to a claim and validation.
- Completion requires a validated proposal followed by an authorized Completion Decision; worker success events are insufficient.
- Governance Memory remains authoritative within its valid scope. Other memory cannot supersede policy, approved specifications, ADRs, or exact current source evidence.

## Data access, storage, and retention

UI, CLI, MCP, APIs, brain adapters, Hermes, workers, plugins, and integrations access data through scoped command/query/artifact ports. Database tables, local files, model context caches, and secrets are never public extension APIs.

V1 uses a local transactional operational database for Core records and event ledger, a content-addressed artifact store for large immutable bodies, Git for source history, and derived exact/full-text/structural/semantic indexes. The database has one local Core writer; WAL is a local optimization, not network-shared storage. Raw telemetry and large logs use bounded stores rather than event-sourcing every byte.

Commands carry expected revisions and/or idempotency keys. Workflow state is never CRDT-merged. Retention uses tombstone, redaction, and supersession references so ordinary retrieval can exclude expired/corrected data while preserving policy-permitted audit identity and proof. Legal hold and enterprise retention override ordinary expiry.

## Classification

The baseline taxonomy is Public, Internal, Confidential, Restricted, and Secret. Classification travels with artifacts, context items, memory, mission contracts, model/tool requests, events, and exports. Policy maps it to allowed storage, profiles, channels, retrieval, retention, export, and redaction. Secret is reference-only by default and must not appear in ordinary prompts or artifacts.

See [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [08_Knowledge_System.md](08_Knowledge_System.md), [14_Security_Architecture.md](14_Security_Architecture.md), [17_Event_System.md](17_Event_System.md), and [18_API_Architecture.md](18_API_Architecture.md). The governing record decisions are Proposed ADRs 0015–0017 and 0020; their Proposed status does not authorize implementation.
