# 06 — Brain Architecture

## Terms and invariant

WePLD separates a reasoning role from the model that supplies reasoning:

- **Brain Agent:** the governed planner, architect, risk analyst, and replanner. It proposes structured artifacts and recommendations but never approves them, executes tools, mutates durable state, or communicates directly with a user.
- **Brain profile:** a versioned, policy-scoped configuration for a replaceable reasoning provider.
- **Builder profile/model:** a replaceable reasoning provider selected to implement one bounded TaskPacket; it proposes artifacts and typed actions but has no direct tool authority.
- **Brain Gateway:** the only component that knows provider-specific APIs and normalizes all model invocations.
- **Hermes:** the Engineering Intelligence Runtime that supervises planning/execution context, skills, loops, and subagents. Hermes is never a brain provider and never governance truth.

Core alone owns durable truth, policy, approvals, capabilities, budgets, transitions, completion, and recovery. The Gateway records invocation evidence through Core contracts; it does not acquire that authority.

## Logical architecture

~~~mermaid
flowchart LR
  Core["Core-issued governed request"] --> Role["Brain Agent or bounded builder role"]
  Role --> Request["Provider-neutral BrainRequest"]
  Request --> Gateway["Brain Gateway\nvalidate • route • budget • normalize"]
  Gateway --> Local["Approved local adapters"]
  Gateway --> Hosted["Approved external model adapters"]
  Local --> Result["Normalized BrainResult"]
  Hosted --> Result
  Result --> Role
  Gateway --> Evidence["Invocation evidence\nprofile • settings • usage • cost • latency • validation"]
  Evidence --> Core
~~~

Model names never appear in mission-domain authority rules. Native model tool calls are normalized into `ProposedAction` values and returned to Core; they are not executed by the provider, Gateway, Brain Agent, builder, or Hermes.

## Brain Agent planning and qualification contract

The Brain Agent receives only authorized, provenance-labelled inputs, including:

- the current approved EngineeringSpecification and OutcomeContract;
- repository map plus LSP symbols, dependencies, diagnostics, and affected-test evidence;
- applicable policy, ADRs, approved plan versions, and Git history;
- previous verified evidence and scoped Engineering Memory;
- available Hermes skills, supported profiles, tools, capabilities, budgets, and deadlines;
- current risks, assumptions, uncertainties, decisions, and phase state.

Its structured `PlanProposal` output includes:

- delivery strategy and tailored phase graph;
- task decomposition and requirement-to-phase/task/evidence traceability;
- dependencies, risks, mitigations, assumptions, and open decisions;
- required skills, model/role capabilities, tools, and verification level;
- writable and forbidden scope;
- phase entry/exit conditions, WIP, budgets, and evidence requirements;
- estimates, stop conditions, escalation conditions, and recovery considerations.

The output is never a `DeliveryPlan` and never carries approval authority. Plan qualification follows one explicit pipeline:

`Brain Agent → PlanProposal → deterministic compiler/normalization → candidate DeliveryPlan → structural validation → initial PlanAssessment → independent review when policy requires → finalized Ready PlanAssessment → authenticated PlanDecision → approved DeliveryPlan`

The deterministic compiler canonicalizes identifiers, versions, ordering, defaults, trace edges, budgets, and policy references without inventing strategy. Structural validation rejects schema errors, stale provenance, missing trace edges, dependency cycles, invalid scopes, budget/WIP violations, and policy-incompatible gates. A `PlanAssessment` then records:

- specification and Outcome Contract coverage;
- acceptance and evidence sufficiency;
- DAG validity and delivery ordering;
- architecture fitness and constraint compliance;
- proportionality to mission size and risk;
- security, operational, delivery, and residual risk;
- budget and WIP feasibility;
- rollback and recovery adequacy;
- assumptions, uncertainty, and material alternatives;
- reviewer identity, role, and independence;
- blockers and an explicit readiness recommendation.

Policy classifies the candidate before decision. The initial assessment becomes `ReviewRequired` when policy names independent reviews. Those reviewers create separate immutable records; Core then finalizes a new `Ready` assessment version bound to the exact policy/risk-tier version and every required review record ID/version/hash. A low-risk plan may move from deterministic validation to a `Ready` assessment and an authenticated decision by an authorized user without an independent-review set when policy explicitly permits it. Medium- and high-risk plans require independent architecture, quality, and security review in the combinations named by policy. The proposal producer cannot approve it or act as the sole acceptance-critical reviewer. Multiple models may surface disagreements or alternatives, but model voting is evidence only and never authority. Core alone validates the decision actor and records the immutable `PlanDecision` that creates an approved `DeliveryPlan` version.

WePLD does not require multiple plans by default. Alternative proposals are requested only when risk, uncertainty, material architectural choice, or a failed assessment justifies their cost. Evidence discovered during execution can trigger the same controlled qualification path for a replacement plan, not an implicit plan rewrite.

## Provider-neutral request and result

| `BrainRequest` field | Meaning |
| --- | --- |
| Intent and role | planning, clarification, implementation, review, diagnosis, classification, or synthesis under a named role |
| Governing references | exact policy, specification, contract, plan, phase, and TaskPacket versions applicable to the call |
| Context pack | immutable, ranked, provenance-labelled references selected by the Context Compiler |
| Output contract | versioned schema, acceptance constraints, required citations, and permitted proposed actions |
| Constraints | role policy, data classification, writable/forbidden scope, deadline, quality level, and prohibited behavior |
| Budget | tokens, spend, retries, latency, tool-call proposal limit, and allowed fallback sequence |
| Trace identity | mission, phase, task attempt, correlation, causation, and cancellation identifiers |

| `BrainResult` field | Meaning |
| --- | --- |
| Structured output | schema-valid proposal/artifact or an explicit validation failure |
| Evidence links | context/artifact references supporting material claims |
| Proposed actions | typed suggestions for Core evaluation, never executable effects |
| Invocation record | profile, provider/model identifier, adapter/settings version, usage, cost, latency, and warnings |
| Uncertainty | confidence, missing context, contradictions, refusal, degradation, and non-convergence reason |

Schema failure, missing citations, stale governing versions, or prohibited action types produce a failed result. Prompt text cannot waive these validations.

## Profiles, routing, and fallback

A brain or builder profile declares allowed adapters/models, measured capabilities, context limits, residency and data-classification restrictions, cost ceiling, latency target, retention policy, compatibility, evaluation status, and fallback sequence. Roles request capabilities and a policy-qualified profile, not a vendor name.

Hermes may recommend a route based on task type, required skill, risk, structured-output reliability, evaluation evidence, locality, latency, availability, cost, context capacity, and prior convergence. Core enforces allowed profiles and budgets. Fallback is permitted only when governance and output semantics remain unchanged; it cannot silently expand egress, scope, authority, budget, or retention.

Different profiles may use different implementation strategies and attempt counts. Acceptance remains fixed by the OutcomeContract, architecture/security policy, quality gates, regression behavior, evidence completeness, and unresolved-risk threshold. If a profile cannot converge, the system specializes context/skills, splits the task, seeks independent review, replans, switches an allowed profile, requests clarification/authority, or stops safely.

## Privacy and credential rules

Provider credentials live in an OS-backed secure store or approved enterprise secret provider, never in skills, context packs, prompts, logs, or plugin manifests. The Context Compiler minimizes, redacts, provenance-labels, and validates content before egress. Invocation records preserve identifiers, policy decisions, content hashes, usage, and warnings; retention of raw request/response content follows classification and explicit policy. An unavailable provider leaves visible degraded, blocked, or waiting state rather than fabricated progress.

## Evaluation and change management

Every supported profile is certified for named roles and task classes through controlled harness evaluations. Evaluation includes schema validity, outcome-equivalence rate, gate pass rate, regressions, unsafe-effect proposals, evidence completeness, attempts, tokens/cost, wall time, interventions, escalations, recovery, and non-convergence honesty.

Controlled comparisons hold mission, repository commit, approved specification, policy, OutcomeContract, tools, environment, budget class, and maximum attempts constant. An `EvaluationCase` expands into one or more `EvaluationRun` records with explicit `TreatmentArm`, immutable `RunManifest`, append-only `MetricObservation`, any `ProtocolDeviation`, and a derived `EvaluationResult`. Exact provenance binds fixture and source hashes, governing artifact versions, profile/provider/model and adapter versions, prompts/context manifests, skills/tools, environment, seed where supported, budgets, timestamps, and raw evidence references. Comparisons vary brain/builder profile and, where testing the harness, LSP, retrieval, memory, loops, subagents, and skill routing. Promotion, quarantine, or removal is a documented Core policy decision; provider regressions never require mission-domain changes.

The full protocol is defined in [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).

## Non-goals

- A universal prompt format that hides materially different provider behavior.
- Treating Hermes as a model/provider or letting a provider SDK define domain semantics.
- Trusting a model's report of an effect, test, diagnostic, or acceptance result without independent evidence.
- Letting the Brain Agent approve its own specification/plan or a builder redefine its packet.
- Claiming equal capability, byte-identical output, or guaranteed convergence for every model.

See also: [05_Worker_Architecture.md](05_Worker_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [16_Data_Model.md](16_Data_Model.md), [23_Technology_Evaluation.md](23_Technology_Evaluation.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), and [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md).
