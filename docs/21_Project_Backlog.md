# 21 — Project Backlog

## Backlog strategy

This is a product-level, dependency-ordered backlog—not an instruction to begin implementation. Work is prioritized by whether it reduces the central trust risk: can WePLD turn an approved outcome into a bounded engineering operation whose authority, effects, evidence, and acceptance remain durable and explainable across different supported models?

The historical M0–M7 roadmap remains useful lineage. The delivery program now adds a separately named **Build Feature Baseline Gate** followed by Hermes Intelligence milestones H1–H9. These labels are intentionally distinct: Draft PR #1 uses its own internal milestone language, remains open, Draft, unmerged, and unratified, and is only a candidate prerequisite baseline. This plan neither approves that PR nor treats its branch as canonical.

No Hermes Intelligence implementation may begin until the Proposed ADRs applicable to that milestone are accepted and the preceding gate is closed with recorded evidence.

## Gate 0 — Architecture review and candidate-baseline disposition

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Architecture package review | one coherent governed-delivery architecture | documents 01–34 and diagrams use the same authority, artifact, lifecycle, and milestone vocabulary; open decisions have owners | this package |
| Proposed ADR disposition | implementation-authorizing decisions are explicit | Proposed ADR-0015 through ADR-0024 are reviewed; each is accepted, revised, rejected, or deferred before dependent work | architecture package |
| Build Feature Baseline Gate | candidate prerequisite is independently accepted or rejected | PR #1 is reviewed at its final head; contract, security, recovery, documentation, and test evidence are reconciled; Draft status alone proves nothing | PR #1, independent review |
| Contract-extension plan | additive and breaking changes are known before code | current candidate contracts are classified as retained, extended, superseded, or temporary V0 behavior; migration/coexistence rules are named | baseline disposition, ADR review |
| Evaluation charter | Hermes improvements are falsifiable | fixed missions, repositories, policies, environments, budgets, variables, metrics, and evidence ownership are approved | [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md) |

## H1 — Governed Specification Workflow

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Mission charter and outcome contract | user intent becomes governed engineering truth | structured, versioned `MissionCharter`, `EngineeringSpecification`, and `OutcomeContract`; Markdown is projection/export, not sole truth | baseline gate, ADR-0015 |
| Clarification and approval | ambiguity is visible before delivery planning | explicit clarification state; durable authenticated specification approval; no plan is generated from an unapproved version | authority model |
| Specification change control | approved WHAT never changes silently | specification changes create a `ChangeRequest` and new version; supersession and provenance are queryable | immutable versions, ledger |
| Verification bindings | success is testable before execution | each acceptance criterion maps to evidence requirements and verification bindings; unresolved or untestable criteria block approval | testing strategy |

## H2 — Brain Planning and Delivery Control

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Brain Agent planning | approved WHAT becomes a validated HOW proposal | versioned `DeliveryPlan`, tailored phase graph, requirement-to-task/evidence traceability, risks, budgets, stop conditions | H1, ADR-0016/0017 |
| Phase plans and task packets | execution receives bounded operational units | each `PhasePlan` and `TaskPacket` declares objective, entry/exit conditions, tools, skills, writable/forbidden scope, evidence, and budget | plan contract |
| Native Kanban and WIP | flow control is enforced, not decorative | durable task states; policy-configurable WIP limits; conflicting writable work cannot be admitted | Core state machine |
| Controlled change | evidence may challenge HOW without silently changing WHAT | plan-only change and specification change are classified, approved at the correct authority, versioned, and traced | H1 change semantics |

## H3 — Hermes Skill Kernel and Hook Runtime

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Agent Kernel | bounded operational state without competing governance truth | objective, phase/task, hypothesis, observations, retry state, confidence, and next proposal are attempt-scoped; Core remains authoritative | H2, ADR-0018 |
| Skill Runtime and Router | executable procedures are versioned and measurable | manifest, applicability, context/tool needs, capabilities, procedure, verification, failures, output/evidence schema, compatibility, trust | task packets |
| Typed Hook Bus | lifecycle extension points cannot become an escape path | observational, validating, blocking, and effect-producing hooks are distinguished; every blocking/effect hook is policy-mediated and recorded | effect firewall |

## H4 — Context, LSP, and Hybrid Retrieval

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Context Compiler | minimal task context is reproducible | collect→filter→rank→deduplicate→compress→label→fit→validate pipeline; every item states source, trust, freshness, reason, scope, and token estimate | H3, ADR-0019 |
| LSP intelligence | symbols and diagnostics are normalized evidence | language-neutral definitions/references/symbols/diagnostics/impact contract; a deliberately limited initial adapter set passes fixtures | context contract |
| Hybrid code retrieval | exact and structural truth outrank semantic similarity | lexical, LSP, AST/tree-sitter, Git, ADR/spec, evidence, and memory retrieval carry provenance; vector results never override authoritative sources | evaluated retrieval stack |

## H5 — Controlled Engineering Loops

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Loop engine | diagnosis and retry are bounded and evidence-based | each iteration records hypothesis, before/after evidence, intended/expected/actual result, confidence delta, and next decision | H4, ADR-0022 |
| Guard and escalation policy | non-progress stops safely | repeated actions, no change, oscillation, worsening diagnostics, schema failure, budget exhaustion, invalid plan, and required authority are detected | Core policy |
| Controlled replanning | execution evidence can revise HOW through governance | retry, split, skill change, reviewer advice, replan, profile switch, clarification, human decision, and stop form an explicit ladder | H2 change control |

## H6 — Bounded Subagents and Self-Review

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Subagent Supervisor | specialist work is bounded, not a free swarm | one objective, scoped context/tools/capabilities/budget/deadline/output/evidence contract per subagent | H5, ADR-0021 |
| Parallelism controls | read parallelism improves insight without write conflict | bounded read-only exploration; isolated and conflict-controlled writable work; Core-enforced WIP | H2 Kanban |
| Independent review chain | implementation is not its own only judge | builder→deterministic validation→reviewer→test/quality→security where applicable→completion proposal | evidence contracts |

## H7 — Typed Memory Intelligence

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Memory taxonomy | different truth classes cannot be conflated | Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory have explicit authority and retention | H6, ADR-0020 |
| Memory Judge | only eligible evidence becomes durable learning | candidate creation, consolidation, deduplication, contradiction, freshness, expiry, supersession, confidence, scope, and provenance are governed | completion evidence |
| Safe retrieval | memory improves later work without changing authority | bounded retrieval; classification and project scope enforced; governance memory authoritative; other memory influence remains labelled | Context Compiler |

## H8 — Model-Independent Outcome Convergence

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Outcome Equivalence Contract | different implementations meet one engineering truth | functional behavior, public contracts, architecture, security, quality, regression, evidence, and unresolved-risk thresholds remain constant across models | H7, ADR-0023 |
| Provider/profile certification | supported profiles prove bounded fitness | fixed harness measures convergence, safety, evidence truthfulness, cost, attempts, escalation, recovery, and honest non-convergence | ADR-0024 |
| Ablation program | harness value is measured rather than assumed | controlled enable/disable experiments for LSP, RAG, memory, loops, subagents, and skill routing with constant mission/environment/budget | [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md) |

## H9 — Product Surfaces After Runtime Truth

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Shared workflow API | CLI, Studio, MCP, and APIs call the same Core commands | no surface owns business state or bypasses approval/effect/evidence semantics | H1–H8 |
| Operational review surfaces | users operate outcomes and decisions, not tool chatter | Mission Control, Spec Review, Plan Review, Kanban, Decisions, Risks, Evidence, Change Requests, Completion Review | stable read/write contracts |
| Truthful degraded UX | stale, uncertain, denied, and incomplete states remain visible | no client-side workflow mutation; every authoritative badge resolves to Core state/evidence | surface conformance |

## Deferred until earned

Open marketplace distribution, remote worker fleets, cloud-first control planes, cross-organization learning, universal language/model support, autonomous production deployment, full graphical IDE breadth, and uncontrolled multi-agent swarms are not part of H1–H9. They require their own evidence, security review, and authorization after the local governed system proves value.

## Backlog hygiene

Every backlog item must name customer outcome, authoritative artifact, bounded-context owner, state/event impact, authority and policy class, normal/denied/degraded/recovery evidence, dependency, and definition of done. “Add model X,” “add RAG,” “make it autonomous,” and “build integrations” are not backlog items without those constraints.

## Proposed ADR dependencies

These ADRs remain **Proposed** until separately accepted. Acceptance authorizes only the dependent milestone after its preceding gate closes.

- H1: [ADR-0015 — governed specification contract](adr/ADR-0015-governed-specification-contract.md)
- H2: [ADR-0016 — Brain-plan/builder-execution separation](adr/ADR-0016-brain-plan-builder-execution-separation.md) and [ADR-0017 — phase, Kanban, and controlled change](adr/ADR-0017-phase-kanban-controlled-change.md)
- H3: [ADR-0018 — Hermes Skill Runtime and Hook Bus](adr/ADR-0018-hermes-skill-runtime-hook-bus.md)
- H4: [ADR-0019 — Context Compiler, LSP, and hybrid retrieval](adr/ADR-0019-context-compiler-lsp-hybrid-retrieval.md)
- H5: [ADR-0022 — controlled loop and escalation](adr/ADR-0022-controlled-loop-escalation.md)
- H6: [ADR-0021 — bounded subagents and structured handoffs](adr/ADR-0021-bounded-subagents-structured-handoffs.md)
- H7: [ADR-0020 — typed memory and Memory Judge](adr/ADR-0020-typed-memory-memory-judge.md)
- H8: [ADR-0023 — model-independent outcome equivalence](adr/ADR-0023-model-independent-outcome-equivalence.md) and [ADR-0024 — harness evaluation and provider certification](adr/ADR-0024-harness-evaluation-provider-certification.md)

See also: [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [22_Milestones.md](22_Milestones.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), and [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).
