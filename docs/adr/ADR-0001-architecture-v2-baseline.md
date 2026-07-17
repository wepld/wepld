# ADR-0001 — Architecture v2 is the normative baseline

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Chief Architect · **Review:** at each phase gate

## Context

The v1 planning package (docs 01–30) passed an independent Architecture Gate Review with a **conditional GO for discovery only** and a **NO-GO for implementation as scoped**. The review found the vision and boundaries strong but identified critical gaps: a platform-sized V1 with no early user value, an unsolved sandbox story, a persona/value inversion, an undefined load-bearing dependency ("Hermes"), an unvalidated multi-worker orchestration thesis, and a specification that stated principles without mechanisms.

## Decision

Architecture v2 ([docs/v2/](../v2/00_Architecture_V2_Overview.md)) is the normative architecture. The v1 documents remain the **vision and principles layer**; where v1 and v2 conflict on scope, mechanism, or sequencing, **v2 wins**. The product identity, the fifteen non-negotiable principles, and the long-horizon vision (v1 docs 01, 02, 29) are unchanged and remain binding.

| v1 document | v2 status |
| --- | --- |
| 01, 02, 29 (Vision, Principles, Future) | Unchanged, binding |
| 03, 04 (System/Component architecture) | Amended by v2-02 (mechanisms, process model) |
| 05 (Worker architecture) | Superseded by v2-03 (single runtime, role switching) + ADR-0002 |
| 06 (Brain architecture) | Amended by v2-03 §Brain routing and v2-04 (context assembly ownership) |
| 07 (Messenger) | Amended by v2-10 (decision economics, one-agent-identity rule) |
| 08 (Knowledge) | Amended by v2-01 (MVP scope: typed records only, no extraction pipeline) |
| 09 (Skills) | Amended by v2-01 + v2-07 §Skill Contract (minimal package, defer trust machinery) |
| 10 (Loop engineering) | Preserved as the phase model; embodied in one runtime per v2-03 |
| 14 (Security) | Amended by v2-05 (sandbox tiers) + ADR-0004 (envelope enforcement) |
| 15 (Plugins) | Deferred beyond MVP; contract fields reduced per v2-07 |
| 16, 17 (Data/Event) | Superseded by v2-06 + v2-07 (transactional state + audit ledger) + ADR-0003 |
| 18 (API) | Amended by v2-07 (concrete contracts) |
| 19, 21, 22 (Roadmap/Backlog/Milestones) | Superseded by v2-09 |
| 20 (Risk) | Amended: adds orchestration-thesis, governance-overhead-at-N=1, market-timing risks |
| 23–28 | Preserved as guidance; technology decisions gated by v2-09 Phase A spikes |

## Reason

The gate review's central finding: the distance between principle quality and mechanism evidence is itself the project's largest risk. v2 closes that distance without touching the vision.

## Benefits

One unambiguous source of truth; the vision is protected from scope panic because the cuts are recorded as deliberate, reversible decisions with named evolution seams.

## Trade-offs

Two document layers must be kept coherent; every future change touching a superseded area must name which layer it amends.

## Migration impact

None on code (none exists). All Phase A/B work items in v2-09 trace to v2 documents, not v1.
