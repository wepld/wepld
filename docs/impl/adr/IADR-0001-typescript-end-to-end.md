# IADR-0001 — TypeScript end-to-end

**Status:** SUPERSEDED by [IADR-0006](IADR-0006-rust-core-tauri.md) (charter stack: Rust core + Tauri v2) · **Date:** 2026-07-13 · **Scope:** implementation only — no architectural contract changes

## Context

Architecture v2 deliberately left the implementation language open (v2-09 Phase A required "a real ADR with Go/TS honestly considered"; v2-23/v1 named Rust only as a *candidate*). The deciding constraint is now fixed: **one engineer, maximum probability of success, continuous demos.**

## Decision

TypeScript everywhere: **Node.js ≥ 22** for the Core daemon, Hermes worker, and CLI; **React + Vite** for the Studio; **pnpm workspaces** monorepo; **better-sqlite3** for the store; **zod** for contract schemas (JSON-Schema exported from zod definitions so contracts stay language-neutral); **vitest** for tests; **dependency-cruiser** for boundary enforcement.

Considered: Rust (best runtime footprint; slowest solo iteration; hiring/future-proofing irrelevant at N=1), Go (good middle; still two languages once the Studio exists). TypeScript wins on the only axis that matters now: one language across daemon, worker, UI, tests, and fixtures means zero context-switching and the fastest possible loop.

## Why this is safe

- The domain is I/O orchestration — the Core waits on LLMs, builds, and humans. At MVP scale (≤5k ledger entries, ≤2 processes), Node's performance is irrelevant; v2-27 targets are comfortably met.
- `better-sqlite3` is **synchronous**, which is not a compromise but a fit: the single-writer transition function (v2-06) is naturally a synchronous transaction — no async-transaction hazards.
- Sandbox launchers (`bwrap`, `sandbox-exec`, Job Objects via a helper) are external processes; the launching language is immaterial.
- Every cross-process contract (WWP JSON-RPC, HTTP/SSE, JSON-Schema contracts) is language-neutral by architecture. A future Rust/Go rewrite of any hot component is a component swap, not a redesign.

## Trade-offs

Native-module friction (better-sqlite3 rebuilds) on some platforms — pinned via prebuilds; single-file distribution is less elegant than a Rust binary — deferred to M8 packaging (Node SEA or bundled runtime). Memory footprint higher than Rust — irrelevant at one mission at a time.

## Migration impact

None on frozen contracts. Repo layout (IMPL-01) uses `packages/` not `crates/`; v1 doc 24's crate map remains the shape reference, translated.
