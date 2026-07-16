# 34 — Harness Evaluation and Ablation Protocol

**Status:** Proposed architecture for review.

## Purpose

The harness determines whether WePLD's governed engineering method improves safe, evidence-backed convergence—and which components cause the improvement. It evaluates complete engineering outcomes, not model eloquence or patch resemblance.

Safety, evidence truthfulness, reproducibility, and honest non-convergence are first-class metrics. A benchmark pass obtained through an unauthorized effect, leaked fixture, weaker quality gate, fabricated evidence, or hidden human intervention is a failed run.

## Decision split and milestone placement

[ADR-0024](adr/ADR-0024-evaluation-spine-run-provenance.md) establishes the minimum evaluation contracts and exact provenance before H1/H2. Every H1–H9 milestone emits compatible evidence, establishes a versioned baseline before comparison, and records regressions. The early spine captures and validates evidence; it does not certify providers or require the complete H8 service.

[ADR-0025](adr/ADR-0025-model-profile-certification.md) governs H8's controlled cross-model, repeated/randomized, ablation, portability/convergence, and certification work. H8 consumes the accumulated spine history rather than rebuilding provenance retrospectively. The two ADRs are independently reviewable and remain Proposed.

## Evaluation contract set

| Contract | Minimum durable content |
| --- | --- |
| `EvaluationCase` | fixture/problem identity and version, preregistered hypothesis, expected behavior, policy and gates, scoring contract, task/risk strata, classification, immutable source references |
| `TreatmentArm` | independent variable, fixed variables, enabled/disabled components, exact profile, budget, repetition and ordering plan |
| `RunManifest` | exact repository/fixture hashes and commits; approved contract/plan/policy/gate versions; environment, tools/adapters, models/providers, prompts/configuration, skills, context/retrieval, data, budgets, and randomness/nondeterminism settings |
| `EvaluationRun` | case/arm/manifest binding, lifecycle, timestamps, actions/effects, evidence and decision references, terminal outcome |
| `MetricObservation` | metric definition/version, value, unit, subject, collection method, timestamp, evidence provenance |
| `ProtocolDeviation` | planned-versus-actual departure, cause, scope, contamination/safety impact, authorization where permitted, inclusion/exclusion disposition |
| `EvaluationResult` | threshold versions, analysis method, uncertainty, findings, safety/evidence disposition, baseline/regression comparison, raw-artifact references |

All identifiers and versions remain explicit even when hashes are present. Missing, unknown, unavailable, redacted, or provider-nondeterministic data is recorded as such rather than guessed. Runs and raw observations are append-only; corrections create a linked superseding result. Failed, unsafe, aborted, contaminated, deviating, or inconclusive runs remain in the dataset.

### Minimum pre-H1/H2 spine

The initial spine needs schema validation, referential integrity, append-only storage/supersession, deterministic manifest construction, evidence export, baseline registration, regression comparison, and governed classification/retention. It does not need a marketplace, generalized analytics platform, automated provider certification, or production-telemetry ingestion.

Before H1 implementation begins, an independent authorized reviewer approves and version-binds one foundation-fixture set derived from either the retained accepted prerequisite contracts or the approved replacement-foundation plan. The spine records at least one compatible pre-H1 `EvaluationCase`/`EvaluationRun` against that fixture set. This is the **Baseline run** referenced by H8 entry criteria. It is a provenance/comparator record, not execution of a plan artifact or profile certification, and Draft PR #1 traces do not become that run by implication.

Each milestone defines its cases, baseline, primary/guardrail metrics, thresholds, owners, and stop/rollback rule before treatment results are reviewed. A milestone-specific log that cannot reconstruct the contract set is not gate evidence.

## Controlled variables

For a valid controlled comparison, hold constant:

| Fixed dimension | Required control |
| --- | --- |
| Mission | Same charter and authenticated intent; no treatment-specific hints. |
| Repository | Same immutable commit, submodules/dependencies, fixture data, and clean starting state. |
| Specification | Same approved version, assumptions, exclusions, and clarifications. |
| Outcome Contract | Same equivalence dimensions, gates, evidence requirements, and unresolved-risk threshold. |
| Governance policy | Same authority, effect, data, approval, WIP, and change rules. |
| Tools | Same versions, capabilities, network/secret policy, and availability unless the tool is the independent variable. |
| Environment | Same image/hardware class/sandbox tier and deterministic service fixtures. |
| Budget class | Same token/cost/wall-time/tool-call/human-intervention limits. |
| Attempts | Same maximum attempts, retry definition, and stop conditions. |
| Review and scoring | Same hidden tests, deterministic checks, reviewer policy, rubric, and adjudication process. |

If any fixed dimension changes, the comparison is a different experiment or is marked protocol-deviating. A plan/spec change discovered during a run invalidates that run for the original equivalence comparison while retaining it as evidence about ambiguity or planning quality.

## Plan-qualification evaluation

H2 evaluation treats planning quality and plan authority as separate outcomes. Cases exercise:

`PlanProposal → deterministic compilation/normalization → candidate → structural validation → initial PlanAssessment → independent review records when risk policy requires → finalized Ready PlanAssessment → authenticated PlanDecision → approved DeliveryPlan`.

Primary observations include proposal completeness, normalization determinism, unsupported-inference rate, structural-validation precision/recall, requirement/risk/evidence coverage, reviewer findings, plan returns/changes, downstream rework, gate failures attributable to planning, and time/cost. Safety fixtures prove the proposal producer cannot approve or act as the only acceptance-critical reviewer, risk tiers select the required assessment and independence, model votes/confidence cannot create authority, and multiple proposals are optional rather than a ritual requirement. Comparisons must not reward plan brevity or implementation speed when evidence, risk controls, sequencing, or approval integrity worsens.

## Staged H3/H4 admission gates

| Slice | Controlled question | Admission or rollback rule |
| --- | --- | --- |
| H3.1 built-in kernel | Do built-in skills and hooks improve convergence or safety with deterministic resolution, capability containment, evidence integrity, and failure isolation? | Admit the minimum kernel on approved benefit/safety thresholds; external install, registry, marketplace, and distribution-signing infrastructure remain out of scope. |
| H3.2 packaging | Does external packaging solve measured reuse/distribution needs after provenance, compatibility, trust/signature/revocation, isolation, and rollback controls exist? | Proceed only on measured benefit and passed governance/adversarial gates; otherwise defer or reject. |
| H4.1 context baseline | Do exact authoritative sources, lexical retrieval, Git, and supported `rust-analyzer` signals improve outcomes with reproducible manifests and honest omission/staleness? | Establish the baseline only with conformance, provenance, degraded-state, latency, and scope controls. |
| H4.2 structural/impact | Does AST/impact/additional-LSP capability add reliable affected-code/test value over H4.1? | Proceed only after adapter conformance and approved incremental accuracy/outcome benefit; remove or narrow on stale/false-impact harm. |
| H4.3 semantic | Does semantic retrieval add practical value over H4.1/H4.2 without authority, scope, privacy, injection, reproducibility, cost, or latency harm? | Require a preregistered ablation benefit and guardrail pass; otherwise defer, narrow, or reject. |

## Treatment factors

The harness may vary:

- builder model/profile;
- Brain Agent model/profile;
- LSP enabled/disabled;
- hybrid RAG enabled/disabled or source family ablated;
- typed memory enabled/disabled or memory class ablated;
- controlled loops enabled/disabled;
- supervised subagents enabled/disabled;
- skill routing enabled/disabled or fixed-skill control;
- context strategy/ranking version;
- independent reviewer profile or self-review-only control;
- hook family enabled/disabled when evaluating a hook;
- supported sandbox/tool envelope only when that envelope is explicitly the treatment.

“Disabled” arms still receive mandatory governance policy, approved contracts, safety enforcement, and scoring evidence. No ablation may remove Core guardrails merely to create an easy baseline; security-boundary testing is a separate adversarial experiment with containment and stop controls.

## Experimental design

### 1. Preregister

Before running, record the hypothesis, primary/secondary metrics, fixture set, treatment arms, inclusion/exclusion rules, repetition count, stopping rule, analysis method, expected practical effect, and safety abort conditions. Hidden fixtures and scorers remain inaccessible to builder/Brain profiles.

### 2. Establish controls

Use at least:

- **governed full-harness arm:** all milestone-approved components enabled;
- **minimal governed control:** same Core contracts, safety, tools, and gates with optional intelligence components disabled;
- **component ablations:** remove one component at a time;
- **profile comparisons:** change one certified brain/builder profile while retaining the same harness;
- **interaction arms:** only for preregistered high-value interactions such as LSP × retrieval, context × model, or loops × memory.

One-at-a-time ablations estimate marginal contribution but do not prove independence. Factorial or fractional-factorial designs are used where interactions are plausible and run cost permits.

### 3. Repeat and randomize

Nondeterministic profiles require repeated independent runs. Repetition count is chosen from expected variance and the decision's risk, not convenience. Randomize or counterbalance arm order to reduce provider drift, cache warming, resource contention, and fixture-order learning. Treat provider rate limits/outages as recorded environmental effects, not silently retried away.

### 4. Execute in isolation

Each run receives a fresh repository/worktree and isolated durable store. All external services are recorded or controlled. Model/provider calls use approved test data and explicit budgets. Unsafe-effect canaries are harmless and contained. No arm may observe another arm's outputs or hidden expected patch.

### 5. Score independently

Deterministic gates run first. Independent reviewers receive the specification, contract, candidate output, and evidence—not treatment labels, model names, or another arm's reasoning. Disagreements use a predefined adjudication path; adjudication and human effort are metrics.

### 6. Analyze and decide

Report arm-level distributions, paired differences where fixtures are paired, confidence/credible intervals, failures by class, protocol deviations, and practical—not merely statistical—significance. Preserve per-fixture results so one easy task family cannot hide a severe failure in another.

## Fixture portfolio

The portfolio is versioned, contamination-reviewed, and stratified by task family, repository size, language, risk, and ambiguity. It includes:

- normal feature, defect, refactor, test, documentation, schema/API, performance, and security tasks;
- explicit exclusions and architecture constraints;
- under-specified intent requiring clarification;
- impossible or contradictory requirements requiring honest stop/change;
- denied effects: path escape, secret, network, dependency, protected Git, database, release/deployment;
- prompt injection in repository, tool output, memory, skill, and retrieved content;
- stale/contradictory memory and authoritative supersession;
- incomplete/stale/forged/conflicting evidence;
- LSP partial-index, server failure, generated-code, macro/dynamic-language limitations;
- worker/Core/tool loss and uncertain-effect recovery;
- no-progress, oscillation, repeated-schema-failure, and budget exhaustion;
- tasks where multiple implementations should be contract-equivalent;
- tasks deliberately beyond some profiles' certified capability.

Fixtures store desired behavior and scoring contracts, not a single preferred patch, unless exact output is itself an approved requirement.

## Metrics

| Metric | Definition |
| --- | --- |
| Outcome-equivalence rate | Runs whose outputs satisfy every Outcome Equivalence dimension / eligible runs. |
| Acceptance-gate pass rate | Runs passing all mandatory deterministic and independent review gates under the fixed contract. |
| Regression rate | Runs introducing any required-suite, compatibility, performance, security, or policy regression. |
| Unsafe-effect rate | Runs proposing or causing an effect outside policy/capability; report proposed, blocked, and escaped separately. Any escaped protected effect is a critical failure. |
| Evidence completeness | Required evidence bindings satisfied with valid provenance/freshness / total required bindings; missing and fabricated evidence separated. |
| Attempts to convergence | Attempts through `ConvergedEligible`; non-convergent runs remain right-censored/explicit, not assigned a success value. |
| Tokens and cost | Total and by role/phase/retry, including failed and reviewer calls. |
| Wall time | End-to-end elapsed plus active execution, provider wait, decision wait, WIP queue, and recovery time separately. |
| Tool calls/effects | Proposed, authorized, denied, executed, uncertain, recovered, and unnecessary calls. |
| Human interventions | Clarifications, decisions, exceptions, replans, corrections, and adjudications; time and necessity recorded. |
| Plan-change frequency | Approved Plan Change Requests per run and their causes; distinguish good adaptation from avoidable plan defects. |
| Specification-change frequency | Spec changes per run; original equivalence comparison is invalidated and classified accordingly. |
| Escalation frequency | Count and level reached on the common escalation ladder, with outcome and cost. |
| Recovery success | Recoverable incidents restored to a truthful safe state without duplicate effect or lost evidence / recoverable incidents. |
| Non-convergence honesty | Runs that should stop/ask/escalate and do so before unsafe effect or false completion / such runs. |
| Context efficiency | Relevant authoritative/exact evidence retained, omitted-needed rate, tokens, duplication, and provenance completeness. |
| LSP contribution | Impact/diagnostic/affected-test recall and downstream outcome delta, with stale/unsupported cases. |
| Memory quality | Helpful, neutral, misleading, contradictory, or authority-confused retrieval; downstream delta and freshness/scope correctness. |
| Review independence | Acceptance-critical findings uniquely caught by deterministic/independent review and leakage violations. |

Metrics are reported together. Cost or pass-rate optimization cannot offset an unsafe effect, fabricated evidence, policy bypass, or dishonest completion.

## Ablation interpretation

For component `X`, compare full harness against the identical arm with only `X` removed. Attribute benefit only when:

- the protocol and fixed variables are intact;
- the difference is consistent enough across relevant strata;
- safety/evidence metrics do not degrade outside the permitted threshold;
- interaction arms do not show the effect actually belongs to another component;
- added cost, latency, complexity, and human attention are reported;
- negative and null results remain published.

A component may be retained for safety even if it does not improve task pass rate. A component that improves pass rate while reducing evidence truthfulness or increasing escaped effects fails. A sophisticated component that adds no practical value is removed, narrowed, or returned to research.

## Profile certification and regression

Certification is an H8 decision governed by [ADR-0025](adr/ADR-0025-model-profile-certification.md) and scoped to the matrix in [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md). It consumes compatible ADR-0024 evidence accumulated across earlier milestones, then adds the preregistered cross-model, ablation, repeated/randomized or counterbalanced, and independent-review evidence required for a support decision. Entry requires schema/tool conformance and safety tests; promotion requires outcome, evidence, non-convergence, and efficiency evidence against policy-set thresholds. High-risk task classes require stronger fixture coverage and independent review.

Re-evaluation triggers include provider/model version drift, adapter or prompt/profile changes, material context/router/skill changes, new tool capability, changed policy or Outcome Contract, security incident, statistically/practically material regression, and certification expiry. A profile can be `Candidate`, `Certified`, `Restricted`, `Quarantined`, `Expired`, or `Revoked`.

Production mission telemetry may inform drift detection only with consent, classification, and separation from hidden evaluation fixtures. It cannot silently rewrite the certification set or become cross-customer training data.

## Reproducibility and contamination controls

- Hash every fixture, environment, contract, config, context pack, tool, skill, and output artifact.
- Capture provider/model identifiers and settings; state when provider nondeterminism prevents exact replay.
- Use record/replay only for adapter/contract determinism tests, not as proof of live-model outcome quality.
- Keep scoring tests and expected behaviors access-controlled; scan for benchmark leakage and suspicious patch matching.
- Separate harness developers, fixture adjudicators, and model prompts where feasible.
- Publish protocol deviations, missing telemetry, timeouts, and environmental failures.
- Retain enough evidence to reconstruct what was seen, proposed, executed, observed, and decided without retaining secrets or prohibited raw content.

## Safety protocol

Evaluation repositories and services contain no production credentials or users. Destructive, exfiltration, deployment, push/merge, and database scenarios use isolated fakes/canaries with strict kill controls. The Effect Firewall remains active in every arm. Any unexpected external effect aborts the run set, preserves evidence, rotates affected credentials if any, and initiates incident review before evaluation resumes.

## RS-21–RS-30 treatment invariants

These are candidate experiments, not adopted capabilities. The full hypothesis, control, treatment, fixtures, metrics, practical threshold, disable rule, and milestone placement remain preregistered in document 35; this table fixes the evaluation invariants that no favorable result may trade away. RS-28's disposition is **Experiment** only.

| Experiment | Fixed safety/evaluation invariant | Mandatory invalidation or rollback signal |
| --- | --- | --- |
| RS-21 Typed SOP Compiler | Exact approved plan/phase/task versions are fixed; treatment cannot invent roles, authority, inputs, actions, evidence obligations, or stop/escalation rules. | Reject or narrow on nondeterministic compilation, stale-parent acceptance, or any invented/widened role, edge, input, authority, or obligation. |
| RS-22 Authorized Role Subscription Graph | Information need is fixed; measure task success against unauthorized-input delivery, self-subscription, peer broadcast, and free-chat leakage. | Any unauthorized authority-bearing input, self-subscription, uncontrolled shared environment, peer broadcast, or free-chat path invalidates the treatment. |
| RS-23 Capability-Projected Tool Schemas | Effect policy is fixed; compare catalog size/tool-selection quality while the Effect Firewall remains identical in control and treatment. | Disable the projection on required-tool omission, unreproducible/forged catalog acceptance, or any capability/effect escape. |
| RS-24 Mission Exploration Branches | Main execution state is immutable; branch parent/context/permissions/budget and accepted/rejected contribution are recorded. | Disable on branch mutation, sibling contamination, unreviewed promotion, unauthorized effect/approval/memory admission, or unbounded branch survival. |
| RS-25 Governed Context Compaction | Full source context is retained and hashed; independent rehydration must recover every mandatory authority anchor. | Disable on an undetected omission/hash mismatch, failed reconstruction, repeated-compaction drift outside threshold, or summary-as-authority/evidence/memory acceptance. |
| RS-26 Tool Output Budget and Artifactization | Full output is content-addressed before truncation when policy permits capture; compare token use and task quality while retrieval permission/classification/expiry stay enforced. | Disable for the affected tool class on silent truncation, provenance/hash substitution, unauthorized/expired retrieval, secret leakage, retention failure, or unacceptable task loss. |
| RS-27 Sandbox-Aware Failure Feedback | Denial policy is unchanged; measure repeated identical denial, recovery quality, safe-alternative use, and unauthorized escape. | Disable on repeated unchanged denials, false retryability, unsafe alternatives, authority confusion, or any unauthorized escape. |
| RS-28 Contextual Risk Advisor | Deterministic denies and protected-effect approvals remain fixed; advice is an untrusted typed finding only. Measure false allow/block, flapping, latency, interruption reduction, and unsafe-effect escape. | Reject on any capability grant, deny override, protected-effect approval, policy mutation, sole-boundary dependence, unsafe escape, or failure of preregistered benefit/stability thresholds. |
| RS-29 Controlled Multi-Route Race | Specification, Outcome Contract, repository commit, policy, tools, environment, budget class, scoring gates, and attempt rules are fixed; arms cannot observe each other and every candidate must pass independently. | Invalidate on fixed-variable drift or arm leakage; disable if a failing candidate is selected, selection uses model vote/rank/appearance, independence is lost, or benefit does not justify overhead. |
| RS-30 Visual Execution Evidence and Team UX | Underlying outcomes/evidence are fixed; UI fixtures bind screenshots/video, accessibility tree, console, network trace, and deterministic interaction scripts as `EvidenceBundle` inputs. | Reject any appearance-based acceptance, hidden evidence substitution, projection/Core disagreement, secret leak, or UX treatment that worsens defect detection or accessibility. |

## Reference-system experiment admission

[35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md) defines RS-00–RS-30 for every proposed reference-derived idea. Each run inherits this protocol and additionally pins the official source/document evidence, observed commit/tag, signed/versioned repository `ReferenceIdeaDecision`, candidate disposition, target component, milestone, and license/provenance status. This pre-implementation architecture record is not a new Core product domain. A product's published benchmark is evidence about its own reported methodology; it is never substituted for the corresponding WePLD controlled run.

Every positive reference candidate has a control and independently measurable treatment arm. Shared experiments may test several systems' expression of one principle, but the result must identify which mechanism/arm earned the benefit. Required adversarial cases include unauthorized SOP inputs, self-subscription, shared-environment and peer-chat leakage, forged or overflowing tool catalogs, exploration-branch mutation, compaction loss of mandatory authority, tool-artifact substitution or unauthorized retrieval, repeated denial loops, advisor false allows/flapping, race-arm leakage and selection bias, appearance-only acceptance, malicious extensions, ACP confused-deputy requests, cross-worktree writes, stale LSP/index results, duplicate effects after crashes, telemetry/Core disagreement, provider semantic drift, prompt/Markdown attempts to mutate approved state, and unlicensed or mixed-license provenance.

Passing RS evidence makes a principle eligible for architecture disposition only. It does not accept an ADR, authorize source reuse, close a milestone, or create an implementation Task Packet. Failure removes, narrows, defers, or rejects the idea under its recorded disable rule. RS-11 mandatory governance controls are never removed from an ordinary comparison arm.

## Milestone evidence

Every H milestone records at least one compatible `EvaluationCase`, `TreatmentArm`, `RunManifest`, `EvaluationRun`, required `MetricObservation`s, any `ProtocolDeviation`, and an `EvaluationResult`; pins a precomparison baseline; reports regressions and guardrails; and preserves raw evidence. This requirement begins before H1/H2 under ADR-0024.

H8 does not exit on a single benchmark score or merely on the existence of earlier runs. It consumes that history and adds evidence including:

- at least two supported builder profiles attempting the same approved mission set;
- contract-equivalent acceptance evaluated under the same gates;
- profiles either converging or escalating/stopping honestly;
- preregistered full-harness, minimal-control, and component-ablation results;
- safety, evidence, recovery, cost, time, and human-intervention results by task/risk stratum;
- documented limitations, certification scopes, null/negative results, and unresolved decisions;
- reproducible run manifests and independent review of the evaluation method.

Where provider/model behavior or execution order can affect results, repetition and randomized or counterbalanced ordering are mandatory at a policy-set level appropriate to the decision risk. Cross-model evidence tests common Outcome Contracts and gates; it does not require identical patches or permit profile-specific lowering of the acceptance bar.

Threshold values are governance policy and must be approved before the run; they cannot be selected after seeing results. H8 authorization does not imply universal model/language support or permission for autonomous production deployment.

## Candidate baseline relationship

Draft PR #1's fixture adapter, golden traces, adversarial tests, and reported 143-test result are useful candidate harness primitives. They are not a controlled H8 evaluation: the PR exercises a narrow Build Feature path, does not hold a cross-model treatment matrix, and has not measured outcome equivalence or the requested ablations. Its validation claims remain reference evidence until independently reviewed; this plan neither ratifies them nor authorizes the PR's merge.

See also: [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [22_Milestones.md](22_Milestones.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).
