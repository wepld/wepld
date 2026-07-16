# ADR-0022 — Engineering loops record hypotheses and stop through one escalation model

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H5 implementation authorization

## Context

The current architecture requires evidence-producing retries and stop conditions, but does not define a per-iteration record, belief update, complete no-progress guards, or one escalation ladder shared across skills and profiles. Repeated model calls can look active while making no progress.

## Decision

Use the bounded loop in [32_Hermes_Engineering_Intelligence_Runtime.md](../32_Hermes_Engineering_Intelligence_Runtime.md): Observe → diagnose → hypothesize → select minimal action → execute through the Effect Firewall → verify → compare → update belief → continue, replan, escalate, or stop.

Record hypothesis, before evidence, intended action, expected result/falsifier, actual result, after evidence, confidence/diagnostic delta, cost, and next decision for every iteration. Guards detect repeated or equivalent actions, no state change, oscillation, increasing diagnostics or blast radius, unchanged hypotheses, schema failures, exhausted budgets, stale context, invalid plans, unresolved uncertainty, and required human authority.

Use one escalation ladder: improved context, specialized skill, task split, bounded advisor/reviewer, plan change, certified profile switch, clarification, human decision, and safe stop.

A sandbox or capability denial produces a typed `SandboxFailureResult` naming the boundary, policy/capability reason, attempted effect, retryability, safe alternatives, required authority, recovery state, and evidence reference. Hermes cannot repeat an identical denied action unless the hypothesis, capability, plan, or authority has materially changed.

A `ContextualRiskAdvisor` may be evaluated only as an advisory treatment after deterministic allowlist/sandbox/policy checks. It may recommend allow-under-existing-policy, narrow, block, or request authority and return rationale/safer alternatives. It cannot grant capability, override deterministic denial, approve a protected effect, change policy, replace Core authorization, or be the sole security boundary.

## Reason

Progress becomes falsifiable rather than narrative. One loop contract makes retries, budgets, recovery, profile comparison, and honest non-convergence testable.

## Benefits

- Explicit no-progress and oscillation detection.
- Comparable, reconstructable attempts.
- Evidence-driven replanning and safer stopping.
- Preserved uncertainty and partial results.

## Trade-offs

- Iteration evidence increases storage and implementation complexity.
- False-positive guards can stop useful exploration.
- Policies require task- and risk-specific thresholds.

## Migration

H5 evidence must exercise every guard; retry with a changed hypothesis; typed denial recovery; prohibition of identical denied-action replay; safe alternatives and correct authority escalation; safe partial output; budget/time exhaustion; schema failure; specification-versus-plan change escalation; uncertain external effects; and exact reconstruction of why each loop continued or stopped. RS-27 must reduce repeated denials without authority escape. RS-28 remains an experiment measured for false allow, false block, flapping, latency, interruption reduction, and unsafe-effect escape, and is disabled/rejected on any grant/override behavior or unacceptable instability.

Draft PR #1's attempts, heartbeat/watchdog, brain-call cap, `Deferred`/`Uncertain`, gates, and recorded failures are candidate primitives. Its single-call phase paths are not the proposed Loop Engine, and this ADR does not authorize their extension.
