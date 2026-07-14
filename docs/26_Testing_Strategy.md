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

For `MissionCharter`, `EngineeringSpecification`, `OutcomeContract`, `DeliveryPlan`, `PhasePlan`, `TaskPacket`, `RiskItem`, `Assumption`, `DecisionRequest`, `ChangeRequest`, `EvidenceRequirement`, `EvidenceBundle`, `CompletionProposal`, `CompletionDecision`, `MemoryCandidate`, and `Retrospective`, test:

- schema/version/provenance round-trip;
- valid lifecycle and every invalid transition;
- proposer/approver authority;
- authoritative, derived, or untrusted classification;
- supersession and compatibility;
- missing/stale/contradictory references;
- trace chain from user intent → specification requirement → outcome contract → phase → task → evidence → completion decision → memory candidate.

The trace must be complete by reference and version, not inferred later from prose.

## Phase, Kanban, WIP, and change-control suite

- Phase entry requires all dependencies, authority, budget, allowed skills/tools, scope, and risk controls.
- Task flow supports Backlog, Ready, InProgress, Review, Verification, Done plus Blocked, NeedsClarification, NeedsApproval, Returned, Deferred, Uncertain, and Cancelled.
- Done requires valid exit evidence; Blocked/Deferred/Uncertain never render as Done.
- Default one writable task per isolated worktree; bounded read-only parallelism; policy-configured limits on decisions and pending protected effects.
- Concurrent writers with overlapping scope are refused or isolated/conflict-controlled.
- Plan evidence cannot silently change specification truth. Change classification, approval, invalidation, replan, and resume are replayable.

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

## Context, LSP, and hybrid-retrieval suite

Context Compiler tests cover collection, filtering, authority ranking, deduplication, compression, provenance labels, budget fitting, validation, stable hashing, and omission reporting. Every context item must retain source, trust, freshness, selection reason, scope, and token estimate.

LSP adapter conformance covers capability discovery, definitions, references, symbols, implementations, call hierarchy, diagnostics, type data, rename/affected-file impact, affected-test mapping, cancellation, restart, stale document versions, malformed server output, and unavailable-server degraded state. Initial adapters are intentionally limited; language-neutral contracts do not imply universal support.

Hybrid retrieval tests compare lexical, LSP, structural AST/tree-sitter, semantic, Git, ADR/spec, evidence, and memory signals. Exact authoritative sources must outrank semantic similarity. Security fixtures prove readable scope/classification is not widened, repository instructions remain untrusted, and vectors/indexes can be deleted and rebuilt without becoming a source of truth.

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

## Subagent and independent-review suite

- Each subagent receives one objective, scoped context, skills, tools, capabilities, budget, deadline, output schema, and evidence requirements.
- Read-only exploration respects concurrency limits; writable work is isolated and conflict-controlled.
- Communication is structured finding/artifact → Hermes Supervisor → Core-recorded durable result where relevant; no free uncontrolled swarm channel.
- Subagents cannot approve plan/effect/completion, contact the user, expand scope, or inherit secrets/capabilities implicitly.
- Lost, late, malformed, contradictory, or malicious findings remain attributed and cannot mutate truth.
- Review fixtures prove builder → deterministic validation → independent reviewer → test/quality → security where applicable → completion proposal, and that reviewer evidence does not rely solely on builder rationalization.

## Typed-memory suite

Test Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory separately. Memory Candidate/Judge tests cover evidence eligibility, accepted-vs-rejected mission status, project/classification scope, deduplication, contradiction, confidence, freshness, expiry, supersession, retention/deletion, bounded retrieval, and provenance.

No experiential memory may change policy, specification, acceptance criteria, capability, or approval. Governance Memory remains authoritative and cannot be demoted to optional similarity search. Prompt-influence remains a residual risk and must be measured, not denied.

## Outcome-equivalence and harness protocol

For each controlled evaluation hold constant:

- mission and repository commit;
- approved specification and Outcome Contract;
- policy, tools, environment, budget class, and maximum attempts;
- required quality/security/evidence gates.

Vary only the declared independent variable: builder model, Brain model, LSP, RAG, memory, loops, subagents, or skill routing. Record exact configuration and random/nondeterministic factors.

Measure outcome-equivalence rate, gate pass, regression, unsafe effects, evidence completeness, attempts to convergence, tokens/cost, wall time, tool calls, human interventions, plan changes, escalation, recovery, and non-convergence honesty. Different code is allowed; a weaker profile may cost more or escalate more often; no profile receives a lower acceptance bar.

Ablation results must include negative or null findings and small-sample uncertainty. Benchmark pass rate alone cannot certify a profile or component. See [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md) and [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).

Reference-derived candidates use RS-00–RS-20 in [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md). The suite must cover malicious extensions, Markdown authority attempts, compaction constraint loss, stale structural/index results, ACP confused-deputy requests, cross-worktree writes, crash/duplicate effects, telemetry/Core disagreement, provider semantic drift, memory poisoning, and source/license provenance. Passing a protocol or vendor benchmark never replaces these outcome, safety, recovery, and evidence tests.

## Critical end-to-end scenarios

1. User intent is clarified, approved as immutable specification, planned, approved, executed phase by phase, evidenced, reviewed, and explicitly accepted/returned/deferred/cancelled.
2. A worker requests undeclared filesystem, process, network, secret, dependency, database, protected-Git, external-service, or budget authority; enforcement denies it and records why.
3. A process crashes while an effect is uncertain; recovery probes rather than guesses and avoids silent duplication.
4. Brain/skill/memory/repository content proposes injection, invalid structure, uncited confidence, or scope expansion; validation and authority boundaries contain it.
5. Missing, stale, forged, contradictory, or failing evidence blocks phase/completion gates.
6. WIP overflow, overlapping writers, lost subagent, stale LSP index, provider outage, disk pressure, and disconnected surface produce accurate degraded state.
7. Two supported model profiles attempt the same accepted fixture; only contract-equivalent outcomes pass, and non-convergent profiles stop honestly.
8. Any material change traces end to end from approved requirement through effect and evidence to completion and memory disposition.

## Gate evidence and test governance

Each result identifies command/toolchain, environment, inputs and versions, policy/profile, expected threshold, actual output, artifact hash, owner, and disposition. A retry produces new linked evidence; it cannot overwrite failure. Fixtures are versioned, classified, reviewable, and use synthetic/sanitized data unless an approved isolated environment requires otherwise.

Before implementation of any H milestone, its test plan names invariant, owner, normal/denied/degraded/recovery/adversarial behavior, evidence artifact, performance/security/accessibility implications, and release gate. If behavior cannot be tested, narrow the contract or record an explicit risk decision.

See also: [14_Security_Architecture.md](14_Security_Architecture.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), [22_Milestones.md](22_Milestones.md), [25_Development_Guidelines.md](25_Development_Guidelines.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), and [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md).
