# 10 — Loop Engineering

## Two governed loop levels

WePLD does not perform one-pass AI coding. It separates two complementary loops:

1. **Outer delivery flow:** Core-governed phases and Kanban move an approved outcome from discovery/specification through implementation, verification, delivery, and completion decision.
2. **Inner Hermes engineering loop:** for one bounded TaskPacket, Hermes observes evidence, diagnoses, forms a hypothesis, selects the minimal permitted action, obtains Core authorization for any effect, verifies the actual result, updates belief, and decides whether to continue, replan, escalate, or stop.

The outer loop owns durable delivery state in Core. The inner loop holds bounded operational state in Hermes and emits durable iteration/evidence records. Neither Hermes nor a model may close a task, phase, or mission.

## Outer phase model

Phase is the primary delivery unit. A common graph is Discovery → Specification → Architecture and Contract Design → Implementation → Verification → Delivery, but the Brain Agent may propose a smaller, larger, parallel, or returned graph when policy and dependencies permit. It cannot approve the graph.

Every approved `PhasePlan` declares:

- objective, governing artifact versions, dependencies, and entry/exit conditions;
- allowed skills, profiles, tools, capabilities, and writable/forbidden scope;
- task set, WIP limits, budget, deadline, risk controls, and escalation conditions;
- required evidence, phase gate, rollback/recovery obligations, and decisions requiring authority.

Phase states are `Pending`, `Ready`, `Active`, `Blocked`, `Review`, `Verification`, `Closed`, `Returned`, `Deferred`, `Uncertain`, and `Cancelled`. Only Core validates transitions. Evidence can cause a return or ChangeRequest; it cannot silently rewrite an approved plan.

## Task Kanban and WIP

Normal task flow is:

`Backlog → Ready → InProgress → Review → Verification → Done`.

Exception states are `Blocked`, `NeedsClarification`, `NeedsApproval`, `Returned`, `Deferred`, `Uncertain`, and `Cancelled`. Kanban state is not worker-attempt state, and `Done` does not itself close a phase or mission.

Initial Core-enforced WIP principles are:

- one writable implementation task per isolated worktree;
- bounded parallel read-only research with explicit objectives;
- bounded unresolved decisions and clarification requests;
- bounded pending protected effects and unresolved uncertainty;
- no new writable work when phase budget, risk, or recovery policy requires a stop.

## Inner Hermes loop

~~~mermaid
flowchart LR
  O["Observe"] --> D["Diagnose"]
  D --> H["Hypothesize"]
  H --> S["Select minimal action"]
  S --> A["Core authorize if effectful"]
  A --> X["Tool boundary execute"]
  X --> V["Verify actual result"]
  V --> C["Compare expected vs observed"]
  C --> U["Update belief and confidence"]
  U --> N{"Next decision"}
  N -->|continue| O
  N -->|plan invalid| R["Request controlled replan"]
  N -->|authority / uncertainty| E["Escalate"]
  N -->|criteria met or cannot progress| Stop["Stop honestly"]
~~~

Read-only reasoning may omit effect authorization and execution but still records observation and validation. An effectful iteration cannot skip Core authorization or actual-result probing.

## Iteration record

Every inner-loop iteration records:

| Field | Requirement |
| --- | --- |
| Governing context | policy, specification, contract, plan, phase, TaskPacket, skill, profile, and context-pack versions |
| Hypothesis | falsifiable explanation of the current gap and confidence before action |
| Evidence before | diagnostics, artifacts, tests, findings, and unresolved uncertainty |
| Proposed action | minimal typed action, expected result, capability/effect class, cost, and rollback |
| Authorization | Core policy/capability/budget/approval decision and durable intent identity |
| Actual result | tool-boundary observation, exit/result data, changed artifacts, and evidence after |
| Comparison | expected versus observed result and confidence delta |
| Next decision | continue, retry with changed hypothesis, split, replan, escalate, or stop |

A retry without a changed, evidence-supported hypothesis is not progress.

## Loop guards

Core/Hermes stop or escalate an inner loop when it detects:

- repeated equivalent actions or identical failures;
- no observable state change despite claimed progress;
- oscillation between states or strategies;
- increasing diagnostics, regressions, or unresolved risk;
- exhausted time, token, cost, tool-call, or retry budget;
- repeated schema, citation, context, or verification failure;
- invalid/stale governing artifact or broken traceability;
- unresolved uncertainty about a non-idempotent effect;
- required human authority, new scope, dependency, secret, external transfer, or protected effect;
- environment, sandbox, repository, or evidence-integrity failure.

Loop guards are policy-configurable and evaluated from durable observations, not prompt instructions or a model's self-assessment.

## Delivery quality stages and self-review

Within appropriate phases, task/phase plans select applicable quality stages such as Understand, Plan, Build, Compile/Static Analysis, Test, Review, Benchmark, Security, Refactor, Document, and Evaluate. These stages are evidence obligations, not a rigid universal phase graph.

Independent review proceeds as applicable:

`builder → deterministic validation → reviewer subagent → test/quality subagent → security/performance review → CompletionProposal`.

The same model and context that produced an implementation must not be the only basis for review. Reviewers and QA return findings or gate evaluations; they do not approve plans, effects, exceptions, or mission completion.

| Quality activity | Required output | Evidence producer |
| --- | --- | --- |
| Understand / impact | bounded problem, affected symbols/files/tests, ambiguity | Brain Agent / Explorer / Architecture Analyst |
| Build | isolated change artifact, action/effect trace, implementation uncertainty | Builder |
| Compile / static / LSP | reproducible command and diagnostic delta | Tool boundary + QA/specialist |
| Test | requirement-bound test matrix and result artifacts | Test Engineer / QA |
| Review | independent findings and dispositions | Reviewer |
| Benchmark | baseline, method, result, regression threshold | Performance Reviewer |
| Security | dependency, secret, code, supply-chain, and policy findings | Security Reviewer |
| Documentation | user/design/runbook/memory-candidate impact | Documentation Agent |
| Evaluate | OutcomeContract trace, evidence completeness, unresolved risk | Core gate evaluator from supplied evidence |

## Effect firewall

Every material effect follows one path:

`agent/model proposes → Core classifies → policy decision → capability check → approval where required → durable intent → tool boundary executes → probe actual result → record evidence`.

The firewall covers files, processes, shell, Git, network, secrets, dependencies, databases, pushes, pull requests, merges, deployments, budgets, and model calls. Skills and hooks use the same path. Guardrails are enforced at Core and tool boundaries, never by prompt wording alone.

## Quality and outcome thresholds

Thresholds come from policy and the approved OutcomeContract, not from model capability. Initial templates require applicable build/static checks and targeted tests to pass; findings to be dispositioned; no unresolved blocking security issue; regression, coverage, and performance within approved bounds; documentation/migration impact addressed; evidence complete; and work inside scope, budget, and architecture constraints.

Different supported profiles may use different strategies, tools, context, retries, time, and cost. Acceptance is permitted only when functional behavior, public contracts, architecture, security, quality, regressions, evidence completeness, and unresolved risk are contract-equivalent. Formatting beyond repository rules and non-contractual internal style need not match.

## Escalation and stop semantics

The escalation ladder is:

1. retry with improved evidence/context and a new hypothesis;
2. select a more specialized approved skill;
3. split the TaskPacket without changing its requirement envelope;
4. invoke an independent reviewer/advisor subagent;
5. request a controlled plan change or replan;
6. switch to another supported profile within policy/budget;
7. request clarification;
8. request a human decision or protected-effect approval;
9. stop safely with uncertainty and partial evidence preserved.

Success requires every applicable acceptance criterion and evidence binding, no blocking finding, approved scope satisfied, artifacts preserved, and budget/transition rules met. Failure, cancellation, or uncertainty records the reason, partial output, observed effect state, and recovery recommendation. Optimistic summaries cannot conceal non-convergence.

## Autonomy mode interaction

Manual mode gates configured material effects. Limited Approval permits low-risk work inside declared capabilities and gates strategic changes. Full Autonomous runs inner loops inside an approved envelope while preserving hard effect, specification, plan, authority, change, and completion gates. Enterprise Policy adds organization controls and retention. No mode lets Brain Agent, Hermes, a builder, reviewer, skill, or hook approve itself.

See also: [05_Worker_Architecture.md](05_Worker_Architecture.md), [13_Mission_Control.md](13_Mission_Control.md), [14_Security_Architecture.md](14_Security_Architecture.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), and [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md).
