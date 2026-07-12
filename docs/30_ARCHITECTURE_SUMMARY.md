# 30 — Architecture Summary

## Executive decision

Build WePLD as a **desktop-first, local-first modular control plane for autonomous engineering work**. A Studio desktop client connects to a long-lived local Core Daemon. The daemon is authoritative for mission/task state, policy, event history, worker leases, artifacts, knowledge provenance, and user-facing decision flow. The product begins with one user and one local Git project; cloud collaboration and marketplace scale are future capabilities, not V1 assumptions.

## Core architecture in one view

~~~mermaid
flowchart LR
  H["Human executive"] --> S["Studio / Messenger"]
  S --> C["Local Core Daemon"]
  C --> O["Orchestration\nmission, plan, task, lease"]
  C --> P["Policy & Security\ncapabilities, approvals, risk"]
  C --> W["Worker System\nrole + skills + scoped tools"]
  W --> B["Brain Gateway\nreplaceable reasoning providers"]
  W --> T["Tool/Worktree Executor\nisolated effects"]
  C --> K["Knowledge & Evidence\nartifacts, claims, decisions"]
  C --> E["Event Ledger / Projections\nTimeline, Mission Control"]
  C --> I["Registry / Integration Gateway"]
~~~

The essential separation is: **brains reason; workers execute; the Core orchestrates; policy authorizes; tools enforce; evidence verifies; Messenger communicates.** No component is allowed to collapse those roles.

## Key decisions and rationale

| Decision | Why |
| --- | --- |
| Modular monolith/local daemon before microservices | delivers privacy, reliability, speed, and a lower operational burden while preserving clean ports for future split |
| Tauri-style desktop shell + React/TypeScript Studio; Rust Core candidate | aligns a cross-platform UI with a resource-aware local control plane; final adoption awaits spikes |
| SQLite event/projection store + content-addressed artifacts; Git for source | durable local workflow/audit without replacing source history or requiring a server |
| Versioned local RPC and schema contracts | preserves replaceability and prevents UI/adapters from coupling to storage |
| Worker = role policy + brain + skills + scoped tools + task lease | enables specialized, safe, reproducible execution independent of provider |
| Policy decision/enforcement at every boundary | prevents model, worker, plugin, or channel from gaining ambient authority |
| Evidence-driven loop gates and explicit stop conditions | prevents optimistic one-pass coding and runaway retries |
| Typed knowledge graph over artifacts, not chat history | captures durable learning with provenance, classification, freshness, and deletion semantics |
| Signed/versioned extension lifecycle | makes brains, workers, skills, plugins, MCP, integrations, and toolchains installable without creating a supply-chain bypass |

## V1 scope

One user operates one local Git repository through a planner → builder → QA/reviewer loop. The system has a local Core, isolated worktrees, a Hermes-compatible worker adapter, a local-capable and optional hosted brain profile, policy-controlled tool effects, build/test/review evidence, Mission Control, Timeline, a Studio Messenger/decision queue, and Manual/Limited Approval modes. Full autonomy arrives only after recovery, gate, budget, and safety tests pass.

V1 explicitly excludes full IDE parity, multi-user sync, remote worker fleets, arbitrary third-party packages, open marketplace distribution, all messaging channels, and autonomous production deployment.

## Requirement traceability

| Brief requirement | Primary documents | Validation evidence |
| --- | --- | --- |
| Engineering studio / executive human role | 01, 11, 12, 13, 30 | workspace usability and mission E2E fixtures |
| Five systems | 03, 04, 30 | context boundary and contract tests |
| Replaceable brains | 06, 18, 23 | provider adapter conformance/evaluations |
| Specialized workers / no direct user contact | 05, 07, 17 | lease/channel deny-path tests |
| Sole Messenger / channels | 07, 18, 12 | authenticated inbound/outbound adapter tests |
| Autonomous modes | 02, 07, 10, 14, 20 | action-risk policy fixtures |
| Versioned skills / marketplace / improvement | 09, 15, 26, 28 | package validation/revocation tests |
| Iterative engineering loops / thresholds | 10, 26, 27 | gate/stop/retry scenarios |
| Searchable knowledge graph | 08, 16, 17 | provenance/retrieval/retention fixtures |
| Mission Control | 13, 11, 12, 17 | projection freshness and explainability tests |
| IDE, Executive, Architecture, Timeline workspaces | 11, 12 | accessibility/usability evidence |
| Security division | 14, 20, 26, 28 | threat model, scans, incident drills |
| Plugin system | 15, 18, 23, 28 | install/permission/rollback/revoke tests |
| Modular, local, observable, extensible engineering standards | 02, 03, 04, 23, 25 | architecture review and conformance suite |
| Data/event/API foundations | 16, 17, 18 | schema/contract/replay tests |
| Roadmap, backlog, milestones, risk, tech, releases | 19–29 | M0 review and milestone evidence |

## Delivery sequence

1. **M0:** review/approve this architecture, action taxonomy, threat model, contracts, fixtures, and V1 scope.
2. **M1/M2:** prove durable local Core state and a single controlled isolated task effect.
3. **M3:** prove a full bounded mission loop with a reviewable change and evidence.
4. **M4/M5:** add bounded parallelism, recovery, quality/security gates, budgets, Mission Control, knowledge, and governed autonomy.
5. **M6/M7:** expand workspaces, package lifecycle, selected integrations, release reliability, then enterprise/remote capability.

## Open strategic decisions for the human executive

These should be resolved at M0 before implementation begins:

1. Which operating systems and minimum hardware tiers are supported in the first release, given sandbox parity?
2. May any hosted brain process Confidential project context? If yes, under which providers, residency, retention, and consent terms?
3. What default autonomy envelope and hard approvals should V1 use for code writes, dependency installation, network, merges, and release proposals?
4. Which customer segment and project types define the first evaluation fixtures and quality bar?
5. What budget, cost attribution, and local resource limits are acceptable by default?
6. Is a hosted WePLD service a future optional offering, or a core enterprise objective that should shape Phase 0 research?

## Document map

1. [Project Vision](01_Project_Vision.md) · 2. [Product Principles](02_Product_Principles.md) · 3. [System Architecture](03_System_Architecture.md) · 4. [Component Architecture](04_Component_Architecture.md) · 5. [Worker Architecture](05_Worker_Architecture.md) · 6. [Brain Architecture](06_Brain_Architecture.md) · 7. [Messenger Agent](07_Messenger_Agent.md) · 8. [Knowledge System](08_Knowledge_System.md) · 9. [Skills System](09_Skills_System.md) · 10. [Loop Engineering](10_Loop_Engineering.md)

11. [UI/UX Architecture](11_UI_UX_Architecture.md) · 12. [Workspaces](12_Workspaces.md) · 13. [Mission Control](13_Mission_Control.md) · 14. [Security Architecture](14_Security_Architecture.md) · 15. [Plugin System](15_Plugin_System.md) · 16. [Data Model](16_Data_Model.md) · 17. [Event System](17_Event_System.md) · 18. [API Architecture](18_API_Architecture.md) · 19. [Implementation Roadmap](19_Implementation_Roadmap.md) · 20. [Risk Assessment](20_Risk_Assessment.md)

21. [Project Backlog](21_Project_Backlog.md) · 22. [Milestones](22_Milestones.md) · 23. [Technology Evaluation](23_Technology_Evaluation.md) · 24. [Repository Structure](24_Repository_Structure.md) · 25. [Development Guidelines](25_Development_Guidelines.md) · 26. [Testing Strategy](26_Testing_Strategy.md) · 27. [Performance Goals](27_Performance_Goals.md) · 28. [Release Strategy](28_Release_Strategy.md) · 29. [Future Vision](29_Future_Vision.md) · 30. this summary.

## Architecture-gate conclusion

The requested architecture package is complete. It recommends no production implementation until M0 approval. The first implementation effort should be a set of constrained technical spikes and contract/evaluation fixtures that prove the hardest assumptions: cross-platform isolation, durable recovery, provider portability, and evidence-driven mission completion.

