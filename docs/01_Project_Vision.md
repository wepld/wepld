# 01 — Project Vision

## Purpose

WePLD is an **Autonomous Software Engineering Operating System**: a professional engineering studio in which a human executive gives outcomes while a governed engineering organization plans, builds, reviews, tests, documents, and improves software. It is not an AI-enhanced text editor and it is not a collection of chat windows.

The enduring product promise is: **turn a well-scoped mission into evidence-backed engineering progress without requiring the human to coordinate every specialist, tool, or model.** The human remains accountable for strategy, priorities, budgets, and irreversible choices; the organization owns execution within declared policy.

## Users and jobs

| User | Primary job | WePLD outcome |
| --- | --- | --- |
| Founder / product executive | Turn strategy into reliable product delivery | Understand progress, risk, cost, and decisions without managing a swarm |
| Engineering leader | Run several initiatives with consistent standards | See portfolio health, quality evidence, staffing, and dependencies |
| Individual engineer | Delegate bounded engineering work safely | Receive reviewable artifacts, not opaque edits |
| Security / compliance leader | Govern automated change | Enforce policy and retain evidence from intent through release |
| Platform administrator | Provide approved models, tools, and integrations | Control capabilities without changing mission logic |

## What makes the category distinct

An IDE optimizes a developer’s interaction with code. WePLD optimizes the operation of an engineering organization. The IDE is one optional workspace; Mission Control, the decision queue, the knowledge graph, policy enforcement, and durable engineering loops are equally primary.

WePLD differentiates by making five separations explicit:

1. **Brains reason; workers execute.** A model provider cannot silently obtain filesystem or shell access.
2. **The control plane coordinates.** Workers exchange typed work through the Orchestration Engine rather than informal direct chat.
3. **Evidence governs quality.** A worker’s claim of completion is not a release signal; tests, reviews, scans, and artifacts are.
4. **Messenger alone talks to people.** The organization remains active while the user considers a decision.
5. **Knowledge is a cited organizational asset.** It is not merely a prompt transcript or vector search index.

## Product boundaries for the first product

The first usable release serves one person operating one local Git project on one desktop. It orchestrates a planner, a builder, a reviewer, and QA around a small, bounded mission. It captures a durable timeline, produces evidence, protects the primary worktree, and routes genuinely strategic decisions to the Messenger.

The following are intentionally deferred: a full replacement IDE, multi-user real-time collaboration, an open marketplace, unrestricted third-party plugins, autonomous production deployment, all communication channels, and universal model support. A category-defining product needs a reliable control plane before it needs breadth.

## North-star outcomes

| Outcome | Initial measure | Direction |
| --- | --- | --- |
| Mission usefulness | Percentage of completed missions accepted with only review-level human changes | Up |
| Trustworthiness | Material actions with a linked policy decision and evidence trail | 100% |
| Quality | Missions passing required validation gates on first final review | Up |
| Executive attention | Routine execution interruptions per completed mission | Down |
| Local autonomy | Supported core flow available with no cloud control-plane dependency | 100% |
| Portability | Provider/worker replacement without mission-domain changes | 100% of supported adapters |

## Product narrative

An executive creates a mission with scope, success criteria, budget, and autonomy mode. The Orchestration Engine assesses policy, asks a planning worker to turn it into a task graph, and assigns workers with least-privilege capability sets. Workers deliver artifacts, not direct messages. The quality and security functions attach evidence. The Messenger summarizes progress continuously and asks only for a decision packet when a real strategic threshold is crossed. Once the acceptance criteria and gates are satisfied, the organization proposes completion with a replayable explanation of what changed, why, and how it was verified.

## Architectural decisions

- **Desktop-first, local-first:** the first control plane is a long-lived local daemon, not a web app dependent on a hosted service.
- **Modular monolith first:** bounded contexts are separate modules and ports now; independently deployed services are a later scaling decision.
- **Human executive, not human tool operator:** default interaction is goals, decisions, and reports rather than every command approval.

## Success criteria for this planning baseline

This vision is adequately represented only when the architecture supplies a single source of truth for missions, policy, evidence, workers, brains, knowledge, and user communication; preserves local operation; and makes all autonomous effects observable and governable.

See also: [02_Product_Principles.md](02_Product_Principles.md), [03_System_Architecture.md](03_System_Architecture.md), [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), and [30_ARCHITECTURE_SUMMARY.md](30_ARCHITECTURE_SUMMARY.md).

