# 10 — Loop Engineering

## Principle

WePLD does not perform one-pass AI coding. Engineering is a durable, iterative loop with evidence-producing stages and controlled retries. A loop progresses only when it gains measurable evidence or reduces a declared uncertainty; it never repeats merely because a model requests another attempt.

## Standard loop

~~~mermaid
flowchart LR
  U[Understand] --> P[Plan]
  P --> B[Build]
  B --> C[Compile / Static checks]
  C --> T[Test]
  T --> R[Review]
  R --> BM[Benchmark]
  BM --> S[Security scan]
  S --> RF[Refactor]
  RF --> D[Document]
  D --> E[Evaluate]
  E -->|gaps remain| U
  E -->|criteria satisfied| Done[Completion proposal]
~~~

Stages may be skipped only when a task’s declared change type makes them inapplicable and the policy records why. For example, a documentation-only task may not require a benchmark, but it still requires review and validation appropriate to its risk.

## Stage contracts

| Stage | Required output | Gate owner |
| --- | --- | --- |
| Understand | bounded problem statement, affected artifacts, ambiguity list | Planner / Architecture |
| Plan | task DAG, acceptance criteria, risks, capabilities, rollback approach | Orchestration / Policy |
| Build | isolated change artifact and implementation log | Builder |
| Compile / static | reproducible command results, lint/type/build evidence | QA / specialist |
| Test | test matrix and result artifacts | QA / Testing |
| Review | independent findings and disposition | Reviewer |
| Benchmark | baseline, method, result, regression threshold | Performance / Benchmark |
| Security scan | dependency, secret, code/supply-chain findings | Security |
| Refactor | constrained cleanup with tests retained | Builder / Reviewer |
| Document | changelog, design impact, runbook/knowledge updates | Documentation |
| Evaluate | acceptance-criteria traceability and quality summary | Orchestration |

## Quality thresholds

Thresholds are project policy, not universal constants. The initial policy template requires: build/static checks pass; targeted tests pass; all required reviewers approve or findings are dispositioned; no unresolved critical/high security finding; coverage does not regress below the project baseline without approved exception; performance stays within an approved regression budget; documentation and migration impact are addressed; and the change remains within mission scope and budget.

For safety-critical, financial, regulated, or release tasks, policy can add independent-review, reproducibility, accessibility, compliance, or human acceptance gates. A model assertion, screenshot, or “looks good” statement is never gate evidence by itself.

## Stop conditions

A task or mission stops successfully only when all required acceptance criteria and gates have linked evidence, no blocking finding remains, the approved scope is satisfied, required artifacts are preserved, and budget/time/retry limits are respected. It stops unsuccessfully or escalates when any of these occur:

- a strategic decision, new permission, external transfer, dependency, or scope expansion is needed;
- retries exceed a configured cap or repeated attempts show no measurable progress;
- evidence conflicts materially;
- a critical security/compliance issue is detected;
- cost, time, or resource budget is exhausted;
- environment/sandbox integrity cannot be established.

The system records the reason, partial output, and recommended recovery path. It does not hide a failed loop behind an optimistic summary.

## Retry and improvement policy

Retries must have a named hypothesis: changed context, alternate implementation strategy, additional test fixture, human decision, or different compatible brain/worker profile. Every retry consumes a budget and emits a comparison to the prior attempt. A quality regression triggers rollback/review, not iterative rationalization.

## Autonomy mode interaction

Manual mode gates material stages before execution. Limited Approval permits low-risk planning and verification but gates declared effects. Full Autonomous performs the loop inside its envelope while hard safety gates remain. Enterprise Policy Mode injects organization-specific controls and retention requirements. The same stage model applies to all modes; only policy and approval routing differ.

See also: [05_Worker_Architecture.md](05_Worker_Architecture.md), [13_Mission_Control.md](13_Mission_Control.md), [14_Security_Architecture.md](14_Security_Architecture.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).

