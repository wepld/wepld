# 32 — Hermes Engineering Intelligence Runtime

**Status:** Proposed architecture for review. Hermes Intelligence implementation is not authorized by this document.

## Identity and invariant

Hermes is WePLD's governed **Engineering Intelligence Runtime**: the supervisor that turns an approved Phase Plan into bounded engineering work. It is not a model provider, a general chat assistant, a durable workflow database, or an autonomous authority.

The runtime separation is:

- **Core owns truth:** governance, approved artifacts, policy, capabilities, budgets, transitions, effect authorization, evidence requirements, completion, and recovery.
- **Brain Agent plans:** it uses replaceable brain profiles to draft specifications, delivery strategies, phase graphs, risks, and replans; it cannot approve its own output or perform effects.
- **Hermes supervises execution:** it maintains bounded operational state, routes skills/models/subagents, compiles context, runs controlled loops, and proposes effects and completion.
- **Builder models execute Task Packets:** they reason within the packet and return structured action proposals and artifacts; they do not receive ambient host authority.
- **Tool boundaries perform effects:** only after Core policy, capability, approval, and durable-intent checks.

Durable mission truth always remains in Core. If Hermes stops, restarts, or is replaced, the mission can be reconstructed from Core records without trusting Hermes's private state.

~~~mermaid
flowchart LR
  Spec["Approved Specification + Outcome Contract"] --> Brain["Brain Agent\nplan proposal"]
  Brain --> Core["WePLD Core\nvalidate • approve • authorize • record"]
  Core --> Phase["Approved Phase Plan"]
  Phase --> Hermes["Hermes Supervisor"]
  Hermes --> Router["Skills • models • subagents • context"]
  Router --> Builder["Bounded builder/reviewer packets"]
  Builder --> Proposal["Typed actions • artifacts • findings"]
  Proposal --> Firewall["Core Effect Firewall"]
  Firewall --> Tools["Tool boundaries"]
  Tools --> Evidence["Observed result + evidence"]
  Evidence --> Core
~~~

## Brain Agent architecture

The Brain Agent is a Core-governed role implemented through one or more replaceable brain profiles. Provider selection cannot change its authority.

### Inputs

- approved Engineering Specification and Outcome Contract;
- repository map plus file/language/build metadata;
- LSP symbol, reference, call, diagnostic, and dependency graph;
- applicable ADRs, governance policy, and architecture constraints;
- Git history and current diff/worktree state;
- current diagnostics and prior verified Evidence Bundles;
- typed Engineering and Governance Memory;
- available, compatible Hermes skills and measured skill performance;
- supported brain/builder profiles and measured profile performance;
- budgets, deadlines, WIP policy, and sandbox posture;
- current risks, assumptions, decisions, and uncertainty.

### Structured output

The proposal contains delivery strategy, tailored phase graph, task decomposition, requirement-to-phase/task/evidence traceability, dependency edges, risks and mitigations, required skills/tools, writable and forbidden scopes, verification strategy, decisions needing human authority, estimates and budgets, and stop/escalation conditions.

Core validates schema, authorization, DAG integrity, trace coverage, budget feasibility, policy compatibility, evidence sufficiency, scope monotonicity, WIP feasibility, and architecture rules before presenting the plan. A plan is never approved by the same Brain Agent invocation that produced it. Execution evidence may justify a controlled Plan Change Request; it never permits silent replanning.

## 1. Agent Kernel

The Agent Kernel holds only the operational state required for the active packet:

`objective`, `phase_id`, `task_id`, `packet_version`, `active_hypothesis`, `observations[]`, `selected_strategy`, `unresolved_questions[]`, `retry_state`, `confidence`, `budget_remaining`, `capabilities`, and `next_proposed_action`.

Kernel state is bounded, serializable, and disposable. Material observations, loop iterations, artifacts, findings, and decisions are reported to Core. Hidden transcripts, provider-local memory, or process state never become recovery truth. On restart, Core issues a new packet or recovery packet from durable state; Hermes does not guess.

## 2. Skill Runtime

A skill is an executable, versioned engineering procedure—not prompt text. Its manifest includes:

| Contract area | Required fields |
| --- | --- |
| Identity | name, semantic version, publisher, content hash, signature/provenance, license, trust tier, status |
| Applicability | task/change types, languages, frameworks, risk classes, phase/role compatibility, exclusions |
| Context | required authoritative sources, repository signals, LSP capabilities, memory classes, freshness bounds |
| Tools | required tool adapters and versions, allowed capabilities/effects, network/secrets needs, sandbox tier |
| Procedure | ordered steps, inputs, preconditions, branching rules, bounded retry strategy, expected outputs |
| Verification | deterministic checks, reviewer independence, evidence requirements, thresholds, negative cases |
| Failure | failure classes, safe cleanup, stop/escalation behavior, partial-result schema |
| Output | schema IDs for actions, artifacts, findings, summaries, and Evidence Bundles |
| Compatibility | Core/Hermes/WWP/schema ranges, supported platforms and toolchains |
| Evaluation | fixture suite, baseline metrics, known limitations, last certification and expiry |

The runtime resolves an exact hash-pinned skill version. A procedure may invoke only declared tool ports through an issued capability. Its own tests and claimed permissions cannot override mission policy. Expanded capabilities, changed verification, or a major output schema require a new version and review.

Initial skill families are repository exploration, architecture analysis, Rust, TypeScript, Python, debugging, test planning/generation, security review, dependency analysis, API/schema design, database migration, performance analysis, documentation, Git forensics, and recovery investigation. Initial releases support only the families and language adapters proven at their milestone gate; the architecture does not claim universal coverage.

## 3. Skill Router

For each Task Packet, the router selects a tuple:

`skill version + brain profile + builder profile + subagent role + tool set + context strategy + budget + verification level`.

Hard filters run first: policy, data classification, platform/sandbox posture, packet capabilities, compatibility, required evidence, provider residency, and budget. Eligible routes are then ranked by measured task-family success, outcome-equivalence rate, unsafe-action rate, schema reliability, evidence completeness, cost, latency, context needs, retry history, and freshness of evaluation.

The routing policy is versioned, explainable, replaceable, and recorded with the attempt. A model or skill cannot select itself by self-reported competence. Missing or inconclusive performance data produces a conservative candidate route or an explicit decision—not a fabricated score.

## 4. Context Compiler

The Context Compiler produces minimal, task-specific, reproducible context packs:

`Collect → authorize/filter → rank → deduplicate → compress → provenance-label → fit budget → validate → capture → send`

Every `ContextItem` carries `source_ref`, `source_version/hash`, `source_kind`, `trust_level`, `authority_level`, `classification`, `freshness`, `scope`, `selection_reason`, `retrieval_method`, `estimated_token_cost`, `included/excerpted/omitted`, and `supersession_state`.

Context tiers are:

1. **Mandatory authority:** applicable governance policy, approved specification/outcome/plan/packet, binding ADRs, and output/evidence schemas. These are never displaced by retrieval score.
2. **Exact task state:** current diagnostics, changed files, exact symbols, relevant tests, prior phase results, and unresolved decisions.
3. **Structural repository evidence:** LSP/AST relationships, module boundaries, build graph, Git history, and architecture records.
4. **Verified engineering evidence and memory:** current Evidence Bundles and approved, scoped, fresh memory.
5. **Semantic candidates:** similarity results used only to discover candidates, never to override exact or authoritative sources.

The compiler records all considered items and omission reasons, including policy exclusion and budget pressure. It fails loudly if mandatory authority does not fit or a required source is stale/unavailable. Packs are content-addressed and every brain invocation records the exact pack hash and compiler/ranking version.

## 5. LSP Intelligence

Hermes uses a language-neutral LSP broker owned behind a Core port. Initial adapters may include `rust-analyzer`, Pyright, TypeScript language server, and `gopls`, but only adapters that pass compatibility and fixture tests are supported.

The broker normalizes:

- definitions, declarations, references, symbols, implementations, and type information;
- call/type hierarchy and import/dependency edges;
- diagnostics with source, severity, range, version, and freshness;
- rename/impact analysis and affected-file sets;
- affected-test mappings with confidence and deriving rule.

LSP output is observational evidence, not authority. Every result records adapter/server/version, repository commit, document versions, request, time, completeness, timeout/degraded state, and provenance. Unsupported language features, stale indexes, generated code, macros, dynamic dispatch, and partial workspaces remain explicit limitations. LSP informs planning, context selection, scope re-checks, review, and verification; unavailable LSP degrades visibly or blocks when the Outcome Contract requires it.

## 6. Hybrid Code RAG

Retrieval combines complementary sources under one evidence contract:

- exact lexical/path/identifier search;
- LSP symbol and relationship retrieval;
- structural AST/tree-sitter retrieval;
- semantic retrieval;
- Git history and blame/commit rationale;
- ADR, specification, plan, and policy retrieval;
- current and historical evidence retrieval;
- typed Engineering Memory retrieval.

The ordering rule is **authority before relevance, exact before approximate, current before stale, structural before semantic, and verified before inferred**. Semantic vectors may widen recall but cannot displace an applicable policy, approved contract, exact symbol match, LSP fact, or structural edge. Each result exposes independent lexical, structural, semantic, freshness, trust, authority, and scope signals; a composite rank without those components is not acceptable evidence.

Repository content and external results are untrusted data and are fenced/labeled before model use. Retrieval is authorization-filtered and logged because context selection can change behavior. Poisoned, superseded, unauthorized, or contradictory items are excluded or visibly marked; they are never silently reconciled by model confidence.

## 7. Typed memory and Memory Judge

Memory classes have different authority and retention semantics:

| Class | Contents | Durability / authority |
| --- | --- | --- |
| Working Memory | Current hypothesis, observations, scratch state | Attempt-scoped and disposable; never governance truth. |
| Mission Memory | Decisions, phase summaries, risks, evidence, attempts for one mission | Durable through Core; bounded to mission scope. |
| Engineering Memory | Verified repository lessons, patterns, findings, conventions | Consolidated only from evidence-derived candidates; advisory within explicit scope. |
| Skill Memory | Measured procedure outcomes and failure modes by task family/version | Derived evaluation record; routes skills but grants no capability. |
| Provider/Model Performance Memory | Schema reliability, convergence, cost, latency, escalation and unsafe-action rates | Derived evaluation record; routes profiles but never lowers gates. |
| Governance Memory | Policy, accepted ADRs, approved specs/outcomes/plans and supersession | Authoritative, mandatory when applicable, never optional retrieval advice. |

The `MemoryJudge` processes a `MemoryCandidate` through schema validation, source/evidence verification, authorization and classification, scope derivation, deduplication, contradiction detection, confidence calibration, freshness/expiry assignment, security/prompt-injection checks, and required human/policy review. Outcomes are approve, reject, quarantine, merge-as-new-version, or supersede. Consolidation never edits an existing authoritative record silently.

Memory retrieval enforces project/repository/branch/path/language/skill/profile scope, classification, freshness, confidence, contradiction, and token limits. Raw prompts, secret material, unverifiable model advice, failed-attempt conclusions, and cross-project content do not become Engineering Memory merely because they were useful once.

## 8. Controlled Loop Engine

Each execution loop is explicit and bounded:

`Observe → diagnose → hypothesize → select minimal action → propose/execute → verify → compare expected and observed → update belief → continue, replan, escalate, or stop`

Every iteration records:

- hypothesis and confidence before;
- evidence/diagnostics before;
- intended action and affected scope;
- expected result and falsification condition;
- policy/capability/approval references;
- actual result and evidence after;
- diagnostic and confidence delta;
- cost/time/tool usage;
- next decision and rationale.

Loop guards detect repeated identical or equivalent actions, no observable state change, oscillation between states, increasing diagnostics or blast radius, exhausted budget/deadline/retry cap, repeated schema failures, unchanged hypothesis, unresolved uncertainty, invalid/superseded plan, stale context, and required human authority. A guard stops the affected packet and records `Blocked`, `Uncertain`, or an escalation; it does not manufacture progress.

Retries require a named changed hypothesis, new evidence, or a changed approved strategy. Evidence that challenges WHAT creates a Specification Change Request; evidence that changes HOW creates a Plan Change Request. The loop cannot perform either change itself.

## 9. Hook Bus

Hooks are typed lifecycle integrations registered through Core policy. Each hook has identity/version, event schema, class, priority/order, timeout, idempotency, capability request, data classification, output schema, failure policy, and evidence contract.

| Class | May do | Must not do |
| --- | --- | --- |
| Observational | Emit telemetry, derived findings, or artifacts after receiving an authorized view. | Block, mutate state, or produce effects. |
| Validating | Return a schema-valid pass/fail/finding used by Core policy or a gate. | Commit the transition it evaluates. |
| Blocking | Veto or require approval before Core proceeds, with an explicit policy/validation reason. | Grant approval, widen scope, or perform the blocked effect. |
| Effect-producing | Propose a typed effect that re-enters the Effect Firewall as a new action intent. | Execute through a side channel or treat hook invocation as authorization. |

Required lifecycle points include `mission_received`, `before/after_specification`, `before/after_context_compile`, `before/after_brain_call`, `before/after_tool_call`, `before/after_file_write`, `before/after_test`, `diagnostics_changed`, `before_snapshot`, `before_phase_gate`, `before_completion`, `failure_detected`, `recovery_started`, and `mission_closed`.

Observational `after_*` hooks cannot rewrite facts. `before_*` validating/blocking hooks run under bounded time and fail according to explicit policy; protected boundaries fail closed. Reentrancy depth, recursion, duplicate delivery, ordering conflicts, and hook failure are guarded and recorded. Hooks have no direct database, secret, filesystem, network, or model access beyond issued ports.

## 10. Subagent Supervisor

Hermes may supervise specialized, bounded subagents: Repository Explorer, Architecture Analyst, Implementer, Test Engineer, Security Reviewer, Performance Reviewer, Documentation Agent, and Recovery Investigator.

Each receives exactly one objective, scoped context pack, allowed skills/tools/capabilities, budget, deadline, output schema, evidence requirements, read/write classification, and stop/escalation conditions. It returns a structured `Finding`, `Artifact`, `ActionProposal`, or `EvidenceBundle` to Hermes. Hermes validates the handoff and asks Core to record any result with durable relevance.

Communication is never uncontrolled peer chat:

`Subagent → structured artifact/finding → Hermes Supervisor → Core-recorded result when durable`

Parallelism rules are:

- bounded read-only exploration may run in parallel under a shared phase budget;
- writable work uses isolated worktrees and one writer per scope unless Core proves disjoint footprints;
- reviewer/test/security contexts exclude builder rationalization unless explicitly required as labeled evidence;
- no subagent may approve a plan, capability, effect, exception, gate, or completion;
- cancellation, timeout, loss, partial results, and conflicting findings remain visible and are reconciled by policy or escalation, not by majority vote.

## 11. Independent self-review

The default review chain is:

`Builder → deterministic validation → independent reviewer → test/quality reviewer → security review when applicable → Completion Proposal`

The same model/profile and context may not be the sole basis for both implementation and acceptance-critical review. Core selects independence appropriate to risk: fresh context, separate attempt, different role/skill, deterministic checks, and—where justified—different supported profile. Review findings do not mutate the implementation; remediation is a new Task Packet with traceable disposition. A green reviewer recommendation cannot replace required test, security, architecture, or evidence gates.

## 12. Effect Firewall and guardrails

Every real-world or durable effect follows:

`Agent proposes → classify → evaluate policy → check capability → obtain approval if required → record durable intent → execute at boundary → probe actual result → record evidence/result → reconcile state`

| Effect class | Minimum enforcement and evidence |
| --- | --- |
| Files and workspaces | Validated relative paths, realpath/symlink protections, write allowlist, isolated worktree, diff/snapshot probe. |
| Processes and shell | Structured command/args/cwd/env, executable/toolchain identity, sandbox/resource/time limits, captured exit/logs. |
| Git | Validated refs, expected-old compare-and-swap, protected-ref policy, base/worktree probe, before/after commits. |
| Network and external transfer | Destination/scheme/purpose/data classification, egress approval, credential mediation, request/receipt evidence. |
| Secrets | Broker reference, purpose, narrow expiry, non-disclosure/logging controls, access receipt; no raw secret in model context. |
| Dependencies/toolchains | Package/source/version/integrity/license/advisory, lockfile impact, install scripts/network class, rollback. |
| Databases | Target/environment, transaction/migration plan, backup/rollback, affected rows/schema, postcondition probe. |
| Push, pull request, merge | Repository/ref identity, remote, actor, checks, expected commit, protected workflow, returned URL/SHA; each is a distinct effect. |
| Deployment/release | Environment, artifact provenance/signature, approval, rollout/health/rollback, post-deploy evidence. |
| Budget/model calls | Profile, provider, classification, exact context hash, projected/actual usage, retry/fallback policy, result schema. |

Prompt wording is not enforcement. Guardrails live at Core transitions, capability issuance, adapter/tool boundaries, sandbox controls, schema validators, and post-effect probes. An uncertain external result is recorded as `Uncertain` and investigated; it is never converted to success because a call returned without an error.

## Escalation and safe stopping

Hermes follows the common escalation ladder: improve context once under the same contract; select a more specialized skill; split the packet; invoke a bounded reviewer/advisor; propose a plan change; switch to a certified supported profile; request clarification; request a human decision; stop safely. It may skip directly to the required authority when risk or policy demands it.

A model that cannot produce schema-valid, evidence-backed work under the fixed quality bar must stop honestly. The system records partial artifacts, uncertainty, budget consumed, attempted routes, and the minimum decision needed to continue.

## Candidate V0 relationship

Draft PR #1 provides useful candidate seams: a Core-owned ledger/CAS, provider-neutral gateway, WWP process boundary, `AttemptStart` envelope/skills/context/budget shape, staged plan/completion approvals, isolated worktrees, proposal refs, validation, and a narrow evidence-derived lesson loop. Its Hermes binary is a V0 single-phase worker with narrow packs, a reduced message set, empty skill routing, DEV-tier limitations, no LSP/RAG/hooks/subagent supervisor, and no generalized controlled-loop or independent review pipeline. Those are temporary candidate limitations, not proof of this target architecture.

No Hermes Intelligence implementation begins until the applicable Proposed ADRs are accepted and the preceding milestone gate in [22_Milestones.md](22_Milestones.md) is closed.

See also: [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [05_Worker_Architecture.md](05_Worker_Architecture.md), [06_Brain_Architecture.md](06_Brain_Architecture.md), and [14_Security_Architecture.md](14_Security_Architecture.md).
