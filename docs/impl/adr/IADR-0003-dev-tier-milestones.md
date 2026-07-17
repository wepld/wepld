# IADR-0003 — Early milestones run at a declared "dev" sandbox tier; real sandboxing lands at M4

**Status:** Accepted · **Date:** 2026-07-13 · **Scope:** implementation only — uses ADR-0007's own mechanism

## Context

The sandbox (v2-05) is substantial work per OS. Building it before anything demos violates "never build invisible infrastructure for weeks." But running workers unsandboxed while *claiming* containment would violate the architecture's honesty rule.

## Decision

The tier system ships in M0 with one initial tier: **`DEV` — no containment, full disclosure**. It is detected (i.e., hardcoded until M4), written to the ledger (`SandboxTierDetected{tier: DEV, statement: "no isolation — development tier"}`), displayed everywhere a tier is displayed, and **capped exactly as ADR-0007 prescribes**: Manual mode only, fixture repos only (a guard refuses missions on repositories outside `fixtures/` while tier is DEV, overridable by an explicit `--i-understand-dev-tier` flag for the founder's own throwaway repos). M4 implements the founder's-OS real tier (IADR-0005) and lifts the caps per the ADR-0007 table.

## Why

The architecture already contains the mechanism for honest degraded security — tier disclosure + autonomy capping. Using it for the development period means zero special cases: the MVP's security posture story and the week-2 demo's security posture story are the *same story* with different tier values. No architectural exception is created; the freeze holds.

## Trade-offs

Early demos can't safely run on arbitrary real repos (acceptable: demos use fixture repos, which are also what cassettes are recorded against). Risk of "temporary" DEV tier surviving too long — mitigated by making M4's Definition of Done include *deleting the DEV-tier mission-creation default* (DEV remains only behind the explicit flag, forever capped).

## Migration impact

None. `DEV` remains in the tier enum permanently as the honest name for "you chose to run uncontained."
