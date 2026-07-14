# 31 — Governed Specification Workflow

**Status:** Proposed architecture for review. This document defines target product contracts; it does not authorize implementation or the merge of Draft PR #1.

## Product contract

WePLD supplies the engineering method. The user supplies the desired outcome. The Brain Agent proposes a governed delivery plan. Hermes operates the bounded engineering organization. Builder models execute approved task packets through mediated actions. WePLD Core governs every transition and effect. Evidence determines whether the outcome is acceptable.

The user is not expected to bring an external project-management or specification method. CLI, Studio, MCP, and APIs are surfaces over this same Core workflow:

1. **Describe** the outcome, constraints, urgency, and context.
2. **Clarify** material ambiguity, assumptions, exclusions, risks, and evidence needs.
3. **Review and approve the Engineering Specification.**
4. **Review and approve the Delivery Plan.**
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

Core is the only authority that records approvals, issues capabilities, commits transitions, accounts budgets, authorizes effects, and establishes recovery or completion state. The Brain Agent may propose specifications and plans but never approve them. Hermes may orchestrate and execute approved work but never own governance truth. Builders and subagents may return artifacts, findings, and evidence but never redefine scope or accept a mission.

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
| `DeliveryPlan` | Defines delivery strategy, tailored phase graph, requirement mapping, dependencies, risks/mitigations, skills/tools, scopes, verification strategy, estimates, budgets, decision points, stop/escalation conditions. | Authoritative at `Approved`; Brain Agent output is untrusted until validated and approved. | Specification + Outcome Contract → Phase Plans. |
| `PhasePlan` | Authorizes one phase: objective, entry/exit conditions, dependencies, skills, tools, writable/forbidden scopes, task set, WIP limits, budget, risk controls, evidence requirements, gate, escalation conditions. | Derived-governed and binding after Core records delegated or human approval. | Delivery Plan phase → Task Packets and phase gate. |
| `TaskPacket` | Gives one objective, requirement refs, inputs, context manifest, skill/model/subagent profile, allowed tools/capabilities, writable/forbidden scope, budget/deadline, expected outputs, acceptance/evidence schema, stop conditions. | Derived-governed; Hermes proposes, Core validates and authorizes. | Phase Plan task → actions, artifacts, and evidence. |
| `RiskItem` | Tracks risk statement, category, likelihood, impact, exposure, triggers, owner, mitigation, contingency, residual risk, review date, blocking flag, evidence refs. | Authoritative control record once validated; cannot redefine higher-layer scope. | Any artifact/phase → decision, control, or gate. |
| `Assumption` | Records statement, source, scope, confidence, owner, validation method/deadline, consequence if false, and evidence. | Untrusted until confirmed or explicitly accepted by the authorized owner. | Charter/spec/plan → validation evidence or change request. |
| `DecisionRequest` | Presents one decision, authority, why-now trigger, options, recommendation, consequences, evidence, uncertainty, deadline, and blocked dependants. | Derived request; its resolution is not authoritative until Core validates the actor and records it. | Risk/assumption/phase/task → decision fact and affected artifacts. |
| `ChangeRequest` | Identifies target and base version, `SpecificationChange` or `PlanChange`, proposed delta, rationale, evidence, impact cone, risk/budget/schedule effect, rollback, requested authority. | Untrusted proposal until approved by the authority of the affected layer. | Current approved version → replacement version and invalidated descendants. |
| `EvidenceRequirement` | Defines what must be observed: criterion/risk ref, method, producer independence, environment/tool version, threshold, freshness, reproducibility, security/classification, waiver authority. | Authoritative binding when approved with the Outcome Contract or plan gate. | Outcome criterion/risk → expected Evidence Bundle. |
| `EvidenceBundle` | Collects observed results: requirement refs, artifacts/hashes, commands/tools/environment, inputs, timestamps, measurements, findings, independence, provenance, gaps, uncertainty, signatures/attestations. | Derived evidence; untrusted until schema, provenance, freshness, and result are validated. | Task/phase execution → Evidence Requirements and completion. |
| `CompletionProposal` | States claimed outcome, requirement/evidence matrix, gate results, change summary, residual risks, budget/scope conformance, unresolved items, rollback/delivery refs. | Derived proposal only; Hermes/Quality may assemble it but cannot accept it. | Delivery Plan + Evidence Bundles → Completion Decision. |
| `CompletionDecision` | Immutable authorized disposition: `Accept`, `Return`, `Defer`, or `Cancel`, actor/authority, rationale, conditions, evidence reviewed, effect authorization, timestamp. | Authoritative Core-recorded decision; never inferred from a green model response. | Completion Proposal → mission state and delivery effects. |
| `MemoryCandidate` | Proposes a reusable lesson, decision, pattern, finding, skill observation, or provider observation with scope, sources, confidence, freshness, contradictions, sensitivity, expiry, and suggested retrieval tags. | Untrusted candidate until the Memory Judge and required human/policy review approve consolidation. | Verified mission evidence/retrospective → typed memory. |
| `Retrospective` | Reviews outcome, plan accuracy, decisions, risks, failures, recoveries, cost/time, model/skill performance, evidence quality, and improvement candidates. | Derived analysis; finalized after review but cannot itself change policy, skills, or memory. | Closed mission → Memory Candidates, risks, and evaluation cases. |

### Lifecycle, versioning, validation, and authority

| Artifact | Lifecycle and versioning | Provenance and validation | May propose | May approve / finalize |
| --- | --- | --- | --- | --- |
| `MissionCharter` | `Draft → ClarificationRequired → ReadyForReview → Approved → Superseded`; or `Cancelled`. Approved revisions create a new version. | Authenticated intent, cited source material, required-field and scope/budget checks. | User, Messenger from authenticated intent, Brain Agent as a draft. | User/Founder or delegated mission sponsor. |
| `EngineeringSpecification` | `Draft → ClarificationRequired → ReadyForReview → Approved → Superseded → Completed`; or `Cancelled`. | Requirement IDs unique; WHAT not HOW; criteria testable; assumptions/open questions explicit; evidence and risk bindings complete. | Brain Agent, user, authorized architect. | User/Founder or designated product authority; never the Brain Agent. |
| `OutcomeContract` | `Draft → ReadyForReview → Approved → Active → Satisfied/Unsatisfied → Superseded`; or `Cancelled`. | Every criterion has a verification/evidence binding; thresholds and allowed variation are internally consistent. | Brain Agent or Quality from the approved spec. | Same authority that approves the specification plus required Quality/Security authority. |
| `DeliveryPlan` | `Draft → ValidationFailed/ReadyForReview → Approved → Active → ChangePending → Superseded → Closed`; or `Cancelled`. | DAG, traceability, budgets, scopes, WIP, risks, skills, gates, and escalation validated against higher layers. | Brain Agent. | User/Founder or policy-delegated plan authority; Brain Agent cannot self-approve. |
| `PhasePlan` | `Draft → Ready → Approved → Active → GateReview → Closed/Returned/Deferred`; or `Cancelled`. | Entry conditions, task graph, WIP, capability envelope, evidence, and dependencies validated against Delivery Plan. | Brain Agent or Hermes Supervisor. | Core under explicit delegation; human authority when risk/scope/policy requires it. |
| `TaskPacket` | `Draft → Validated → Ready → Leased → Active → Review → Verification → Done`; exceptional board states remain explicit. A retry is a new attempt, not packet mutation. | Exact parent versions, context manifest, paths, capabilities, budgets, output/evidence schema, and stop guards validated. | Hermes Supervisor from an approved Phase Plan. | Core authorizes; no builder or subagent approval. |
| `RiskItem` | `Proposed → Open → Mitigating → Monitoring → Accepted/Closed/Expired`; revisions preserve history. | Evidence, owner, trigger, treatment, residual exposure, and review date required. | Any role; Hermes and subagents may surface candidates. | Risk owner; residual high/critical risk requires named human/security authority. |
| `Assumption` | `Proposed → Validating → Confirmed/Invalidated/AcceptedRisk → Superseded`. | Source and falsifiable validation method required; conflicting evidence blocks confirmation. | User, Brain Agent, Hermes, any subagent. | Named assumption owner; policy may forbid acceptance of some classes. |
| `DecisionRequest` | `Draft → Pending → Answered/Deferred/Expired/Cancelled`. Resolution is append-only. | Authority, options, evidence, consequences, and affected descendants validated. | Brain Agent, Hermes, Core policy, reviewer. | Only the named authorized principal; Core records and applies. |
| `ChangeRequest` | `Proposed → ImpactAnalysis → PendingApproval → Approved/Rejected/Withdrawn → Applied`. | Base version, typed delta, impact cone, trace invalidation, risk, migration/rollback, and re-verification required. | Any participant or evidence-producing role. | Specification authority for WHAT; plan authority for HOW; Core applies only after approval. |
| `EvidenceRequirement` | `Draft → Bound → Active → Satisfied/Failed/Waived → Superseded`. | Method is reproducible, threshold objective, producer independence/risk proportionate, waiver authority explicit. | Brain Agent, Quality, Security. | Outcome/plan approval authority; waivers require the named higher authority. |
| `EvidenceBundle` | `Collecting → Submitted → Validating → Validated/Rejected/Stale → Superseded`. | Tool/environment identity, inputs, hashes, timing, freshness, signatures, and raw refs probed by Core/validators. | Tool Boundary, builder, reviewer, test/security subagent. | Quality/Security validate findings; Core records satisfaction, not the producer. |
| `CompletionProposal` | `Draft → ReadyForReview → Submitted → Returned/Withdrawn/Decided`. | Complete trace matrix, mandatory gates, scope/budget checks, residual-risk ceiling, no stale evidence. | Hermes Supervisor or Core completion evaluator. | No approval state; it is disposed only by a Completion Decision. |
| `CompletionDecision` | Created once with terminal disposition; corrections are new superseding decisions, never edits. | Actor authority, proposal/version, evidence reviewed, conditions, and intended protected effect validated. | Authorized user may request disposition. | User/Founder or delegated completion authority; Core commits it. |
| `MemoryCandidate` | `Proposed → Judging → Approved/Rejected/Quarantined → Consolidated/Superseded/Expired`. | Evidence sources, contradictions, freshness, scope, security, dedupe, and authority-confusion checks. | Retrospective, Hermes, reviewer, Memory Judge rules. | Memory Judge plus human/policy authority required by memory class. |
| `Retrospective` | `Draft → Reviewed → Finalized → Archived/Superseded`. | Must separate observed facts, analysis, and recommendations; every material claim cites evidence. | Hermes, Brain Agent, Quality, mission owner. | Mission owner or delegated review authority finalizes; no direct governance effect. |

## Specification semantics: WHAT, not HOW

An Engineering Specification must be sufficient to judge success while remaining implementation-neutral where the outcome permits variation. It may constrain architecture, security, compatibility, data, performance, or technology when those are genuine outcome requirements. It must not smuggle an unapproved implementation plan into task-like requirements.

An approved specification is immutable. New evidence can challenge it, but cannot edit it silently:

| Change | Required control | Invalidates |
| --- | --- | --- |
| WHAT changes: outcome, requirement, exclusion, acceptance criterion, binding constraint, risk tolerance, or evidence threshold | `ChangeRequest(kind=SpecificationChange)` and a new Specification/Outcome Contract version | Affected plan, phase, tasks, evidence, and completion claims. |
| HOW changes only: sequencing, decomposition, skill/model choice, tool, estimate, or implementation strategy within the approved outcome | `ChangeRequest(kind=PlanChange)` and a new Delivery/Phase Plan version | Affected phases, tasks, capabilities, and evidence produced under the old plan. |
| No semantic change: typo or presentation correction | New render/projection from the same structured version | No governed descendant. |

Agile adaptation occurs through these controlled revisions. Governance freezes approved meaning, not learning.

## Brain Agent and plan approval

The Brain Agent is a governed planning role, not a provider and not an approval authority. It consumes the approved specification and outcome contract plus repository maps, LSP symbol/dependency data, applicable ADRs/policies, Git history, diagnostics, verified evidence, Engineering Memory, available skills, budgets/deadlines, and current risks.

It returns a schema-valid proposal containing delivery strategy, tailored phase graph, task decomposition, requirement-to-task traceability, dependencies, risks/mitigations, skills/tools, writable/forbidden scopes, verification strategy, human decisions, estimates/budgets, and stop/escalation conditions. Core rejects cyclic, untraceable, over-budget, under-evidenced, policy-incompatible, or authority-violating plans before review. Approval is a separate authenticated command.

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

## Traceability matrix

Every edge is explicit, versioned, and queryable in both directions.

| Trace stage | Required link | Gate question |
| --- | --- | --- |
| User intent | intent/source → `MissionCharter` objective | Can the sponsor verify that the charter represents the desired outcome? |
| Specification | charter objective → `REQ-*` | Is every requirement in scope and every exclusion explicit? |
| Outcome | `REQ-*`/`AC-*` → Outcome Contract clause + Evidence Requirement | What observable evidence can establish contract-equivalent success? |
| Plan | clause → Delivery Phase | Where and under which controls will the obligation be addressed? |
| Task | phase → `TaskPacket` | Which bounded packet produces the implementation or analysis? |
| Evidence | task/gate → validated `EvidenceBundle` | What was actually observed, by whom/what, on which version? |
| Completion | evidence matrix → `CompletionProposal` → `CompletionDecision` | Did an authorized actor review a complete, current contract? |
| Memory | completion/retrospective → `MemoryCandidate` → consolidated record | Is the lesson evidence-derived, scoped, current, non-conflicting, and safe to retrieve? |

A missing edge blocks the relevant approval or gate. Superseding any node marks its affected descendants stale until revalidated.

## Authority matrix

| Actor | May propose | May approve / commit | May perform effects | Must not |
| --- | --- | --- | --- | --- |
| User / Founder | Outcomes, specs, plans, decisions, changes, completion, memory corrections | Charter/spec/outcome/plan/change/completion according to role | Only through an authenticated Core command and Effect Firewall | Fabricate evidence or bypass policy. |
| WePLD Core | Validations, policy decisions, required gates, recovery dispositions | Durable transitions, delegated approvals, capability/effect authorization, budget and completion state | Dispatch approved effects to enforcement boundaries | Invent user consent, implementation strategy, or test results. |
| Brain Agent | Specification drafts, plans, risks, assumptions, replans | Nothing it proposed | None | Self-approve, mutate state, or call tools. |
| Hermes Supervisor | Phase/Task Packets, routing, actions, findings, completion proposal | No governance or completion artifact | Coordinates only within issued capabilities; effects remain mediated | Redefine higher contracts or retain hidden durable truth. |
| Builder | Typed actions, implementation artifacts, evidence candidates | No plan, effect, gate, or completion approval | None directly; Tool Boundary executes authorized actions | Expand scope or lower acceptance criteria. |
| Explorer Subagent | Repository map and cited findings | Nothing | Read-only authorized retrieval | Write or coordinate peers. |
| Reviewer | Findings and disposition recommendation | No completion; may satisfy a review requirement only through validated evidence | Read/test within its envelope | Rely solely on builder reasoning or repair without a new task. |
| Test / QA | Test plan, checks, results, quality findings | No completion; validates evidence in assigned domain | Core-authorized test execution | Alter implementation while acting as independent QA. |
| Security Reviewer | Threat findings, severity, remediation/exception recommendation | Security gate/exception only when explicitly designated | Security tooling inside its envelope | Grant broader mission authority. |
| Tool Boundary | Observed action result, probe result, evidence | Nothing | The exact capability-bound action | Infer intent, change scope, or report success without probing. |

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
| Plan | proposed `DeliveryPlan` and tailored `PhasePlan`, validated and separately approved |
| Checklist / tasks | `EvidenceRequirement`, verification bindings, and authorized `TaskPacket` graph |
| Analyze | cross-artifact trace/contradiction validators that can hold or request change, never silently edit |
| Implement | Core-admitted lease, typed actions through the Effect Firewall, artifacts, and evidence |
| Converge | bounded remediation/change loop followed by `CompletionProposal` and authorized `CompletionDecision` |
| Preset, bundle, prompt, Markdown | versioned import or review/export projection with provenance; never the authoritative record |

No lifecycle stage advances because a prompt says it did. Each transition binds authenticated authority, expected prior version, validation, policy, and evidence. Workflow shell steps have no ambient execution path; they compile to typed effects or are rejected.

## Candidate baseline compatibility

Draft PR #1 is a candidate prerequisite baseline only. Its staged plan/completion approvals, structured specification seed, ledger, CAS, WWP boundary, worktree isolation, proposal refs, validation, and evidence-derived lesson work are useful foundations. It does not yet supply this complete specification lifecycle, separate Outcome Contract, Phase/Kanban/change-control model, or generalized memory workflow. This planning package neither ratifies the PR's branch-local decisions nor authorizes its merge. See [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md) and [30_ARCHITECTURE_SUMMARY.md](30_ARCHITECTURE_SUMMARY.md).

See also: [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [16_Data_Model.md](16_Data_Model.md), and [17_Event_System.md](17_Event_System.md).
