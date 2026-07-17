# v2-00 — Architecture v2 Overview

**Status:** Normative baseline (ADR-0001). Supersedes v1 scope and mechanisms where they conflict; the vision (v1 docs 01, 02, 29) is unchanged and binding.

## Identity — unchanged

WePLD is **the Operating System for Autonomous Engineering**: a local-first control plane where a human executive states missions, a governed engineering organization executes them, and every material outcome carries evidence. It is not an AI IDE, not a Cursor variant, not a coding assistant. The Studio — Mission, Timeline, Decisions — is the product center; the editor is one future workspace among several.

v2 changes *how little machinery it takes to be that*, not *what it is*.

## What v2 changes, in one table

| Axis | v1 | v2 | ADR |
| --- | --- | --- | --- |
| First deliverable | platform vertical slice (daemon + fleet + policy engine + Studio) | **One Governed Mission** MVP | 0008 |
| Worker topology | role-specialized fleet with scheduler/leases | one runtime, role switching per phase; fleet-ready protocol | 0002 |
| Worker protocol | "Hermes-compatible" (undefined) | **WWP** (WePLD-owned); Hermes = reference runtime | 0005 |
| State | event-sourced + CQRS + outbox | transactional state + append-only audit ledger | 0003 |
| Enforcement | per-action capability tokens | **sandbox envelope** per phase + short hard-gate list | 0004 |
| Sandbox | "policy reduces autonomy" (one sentence) | explicit per-OS tiers S0–S3 capping autonomy | 0007 |
| Context construction | unowned | **Context Assembly** subsystem, packs captured for replay | 0006 |
| Autonomy modes | 4 | 2 presets over one mechanism | 0009 |
| Persona | 5 personas | solo professional wedge; enterprise as consequence | 0010 |
| Specification style | principles + acceptance criteria | + concrete contracts (v2-07) and a worked example (v2-08) | — |

## Non-negotiable principles → v2 mechanisms

Every principle from the vision now names the mechanism that makes it true. This table is the spine of v2; each row links to the document that specifies the mechanism.

| # | Principle | v2 mechanism |
| --- | --- | --- |
| 1 | Local-first whenever practical | single local Core process, SQLite + file artifact store, local UI; only brain calls and opt-in telemetry leave the machine (v2-02) |
| 2 | Replaceable Brains | Brain Contract: provider-neutral request/result routed by Core; workers never see providers or credentials (v2-03 §Brain routing, v2-07) |
| 3 | Replaceable Worker Runtime | WWP contract + conformance suite; Hermes is one conformant implementation (v2-03, ADR-0005) |
| 4 | Messenger is the only human channel | one agent identity across all Studio surfaces; workers have no user transport by construction — no WWP message can address a human (v2-10) |
| 5 | Mission-based engineering | Mission Contract: structured brief with scope, criteria, budget, envelope — no free chat entry point (v2-07) |
| 6 | Evidence before completion | gates are Core-run checks producing ledger facts; model prose can never satisfy a gate (v2-02 §Gates) |
| 7 | Human makes strategic decisions | hard-gate list + Decision Contract + interrupt budget (v2-10) |
| 8 | Engineering organization instead of chat | governed phases with role profiles, envelope/context/brain isolation per phase (v2-03) |
| 9 | Studio-first experience | MVP UI is three mission-centric surfaces; no editor surface exists in the MVP at all (v2-01) |
| 10 | Strong observability | every state change writes a hash-chained ledger entry with actor/causation/correlation (v2-06) |
| 11 | Replayability | context packs + brain invocation records + ledger reconstruct exactly what happened and what each model saw (v2-04, v2-08 §Replay) |
| 12 | Knowledge accumulates | typed Decision/Lesson/Finding records with source links; extraction pipelines deferred, records are first-class now (v2-07 §Knowledge) |
| 13 | Skills evolve | versioned, hash-pinned skill packages resolved into context packs; evolution workflow returns with the registry in V2 (v2-07 §Skill) |
| 14 | Vendor independence | all provider access behind the Brain Contract in Core; adapters are leaf modules (v2-03) |
| 15 | Modular architecture | in-process modules behind the same ports that later become process boundaries (v2-02 §Module map) |

## How v2 addresses the gate review

| Finding | Resolution |
| --- | --- |
| C1 platform-before-product | ADR-0008 MVP; sized roadmap v2-09 |
| C2 sandbox + capability contradiction | ADR-0004 envelopes, ADR-0007 tiers, v2-05 |
| C3 persona inversion | ADR-0010; interruption economics v2-10 |
| C4 Hermes undefined | ADR-0005; WWP in v2-03/v2-07 |
| C5 orchestration thesis unvalidated | ADR-0002 single runtime; Phase A experiment E1 (v2-09) with a decision rule |
| H1 no competitive/economic grounding | §Positioning below; Phase A gate deliverable P1 (v2-09) |
| H2 context assembly unowned | ADR-0006; v2-04 |
| H3 mechanism-free specs | v2-02 mechanisms, v2-07 contracts, v2-08 worked example |
| H4 parallel-merge undefined | MVP is sequential-by-design; footprint-disjointness rule for V2 parallelism (v2-09 §Earn-back) |
| H5 escalation economics | v2-10: interrupt budget, packet classes, batching, default-with-undo |
| H6 unfalsifiable plan | v2-09 carries per-epic sizing and a team plan |
| H7 one-voice contradiction + injection path | v2-10: one-agent-identity rule; ledger-rendered claims; provenance chips |

## Positioning (summary; full analysis is Phase A deliverable P1)

What incumbents optimize: the *session* — a developer, an editor, an agent, now. What WePLD optimizes: the *operation* — missions, evidence, decisions, memory, over time. The durable moats are the ones that require architectural commitments incumbents have not made: (a) a local-first control plane where the audit ledger, not the vendor cloud, is the source of truth; (b) evidence-gated completion enforced by the system rather than asserted by the model; (c) provider-neutral governance — the customer's policy survives switching brains. Features incumbents will absorb (and we should not compete on): editor UX, chat ergonomics, raw agent capability. The Phase A positioning document must test these claims against Claude Code, Cursor, Devin, OpenHands, and GitHub's agent surfaces, and state the business-model hypothesis.

## Document map

| Doc | Contents |
| --- | --- |
| [v2-01](01_MVP_One_Governed_Mission.md) | MVP definition, cuts, earn-back seams |
| [v2-02](02_System_Design_Mechanisms.md) | processes, storage, state machine, gates, recovery — how it works |
| [v2-03](03_Worker_Runtime_and_Hermes.md) | WWP, role switching, Hermes, brain routing |
| [v2-04](04_Context_Assembly.md) | context packs, budgeting, compression, redaction, capture |
| [v2-05](05_Sandbox_Strategy.md) | per-OS tiers, enforcement, disclosure |
| [v2-06](06_State_and_Ledger.md) | tables, ledger schema, consistency, ES migration path |
| [v2-07](07_Contracts.md) | the nine contracts with concrete schemas |
| [v2-08](08_Worked_Example.md) | one mission end-to-end, ledger entries included, plus replay |
| [v2-09](09_Roadmap_and_Sizing.md) | phases, experiments, sizing, earn-back triggers |
| [v2-10](10_Decision_Economics_and_Messenger.md) | interrupt budget, packet classes, one-voice rule, injection hardening |
| [v2-11](11_Chronicle_Overview.md) | **Chronicle** — the Engineering Intelligence pillar: overview + capability classification |
| [v2-12](12_Replay_Engine.md) | frames, lenses, sessions, checkpoints, replay performance/consistency |
| [v2-13](13_Engineering_Cinema.md) | the replay experience: player, scrubber, cameras, orientation model |
| [v2-14](14_Engineering_Forensics.md) | causal index, question bench, Root Cause engine |
| [v2-15](15_Mission_Branching.md) | snapshots, forks, decision editing, comparison |
| [v2-16](16_Engineering_Intelligence.md) | insight pipeline, learning loop, the compounding claim |
| [v2-17](17_Chronicle_Contracts_and_API.md) | Chronicle contracts C1–C10 and full API |
| [v2-18](18_Chronicle_Worked_Examples.md) | live replay, PostgreSQL→SQLite decision edit, forensic RCA |

ADRs: [adr/](../adr/) — ADR-0001 through ADR-0014 record every major change with reason, benefits, trade-offs, and migration impact (0011–0014 cover Chronicle: projection-not-recording, fork-never-rewind, Git snapshot refs, causal index).
