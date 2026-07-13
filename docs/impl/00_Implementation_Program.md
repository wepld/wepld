# IMPL-00 — The WePLD Implementation Program

**Architecture:** v2.0, FROZEN (docs/v2, ADR-0001…0014). This program adds no subsystems, pillars, engines, protocols, or abstractions. Implementation-level choices are recorded as IADRs ([adr/](adr/)); if anything ever *appears* architecturally missing, the required first step is a **gap note** proving it cannot be built with existing mechanisms — before any proposal.

**Operating principle:** record once, verify once, store once, derive forever — applied to engineering itself: one contracts package every module imports, one golden-trace suite every PR must pass, one ledger every feature reads instead of inventing state.

**Execution doctrine:** *tracer bullet, then fatten.* Sprint 1 drives a thin thread through every layer — contracts → ledger → WWP → brain → runtime → CLI — producing a complete (if skeletal) mission in ten working days. Every subsequent milestone deepens one slice of that living system. Nothing is ever built that the current demo cannot exercise.

## Program map

| Doc | Contents |
| --- | --- |
| [IMPL-01](01_Repository_Layout.md) | final repo layout, every package, ownership, dependency graph |
| [IMPL-02](02_Interfaces_and_Boundaries.md) | public/internal interfaces, layering rules, drift prevention |
| [IMPL-03](03_Milestones_and_Timeline.md) | implementation order, milestones M0–M8, DoD, build timeline |
| [IMPL-04](04_Sprint_1.md) | the first sprint, day by day, and the demo ladder |
| [IMPL-05](05_Package_Specs.md) | package-by-package: purpose, APIs, deps, tests, acceptance, future |
| [IMPL-06](06_Testing_Strategy.md) | unit / golden / contract / integration / E2E / conformance |
| [IMPL-07](07_Risk_Register.md) | per-milestone risks and mitigations |
| [IMPL-08](08_Solo_Founder_Guide.md) | execution rhythm, checklists, freeze discipline |

IADRs: [0001 TypeScript end-to-end](adr/IADR-0001-typescript-end-to-end.md) · [0002 fixture-first brain](adr/IADR-0002-fixture-first-brain.md) · [0003 dev-tier milestones](adr/IADR-0003-dev-tier-milestones.md) · [0004 golden-trace conformance](adr/IADR-0004-golden-trace-conformance.md) · [0005 founder-OS-first sandbox](adr/IADR-0005-founder-os-first.md)

## The North Star Demo (defined once, chased continuously)

> Open WePLD (Studio in browser) → create a Mission (structured brief) → mission enters the queue → Hermes executes over WWP → the reasoning provider is invoked → Context Assembly builds and captures the pack → the ledger records everything → gates pass → Messenger reports completion with verified claims → the human accepts → **replay the mission in Cinema.**

This is the exit of **M6** (~week 16 expected). Two precursors keep the thread taut: the **CLI demo** (M0, week 2 — same flow minus browser/replay, timeline printed in the terminal) and the **Browser demo** (M3 — same flow in the Studio, Contact-Sheet timeline). It must feel like WePLD at every stage: brief → evidence → decision → timeline. There is no chat box to fall back on; the identity is structural (v2-01).

## Implementation Readiness Report

**Verdict: READY. Sprint 1 can begin immediately.** Basis:

| Readiness question | Status |
| --- | --- |
| Are contracts concrete enough to code against? | Yes — v2-07/v2-17 give field-level schemas; Sprint 1 transcribes them into zod |
| Is expected behavior specified? | Yes — v2-08/v2-18 traces are normative and become golden tests (IADR-0004) |
| Is the stack chosen? | Yes — IADR-0006: Rust core + Tauri v2 per the charter (supersedes IADR-0001) |
| Is scope bounded for one engineer? | Yes — M0–M8 ladder, ~26–34 solo weeks to design-partner preview (IMPL-03) |
| Are the hard risks scheduled, not hidden? | Yes — sandbox at M4 behind honest DEV tier (IADR-0003); recovery drills at M5; risk register IMPL-07 |
| Any remaining architectural work? | **None required.** The freeze holds |

**Prerequisites before Day 1 (founder actions, ~half a day):** create the GitHub repo + Actions CI; install rustup (stable Rust) and git — Node ≥22 arrives with the Studio at M3; Windows hosts enable WSL2 (IADR-0006); record the founder OS in `DECISIONS.md` (drives IADR-0005); obtain one hosted-provider API key (first used M1; Sprint 1 is cassette-only); select two fixture repositories (suggest: a ~2k-LOC TypeScript CLI and a ~5k-LOC API service, permissively licensed); set a token budget (~$100–300/month from M1, near-zero in CI thanks to IADR-0002).

**Known gaps, accepted consciously:** one OS tier until M8 (IADR-0005); the v2-09 E1 topology experiment runs at M5.5 using cassettes and config-only arms (both topologies are parameterizations of the same runtime per ADR-0002 — no code fork needed); no packaging polish until M8 (`git clone && pnpm i && pnpm wepld` is the distribution until then).

**Success criterion for the whole program:** the Phase C thesis readout (v2-01 metrics) collected from 10–20 design partners on the M8 build — not feature count, not stars, not demo applause.
