# 19 — Implementation Roadmap

## Roadmap decision

Build and prove governed runtime truth before product breadth. WePLD’s greatest early risk is a convincing interface over an unreliable authority, effect, or evidence system. Each roadmap gate closes only with measured evidence, accepted architecture decisions, and explicit authorization for the next increment.

Roadmap increments are named **Baseline Gate** and **H1–H9** to avoid confusing them with runtime delivery phases. A mission’s `PhasePlan` remains its primary tailored delivery unit; it is not one of these program milestones.

## Repository and Draft PR #1 boundary

`main` remains canonical. Draft PR #1 is an open Draft, unmerged, unratified candidate **Build Feature baseline** and a possible prerequisite; it is reference material for compatibility assessment, not an approved architecture, merge authorization, or branch on which to implement this plan. This documentation package neither approves nor rejects the PR and must not expand it.

No Hermes Intelligence implementation starts until:

1. the Baseline Gate’s applicable prerequisite decision is recorded;
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

**Exit:** authorized reviewers record whether a baseline is accepted, returned, deferred, or rejected and which H1 entry conditions are met. The planning package itself cannot close this gate.

## H1 — Governed Specification Workflow

**Purpose:** turn desired outcome into an approved, immutable, structured Engineering Specification and Outcome Contract.

**Proof:** Describe/Clarify/Review works through Core; requirements, exclusions, assumptions, risks, verification bindings, and evidence requirements are versioned; approval is durable; WHAT changes create Specification Change Requests; Markdown is projection only.

**Dependency:** Baseline Gate decision and accepted [ADR-0015](adr/ADR-0015-governed-specification-contract.md).

## H2 — Brain Planning and Delivery Control

**Purpose:** convert the approved outcome into a validated, traceable Delivery Plan, tailored Phase Plans, Task Packets, Kanban flow, WIP, budgets, and controlled HOW changes.

**Proof:** the Brain Agent proposes and cannot self-approve; requirements trace to phases/tasks/evidence; Core validates/records approval and transitions; Hermes supervises only approved work; Plan Change Requests preserve higher truth.

**Dependencies:** H1; accepted [ADR-0016](adr/ADR-0016-brain-plan-builder-execution-separation.md) and [ADR-0017](adr/ADR-0017-phase-kanban-controlled-change.md).

## H3 — Hermes Skill Kernel and Hook Runtime

**Purpose:** establish Hermes as a governed engineering runtime rather than a thin model or worker wrapper.

**Proof:** signed versioned skills execute typed procedures; routing uses declared context/tools/capabilities/budget/evidence; typed hooks are bounded and cannot bypass policy; all proposed effects re-enter Core’s Effect Firewall.

**Dependencies:** H2; accepted [ADR-0018](adr/ADR-0018-hermes-skill-runtime-hook-bus.md).

## H4 — Context, LSP, and Hybrid Retrieval

**Purpose:** compile minimal, cited, reproducible task context from authoritative and derived sources.

**Proof:** collect/filter/rank/deduplicate/compress/label/fit/validate is reproducible; initial language-neutral LSP broker adapters provide symbols, references, diagnostics, call/impact and test mapping; exact/policy/specification sources outrank semantic retrieval; trust, freshness, scope, selection reason and cost accompany every item.

**Dependencies:** H3; accepted [ADR-0019](adr/ADR-0019-context-compiler-lsp-hybrid-retrieval.md). Universal language support is a non-goal.

## H5 — Controlled Engineering Loops

**Purpose:** let Hermes observe, diagnose, hypothesize, act minimally, verify, update belief, replan/escalate, or stop under fixed authority and budgets.

**Proof:** each iteration records before/after evidence, expected/actual result, confidence delta and next decision; repeated action, no change, oscillation, diagnostic growth, schema failure, invalid plan, uncertainty, budget exhaustion, and required authority stop or escalate safely.

**Dependencies:** H4; accepted [ADR-0022](adr/ADR-0022-controlled-loop-escalation.md).

## H6 — Specialized Subagents and Self-Review

**Purpose:** add bounded expertise and independent review without an uncontrolled swarm or authority leakage.

**Proof:** read-only exploration is bounded in parallel; writable work is isolated/conflict-controlled; each subagent has one objective, context, skills/tools/capabilities, budget, deadline, output/evidence schema; builder output passes deterministic validation, independent reviewer/test/quality and applicable security review before Completion Proposal.

**Dependencies:** H5; accepted [ADR-0021](adr/ADR-0021-bounded-subagents-structured-handoffs.md).

## H7 — Memory Intelligence

**Purpose:** improve later missions with typed, verified learning without weakening governance truth.

**Proof:** Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory are separated; Memory Candidates pass evidence, contradiction, scope, freshness, expiry and supersession judgment; Governance Memory cannot be downgraded to optional advice.

**Dependencies:** H6; accepted [ADR-0020](adr/ADR-0020-typed-memory-memory-judge.md).

## H8 — Model-Independent Outcome Convergence

**Purpose:** prove “Different brains. Same engineering truth.” against fixed outcome contracts and gates.

**Proof:** multiple supported profiles may take different paths but accepted outputs are contract-equivalent across function, public contracts, architecture, security, regression, quality, evidence and residual risk; quality does not fall for weaker profiles; non-converging profiles retry with justified changes, specialize, split, review, replan, switch, clarify, seek decision, or stop honestly.

Controlled harness and component-ablation runs hold mission, commit, specification, outcome contract, policy, tools, environment, budget class, and attempts constant while varying model and intelligence components. Safety, evidence truthfulness, reproducibility, and honest non-convergence are first-class metrics.

**Dependencies:** H7; accepted [ADR-0023](adr/ADR-0023-model-independent-outcome-equivalence.md) and [ADR-0024](adr/ADR-0024-harness-evaluation-provider-certification.md); protocol in [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md) and [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).

## H9 — Product Surfaces

**Purpose:** expose proven operational contracts as a coherent product.

**Proof:** CLI refinement and Studio surfaces for Mission Control, Spec Review, Plan Review, Kanban, Decisions, Risks, Evidence, Change Requests, Completion Review, Timeline, Executive, Architecture, Knowledge, and review-oriented IDE all call the same Core workflow and pass accessibility, degraded-mode, authority, freshness, and evidence-truthfulness tests.

**Dependencies:** H1–H8 gates closed and the applicable architecture/security decisions accepted. UI breadth, a full graphical IDE, general consumer chat, an open marketplace, and cloud-first control plane remain non-goals.

## Cross-cutting rules

- Every increment includes threat-model, failure/recovery, evaluation, migration/rollback, observability, documentation, and evidence work; these are not final hardening.
- A milestone may narrow scope but cannot skip its exit evidence or lower the Outcome Contract for a selected model.
- Agile adaptation occurs through versioned Specification or Plan Change Requests, not silent “scope lock” edits.
- Policy/event/artifact/effect foundations precede autonomy; evidence gates precede completion; local recovery precedes synchronization; extension trust precedes any marketplace; H1–H8 runtime truth precedes H9 breadth.
- Scale-out, enterprise collaboration, autonomous production deployment, and broad language/model support require later explicit roadmap authorization.

## Initial team shape

The early program needs an accountable product/governance owner, Core/runtime engineer, Hermes/intelligence engineer, security engineer, quality/evaluation engineer, and UX researcher/designer, with language/tooling specialists as needed. Architecture, security, evaluation, and acceptance authorities must be named separately from the Brain Agent, Hermes, and builders.

See [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), [21_Project_Backlog.md](21_Project_Backlog.md), [23_Technology_Evaluation.md](23_Technology_Evaluation.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).
