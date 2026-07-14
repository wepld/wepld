# 22 — Milestones

## Milestone philosophy

Milestones are evidence gates, not calendar promises or feature bundles. A gate closes only when its acceptance, failure, recovery, and security scenarios produce reviewable evidence and the named authority explicitly authorizes the next increment.

The original M0–M7 sequence remains historical architecture lineage. It is not deleted or silently redefined. The execution-oriented sequence below uses a distinct **Build Feature Baseline Gate** and H1–H9 so it cannot be confused with either the original milestones or Draft PR #1's branch-local milestone names.

Draft PR #1 is reference material and a candidate prerequisite only. It is open, Draft, unmerged, and unratified. Nothing in this document approves it, declares its tests sufficient, or authorizes its merge.

## Historical M0–M7 lineage

| Historical milestone | Enduring proof intent | Relationship to H1–H9 |
| --- | --- | --- |
| M0 — Architecture Gate | agree what is being built and under which authority | architecture review and Proposed ADR disposition precede the Baseline Gate |
| M1 — Durable Local Core | state and audit survive normal failure | candidate prerequisite; durability continues as a gate invariant |
| M2 — Safe Task Effect | one bounded effect is isolated and evidenced | candidate prerequisite; generalized by H2/H3 effect and task contracts |
| M3 — First Mission Loop | one bounded outcome is reviewable end to end | decomposed into H1–H6 so specification, planning, execution, and review are independently provable |
| M4 — Governed Autonomy | unattended work stays bounded | H5/H6/H8 prove bounded loops, supervision, and honest non-convergence; Full-Auto is not authorized |
| M5 — Organizational Memory | verified history improves later work safely | H7 gives memory types, judging, consolidation, freshness, and authority |
| M6 — Studio Beta | users operate through truthful surfaces | H9 follows stable runtime contracts |
| M7 — Ecosystem/Enterprise Readiness | extension and remote growth preserve trust | remains deferred beyond H1–H9 |

## Build Feature Baseline Gate — candidate prerequisite disposition

| Field | Gate contract |
| --- | --- |
| **Purpose** | Decide whether the narrow Build Feature slice is an acceptable prerequisite substrate for later Hermes Intelligence work. |
| **Entry gate** | Architecture package available; PR #1 final head identified; independent reviewers assigned. |
| **Scope** | Candidate contracts, staged plan/completion approvals, ledger/CAS/worktree behavior, DEV disclosure, proposal-ref acceptance, local provider boundary, specification conversion, narrow Engineering Memory, and committed tests/docs. |
| **Non-goals** | No merge authorization; no claim that V0 implements H1–H9; no Bounded-Auto, hosted provider, general memory, UI, marketplace, or production release. |
| **Deliverables** | Independent review record; retained/extended/temporary contract matrix; stale-doc reconciliation; security and recovery evidence; explicit accept/return/defer decision. |
| **Required ADRs** | None are accepted by implication. Proposed ADR-0015–ADR-0024 may be reviewed in parallel but do not ratify the candidate. |
| **Acceptance scenarios** | Authenticated plan and completion decisions remain separate; green evidence produces only a proposal ref; `Deferred` remains recoverable; base branch and primary worktree remain unchanged. |
| **Failure scenarios** | Invalid spec/plan/id/path/ref is rejected without partial state; worker/provider/gate/acceptance interruption becomes failed or uncertain honestly; proposal-ref conflict is not overwritten. |
| **Security scenarios** | DEV runs disclose ambient host authority; unauthorized repositories are refused; traversal/symlink/ref/URL attacks fail closed; no credential leakage. |
| **Measurable exit evidence** | Review findings dispositioned; candidate head pinned; relevant tests independently reproduced or gaps recorded; documentation matches implemented V0; one explicit baseline decision. |
| **Dependencies** | Draft PR #1, docs 01–34, risk register, test evidence. |
| **Next authorization** | Only an accepted baseline decision plus accepted ADR-0015 and an explicit H1 authorization may start H1 implementation. |

## H1 — Governed Specification Workflow

| Field | Gate contract |
| --- | --- |
| **Purpose** | Make user intent, success, exclusions, assumptions, risks, and evidence requirements a durable governance contract before planning. |
| **Entry gate** | Build Feature Baseline Gate accepted; ADR-0015 accepted; contract migration/coexistence plan approved. |
| **Scope** | `MissionCharter`, `EngineeringSpecification`, `OutcomeContract`, clarification, review, approval, immutable versions, verification bindings, specification change requests, projections/exports. |
| **Non-goals** | No delivery-plan generation, Hermes execution, UI breadth, or autonomous specification approval. |
| **Deliverables** | Versioned schemas and lifecycle; authority/traceability rules; Core commands/events; validators; canonical storage and Markdown projection; CLI/API conformance fixtures. |
| **Required ADRs** | [ADR-0015 — specification as executable governance contract](adr/ADR-0015-governed-specification-contract.md). |
| **Acceptance scenarios** | Describe→clarify→review→authenticated approval; approved version binds every criterion to required evidence; same commands work through supported callers. |
| **Failure scenarios** | Missing exclusions, unresolved questions, untestable criteria, stale expected version, or unauthorized approval blocks progression without mutating the approved version. |
| **Security scenarios** | Prompt/repository content cannot mint requirements, approval, capabilities, or evidence; classified content is filtered and attributed. |
| **Measurable exit evidence** | 100% lifecycle conformance fixtures pass; mutation attempts against an approved version fail; full charter→requirement→outcome/evidence trace reconstructs from durable state. |
| **Dependencies** | Baseline ledger/CAS/command semantics; [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md). |
| **Next authorization** | Architecture/product/security review explicitly authorizes H2 after the H1 evidence packet is accepted. |

## H2 — Brain Planning, Phase, Kanban, and Change Control

| Field | Gate contract |
| --- | --- |
| **Purpose** | Turn an approved specification into a validated, phased delivery proposal while Core preserves authority and flow truth. |
| **Entry gate** | H1 closed; ADR-0016 and ADR-0017 accepted. |
| **Scope** | Brain Agent inputs/outputs, `DeliveryPlan`, tailored phase graph, `PhasePlan`, `TaskPacket`, requirement/task/evidence traceability, risks/budgets, native Kanban states, enforced WIP, plan approval, spec-vs-plan change classification. |
| **Non-goals** | No plan self-approval, builder scope redefinition, skill runtime, uncontrolled parallel writes, or UI board. |
| **Deliverables** | Plan validators; phase/task/WIP state machines; change-request contracts; Core transition/event rules; deterministic fixtures for plan approval, return, revision, and resume. |
| **Required ADRs** | [ADR-0016 — Brain-plan/builder-execution authority separation](adr/ADR-0016-brain-plan-builder-execution-separation.md); [ADR-0017 — phase, Kanban, and controlled-change semantics](adr/ADR-0017-phase-kanban-controlled-change.md). |
| **Acceptance scenarios** | Approved spec→validated phased plan→authenticated plan approval; requirements map to tasks and evidence; task packets cannot exceed phase scope; WIP admits only policy-allowed work. |
| **Failure scenarios** | Invalid phase dependency, duplicate task, missing trace, over-budget plan, stale version, or WIP overflow is rejected; execution evidence that changes WHAT opens a specification change request. |
| **Security scenarios** | Brain output is untrusted; requested tools/skills/writable scope are policy inputs, not grants; plan text cannot bypass authorization. |
| **Measurable exit evidence** | Traceability completeness 100% on fixtures; no invalid transition/WIP overflow accepted; replay reconstructs every plan and change decision with actor/version. |
| **Dependencies** | H1 authoritative artifacts; Core command/ledger semantics. |
| **Next authorization** | H3 begins only after plan/change/WIP fixtures and authority review pass. |

## H3 — Hermes Skill Kernel and Hook Runtime

| Field | Gate contract |
| --- | --- |
| **Purpose** | Make Hermes a governed engineering runtime with executable, versioned procedures and typed lifecycle extension points. |
| **Entry gate** | H2 closed; ADR-0018 accepted. |
| **Scope** | Attempt-scoped Agent Kernel; skill manifest/runtime/router; initial bounded skill families; typed Hook Bus; evidence contracts; compatibility/trust metadata; policy-mediated effect proposals. |
| **Non-goals** | No marketplace, arbitrary executable plugins, free-form hook scripts, universal skill catalog, or durable state authority inside Hermes. |
| **Deliverables** | Skill schema and fixture harness; routing record; hook taxonomy and ordering; failure/output/evidence schemas; compatibility tests; initial repository exploration/architecture/build/test/security/documentation skills. |
| **Required ADRs** | [ADR-0018 — Hermes Skill Runtime and Hook Bus](adr/ADR-0018-hermes-skill-runtime-hook-bus.md). |
| **Acceptance scenarios** | A task packet deterministically resolves an eligible skill/profile/tool set; procedure executes within granted bounds; verification and evidence are recorded; observational hooks cannot affect outcome. |
| **Failure scenarios** | Missing/incompatible/revoked skill, malformed output, hook timeout, failed validation, or absent evidence stops or escalates according to contract. |
| **Security scenarios** | Hooks cannot grant capabilities, alter higher-authority artifacts, access undeclared data, or produce unrecorded effects; effect-producing hooks traverse the same firewall. |
| **Measurable exit evidence** | Skill and hook conformance suites pass; identical inputs yield the same resolution record; adversarial bypass attempts produce zero unauthorized effect. |
| **Dependencies** | H2 task/phase contracts; WWP and effect boundary; [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md). |
| **Next authorization** | H4 requires accepted H3 evidence and explicit authorization to add context intelligence adapters. |

## H4 — Context Compiler, LSP, and Hybrid Retrieval

| Field | Gate contract |
| --- | --- |
| **Purpose** | Supply minimal, cited, reproducible task context and language intelligence without confusing similarity with truth. |
| **Entry gate** | H3 closed; ADR-0019 accepted; initial language/repository fixtures approved. |
| **Scope** | Context Compiler pipeline and manifests; trust/freshness/provenance; normalized LSP broker; limited initial adapters; lexical, LSP, structural AST/tree-sitter, Git, ADR/spec/evidence/memory retrieval; semantic ranking as subordinate signal. |
| **Non-goals** | No universal language support, semantic-only retrieval, cross-project cloud index, or provider-specific context contract. |
| **Deliverables** | Context item/pack schemas; ranking and budget policy; adapter conformance fixtures; affected-file/test mappings; reproducibility and omission reports; evaluated tree-sitter/vector choices. |
| **Required ADRs** | [ADR-0019 — Context Compiler, LSP, and hybrid retrieval](adr/ADR-0019-context-compiler-lsp-hybrid-retrieval.md). |
| **Acceptance scenarios** | Same repository commit/task/config produces the same manifest; authoritative exact/policy sources outrank semantic hits; LSP diagnostics and impact evidence bind to file/symbol/version. |
| **Failure scenarios** | Stale index, unavailable LSP, parse failure, token overflow, contradictory sources, or missing required context becomes explicit degraded/blocked state, never silent omission. |
| **Security scenarios** | Repository/tool content is untrusted; secrets/classification are filtered; retrieval cannot widen readable scope; provenance survives compression. |
| **Measurable exit evidence** | Pack hash/manifest reproducibility; required-source recall floor on curated fixtures; zero out-of-scope retrieval; freshness and omission labels present in every pack. |
| **Dependencies** | H3 skills/hooks; H1 governance sources; evaluated adapter/tool choices. |
| **Next authorization** | H5 begins only after context quality and security evidence are accepted. |

## H5 — Controlled Engineering Loops

| Field | Gate contract |
| --- | --- |
| **Purpose** | Let Hermes diagnose, act minimally, verify, retry, replan, escalate, or stop under fixed authority and budget. |
| **Entry gate** | H4 closed; ADR-0022 accepted; loop budgets and guard thresholds approved. |
| **Scope** | Observe→diagnose→hypothesize→select→execute→verify→compare→update; iteration records; repeated/no-progress/oscillation/worsening/schema/budget/uncertainty guards; escalation ladder; controlled replanning. |
| **Non-goals** | No open-ended autonomy, hidden chain-of-thought requirement, infinite retries, or automatic acceptance. |
| **Deliverables** | Loop state and evidence schema; guard detector; budget accounting; recovery/resume rules; escalation and replan commands; adversarial/no-progress fixtures. |
| **Required ADRs** | [ADR-0022 — controlled loop and escalation semantics](adr/ADR-0022-controlled-loop-escalation.md). |
| **Acceptance scenarios** | New evidence changes a hypothesis/action; successful convergence satisfies unchanged gates; a justified replan preserves traceability and approval rules. |
| **Failure scenarios** | Repeated action, zero state change, oscillation, increasing diagnostics, schema failures, exhausted budget, invalid plan, and unresolved uncertainty stop or escalate honestly. |
| **Security scenarios** | Every effect follows classify→policy→capability→approval if required→durable intent→execute→probe→evidence; retries cannot replay uncertain non-idempotent effects silently. |
| **Measurable exit evidence** | All guard fixtures detected within configured bounds; zero completion after guard/authority breach; recovery and non-convergence outcomes are distinguishable and replayable. |
| **Dependencies** | H2 change control; H3 effect hooks; H4 context evidence. |
| **Next authorization** | H6 requires the controlled-loop evidence packet and an approved bounded-parallelism policy. |

## H6 — Bounded Subagents and Independent Self-Review

| Field | Gate contract |
| --- | --- |
| **Purpose** | Add specialized parallel insight and independent review without authority leakage or an uncontrolled swarm. |
| **Entry gate** | H5 closed; ADR-0021 accepted; WIP/resource limits approved. |
| **Scope** | Subagent Supervisor; explorer, architecture, implementer, test, security, performance, documentation, recovery roles; structured handoffs; bounded read-only parallelism; isolated writable work; reviewer/test/security chain. |
| **Non-goals** | No free agent chat, arbitrary delegation trees, shared writable workspace, self-approval, or remote fleet. |
| **Deliverables** | Assignment/finding/handoff contracts; supervisor scheduling rules; conflict controls; independent-context rules; review disposition and evidence schemas. |
| **Required ADRs** | [ADR-0021 — bounded subagents and structured handoffs](adr/ADR-0021-bounded-subagents-structured-handoffs.md). |
| **Acceptance scenarios** | Bounded explorers run in parallel and return structured findings; one isolated writer changes code; independent reviewers receive required evidence without relying solely on builder context. |
| **Failure scenarios** | Timeout, contradictory finding, malformed handoff, writer conflict, lost subagent, or review disagreement becomes blocked/uncertain/escalated with evidence. |
| **Security scenarios** | Subagents cannot approve plans/effects/completion, expand scope, contact users, inherit undeclared secrets, or bypass Core recording. |
| **Measurable exit evidence** | Concurrency never exceeds limits; zero cross-worktree mutation; every durable finding has provenance; reviewer/test/security independence fixtures pass. |
| **Dependencies** | H2 WIP/task packets; H3 skills; H5 loop/supervision semantics. |
| **Next authorization** | H7 begins only after supervisor authority and isolation review passes. |

## H7 — Typed Memory Intelligence

| Field | Gate contract |
| --- | --- |
| **Purpose** | Convert eligible evidence into scoped, fresh, contradiction-aware learning without turning memory into governance. |
| **Entry gate** | H6 closed; ADR-0020 accepted; retention/classification policy approved. |
| **Scope** | Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory; Memory Candidate; Memory Judge; consolidation, deduplication, contradiction, confidence, freshness, expiry, supersession, retrieval limits. |
| **Non-goals** | No cross-project/cross-customer learning, autonomous policy rewriting, raw transcript memory, silent lesson promotion, or cloud sync. |
| **Deliverables** | Typed schemas/lifecycles; judge rules and evidence threshold; scope/retention/security controls; consolidation and retrieval fixtures; memory-use ledger facts. |
| **Required ADRs** | [ADR-0020 — typed memory and Memory Judge](adr/ADR-0020-typed-memory-memory-judge.md). |
| **Acceptance scenarios** | Accepted evidence yields a candidate; judge promotes only eligible non-contradictory content; later task receives bounded relevant memory with authority/trust labels. |
| **Failure scenarios** | Rejected/cancelled/uncertain missions, weak evidence, duplicate, contradiction, staleness, or scope mismatch block promotion/application and remain visible. |
| **Security scenarios** | Memory cannot change policy/spec/criteria/capabilities; governance memory is authoritative while experiential memory is labelled; classification/project boundaries fail closed. |
| **Measurable exit evidence** | Zero ineligible promotions in adversarial fixtures; contradiction/supersession/freshness tests pass; every applied item traces to evidence and judge decision. |
| **Dependencies** | H6 review evidence; H4 retrieval; baseline memory lessons treated as temporary V0 input. |
| **Next authorization** | H8 requires accepted memory-quality evidence and frozen evaluation fixtures. |

## H8 — Model-Independent Outcome Convergence and Ablation

| Field | Gate contract |
| --- | --- |
| **Purpose** | Prove “Different brains. Same engineering truth” as contract-equivalent acceptance, not identical code or equal model capability. |
| **Entry gate** | H7 closed; ADR-0023 and ADR-0024 accepted; evaluation charter and fixed fixture versions approved. |
| **Scope** | Outcome Equivalence Contract; profile certification; fixed mission/repo/spec/policy/tools/environment/budget; builder/brain variation; LSP/RAG/memory/loops/subagents/skill-routing ablations; safety/evidence/cost/convergence metrics. |
| **Non-goals** | No byte-identical output promise, quality-bar reduction, universal provider support, benchmark-only optimization, or claim that weak/strong models are equal. |
| **Deliverables** | Certification matrix; harness manifests; raw evidence/results; statistical and small-sample caveats; escalation/non-convergence criteria; component ablation reports. |
| **Required ADRs** | [ADR-0023 — model-independent outcome equivalence](adr/ADR-0023-model-independent-outcome-equivalence.md); [ADR-0024 — harness evaluation and provider certification](adr/ADR-0024-harness-evaluation-provider-certification.md). |
| **Acceptance scenarios** | Multiple supported profiles attempt identical approved missions; accepted outputs meet identical functional/contract/architecture/security/quality/evidence thresholds. |
| **Failure scenarios** | A profile exceeds attempts/budget, cannot satisfy gates, or produces uncertain evidence and stops/escalates honestly without lowering the bar. |
| **Security scenarios** | Unsafe-effect and evidence-forgery rates are first-class blockers; harness isolation prevents fixture/data leakage and uncontrolled external effects. |
| **Measurable exit evidence** | Published outcome-equivalence, gate-pass, regression, unsafe-effect, evidence-completeness, convergence, cost/time, intervention, escalation, recovery, and honesty results; ablations identify measured contribution or no effect. |
| **Dependencies** | H1–H7 operational contracts; [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md); [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md). |
| **Next authorization** | H9 begins only after runtime contracts are declared stable enough for surfaces and the H8 evidence is accepted. |

## H9 — Product Surfaces After Runtime Truth

| Field | Gate contract |
| --- | --- |
| **Purpose** | Let users operate outcomes, decisions, risks, and evidence through truthful surfaces over one Core workflow. |
| **Entry gate** | H8 closed; shared Core command/query/subscription contracts versioned and stable; surface scope approved. |
| **Scope** | CLI refinement; Mission Control; Spec Review; Plan Review; Kanban; Decisions; Risks; Evidence; Change Requests; Completion Review; shared CLI/Studio/MCP/API semantics; accessibility and degraded-state behavior. |
| **Non-goals** | No full graphical IDE, client-owned workflow state, chat-first assistant, marketplace, remote collaboration, or autonomous production deployment. |
| **Deliverables** | Contract-generated clients/adapters; workflow surfaces; evidence navigation; live/stale/uncertain states; accessibility fixtures; surface conformance and usability evidence. |
| **Required ADRs** | No new authority ADR is implied; any material transport/identity/data-flow change requires a new Proposed ADR. ADR-0015–ADR-0024 remain binding where accepted. |
| **Acceptance scenarios** | Describe→approve spec→approve plan→observe phase/Kanban→resolve decisions/changes→review evidence→accept/return/defer/cancel through each supported caller with equivalent Core outcomes. |
| **Failure scenarios** | Disconnect, stale revision, partial stream, Core restart, unavailable adapter, and uncertain mission render accurately and resume without client mutation. |
| **Security scenarios** | Authenticated identity, CSRF/replay/spoof resistance, evidence-link resolution, classification filtering, and no privileged UI bypass. |
| **Measurable exit evidence** | Command/outcome conformance across callers; zero client-only authoritative states; accessibility/usability acceptance; users correctly identify mission state, risks, next decision, and evidence. |
| **Dependencies** | Stable H1–H8 contracts and trustworthy projections. |
| **Next authorization** | Architecture/product/security review decides whether to enter release preparation; ecosystem, remote, marketplace, and deployment automation remain separately gated. |

## Gate governance

Every exit review includes product, Core, quality, and security authority. A visual demo, model narrative, passing happy path, or Draft PR status is insufficient. Evidence must include normal, denied, degraded, recovery, and adversarial behavior. Material scope or authority changes require a revised gate contract and a new Proposed ADR where applicable.

See also: [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), [21_Project_Backlog.md](21_Project_Backlog.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), and [28_Release_Strategy.md](28_Release_Strategy.md).
