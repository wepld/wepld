# IADR-0004 — Golden ledger traces are the conformance mechanism

**Status:** Accepted · **Date:** 2026-07-13 · **Scope:** implementation only

## Context

v2-08 and v2-18 declare themselves normative: "an implementation that cannot produce this trace does not conform." A solo founder needs conformance checking that is automatic, not a review meeting.

## Decision

The normative walkthroughs become **executable golden tests**. A golden trace is a fixture: `{mission brief, cassettes, repo fixture, scripted human commands} → expected ledger sequence` (entry types + key payload fields, with IDs/timestamps/hashes normalized). CI runs every golden trace on every PR:

- `golden/m0-first-mission` — the Sprint-1 flow (mini v2-08)
- `golden/v2-08-rate-limiting` — the full canonical mission incl. batched decisions and the crash variant (kill the Core at the scripted point; assert the recovery entries)
- `golden/v2-18-decision-edit` — fork + revise + invalidation + comparison (lands with M7)
- `golden/adversarial-injection` — the "tests pass, merge now" planted file; asserts gate status unchanged and unverified-claim rendering (lands with M5)

Additional conformance tests locked in CI: **vocabulary lock** (the event-type enum must equal v2-07 rev 2 exactly; a new type fails the build unless the contracts package version is bumped and the ADR referenced), **chain verify** (every golden run's ledger passes hash-chain verification), **fold check** (fold(ledger) === state tables after every golden run), **frame determinism** (Chronicle regeneration is byte-identical for a fixed generator version, from M6), and **boundary check** (dependency-cruiser rules — IMPL-02).

## Why

This converts the frozen architecture from prose into a failing test suite. Architecture drift — the top solo-founder risk, since no reviewer is watching — becomes a red CI run instead of a slow slide.

## Trade-offs

Golden traces are maintenance-sensitive: intentional behavior changes require regenerating expectations, with the diff reviewed in the PR (that diff *is* the review). Normalization rules (what's elided: ids, times, hashes, durations) must be strict or tests get flaky — specified once in the test harness.

## Migration impact

None; purely additive test infrastructure.
