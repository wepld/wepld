# IMPL-05 — Package-by-Package Specifications

> **Amended by [IADR-0006](adr/IADR-0006-rust-core-tauri.md):** packages are Cargo crates (`wepld-<name>`); tool references translate per the IADR-0006 table. Purposes, boundaries, tests, and acceptance bars unchanged.

Public APIs are specified in IMPL-02; this document adds purpose, internals, dependencies, tests, acceptance, and future extensions per package. "Acceptance" = merge bar for the package's first complete version; later milestones extend via the Future rows only.

---

### `contracts` (L0 · spine)
**Purpose:** the frozen architecture as importable truth — zod schemas + inferred types for every v2-07/v2-17 contract, the closed event enum (rev 2), WWP message shapes, JSON-Schema export for language-neutral consumers.
**Internal:** `schemas/` one file per contract; `vocabulary.ts` (event enum); `versions.ts`.
**Deps:** zod. **Tests:** round-trip parse/serialize per schema; vocabulary-lock; JSON-Schema snapshot.
**Acceptance:** every field named in v2-07/v2-17 exists and is typed; no `any`; no logic.
**Future:** additive minors only; major = coexistence window per v2-07 rules.

### `ledger` (L1 · spine)
**Purpose:** SQLite store — state tables, hash-chained append-only ledger, fold reducer, checkpoints, work queue. ADR-0003 embodied.
**Internal:** `db.ts` (open/pragma/guards), `chain.ts`, `fold/` (the reducer — pure, no I/O), `tables/`, `queue.ts`.
**Deps:** contracts, better-sqlite3.
**Tests:** property (random transition sequences → fold==tables), chain tamper detection, crash-in-transaction (child-process kill harness), synced-folder refusal, queue at-least-once claim semantics.
**Acceptance:** `verifyChain` catches any single-byte payload mutation; fold of golden traces equals table state; 10k-entry timeline query < 100 ms.
**Future (M6):** checkpoint table + `fold(upto)` fast path; (V2, frozen path) promotion per v2-06 §Promotion.

### `artifacts` (L1)
**Purpose:** content-addressed evidence store with classification/retention metadata and tombstoning.
**Internal:** `store.ts` (sharded dirs), `meta.ts` (rows live in the ledger DB for one-file backup).
**Deps:** contracts. **Tests:** put/get/verify, dedup, tombstone leaves hash+reason, corrupt-body detection on read.
**Acceptance:** identical content stored once; every get re-verifies hash.
**Future:** streaming bodies (M3, log tailing); export bundles (Chronicle V1).

### `workspace` (L1)
**Purpose:** Git as isolation + time machine: per-attempt worktrees, hidden snapshot refs (ADR-0013), diffs, scope re-verification, fork restore.
**Internal:** `git.ts` (spawned `git`, no libgit binding — fewer native deps), `refs.ts` (`refs/wepld/**` naming + retention sweep), `worktrees.ts`.
**Deps:** contracts. **Tests:** integration on fixture repo — worktree lifecycle, snapshot/materialize/diff, `changedPaths` vs. scope globs, branchFrom, interrupted-operation cleanup (kill mid-checkout → recover).
**Acceptance:** user-visible `git log`/`status` in the primary worktree never shows WePLD noise; snapshot+materialize round-trips byte-identical trees.
**Future:** salvage support (M7/V1), bisection driver (V2).

### `sandbox` (L1)
**Purpose:** tier detection with canary self-test; envelope-enforcing process launcher; ADR-0007 caps table.
**Internal:** `tiers/` one launcher per tier (`dev.ts` now; `s1_bwrap.ts`/`s2_seatbelt.ts`/`s2w_wsl.ts` per IADR-0005; `s0_container.ts` fallback), `canary.ts`, `caps.ts`.
**Deps:** contracts. **Tests:** unit (envelope→launcher args); on the supported OS runner: canary asserts forbidden read/write/egress actually fail; quota kill (memory/time) on a runaway fixture.
**Acceptance (M4):** the tier reported is the tier *proven* by the canary, never the tier hoped for.
**Future:** second tier at M8; WFP/network tiers per v2-05's honest roadmap.

### `wwp` (L2)
**Purpose:** the worker boundary — JSON-RPC 2.0 framing over stdio, heartbeat watchdog, cancellation, typed dispatch for the v2-07 §2 message set. ADR-0005's contract made code.
**Internal:** `frame.ts` (LSP-style Content-Length framing), `server.ts` (Core side), `client.ts` (worker side), `watchdog.ts`.
**Deps:** contracts. **Tests:** the **WWP conformance harness** (also exported for third parties): scripted sessions asserting lease honor, heartbeat cadence, cancel compliance, envelope-denial behavior, schema validity of every message; fuzzed malformed frames.
**Acceptance:** Hermes passes the harness; a deliberately misbehaving test worker is detected and killed within one heartbeat interval.
**Future:** socket transport + lease.renew (V2 fleet — additive, per ADR-0002).

### `providers` (L2) — renamed from `brains` per IADR-0007 §5
**Purpose:** provider neutrality (the architecture's Brain Gateway role): gateway (validate → profile-route → budget-gate → invoke → schema-check → single reformat retry → record) + adapters `fixture`, `anthropic`, `openaiCompat`. Credentials only here, from OS keychain/env. Invocation is optional per IADR-0007 §1 — deterministic phases make zero calls and that is a first-class path.
**Internal:** `gateway.ts`, `profiles.ts` (named configs as data), `adapters/`, `cassette.ts` (IADR-0002 record/replay).
**Deps:** contracts (+ artifacts via injected callback for pack/response storage — keeps L2 flat).
**Tests:** cassette replay determinism; schema-failure path (invalid → retry → SCHEMA error); budget denial; adapter conformance run against cassettes recorded from real APIs.
**Acceptance (M1):** the same mission passes on both real adapters (provider-swap fixture); no package outside `brains` mentions a provider name — grep-enforced in CI.
**Future:** fallback chains, dual-profile disagreement (V2), evaluation suites.

### `context` (L2)
**Purpose:** Context Assembly per v2-04 — tiers T0–T4, deterministic selection with manifest, compression via phase summaries, redaction + trust fencing, pack capture.
**Internal:** `tiers.ts`, `select/` (seeds, import-neighbor expansion, identifier mentions, ranking, packing), `redact.ts` (token patterns + entropy), `pack.ts` (serialize/hash/capture).
**Deps:** contracts, artifacts.
**Tests:** manifest correctness (included/excerpted/omitted-with-reason exactly reflects decisions); T0-overflow throws; redaction corpus (known key formats → placeholders, log entry emitted); determinism (same inputs → same pack hash); budget-split respected.
**Acceptance (M1):** every brain invocation in golden traces references a retrievable pack whose manifest explains every inclusion and omission.
**Future:** semantic ranking signal (V2, inside `select/` — pack format frozen), knowledge T3 wiring as knowledge records grow.

### `runtime` (L3 · spine)
**Purpose:** the Core: command pipeline, transition function, phase engine, Core-run gates, decisions/interrupt budget, messenger composition, budgets, recovery. v2-02 §§2–10.
**Internal:** the eight modules of IMPL-02's table; `state-machine/apply.ts` is the only `Tx` consumer in the codebase.
**Deps:** all L1+L2.
**Tests:** unit per module (transition guards, budget math, batching rules, failure taxonomy dispatch); integration: full phases against fixture brain; the crash matrix (M5); every golden trace.
**Acceptance:** the four v2-02 §4 completion guards are individually tested (gate evidence, review disposition, scope re-check via `changedPaths`, budget); no state change bypasses `apply()` (lint rule).
**Future:** Full-Auto preset (config), parallel tasks behind footprint rule (V2), salvage.

### `hermes` (app · reference worker — the flagship, IADR-0007 §4)
**Purpose:** the flagship WWP runtime — an autonomous engineering runtime, not "an agent that calls an LLM." Phase loop: get pack → execute deterministic engineering itself (analysis, edits, commands, validation *inside its envelope*) → invoke `brain.request` **only when reasoning would improve the mission** (IADR-0007 §1) → artifacts → phase.result with schema-enforced summary. The user hires Hermes; Hermes hires models. Proves runtime replaceability by importing only `wwp`.
**Internal:** `loop.ts`, `actions.ts` (edit-script applier), `roles/` (prompt scaffolds per role profile — data, not code).
**Deps:** wwp (contracts transitively). **CI rule:** any further dependency fails the build.
**Tests:** WWP conformance harness; edit-script property tests (apply→diff→reapply idempotent); cancellation mid-action.
**Acceptance (M0):** killable at any point with recoverable, explainable ledger state.
**Future:** richer toolterms (search, multi-file refactors) — always within envelope; never a user channel (no such WWP verb exists).

### `chronicle` (L4)
**Purpose:** Chronicle MVP per v2-11: checkpoints/state_at, frame generator + 4 lenses, replay sessions, causal walk, fork planning, comparison, stats.
**Internal:** `frames/` (rules per entry type, generator_version), `lenses.ts`, `session.ts` (v2-12 state machine), `causal.ts` (deterministic edges MVP), `fork.ts` (invalidation cone → ForkPlan), `compare.ts`.
**Deps:** ledger, artifacts, workspace, contracts — **never runtime** (IMPL-02 rule 4).
**Tests:** frame determinism (byte-identical regeneration); `state_at(seq)` == fold(upto seq) for all golden traces; session transitions incl. FOLLOWING detach; fork plan on golden `v2-18` matches expected invalidation set; comparison facets on the same golden.
**Acceptance (M6/M7):** golden `v2-18-decision-edit` green end-to-end; replay of the M0 mission (recorded before Chronicle existed) works unmodified — the retroactivity proof.
**Future:** RCA reports, Mission Map, insights (V1 per v2-11 table) — new rules over the same stores.

### `studio-api` (L4)
**Purpose:** the only ingress: loopback HTTP + SSE, startup session token, v2-17 routes; command port for Studio/CLI/Chronicle-UI actions.
**Internal:** `auth.ts`, `routes/` (commands, queries, stream, replay, forensics, compare), `sse.ts` (ledger tail w/ cursor resume).
**Deps:** runtime, chronicle, contracts.
**Tests:** auth (no token → 401; token in URL only at bootstrap), route contract snapshots, SSE resume-from-cursor, command idempotency pass-through.
**Acceptance (M3):** the route list equals the documented subset of v2-17 — an undocumented route fails a conformance test that enumerates the router.
**Future:** channels mount here (V3); second-seat read-only tokens (V2 candidate).

### `studio` (app)
**Purpose:** the product surface — Mission (brief form, plan matrix, status, tier/budget display), Decisions (packet inbox, claims rendering), Cinema at M6 (player, scrubber, drawers; Contact Sheet from M3). Visual screens are v0-generated per [IADR-0008](adr/IADR-0008-v0-ui-integration.md); engineering owns the integration layers.
**Internal:** `screens/` (v0-generated, presentation-only — lint-enforced), `viewmodels/`, `state/`, `transport/` (typed dual-transport client), `design/` (versioned v0 prompts).
**Deps:** none in-workspace (HTTP only); React, Vite.
**Tests:** component tests for claim rendering (verified vs. unverified styling — the v2-10 invariant); Playwright E2E: create→decide→accept; M6: scrub/step/follow flows.
**Acceptance (M3):** a mission is completable start-to-finish without the CLI; nothing renders a gate/status not backed by a ledger query.
**Future:** Mission Map, split view, heatmaps per v2-11 classification.

### `cli` (app)
**Purpose:** founder's daily driver and M0 demo vehicle: `init`, `daemon`, `mission new/plan/run/accept`, `timeline`, `verify`, `demo`; later `replay --json`, `fork`.
**Deps:** runtime, studio-api (to launch daemon+UI), chronicle.
**Tests:** golden traces execute through the CLI's command path (so the CLI *is* covered by every golden run); snapshot tests on `timeline` formatting.
**Acceptance (M0):** the Day-10 demo script runs unmodified.
**Future:** stays forever — scriptability is a feature, and the goldens depend on it.
