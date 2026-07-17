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

Each arm reviews the identical frozen subject under identical budgets where
the arm's structure permits, and every arm's total cost is recorded.

| Arm | Configuration |
| --- | --- |
| EC-A1 | single Mastermind review (baseline) |
| EC-A2 | three independent models, no cross-review, mechanically merged findings |
| EC-A3 | Committee with one challenge round |
| EC-A4 | Committee without minority preservation (ablation; measures the cost of dropped dissent) |
| EC-A5 | Committee with Wisdom synthesis |
| EC-A6 | Committee plus deterministic evidence (gates/tests included in the pack) |
| EC-A7 | Committee across same-family models |
| EC-A8 | Committee across diverse providers/profiles |

EC-A4 exists to measure what minority preservation contributes; it is never a
product configuration.

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

Committee V0 ([36 §16](36_Engineering_Committee.md)) may be authorized only
after: EC-A1, EC-A2, and EC-A3 have terminal, assessed results; EC-A3 shows a
preregistered, material improvement on defect detection or risk detection at
acceptable cost; and no rejection criterion has fired. Wider presets
(Deep Committee, Critical Review Board) additionally require EC-A5–EC-A8
evidence. The normal roadmap admission rule and gate discipline of
[22 — Milestones](22_Milestones.md) apply unchanged.
