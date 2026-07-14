# ADR-0024 — Use controlled harness evaluation and scoped profile certification

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H8 implementation authorization

## Context

Model benchmarks often change missions, prompts, tools, budgets, gates, or hidden human help between runs and optimize only for patch or test pass rate. Such results cannot establish which WePLD component caused improvement or whether the system remained safe and truthful.

## Decision

Adopt the controlled protocol in [34_Harness_Evaluation_Protocol.md](../34_Harness_Evaluation_Protocol.md). Hold mission, repository commit, approved specification, policy, OutcomeContract, tools, environment, budget class, and maximum attempts constant. Vary named brain/builder profiles and intelligence components through preregistered controls, ablations, repetitions, and interaction arms.

Measure outcome equivalence, gates, regressions, unsafe effects, evidence completeness, convergence attempts, tokens/cost, wall time, tool calls, human interventions, plan/specification changes, escalation, recovery, context/LSP/memory quality, review independence, and non-convergence honesty. Preserve failures and protocol deviations.

Certify a versioned profile only for an explicit task, language, risk, context, tool, data, and platform matrix. Drift or changed adapters/configuration triggers reevaluation, restriction, quarantine, expiry, or revocation.

## Reason

Controlled evaluation distinguishes harness value from model value, rewards safety and honesty, and makes support claims reproducible and scoped.

## Benefits

- Evidence-based investment in harness components.
- Transparent profile economics and capability scope.
- Regression and provider-drift detection.
- Publishable null, negative, and safety results.

## Trade-offs

- Repeated controlled runs are expensive.
- Fixture contamination and provider drift require active management.
- Certification cannot imply universal capability.

## Migration

H8 evidence requires preregistered full-harness, minimal-control, component-ablation, and interaction results; independent scoring; complete run manifests; safety-abort tests; scoped certification decisions; and architecture review of the method. Thresholds must be approved before results are observed.

Draft PR #1's golden traces, cassette adapter, adversarial tests, and reported validation can seed harness tooling only if independently accepted. They are reference material, not H8-controlled results, and this ADR does not ratify the PR.
