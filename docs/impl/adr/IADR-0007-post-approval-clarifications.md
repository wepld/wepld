# IADR-0007 — Post-approval implementation clarifications

**Status:** Accepted · **Date:** 2026-07-13 · **Scope:** implementation only — Architecture v2.0 and Chronicle remain frozen; no contract changes

## Context

At implementation approval, the founder issued six clarifications. None alter frozen contracts; each sharpens how the program reads them. Recorded here so every future PR can cite one authority.

## 1. Runtime owns engineering; reasoning is an optional tool

"Hermes-only mode" does **not** mean "no reasoning" — it means **no model inference**. Hermes (and the Runtime around it) performs deterministic engineering itself: orchestration, workflow execution, file and Git operations, project analysis, build/test/benchmark execution, replay and chronology generation, ledger operations, context assembly, policy evaluation, sandbox management, artifact management, dependency analysis, template-based code generation, validation, verification, and state transitions. A Reasoning Provider is invoked **only when the worker determines reasoning would improve the mission** — architectural planning, complex debugging, large refactoring, ambiguity resolution, design exploration, multi-option evaluation, long-horizon planning.

Implementation consequences: `brain.request` is *optional* per phase, never a mandatory loop step; the phase engine and Hermes treat "zero invocations" as a normal, first-class execution path; a mission whose tasks are fully deterministic completes with an empty `brain_invocations` table and that is a *feature* (cost 0, fully local). The relationship is never inverted: **the user hires Hermes; Hermes hires models when needed.**

## 2. Strict Core/UI separation, reconciled with the frozen ingress contract

React is presentation only — never business, mission, worker, ledger, policy, chronicle, reasoning, context, transition, or verification logic. The desktop flow is **React → Tauri Commands/Events → Rust API → Runtime**. Reconciliation with IADR-0006 and v2-17: there is **one API surface, two transports**. The `studio-api` crate owns all command/query/stream handlers (the v2-17 semantics). The Tauri shell exposes those same handlers as thin generated `tauri::command`/event wrappers (desktop transport); the HTTP+SSE server exposes them for CLI, tests, golden traces, and any future web/SwiftUI/Flutter frontend. Wrappers contain zero logic — a wrapper with a conditional is a review-blocking defect. The Runtime never depends on React; deleting `apps/studio` must not break `cargo test --workspace`.

## 3. Runtime-first development order

Within every milestone the build order is: **Contracts → Core → Runtime → Storage → Verification → API → UI.** The UI is always the final consumer. This was already the program's implicit order (IMPL-03); it is now an explicit per-milestone rule.

## 4. Hermes product identity

Connecting to models is not defensible — anyone can. The defensible asset is **Hermes as an autonomous engineering runtime**: it owns planning, execution, verification, recovery, learning, mission management, and orchestration; providers only extend it. Marketing, docs, and code comments must never describe Hermes as "an agent that calls an LLM."

## 5. Neutral internal naming

Platform crates use neutral names; "Hermes" names only the flagship worker implementation. Applied renames: `crates/brains` → **`crates/providers`** (crate `wepld-providers`; the architecture's Brain Gateway is its *role*, not its package name). Already-neutral names stand: `contracts, ledger, artifacts, workspace, sandbox, wwp, context, runtime, chronicle, studio-api, cli`. `crates/hermes` keeps its name because it specifically *is* the Hermes implementation — the one case the clarification permits. The platform must always support future WWP runtimes; the CI rule "hermes depends only on wwp" remains the enforcement.

## 6. Priority discipline

Continue exactly per the frozen program: no redesign, no new subsystems, no scope expansion, small commits, runnable main, session reports in the mandated format.

## Migration impact

Doc banners/edits in IMPL-01/02/03/05; no code existed yet under the old `brains` name, so the rename is free. No frozen document is touched.
