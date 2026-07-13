# ADR-0008 — MVP is "One Governed Mission"

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Product/CTO · **Review:** end of Phase C (design partners)

## Context

v1's "narrow vertical slice" still required the daemon, event sourcing, policy engine, worker fleet, brain gateway, sandbox on three OSes, artifact store, Messenger, Mission Control, Timeline, and a Tauri Studio before one user completed one mission (gate finding C1). The market ships monthly; the thesis needs contact with users in months, not years.

## Decision

The MVP is defined in [v2-01](../v2/01_MVP_One_Governed_Mission.md): one user, one repository, one mission at a time, executed by one WWP worker runtime through governed phases with envelope isolation, evidence gates, a decision queue, an append-only ledger, and a three-surface Studio (Mission, Timeline, Decisions). Everything cut has a named seam and an earn-back trigger in [v2-09](../v2/09_Roadmap_and_Sizing.md).

**Optimization target: prove the product thesis with the least engineering effort — not build the smallest product.** The thesis: *a user will delegate bounded engineering work and accept the result when it is isolated, evidence-gated, decision-routed, and replayable.* Every MVP component exists to measure that sentence; anything that doesn't is out.

## Reason

Each of the v1 north-star metrics (mission acceptance rate, interruptions per mission, evidence coverage, local operation, provider swap) is measurable against this MVP. Nothing smaller can measure them; nothing larger measures them sooner.

## Benefits

3–4 engineers to a design-partner preview in roughly one to two quarters (sizing in v2-09); the identity — mission brief, evidence, decisions, timeline, studio-not-editor — is fully present; false-negative risk on the thesis (C3) is reduced because the interrupt budget and solo-wedge framing (ADR-0010) are built in.

## Trade-offs

No parallelism, no channels beyond the Studio inbox, no skills registry, no knowledge extraction, no marketplace, no Tauri packaging decision yet. Competitors will have flashier demos; WePLD's demo is a mission a user can *audit and replay*, which is the point.

## Migration impact

None backward. Forward: every deferred v1 system maps to a v2 seam — fleet → WWP transport swap; policy engine → hard-gate table replacement; event sourcing → ledger promotion; registry → skill dir → package lifecycle. See v2-09 §Earn-back.
