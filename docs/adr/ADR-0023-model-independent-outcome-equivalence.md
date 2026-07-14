# ADR-0023 — Accept contract-equivalent outcomes, not model-identical implementations

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H8 implementation authorization

## Context

Provider replaceability alone does not define what must remain invariant across models. Byte-identical output is unrealistic, while model-specific acceptance bars would make “support” unsafe and impossible to compare honestly.

## Decision

Adopt [33_Model_Independent_Outcome_Convergence.md](../33_Model_Independent_Outcome_Convergence.md). The approved OutcomeContract fixes functional behavior, acceptance criteria, public contracts, architecture, security and policy, quality gates, regression behavior, evidence completeness, scope/change control, and residual-risk threshold.

Internal strategies, attempt counts, model/tool sequences, formatting, and non-contractual style may vary. The final quality and evidence bar never changes with profile. A profile that cannot converge follows the common escalation ladder and stops honestly as `NonConvergent` rather than lowering gates or fabricating completion.

## Reason

Contract-relative equivalence is the engineering truth users need from replaceable models. It permits implementation and cost choice while keeping acceptance, security, and evidence invariant.

## Benefits

- Meaningful model and provider portability.
- Stable, explicit acceptance semantics.
- Measurable profile capability and honest weak/strong differences.
- No preferred-patch or byte-identity benchmark bias.

## Trade-offs

- Outcome contracts and fixtures must be precise.
- Some equivalence judgments require independent human review.
- Not every profile can be certified for every task or risk class.

## Migration

H8 evidence must show at least two supported builder profiles attempting the same approved mission set under identical gates; accepted outputs that are contract-equivalent; unchanged thresholds; documented efficiency and escalation differences; and honest non-convergence on out-of-scope capability cases.

Draft PR #1's provider-neutral gateway, acceptance criteria, gates, staged completion, and fixture replay are candidate inputs. A narrow single-profile Build Feature trace is not evidence of outcome equivalence or certification, and this ADR does not ratify the PR.
