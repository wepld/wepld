# ADR-0024 — Establish the evaluation spine and exact run provenance

**Status:** Proposed
**Date:** 2026-07-16
**Owner:** Architecture Review Board
**Review:** Before H1 or H2 implementation authorization

## Context

If evaluation contracts arrive only at H8, earlier milestones can produce incompatible logs, lose exact inputs, and select baselines after results are known. Later ablation and certification would then depend on reconstructed or incomparable evidence. WePLD needs a small, durable evaluation spine before implementation begins without prematurely building the full H8 certification service.

## Decision

Introduce versioned, append-only evaluation contracts before H1/H2:

- `EvaluationCase` identifies the fixture, problem, preregistered hypothesis, expected behavior, gates, scoring contract, classification, and immutable source versions.
- `TreatmentArm` declares the independent variable, controlled variables, enabled components, profile, budget, and planned repetitions.
- `RunManifest` records exact repository and fixture commits/hashes; specification, Outcome Contract, plan, policy, and gate versions; environment, tools, adapters, models, prompts/configuration, skills, context strategy, data, budgets, and randomness/nondeterminism settings.
- `EvaluationRun` binds one case, arm, and manifest to lifecycle state, observed actions/effects, evidence references, timing, and terminal outcome.
- `MetricObservation` records a typed metric definition/version, value, unit, collection method, subject, time, and evidence provenance.
- `ProtocolDeviation` preserves any departure, cause, scope, contamination impact, authorizing decision where permitted, and inclusion/exclusion disposition.
- `EvaluationResult` records threshold versions, analysis method, uncertainty, findings, safety/evidence disposition, and linked raw artifacts without erasing failed or inconclusive runs.

Every Hermes milestone H1–H9 emits evidence compatible with these contracts. Each milestone establishes a versioned baseline before comparison and records regressions against that baseline. Exact fixture, repository, contract, policy, configuration, environment, provider, and tool provenance is mandatory; hashes do not replace governed identifiers and versions.

The early spine supports capture, validation, append-only storage, export, and deterministic comparison inputs. Before H1 implementation, the resolved Baseline Gate identifies the retained accepted-prerequisite or approved replacement-foundation path; an independent authorized reviewer approves and version-binds fixtures derived from that path; and the spine records a pre-H1 foundation `EvaluationCase`/`EvaluationRun`. That Baseline run must reach an honest terminal state, be assessed with required observations validated and deviations dispositioned, and produce a finalized `EvaluationResult`. It is a comparator/provenance record, not execution of the plan artifact or certification. Cross-model experimentation, randomized/repeated trials, ablation policy, and profile certification remain H8 responsibilities under ADR-0025.

## Reason

Early common contracts make later causal comparison possible, prevent retrospective baseline selection, and turn evaluation evidence into durable reviewable state rather than milestone-specific telemetry.

## Benefits

- Comparable evidence accumulates from H1 onward.
- Regressions and protocol deviations remain visible and attributable.
- H8 can consume a history of reproducible runs instead of retrofitting provenance.
- The early implementation stays narrower than a certification platform.

## Trade-offs

- H1/H2 must carry evaluation metadata before advanced evaluation features exist.
- Exact provenance increases storage and adapter discipline.
- Contract evolution requires explicit compatibility and supersession rules.

## Migration

Before H1 or H2 authorization, architecture review must approve the minimum schemas, lifecycle rules, append-only/supersession behavior, provenance requirements, baseline procedure, export format, and access/retention controls. Validation must prove schema round-trip, referential integrity, immutable raw-run retention, deterministic manifest hashing, explicit missing/unknown fields, and faithful recording of failed, unsafe, and deviating runs.

Existing Draft PR #1 traces or fixtures may become candidate `EvaluationCase` inputs only after independent provenance and contract review. They are not accepted baselines, controlled runs, or certification evidence by virtue of this ADR.
