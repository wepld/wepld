# IADR-0008 — Visual design via v0; engineering owns the integration seam

**Status:** Accepted · **Date:** 2026-07-13 · **Scope:** implementation only — Architecture v2.0, Chronicle, and the charter remain frozen and governing

## Context

The founder separated Visual Design (generated externally with v0: layouts, page composition, dashboards, component appearance, interaction prototypes) from Platform Engineering (everything else). The Rust Runtime remains the primary engineering focus; frontend engineering effort goes to architecture and integration, not pixel design.

## Decision

### Layered frontend architecture (fixed now, built at M3 per runtime-first order)

~~~text
screens/        v0-generated components — PRESENTATION ONLY (props in, callbacks out)
   ↓
viewmodels/     typed mappings: contract types → display props; the ONLY layer screens import
   ↓
state/          one lightweight app store (Zustand-class; final pick at M3 kickoff) + SSE/event cursor
   ↓
transport/      generated typed client from @wepld contracts — dual transport:
                Tauri Commands/Events (desktop) and HTTP+SSE (dev/tests/CLI-web), same handlers
   ↓
Rust Core (studio-api → runtime)
~~~

### Integration rules for generated screens

1. Generated code lands in `apps/studio/src/screens/` **unmodified where possible**; adaptations only for performance, maintainability, security, accessibility, state management, or Runtime integration — never visual redesign without a functional limitation.
2. **Mechanical enforcement of "presentation only":** a lint rule forbids `screens/` from importing `transport/`, `tauri`, `fetch`, or the store — screens receive view-model props and emit callbacks, nothing else. This makes "React owns no business logic" a CI failure instead of a review hope.
3. View models expose **only ledger-derived data** — the v2-10 claims discipline (verified chips resolve evidence refs; unverified prose demoted) is implemented in the view-model layer so no generated screen can weaken it, accidentally or otherwise.
4. Every screen must render correct **loading, error, offline, and stale** states from a standard view-model envelope (`{data, freshness, syncState}`) — generated designs get these states wired in, not painted on.
5. v0 prompts derive from v2-13/v2-01 surface specs (Mission Control / engineering studio / digital twin — never chat UI); prompt files are versioned in `apps/studio/design/` so screens are regenerable.

### What does not change

Runtime-first order (Contracts → Core → Runtime → Storage → Verification → API → Frontend Integration → Visual Polish); no frontend work before M3; the M3 Definition of Done gains one line — *screens are v0-integrated, not hand-designed*; the studio-api dual-transport rule of IADR-0006/0007 is unchanged and is exactly the seam v0 screens plug into.

## Benefits / Trade-offs

Speed and professional polish without founder design hours; the integration contract keeps generated churn away from the Runtime. Trade-off: generated code quality varies — bounded by rule 1 (adapt minimally) and rule 2 (blast radius is one directory); regeneration may fight local edits — mitigated by keeping adaptations in wrapper components rather than editing generated files where feasible.

## Migration impact

None to code (no frontend exists yet). IMPL-05 `studio` spec amended to reference this IADR.
