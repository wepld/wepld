# WePLD Architecture & Master Plan

**Status:** proposed governed-delivery and Hermes Intelligence architecture; architecture review required before implementation authorization.

WePLD is the Operating System for Autonomous Engineering. It provides a native engineering method in which the user states an outcome, the Brain Agent proposes a governed plan, Hermes operates the bounded engineering organization, builder models execute Task Packets, Core governs every transition and effect, and evidence determines acceptance.

The strategic promise is **“Different brains. Same engineering truth.”** Supported profiles may take different implementation paths, but accepted outputs must satisfy the same approved specification, outcome contract, policy, architecture, security, quality, regression, and evidence gates.

## Reading order

1. Start with [30 — Architecture Summary](docs/30_ARCHITECTURE_SUMMARY.md).
2. Read [31 — Governed Specification Workflow](docs/31_Governed_Specification_Workflow.md) for the authority hierarchy, native user workflow, domain contracts, Phase/Kanban model, and traceability.
3. Read [32 — Hermes Engineering Intelligence Runtime](docs/32_Hermes_Engineering_Intelligence_Runtime.md) for the Brain Agent, skills, routing, context, LSP/RAG, hooks, loops, subagents, memory, review, and Effect Firewall.
4. Read [33 — Model-Independent Outcome Convergence](docs/33_Model_Independent_Outcome_Convergence.md) and [34 — Harness Evaluation Protocol](docs/34_Harness_Evaluation_Protocol.md) for the invariant quality bar and evidence needed to support model/profile claims.
5. Read [35 — Reference Systems and Competitive Architecture](docs/35_Reference_Systems_and_Competitive_Architecture.md) for the official-source study, exact source/license snapshots, adoption matrix, and controlled admission experiments behind reference-informed roadmap ideas.
6. Read [19 — Implementation Roadmap](docs/19_Implementation_Roadmap.md), [22 — Milestones](docs/22_Milestones.md), and the [Proposed ADR package](docs/adr/README.md) before considering delivery authorization.
7. Treat [14 — Security](docs/14_Security_Architecture.md), [16 — Data](docs/16_Data_Model.md), [17 — Events](docs/17_Event_System.md), and [18 — APIs](docs/18_API_Architecture.md) as cross-cutting boundary contracts.

## Scope and branch boundary

This branch contains planning and documentation only. The package does not implement Hermes Intelligence, change Rust source, push a branch, open a pull request, merge code, or authorize a release.

[Draft PR #1](https://github.com/wepld/wepld/pull/1) is an open, unmerged candidate Build Feature baseline. Its staged approvals, Core ledger, artifact/worktree boundaries, specification seed, proposal-ref acceptance, and narrow Engineering Memory are reference material. Its branch-local claims and ADRs are not canonical. This package neither ratifies nor authorizes its merge, and no Hermes Intelligence implementation begins until applicable Proposed ADRs are accepted and the preceding milestone gate closes with evidence.

## Source-of-truth rules

- These documents are the proposed product and architecture source of truth pending review; accepted ADRs govern any later supersession.
- In the product, structured, versioned Core records—not Markdown, Git branches, model output, or UI state—are authoritative for specifications, outcome contracts, plans, approvals, capabilities, effects, budgets, evidence, completion, and recovery.
- Markdown, diagrams, exports, indexes, and Git references are reviewable projections or content artifacts with explicit provenance.
- Authority is monotonic: governance policy → approved Engineering Specification → Outcome Contract → approved Delivery Plan → approved Phase Plan → Task Packet → Tool Action. No lower layer may silently redefine a higher one.
- The Brain Agent proposes; it does not approve. Hermes supervises; it does not own truth. Builders/subagents produce artifacts and evidence; they do not accept missions. Tool boundaries perform only Core-authorized effects.
- Approved meaning changes only through typed specification or plan Change Requests and new immutable versions.

## Compatibility note

The canonical `origin/main` used for this planning worktree is documentation-only. Draft PR #1 remains separate. If that candidate baseline is accepted later, its final behavior and contracts must be reconciled with this package rather than treated as an implicit implementation of H1–H9.


