# 30 — Architecture Summary

## Executive decision

Build WePLD as a **desktop-first, local-first governed engineering control plane with a native specification-to-acceptance method and Hermes as its first-party Engineering Intelligence Runtime**.

WePLD provides the engineering method. The user provides the desired outcome. The Brain Agent submits a `PlanProposal`; deterministic compilation, durable assessment, risk-tier review, and an authenticated `PlanDecision` determine whether it becomes the governed `DeliveryPlan`. Hermes operates bounded engineering work. Builder models execute task packets. WePLD Core governs every transition and effect. Evidence determines whether the result is acceptable.

The strategic promise is:

> **Different brains. Same engineering truth.**

Supported models may take different implementation paths. WePLD promises neither byte-identical code nor equal model capability. It promises that accepted outputs satisfy the same approved specification, Outcome Contract, policy, architecture, quality, security, regression, evidence, and unresolved-risk thresholds. A model that cannot converge within its bounds stops or escalates honestly; the quality bar is never reduced.

## Repository and candidate-baseline truth

Canonical `main` contains the architecture/master-plan package; no implementation is canonical there. Draft PR #1 is an open, Draft, unmerged, unratified candidate Build Feature baseline. It is reference evidence only. This planning package:

- does not modify, approve, merge, or mark PR #1 ready;
- does not treat its branch-local “Accepted/FROZEN/READY/M0 complete” labels as canonical decisions;
- preserves its useful candidate contracts without claiming they are merged;
- requires an independent **Build Feature Baseline Gate** to become **Resolved**, with either candidate prerequisite contracts accepted or an approved replacement-foundation plan covering every missing prerequisite; a returned, deferred, or rejected PR #1 is not a permanent H1 blocker;
- requires Proposed ADR-0024 and the Early Evaluation Spine before H1/H2, then requires every H milestone to emit exact-provenance baseline/regression evidence;
- requires the relevant Proposed ADR-0015 through ADR-0026 to be accepted and the preceding H gate to close before dependent implementation.

## Authority hierarchy

~~~text
1. WePLD governance policy
2. Approved Engineering Specification
3. Outcome and Acceptance Contract
4. Approved Delivery Plan
5. Approved Phase Plan
6. Core-projected SOP / authorized role-subscription graph
7. Builder Task Packet and exact capability projection
8. Tool action
~~~

No lower layer may silently redefine a higher layer.

| Actor | May do | May not do |
| --- | --- | --- |
| User/Founder | state outcomes, approve specification, authenticate `PlanDecision`, approve protected effects/completion, resolve real decisions | bypass policy/evidence through a surface shortcut |
| Core | own durable state, deterministic plan normalization, policy, assessments/decisions, capabilities, budgets, WIP, transitions, evidence requirements, completion, recovery | fabricate human approval or accept model prose/votes as evidence or authority |
| Brain Agent | propose architecture, risks, `PlanProposal`, tasks, verification, replans | approve its proposal, act as its sole acceptance-critical reviewer, execute effects, change approved WHAT |
| Hermes Supervisor | maintain bounded attempt state, route skills/models/subagents/context, coordinate loops | own governance truth, grant capabilities, accept completion |
| Builder | implement one bounded task packet and report artifacts/evidence | expand scope, alter acceptance, approve itself |
| Explorer/reviewer/test/security subagents | return scoped structured findings and evidence | chat as a swarm, inherit authority, mutate mission truth |
| Tool/effect boundary | enforce granted operation, probe result, record evidence | infer approval from prompt text or execute undeclared effects |

## Core architecture

~~~mermaid
flowchart LR
  H["Human executive"] --> S["CLI / Studio / MCP / API\nsame Core workflow"]
  S --> C["WePLD Core\ngovernance • state • policy • WIP\napprovals • budgets • evidence • recovery"]
  C --> G["Specification & Delivery\ncharter • outcome • plan qualification\nSOP graph • phases • Kanban • change"]
  C --> X["Hermes Supervisor\nskills • hooks • context • loops • subagents"]
  X --> B["Brain Agent / builder profiles\nprovider-neutral reasoning"]
  X --> T["Effect Firewall + isolated tools/worktrees"]
  C --> E["Ledger + CAS + Git\nauthoritative facts and evidence"]
  C --> M["Typed Memory + Memory Judge"]
  E --> Q["Projections / Mission Control / evaluation"]
~~~

The essential separation is: **brains reason; Hermes supervises bounded execution; builders and subagents produce artifacts; Core governs; policy authorizes; tools enforce; evidence verifies; the user decides where authority requires.**

## Native user workflow

1. Describe the desired outcome.
2. Clarify unresolved questions.
3. Review and approve the Engineering Specification and Outcome Contract.
4. Review the compiled candidate and `PlanAssessment`; record an authenticated `PlanDecision` before any `DeliveryPlan` becomes authoritative.
5. Execute tailored phases through durable Kanban/WIP control.
6. Observe evidence, risks, budgets, and flow.
7. Resolve genuine decisions and controlled change requests.
8. Review verified completion.
9. Accept, return, defer, or cancel.
10. Consolidate eligible Engineering Memory through the Memory Judge.

The user operates outcomes, decisions, risks, and reports—not low-level tool commands. Every caller uses the same Core commands and authority.

## Structured domain truth

The architecture defines versioned contracts for at least:

`MissionCharter · EngineeringSpecification · OutcomeContract · PlanProposal · PlanAssessment · PlanDecision · DeliveryPlan · PhasePlan · TaskPacket · RiskItem · Assumption · DecisionRequest · ChangeRequest · EvidenceRequirement · EvidenceBundle · CompletionProposal · CompletionDecision · MemoryCandidate · Retrospective`

The Early Evaluation Spine adds `EvaluationCase · TreatmentArm · RunManifest · EvaluationRun · MetricObservation · ProtocolDeviation · EvaluationResult`. Exact fixture, repository, contract, configuration, tool, environment, profile, and seed provenance is captured before execution. These records establish comparable milestone baselines and regressions; they do not themselves certify a provider or profile.

Reference-informed candidate contracts add `SOPGraph · AuthorizedRoleSubscriptionGraph · CapabilityProjectedToolCatalog · MissionExplorationBranch · CompactionRecord · BoundedToolResult · ToolOutputArtifact · SandboxFailureResult`. They are typed, versioned, Core-projected or Core-validated artifacts—not proof that another system's shared environment, chat, classifier, race, or visual artifact is an authority or security boundary. `ContextualRiskAdvisor` and `ControlledMultiRouteRace` remain measured experiments at their assigned gates.

Each declares purpose, authority, required fields, lifecycle, versioning, provenance, validation, proposer/approver, trace links, and authoritative/derived/untrusted class. Markdown is a review/export projection, never the sole source of truth.

The required trace is:

~~~text
user intent
  → specification requirement
  → Outcome Contract
  → PlanProposal
  → compiled candidate + PlanAssessment
  → authenticated PlanDecision
  → approved DeliveryPlan / plan phase
  → Core-projected SOP / authorized role subscriptions
  → task packet
  → required evidence
  → completion decision
  → memory candidate / judge disposition
~~~

## Hermes Engineering Intelligence Runtime

Hermes is not a thin LLM wrapper and not a second control plane. Its planned internal capabilities are:

1. **Agent Kernel and typed SOP execution** — bounded objective/phase/task/hypothesis/observation/retry/confidence/next-action state; an H2-designed `SOPGraph` may drive H3/H6 execution only as a Core-projected graph, never as a shared-environment authority.
2. **Skill Runtime and Router** — H3.1 proves repository-owned built-in procedures and typed lifecycle contracts; H3.2 package installation, signing, activation, rollback, and revocation is a later optional gate after measured benefit and governance proof.
3. **Context Compiler** — H4.1 collects, filters, authority-ranks, deduplicates, provenance-labels, fits, validates, and captures reproducible minimal packs; non-authoritative `MissionExplorationBranch` records and source-linked `CompactionRecord` artifacts preserve promotion, omission, and rehydration truth.
4. **LSP Intelligence** — H4.1 starts with a conformance-tested `rust-analyzer` boundary; H4.2 adds structural/impact intelligence and other adapters only after incremental benefit and reliability evidence.
5. **Hybrid Code Retrieval** — H4.1 combines exact authoritative sources, lexical, Git, and supported LSP signals; H4.2 may add structural ranking; H4.3 may add semantic retrieval only after ablation benefit and no authority, privacy, security, reproducibility, latency, or cost harm.
6. **Controlled Loop Engine** — observe→diagnose→hypothesize→minimal action→execute→verify→compare→update→continue/replan/escalate/stop, with non-progress guards, typed `SandboxFailureResult` feedback, and an advisory-only `ContextualRiskAdvisor` experiment at H5/H6.
7. **Typed Hook Bus** — H3.1 supports built-in observational, validating, blocking, and effect-producing lifecycle contracts; no policy-bypass plugin path or implied third-party package surface.
8. **Subagent Supervisor** — bounded specialist assignments and `AuthorizedRoleSubscriptionGraph` projections, read-only parallelism, isolated writes, structured artifact/event handoffs, no role self-subscription, free peer chat, shared writable environment, or free swarm.
9. **Independent Self-Review** — builder→deterministic validation→reviewer→test/quality→security where applicable→completion proposal.
10. **Effect Firewall and tool projection** — exact grants compile to a version-bound `CapabilityProjectedToolCatalog`; every call re-enters proposed effect→classification→policy→capability→approval where required→durable intent→execute→probe→evidence, then returns a bounded result plus a classified content-addressed `ToolOutputArtifact` when raw output exceeds the context envelope.
11. **Typed Memory** — Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory with judge-controlled consolidation; exploration and compaction never become durable learning by implication.
12. **Evaluation-only route orchestration** — `ControlledMultiRouteRace` remains an H8 treatment with fixed contracts, isolated/read-only routes, deterministic scoring/join, cancellation, full cost/evidence accounting, and no authority from whichever model finishes first.

Durable mission truth remains in Core throughout.

## Model-independent outcome convergence

The Outcome Equivalence Contract compares accepted outputs across:

- functional behavior and acceptance criteria;
- public/API/data contracts;
- approved architecture and scope constraints;
- security policy and unsafe-effect threshold;
- build/test/review/regression quality gates;
- evidence completeness and unresolved-risk threshold.

Formatting beyond repository rules, internal strategy, attempt count, model/tool sequence, and non-contractual style need not match. The escalation ladder is improved context → specialized skill → task split → reviewer/advisor → replan → supported profile switch → clarification → human decision → safe stop.

Proposed ADR-0024 establishes the Early Evaluation Spine before H1/H2. Every H milestone preregisters compatible cases/arms, freezes a `RunManifest`, captures `EvaluationRun` observations and deviations with exact provenance, and compares a versioned baseline against regressions. Proposed ADR-0025 governs H8 certification: H8 consumes the accumulated history, then adds controlled cross-model, repeated/randomized or counterbalanced, ablation, portability, independent-scoring, drift, expiry, and revocation evidence. A `ControlledMultiRouteRace` is one optional H8 treatment, not an inferred production primitive; fixed allocation, isolation, budget, scoring, join, loser evidence, and rollback criteria are mandatory.

For controlled comparisons, the harness holds mission, repository commit, approved specification, policy, tools, environment, budget class, and maximum attempts constant. It varies one declared component/profile and measures equivalence, gate pass, regressions, unsafe effects, evidence, convergence, cost/time, interventions, plan changes, escalation, recovery, and honest non-convergence. Ablations cover LSP, retrieval, memory, loops, subagents, and skill routing. A stored run is evidence, not certification authority.

## Compatibility with Draft PR #1

| Classification | Candidate baseline examples | Planning treatment |
| --- | --- | --- |
| **Supporting** | command outcomes including `Deferred`; ledger/CAS/provenance; structured spec seed; mission criteria/gates/budget; WWP no-human/no-state-mutation rule; provider-neutral gateway; worktrees/snapshots/proposal refs; validated paths/ids/refs; explicit plan/completion decisions; bounded untrusted lessons | retain semantics if baseline is accepted |
| **Needs extension** | specification lifecycle; minimal plan/task; reduced WWP; empty skills; narrow context; generic evidence/decision/memory shapes | add H1–H8 contracts through accepted ADRs and migration/coexistence plans |
| **Unchanged boundary** | Core authority; Brain proposes; worker reports; no fabricated human actor; proposal ref rather than merge; memory never grants authority; evidence before completion | preserve as non-negotiable |
| **Temporary V0** | Build Feature only; no specification-approval gate; deterministic single `T1`; default `src/**`; DEV no containment/Manual/fixtures; loopback-only provider; role `stub`; empty skills; no LSP/RAG/hooks/subagents/self-review/UI; local fingerprint memory; Linux/WSL2 focus | disclose honestly; do not generalize or present as H capability |
| **Must wait** | all H1–H9 implementation and contract/vocabulary changes | Baseline Gate `Resolved` through accepted prerequisite contracts or an approved replacement-foundation plan; relevant ADR accepted; preceding gate closed; explicit next authorization |

Candidate documents that describe auto-merge, Bounded-Auto under V0, full OS sandboxing, full Context Assembly/WWP/reviewer isolation, or branch-local “frozen/approved” status must not override the final narrow V0 contract or canonical planning decisions.

## Delivery sequence

The original M0–M7 sequence remains historical lineage. Current execution gates are:

1. **Build Feature Baseline Gate:** independently accept, return, defer, or reject PR #1, then mark the gate `Resolved` only with accepted prerequisite contracts or an approved replacement-foundation plan; never imply merge approval or permanent deadlock.
2. **Early Evaluation Spine:** before H1/H2, accept ADR-0024 and operationalize exact-provenance cases, arms, manifests, runs, observations, deviations, results, and milestone baseline/regression comparisons.
3. **H1:** governed specification and outcome contract.
4. **H2:** `PlanProposal` compilation, structural validation, `PlanAssessment`, risk-tier independent review, authenticated `PlanDecision`, phase/Kanban/WIP and controlled change; design typed `SOPGraph` and authorized role-subscription contracts without importing a free shared environment.
5. **H3.1 / optional H3.2:** prove the built-in Agent/Skill Kernel, typed hooks, SOP execution, capability-projected tool catalogs and bounded tool results first; consider governed external packaging only after measured benefit and explicit architecture/security authorization.
6. **H4.1 / optional H4.2 / optional H4.3:** prove reproducible exact/lexical/Git/`rust-analyzer` context, non-authoritative exploration branches, compaction records and tool-output artifactization first; add structural/impact or semantic intelligence only through separate conformance and ablation gates.
7. **H5:** bounded engineering loops, typed sandbox-denial feedback, escalation, and an advisory Contextual Risk Advisor experiment only.
8. **H6:** supervised subagents, Core-projected authorized role subscriptions, typed handoffs, isolated writes, and independent self-review.
9. **H7:** typed memory and Memory Judge.
10. **H8:** consume accumulated evaluation evidence for model-independent outcome convergence, controlled ablations, an optional controlled multi-route-race treatment, and scoped profile certification under ADR-0025.
11. **H9:** runtime-truth-first product surfaces—Mission Control, Spec Review, Plan Review, Kanban, Decisions, Risks, Evidence, Change Requests, Completion Review, and visual execution/team views that resolve to Core events, artifacts, and `EvidenceBundle` provenance.

No milestone authorizes the next merely by finishing code. Each needs accepted normal/failure/security/recovery evidence and explicit next authorization as defined in [22_Milestones.md](22_Milestones.md).

## Proposed ADR package

All are **Proposed**, not Accepted:

1. [ADR-0015 — governed specification contract](adr/ADR-0015-governed-specification-contract.md)
2. [ADR-0016 — Brain-plan/builder-execution separation](adr/ADR-0016-brain-plan-builder-execution-separation.md)
3. [ADR-0017 — phase, Kanban, and controlled change](adr/ADR-0017-phase-kanban-controlled-change.md)
4. [ADR-0018 — Hermes Skill Runtime and Hook Bus](adr/ADR-0018-hermes-skill-runtime-hook-bus.md)
5. [ADR-0019 — Context Compiler, LSP, and hybrid retrieval](adr/ADR-0019-context-compiler-lsp-hybrid-retrieval.md)
6. [ADR-0020 — typed memory and Memory Judge](adr/ADR-0020-typed-memory-memory-judge.md)
7. [ADR-0021 — bounded subagents and structured handoffs](adr/ADR-0021-bounded-subagents-structured-handoffs.md)
8. [ADR-0022 — controlled loop and escalation](adr/ADR-0022-controlled-loop-escalation.md)
9. [ADR-0023 — model-independent outcome equivalence](adr/ADR-0023-model-independent-outcome-equivalence.md)
10. [ADR-0024 — evaluation spine and exact run provenance](adr/ADR-0024-evaluation-spine-run-provenance.md)
11. [ADR-0025 — model/profile certification from accumulated evidence](adr/ADR-0025-model-profile-certification.md)
12. [ADR-0026 — governed Engineering Committee deliberation](adr/ADR-0026-engineering-committee.md)

Numbering avoids collision with the candidate branch’s ADR-0001–ADR-0014 and does not ratify those unmerged records.

## Scope and deferred systems

H1–H9 target one user, one local project, bounded local execution, limited initial languages/adapters/profiles, explicit approvals, and reviewable proposal refs. Open marketplace distribution, remote fleets, cloud-first control planes, universal language/model support, uncontrolled swarms, role self-subscription, free inter-agent chat/shared writable environments, cross-customer learning, autonomous production deployment, and full IDE breadth remain deferred or rejected as foundations.

## Document map

Documents 01–20 retain product, system, component, security, data/event/API, roadmap, and risk foundations. Documents 21–30 define delivery, technology, repository, engineering, testing, performance, release, future, and summary governance. The focused extension package is:

- [31 — Governed Specification Workflow](31_Governed_Specification_Workflow.md)
- [32 — Hermes Engineering Intelligence Runtime](32_Hermes_Engineering_Intelligence_Runtime.md)
- [33 — Model-Independent Outcome Convergence](33_Model_Independent_Outcome_Convergence.md)
- [34 — Harness Evaluation Protocol](34_Harness_Evaluation_Protocol.md)
- [35 — Reference Systems and Competitive Architecture](35_Reference_Systems_and_Competitive_Architecture.md)
- [36 — Engineering Committee](36_Engineering_Committee.md)
- [37 — Committee Evaluation Protocol](37_Committee_Evaluation_Protocol.md)

Documents 36–37 define the advisory Engineering Committee: governed multi-model deliberation that sits outside the authority chain, preserves dissent, carries hard budgets and durable failure dispositions, changes plans only through the normal PlanProposal→PlanDecision pipeline, and earns admission solely through the falsifiable EC-A1–EC-A8 experiments — Committee agreement is not engineering truth. Document 35 supplies the evidence boundary for reference-informed decisions: Spec Kit informs the typed Delivery Protocol; Pi informs composable Hermes primitives, exploration, compaction, and bounded output; Zed/ACP informs future editor interoperability; Warp informs bounded execution operations; MetaGPT informs typed SOP compilation and authorized role subscriptions; OpenCode informs capability-projected tool schemas; Cursor informs exploration, typed denial feedback, and an advisory risk experiment; Atoms informs controlled multi-route and evidence-linked team/visual UX experiments; Claude Code, Codex, Aider and OpenHands supply additional controlled treatments. Shared environments/chat, product classifiers, fastest-wins/model-vote/appearance selection, and visual artifacts do not become Core authority, security boundaries, or acceptance evidence by analogy. RS-00–RS-30, Core authority, license/provenance review and the H1–H9 gates decide admission.

## Architecture-gate conclusion

The architecture package in Draft PR #2 is ready to return to architecture review, not implementation or merge. Reviewers must disposition the Proposed ADRs and independently resolve the Build Feature Baseline Gate without modifying or treating Draft PR #1 as canonical. These documents authorize no Hermes Intelligence implementation, source change, ADR acceptance, release, marketplace, remote fleet, or production deployment.
