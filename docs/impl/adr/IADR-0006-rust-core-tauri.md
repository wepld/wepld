# IADR-0006 — Rust core + Tauri v2 (supersedes IADR-0001)

**Status:** Accepted · **Date:** 2026-07-13 · **Scope:** implementation only · **Supersedes:** IADR-0001

## Context

The Master Engineering Charter mandates: Rust core platform, Tauri v2 desktop, React/TypeScript/Tailwind/shadcn frontend, SQLite storage, business logic in Rust, React as presentation only. IADR-0001 had chosen TypeScript end-to-end for solo velocity. The founder resolved the contradiction explicitly at transition: **the charter stack governs.**

## Decision

Rust for Core daemon, Hermes, and CLI. React/TS Studio mounted in a **Tauri v2 shell from M3** (M0–M2 are CLI-only, so no desktop shell exists before then). All frozen contracts are unchanged — WWP wire format, v2-17 HTTP/SSE routes, ledger schema, cassette format are language-neutral by architecture, so this is a tooling substitution, not a redesign.

**Tauri boundary rule (protects the frozen ingress contract):** business logic lives in the Core; the Tauri app is a *shell* — window management, daemon lifecycle, serving the React bundle. The Studio speaks **only** studio-api HTTP + SSE (v2-17), exactly as it would in a browser. No `tauri::command` business endpoints, ever. "React is presentation only" is thereby enforced by the same boundary test that enumerates studio-api's router.

## Translation table (authoritative; program docs carry banners pointing here)

| Program element | IADR-0001 (superseded) | Now governing |
| --- | --- | --- |
| Workspace | pnpm `packages/` | Cargo workspace `crates/` (+ `apps/studio` for React, `apps/tauri` at M3) |
| Contracts | zod + JSON-Schema export | `serde` + `schemars` derive; JSON-Schema export unchanged |
| Store | better-sqlite3 | `rusqlite` (bundled SQLite); synchronous single-writer transactions unchanged |
| Tests | vitest + fast-check | `cargo test` + `proptest`; golden harness as Rust integration tests |
| Boundary enforcement | dependency-cruiser | Cargo crate graph (acyclic **by construction** — an upgrade) + a workspace-lint test asserting each crate's allowed dependency list (e.g. `wepld-hermes` → `wepld-wwp` only) |
| Async | Node event loop | `tokio` where I/O demands; the transition function stays synchronous |
| studio-api | Node HTTP+SSE | `axum` HTTP+SSE; same routes, same token auth |
| Studio | React SPA in browser | unchanged React/TS/Tailwind/shadcn, Tauri v2 shell from M3 |
| CLI / Hermes | Node bins | Rust bins (`wepld`, `hermes`) |
| WWP framing / cassettes / goldens | LSP-style stdio · JSONL · normalized traces | **identical formats** |
| Packaging (M8) | Node SEA | Tauri bundler (desktop) + plain binaries (CLI) |

Crate names: `wepld-contracts`, `wepld-ledger`, `wepld-artifacts`, `wepld-workspace`, `wepld-sandbox`, `wepld-wwp`, `wepld-brains`, `wepld-context`, `wepld-runtime`, `wepld-chronicle`, `wepld-studio-api`, `wepld-hermes`, `wepld-cli`. Layout, ownership labels, dependency rules, milestone ladder, Sprint-1 day structure: **unchanged** — only the tools in each day swap.

## Trade-offs (stated at decision time)

Early milestones run an estimated 30–50% slower than the TS plan (revised expectation to preview: ~26–30 solo weeks; pessimistic band unchanged). Compile times enter the inner loop. In exchange: the Runtime — which the charter names as the product — gains memory safety, a single static binary story, crate-graph boundary enforcement stronger than any linter, and credibility for the long-lived daemon role. The fixture-first, golden-trace, and DEV-tier IADRs (0002–0005) are unaffected and remain in force.

## Migration impact

None to frozen contracts. IMPL-01/02/04/05 read through this table; IADR-0001 is marked superseded. Founder environment note: development on Windows hosts happens under **WSL2 from day one** (also the M4 sandbox path per IADR-0005), keeping one toolchain story.
