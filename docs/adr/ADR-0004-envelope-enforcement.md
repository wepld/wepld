# ADR-0004 — Envelope enforcement with a short hard-gate list, not per-action capability adjudication

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Security owner · **Review:** after Phase A sandbox spikes

## Context

v1 required every tool effect to pass a policy decision and carry a scoped capability token. The gate review (C2) showed this contradicts real toolchains: `npm install`, `cargo build`, and test suites execute arbitrary code by design, so once "run the build" is approved, per-action adjudication inside that step is theater — the sandbox boundary is the only real control. Per-action tokens also put the policy engine on the latency path of every effect.

## Decision

Enforcement is **coarse-grained and honest**:

1. Each attempt/phase receives a **sandbox envelope** — filesystem scope (worktree + declared read paths), network policy (default deny), resource quotas, secret set (default empty), time limit — enforced by the OS-level sandbox tier (v2-05), not by worker cooperation.
2. A short, fixed **hard-gate list** always requires a Core-mediated decision regardless of autonomy mode: enable/extend network egress, install a new dependency, access a secret, write outside the worktree, merge to a protected branch, expand scope or budget, any destructive operation, any external data transfer.
3. Workers request envelope *extensions* (`envelope.extend` in WWP); the Core converts gated requests into decision packets or auto-approves within the mission's declared envelope, and records both in the ledger.

Capability tokens survive in one place: the envelope itself is a signed, expiring grant bound to attempt id, paths, network policy, and quotas — one token per phase, not one per action.

## Reason

Security claims must match enforcement reality. The envelope model states exactly what is contained (the blast radius of arbitrary code) and exactly what is decided (crossings of the envelope), instead of implying per-command control that no OS delivers for dev toolchains.

## Benefits

Removes the policy engine from the hot path; shrinks MVP policy to a declarative envelope schema plus a hard-gate table; produces an audit trail that reflects true containment; makes the sandbox tier the explicit, testable security statement per platform.

## Trade-offs

Coarser audit granularity inside an approved phase (compensated by command logging inside the sandbox as *observability*, explicitly not as *enforcement*). Fine-grained capability policy remains possible later for specific high-risk tools by routing them through Core-side executors — additive, per tool, when justified.

## Migration impact

v1's Policy Decision Point survives as the envelope grantor and hard-gate evaluator. A future generalized policy engine (V2+) replaces the hardcoded hard-gate table with rules — same enforcement points, no worker-contract change.
