# 09 — Skills System

## Purpose and invariant

A skill is a versioned, executable engineering procedure—not prompt text or ambient expertise. It tells Hermes when a method applies, what context and capabilities it requires, how to perform it, how to verify it, which evidence it must return, and how it fails safely.

Skills refine execution inside a Core-issued TaskPacket. They cannot redefine policy, an approved EngineeringSpecification, OutcomeContract, DeliveryPlan, PhasePlan, scope, acceptance criteria, budget, or authority. A skill may propose typed actions; only Core-authorized tool boundaries perform effects.

## Skill definition contract and staged distribution

H3.1 proves a repository-owned built-in Skill Kernel. Built-ins use static, versioned manifests and exact repository/content hashes; there is no installer, public/local registry, marketplace, third-party executable hook, or generalized signing service in the H3.1 minimum. Conditional H3.2 may add governed packaging only after accepted H3.1 evidence shows measurable value without governance loss.

| Part | Required content |
| --- | --- |
| Manifest | stable identity, semantic version, exact repository revision/content hash, compatibility, and built-in trust classification; H3.2 package descriptors additionally require publisher, license, release channel, signature/provenance, and trust tier |
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

The manifest declares a requested capability ceiling. Core grants a subset for a specific project, phase, TaskPacket, attempt, and effect. A built-in definition has no ambient access. If H3.2 is later authorized, package identity and signature still grant no capability and package code remains isolated.

## Skill Runtime

At task lease time, Hermes loads only pinned, policy-qualified skills and runs them through the Skill Runtime. H3.1 loads only repository-owned built-ins registered by the release; H3.2 package activation is a separate conditional path. The runtime:

1. validates built-in registration, repository/content hash, version, compatibility, revocation and TaskPacket applicability; only an authorized H3.2 path additionally validates package identity/signature/provenance;
2. asks the Context Compiler for the declared minimal provenance-labelled inputs;
3. verifies the Core-issued capability subset and sandbox/tool compatibility;
4. executes the bounded procedure and typed hooks with cancellation and budget accounting;
5. validates output schema, verification results, evidence contract, and uncertainty;
6. returns artifacts, findings, proposed actions, measurements, and failure classification to Core.

Skill execution state is bounded Hermes state. Exact built-in skill versions/hashes—and exact package identities only when H3.2 exists—plus inputs, outputs, evidence, and invocation events are durable Core records for reproducibility.

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

## Staged lifecycle

~~~mermaid
stateDiagram-v2
  [*] --> BuiltInDraft
  BuiltInDraft --> BuiltInCandidate: static manifest + exact repository/content hash
  BuiltInCandidate --> BuiltInValidated: schema + security + compatibility + evaluation pass
  BuiltInValidated --> BuiltInApproved: release review + Core policy
  BuiltInApproved --> Deprecated: successor or risk notice
  BuiltInApproved --> Quarantined: regression or unresolved finding
  BuiltInApproved --> Revoked: critical security/policy issue
  Quarantined --> BuiltInApproved: revalidated + approved
  BuiltInApproved --> PackageCandidate: H3.2 explicitly authorized
  PackageCandidate --> PackageStaged: identity + provenance + signature + dependency review
  PackageStaged --> PackageActive: atomic Core activation
  PackageActive --> PackageRevoked: advisory or policy decision
  PackageRevoked --> [*]
  Deprecated --> [*]
  Revoked --> [*]
~~~

H3.1 ends at a release-owned `BuiltInApproved` definition and has no package activation surface. An active attempt records the exact skill version/hash. Revocation prevents new selection and triggers a visible risk/compatibility decision for affected active work; it does not erase historical evidence. Package states exist only after a separately accepted H3.2 gate.

## Skill evolution and Skill Memory

A worker or Hermes may propose a skill improvement only as a `MemoryCandidate` and candidate built-in change with linked mission evidence, before/after fixtures, expected benefit, compatibility impact, new capabilities, and risk. Deterministic validation, security/license checks, controlled evaluation, ablation where useful, and designated release approval precede promotion. An H3.2 package candidate is permitted only after packaging itself is authorized.

Skill Memory stores measured procedure behavior by version, task class, environment, profile, cost, failure mode, and evidence quality. It can influence routing but cannot silently alter a built-in definition, active package, policy, or acceptance threshold. Successful project work alone never self-upgrades a shared skill.

## Conditional H3.2 registry, distribution, and trust

H3.1 provides no registry, installer, or approved-package catalogue. If accepted H3.1 value evidence and a later H3.2 authorization justify distribution, H3.2 may introduce a local staging/activation catalogue; an open marketplace remains deferred. Entries record publisher identity, signature/provenance, package hash, compatibility range, changelog, permissions, license, evaluation summary, support/deprecation status, and vulnerability/revocation metadata.

| Tier | Source | Default treatment |
| --- | --- | --- |
| Core | WePLD-maintained | H3.1 release-owned built-in with exact repository/content hash; still TaskPacket/capability limited |
| Organization | approved internal publisher | organization policy and review |
| Project-local | scoped to one repository | quarantined to project; no automatic cross-project publishing |
| Community candidate | external publisher | unavailable to execution until explicit quarantine review and approval |

If H3.2 is authorized, installation/staging, update, removal, publishing, activation, rollback, revocation, and trust changes are Core-governed effects. Removal checks dependent profiles and preserves historical package metadata. None of these package operations exists in the H3.1 minimum.

## Acceptance criteria

- Every attempt identifies exact skill versions, inputs, procedure, capabilities, outputs, verification, and evidence.
- A skill cannot call a model/tool, register a hook, write state, or access context outside its granted contract.
- Router decisions are recorded, policy-valid, reproducible, and measurable.
- Revoked skills cannot be newly selected and cannot disappear from historical audit.
- No skill can self-install, self-publish, self-upgrade, lower gates, or bypass Core.
- H3.1 can operate with only static repository-owned built-ins and must prove value before H3.2 packaging, signing, activation, rollback, or revocation is authorized.

See also: [05_Worker_Architecture.md](05_Worker_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), and [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).
