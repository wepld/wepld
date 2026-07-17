# ADR-0007 — Honest per-OS sandbox tiers bound to autonomy limits

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Security owner · **Review:** after Phase A spikes S1–S3 on all three OSes

## Context

v1 acknowledged the cross-platform sandbox gap in one sentence ("policy reduces allowed autonomy") while resting the entire trust model on sandboxing. The gate review (C2) required a per-OS design that does not pretend the platforms are equivalent. There is no uniform cross-platform primitive that contains arbitrary dev toolchains.

## Decision

WePLD defines **sandbox tiers** with explicit guarantees, detects the achievable tier at mission start, records it in the ledger, displays it in the Studio, and **caps the autonomy envelope by tier**. Full mechanics in [v2-05](../v2/05_Sandbox_Strategy.md).

| Tier | Platform basis | Filesystem | Network | Honest statement |
| --- | --- | --- | --- | --- |
| S0 | Container backend (Podman/Docker), any OS | strong | strong | strongest available; optional dependency |
| S1 | Linux native: namespaces + cgroups v2 + Landlock (+ seccomp) | strong | strong | reference platform |
| S2 | macOS: Seatbelt profile + rlimits + proxy-enforced egress | strong | **best-effort** | egress control is proxy-based, not kernel-guaranteed |
| S2W | Windows via WSL2 (S1 inside) | strong | strong | recommended Windows path when WSL2 present |
| S3 | Windows native: restricted token + Job Object + worktree ACLs | moderate | **weak** | no syscall filtering; reduced autonomy enforced |

Tier caps (defaults): S0/S1/S2W — full Bounded-Auto envelope; S2 — Bounded-Auto but network extensions always human-gated; S3 — Manual-leaning: any network use, dependency install, or test execution of untrusted repos requires approval.

## Reason

Security that varies by platform must be *stated*, not averaged. Binding tier to autonomy converts an uncomfortable fact into a governable, visible policy input — exactly the product's own philosophy (evidence before assertion) applied to itself.

## Benefits

No false uniform promise; Windows ships honestly instead of blocking the product; the tier is a single tested, versioned surface per OS; container backend gives security-sensitive users a strong opt-in everywhere.

## Trade-offs

Real capability differences by platform will surface in reviews and support; S2 macOS relies on Seatbelt (`sandbox-exec`), which Apple has deprecated but still uses internally and which Bazel/Nix depend on — a monitored platform risk with the container backend as fallback. S3 autonomy limits will frustrate native-Windows users; WSL2 guidance mitigates.

## Migration impact

Envelope schema (v2-07) carries `sandbox_tier`; nothing else changes when a platform's tier improves. If Seatbelt is removed in a future macOS, S2 falls back to S0 guidance without contract change.
