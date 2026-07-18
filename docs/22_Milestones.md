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
| **Purpose** | Resolve how H1 prerequisite contracts will be supplied without making candidate acceptance the only viable outcome. |
| **Entry gate** | Architecture package available; PR #1 final head identified; independent reviewers assigned. |
| **Scope** | Candidate contracts, staged plan/completion approvals, ledger/CAS/worktree behavior, DEV disclosure, proposal-ref acceptance, local provider boundary, specification conversion, narrow Engineering Memory, and committed tests/docs. |
| **Non-goals** | No merge authorization; no claim that V0 implements H1–H9; no Bounded-Auto, hosted provider, general memory, UI, marketplace, or production release. |
| **Deliverables** | Independent review record; retained/extended/temporary contract matrix; stale-doc reconciliation; security and recovery evidence; explicit accept/return/defer/reject disposition; and either an accepted-prerequisite inventory or an approved replacement-foundation plan for every gap. |
| **Required ADRs** | None are accepted by implication. ADR-0015–ADR-0026 may be reviewed in parallel but do not ratify the candidate. |
| **Acceptance scenarios** | Authenticated plan and completion decisions remain separate; green evidence produces only a proposal ref; `Deferred` remains recoverable; base branch and primary worktree remain unchanged. |
| **Failure scenarios** | Invalid spec/plan/id/path/ref is rejected without partial state; worker/provider/gate/acceptance interruption becomes failed or uncertain honestly; proposal-ref conflict is not overwritten. |
| **Security scenarios** | DEV runs disclose ambient host authority; unauthorized repositories are refused; traversal/symlink/ref/URL attacks fail closed; no credential leakage. |
| **Measurable exit evidence** | Review findings dispositioned; candidate head pinned; relevant tests independently reproduced or gaps recorded; documentation matches implemented V0; candidate disposition recorded; gate marked **Resolved** with the exact accepted-contract or approved replacement-foundation path. |
| **Dependencies** | Draft PR #1, docs 01–35, risk register, test evidence. |
| **Next authorization** | H1 may start only after the gate is **Resolved**, required prerequisite contracts are accepted from the candidate **or** an explicit replacement-foundation plan is approved, ADR-0015 and early-spine ADR-0024 are accepted, the spine is operational, the pre-H1 foundation Baseline `EvaluationRun` defined by document 34 has reached an honest terminal state, been assessed with required observations validated and deviations dispositioned, and produced a finalized `EvaluationResult`, and H1 is explicitly authorized. A returned, deferred, or rejected PR #1 is not a permanent blocker. |

## H1 — Governed Specification Workflow

| Field | Gate contract |
| --- | --- |
| **Purpose** | Make user intent, success, exclusions, assumptions, risks, and evidence requirements a durable governance contract before planning. |
| **Entry gate** | Build Feature Baseline Gate **Resolved** through an accepted-contract or approved replacement-foundation path; ADR-0015 and early-spine ADR-0024 accepted; contract migration/coexistence plan approved; minimum evaluation contracts operational; pre-H1 foundation Baseline `EvaluationRun` terminal and assessed with required observations validated, deviations dispositioned, and `EvaluationResult` finalized. |
| **Scope** | `MissionCharter`, `EngineeringSpecification`, `OutcomeContract`, clarification, review, approval, immutable versions, verification bindings, specification change requests, projections/exports. |
| **Non-goals** | No delivery-plan generation, Hermes execution, UI breadth, or autonomous specification approval. |
| **Deliverables** | Versioned schemas and lifecycle; authority/traceability rules; Core commands/events; validators; canonical storage and Markdown projection; CLI/API conformance fixtures; `EvaluationCase`, `EvaluationRun`, `TreatmentArm`, `RunManifest`, `MetricObservation`, `ProtocolDeviation`, and `EvaluationResult` evidence with exact provenance and a baseline/regression comparison. |
| **Required ADRs** | [ADR-0015 — specification as executable governance contract](adr/ADR-0015-governed-specification-contract.md); [ADR-0024 — evaluation spine and run provenance](adr/ADR-0024-evaluation-spine-run-provenance.md). |
| **Acceptance scenarios** | Describe→clarify→review→authenticated approval; approved version binds every criterion to required evidence; same commands work through supported callers. |
| **Failure scenarios** | Missing exclusions, unresolved questions, untestable criteria, stale expected version, or unauthorized approval blocks progression without mutating the approved version. |
| **Security scenarios** | Prompt/repository content cannot mint requirements, approval, capabilities, or evidence; classified content is filtered and attributed. |
| **Measurable exit evidence** | 100% lifecycle conformance fixtures pass; mutation attempts against an approved version fail; full charter→requirement→outcome/evidence trace reconstructs from durable state; the H1 evaluation run and exact manifest reproduce and compare cleanly with the registered baseline. |
| **Dependencies** | Baseline ledger/CAS/command semantics; [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md). |
| **Next authorization** | Architecture/product/security review explicitly authorizes H2 after the H1 evidence packet is accepted. |

## H2 — Brain Planning, Phase, Kanban, and Change Control

| Field | Gate contract |
| --- | --- |
| **Purpose** | Qualify a Brain-produced plan proposal into an approved phased delivery plan while Core preserves authority and flow truth. |
| **Entry gate** | H1 closed; ADR-0016 and ADR-0017 accepted. |
| **Scope** | Brain Agent inputs/outputs; `PlanProposal`; deterministic Plan Compiler/normalization; Plan Candidate; structural validation; architecture/risk/evidence `PlanAssessment`; risk-tiered independent review; authenticated `PlanDecision`; approved `DeliveryPlan`; tailored phase graph, `PhasePlan`, `TaskPacket`; deterministic `SOPGraph`; authorized role subscriptions; traceability, risks/budgets, Kanban/WIP, and change classification. |
| **Non-goals** | No plan self-approval, builder scope redefinition, skill runtime, uncontrolled parallel writes, or UI board. |
| **Deliverables** | `PlanProposal`, `PlanAssessment`, and `PlanDecision` schemas/lifecycles; deterministic Plan Compiler/validators; risk-tier/reviewer-independence policy; `SOPGraph`, `RoleNode`, `ActionContract`, authorized `InputSubscription`, `OutputContract`, dependency/evidence/stop/escalation edges and exact parent versions; phase/task/WIP state machines; change contracts; Core transition/event rules. |
| **Required ADRs** | [ADR-0016 — Brain-plan/builder-execution authority separation](adr/ADR-0016-brain-plan-builder-execution-separation.md); [ADR-0017 — phase, Kanban, and controlled-change semantics](adr/ADR-0017-phase-kanban-controlled-change.md). |
| **Acceptance scenarios** | Brain→proposal→deterministic compile→candidate→structural validation→architecture/risk/evidence assessment→independent review when required→authenticated Core decision→approved plan; requirements map to tasks/evidence; task packets remain within scope; WIP admits only policy-allowed work. |
| **Failure scenarios** | Missing Outcome Contract/evidence coverage, invalid DAG, architecture deviation, disproportionate scope, unmitigated risk, infeasible budget/WIP, weak rollback/recovery, unresolved assumptions, blocking finding, stale version, or WIP overflow is rejected or returned; execution evidence that changes WHAT opens a specification change request. |
| **Security scenarios** | Brain output is untrusted; the producer cannot approve or supply the only acceptance-critical review; medium/high risk receives independent assessment; model voting is not authority; roles cannot self-subscribe or observe a free shared environment; Core projects only assignment-authorized inputs. |
| **Measurable exit evidence** | Traceability completeness 100% on fixtures; assessment covers every mandated field and reviewer identity/independence; no producer self-approval, invalid transition, or WIP overflow is accepted; replay reconstructs every proposal, assessment, decision, and change with actor/version; low-risk and medium/high-risk policy paths pass. |
| **Dependencies** | H1 authoritative artifacts; Core command/ledger semantics. |
| **Next authorization** | H3 begins only after plan/change/WIP fixtures and authority review pass. |

## H3 — Hermes Skill Kernel and Hook Runtime

### H3.1 — Built-in Skill Kernel (minimum H3 exit)

| Field | Gate contract |
| --- | --- |
| **Purpose** | Prove a small governed Hermes runtime using only repository-owned built-in skills and hooks. |
| **Entry gate** | H2 closed; ADR-0018 accepted. |
| **Scope** | Attempt-scoped Agent Kernel; repository-owned built-in skills; static/versioned manifests/hashes; typed inputs/outputs and lifecycle hooks; `SOPGraph` role/action execution; authorized input subscriptions; capability-projected tool schemas; context/MCP-catalog budgets; bounded/artifactized tool results; internal control/events; policy-mediated effects. |
| **Non-goals** | No external package installation, public registry, marketplace, third-party executable hooks, generalized signing infrastructure, public SDK/protocol, ACP/MCP/editor adapter, arbitrary scripts, universal catalog, or durable state authority inside Hermes. |
| **Deliverables** | Built-in skill schema/harness; exact-hash routing; hook taxonomy; role-subscription projection; per-invocation tool-catalog manifest derived from policy/task/capabilities/role/sandbox/classification/phase/budget; tool-result budget/artifact contract; internal control/event schemas; compatibility tests; initial bounded skill set. |
| **Required ADRs** | [ADR-0018 — Hermes Skill Runtime and Hook Bus](adr/ADR-0018-hermes-skill-runtime-hook-bus.md). |
| **Acceptance scenarios** | A task deterministically resolves an eligible built-in skill/profile/tool set; denied tools are absent from model-visible schemas where possible; MCP/context catalog stays within budget; only authorized upstream artifacts reach the role; oversized results retain bounded preview and retrievable full artifact; Effect Firewall still decides effects. |
| **Failure scenarios** | Missing/incompatible built-in skill, hash mismatch, malformed/duplicate/out-of-order event, reconnect mismatch, hook timeout, failed validation, or absent evidence stops or escalates according to contract. |
| **Security scenarios** | Hooks/internal clients cannot grant capabilities, self-subscribe, alter higher artifacts, access undeclared data, forge results, or produce unrecorded effects; schema omission is defense in depth, not final enforcement; extensions/packages with ambient host authority and automatic trust of project-local executable packages are rejected. |
| **Measurable exit evidence** | Built-in skill/hook/internal-port conformance suites pass; identical inputs yield the same resolution record; reconnect reproduces the same projection hash; adversarial bypass attempts produce zero unauthorized effect; evaluation demonstrates measurable value without governance regression. |
| **Dependencies** | H2 task/phase contracts; WWP and effect boundary; [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md). |
| **Next authorization** | H4 requires accepted H3.1 evidence and explicit authorization to add context intelligence adapters. H3.2 is optional and separately gated. |

### H3.2 — Governed Skill Packaging (later extension)

| Field | Gate contract |
| --- | --- |
| **Entry gate** | H3.1 closed with measured benefit and no governance regression; packaging threat model, provenance policy, and explicit architecture/security authorization approved. |
| **Scope** | Package staging; signature and provenance verification; Core-owned atomic activation; rollback; revocation. |
| **Non-goals** | No public marketplace, arbitrary third-party execution, or package-authority bypass. |
| **Measurable exit evidence** | Malicious, partial, incompatible, revoked, and permission-expanding packages never activate; activation/rollback/revocation are atomic and replayable; packaging benefit exceeds operational and security cost. |
| **Rollback/rejection** | Disable the package path and retain H3.1 built-ins if benefit is not material or any governance/security threshold fails. |

## H4 — Context Compiler, LSP, and Hybrid Retrieval

### H4.1 — Reproducible Context and Initial LSP (minimum H4 exit)

| Field | Gate contract |
| --- | --- |
| **Purpose** | Supply minimal, cited, reproducible task context and language intelligence without confusing similarity with truth. |
| **Entry gate** | H3.1 closed; ADR-0019 accepted; initial language/repository fixtures approved. H3.2 packaging is not a prerequisite. |
| **Scope** | Authority-first Context Compiler/manifests; exact path/identifier, lexical, Git, ADR/spec/evidence retrieval; one rust-analyzer adapter; read-only `MissionExplorationBranch`; governed `CompactionRecord`; bounded full-tool-artifact retrieval; trust/freshness/provenance. |
| **Non-goals** | No AST/tree-sitter or semantic retrieval in the minimum exit; no embeddings, additional LSP adapters, universal language support, cross-project cloud index, or provider-specific contract. |
| **Deliverables** | Context item/pack schemas; exact manifest; ranking/budget policy; branch record with parent/objective/context hash/permissions/budget/findings/evidence/status/contribution; compaction record with mandatory authority anchors, omissions and full source hash; rust-analyzer fixtures; reproducibility/freshness/omission reports. |
| **Required ADRs** | [ADR-0019 — Context Compiler, LSP, and hybrid retrieval](adr/ADR-0019-context-compiler-lsp-hybrid-retrieval.md). |
| **Acceptance scenarios** | Same repository commit/task/config produces the same manifest; authoritative exact/policy sources outrank every derived signal; exact, lexical, Git, and rust-analyzer evidence binds to file/symbol/version. |
| **Failure scenarios** | Stale index, unavailable LSP, parse failure, token overflow, contradictory source, missing authority anchor, non-rehydratable compaction, or unauthorized branch contribution becomes explicit degraded/blocked state, never silent omission. |
| **Security scenarios** | Repository/tool content is untrusted; secrets/classification are filtered; retrieval/branching cannot widen scope; summaries/transcripts never supersede Core artifacts; full tool artifacts retain permission, expiry, classification, and content hash. |
| **Measurable exit evidence** | Pack hash/manifest reproducibility; required-source recall floor on curated fixtures; rust-analyzer conformance passes; zero out-of-scope retrieval; freshness/omission labels present in every pack; H4.1 treatment improves the preregistered baseline without governance harm. |
| **Dependencies** | H3.1 built-in skills/hooks; H1 governance sources; evaluated adapter/tool choices. |
| **Next authorization** | H5 may begin after H4.1 context quality/security evidence is accepted. H4.2 and H4.3 remain optional, separately measured extensions. |

### H4.2 — Structural and Impact Intelligence

| Field | Gate contract |
| --- | --- |
| **Entry gate** | H4.1 closed; structural fixtures and adapter conformance thresholds approved. |
| **Scope** | AST/tree-sitter retrieval; affected-file and affected-test mapping; additional LSP adapters only after each passes the normalized conformance suite. |
| **Measurable exit evidence** | Structural/impact treatment improves relevant recall and change accuracy without freshness, scope, latency, or security regression. |
| **Rollback/rejection** | Remove failing adapters/signals and retain H4.1. |

### H4.3 — Governed Semantic Retrieval

| Field | Gate contract |
| --- | --- |
| **Entry gate** | H4.1 baseline exists; H4.2 need is understood; controlled semantic-treatment ablation and security/privacy review are preregistered. |
| **Scope** | Semantic retrieval and embeddings as subordinate, attributable signals. |
| **Default** | Disabled unless controlled ablation shows practical benefit without unacceptable authority, security, freshness, cost, or token harm. |
| **Rollback/rejection** | Disable semantic retrieval and retain exact/lexical/Git/LSP paths plus any separately admitted structural paths if any threshold fails. |

## H5 — Controlled Engineering Loops

| Field | Gate contract |
| --- | --- |
| **Purpose** | Let Hermes diagnose, act minimally, verify, retry, replan, escalate, or stop under fixed authority and budget. |
| **Entry gate** | H4.1 closed with accepted evidence; H4.2 and H4.3 are optional and not prerequisites; ADR-0022 accepted; loop budgets and guard thresholds approved. |
| **Scope** | Observe→diagnose→hypothesize→select→execute→verify→compare→update; iteration records; repeated/no-progress/oscillation/worsening/schema/budget/uncertainty guards; escalation ladder; controlled replanning. |
| **Non-goals** | No open-ended autonomy, hidden chain-of-thought requirement, infinite retries, or automatic acceptance. |
| **Deliverables** | Loop state/evidence schema; typed sandbox/capability denial result (boundary, reason, attempted effect, retryability, safe alternatives, required authority, recovery state, evidence ref); guard detector; budget accounting; recovery/resume; advisory risk-classifier treatment; adversarial/no-progress fixtures. |
| **Required ADRs** | [ADR-0022 — controlled loop and escalation semantics](adr/ADR-0022-controlled-loop-escalation.md). |
| **Acceptance scenarios** | New evidence changes a hypothesis/action; successful convergence satisfies unchanged gates; a justified replan preserves traceability and approval rules. |
| **Failure scenarios** | Repeated action or identical denial without changed hypothesis/capability/plan/authority, zero change, oscillation, increasing diagnostics, schema failure, exhausted budget, invalid plan, and uncertainty stop or escalate honestly. |
| **Security scenarios** | Every effect follows classify→policy→capability→approval if required→durable intent→execute→probe→evidence. A contextual risk advisor may recommend allow-under-existing-policy/narrow/block/request authority but cannot grant, override deterministic deny, approve protected effect, change policy, or be the sole boundary. |
| **Measurable exit evidence** | All guard fixtures detected within configured bounds; zero completion after guard/authority breach; recovery and non-convergence outcomes are distinguishable and replayable. |
| **Dependencies** | H2 change control; H3 effect hooks; H4 context evidence. |
| **Next authorization** | H6 requires the controlled-loop evidence packet and an approved bounded-parallelism policy. |

## H6 — Bounded Subagents and Independent Self-Review

| Field | Gate contract |
| --- | --- |
| **Purpose** | Add specialized parallel insight and independent review without authority leakage or an uncontrolled swarm. |
| **Entry gate** | H5 closed; ADR-0021 accepted; WIP/resource limits approved. |
| **Scope** | Subagent Supervisor; explorer, architecture, implementer, test, security, performance, documentation, recovery roles; SOPGraph-authorized subscriptions; structured handoffs; bounded read-only parallelism; isolated writable work; reviewer/test/security chain; contextual risk-advisor experiment. |
| **Non-goals** | No free agent chat, arbitrary delegation trees, shared writable workspace, self-approval, or remote fleet. |
| **Deliverables** | Assignment/finding/handoff contracts; supervisor scheduling rules; conflict controls; independent-context rules; review disposition and evidence schemas. |
| **Required ADRs** | [ADR-0021 — bounded subagents and structured handoffs](adr/ADR-0021-bounded-subagents-structured-handoffs.md). |
| **Acceptance scenarios** | Bounded explorers run in parallel and return structured findings; one isolated writer changes code; independent reviewers receive required evidence without relying solely on builder context. |
| **Failure scenarios** | Timeout, contradictory finding, malformed handoff, writer conflict, lost subagent, or review disagreement becomes blocked/uncertain/escalated with evidence. |
| **Security scenarios** | Subagents cannot approve plans/effects/completion, expand scope, self-subscribe, contact users, inherit undeclared secrets, broadcast freely, chat peer-to-peer outside typed handoffs, or bypass Core recording. |
| **Measurable exit evidence** | Concurrency never exceeds limits; zero cross-worktree mutation; every durable finding has provenance; reviewer/test/security independence fixtures pass. |
| **Dependencies** | H2 WIP/task packets; H3 skills; H5 loop/supervision semantics. |
| **Next authorization** | H7 begins only after supervisor authority and isolation review passes. |

## H7 — Typed Memory Intelligence

| Field | Gate contract |
| --- | --- |
| **Purpose** | Convert eligible evidence into scoped, fresh, contradiction-aware learning without turning memory into governance. |
| **Entry gate** | H6 closed; ADR-0020 accepted; retention/classification policy approved. |
| **Scope** | Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory; Memory Candidate; Memory Judge; accepted/rejected Mission Exploration Branch contributions; consolidation, deduplication, contradiction, confidence, freshness, expiry, supersession, retrieval limits. |
| **Non-goals** | No cross-project/cross-customer learning, autonomous policy rewriting, raw transcript memory, silent lesson promotion, or cloud sync. |
| **Deliverables** | Typed schemas/lifecycles; judge rules and evidence threshold; scope/retention/security controls; consolidation and retrieval fixtures; memory-use ledger facts. |
| **Required ADRs** | [ADR-0020 — typed memory and Memory Judge](adr/ADR-0020-typed-memory-memory-judge.md). |
| **Acceptance scenarios** | Accepted evidence yields a candidate; judge promotes only eligible non-contradictory content; later task receives bounded relevant memory with authority/trust labels. |
| **Failure scenarios** | Rejected/cancelled/uncertain missions, weak evidence, duplicate, contradiction, staleness, or scope mismatch block promotion/application and remain visible. |
| **Security scenarios** | Memory cannot change policy/spec/criteria/capabilities; governance memory is authoritative while experiential memory is labelled; branch transcripts and compaction summaries are not truth; classification/project boundaries fail closed. |
| **Measurable exit evidence** | Zero ineligible promotions in adversarial fixtures; contradiction/supersession/freshness tests pass; every applied item traces to evidence and judge decision. |
| **Dependencies** | H6 review evidence; H4 retrieval; baseline memory lessons treated as temporary V0 input. |
| **Next authorization** | H8 requires accepted memory-quality evidence and frozen evaluation fixtures. |

## H8 — Model-Independent Outcome Convergence and Ablation

| Field | Gate contract |
| --- | --- |
| **Purpose** | Prove “Different brains. Same engineering truth” as contract-equivalent acceptance, not identical code or equal model capability. |
| **Entry gate** | H7 closed; the pre-H1 foundation Baseline run defined in document 34 plus H1–H7 evaluation-spine runs are terminal, assessed, finalized and comparable, including honest failed/aborted outcomes; ADR-0023 and certification ADR-0025 accepted; certification charter and fixed fixture versions approved. |
| **Scope** | Consume accumulated ADR-0024 evidence; Outcome Equivalence Contract; cross-model controlled comparisons; repeated/randomized experiments; provider/profile certification; builder/brain variation, component ablations, and risk-triggered controlled multi-route races; safety/evidence/cost/convergence/portability metrics. |
| **Non-goals** | No byte-identical output promise, quality-bar reduction, universal provider support, benchmark-only optimization, or claim that weak/strong models are equal. |
| **Deliverables** | Certification matrix linked to prior `EvaluationRun` history; controlled/repeated/randomized manifests; raw evidence/results; statistical and small-sample caveats; escalation/non-convergence criteria; component ablation and portability reports. |
| **Required ADRs** | [ADR-0023 — model-independent outcome equivalence](adr/ADR-0023-model-independent-outcome-equivalence.md); [ADR-0024 — evaluation spine and run provenance](adr/ADR-0024-evaluation-spine-run-provenance.md), already operational; [ADR-0025 — model/profile certification](adr/ADR-0025-model-profile-certification.md). |
| **Acceptance scenarios** | Multiple supported profiles or routes attempt identical approved missions under fixed spec/contract/commit/policy/tools/environment/budget/gates; every accepted candidate independently meets identical functional/contract/architecture/security/quality/evidence thresholds. |
| **Failure scenarios** | A profile exceeds attempts/budget, cannot satisfy gates, or produces uncertain evidence and stops/escalates honestly without lowering the bar. |
| **Security scenarios** | Unsafe-effect/evidence-forgery rates are blockers; harness isolation prevents leakage/effects; model vote, visual preference, and relative rank have no acceptance authority. |
| **Measurable exit evidence** | Published outcome-equivalence, gate-pass, regression, unsafe-effect, evidence-completeness, convergence, cost/time, intervention, escalation, recovery, and honesty results; ablations identify measured contribution or no effect. |
| **Dependencies** | H1–H7 operational contracts and their versioned baseline/regression runs; [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md); [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md). |
| **Next authorization** | H9 begins only after runtime contracts are declared stable enough for surfaces and the H8 evidence is accepted. |

## H9 — Product Surfaces After Runtime Truth

| Field | Gate contract |
| --- | --- |
| **Purpose** | Let users operate outcomes, decisions, risks, and evidence through truthful surfaces over one Core workflow. |
| **Entry gate** | H8 closed; shared Core command/query/subscription contracts versioned and stable; surface scope approved. |
| **Scope** | CLI refinement; Mission Control and engineering-team projection of roles/work/blockers/decisions/budgets/evidence/results; Spec/Plan/Completion Review; Kanban; Risks; Evidence; Change Requests; shared CLI/Studio/MCP/API semantics; accessibility and degraded-state behavior. |
| **Non-goals** | No full graphical IDE, client-owned workflow state, chat-first assistant, marketplace, remote collaboration, autonomous production deployment, SEO/ads/growth automation, or no-code hosting platform. |
| **Deliverables** | Contract-generated clients/adapters; workflow/team surfaces; evidence navigation; live/stale/uncertain states; UI EvidenceBundle support for screenshots, video, accessibility tree, browser console, network trace, deterministic interaction scripts; accessibility/surface/usability evidence. |
| **Required ADRs** | No new authority ADR is implied; any material transport/identity/data-flow change requires a new Proposed ADR. ADR-0015–ADR-0026 remain binding where accepted. |
| **Acceptance scenarios** | Describe→approve spec→approve plan→observe phase/Kanban→resolve decisions/changes→review evidence→accept/return/defer/cancel through each supported caller with equivalent Core outcomes. |
| **Failure scenarios** | Disconnect, stale revision, partial stream, Core restart, unavailable adapter, and uncertain mission render accurately and resume without client mutation. |
| **Security scenarios** | Authenticated identity, CSRF/replay/spoof resistance, evidence-link resolution, classification filtering, and no privileged UI bypass. |
| **Measurable exit evidence** | Command/outcome conformance across callers; zero client-only authoritative states; accessibility/usability acceptance; users correctly identify mission state, risks, next decision, and evidence. |
| **Dependencies** | Stable H1–H8 contracts and trustworthy projections. |
| **Next authorization** | Architecture/product/security review decides whether to enter release preparation; ecosystem, remote, marketplace, and deployment automation remain separately gated. |

## Reference-system evidence gate

The [reference study and experiment register](35_Reference_Systems_and_Competitive_Architecture.md) is binding planning input for every idea attributed to Pi, Zed/ACP, Warp, GitHub Spec Kit, Claude Code, Codex, Cursor, OpenCode, Aider, or OpenHands. Before product Core records exist, each candidate uses a signed, versioned repository architecture-review record called `ReferenceIdeaDecision`; this is planning evidence, not product authority. Each applicable exit packet identifies that record, source revision, RS experiment treatment/result, limitations, security outcome, disable path and license/provenance disposition.

The milestone mapping is Baseline/early spine RS-00/RS-11 plus versioned run provenance; H1 RS-01; H2 RS-01/RS-02/RS-15/RS-20 plus RS-21 typed SOP compilation and RS-22 role-subscription design; H3.1 RS-03/RS-05/RS-11/RS-20 plus RS-21/RS-22 execution projection, RS-23 tool schemas, and RS-26 output budgets, with H3.2 later applying the RS-19 package subset; H4.1 RS-04/RS-07/RS-20 plus RS-24 exploration branches, RS-25 compaction, and RS-26 artifact retrieval, H4.2 structural/impact and adapters, H4.3 semantic treatment only after ablation; H5 RS-02/RS-04/RS-12/RS-15 plus RS-27 failure feedback and RS-28 advisor experiment; H6 RS-08/RS-10/RS-13/RS-20 plus RS-21/RS-22 and RS-28 independence; H7 RS-18 plus RS-24/RS-25 contribution controls; H8 RS-14/RS-16/RS-19 plus RS-29 controlled races and all adopted ablations; H9 RS-06/RS-09/RS-17/RS-19/RS-20 plus RS-30 visual evidence/team UX. Every package type repeats RS-19 at its owning milestone.

A spike may run before its target milestone to resolve architecture uncertainty. It does not satisfy the milestone by itself, accept an ADR, authorize implementation, or weaken the gate. Matrix `Adopt`/`Adapt` entries are candidate target dispositions only; without a passing preregistered experiment and ratifying review, their admission state remains `Candidate` and they are excluded from implementation Task Packets.

## Gate governance

Every exit review includes product, Core, quality, and security authority. A visual demo, model narrative, passing happy path, or Draft PR status is insufficient. Evidence must include normal, denied, degraded, recovery, and adversarial behavior. Every H1–H9 exit packet includes a versioned `EvaluationRun`, exact provenance manifest, declared deviations, and milestone baseline/regression comparison. Material scope or authority changes require a revised gate contract and a new Proposed ADR where applicable.

See also: [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), [21_Project_Backlog.md](21_Project_Backlog.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), [28_Release_Strategy.md](28_Release_Strategy.md), and [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md).
