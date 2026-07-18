# 37 — Committee Evaluation Protocol

**Standing:** Proposed, falsifiable admission protocol for the
[Engineering Committee](36_Engineering_Committee.md). This document defines how
the Committee earns — or fails to earn — a place in the product. **No claim
that Committees improve outcomes is made or implied before these experiments
prove it.**

## Method and provenance

All experiments run on the evaluation spine of
[ADR-0024](adr/ADR-0024-evaluation-spine-run-provenance.md) and follow the
controlled-protocol discipline of
[34 — Harness Evaluation Protocol](34_Harness_Evaluation_Protocol.md):
preregistered cases and arms, frozen `RunManifest`s, fixed subject artifacts
(specification, Outcome Contract, plan version, diff, repository commit),
recorded deviations, and blind independent scoring. Review subjects combine
seeded-defect corpora (planted, ground-truth-known defects across
architecture, implementation, security, and verification classes) with
historical review subjects whose accepted resolutions are known. Committee
runs record `CommitteeEvaluationResult` artifacts; longitudinal member and
preset behavior accumulates in `CommitteePerformanceRecord`s.

## Compared configurations

Each arm reviews the identical frozen subject. Every arm's actual spend is
recorded and reported (see budget normalization below).

| Arm | Configuration | Distinct purpose |
| --- | --- | --- |
| EC-A1 | single Mastermind review (baseline) | the cost/recall floor every structure must beat |
| EC-A2 | three independent models, no cross-review, mechanically merged findings | measures what independence alone contributes |
| EC-A3 | Committee with one bounded challenge round, no synthesis | proves whether bounded cross-review adds value over independent opinions alone |
| EC-A4 | Committee without minority preservation (evaluation-only ablation) | measures the cost of dropped dissent; **must never become a product mode** |
| EC-A5 | Committee with Wisdom synthesis | proves whether Wisdom synthesis adds value **without suppressing disagreement** (dissent-survival is scored) |
| EC-A6 | Committee plus deterministic evidence (gates/tests in the pack) | proves whether deterministic evidence materially improves finding quality, evidence linkage, or unsupported-claim rate |
| EC-A7 | full Committee protocol across same-family models | isolates the family-correlation penalty |
| EC-A8 | full Committee protocol across diverse providers/profiles | isolates cross-provider benefit; supports diversity claims only here |

### Arm output mechanisms

No two arms are compared without stating exactly how each produces its final
finding set:

| Arm | Final finding set produced by | Challenge | Synthesis | Minority preserved | Deterministic evidence in pack |
| --- | --- | --- | --- | --- | --- |
| EC-A1 | the single reviewer's own report | no | no | n/a (one voice) | no |
| EC-A2 | deterministic Core merge (union, duplicate-collapsed, per-member attribution kept) | no | no | yes — opinions remain separate | no |
| EC-A3 | deterministic Core merge of post-challenge positions | one round | no | yes | no |
| EC-A4 | Chair synthesis with dissent deliberately dropped | one round | yes | **no (ablation)** | no |
| EC-A5 | Chair (`Wisdom`) synthesis | one round | yes | yes — verbatim | no |
| EC-A6 | Chair synthesis | one round | yes | yes — verbatim | yes |
| EC-A7 | Chair synthesis | one round | yes | yes — verbatim | yes |
| EC-A8 | Chair synthesis | one round | yes | yes — verbatim | yes |

### Budget normalization

Every arm runs under the same hard session token/request budget class;
structurally required extra stages (challenge, synthesis) spend from that same
session budget rather than receiving more. Arms that structurally cannot spend
the full budget (EC-A1) record their lower actual spend. Metrics are reported
both absolute and cost-normalized (per token and per unit cost), and **no
cross-arm comparison may present two arms as budget-identical without the
recorded actual spends** — structural budget differences are part of the
result, not noise to hide.

### Identity and lineage constraints on evaluation

Every member response in every run records `ModelIdentityEvidence`
([36 §5](36_Engineering_Committee.md)). Runs whose members fall below the
policy-required identity-assurance tier are recorded as such and flagged in
assessment. EC-A7/EC-A8 diversity conclusions may rest only on lineage whose
`LineageEvidence` status is `Known` or `IndependentlyDocumented`
(`ProviderClaimed` demands an explicit caveat); a member with `Unknown`
lineage cannot support a diversity claim in either arm.

## Metrics

| Metric | Reading |
| --- | --- |
| material defect detection | recall against seeded/known defects |
| unsupported-claim rate | findings with no resolvable evidence reference |
| architecture-risk detection | recall on seeded architecture-class defects |
| security-finding recall | recall on seeded security-class defects |
| false-positive findings | findings scored invalid by blind review |
| plan-quality score | blind rubric score of any resulting `PlanChangeProposal` |
| human corrections | corrections required after review |
| reviewer disagreement | measured divergence between members |
| minority finding value | share of minority findings later validated |
| cost | total per arm, per accepted finding |
| time | wall time to disposition |
| tokens | total and per member |
| user intervention | prompts/decisions demanded of the user |
| plan churn | plan versions created downstream per subject |
| evidence linkage | share of findings with resolvable references |
| duplicated findings | redundancy across members |
| non-convergence | honest `NonConvergent` rate |
| provider failure tolerance | disposition quality under injected member failure |

## Independence and imitation checks

First-round opinions are compared for textual and structural similarity
against challenge-round positions. Convergence achieved by imitation
(positions collapsing toward the first-published opinion without new
evidence) is a failure signal, not a success signal — EC-A2 versus EC-A3
isolates exactly this effect.

## Rejection criteria

The Committee feature is rejected, descoped, or redesigned if experiments
show any of:

- cost exceeds measured review value (cost per validated finding worse than
  EC-A1 with no offsetting recall gain);
- deliberation adds no measurable defect detection over EC-A2;
- members converge through imitation rather than evidence;
- minority reports add no useful information (EC-A4 ≈ EC-A3/EC-A5 on all
  recall metrics);
- plan churn increases without quality improvement;
- user decision burden increases beyond the preset's stated budget;
- privacy risk becomes unacceptable under the data-egress model of
  [36 §8](36_Engineering_Committee.md).

Rejection is recorded as durable evidence, exactly like admission.

## Admission rule

The proposed V0 includes independent opinions, one challenge round, Wisdom
synthesis, and evidence-linked findings — so the admission gate must evaluate
**all** of those components, not a subset.

Committee V0 ([36 §16](36_Engineering_Committee.md)) may be authorized only
after **EC-A1, EC-A2, EC-A3, EC-A5, and EC-A6** all have terminal, assessed
results, and:

- EC-A3 demonstrates whether bounded cross-review adds value over independent
  opinions alone (against EC-A2);
- EC-A5 demonstrates whether Wisdom synthesis adds value without suppressing
  disagreement (against EC-A3, with dissent-survival explicitly scored);
- EC-A6 demonstrates whether supplying deterministic evidence materially
  improves finding quality, evidence linkage, or unsupported-claim rate
  (against EC-A5);
- no rejection criterion has fired.

EC-A4 remains an evaluation-only ablation and must never become a product
mode. **EC-A7 and EC-A8 must be completed before**: diversity-based automatic
routing; any claim of cross-provider advantage; or admission of Deep Committee
or the Critical Review Board wherever diverse membership is part of the
product claim. The normal roadmap admission rule and gate discipline of
[22 — Milestones](22_Milestones.md) apply unchanged.
