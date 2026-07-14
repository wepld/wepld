# 09 — Skills System

## Purpose and invariant

A skill is a versioned, executable engineering procedure—not prompt text or ambient expertise. It tells Hermes when a method applies, what context and capabilities it requires, how to perform it, how to verify it, which evidence it must return, and how it fails safely.

Skills refine execution inside a Core-issued TaskPacket. They cannot redefine policy, an approved EngineeringSpecification, OutcomeContract, DeliveryPlan, PhasePlan, scope, acceptance criteria, budget, or authority. A skill may propose typed actions; only Core-authorized tool boundaries perform effects.

## Skill package contract

| Part | Required content |
| --- | --- |
| Manifest | stable identity, semantic version, publisher, license, release channel, integrity hash/signature, compatibility, and trust tier |
| Applicability | task/role/language/risk predicates, exclusions, prerequisites, and routing signals |
| Context contract | required and optional sources, trust/freshness constraints, Context Compiler recipe, and token estimate |
| Tool contract | required tools, versions, data classes, network, paths, secrets, and maximum capabilities |
| Procedure | ordered executable steps, typed inputs/outputs, bounded branches, and cancellation points |
| Verification | deterministic checks, expected observations, quality thresholds, and independent-review needs |
| Failure modes | detectable failure/uncertainty classes, rollback, retry eligibility, and escalation path |
| Output schema | artifacts, findings, proposed actions, uncertainty, and machine-valid structure |
| Evidence contract | required logs, hashes, diagnostics, measurements, citations, and requirement/gate bindings |
| Hooks | declared typed lifecycle hooks, mode, ordering, failure semantics, and capabilities where applicable |
| Examples and evaluations | representative, adversarial, and negative fixtures with expected outcomes |
| Documentation | roles, limitations, risks, setup, migration, deprecation, and support policy |

The manifest declares a requested capability ceiling. Core grants a subset for a specific project, phase, TaskPacket, attempt, and effect. Package code has no ambient access.

## Skill Runtime

At task lease time, Hermes loads only pinned, policy-qualified skills and runs them through the Skill Runtime. The runtime:

1. validates signature, trust, version, revocation, compatibility, and TaskPacket applicability;
2. asks the Context Compiler for the declared minimal provenance-labelled inputs;
3. verifies the Core-issued capability subset and sandbox/tool compatibility;
4. executes the bounded procedure and typed hooks with cancellation and budget accounting;
5. validates output schema, verification results, evidence contract, and uncertainty;
6. returns artifacts, findings, proposed actions, measurements, and failure classification to Core.

Skill execution state is bounded Hermes state. Exact package versions/hashes, inputs, outputs, evidence, and invocation events are durable Core records for reproducibility.

## Skill Router

The Skill Router selects a proposed execution route containing:

- skill and version;
- Brain Agent, builder, or subagent role/profile;
- required tools and Core capability request;
- context recipe and budget allocation;
- verification and independent-review level;
- expected evidence and stop/escalation conditions.

Routing considers TaskPacket requirements, language/toolchain, risk, data classification, provider locality, compatibility, measured skill/profile outcomes, cost, latency, and current availability. Core validates the route against governing artifacts and policy before issuing a lease or capability. The routing policy is replaceable and evaluated through the harness; it is never an opaque model preference.

## Initial skill families

- repository exploration and Git forensics;
- architecture, dependency, API, and schema analysis;
- Rust, TypeScript, Python, and supported-language engineering;
- debugging and recovery investigation;
- test planning, generation, QA, and regression analysis;
- security, dependency, secret, and supply-chain review;
- database migration and data-integrity verification;
- performance analysis and benchmarking;
- documentation and traceability maintenance;
- LSP/diagnostic impact analysis and affected-test mapping.

Initial support is intentionally bounded. A family is available only after its procedures, compatibility, safety, and evidence contracts pass the applicable milestone gate.

## Hook participation

Skills may subscribe to typed lifecycle hooks such as context compilation, model/tool calls, file writes, tests, diagnostics changes, snapshots, phase gates, failure, recovery, completion, and mission closure. Every hook declares one mode:

- **observational:** emits telemetry/evidence only;
- **validating:** returns a typed validation result;
- **blocking:** may ask Core to deny or pause a transition under an applicable policy rule;
- **effect-producing:** may only propose a typed action that follows the normal effect firewall.

Hooks cannot mutate Core state, issue capabilities, call tools directly, broaden context, or turn a plugin into an escape path. Core controls registration, ordering, timeouts, failure handling, and audit.

## Lifecycle

~~~mermaid
stateDiagram-v2
  [*] --> Draft
  Draft --> Candidate: packaged + signed
  Candidate --> Validated: schema + security + compatibility + evaluation pass
  Validated --> Approved: Core policy / designated curator decision
  Approved --> Published: approved registry channel
  Published --> Deprecated: successor or risk notice
  Published --> Quarantined: regression or unresolved finding
  Published --> Revoked: critical security/policy issue
  Quarantined --> Published: revalidated + approved
  Deprecated --> [*]
  Revoked --> [*]
~~~

An active attempt records the exact skill version/hash. Revocation prevents new selection and triggers a visible risk/compatibility decision for affected active work; it does not erase historical evidence.

## Skill evolution and Skill Memory

A worker or Hermes may propose a skill improvement only as a `MemoryCandidate` and candidate package with linked mission evidence, before/after fixtures, expected benefit, compatibility impact, new capabilities, and risk. Deterministic validation, security/license checks, controlled evaluation, ablation where useful, and designated approval precede promotion.

Skill Memory stores measured procedure behavior by version, task class, environment, profile, cost, failure mode, and evidence quality. It can influence routing but cannot silently alter an installed package, policy, or acceptance threshold. Successful project work alone never self-upgrades a shared skill.

## Registry, distribution, and trust

V1 provides a local registry and approved-package catalog. An open marketplace is deferred. Entries record publisher identity, signature/provenance, package hash, compatibility range, changelog, permissions, license, evaluation summary, support/deprecation status, and vulnerability/revocation metadata.

| Tier | Source | Default treatment |
| --- | --- | --- |
| Core | WePLD-maintained | reviewed and signed; still TaskPacket/capability limited |
| Organization | approved internal publisher | organization policy and review |
| Project-local | scoped to one repository | quarantined to project; no automatic cross-project publishing |
| Community candidate | external publisher | unavailable to execution until explicit quarantine review and approval |

Installation, update, removal, publishing, and trust changes are Core-governed effects. Removal checks dependent profiles and preserves historical package metadata.

## Acceptance criteria

- Every attempt identifies exact skill versions, inputs, procedure, capabilities, outputs, verification, and evidence.
- A skill cannot call a model/tool, register a hook, write state, or access context outside its granted contract.
- Router decisions are recorded, policy-valid, reproducible, and measurable.
- Revoked skills cannot be newly selected and cannot disappear from historical audit.
- No skill can self-install, self-publish, self-upgrade, lower gates, or bypass Core.

See also: [05_Worker_Architecture.md](05_Worker_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), and [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).
