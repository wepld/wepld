# WePLD — Architecture & Master Plan

**Identity:** the Operating System for Autonomous Engineering. Not an AI IDE, not a coding assistant — a local-first control plane where a human executive states missions, a governed engineering organization executes them, and every material outcome carries evidence.

**Status:** Architecture **v2.0 — FROZEN** (including Chronicle, docs v2-11…18, ADR-0001…0014). The **implementation program is active**: [docs/impl/](docs/impl/00_Implementation_Program.md) is the executable blueprint (repository layout, interfaces, milestones M0–M8, Sprint 1, testing, risks, solo-founder guide). Implementation-level decisions are recorded as IADRs in [docs/impl/adr/](docs/impl/adr/). Architectural changes require the post-preview review gate; until then, gaps are handled per the gap-note rule in IMPL-00. Claims in this repository are designs to be proven, not achieved capabilities.

## Repository layers

| Layer | Location | Authority |
| --- | --- | --- |
| Vision & principles | [docs/01](docs/01_Project_Vision.md), [02](docs/02_Product_Principles.md), [29](docs/29_Future_Vision.md) | binding, unchanged |
| Architecture v1 (original 30-document package) | [docs/](docs/) | superseded or amended where v2 overlaps — see ADR-0001 |
| **Architecture v2 (normative)** | [docs/v2/](docs/v2/00_Architecture_V2_Overview.md) | governs scope, mechanisms, contracts, roadmap |
| Decision records | [docs/adr/](docs/adr/) | ADR-0001…0010 record every major change with reason, benefits, trade-offs, migration impact |

## Reading order

1. [v2-00 Overview](docs/v2/00_Architecture_V2_Overview.md) — what v2 changes and why; principle→mechanism map.
2. [v2-01 MVP](docs/v2/01_MVP_One_Governed_Mission.md) and [v2-08 Worked Example](docs/v2/08_Worked_Example.md) — what gets built and exactly how a mission runs.
3. [v2-07 Contracts](docs/v2/07_Contracts.md) — the nine normative contracts (Mission, Worker/WWP, Brain, Messenger, Event, Decision, Artifact, Knowledge, Skill).
4. [v2-02](docs/v2/02_System_Design_Mechanisms.md)–[v2-06](docs/v2/06_State_and_Ledger.md), [v2-10](docs/v2/10_Decision_Economics_and_Messenger.md) — subsystem mechanics.
5. [v2-11 Chronicle](docs/v2/11_Chronicle_Overview.md) — the Engineering Intelligence pillar: replay, cinema, forensics, branching, learning (docs v2-11…v2-18, ADR-0011…0014).
6. [v2-09 Roadmap](docs/v2/09_Roadmap_and_Sizing.md) — phases, experiments, sizing, earn-back triggers for every deferred v1 system.
7. v1 documents for the full original vision context.

## Source-of-truth rules

- Where v1 and v2 conflict on scope, mechanism, or sequencing, **v2 wins** (ADR-0001 contains the supersession table). The vision and the fifteen non-negotiable principles do not change without a new ADR.
- Any future change to a boundary, authority, contract, data classification, or security posture requires an ADR in [docs/adr/](docs/adr/).
- No production implementation begins before the Phase A gate exits with evidence (spikes green, contracts frozen, orchestration-thesis experiment decided, positioning reviewed).
- The directory named `WePLD` elsewhere on the Desktop is not part of this project.
