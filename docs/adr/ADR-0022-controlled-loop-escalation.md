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

H5 evidence must exercise every guard; retry with a changed hypothesis; safe partial output; budget/time exhaustion; schema failure; specification-versus-plan change escalation; uncertain external effects; and exact reconstruction of why each loop continued or stopped.

Draft PR #1's attempts, heartbeat/watchdog, brain-call cap, `Deferred`/`Uncertain`, gates, and recorded failures are candidate primitives. Its single-call phase paths are not the proposed Loop Engine, and this ADR does not authorize their extension.
