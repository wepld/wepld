# 21 — Project Backlog

## Backlog strategy

This is a product-level, dependency-ordered backlog—not an instruction to begin implementation. Work is prioritized by whether it reduces the central trust risk: can WePLD turn an approved outcome into a bounded engineering operation whose authority, effects, evidence, and acceptance remain durable and explainable across different supported models?

The historical M0–M7 roadmap remains useful lineage. The delivery program now adds a separately named **Build Feature Baseline Gate** followed by Hermes Intelligence milestones H1–H9. These labels are intentionally distinct: Draft PR #1 uses its own internal milestone language, remains open, Draft, unmerged, and unratified, and is only a candidate prerequisite baseline. This plan neither approves that PR nor treats its branch as canonical.

No Hermes Intelligence implementation may begin until the Proposed ADRs applicable to that milestone are accepted and the preceding gate is closed with recorded evidence.

## Gate 0 — Architecture review and candidate-baseline disposition

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Architecture package review | one coherent governed-delivery architecture | documents 01–37 and diagrams use the same authority, artifact, lifecycle, and milestone vocabulary; open decisions have owners | this package |
| Proposed ADR disposition | implementation-authorizing decisions are explicit | Proposed ADR-0015 through ADR-0026 are reviewed; each is accepted, revised, rejected, or deferred before dependent work | architecture package |
| Build Feature Baseline Gate | H1 prerequisite supply is resolved without requiring candidate acceptance | PR #1 is reviewed at its final head; disposition is accepted/returned/deferred/rejected; gate becomes **Resolved** only when required candidate contracts are accepted or an approved replacement-foundation plan covers every missing prerequisite | PR #1, independent review |
| Contract-extension or replacement-foundation plan | additive, breaking, and missing foundations are known before code | candidate contracts are classified as retained, extended, superseded, temporary, or absent; migration/coexistence rules are named; non-acceptance cannot permanently block H1 | baseline resolution, ADR review |
| Early Evaluation Spine | every milestone produces comparable, reproducible evidence | versioned `EvaluationCase`, `EvaluationRun`, `TreatmentArm`, `RunManifest`, `MetricObservation`, `ProtocolDeviation`, and `EvaluationResult`; exact fixture/repository/contract/config/tool/environment/seed provenance; baseline/regression comparison | ADR-0024, [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md) |
| Reference evidence and provenance gate | no reference-system idea enters by imitation or unreviewed source reuse | official sources/revisions, matrix disposition, RS experiment, rollback, milestone and license/provenance state are complete; clean-room remains default | [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), RS-00 |

## H1 — Governed Specification Workflow

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Mission charter and outcome contract | user intent becomes governed engineering truth | structured, versioned `MissionCharter`, `EngineeringSpecification`, and `OutcomeContract`; Markdown is projection/export, not sole truth; H1 emits a reproducible baseline/regression `EvaluationRun` | resolved baseline path, ADR-0015/ADR-0024 |
| Clarification and approval | ambiguity is visible before delivery planning | explicit clarification state; durable authenticated specification approval; no plan is generated from an unapproved version | authority model |
| Specification change control | approved WHAT never changes silently | specification changes create a `ChangeRequest` and new version; supersession and provenance are queryable | immutable versions, ledger |
| Verification bindings | success is testable before execution | each acceptance criterion maps to evidence requirements and verification bindings; unresolved or untestable criteria block approval | testing strategy |

## H2 — Brain Planning and Delivery Control

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Plan proposal and deterministic compilation | approved WHAT becomes a normalized HOW candidate without granting authority | Brain emits versioned `PlanProposal`; deterministic Plan Compiler normalizes it into a candidate and structural validators reject invalid DAG/trace/budget/WIP shapes | H1, ADR-0016/0017 |
| Plan assessment | structural validity is supplemented by architecture, risk, and evidence qualification | `PlanAssessment` covers specification/Outcome Contract and evidence coverage, DAG, architecture, proportionality, risk/mitigation, budget/WIP, rollback/recovery, assumptions/uncertainty, alternatives, reviewer identity/independence, blockers, and readiness | plan candidate, risk-tier policy |
| Plan decision | only authenticated authority creates an approved `DeliveryPlan` | low-risk uses deterministic assessment plus authorized user review; medium/high-risk adds independent architecture/quality/security assessment; producer cannot approve or be sole acceptance-critical reviewer; model voting is not authority; Core records `PlanDecision` | qualified assessment, ADR-0016 |
| Alternative-plan trigger | comparison cost is proportional to uncertainty | alternatives are generated when risk/uncertainty policy triggers, not for every mission; considered alternatives remain recorded in assessment | PlanAssessment |
| Typed SOP Compiler | approved delivery artifacts become an executable coordination graph without new authority | deterministic `SOPGraph` contains `RoleNode`, `ActionContract`, authorized `InputSubscription`, `OutputContract`, dependency/evidence/stop/escalation edges, and exact parent versions; Core validates it | approved plan/phase/task, ADR-0017, RS-21 |
| Authorized Role Subscription Graph | each role sees only relevant typed upstream artifacts | Core projects assignment-scoped events/artifacts; roles cannot self-subscribe to authority-bearing input; uncontrolled shared environments, peer broadcast, and free agent chat fail conformance | SOPGraph, RS-22 |
| Phase plans and task packets | execution receives bounded operational units | each `PhasePlan` and `TaskPacket` declares objective, entry/exit conditions, tools, skills, writable/forbidden scope, evidence, and budget | plan contract |
| Native Kanban and WIP | flow control is enforced, not decorative | durable task states; policy-configurable WIP limits; conflicting writable work cannot be admitted | Core state machine |
| Controlled change | evidence may challenge HOW without silently changing WHAT | plan-only change and specification change are classified, approved at the correct authority, versioned, and traced | H1 change semantics |

## H3 — Hermes Skill Kernel and Hook Runtime

### H3.1 — Built-in Skill Kernel

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Agent Kernel | bounded operational state without competing governance truth | objective, phase/task, hypothesis, observations, retry state, confidence, and next proposal are attempt-scoped; Core remains authoritative | H2, ADR-0018 |
| Built-in Skill Runtime and Router | repository-owned procedures are versioned and measurable | static/versioned manifest, exact content hash, typed input/output, declared context/tool/capability requirements, verification/evidence contracts, compatibility, trust | task packets |
| Built-in typed hooks | lifecycle extension points cannot become an escape path | repository-owned observational, validating, blocking, and effect-producing hooks are typed; every blocking/effect hook is policy-mediated and recorded | effect firewall |
| Internal lifecycle/control-event port | Hermes components interoperate without a public alternate control plane | versioned schemas preserve action/result correlation, ordering, idempotency, reconnect projection and backpressure; identity never implies capability | ADR-0018, RS-05/RS-20 internal subset |
| H3.1 scope guard | the minimum kernel does not become a platform project | no external package installation, public registry, marketplace, third-party executable hook, or generalized signing infrastructure is required for H3 exit | ADR-0018 |
| Capability-projected tool schemas | model-visible tools match current authority and budget | compile catalog from policy, TaskPacket, capabilities, role, sandbox tier, classification, phase, and budget; omit denied tools where possible; Effect Firewall remains final enforcement; context/MCP-catalog budgets pass | ADR-0018, RS-23 |
| Tool output budget | tool results cannot flood or bypass model context controls | validate result schema before insertion; enforce inline byte/line limits; record summary, head/tail, truncation reason, original size, content-addressed full artifact, retrieval permission, expiry/classification | ADR-0018/0019, RS-26 |

### H3.2 — Governed Skill Packaging (later gated extension)

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Skill/hook package lifecycle | proven built-ins may become revocable governed packages | begin only after H3.1 demonstrates measured value and governance preservation; stage exact hashes; verify signatures/provenance; Core activates atomically; rollback/revocation and malicious/partial/incompatible cases pass | H3.1 evidence, explicit architecture/security authorization, RS-00/RS-11/RS-19 |

## H4 — Context, LSP, and Hybrid Retrieval

### H4.1 — Reproducible Context and Initial LSP

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Context Compiler | minimal task context is authority-ranked and reproducible | exact file/path/identifier retrieval, lexical search, Git evidence, and collect→filter→rank→deduplicate→compress→label→fit→validate; exact manifests record inputs, hashes, config, source/trust/freshness/reason/scope/tokens | H3.1, ADR-0019 |
| Initial LSP intelligence | one language adapter proves the normalized boundary | rust-analyzer definitions/references/symbols/diagnostics pass reproducibility, version-binding, degradation, and conformance fixtures | context contract |
| Mission Exploration Branches | research alternatives remain durable without changing the execution path | read-only branch records parent, objective, Context Pack hash, permissions, budget, findings, evidence, status, and accepted/rejected contribution; transcript/summary is never Core truth | RS-24 |
| Governed compaction | reduced context can be independently rehydrated without authority loss | `CompactionRecord` preserves policy, approved spec/Outcome Contract, current plan/phase/task versions, unresolved decisions, risks, evidence requirements, stop conditions, omissions, and full source hash | ADR-0019, RS-25 |

### H4.2 — Structural and Impact Intelligence

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Structural retrieval and impact | syntax and change impact improve context after the minimum works | AST/tree-sitter, affected-file, and affected-test treatments meet preregistered benefit/safety thresholds; additional LSP adapters enter only after conformance | H4.1 evidence |

### H4.3 — Governed Semantic Retrieval

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Semantic retrieval and embeddings | similarity is admitted only when it adds practical value | disabled by default until controlled ablation shows benefit without unacceptable authority, security, freshness, cost, or token harm; exact/authoritative sources always outrank it; failing treatment rolls back | H4.1 baseline, optional H4.2 evidence |

## H5 — Controlled Engineering Loops

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Loop engine | diagnosis and retry are bounded and evidence-based | each iteration records hypothesis, before/after evidence, intended/expected/actual result, confidence delta, and next decision | H4, ADR-0022 |
| Guard and escalation policy | non-progress stops safely | repeated actions, no change, oscillation, worsening diagnostics, schema failure, budget exhaustion, invalid plan, and required authority are detected | Core policy |
| Controlled replanning | execution evidence can revise HOW through governance | retry, split, skill change, reviewer advice, replan, profile switch, clarification, human decision, and stop form an explicit ladder | H2 change control |
| Sandbox-aware failure feedback | denied work changes the next hypothesis instead of looping | typed result records boundary, reason, attempted effect, retryability, safe alternatives, required authority, recovery state, evidence; identical denial cannot repeat without changed hypothesis/capability/plan/authority | ADR-0022, RS-27 |
| Contextual Risk Advisor experiment | contextual review may reduce interruptions without weakening enforcement | advisory allow-under-existing-policy/narrow/block/request-authority recommendation only; measure false allow/block, flapping, latency, interruption reduction, and unsafe-effect escape; never grants or overrides | H5/H6, RS-28 |

## H6 — Bounded Subagents and Self-Review

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Subagent Supervisor | specialist work is bounded, not a free swarm | one objective, scoped context/tools/capabilities/budget/deadline/output/evidence contract per subagent | H5, ADR-0021 |
| Parallelism controls | read parallelism improves insight without write conflict | bounded read-only exploration; isolated and conflict-controlled writable work; Core-enforced WIP | H2 Kanban |
| Independent review chain | implementation is not its own only judge | builder→deterministic validation→reviewer→test/quality→security where applicable→completion proposal | evidence contracts |
| Role subscription enforcement | specialist coordination is typed and least-knowledge | builder/reviewer/test/security/explorer receive only authorized SOPGraph subscriptions; peer broadcast and self-subscription are rejected and evidenced | RS-21/RS-22 |
| Engineering Committee (advisory) | governed multi-model deliberation with zero authority | user-triggered three-member V0 per [36_Engineering_Committee.md](36_Engineering_Committee.md); independent first round, one challenge round, verbatim minority reports, durable dispositions, hard budgets; admission only through [37_Committee_Evaluation_Protocol.md](37_Committee_Evaluation_Protocol.md) with EC-A1–EC-A3 evidence and no rejection criterion fired | ADR-0026, ADR-0021, ADR-0024 |

## H7 — Typed Memory Intelligence

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Memory taxonomy | different truth classes cannot be conflated | Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory have explicit authority and retention | H6, ADR-0020 |
| Memory Judge | only eligible evidence becomes durable learning | candidate creation, consolidation, deduplication, contradiction, freshness, expiry, supersession, confidence, scope, and provenance are governed | completion evidence |
| Safe retrieval | memory improves later work without changing authority | bounded retrieval; classification and project scope enforced; governance memory authoritative; other memory influence remains labelled | Context Compiler |
| Exploration contribution judgment | research branches create evidence, not truth | accepted findings may become evidence-linked Memory Candidates; rejected contributions remain durable negative evidence; transcripts/compactions cannot supersede artifacts | RS-24/RS-25 |

## H8 — Model-Independent Outcome Convergence

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Outcome Equivalence Contract | different implementations meet one engineering truth | functional behavior, public contracts, architecture, security, quality, regression, evidence, and unresolved-risk thresholds remain constant across models | H7, ADR-0023 |
| Accumulated evaluation history | H8 starts from comparable milestone evidence rather than new instrumentation | consume the pre-H1 foundation Baseline `EvaluationRun` defined in document 34 plus H1–H7 runs, exact manifests, deviations, and baseline/regression results produced under ADR-0024 | early evaluation spine |
| Provider/profile certification | supported profiles prove bounded fitness | cross-model controlled, repeated/randomized runs measure convergence, safety, evidence truthfulness, portability, cost, attempts, escalation, recovery, and honest non-convergence | ADR-0025 |
| Ablation program | harness value is measured rather than assumed | controlled enable/disable experiments for LSP, RAG, memory, loops, subagents, skill routing, and every positively dispositioned RS-00–RS-30 arm with constant mission/environment/budget | ADR-0024/0025, [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md) |
| Controlled multi-route race | risk-triggered parallel attempts test robustness without voting authority | fixed specification/contract/commit/policy/tools/environment/budget/gates; each candidate independently passes; model vote/visual preference never accepts; cost and selection bias bounded | ADR-0023/0025, RS-29 |

## H9 — Product Surfaces After Runtime Truth

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Shared workflow API | CLI, Studio, MCP, and APIs call the same Core commands | no surface owns business state or bypasses approval/effect/evidence semantics | H1–H8 |
| Operational review surfaces | users operate outcomes and decisions, not tool chatter | Mission Control including the Execution Console view, Spec Review, Plan Review, Kanban, Decisions, Risks, Evidence, Change Requests, Completion Review | stable read/write contracts, RS-09/RS-17 |
| Agent/editor interoperability | compatible clients can project and request work without becoming an authority boundary | ACP conformance, reconnect and confused-deputy fixtures pass; every effect remains Core-mediated; new Proposed ADR accepted | H8, RS-06/RS-20 |
| Truthful degraded UX | stale, uncertain, denied, and incomplete states remain visible | no client-side workflow mutation; every authoritative badge resolves to Core state/evidence | surface conformance |
| Visual execution evidence and team projection | users see roles, work, blockers, decisions, budgets, evidence, and results without appearance-based acceptance | screenshots, video, accessibility tree, console, network trace, and deterministic interaction scripts enter EvidenceBundles; UI remains a projection | RS-30 |

## Deferred until earned

Open marketplace distribution, remote worker fleets, cloud-first control planes, cross-organization learning, universal language/model support, autonomous production deployment, SEO/ads/growth automation, no-code hosting, full graphical IDE breadth, uncontrolled shared agent environments, free peer-to-peer agent chat, and uncontrolled multi-agent swarms are not part of H1–H9. They require their own evidence, security review, and authorization after the local governed system proves value.

## Backlog hygiene

Every backlog item must name customer outcome, authoritative artifact, bounded-context owner, state/event impact, authority and policy class, normal/denied/degraded/recovery evidence, dependency, definition of done, and its evaluation case/baseline. Every H milestone must emit an evaluation-compatible run with exact provenance and a regression comparison. “Add model X,” “add RAG,” “make it autonomous,” and “build integrations” are not backlog items without those constraints.

## Proposed ADR dependencies

These ADRs remain **Proposed** until separately accepted. Acceptance authorizes only the dependent milestone after its preceding gate closes.

- Baseline/H1 prerequisite: [ADR-0024 — evaluation spine and run provenance](adr/ADR-0024-evaluation-spine-run-provenance.md)
- H1: [ADR-0015 — governed specification contract](adr/ADR-0015-governed-specification-contract.md)
- H2: [ADR-0016 — Brain-plan/builder-execution separation](adr/ADR-0016-brain-plan-builder-execution-separation.md) and [ADR-0017 — phase, Kanban, and controlled change](adr/ADR-0017-phase-kanban-controlled-change.md)
- H3: [ADR-0018 — Hermes Skill Runtime and Hook Bus](adr/ADR-0018-hermes-skill-runtime-hook-bus.md)
- H4: [ADR-0019 — Context Compiler, LSP, and hybrid retrieval](adr/ADR-0019-context-compiler-lsp-hybrid-retrieval.md)
- H5: [ADR-0022 — controlled loop and escalation](adr/ADR-0022-controlled-loop-escalation.md)
- H6: [ADR-0021 — bounded subagents and structured handoffs](adr/ADR-0021-bounded-subagents-structured-handoffs.md) and [ADR-0026 — governed Engineering Committee deliberation](adr/ADR-0026-engineering-committee.md)
- H7: [ADR-0020 — typed memory and Memory Judge](adr/ADR-0020-typed-memory-memory-judge.md)
- H8: [ADR-0023 — model-independent outcome equivalence](adr/ADR-0023-model-independent-outcome-equivalence.md) and [ADR-0025 — model/profile certification](adr/ADR-0025-model-profile-certification.md); H8 consumes ADR-0024 run history

See also: [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [22_Milestones.md](22_Milestones.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), and [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md).
