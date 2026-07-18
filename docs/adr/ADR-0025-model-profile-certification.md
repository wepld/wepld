# ADR-0025 — Certify model profiles from controlled accumulated evidence

**Status:** Proposed
**Date:** 2026-07-16
**Owner:** Architecture Review Board
**Review:** Before H8 implementation authorization

## Context

The evaluation spine in ADR-0024 preserves comparable evidence, but stored runs alone do not justify provider or model support claims. Certification requires controlled cross-model experiments, sufficient repetitions, ablations, independent scoring, drift handling, and a scope that reflects task, language, risk, tools, data, and platform constraints.

## Decision

At H8, consume the accumulated ADR-0024 run history through the controlled protocol in [34_Harness_Evaluation_Protocol.md](../34_Harness_Evaluation_Protocol.md). Preregister comparisons; hold mission, repository, approved contracts, policy, tools, environment, budget, gates, and attempt limits constant; and use randomized or counterbalanced repeated runs where nondeterminism or temporal provider effects matter.

Compare governed full-harness and minimal-governed controls, component ablations, supported profile arms, and only justified interaction arms. Measure outcome equivalence, mandatory gates, regressions, unsafe effects, evidence completeness, convergence and honest non-convergence, cost/time/tool use, human intervention, escalation, recovery, context/LSP/memory quality, review independence, protocol deviations, and uncertainty.

Risk-triggered `ControlledMultiRouteRace` arms may compare multiple plans or builders only under the fixed Outcome Contract and controlled variables in ADR-0023. Each route receives an independent gate result; no model judge, vote, score ranking, or visual preference carries certification or acceptance authority.

Certify a versioned model/profile only for an explicit task, language, risk, context, tool, data, and platform matrix. Certification states are `Candidate`, `Certified`, `Restricted`, `Quarantined`, `Expired`, or `Revoked`. Provider/model drift, adapter or prompt/configuration changes, changed tool authority, material harness changes, incidents, expiry, or measured regression trigger reevaluation or restriction.

Certification is an authenticated governance decision based on evidence. Model voting, benchmark reputation, and a single successful run carry no approval authority.

## Reason

Separating early evidence capture from H8 certification makes both decisions independently reviewable and prevents premature support claims while preserving the history needed for credible portability and convergence decisions.

## Benefits

- Scoped, reproducible provider and model support claims.
- Causal evidence for retaining or removing harness components.
- Explicit drift, expiry, quarantine, and revocation behavior.
- Portable outcome standards without lowering gates for weaker profiles.

## Trade-offs

- Repeated controlled runs and independent review are expensive.
- Fixture contamination and provider drift require ongoing management.
- Certification cannot imply universal capability or exact replay of hosted models.

## Migration

H8 entry requires an operational ADR-0024 spine and accumulated compatible evidence from the terminal and assessed pre-H1 foundation Baseline run with finalized `EvaluationResult`, plus H1 through H7. H8 exit requires preregistered full-harness, minimal-control, ablation, repeated/randomized or counterbalanced, cross-profile, and any risk-triggered RS-29 race evidence where applicable; independent per-candidate scoring; complete manifests/deviations; safety-abort tests; null/negative findings; policy-approved thresholds fixed before results; and scoped certification decisions with expiry and rollback criteria. The race is removed when added success does not justify compute/review overhead, independence is not maintained, or selection depends on voting, appearance, or relative preference.

Draft PR #1's golden traces, cassette adapter, adversarial tests, and reported validation may seed fixtures or tooling only if independently accepted. They are neither controlled H8 results nor certification evidence, and this ADR does not ratify or authorize that PR.
