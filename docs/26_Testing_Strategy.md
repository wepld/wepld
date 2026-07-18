# 26 — Testing Strategy

## Quality philosophy

WePLD tests the engineering organization, not only functions. The governing question is: can the system describe, specify, plan, act, recover, explain, review, learn, converge, and refuse while every authority and effect remains bounded?

Quality evidence is an input to Core gates. Model confidence, polished prose, a visual demo, or PR status cannot satisfy a gate. Draft PR #1’s committed tests are candidate-baseline evidence only until independently reproduced and accepted; this plan does not certify them.

## Test layers

| Layer | Focus | Examples | Required owner |
| --- | --- | --- | --- |
| Domain/unit | pure invariants and lifecycle transitions | approved spec immutable; invalid phase/task transition rejected; budget and WIP math | context owner |
| Property/model | broad state-space correctness | transition sequences preserve invariants; no task has two writable attempts; retries remain bounded | Core/quality |
| Contract | schema, authority, wire, and compatibility | artifacts 1–16, WWP, Brain, skill/hook, LSP, evidence, memory, surface commands | contract + adapter owner |
| Integration | real storage/effect/adapters | ledger/CAS atomicity, worktrees, provider gateway, LSP lifecycle, effect intent/probe | Core/platform |
| End-to-end workflow | user-visible governed delivery | describe→spec approve→plan approve→phases/Kanban→evidence→completion decision | product/quality |
| Evaluation | model/harness behavior | outcome equivalence, convergence, non-convergence honesty, ablations | evaluation owner |
| Security/adversarial | boundary and authority resistance | injection, path/ref escape, hook/skill/subagent bypass, forged evidence, credential/data egress | Security |
| Reliability/chaos | failure and uncertainty | process loss, stale index, partial effect, disk pressure, duplicate delivery, conflict | platform |
| Performance/resource | responsiveness and bounded cost | Context Compiler/LSP latency, pack size, loop attempts, parallel WIP, provider cost | platform/evaluation |
| Accessibility/usability | comprehensible operation | keyboard/screen-reader flow, stale/uncertain disclosure, evidence comprehension | UX/accessibility |

## Authority and immutable-truth suite

Mandatory invariants:

1. Governance policy outranks specification, outcome contract, delivery plan, phase plan, task packet, and tool action.
2. Only an authenticated authorized principal approves specification, plan, protected effect, or completion.
3. Brain, Hermes, builder, reviewer, test/security subagent, skill, hook, memory, and tool output cannot approve themselves or mutate higher authority.
4. An approved specification version is immutable. A WHAT change produces a new version through a Specification Change Request; a HOW-only change produces a Plan Change Request.
5. Stale expected versions and replayed/spoofed approvals are rejected without partial mutation.
6. CLI, Studio, MCP, and APIs produce equivalent Core command outcomes and cannot own hidden workflow state.

## Artifact and traceability suite

For `MissionCharter`, `EngineeringSpecification`, `OutcomeContract`, `PlanProposal`, `PlanAssessment`, `PlanDecision`, `DeliveryPlan`, `PhasePlan`, `TaskPacket`, `SOPGraph`, `RoleNode`, `ActionContract`, `InputSubscription`, `OutputContract`, `MissionExplorationBranch`, `CompactionRecord`, `BoundedToolResult`, `ToolOutputArtifact`, `SandboxFailureResult`, `RiskItem`, `Assumption`, `DecisionRequest`, `ChangeRequest`, `EvidenceRequirement`, `EvidenceBundle`, `CompletionProposal`, `CompletionDecision`, `MemoryCandidate`, `Retrospective`, `EvaluationCase`, `TreatmentArm`, `RunManifest`, `EvaluationRun`, `MetricObservation`, `ProtocolDeviation`, and `EvaluationResult`, test:

- schema/version/provenance round-trip;
- valid lifecycle and every invalid transition;
- proposer/approver authority;
- authoritative, derived, or untrusted classification;
- supersession and compatibility;
- missing/stale/contradictory references;
- trace chain from user intent → specification requirement → outcome contract → phase → task → evidence → completion decision → memory candidate.

The trace must be complete by reference and version, not inferred later from prose.

## Plan-qualification suite

Plan qualification tests the complete authority-preserving path:

`Brain proposal → deterministic compilation/normalization → candidate plan → structural validation → initial assessment → independent review when policy requires → finalized Ready assessment → authenticated PlanDecision → approved DeliveryPlan`.

Fixtures prove that:

- `PlanProposal`, `PlanAssessment`, and `PlanDecision` have distinct schemas, identities, authors, versions, provenance, and lifecycle rules;
- deterministic compilation rejects or explicitly reports ambiguous, missing, contradictory, or unsupported proposal content rather than inventing authority;
- structural validity does not imply architectural quality, feasibility, safe sequencing, sufficient evidence, or approval;
- risk-tier policy determines assessment depth, reviewer independence, security/quality participation, and approval authority;
- a required-review candidate remains `ReviewRequired`; reviews are separate immutable records; Core alone finalizes a new `Ready` assessment version after validating their exact bindings;
- a proposal producer cannot approve the resulting plan or act as its sole acceptance-critical reviewer;
- `PlanDecision` binds the exact policy version, risk-tier decision, and every required assessment/review record ID, version, and hash; missing, stale, forged, swapped, or post-review-mutated bindings fail closed;
- model voting, confidence, agreement, and repeated proposals never create authority;
- policy may qualify one proposal without requiring wasteful multiple-plan generation, while allowing alternatives when uncertainty or risk justifies them;
- returned, rejected, superseded, and stale proposals remain distinguishable and replayable; and
- only an authenticated authorized `PlanDecision` can produce an approved `DeliveryPlan`.

## Phase, Kanban, WIP, and change-control suite

- Phase entry requires all dependencies, authority, budget, allowed skills/tools, scope, and risk controls.
- Task flow supports Backlog, Ready, InProgress, Review, Verification, Done plus Blocked, NeedsClarification, NeedsApproval, Returned, Deferred, Uncertain, and Cancelled.
- Done requires valid exit evidence; Blocked/Deferred/Uncertain never render as Done.
- Default one writable task per isolated worktree; bounded read-only parallelism; policy-configured limits on decisions and pending protected effects.
- Concurrent writers with overlapping scope are refused or isolated/conflict-controlled.
- Plan evidence cannot silently change specification truth. Change classification, approval, invalidation, replan, and resume are replayable.

### SOPGraph and authorized subscription conformance

- Deterministic compilation from exact approved `DeliveryPlan`, `PhasePlan`, and `TaskPacket` versions produces stable `RoleNode`, `ActionContract`, authorized `InputSubscription`, `OutputContract`, dependency, evidence, and stop/escalation edges.
- Core projects only events and artifacts allowed by the role's assignment, classification, phase, and capability envelope.
- A role cannot self-subscribe, widen an input selector, observe a free shared environment, or establish an untyped peer-chat channel.
- Builder, reviewer, tester, security, and explorer fixtures prove least-knowledge delivery, deterministic replay, revocation, and no authority through observation.

## Skill, hook, and effect-firewall suite

Skills are tested as executable procedures: manifest validation, applicability, context/tool prerequisites, capability request, deterministic resolution, procedure output, verification, failure modes, evidence, compatibility, trust/signature/revocation behavior where supported.

Hooks are tested by class:

- observational hooks cannot block, mutate, or produce effects;
- validating hooks return typed findings only;
- blocking hooks can stop only through declared Core policy semantics;
- effect-producing hooks must use the same effect firewall as every other actor;
- hook recursion, timeout, crash, schema failure, reordering, and malicious capability requests fail closed.

Every protected effect type—file, process/shell, Git, network, secret, dependency, database, push, pull request, merge, deployment, budget, and model call—tests:

`proposal → classification → policy → capability → approval when required → durable intent → execution → probe → evidence`.

Denied, failed, uncertain, conflicted, and recovered effects remain distinguishable. A retry never silently repeats an uncertain non-idempotent effect.

Capability-projected catalog tests vary policy, Task Packet, issued capabilities, role, sandbox tier, classification, phase, and budget. Denied tools are absent from the model-visible schema where the provider supports it, context and MCP catalog token limits are enforced, schema projection never grants authority, and the Effect Firewall rejects forged or stale catalogs. Tool results are validated before insertion; byte/line limits, summary/head-tail selection, truncation reason, original size, content-addressed artifact, retrieval permission, expiry, and classification round-trip without silently discarding evidence.

## Context, LSP, and hybrid-retrieval suite

Context Compiler tests cover collection, filtering, authority ranking, deduplication, compression, provenance labels, budget fitting, validation, stable hashing, and omission reporting. Every context item must retain source, trust, freshness, selection reason, scope, and token estimate.

LSP adapter conformance covers capability discovery, definitions, references, symbols, implementations, call hierarchy, diagnostics, type data, rename/affected-file impact, affected-test mapping, cancellation, restart, stale document versions, malformed server output, and unavailable-server degraded state. Initial adapters are intentionally limited; language-neutral contracts do not imply universal support.

Hybrid retrieval tests compare lexical, LSP, structural AST/tree-sitter, semantic, Git, ADR/spec, evidence, and memory signals. Exact authoritative sources must outrank semantic similarity. Security fixtures prove readable scope/classification is not widened, repository instructions remain untrusted, and vectors/indexes can be deleted and rebuilt without becoming a source of truth.

Mission Exploration Branch tests prove read-only behavior, exact parent/context-pack linkage, bounded permissions/budget, evidence-backed findings, accepted/rejected contribution disposition, and zero mutation of the main execution path. Compaction tests independently rehydrate governance policy, approved specification/Outcome Contract, current approved plan/phase/task versions, unresolved decisions, risks, evidence requirements, and active stop conditions; missing anchors, omitted-item mismatch, or full-source-hash mismatch invalidates the compacted context. Transcript or summary content can never supersede the referenced Core artifacts.

## Controlled-loop suite

Every loop iteration records hypothesis, evidence before, intended action, expected result, actual result, evidence after, confidence delta, and next decision. Fixtures must trigger:

- repeated identical action;
- no observable state change;
- oscillation between states/strategies;
- increasing diagnostics/regression;
- repeated schema failure;
- exhausted token/cost/time/attempt budget;
- unresolved uncertainty;
- invalid or superseded plan;
- required human authority;
- successful convergence after genuinely new evidence.

The expected outcomes—continue, split task, select skill, invoke reviewer/advisor, replan, switch supported profile, clarify, request decision, defer, or stop—are explicit and bounded.

`SandboxFailureResult` fixtures cover boundary, policy/capability reason, attempted effect, retryability, safe alternatives, required authority, recovery state, and evidence reference. An identical denial cannot be repeated unless hypothesis, capability, plan, or authority has materially changed. The Contextual Risk Advisor treatment measures false allow, false block, flapping, latency, interruption reduction, and unsafe-effect escape; it is rejected if it grants, overrides deterministic denial, approves protected effects, changes policy, or becomes the only security boundary.

## Subagent and independent-review suite

- Each subagent receives one objective, scoped context, skills, tools, capabilities, budget, deadline, output schema, and evidence requirements.
- Read-only exploration respects concurrency limits; writable work is isolated and conflict-controlled.
- Communication is structured finding/artifact → Hermes Supervisor → Core-recorded durable result where relevant; no free uncontrolled swarm channel.
- Role inputs conform to the authorized subscription graph; uncontrolled peer broadcast and free agent-to-agent chat are rejected.
- Subagents cannot approve plan/effect/completion, contact the user, expand scope, or inherit secrets/capabilities implicitly.
- Lost, late, malformed, contradictory, or malicious findings remain attributed and cannot mutate truth.
- Review fixtures prove builder → deterministic validation → independent reviewer → test/quality → security where applicable → completion proposal, and that reviewer evidence does not rely solely on builder rationalization.

## Typed-memory suite

Test Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory separately. Memory Candidate/Judge tests cover evidence eligibility, accepted-vs-rejected mission status, project/classification scope, deduplication, contradiction, confidence, freshness, expiry, supersession, retention/deletion, bounded retrieval, and provenance.

No experiential memory may change policy, specification, acceptance criteria, capability, or approval. Governance Memory remains authoritative and cannot be demoted to optional similarity search. Prompt-influence remains a residual risk and must be measured, not denied.

## Early evaluation-spine suite

The ADR-0024 evaluation spine is a prerequisite to H1/H2, not an H8 retrofit. Contract tests cover `EvaluationCase`, `TreatmentArm`, `RunManifest`, `EvaluationRun`, `MetricObservation`, `ProtocolDeviation`, and `EvaluationResult`:

- schema/version/provenance round-trip and explicit compatibility/supersession;
- exact fixture, repository, specification, Outcome Contract, plan, policy, gate, environment, tool, adapter, model/provider, prompt/configuration, skill, data, budget, and randomness/nondeterminism identity;
- immutable raw-run retention with corrections expressed as linked superseding analyses;
- failed, unsafe, inconclusive, aborted, contaminated, and protocol-deviating runs retained without being rewritten as passes;
- deterministic manifest hashing plus explicit missing, unknown, unavailable, and provider-nondeterministic fields;
- metric definition/version, unit, collection method, subject, timestamp, evidence binding, threshold version, and uncertainty;
- versioned milestone baselines selected before comparison, regression linkage, and comparable exports; and
- access, classification, retention, deletion, and secret-redaction rules that preserve useful provenance without retaining prohibited content.

The pre-H1 gate suite covers both permitted foundation paths: an independently approved, version-bound fixture set derived from retained accepted prerequisite contracts, and one derived from an approved replacement-foundation plan. It rejects a missing/unapproved fixture set, a run against the plan artifact instead of derived fixtures, a stale foundation version, incompatible manifest/result evidence, and any implicit promotion of Draft PR #1 traces into the Baseline run. H1 remains unauthorized until at least one selected-path Baseline `EvaluationRun` reaches an honest terminal state (`Completed`, `Failed`, or `Aborted`), is assessed with every required observation validated and deviation dispositioned, and produces a finalized `EvaluationResult`; a failed/aborted result remains valid provenance and is never relabelled as success.

Every H milestone must emit compatible run evidence and compare against a preregistered or governance-approved baseline. Evidence compatibility is a gate: a milestone-specific log or dashboard that cannot reconstruct the case, arm, manifest, observations, deviations, and result is insufficient.

## Staged H3/H4 evaluation gates

- **H3.1 built-in kernel:** prove deterministic skill resolution, capability containment, hook failure isolation, evidence integrity, and measurable workflow benefit using built-in skills only. No external installer, registry, marketplace, or signing-distribution infrastructure is required for this gate.
- **H3.2 packaging:** proceeds only after H3.1 evidence establishes a real distribution problem and governance controls for provenance, compatibility, trust/signature/revocation, install isolation, and rollback pass adversarial testing.
- **H4.1 reproducible context baseline:** compare exact authoritative sources, lexical retrieval, Git history, and supported `rust-analyzer` signals with complete context/manifests, omission reporting, freshness, latency, and degraded-state behavior.
- **H4.2 structural/impact expansion:** proceeds only after adapter conformance, stale-result containment, impact/affected-test accuracy, and incremental value over H4.1 meet approved thresholds.
- **H4.3 semantic retrieval:** proceeds only when controlled ablations show practical benefit over H4.1/H4.2 without authority-ranking, scope, privacy, prompt-injection, reproducibility, cost, or latency regression; otherwise it is narrowed, deferred, or rejected.

## Outcome-equivalence and harness protocol

For each controlled evaluation hold constant:

- mission and repository commit;
- approved specification and Outcome Contract;
- policy, tools, environment, budget class, and maximum attempts;
- required quality/security/evidence gates.

Vary only the declared independent variable: builder model, Brain model, LSP, RAG, memory, loops, subagents, or skill routing. Record exact configuration and random/nondeterministic factors.

For a risk-triggered multi-route race, the treatment may vary the declared plan/builder route while specification, Outcome Contract, repository commit, policy, tools, environment, budget class, scoring gates, and attempt policy remain fixed. Each candidate is scored independently against the contract; model vote, visual preference, and relative rank cannot create acceptance.

Measure outcome-equivalence rate, gate pass, regression, unsafe effects, evidence completeness, attempts to convergence, tokens/cost, wall time, tool calls, human interventions, plan changes, escalation, recovery, and non-convergence honesty. Different code is allowed; a weaker profile may cost more or escalate more often; no profile receives a lower acceptance bar.

Ablation results must include negative or null findings and small-sample uncertainty. Benchmark pass rate alone cannot certify a profile or component. See [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md) and [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).

Reference-derived candidates use RS-00–RS-30 in [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md). The suite must additionally cover unauthorized SOP subscriptions, shared-environment/peer-chat leakage, forged tool catalogs, MCP catalog overflow, branch mutation, compaction authority loss, inaccessible full tool artifacts, repeated denial loops, advisor false allows/flapping, race selection bias, appearance-only acceptance, malicious extensions, Markdown authority attempts, stale indexes, ACP confused-deputy requests, cross-worktree writes, crash/duplicate effects, telemetry/Core disagreement, provider semantic drift, memory poisoning, and source/license provenance. Passing a protocol or vendor benchmark never replaces these outcome, safety, recovery, and evidence tests.

### Explicit negative controls

The reference-system suite must demonstrate explicit rejection, not merely absence, of:

- an uncontrolled shared agent environment or free peer-to-peer agent chat;
- model vote, score rank, or visual preference as acceptance authority;
- optional Core governance or any advisor, extension, package, or adapter that bypasses Core;
- extensions or packages with ambient host authority;
- automatic trust or execution of project-local packages;
- a semantic summary, transcript, branch finding, or tool output replacing authoritative state;
- deployment, SEO, advertising, or business-growth automation in H1–H9; and
- appearance-only acceptance of UI work without deterministic, accessibility, console, network, and interaction evidence required by the Outcome Contract.

## Critical end-to-end scenarios

1. User intent is clarified, approved as immutable specification, planned, approved, executed phase by phase, evidenced, reviewed, and explicitly accepted/returned/deferred/cancelled.
2. A worker requests undeclared filesystem, process, network, secret, dependency, database, protected-Git, external-service, or budget authority; enforcement denies it and records why.
3. A process crashes while an effect is uncertain; recovery probes rather than guesses and avoids silent duplication.
4. Brain/skill/memory/repository content proposes injection, invalid structure, uncited confidence, or scope expansion; validation and authority boundaries contain it.
5. Missing, stale, forged, contradictory, or failing evidence blocks phase/completion gates.
6. A UI task supplies screenshots/video, accessibility tree, console, network trace, and deterministic interaction evidence; an attractive screenshot with failing behavior or accessibility cannot pass.
7. WIP overflow, overlapping writers, lost subagent, stale LSP index, provider outage, disk pressure, and disconnected surface produce accurate degraded state.
8. Two supported model profiles attempt the same accepted fixture; only contract-equivalent outcomes pass, and non-convergent profiles stop honestly.
9. Any material change traces end to end from approved requirement through effect and evidence to completion and memory disposition.

## Gate evidence and test governance

Each result identifies command/toolchain, environment, inputs and versions, policy/profile, expected threshold, actual output, artifact hash, owner, and disposition. A retry produces new linked evidence; it cannot overwrite failure. Fixtures are versioned, classified, reviewable, and use synthetic/sanitized data unless an approved isolated environment requires otherwise.

Before implementation of any H milestone, its test plan names invariant, owner, normal/denied/degraded/recovery/adversarial behavior, `EvaluationCase` and baseline, evidence artifact, performance/security/accessibility implications, regression thresholds, and release gate. Its executions emit ADR-0024-compatible manifests, observations, deviations, and results. If behavior cannot be tested or captured reproducibly, narrow the contract or record an explicit risk decision.

Evaluation decision split: [ADR-0024 — evaluation spine and run provenance](adr/ADR-0024-evaluation-spine-run-provenance.md) and [ADR-0025 — model/profile certification](adr/ADR-0025-model-profile-certification.md).

See also: [14_Security_Architecture.md](14_Security_Architecture.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), [22_Milestones.md](22_Milestones.md), [25_Development_Guidelines.md](25_Development_Guidelines.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), and [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md).
