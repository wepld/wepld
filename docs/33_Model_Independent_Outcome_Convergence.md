# 33 — Model-Independent Outcome Convergence

**Status:** Proposed architecture for review.

## Strategic promise

> **Different brains. Same engineering truth.**

WePLD does not promise byte-identical code, identical reasoning, equal capability, or successful convergence from every model. It promises model-independent engineering acceptance for supported profiles:

- each profile works against the same approved Engineering Specification and Outcome Contract;
- each profile is constrained by the same policy, architecture, security, quality, and evidence gates;
- accepted outputs are contract-equivalent even when implementation paths differ;
- selecting a weaker, cheaper, faster, local, or specialized model never lowers the final quality bar;
- a profile that cannot converge within its envelope stops safely and escalates honestly.

Supported models may differ in cost, latency, attempts, context needs, tool usage, escalation frequency, and the difficulty they can handle without assistance. Those differences are measured; they are not hidden behind a claim of equal capability.

## Outcome Equivalence Contract

An approved `OutcomeContract` fixes the comparison version and defines the observable acceptance boundary before implementation begins. Two candidate outputs are contract-equivalent only when both independently satisfy every applicable dimension below.

| Dimension | Required equivalence evidence |
| --- | --- |
| Functional behavior | All bound acceptance scenarios and specified edge cases pass on the same fixture/environment class. |
| Acceptance criteria | Every criterion maps to current, validated Evidence Requirements and Evidence Bundles; no criterion is waived by routing policy. |
| Public contracts | API/schema/protocol/CLI/data behavior and compatibility rules match the approved contract; allowed additive variation is explicit. |
| Architecture constraints | Module/dependency/ownership/data-flow constraints and accepted ADRs pass deterministic or reviewed conformance checks. |
| Security and policy | Identical mandatory controls, forbidden effects, data handling, findings threshold, and exception authority apply. |
| Quality gates | Build/static/test/review/documentation/accessibility/performance/compliance gates required by risk all pass at the same thresholds. |
| Regression behavior | The same required regression suite and approved performance/resource budgets pass; no model-specific baseline is substituted. |
| Evidence completeness | Evidence covers every required binding with provenance, tool/environment identity, freshness, and reproducibility. |
| Scope and change control | Changed artifacts and effects remain within the same approved scope or have the same authorized Change Request lineage. |
| Residual risk | Unresolved risk is at or below the same approved threshold, with the same authority required for exceptions. |

Equivalence is **contract-relative**, not subjective similarity. If the contract fails to distinguish an important behavior, the remedy is a Specification Change Request and a new contract version—not reviewer intuition applied after seeing an implementation.

## Permitted non-identity

The following may differ unless the approved contract constrains them:

- internal algorithm or implementation strategy;
- file organization inside permitted architecture boundaries;
- code formatting beyond repository rules;
- non-public naming and implementation style;
- number of attempts, loop iterations, subagents, or tool calls;
- brain/builder profile and routing sequence;
- context-pack composition beyond mandatory authority and evidence requirements;
- cost and wall time within the mission budget;
- non-contractual comments or explanatory phrasing.

Variation never permits a weaker security control, missing evidence, undocumented public behavior, unauthorized dependency, out-of-scope effect, or reduced regression coverage.

## Acceptance algorithm

Core evaluates one candidate at a time; it does not choose a winner by model vote.

1. Pin policy, specification, outcome, plan, toolchain/environment, gate, and evidence-requirement versions.
2. Validate source/effect lineage and confirm no unapproved higher-layer change occurred.
3. Evaluate deterministic functional, contract, policy, architecture, security, quality, regression, and budget gates.
4. Validate Evidence Bundle provenance, freshness, independence, and complete criterion coverage.
5. Require independent review stages proportional to risk; conflicting findings remain unresolved until dispositioned by the proper authority.
6. Compare unresolved risk to the fixed threshold.
7. If eligible, produce a Completion Proposal. Only an authorized Completion Decision accepts, returns, defers, or cancels.

The same algorithm applies regardless of provider or profile. A profile name is never an input to a lower acceptance threshold.

## Convergence states

| State | Meaning | Required behavior |
| --- | --- | --- |
| `ConvergedEligible` | All machine and review gates pass; evidence and risk are complete enough for completion review. | Produce a Completion Proposal; do not auto-accept. |
| `NotYetConverged` | A bounded, actionable gap remains and budget/plan permit another materially different attempt. | Record gap and changed hypothesis; continue under the controlled loop. |
| `NeedsReplan` | Evidence invalidates delivery strategy but not WHAT. | Propose a Plan Change Request; pause affected descendants. |
| `NeedsSpecificationChange` | Evidence shows WHAT is ambiguous, contradictory, infeasible, or wrong. | Propose a Specification Change Request; no implementation workaround may redefine it. |
| `NeedsHumanDecision` | Authority, ambiguity, exception, or protected effect cannot be delegated. | Issue a Decision Request with evidence and consequences. |
| `NonConvergent` | Attempts, budget, supported routes, or uncertainty limits are exhausted without meeting the fixed bar. | Stop safely and report honest non-convergence, partial artifacts, and unresolved gaps. |
| `UnsafeOrInvalid` | Policy violation, invalid plan, tainted environment, or unreliable evidence makes continuation unsafe. | Stop the affected work, preserve evidence, and initiate security/recovery handling. |

`NonConvergent` is a truthful outcome, not a hidden failure. The mission remains unresolved until an authorized user returns, defers, changes the contract, selects a newly certified profile, or cancels.

## Escalation ladder

Escalation changes the means, never the acceptance truth:

1. retry once with corrected or more complete context and a named changed hypothesis;
2. route to a more specialized certified skill;
3. split the Task Packet into smaller independently verifiable packets;
4. invoke a bounded reviewer/advisor subagent;
5. propose a Delivery/Phase Plan change;
6. switch to another supported brain or builder profile certified for the task/risk class;
7. request clarification;
8. request a human decision or exception from the named authority;
9. stop safely as non-convergent.

Core/Hermes may skip directly to a higher step when a lower step cannot address the gap or when policy requires immediate authority. Every step consumes budget and records cause, expected benefit, result, and evidence.

## Profile support and certification

A model is not globally “supported.” A versioned profile is certified for a bounded matrix:

`provider/model/adapter/settings + task families + languages/frameworks + risk classes + context/tool envelope + verification level + data classification + platform`.

Certification requires the controlled harness in [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), including normal, ambiguous, denied, adversarial, failure, recovery, and non-convergence cases. Required records include:

- schema and tool/action proposal reliability;
- outcome-equivalence and acceptance-gate pass rates;
- unsafe-effect and policy-violation rates;
- evidence completeness and citation/provenance correctness;
- regression, recovery, and non-convergence honesty;
- cost, tokens, wall time, attempts, context size, and human interventions;
- known limitations, certification scope, expiry, and revocation triggers.

Provider/model drift, adapter changes, material routing changes, or expired evidence return a profile to `Candidate` or `Quarantined`. Historical attempts retain their exact profile/version metadata. Certification authorizes routing consideration; it grants no tool capability or completion authority.

## Contract change and comparison discipline

Cross-model comparisons are valid only when mission, repository commit, approved specification, Outcome Contract, policy, tools, environment, budget class, and maximum attempts are held constant. If a run discovers a real specification defect, all affected comparisons are marked invalid for equivalence analysis until repeated against the new approved version.

No profile may receive undocumented hints, privileged expected patches, broader tools, or a weaker gate and still be compared as the same condition. Such a run is a different treatment arm and must be labeled accordingly.

## Security and honesty invariants

- A model's self-reported confidence, test result, or completion status is never acceptance evidence by itself.
- Majority agreement among models cannot override an authoritative source or failing deterministic gate.
- A semantic similarity score cannot establish functional or contract equivalence.
- A reviewer cannot waive a policy, evidence, or risk requirement outside its named authority.
- A cheaper model is not “successful” when it stops early by omitting work or evidence.
- Non-convergence honesty is measured and rewarded; unsafe persistence is a failure even if a later patch passes tests.
- Human intervention is recorded as part of the route, not erased from the model's result.

## Product reporting

Mission and evaluation reports distinguish:

- **Outcome status:** eligible, returned, deferred, cancelled, or non-convergent;
- **contract version and complete evidence matrix;**
- **route:** profiles, skills, subagents, retries, replans, and human interventions;
- **efficiency:** cost, tokens, time, tool calls, and WIP delay;
- **safety:** denied/unsafe proposals, exceptions, uncertainty, and recovery;
- **quality:** gates, regressions, findings, and unresolved risk.

This lets users choose profiles based on measured economics and capability without creating multiple definitions of “done.”

## Candidate baseline relationship

Draft PR #1 demonstrates candidate provider neutrality, fixture replay, explicit plan/completion decisions, evidence-gated proposal refs, and safe `Deferred` handling. It does not establish cross-model contract equivalence or certify profiles: its Build Feature path is narrow, provider support is local-loopback-only, review independence and generalized evidence bindings are incomplete, and its reported tests were not an H8 controlled evaluation. The PR may become a prerequisite baseline only after independent acceptance; this plan does not ratify it.

See also: [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [06_Brain_Architecture.md](06_Brain_Architecture.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).
