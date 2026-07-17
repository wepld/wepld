# IADR-0005 — Founder-OS-first: one real sandbox tier before breadth

**Status:** Accepted · **Date:** 2026-07-13 · **Scope:** implementation only

## Context

v2-05 defines tiers for Linux (S1), macOS (S2), Windows (S2W/S3), containers (S0). Implementing three platforms before design partners is 3× the work for 1× the learning.

## Decision

M4 implements exactly **one real tier: the founder's daily-driver OS** (decision recorded at Sprint-1 kickoff in the repo's `DECISIONS.md`; the plan assumes the common case — macOS → S2 Seatbelt, or Linux → S1 bubblewrap; Windows founders start with S2W/WSL2, which reuses the S1 implementation). The `sandbox` package's tier interface is written against the full v2-05 tier table from day one; other tiers return `unsupported` honestly until M8, when the design-partner cohort's actual OS mix decides which tier ships second. S0 (containers) is the documented workaround for unsupported platforms in the interim.

## Why

Every sandbox hour must serve the next demo and the first cohort. The tier abstraction (frozen architecture) already guarantees that adding platforms later is additive; hardcoding *which* platform first is pure sequencing, not architecture.

## Trade-offs

Early design partners must match the founder's OS or accept S0/containers — an acceptable cohort filter at N≤20. Cross-platform CI for the sandbox package is deferred (unit tests mock the launcher; the canary self-test runs only on the supported OS runner).

## Migration impact

None; the tier table and detection contract are unchanged.
