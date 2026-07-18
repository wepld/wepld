# 19 — Implementation Roadmap

## Roadmap decision

Build and prove governed runtime truth before product breadth. WePLD’s greatest early risk is a convincing interface over an unreliable authority, effect, or evidence system. Each roadmap gate closes only with measured evidence, accepted architecture decisions, and explicit authorization for the next increment.

Roadmap increments are named **Baseline Gate** and **H1–H9** to avoid confusing them with runtime delivery phases. A mission’s `PhasePlan` remains its primary tailored delivery unit; it is not one of these program milestones.

## Repository and Draft PR #1 boundary

`main` remains canonical. Draft PR #1 is an open Draft, unmerged, unratified candidate **Build Feature baseline** and a possible prerequisite; it is reference material for compatibility assessment, not an approved architecture, merge authorization, or branch on which to implement this plan. This documentation package neither approves nor rejects the PR and must not expand it.

No Hermes Intelligence implementation starts until:

1. the Baseline Gate is **Resolved**, with either the required prerequisite contracts accepted from the candidate baseline or an approved replacement-foundation plan covering every missing prerequisite;
2. the applicable Proposed ADRs have been reviewed and moved to an accepted status by authorized architecture governance; and
3. the preceding milestone gate is closed with its required evidence and explicit next-increment authorization.

A successful demo, existing draft code, or a planning document marked complete satisfies none of those conditions by itself.

## Historical M0–M7 lineage

The original M0–M7 plan is preserved as planning lineage, not silently deleted and not represented as completed work.

| Existing milestone | Intent retained | Reconciled destination |
| --- | --- | --- |
| M0 — Architecture Gate | approve contracts before implementation | Baseline Gate plus H1/H2 architecture approvals |
| M1 — Durable Local Core | local authority, recovery, commands/events | prerequisite baseline, then extended through H1/H2 |
| M2 — Safe Task Effect | isolated capability-mediated effect | Baseline Gate, H3 runtime contracts, H5 recovery loops |
| M3 — First Mission Loop | bounded planner/builder/reviewer outcome | H1–H6 governed vertical slice |
| M4 — Governed Autonomy | leases, budgets, gates, stop behavior | H5/H6 and H8 convergence proof |
| M5 — Organizational Memory | sourced reusable learning | H7 typed memory and Memory Judge |
| M6 — Studio Beta | operable product surfaces | H9 Product Surfaces |
| M7 — Ecosystem/Enterprise Readiness | trusted extensions and remote growth | extension foundations in H3/H4; enterprise/scale only after H9 authorization |

The detailed milestone cards, entry/exit scenarios, required ADRs, and next-gate authorizations live in [22_Milestones.md](22_Milestones.md). This document holds only the high-level dependency sequence.

## Baseline Gate — Candidate Build Feature review

**Purpose:** finish and independently review the current candidate Build Feature baseline without treating Draft PR #1 as canonical or pre-authorized.

**Proof:** repository/branch provenance is explicit; candidate contracts, tests, sandbox/effect assumptions, and platform limits are compared to `main` and the accepted architecture; gaps are classified as prerequisite, temporary V0 limitation, deferred extension, or rejection concern; review produces a decision packet rather than a merge.

**Non-goals:** approving, merging, expanding, or building Hermes Intelligence in PR #1.

**Exit:** authorized reviewers record a final candidate disposition—accepted, returned, deferred, or rejected—and mark the Baseline Gate **Resolved** only after one H1 prerequisite path is explicit: required contracts accepted from the candidate, or an approved replacement-foundation plan covering all gaps. A non-accept disposition cannot permanently block H1. The planning package itself cannot close this gate.

## Early Evaluation Spine — prerequisite measurement contract

Before H1 or H2 implementation, Proposed [ADR-0024](adr/ADR-0024-evaluation-spine-run-provenance.md) must be accepted and the minimum evaluation spine must be operational: versioned `EvaluationCase`, `EvaluationRun`, `TreatmentArm`, `RunManifest`, `MetricObservation`, `ProtocolDeviation`, and `EvaluationResult` contracts with exact fixture, repository, contract, configuration, tool, environment, and seed provenance. Before H1 authorization, the pre-H1 foundation Baseline `EvaluationRun` defined by document 34 must reach an honest terminal state (`Completed`, `Failed`, or `Aborted`), be assessed with all required observations validated and deviations dispositioned, and have a finalized `EvaluationResult`. Each H milestone records a comparable baseline and regression result; H8 consumes this history rather than introducing instrumentation.

## H1 — Governed Specification Workflow

**Purpose:** turn desired outcome into an approved, immutable, structured Engineering Specification and Outcome Contract.

**Proof:** Describe/Clarify/Review works through Core; requirements, exclusions, assumptions, risks, verification bindings, and evidence requirements are versioned; approval is durable; WHAT changes create Specification Change Requests; Markdown is projection only; an evaluation-compatible H1 run compares the milestone result with its preregistered baseline.

**Dependency:** Baseline Gate **Resolved** through one approved prerequisite path, the terminal/assessed pre-H1 foundation Baseline `EvaluationRun` with finalized `EvaluationResult`, plus accepted [ADR-0015](adr/ADR-0015-governed-specification-contract.md) and early-spine [ADR-0024](adr/ADR-0024-evaluation-spine-run-provenance.md).

## H2 — Brain Planning and Delivery Control

**Purpose:** qualify a Brain-produced `PlanProposal` before it becomes an approved, traceable `DeliveryPlan`, then govern tailored Phase Plans, Task Packets, Kanban flow, WIP, budgets, and controlled HOW changes.

**Proof:** Brain → `PlanProposal` → deterministic Plan Compiler/normalization → Plan Candidate → structural validation → initial architecture/risk/evidence `PlanAssessment` → separate independent architecture, quality, and security review records when policy requires → finalized `Ready` assessment → authenticated `PlanDecision` → approved `DeliveryPlan`. Low-risk plans may reach `Ready` from deterministic assessment plus authorized user review when policy requires no independent-review set; medium/high-risk plans require appropriately independent review. The producing Brain invocation cannot approve or provide the only acceptance-critical review, model voting is not authority, and Core records the decision. Alternative-plan generation is triggered by risk or uncertainty, not required for every mission. Requirements trace to phases/tasks/evidence; Hermes supervises only approved work; Plan Change Requests preserve higher truth.

The deterministic delivery compiler also projects approved `DeliveryPlan`, `PhasePlan`, and `TaskPacket` versions into a candidate `SOPGraph`: typed `RoleNode`, `ActionContract`, authorized `InputSubscription`, `OutputContract`, dependency edges, evidence obligations, stop/escalation rules, and exact parent versions. Core validates and publishes only assignment-relevant artifacts/events; a role cannot subscribe itself to new authority-bearing input, and peer broadcast/free agent chat is rejected.

**Dependencies:** H1; accepted [ADR-0016](adr/ADR-0016-brain-plan-builder-execution-separation.md) and [ADR-0017](adr/ADR-0017-phase-kanban-controlled-change.md).

## H3 — Hermes Skill Kernel and Hook Runtime

### H3.1 — Built-in Skill Kernel

**Purpose:** establish Hermes as a governed engineering runtime rather than a thin model or worker wrapper.

**Proof:** repository-owned built-in skills use static, versioned manifests and exact content hashes; procedures have typed inputs/outputs, declared context/tool/capability needs, and verification/evidence contracts; built-in lifecycle hooks and the internal control/event contract remain typed, bounded, and Core-mediated. External package installation, public registries, marketplaces, third-party executable hooks, and generalized signing infrastructure are excluded from the H3.1 exit.

For each Brain, builder, or subagent invocation, Core compiles a capability-projected tool schema from policy, Task Packet, issued capabilities, role, sandbox tier, data classification, phase, and budget. Denied tools are omitted from the model-visible catalog where the provider permits, context and MCP-catalog token budgets are enforced, and the Effect Firewall remains the final boundary. Tool results are schema-validated and budgeted before context insertion; oversized output becomes a bounded summary/head-tail preview plus a classified, permissioned content-addressed full artifact.

**Dependencies:** H2; accepted [ADR-0018](adr/ADR-0018-hermes-skill-runtime-hook-bus.md).

### H3.2 — Governed Skill Packaging

**Purpose:** add package staging, signature/provenance verification, atomic activation, rollback, and revocation only after H3.1 demonstrates measured benefit and preserved governance.

**Gate:** preregistered H3.1 evaluation shows useful improvement without policy, evidence, recovery, cost, or reliability regression; architecture and security authorities explicitly authorize the packaging threat surface. H3.2 is not required to close the minimum H3 milestone.

## H4 — Context, LSP, and Hybrid Retrieval

### H4.1 — Reproducible Context and Initial LSP

**Purpose:** compile minimal, cited, reproducible task context from authoritative and derived sources.

**Proof:** authority-first collect/filter/rank/deduplicate/compress/label/fit/validate is reproducible; exact file/path/identifier retrieval, lexical search, Git evidence, and one initial rust-analyzer adapter provide the minimum structural signals; every Context Pack manifest records exact inputs, hashes, ranking/configuration, trust, freshness, scope, selection reason, and cost.

Read-only `MissionExplorationBranch` records may investigate alternatives without mutating the main execution path. Each branch records parent, objective, Context Pack hash, permissions, budget, findings, evidence, status, and accepted/rejected contribution; transcripts and summaries are non-authoritative. A `CompactionRecord` preserves independently rehydratable policy, approved specification/Outcome Contract, current approved plan/phase/task versions, unresolved decisions, risks, evidence requirements, and stop conditions, while recording omissions and the full source-context hash. No summary supersedes an authoritative artifact.

**Dependencies:** H3.1; accepted [ADR-0019](adr/ADR-0019-context-compiler-lsp-hybrid-retrieval.md). H3.2 packaging is not a prerequisite. Universal language support is a non-goal.

### H4.2 — Structural and Impact Intelligence

**Purpose:** add AST/tree-sitter retrieval, affected-file and affected-test mapping, and additional LSP adapters only after adapter conformance, reproducibility, security, and freshness evidence passes.

### H4.3 — Governed Semantic Retrieval

**Purpose:** add semantic retrieval and embeddings only when a controlled ablation demonstrates practical benefit without unacceptable authority, security, freshness, cost, or token harm. Semantic retrieval remains off by default until that evidence and an explicit authorization exist.

## H5 — Controlled Engineering Loops

**Purpose:** let Hermes observe, diagnose, hypothesize, act minimally, verify, update belief, replan/escalate, or stop under fixed authority and budgets.

**Proof:** each iteration records before/after evidence, expected/actual result, confidence delta and next decision; repeated action, no change, oscillation, diagnostic growth, schema failure, invalid plan, uncertainty, budget exhaustion, and required authority stop or escalate safely.

Sandbox/capability denials return a typed result naming boundary, reason, attempted effect, retryability, safe alternatives, required authority, recovery state, and evidence. Hermes may not repeat an identical denied action without a changed hypothesis, capability, plan, or authority. A contextual risk advisor may be evaluated as an advisory classifier that recommends allow-under-existing-policy, narrow, block, or request-authority, but it cannot grant capability, override deterministic denial, approve protected effects, change policy, or become the sole security boundary.

**Dependencies:** H4.1 closed with accepted evidence; H4.2 and H4.3 are optional and are not H5 prerequisites; accepted [ADR-0022](adr/ADR-0022-controlled-loop-escalation.md).

## H6 — Specialized Subagents and Self-Review

**Purpose:** add bounded expertise and independent review without an uncontrolled swarm or authority leakage.

**Proof:** read-only exploration is bounded in parallel; writable work is isolated/conflict-controlled; each subagent has one objective, context, skills/tools/capabilities, budget, deadline, output/evidence schema; builder output passes deterministic validation, independent reviewer/test/quality and applicable security review before Completion Proposal.

Builders, reviewers, testers, security agents, and explorers receive only their authorized `SOPGraph` subscriptions and typed upstream artifacts. Uncontrolled shared environments, peer broadcast, and free agent-to-agent chat are prohibited; the contextual risk advisor remains an experiment under independent security evaluation.

**Dependencies:** H5; accepted [ADR-0021](adr/ADR-0021-bounded-subagents-structured-handoffs.md).

## H7 — Memory Intelligence

**Purpose:** improve later missions with typed, verified learning without weakening governance truth.

**Proof:** Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory are separated; Memory Candidates pass evidence, contradiction, scope, freshness, expiry and supersession judgment; Governance Memory cannot be downgraded to optional advice.

Accepted findings from a read-only Mission Exploration Branch may become evidence-linked Memory Candidates; rejected branches remain durable negative evidence. Neither branch transcript nor compacted summary becomes Core truth. H7 repeats the RS-25 summary-to-memory negative regression so compaction cannot bypass the Memory Judge.

**Dependencies:** H6; accepted [ADR-0020](adr/ADR-0020-typed-memory-memory-judge.md).

## H8 — Model-Independent Outcome Convergence and Profile Certification

**Purpose:** prove “Different brains. Same engineering truth.” against fixed outcome contracts and gates.

**Proof:** multiple supported profiles may take different paths but accepted outputs are contract-equivalent across function, public contracts, architecture, security, regression, quality, evidence and residual risk; quality does not fall for weaker profiles; non-converging profiles retry with justified changes, specialize, split, review, replan, switch, clarify, seek decision, or stop honestly.

H8 consumes the `EvaluationRun` history accumulated from the pre-H1 foundation **Baseline run** defined in document 34 and from H1 through H7, then adds controlled cross-model comparisons, component ablations, repeated/randomized experiments, and provider/profile certification. Runs hold mission, commit, specification, outcome contract, policy, tools, environment, budget class, and attempts constant while varying the declared treatment. Safety, evidence truthfulness, reproducibility, and honest non-convergence are first-class metrics.

A risk-triggered controlled multi-route race may compare multiple plans or builders only with specification, Outcome Contract, repository commit, policy, tools, environment, budget class, and scoring gates fixed. Every candidate must independently pass the unchanged contract; model vote, visual preference, or relative rank is never acceptance authority.

**Dependencies:** H7; accumulated early-spine evidence under accepted [ADR-0024](adr/ADR-0024-evaluation-spine-run-provenance.md); accepted [ADR-0023](adr/ADR-0023-model-independent-outcome-equivalence.md) and [ADR-0025](adr/ADR-0025-model-profile-certification.md); protocol in [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md) and [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).

## H9 — Product Surfaces

**Purpose:** expose proven operational contracts as a coherent product.

**Proof:** CLI refinement and Studio surfaces for Mission Control, Spec Review, Plan Review, Kanban, Decisions, Risks, Evidence, Change Requests, Completion Review, Timeline, Executive, Architecture, Knowledge, and review-oriented IDE all call the same Core workflow and pass accessibility, degraded-mode, authority, freshness, and evidence-truthfulness tests.

The future Execution/Team projection may show roles, current work, blockers, decisions, budgets, evidence, and results. UI-task Evidence Bundles may include screenshots, video, accessibility trees, browser-console output, network traces, and deterministic interaction scripts; appearance alone is never acceptance. SEO, ads, growth automation, automatic production deployment, and no-code hosting remain outside H1–H9.

**Dependencies:** H1–H8 gates closed and the applicable architecture/security decisions accepted. UI breadth, a full graphical IDE, general consumer chat, an open marketplace, and cloud-first control plane remain non-goals.

## Reference-informed experiment overlay

[35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md) records the official-source observations, exact matrix dispositions and controlled experiments behind every reference-informed candidate. These are roadmap admission tests, not product commitments. A candidate enters implementation scope only after its named experiment meets preregistered benefit and safety thresholds, architecture/product/security reviewers accept the exact scope, the applicable Proposed ADR is accepted, and the milestone entry gate is authorized.

| Gate | Reference experiments that may supply required evidence |
| --- | --- |
| Baseline / early spine | RS-00 provenance/license intake; RS-11 mandatory policy and containment suite; establish versioned evaluation cases, manifests, observations, deviations, results, and regression comparison before H1 |
| H1 | RS-01 typed Delivery Protocol and Markdown-authority negative cases |
| H2 | RS-01 plan/task compilation; RS-02 cross-artifact analysis/converge; RS-15 planner/builder separation; RS-20 Delivery Protocol/workflow-import corpus; RS-21 typed SOP compilation; RS-22 authorized role-subscription design |
| H3.1 | RS-03 minimal built-in Agent/Skill kernel; internal Hermes lifecycle/control-event subset of RS-05/RS-20; RS-11 mediation; RS-21/RS-22 execution projection; RS-23 capability-projected tool schemas; RS-26 bounded tool-output path; exclude package distribution |
| H3.2 | Only after H3.1 benefit/governance proof: skill/hook package subset of RS-19 with staging, provenance, activation, rollback, and revocation |
| H4.1 | RS-04 reproducible context; RS-07 exact/lexical/Git/rust-analyzer treatment; applicable RS-20 adapter tests; RS-24 read-only Mission Exploration Branches; RS-25 governed compaction; RS-26 tool-output artifact retrieval |
| H4.2 | RS-07 structural/impact treatment and additional adapter conformance; type-specific RS-19 only if packaging is separately authorized |
| H4.3 | RS-07 semantic treatment enabled only after authority/security/freshness/cost/token ablation gates pass |
| H5 | RS-02 bounded converge; RS-04 session recovery; RS-12 crash/effect reconciliation; RS-15 bounded verification; RS-27 typed sandbox/capability failure feedback; RS-28 contextual risk-advisor experiment |
| H6 | RS-08 Core Delivery Control leases plus Tool & Workspace/Worker Host isolation; RS-10 bounded scheduler/budgets; RS-13 subagent topology/review independence; RS-20 workflow fan-out/resume; RS-21/RS-22 role execution/subscriptions; RS-28 advisor independence; RS-19 for packaged worker/subagent types if introduced |
| H7 | RS-18 memory admission, contradiction, expiry and deletion; RS-24 accepted/rejected exploration contribution; RS-25 summary-to-memory non-authority regression; RS-19 for packaged memory adapters if introduced |
| H8 | RS-14 provider/profile portability; RS-16 evaluation triangulation; RS-19 for provider/profile packages; RS-29 controlled multi-route race; rerun every adopted component ablation and security regression |
| H9 | RS-06 ACP adapter under a new Proposed ADR; RS-09 Mission Control Execution Console; RS-17 editor/multibuffer review UX; RS-20 external adapter corpora; RS-30 visual execution evidence/team UX; RS-19 for protocol/UI packages |

Passing a spike does not authorize implementation, and failing one removes, narrows, defers or rejects the idea. Core Delivery Control continues to own admission, WIP and scheduler ports; Hermes only schedules eligible work. The Execution Console is a Mission Control view. ACP and other protocols remain mediated adapters. Remote/cloud handoff stays outside H1–H9.

## Cross-cutting rules

- Every increment includes threat-model, failure/recovery, evaluation, migration/rollback, observability, documentation, and evidence work; these are not final hardening. Every H milestone emits an evaluation-compatible `EvaluationRun`, exact provenance manifest, baseline/regression comparison, and declared deviations.
- A milestone may narrow scope but cannot skip its exit evidence or lower the Outcome Contract for a selected model.
- Agile adaptation occurs through versioned Specification or Plan Change Requests, not silent “scope lock” edits.
- Policy/event/artifact/effect foundations precede autonomy; evidence gates precede completion; local recovery precedes synchronization; extension trust precedes any marketplace; H1–H8 runtime truth precedes H9 breadth.
- Scale-out, enterprise collaboration, autonomous production deployment, and broad language/model support require later explicit roadmap authorization.

## Initial team shape

The early program needs an accountable product/governance owner, Core/runtime engineer, Hermes/intelligence engineer, security engineer, quality/evaluation engineer, and UX researcher/designer, with language/tooling specialists as needed. Architecture, security, evaluation, and acceptance authorities must be named separately from the Brain Agent, Hermes, and builders.

See [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), [21_Project_Backlog.md](21_Project_Backlog.md), [23_Technology_Evaluation.md](23_Technology_Evaluation.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).
