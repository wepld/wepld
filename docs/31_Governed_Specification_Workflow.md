# 31 — Governed Specification Workflow

**Status:** Proposed architecture for review. This document defines target product contracts; it does not authorize implementation or the merge of Draft PR #1.

## Product contract

WePLD supplies the engineering method. The user supplies the desired outcome. The Brain Agent produces a `PlanProposal`; qualification and an authenticated Core-recorded `PlanDecision` create the governed Delivery Plan. Hermes operates the bounded engineering organization. Builder models execute approved task packets through mediated actions. WePLD Core governs every transition and effect. Evidence determines whether the outcome is acceptable.

The user is not expected to bring an external project-management or specification method. CLI, Studio, MCP, and APIs are surfaces over this same Core workflow:

1. **Describe** the outcome, constraints, urgency, and context.
2. **Clarify** material ambiguity, assumptions, exclusions, risks, and evidence needs.
3. **Review and approve the Engineering Specification.**
4. **Qualify and decide the proposed Delivery Plan** through deterministic normalization, structural validation, risk-proportionate assessment, and authenticated approval.
5. **Execute phase by phase** under approved Phase Plans.
6. **Observe Kanban flow and evidence** without operating low-level tools.
7. **Resolve real decisions and controlled changes.**
8. **Review verified completion** against the Outcome Contract.
9. **Accept, return, defer, or cancel.**
10. **Consolidate approved engineering memory** from evidence-derived candidates.

## Authority hierarchy

The hierarchy is strict and monotonic. A lower layer may narrow or operationalize a higher layer; it may not silently broaden, weaken, or reinterpret it.

| Rank | Authority layer | Binding rule |
| --- | --- | --- |
| 1 | WePLD governance policy | Non-bypassable safety, legal, security, data, budget, and approval rules. |
| 2 | Approved `EngineeringSpecification` | Defines **what** outcome is required and excluded. An approved version is immutable. |
| 3 | Approved `OutcomeContract` | Defines contract-equivalent success, verification bindings, evidence, gates, and risk tolerance. |
| 4 | Approved `DeliveryPlan` | Defines **how** the outcome will be delivered through a traceable phase graph. |
| 5 | Approved `PhasePlan` | Defines the authorized objective, work, tools, skills, scope, WIP, budget, controls, and exit gate for one phase. |
| 6 | Authorized `TaskPacket` | Gives one builder or subagent one bounded objective and acceptance/evidence obligations. |
| 7 | Authorized `ToolAction` | Is the smallest proposed effect; the Effect Firewall still decides whether and how it occurs. |

Core is the only authority that records approvals, issues capabilities, commits transitions, accounts budgets, authorizes effects, and establishes recovery or completion state. The Brain Agent may propose specifications and `PlanProposal` records but never approve them. Hermes may orchestrate and execute approved work but never own governance truth. Builders and subagents may return artifacts, findings, and evidence but never redefine scope or accept a mission.

## Structured durable state

Markdown is a review and export projection, not the operational source of truth. Every domain artifact uses a common durable envelope:

`id`, `schema_version`, `artifact_version`, `status`, `mission_id`, `project_id`, `classification`, `revision`, `created_at`, `created_by`, `content_hash`, `provenance[]`, `supersedes?`, `correlation_id`, and `causation_ref`.

Core stores authoritative versions and their transitions. Content-addressed bodies, Git references, Markdown renders, and search indexes are referenced derivatives. Approved versions are never updated in place; a change creates a new version and an explicit supersession edge.

### Domain artifact catalogue

| Artifact | Purpose and required domain fields | Authority / trust class | Traces to |
| --- | --- | --- | --- |
| `MissionCharter` | Captures sponsor intent: objective, user outcome, business value, owner, urgency, constraints, initial scope, exclusions, budget/deadline class, autonomy envelope, stakeholders, open questions. | Authoritative only after user/sponsor approval; drafts are proposals. | User intent → `EngineeringSpecification`. |
| `EngineeringSpecification` | Defines WHAT: objective, user outcome, functional and non-functional requirements, constraints, explicit exclusions, typed assumptions, open questions, acceptance criteria, verification bindings, risk classification, evidence requirements, provenance, version, status. | Authoritative at `Approved`; an approved version is immutable. | Charter → requirements → Outcome Contract. |
| `OutcomeContract` | Defines acceptance equivalence: criterion bindings, observable behavior, public contracts, architecture/security/quality constraints, regression rules, evidence thresholds, unresolved-risk ceiling, allowed variation. | Authoritative after approval; never relaxed by model selection. | Specification criteria → evidence requirements and completion. |
| `PlanProposal` | Defines a proposed delivery strategy, tailored phase graph, requirement mapping, dependencies, risks/mitigations, skills/tools, scopes, verification strategy, estimates, budgets, decision points, stop/escalation conditions, assumptions and uncertainty. | Untrusted Brain/architect proposal; never approval and never executable authority. | Specification + Outcome Contract + exact planning context → candidate DeliveryPlan. |
| `PlanAssessment` | Records specification/outcome coverage, acceptance/evidence sufficiency, DAG validity, architecture fitness, proportionality, risk, budget/WIP feasibility, rollback/recovery, assumptions/uncertainty, alternatives, reviewer identity/independence, blockers and readiness. | Initial version derives from deterministic validation; if reviews are required it remains `ReviewRequired`, then Core finalizes a new `Ready` version bound to the exact immutable review records; cannot approve a plan. | PlanProposal + candidate → initial assessment + exact reviews → finalized assessment → PlanDecision. |
| `PlanDecision` | Binds exact candidate/assessment versions and hashes, exact policy/risk-tier version, required independent-review record IDs/versions/hashes, disposition (`Approve`, `Return`, `Defer`, `Reject`), authenticated actor/authority, rationale, conditions and timestamp. | Authoritative only when Core validates authority, review independence and every exact binding before recording it; model votes/recommendations have no authority. | Assessment + exact reviews → approved DeliveryPlan or explicit non-approval disposition. |
| `DeliveryPlan` | Defines the normalized and assessed delivery strategy and phase graph named by a PlanDecision. | Authoritative only when the exact candidate receives a Core-recorded approving PlanDecision. | Specification + Outcome Contract + proposal/assessment/decision → Phase Plans. |
| `PhasePlan` | Authorizes one phase: objective, entry/exit conditions, dependencies, skills, tools, writable/forbidden scopes, task set, WIP limits, budget, risk controls, evidence requirements, gate, escalation conditions. | Derived-governed and binding after Core records delegated or human approval. | Delivery Plan phase → Task Packets and phase gate. |
| `TaskPacket` | Gives one objective, requirement refs, inputs, context manifest, skill/model/subagent profile, allowed tools/capabilities, writable/forbidden scope, budget/deadline, expected outputs, acceptance/evidence schema, stop conditions. | Derived-governed; Hermes proposes, Core validates and authorizes. | Phase Plan task → actions, artifacts, and evidence. |
| `SOPGraph` | Deterministically compiles exact approved DeliveryPlan/PhasePlan/TaskPacket versions and hashes into RoleNodes, contracts, typed control edges, compiler/schema version and graph hash. | Derived execution projection; Core alone validates/activates it and it cannot replace or broaden a parent. | exact approved parents → assignments, projections, results and evidence. |
| `RoleNode` | Binds graph role, objective/task, profile requirements, permissions/capabilities, budget/WIP, isolation and component-contract refs. | Core-projected operational state inside one active SOPGraph; no independent authority. | SOPGraph/TaskPacket → input/action/output contracts. |
| `ActionContract` | Binds allowed proposed actions, tool/resource/parameter limits, pre/postconditions, idempotency, scopes, budget and denial behavior. | Derived restriction only; it cannot mint a capability or authorize an effect. | RoleNode + capability policy → ToolAction/result. |
| `InputSubscription` | Binds allowed event/artifact types/IDs, filters, purpose, classification/redaction, cursor/replay bounds, expiry and revocation. | Authorized only by Core; subject role cannot create, widen or renew it. | RoleNode/TaskPacket → RoleInputProjection. |
| `RoleInputProjection` | Immutable Core-produced event/artifact snapshot/batch with subscription/version, source refs/hashes, cursor, redactions/omissions and projection hash. | Derived context, never authority; Core is the only producer. | authorized subscription + ledger/artifacts → one RoleNode. |
| `OutputContract` | Binds output schemas, evidence/provenance, destination, size/classification limits, completion/stop semantics and rejection handling. | Derived restriction under an active SOPGraph; output is untrusted until Core validation. | RoleNode/TaskPacket → artifacts, findings and results. |
| `SOPControlEdge` | Typed `Dependency`, `Evidence`, `Stop`, or `Escalation` edge with endpoints, condition/fact, deadline/budget and policy ref. | Core alone evaluates authoritative state; a role can only report an observation. | SOPGraph components → scheduling, gates, stop/escalation. |
| `MissionExplorationBranch` | Read-only parent/objective branch with ContextPack hash, permissions, budget, findings/evidence, status and accepted-or-rejected contribution. | Core-authorized bounded exploration; contribution remains untrusted until separately accepted. | mission/phase/task/context → proposal, assessment, memory or evaluation input. |
| `CompactionRecord` | Records source/session/branch hashes, covered range, retained summary, explicit omissions, compiler/model version, and mandatory authority rehydration set/hash. | Derived convenience only; never authority and unusable on resume until current governing records are rehydrated. | source context/events/artifacts → resumed session plus exact authority. |
| `RiskItem` | Tracks risk statement, category, likelihood, impact, exposure, triggers, owner, mitigation, contingency, residual risk, review date, blocking flag, evidence refs. | Authoritative control record once validated; cannot redefine higher-layer scope. | Any artifact/phase → decision, control, or gate. |
| `Assumption` | Records statement, source, scope, confidence, owner, validation method/deadline, consequence if false, and evidence. | Untrusted until confirmed or explicitly accepted by the authorized owner. | Charter/spec/plan → validation evidence or change request. |
| `DecisionRequest` | Presents one decision, authority, why-now trigger, options, recommendation, consequences, evidence, uncertainty, deadline, and blocked dependants. | Derived request; its resolution is not authoritative until Core validates the actor and records it. | Risk/assumption/phase/task → decision fact and affected artifacts. |
| `ChangeRequest` | Identifies target and base version, `SpecificationChange` or `PlanChange`, proposed delta, rationale, evidence, impact cone, risk/budget/schedule effect, rollback, requested authority. | Untrusted proposal until approved by the authority of the affected layer. | Current approved version → replacement version and invalidated descendants. |
| `EvidenceRequirement` | Defines what must be observed: criterion/risk ref, method, producer independence, environment/tool version, threshold, freshness, reproducibility, security/classification, waiver authority. | Authoritative binding when approved with the Outcome Contract or plan gate. | Outcome criterion/risk → expected Evidence Bundle. |
| `EvidenceBundle` | Collects observed results: requirement refs, artifacts/hashes, commands/tools/environment, inputs, timestamps, measurements, findings, independence, provenance, gaps, uncertainty, signatures/attestations. | Derived evidence; untrusted until schema, provenance, freshness, and result are validated. | Task/phase execution → Evidence Requirements and completion. |
| `ToolCatalogManifest` | Capability-projects allowed tool/action schemas for one subject/task with exact governing/capability refs, resource/parameter bounds, compiler version, hash and expiry. | Core-derived discovery record only; listing is not authority and revocation immediately invalidates it. | RoleNode/ActionContract/capabilities → allowed tool proposals. |
| `BoundedToolResult` | Records action/capability refs, disposition, bounded stdout/stderr summaries, truncation, resources/timing, postcondition probe, errors and output-artifact refs. | Boundary observation; Core validation distinguishes result, rejection and uncertainty. | ToolAction → evidence/output/failure. |
| `ToolOutputArtifact` | Stores immutable large/binary output with content hash, media/schema, size, truncation/chunking, classification, tool/version, producer, source action and retention. | Content carrier only; never execution success or evidence satisfaction by itself. | BoundedToolResult → EvidenceBundle/reviewer projection. |
| `SandboxFailureResult` | Records denial, policy violation, timeout, crash or resource exhaustion, sandbox/profile, partial-effect observation, uncertainty, diagnostics, probe and recovery. | Typed failure fact; cannot be summarized into success or retried outside Core direction. | action/sandbox → recovery, risk, decision and evidence. |
| `VisualEvidenceCapture` | Binds screenshot/frame/video refs to build/commit, route/state, viewport/DPI, theme/locale, input fixture, capture tool/version, timestamp, masking and classification. | Derived visual observation; producer cannot establish acceptance. | visual EvidenceRequirement → comparison/bundle. |
| `VisualComparisonResult` | Binds expected/baseline/observed captures, annotated regions, criteria, perceptual/structural/accessibility findings, reviewer identity/independence, uncertainty and disposition. | Independently validated evidence when acceptance-critical; never completion authority. | captures + criterion → EvidenceBundle/gate. |
| `CompletionProposal` | States claimed outcome, requirement/evidence matrix, gate results, change summary, residual risks, budget/scope conformance, unresolved items, rollback/delivery refs. | Derived proposal only; Hermes/Quality may assemble it but cannot accept it. | Delivery Plan + Evidence Bundles → Completion Decision. |
| `CompletionDecision` | Immutable authorized disposition: `Accept`, `Return`, `Defer`, or `Cancel`, actor/authority, rationale, conditions, evidence reviewed, effect authorization, timestamp. | Authoritative Core-recorded decision; never inferred from a green model response. | Completion Proposal → mission state and delivery effects. |
| `MemoryCandidate` | Proposes a reusable lesson, decision, pattern, finding, skill observation, or provider observation with scope, sources, confidence, freshness, contradictions, sensitivity, expiry, and suggested retrieval tags. | Untrusted candidate until the Memory Judge and required human/policy review approve consolidation. | Verified mission evidence/retrospective → typed memory. |
| `Retrospective` | Reviews outcome, plan accuracy, decisions, risks, failures, recoveries, cost/time, model/skill performance, evidence quality, and improvement candidates. | Derived analysis; finalized after review but cannot itself change policy, skills, or memory. | Closed mission → Memory Candidates, risks, and evaluation cases. |
| `EvaluationCase` | Defines a versioned problem/fixture, expected benefit, outcome/evidence bindings, cohort, risk class, controls, acceptance metrics and rejection/rollback criterion. | Governed evaluation definition; authoritative only after evaluation-policy approval. | Architecture hypothesis/risk → TreatmentArms and EvaluationRuns. |
| `TreatmentArm` | Freezes one controlled configuration, independent variable, fixed controls, profile/harness settings, expected direction and allocation rule. | Validated and immutable for active allocation; it carries no delivery or certification authority. | EvaluationCase → RunManifest/EvaluationRun. |
| `EvaluationRun` | Records one execution identity, case/arm/manifest refs, state, attempt/correlation refs, timing, deviations and result ref. | Core-recorded controlled observation; a completed run is evidence, not an outcome or certification decision. | case + arm + manifest → observations/deviations/result. |
| `RunManifest` | Freezes exact provenance: fixture/source hashes, governing artifact versions, repository/worktree commit, provider/model/profile/adapter/settings, prompt/context/skill/tool versions, environment, seed where supported, budgets, and manifest creation/freeze/allocation timestamps. | Immutable before run start; any material change requires a new manifest. Observed run start/end belong to `EvaluationRun`. | treatment arm + run → reproducible inputs and raw evidence. |
| `MetricObservation` | Records metric/version, value/unit, method, sample, threshold, observation time, producer, confidence and raw evidence refs. | Append-only observation, independently validated or invalidated; corrections are linked new records. | EvaluationRun → EvaluationResult. |
| `ProtocolDeviation` | Records expected step, observed deviation, cause, affected metrics/runs, severity, impact and disposition. | Mandatory exception evidence; cannot be hidden by aggregation. | run/manifest → result, exclusion and audit. |
| `EvaluationResult` | Derives controlled comparison, baseline/regression deltas, uncertainty, acceptance disposition, harms, limitations and recommendation. | Reviewed evidence for milestone/certification policy; never authority by itself. | runs + observations + deviations → milestone gate/profile decision. |
| `ControlledMultiRouteRace` | Binds fixed parent task/outcome/context, isolated routes/profiles, per-route/total budgets, allocation/cancellation, selection metric, independent evaluator and EvaluationCase/TreatmentArm/RunManifest/Run/Result refs. | Policy-authorized evaluation construct; selected route remains an untrusted candidate and route agreement is not authority. | TaskPacket + evaluation case → route runs/result → candidate contribution. |

### Lifecycle, versioning, validation, and authority

| Artifact | Lifecycle and versioning | Provenance and validation | May propose | May approve / finalize |
| --- | --- | --- | --- | --- |
| `MissionCharter` | `Draft → ClarificationRequired → ReadyForReview → Approved → Superseded`; or `Cancelled`. Approved revisions create a new version. | Authenticated intent, cited source material, required-field and scope/budget checks. | User, Messenger from authenticated intent, Brain Agent as a draft. | User/Founder or delegated mission sponsor. |
| `EngineeringSpecification` | `Draft → ClarificationRequired → ReadyForReview → Approved → Superseded → Completed`; or `Cancelled`. | Requirement IDs unique; WHAT not HOW; criteria testable; assumptions/open questions explicit; evidence and risk bindings complete. | Brain Agent, user, authorized architect. | User/Founder or designated product authority; never the Brain Agent. |
| `OutcomeContract` | `Draft → ReadyForReview → Approved → Active → Satisfied/Unsatisfied → Superseded`; or `Cancelled`. | Every criterion has a verification/evidence binding; thresholds and allowed variation are internally consistent. | Brain Agent or Quality from the approved spec. | Same authority that approves the specification plus required Quality/Security authority. |
| `PlanProposal` | `Draft → Submitted → Compiled/Rejected/Superseded`. | Exact inputs/provenance and schema validated; deterministic compiler canonicalizes IDs, versions, defaults, ordering, trace edges and policy refs without inventing strategy. | Brain Agent or authorized architect. | No approval state; proposal producer cannot approve its candidate. |
| `PlanAssessment` | `Pending` → one of `StructurallyInvalid`, `ReviewRequired`, `Ready`, or `Blocked`; `ReviewRequired` → new `Ready` or `Blocked` assessment version after separate immutable reviews; replaced versions → `Superseded`. | Structural checks plus complete coverage, evidence, DAG, architecture, proportionality, risk, budget/WIP, recovery, uncertainty, alternatives, reviewer independence, blockers and readiness fields; final version binds exact policy/risk tier and review IDs/versions/hashes. | Deterministic validators create the initial assessment; policy-designated architecture/quality/security reviewers create separate records; Core finalizes the assessment. | No approval state; Core validates completeness and independence. |
| `PlanDecision` | Immutable once recorded; corrections or reconsideration create a new superseding decision against a new/current candidate version. | Authenticated actor, exact candidate/assessment hashes, exact policy/risk-tier version, required independent-review record IDs/versions/hashes, rationale and conditions validated. | Authorized user may request a disposition. | User/Founder or policy-delegated plan authority; Core records it. No model vote is authority. |
| `DeliveryPlan` | `Candidate → ValidationFailed/Assessed → Approved → Active → ChangePending → Superseded → Closed`; or `Cancelled`. | Exact candidate must pass structural validation, assessment and the risk-tier review policy before approval. | Deterministic compiler from PlanProposal; a replacement begins with a new proposal. | Only an authenticated approving PlanDecision makes the exact candidate authoritative. |
| `PhasePlan` | `Draft → Ready → Approved → Active → GateReview → Closed/Returned/Deferred`; or `Cancelled`. | Entry conditions, task graph, WIP, capability envelope, evidence, and dependencies validated against Delivery Plan. | Brain Agent or Hermes Supervisor. | Core under explicit delegation; human authority when risk/scope/policy requires it. |
| `TaskPacket` | `Draft → Validated → Ready → Leased → Active → Review → Verification → Done`; exceptional board states remain explicit. A retry is a new attempt, not packet mutation. | Exact parent versions, context manifest, paths, capabilities, budgets, output/evidence schema, and stop guards validated. | Hermes Supervisor from an approved Phase Plan. | Core authorizes; no builder or subagent approval. |
| `SOPGraph` | `Compiled → ValidationFailed/Validated → Active → Superseded/Revoked/Closed`. | Same compiler/schema and exact approved parent hashes must produce the same canonical graph/hash; stale/forged/missing parents fail. | Core compiler; Hermes may request compilation. | Core alone validates and activates; activation grants no authority beyond parents/capabilities. |
| `RoleNode` | `Compiled → Eligible → Active → Waiting/Stopped/Completed/Revoked`. | Identity, objective, contracts, isolation, permissions and budgets must be derivable from exact parents. | SOP Compiler only. | Core records state; role cannot alter its node. |
| `ActionContract` | `Compiled → Validated → Active → Revoked/Expired/Superseded` with graph. | Allowed action schemas and bounds must be a subset of TaskPacket/policy/capabilities. | SOP Compiler only. | Core validates; capability/effect approval remains separate. |
| `InputSubscription` | `Proposed → Authorized → Active → Revoked/Expired/Superseded`. | Source/filter/purpose/classification/cursor/replay bounds and subject identity checked against assignment. | SOP Compiler or Hermes may request the derived subscription. | Core alone authorizes; subject cannot self-subscribe or widen it. |
| `RoleInputProjection` | `Produced → Delivered/Acknowledged → Superseded/Expired`. | Every item cites source identity/hash and subscription version; redactions/omissions are explicit. | Core projection service only. | No approval state; only Core may materialize it. |
| `OutputContract` | `Compiled → Validated → Active → Revoked/Expired/Superseded` with graph. | Schema, provenance, destination, classification and size/stop constraints checked against parents. | SOP Compiler only. | Core validates contract and each submitted output. |
| `SOPControlEdge` | `Compiled → Validated → Active → Satisfied/Triggered/Invalidated/Superseded`. | Edge kind, endpoints, required fact/condition, deadline/budget and policy ref must be canonical and acyclic where required. | SOP Compiler only. | Core alone records authoritative edge state. |
| `MissionExplorationBranch` | `Proposed → Authorized → Active → FindingsSubmitted → ContributionAccepted/ContributionRejected`; or `Expired/Cancelled`. | Parent/objective, ContextPack hash, read-only permissions, budget, findings/evidence and contribution destination required. | Brain Agent, Hermes, reviewer or explorer may request. | Core authorizes and records contribution disposition; branch cannot approve its findings. |
| `CompactionRecord` | `Draft → Validated/Rejected → Active → Superseded/Expired`. | Source/range hashes, retained content, omissions and authority-rehydration set/hash reproducible; current versions checked at resume. | Hermes/context runtime may produce. | Core validates activation; record never approves or replaces authority. |
| `RiskItem` | `Proposed → Open → Mitigating → Monitoring → Accepted/Closed/Expired`; revisions preserve history. | Evidence, owner, trigger, treatment, residual exposure, and review date required. | Any role; Hermes and subagents may surface candidates. | Risk owner; residual high/critical risk requires named human/security authority. |
| `Assumption` | `Proposed → Validating → Confirmed/Invalidated/AcceptedRisk → Superseded`. | Source and falsifiable validation method required; conflicting evidence blocks confirmation. | User, Brain Agent, Hermes, any subagent. | Named assumption owner; policy may forbid acceptance of some classes. |
| `DecisionRequest` | `Draft → Pending → Answered/Deferred/Expired/Cancelled`. Resolution is append-only. | Authority, options, evidence, consequences, and affected descendants validated. | Brain Agent, Hermes, Core policy, reviewer. | Only the named authorized principal; Core records and applies. |
| `ChangeRequest` | `Proposed → ImpactAnalysis → PendingApproval → Approved/Rejected/Withdrawn → Applied`. | Base version, typed delta, impact cone, trace invalidation, risk, migration/rollback, and re-verification required. | Any participant or evidence-producing role. | Specification authority for WHAT; plan authority for HOW; Core applies only after approval. |
| `EvidenceRequirement` | `Draft → Bound → Active → Satisfied/Failed/Waived → Superseded`. | Method is reproducible, threshold objective, producer independence/risk proportionate, waiver authority explicit. | Brain Agent, Quality, Security. | Outcome/plan approval authority; waivers require the named higher authority. |
| `EvidenceBundle` | `Collecting → Submitted → Validating → Validated/Rejected/Stale → Superseded`. | Tool/environment identity, inputs, hashes, timing, freshness, signatures, and raw refs probed by Core/validators. | Tool Boundary, builder, reviewer, test/security subagent. | Quality/Security validate findings; Core records satisfaction, not the producer. |
| `ToolCatalogManifest` | `Projected → Active → Revoked/Expired/Superseded`. | Exact subject/task, active capability/governing versions, schemas/bounds, compiler version and hash required. | Core capability-projection service only. | Core activates/revokes; manifest never grants a capability. |
| `BoundedToolResult` | `Reported → Validated/Rejected/Uncertain → Superseded`. | Result size/truncation, action/capability, probe, resources, errors and artifact refs verified. | Tool Boundary. | Core records disposition; producer cannot claim evidence satisfaction. |
| `ToolOutputArtifact` | `Recorded → Validated/Rejected/Stale → Superseded/Expired`. | Content identity, format, size/truncation, classification, tool/version and source action required. | Tool Boundary via BoundedToolResult. | Artifact validator/Core records state; no acceptance authority. |
| `SandboxFailureResult` | `Reported → Validated → Recovered/RetryAuthorized/Blocked/Uncertain/Closed`. | Failure kind, sandbox/profile, possible partial effects, probe and recovery evidence required. | Sandbox/Tool Boundary. | Core records recovery or named authority decides uncertain effects. |
| `VisualEvidenceCapture` | `Recorded → Validating → Validated/Rejected/Stale → Superseded`. | Reproducible UI build/state/display/capture metadata, content hashes and redaction required. | Approved capture boundary or Test/QA. | Independent visual/quality validator records validation when acceptance-critical. |
| `VisualComparisonResult` | `Draft → Reviewed → Validated/Rejected → Superseded`. | Exact captures/criteria, regions, method, findings, uncertainty and reviewer independence required. | Visual reviewer or deterministic comparator. | Quality/authorized independent reviewer validates; no completion authority. |
| `CompletionProposal` | `Draft → ReadyForReview → Submitted → Returned/Withdrawn/Decided`. | Complete trace matrix, mandatory gates, scope/budget checks, residual-risk ceiling, no stale evidence. | Hermes Supervisor or Core completion evaluator. | No approval state; it is disposed only by a Completion Decision. |
| `CompletionDecision` | Created once with terminal disposition; corrections are new superseding decisions, never edits. | Actor authority, proposal/version, evidence reviewed, conditions, and intended protected effect validated. | Authorized user may request disposition. | User/Founder or delegated completion authority; Core commits it. |
| `MemoryCandidate` | `Proposed → Judging → Approved/Rejected/Quarantined → Consolidated/Superseded/Expired`. | Evidence sources, contradictions, freshness, scope, security, dedupe, and authority-confusion checks. | Retrospective, Hermes, reviewer, Memory Judge rules. | Memory Judge plus human/policy authority required by memory class. |
| `Retrospective` | `Draft → Reviewed → Finalized → Archived/Superseded`. | Must separate observed facts, analysis, and recommendations; every material claim cites evidence. | Hermes, Brain Agent, Quality, mission owner. | Mission owner or delegated review authority finalizes; no direct governance effect. |
| `EvaluationCase` | `Draft → Reviewed → Approved → Active → Retired/Superseded`. | Expected benefit, controlled factors, acceptance metrics, security/governance analysis and rollback/rejection criterion required. | Evaluation owner, architecture, quality or security. | Evaluation-policy owner; approval authorizes only the experiment definition. |
| `TreatmentArm` | `Draft → Validated → Frozen → Retired/Superseded`. | Independent variable, controls and allocation rule validated; frozen before run allocation. | Evaluation owner or harness operator. | Core freezes after policy validation; no outcome authority. |
| `EvaluationRun` | `Registered → Ready → Running → Completed/Failed/Aborted → Assessed`. | Exact case, arm and frozen manifest required before `Running`; deviations and raw evidence required before assessment. | Harness/operator may request registration and report observations. | Core records state; no run producer may certify its result. |
| `RunManifest` | `Draft → Frozen → Used/Invalidated`. | Exact provenance fields are complete and content-hashed; changed inputs create a new manifest. | Deterministic evaluation compiler/harness. | Core freezes/invalidates under evaluation policy. |
| `MetricObservation` | append-only `Recorded → Validated/Invalidated`; corrections link a replacement observation. | Metric definition/version, method, unit, sample, source and raw evidence reproducible. | Approved measurement boundary. | Independent validator/Core records validation; producer cannot overwrite. |
| `ProtocolDeviation` | `Open → Assessed → Accepted/ExcludesRun/Resolved`. | Cause, affected metrics, severity and impact analysis mandatory; omissions invalidate result readiness. | Any run participant or monitor. | Evaluation reviewer records disposition under policy. |
| `EvaluationResult` | `Draft → Reviewed → Finalized/Superseded`. | Includes all eligible runs, observations and deviations; reports baseline/regression deltas, uncertainty, harms and limitations. | Evaluation analyst or deterministic aggregator. | Independent evaluation authority finalizes; certification remains a separate policy decision. |
| `ControlledMultiRouteRace` | `Proposed → PolicyValidated → Authorized → Active → Completed/Aborted → Assessed/Superseded`. | Fixed parent/outcome/context, isolation, routes, budgets, allocation/cancellation, selection metric, independent evaluator and complete evaluation refs required. | Brain Agent, Hermes or evaluation owner. | Core authorizes bounded execution; independent evaluation authority assesses. Candidate selection is not approval. |

## Specification semantics: WHAT, not HOW

An Engineering Specification must be sufficient to judge success while remaining implementation-neutral where the outcome permits variation. It may constrain architecture, security, compatibility, data, performance, or technology when those are genuine outcome requirements. It must not smuggle an unapproved implementation plan into task-like requirements.

An approved specification is immutable. New evidence can challenge it, but cannot edit it silently:

| Change | Required control | Invalidates |
| --- | --- | --- |
| WHAT changes: outcome, requirement, exclusion, acceptance criterion, binding constraint, risk tolerance, or evidence threshold | `ChangeRequest(kind=SpecificationChange)` and a new Specification/Outcome Contract version | Affected plan, phase, tasks, evidence, and completion claims. |
| HOW changes only: sequencing, decomposition, skill/model choice, tool, estimate, or implementation strategy within the approved outcome | `ChangeRequest(kind=PlanChange)` and a new Delivery/Phase Plan version | Affected phases, tasks, capabilities, and evidence produced under the old plan. |
| No semantic change: typo or presentation correction | New render/projection from the same structured version | No governed descendant. |

Agile adaptation occurs through these controlled revisions. Governance freezes approved meaning, not learning.

## Brain Agent, plan qualification, and decision

The Brain Agent is a governed planning role, not a provider and not an approval authority. It consumes the approved specification and outcome contract plus repository maps, LSP symbol/dependency data, applicable ADRs/policies, Git history, diagnostics, verified evidence, Engineering Memory, available skills, budgets/deadlines, and current risks.

It returns a schema-valid `PlanProposal` containing delivery strategy, tailored phase graph, task decomposition, requirement-to-task traceability, dependencies, risks/mitigations, skills/tools, writable/forbidden scopes, verification strategy, human decisions, estimates/budgets, stop/escalation conditions, assumptions and uncertainty. Qualification is explicit and durable:

`Brain Agent → PlanProposal → deterministic compiler/normalization → candidate DeliveryPlan → structural validation → initial PlanAssessment → independent review when policy requires → finalized Ready PlanAssessment → authenticated PlanDecision → approved DeliveryPlan`

The compiler may normalize only representation: IDs, versions, ordering, defaults, trace links, budgets and policy references. It cannot invent or silently change strategy. Structural validation rejects cyclic, untraceable, over-budget, under-evidenced, stale, policy-incompatible, scope-invalid or authority-violating candidates before review.

The initial `PlanAssessment` must cover specification/outcome coverage, acceptance/evidence sufficiency, DAG correctness, architecture fitness, proportionality, risk, budget/WIP feasibility, rollback/recovery, assumptions/uncertainty, material alternatives, blockers and readiness. Risk-tier policy then determines the review set. If reviews are required, the assessment remains `ReviewRequired`; reviewers create separate immutable records, and Core finalizes a new `Ready` assessment version with reviewer identity/independence and exact record bindings:

- **Low risk:** deterministic validation and an authenticated decision by an authorized user may suffice.
- **Medium/high risk:** independent architecture, quality and security reviewers participate in the combinations required by policy and recorded risk exposure.
- **All tiers:** the proposal producer cannot approve the plan or be the sole acceptance-critical reviewer. A set of agreeing models is not an approval quorum; model voting remains non-authoritative evidence.

Core validates the actor, expected versions, assessment completeness, exact policy/risk-tier version, and each required independent-review record ID/version/hash before recording the immutable `PlanDecision`. Only `Approve` creates an authoritative `DeliveryPlan`; `Return`, `Defer`, and `Reject` remain explicit, non-authorizing dispositions. Multiple proposals are not required routinely. A competing plan is requested only when risk, uncertainty, a material architectural choice or a failed assessment makes comparison proportionate.

## Phase as the primary delivery unit

The Brain Agent may tailor the phase graph within policy; not every mission must use the same phases. The reference graph is Discovery → Specification → Architecture and Contract Design → Implementation → Verification → Delivery, with safe parallel branches where dependencies and scopes permit.

Every Phase Plan contains objective, entry/exit conditions, dependencies, allowed skills/tools, writable and forbidden scope, task set, WIP limits, budget, risk controls, evidence requirements, gate, and escalation conditions.

Phase lifecycle is namespaced from task/Kanban state:

`Pending → Ready → Active → Review → Verification → Closed`

with `Blocked`, `Returned`, `Deferred`, `Uncertain`, and `Cancelled` as explicit non-happy states. A phase closes only when Core validates its exit conditions and evidence. Closing a phase authorizes consideration of the next gate; it does not approve a plan change or complete the mission.

## Kanban and enforced WIP

Within each phase, Task Packets flow:

`Backlog → Ready → InProgress → Review → Verification → Done`

and may enter `Blocked`, `NeedsClarification`, `NeedsApproval`, `Returned`, `Deferred`, `Uncertain`, or `Cancelled`. Board movement is a Core transition derived from durable task/attempt/evidence state, not a UI drag operation.

Initial policy principles are:

- at most one writable implementation task in one isolated worktree unless Core proves disjoint writable scopes;
- bounded parallel read-only exploration with separate budgets and structured findings;
- bounded unresolved Decision Requests per mission and phase;
- bounded pending protected effects;
- no task starts when doing so would violate its phase WIP limit, capability budget, dependency, or decision state.

WIP limits are versioned policy values enforced by Core/Hermes scheduling. A UI displays the constraint and its cause; it does not merely decorate over uncontrolled concurrency.

## Deterministic SOP projection and role dataflow

After plan and packet approval, the SOP Compiler deterministically projects the exact approved `DeliveryPlan`, `PhasePlan`, and `TaskPacket` versions and hashes into a candidate `SOPGraph`. Core rejects any stale, missing, forged, broader, non-canonical, cyclic, or policy-incompatible projection before activation. The graph contains `RoleNode`, `ActionContract`, authorized `InputSubscription`, `OutputContract`, and typed `SOPControlEdge` records for dependency, evidence, stop, and escalation. It coordinates delivery but never becomes a new authority layer.

Role dataflow is closed and Core-mediated:

`Core ledger/artifact store → authorized InputSubscription → RoleInputProjection → RoleNode → OutputContract → Core validation`

Core alone materializes the event/artifact projection, with exact source refs, cursor, redactions and omissions. A role cannot self-subscribe, inspect a shared ambient environment, broadcast to peers, open a free-chat channel, or treat another role's output as a command. Revocation stops future projection; replay remains bounded by the authorized cursor/range. All output returns to Core for schema, provenance, authority and evidence validation.

`ToolCatalogManifest` is the derived intersection of the RoleNode/ActionContract and current active capabilities. It aids discovery but grants nothing: the Effect Firewall still admits each `ToolAction`. Tool boundaries return a bounded `BoundedToolResult`; large or binary output becomes an immutable `ToolOutputArtifact`; denial, timeout, crash, violation, resource exhaustion, or possible partial effect becomes a `SandboxFailureResult` and follows explicit recovery.

## Exploration, compaction, races, and visual evidence

A `MissionExplorationBranch` is a logical, read-only mission branch, not authority and not a writable Git branch. It binds one parent, objective, ContextPack hash, permissions and budget. Findings cite evidence and end with a Core-recorded accepted or rejected contribution; acceptance admits only the named contribution into the next proposal, assessment, memory-candidate, or evaluation flow.

A `CompactionRecord` declares its covered sources, hashes, retained content and omissions. Before a compacted session resumes, Core must rehydrate and version-check current policy, specification, Outcome Contract, DeliveryPlan, PhasePlan, TaskPacket, decisions, capabilities, and active SOP contracts. The compacted text is context only and cannot supply authority that rehydration does not recover.

A `ControlledMultiRouteRace` is permitted only as a budgeted, isolated evaluation linked to an `EvaluationCase`, per-route `TreatmentArm` and `RunManifest`, `EvaluationRun` records, deviations, metrics and an `EvaluationResult`. Routes cannot communicate or share an ambient environment. Candidate selection follows the predeclared metric and independent assessment; it is neither model voting nor plan/task acceptance, and the selected output still passes ordinary evidence and authority gates.

Acceptance-critical UI claims bind a reproducible `VisualEvidenceCapture` to build/commit, UI state, display/capture conditions and artifacts, then a `VisualComparisonResult` to the exact criterion, baseline/expected view, annotations, reviewer independence, uncertainty and accessibility findings. A screenshot or visual-diff score alone never satisfies the gate.

## Traceability matrix

Every edge is explicit, versioned, and queryable in both directions.

| Trace stage | Required link | Gate question |
| --- | --- | --- |
| User intent | intent/source → `MissionCharter` objective | Can the sponsor verify that the charter represents the desired outcome? |
| Specification | charter objective → `REQ-*` | Is every requirement in scope and every exclusion explicit? |
| Outcome | `REQ-*`/`AC-*` → Outcome Contract clause + Evidence Requirement | What observable evidence can establish contract-equivalent success? |
| Plan qualification | clause → `PlanProposal` → candidate `DeliveryPlan` → initial `PlanAssessment` → exact required reviews → finalized `Ready` assessment → `PlanDecision` → approved Delivery Phase | Is the proposed HOW structurally valid, fit, proportionate, independently reviewed where required, and authorized by the right actor? |
| Task | phase → `TaskPacket` | Which bounded packet produces the implementation or analysis? |
| SOP projection | exact approved DeliveryPlan/PhasePlan/TaskPacket → `SOPGraph` → RoleNode/contracts/control edges → InputSubscription/RoleInputProjection/output | Can every role input, action, output, dependency, evidence, stop, and escalation path be derived without widening authority? |
| Exploration / continuity | parent mission/task/context → `MissionExplorationBranch` → accepted/rejected contribution; source range/hashes → `CompactionRecord` → rehydrated current authority | Are branch findings explicitly disposed, omissions visible, and all governing authority rehydrated before reuse? |
| Tool execution | RoleNode/ActionContract/capabilities → `ToolCatalogManifest` → ToolAction → `BoundedToolResult`/`ToolOutputArtifact`/`SandboxFailureResult` | Did the boundary stay within the projected catalog and active capability, preserve bounded output, and report failure/uncertainty honestly? |
| Evidence | task/gate → validated `EvidenceBundle` | What was actually observed, by whom/what, on which version? |
| Visual evidence | visual criterion → `VisualEvidenceCapture` → `VisualComparisonResult` → validated EvidenceBundle | Can the UI observation be reproduced from exact build, state, display/capture conditions and independent review? |
| Evaluation | hypothesis/risk → `EvaluationCase` → optional `ControlledMultiRouteRace` → `TreatmentArm` + `RunManifest` → `EvaluationRun` → `MetricObservation`/`ProtocolDeviation` → `EvaluationResult` | Can the measured benefit, route selection, or regression be reproduced from exact provenance, including isolation and every deviation? |
| Completion | evidence matrix → `CompletionProposal` → `CompletionDecision` | Did an authorized actor review a complete, current contract? |
| Memory | completion/retrospective → `MemoryCandidate` → consolidated record | Is the lesson evidence-derived, scoped, current, non-conflicting, and safe to retrieve? |

A missing edge blocks the relevant approval or gate. Superseding any node marks its affected descendants stale until revalidated.

## Authority matrix

| Actor | May propose | May approve / commit | May perform effects | Must not |
| --- | --- | --- | --- | --- |
| User / Founder | Outcomes, specs, PlanProposals, PlanDecisions, changes, completion, memory corrections | Charter/spec/outcome/plan decision/change/completion according to role | Only through an authenticated Core command and Effect Firewall | Fabricate evidence or bypass policy. |
| WePLD Core | Deterministic plan/SOP compilation and validation, role projections, tool-catalog projections, policy decisions, required gates, recovery dispositions | Durable transitions, PlanDecisions/delegated approvals, SOP/subscription/branch/race authorization, capability/effect authorization, contribution disposition, budget and completion state | Dispatch approved effects to enforcement boundaries | Invent user consent, strategy, branch findings, reviewer findings, tool/visual results, or test results. |
| Brain Agent | Specification drafts, PlanProposals, risks, assumptions, replans | Nothing it proposed | None | Self-approve, serve as sole acceptance-critical reviewer, mutate state, or call tools. |
| Hermes Supervisor | Phase/Task Packets, SOP compilation/subscription requests, exploration branches, controlled-race requests, routing, actions, findings, compaction and completion proposal | No governance or completion artifact | Coordinates only within issued capabilities; effects remain mediated | Activate its graph/subscription, accept branch contributions, select authority by race, redefine higher contracts, or retain hidden durable truth. |
| SOP Role | Contract-conforming output, findings and typed action proposals | Nothing | None directly; receives only Core RoleInputProjections and proposes through its contracts | Self-subscribe, query shared state, free-chat/broadcast to peers, widen contracts, treat peer output as authority, or perform direct effects. |
| Builder | Typed actions, implementation artifacts, evidence candidates | No plan, effect, gate, or completion approval | None directly; Tool Boundary executes authorized actions | Expand scope or lower acceptance criteria. |
| Explorer Subagent | `MissionExplorationBranch` request, repository map and cited findings | Nothing; Core separately accepts/rejects the named contribution | Read-only authorized retrieval within branch permissions/budget | Write, coordinate peers, self-subscribe, or treat findings as accepted. |
| Reviewer | PlanAssessment findings and disposition recommendation | No plan/completion approval unless separately authenticated as the policy-named authority; may satisfy a review requirement only through validated evidence | Read/test within its envelope | Review a proposal it produced, rely solely on producer reasoning, or repair without a new task. |
| Test / QA / Visual Reviewer | Test plan, checks, visual captures/comparisons, results and quality findings | No completion; validates evidence in assigned domain with required independence | Core-authorized test/capture execution | Alter implementation while acting as independent QA or accept its own capture without required review. |
| Security Reviewer | Threat findings, severity, remediation/exception recommendation | Security gate/exception only when explicitly designated | Security tooling inside its envelope | Grant broader mission authority. |
| Tool / Sandbox Boundary | `BoundedToolResult`, `ToolOutputArtifact`, `SandboxFailureResult`, probe result and evidence | Nothing | The exact capability-bound action inside the named sandbox | Infer intent, change scope, exceed output bounds, hide truncation/failure/partial effects, or report success without probing. |

## Completion and memory

Completion eligibility requires every applicable Outcome Contract clause, mandatory gate, policy, budget, scope, regression rule, evidence freshness rule, and unresolved-risk threshold to pass. Different implementations may satisfy this contract; [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md) defines equivalence.

Acceptance is never a worker or model status. Core presents a validated Completion Proposal and records an authorized Completion Decision. Any delivery effect—proposal ref, push, pull request, merge, release, or deployment—remains a separately classified protected effect.

After closure, a Retrospective can emit Memory Candidates. The Memory Judge defined in [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md) checks provenance, contradictions, freshness, scope, and authority before consolidation. Governance Memory remains authoritative and is never downgraded to optional advice.

## Spec Kit-informed lifecycle, internalized as Core state

GitHub Spec Kit's observed lifecycle is a useful process reference, not WePLD's authority or storage model. The exact clean-room mapping evaluated by RS-01/RS-02 is:

| Reference lifecycle concept | WePLD durable meaning |
| --- | --- |
| Constitution | versioned governance Policy and project constraints |
| Specify | `MissionCharter`, `EngineeringSpecification`, and `OutcomeContract` |
| Clarify | `Assumption`, `DecisionRequest`, clarification response, and a new proposed artifact version |
| Plan | `PlanProposal`, deterministic normalized candidate, `PlanAssessment`, authenticated `PlanDecision`, approved `DeliveryPlan`, and tailored `PhasePlan` |
| Checklist / tasks | `EvidenceRequirement`, verification bindings, and authorized `TaskPacket` graph |
| Analyze | cross-artifact trace/contradiction validators that can hold or request change, never silently edit |
| Implement | deterministic SOPGraph projection, Core-authorized role inputs, admitted lease, typed bounded tool results/actions through the Effect Firewall, artifacts, and evidence |
| Converge | bounded remediation/change loop followed by `CompletionProposal` and authorized `CompletionDecision` |
| Preset, bundle, prompt, Markdown | versioned import or review/export projection with provenance; never the authoritative record |

No lifecycle stage advances because a prompt says it did. Each transition binds authenticated authority, expected prior version, validation, policy, and evidence. Workflow shell steps have no ambient execution path; they compile to typed effects or are rejected.

## Candidate baseline compatibility

Draft PR #1 is a candidate prerequisite baseline only. Its staged plan/completion approvals, structured specification seed, ledger, CAS, WWP boundary, worktree isolation, proposal refs, validation, and evidence-derived lesson work are useful foundations. It does not yet supply this complete specification lifecycle, separate Outcome Contract, Phase/Kanban/change-control model, or generalized memory workflow. This planning package neither ratifies the PR's branch-local decisions nor authorizes its merge.

The baseline gate is disposition-neutral: it must reach `RESOLVED`, but resolution may accept, return, defer, or reject the candidate. H1 may start only after that resolution and either (a) the candidate prerequisite contracts are accepted, or (b) an approved replacement-foundation plan covers the identified contract gaps. A returned, deferred, or rejected Draft PR #1 therefore cannot permanently block H1; it changes the prerequisite path and evidence, not the requirement for a resolved gate. See [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md) and [30_ARCHITECTURE_SUMMARY.md](30_ARCHITECTURE_SUMMARY.md).

See also: [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [16_Data_Model.md](16_Data_Model.md), and [17_Event_System.md](17_Event_System.md).
