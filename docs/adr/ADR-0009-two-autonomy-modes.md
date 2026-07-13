# ADR-0009 — Two autonomy modes in the MVP: Manual and Bounded-Auto

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Product/CTO · **Review:** end of Phase C

## Context

v1 defined four modes (Manual, Limited Approval, Full Autonomous, Enterprise Policy). Each mode multiplies the policy/approval test matrix, and the differences between Limited and Full are envelope parameters, not distinct semantics (gate finding M6).

## Decision

MVP ships two modes over one mechanism:

- **Manual** — the plan and every hard-gate crossing require approval; phase results are reported, not gated.
- **Bounded-Auto** — the plan requires approval; execution proceeds autonomously inside the declared envelope; only hard gates (ADR-0004 list) interrupt.

Both are parameterizations of the same envelope + hard-gate machinery; "mode" is a preset, not a code path. Full-Autonomous (plan auto-approval within envelope) and Enterprise Policy (externally supplied presets and routing) are V2 presets over the same mechanism, per v2-09.

## Reason

The thesis experiment needs the two ends of the trust spectrum, not four gradations. One mechanism with presets prevents mode-specific logic from fossilizing.

## Benefits

Halves the approval-flow test matrix; makes "mode" explainable in one sentence each; later modes are configuration, not architecture.

## Trade-offs

Enterprise conversations lose a checkbox temporarily. Acceptable: the enterprise mode was never load-bearing for the thesis.

## Migration impact

Mission Contract's `autonomy_mode` field is an enum that grows; no schema break.
