# 26 — Testing Strategy

## Quality philosophy

WePLD must test the engineering organization as rigorously as it tests its product code. The key question is not only “does a function return the expected result?” but “can an autonomous system safely plan, act, recover, explain, and refuse under adversarial conditions?” Quality evidence is a first-class input to orchestration gates.

## Test layers

| Layer | Focus | Examples | Required owner |
| --- | --- | --- | --- |
| Domain/unit | pure invariants and state transitions | invalid task transition rejected; budget calculation; risk classification | context owner |
| Property/model | broad state-space correctness | event replay preserves projection; lease cannot have two active holders | Core/quality |
| Contract | stable ports and schema compatibility | brain/worker/tool/plugin adapter conformance | port owner + adapter owner |
| Integration | real adapters and persistence boundaries | local RPC auth, SQLite recovery, Git worktree lifecycle | Core/platform |
| End-to-end mission | user-visible engineering flow | mission → plan → task → evidence → decision → completion | product/quality |
| Evaluation | model/worker behavior under fixtures | structured outputs, citations, review accuracy, safe refusal | evaluation owner |
| Security/adversarial | abuse and boundary resistance | prompt injection, secret exposure, package tampering, capability misuse | Security |
| Performance/reliability | latency, resource, failure behavior | event burst, worker loss, provider outage, quota pressure | platform/performance |
| Accessibility/usability | inclusive, comprehensible operation | keyboard flow, screen reader live status, evidence comprehension | UX/accessibility |

## Critical acceptance scenarios

The following scenarios are mandatory before corresponding autonomy capabilities ship:

1. A worker requests an undeclared filesystem, network, secret, or protected-branch action; enforcement denies it and records an explainable policy outcome.
2. A worker or daemon crashes while an effect is uncertain; recovery avoids silent duplicate action and exposes a safe next step.
3. A brain returns invalid structured output, a tool-call proposal, an injection-tainted recommendation, or a confident uncited claim; the system validates, contains, and routes appropriately.
4. A mission completes only with all required checks; missing, stale, forged, or failing evidence blocks completion.
5. A privileged channel message is spoofed, replayed, or sent from an unauthorized identity; no decision is applied.
6. A package signature/advisory/revocation changes; new selection stops and active work impact is visible.
7. A lost provider, index, telemetry path, or Studio client produces accurate degraded state, not false health.
8. A user can trace any material change from mission through task, policy, artifacts, tests/review, and decision.

## Test fixtures and evaluation sets

Fixtures are versioned, classified, reviewable artifacts. They include representative repositories, mission briefs, task graphs, test outputs, provider prompts/responses, security attacks, failure timelines, and expected policy decisions. Sensitive fixtures use sanitized synthetic data unless an approved isolated environment is required. Evaluation results record profile/tool/skill versions and cost/latency so regressions are detectable.

Model evaluation measures schema validity, evidence/citation correctness, task outcome quality, unsafe-action proposal rate, refusal correctness, consistency, latency, and cost. It never promotes a profile purely on benchmark eloquence. Test data must cover missing context, contradictory evidence, malicious repository content, ambiguous instructions, and budget pressure.

## Gate policy

Every task declares required gates. Baseline gates are build/static checks, targeted tests, independent review, security scan, and documentation impact. Coverage is used as a regression signal rather than a universal substitute for behavior testing; thresholds are project-specific and exceptions are time-bound decisions. Benchmark, accessibility, compliance, migration, and release gates are added by change type/risk.

Gate evidence must identify the command/toolchain/environment, input artifact, results, threshold, and owner. A retry cannot overwrite a failing result; it produces a new check linked to the prior one and a disposition.

## Reliability and chaos testing

The platform must deliberately test daemon restarts, corrupted/absent projections, worker heartbeat loss, timeout/retry races, duplicate delivery, partial artifact writes, disk pressure, network outage, provider failure, clock change, and revocation during active work. The expected response is data integrity plus a visible state—not merely process survival.

## Test execution governance

Tests run locally before a task proposes completion and in isolated CI/release environments when implementation begins. Test runners are themselves toolchain capabilities with version, provenance, resource limits, network policy, and artifact output. Flaky tests, ignored security findings, leaked fixture data, and nondeterministic evaluation changes are defects with owners.

## Definition of testable

A feature is not ready for implementation planning unless it identifies: invariant, owner, input fixture, expected normal/denied/degraded/recovery behavior, evidence artifact, performance/security/accessibility implications, and release gate. If a behavior cannot be tested, it needs a narrower contract or an explicit risk decision.

See also: [10_Loop_Engineering.md](10_Loop_Engineering.md), [14_Security_Architecture.md](14_Security_Architecture.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), [25_Development_Guidelines.md](25_Development_Guidelines.md), and [27_Performance_Goals.md](27_Performance_Goals.md).

