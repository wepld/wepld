# 01 — Project Vision

## Purpose

WePLD is an **Autonomous Software Engineering Operating System** and a native engineering-delivery method. The user supplies the desired outcome; WePLD supplies governed specification, planning, phased delivery, flow control, verification, change management, and completion semantics. The user should not need to bring an external project-management or specification methodology or coordinate every specialist, tool, or model.

The product thesis is:

> WePLD provides the engineering method. The user provides the desired outcome. The Brain Agent creates the governed delivery proposal. Hermes operates the engineering organization. Builder models execute bounded task packets. WePLD Core governs every transition and effect. Evidence determines whether the outcome is acceptable.

The human remains accountable for intent, priorities, budgets, approvals, and irreversible choices. Core alone owns durable truth, policy, approvals, capability issuance, budget truth, transitions, completion, and recovery. Neither a model nor an agent may silently acquire that authority.

## Strategic promise

**Different brains. Same engineering truth.** Supported brain and builder profiles may choose different implementation paths, consume different budgets, need different context, or escalate at different rates. They work against the same approved specification, outcome contract, architecture and policy constraints, quality gates, and evidence requirements. Accepted results must be contract-equivalent; they need not be byte-identical. A profile that cannot converge must stop safely and report the uncertainty honestly. Model choice never lowers the final acceptance bar.

## Users and jobs

| User | Primary job | WePLD outcome |
| --- | --- | --- |
| Founder / product executive | Turn strategy into reliable product delivery | Approve outcomes and plans; understand progress, risk, cost, and decisions without managing a swarm |
| Engineering leader | Run initiatives with consistent standards | See phase flow, dependencies, quality evidence, and delivery health |
| Individual engineer | Delegate bounded engineering work safely | Receive traceable specifications, reviewable artifacts, and reproducible evidence |
| Security / compliance leader | Govern automated change | Enforce policy and retain evidence from intent through completion |
| Platform administrator | Provide approved models, tools, skills, and integrations | Control capabilities without changing mission semantics |

## What makes the category distinct

An IDE optimizes a developer's interaction with code. WePLD governs an engineering organization. Studio and Mission Control are product surfaces over the same Core workflow; they are not alternate sources of truth.

WePLD makes these separations explicit:

1. **Core governs.** Structured durable records, policy decisions, approvals, budgets, effects, transitions, and recovery belong to Core.
2. **The Brain Agent proposes.** It acts as planner, architect, risk analyst, and replanner, but cannot approve its own work or perform effects.
3. **Hermes supervises delivery.** Hermes is the Engineering Intelligence Runtime, not a brain provider and not governance truth.
4. **Builders and subagents execute bounded work.** They receive TaskPackets, scoped context, skills, tools, capabilities, budgets, and evidence obligations; they return typed artifacts, findings, and proposed actions.
5. **Tool boundaries perform effects.** A model-produced action is only a proposal until Core authorizes it and a mediated tool probes and records the actual result.
6. **Evidence governs acceptance.** A completion claim is not a completion decision.
7. **Messenger is the human-facing agent.** Direct CLI, Studio, MCP, and API commands still enter the same Core workflow.
8. **Knowledge is cited and typed.** Verified lessons may inform later work, while governance records remain authoritative.

## Native user workflow

The product-level workflow is:

1. Describe the desired outcome.
2. Clarify ambiguity, constraints, exclusions, assumptions, risks, and verification needs.
3. Review and approve the versioned EngineeringSpecification and OutcomeContract.
4. Review and approve the traceable DeliveryPlan and initial PhasePlans.
5. Execute phase by phase through Hermes under Core-issued TaskPackets.
6. Observe Kanban flow, WIP, budget, risks, and evidence.
7. Resolve genuine DecisionRequests and controlled specification or plan ChangeRequests.
8. Review a verified CompletionProposal.
9. Accept, return, defer, or cancel through an authorized CompletionDecision.
10. Consolidate only approved, evidence-derived MemoryCandidates.

The structured workflow is defined in [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md). Markdown is a useful review and export projection, not the sole operational source of truth.

## Product boundaries for the first product

The first usable release serves one person operating one local Git project on one desktop. It proves governed specification, planning, one phase at a time, bounded Kanban flow, a planner, a builder, an independent reviewer, QA, durable evidence, isolated worktrees, and safe escalation.

Deferred scope includes a full replacement IDE, multi-user real-time collaboration, an open marketplace, unrestricted third-party plugins, autonomous production deployment, a cloud-first control plane, uncontrolled agent swarms, universal language/model support, and byte-identical output across models. Product-surface breadth follows stable runtime contracts.

Draft PR #1 is an unmerged candidate prerequisite for the Build Feature baseline. This architecture package neither ratifies nor authorizes its merge. Hermes Intelligence implementation begins only after the applicable Proposed ADRs are accepted and the preceding milestone gate is explicitly closed.

## North-star outcomes

| Outcome | Initial measure | Direction |
| --- | --- | --- |
| Outcome usefulness | Verified missions accepted with only review-level human changes | Up |
| Contract equivalence | Supported profiles satisfying the same OutcomeContract and gates | Up |
| Trustworthiness | Material actions with linked policy decision, durable intent, and evidence | 100% |
| Evidence completeness | Required evidence bindings satisfied at completion review | 100% |
| Non-convergence honesty | Failed convergence attempts stopped and escalated without false completion | 100% |
| Executive attention | Routine execution interruptions per accepted mission | Down |
| Local autonomy | Supported core flow available without a hosted WePLD control plane | 100% |
| Portability | Provider, builder, and worker replacement without mission-domain changes | 100% of supported adapters |

## Architectural decisions

- **Desktop-first, local-first:** the initial control plane is a long-lived local daemon.
- **Modular monolith first:** bounded contexts use explicit ports and events before any service split.
- **Phase as the primary delivery unit:** the Brain Agent may tailor the phase graph inside policy; Core enforces entry, exit, WIP, budget, and evidence gates.
- **Human executive, not human tool operator:** normal interaction concerns outcomes, approvals, decisions, risks, change requests, and reports.
- **Structured contracts before surfaces:** runtime truth precedes broad UI, channel, or integration expansion.

## Success criteria for this planning baseline

The architecture is adequate only when it supplies a single Core-governed source of operational truth; preserves the authority hierarchy; supports a native specification-to-completion workflow; makes autonomous effects observable and governable; and can demonstrate model-independent acceptance through reproducible evaluation.

See also: [02_Product_Principles.md](02_Product_Principles.md), [03_System_Architecture.md](03_System_Architecture.md), [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [30_ARCHITECTURE_SUMMARY.md](30_ARCHITECTURE_SUMMARY.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), and [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).
