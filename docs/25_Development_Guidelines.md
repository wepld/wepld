# 25 — Development Guidelines

## Purpose and authorization

These guidelines apply only after the relevant architecture decision and milestone gate authorize implementation. Draft PR #1 is a candidate Build Feature prerequisite, not canonical implementation and not permission to begin Hermes Intelligence work. A fast feature that bypasses authority, loses provenance, weakens acceptance, or couples an adapter to domain truth is a defect.

## Authority hierarchy

Every implementation must preserve this order:

1. WePLD governance policy
2. Approved Engineering Specification
3. Outcome and Acceptance Contract
4. Approved Delivery Plan
5. Approved Phase Plan
6. Builder Task Packet
7. Tool action

No lower layer may silently redefine a higher layer. The Brain Agent proposes plans; it cannot approve them. Hermes coordinates attempt work; it does not own durable governance truth. Builders and subagents produce changes, findings, and evidence; they cannot change scope, approve effects, or accept completion. Core owns policy, authenticated approvals, capability issuance, budgets, transitions, evidence requirements, completion state, and recovery truth.

## Contract-first delivery

1. Name the user outcome, authoritative artifact/version, acceptance contract, milestone, and affected bounded contexts.
2. Confirm the preceding gate is closed and every applicable Proposed ADR is accepted.
3. Decide whether the change affects WHAT, HOW, or only an internal implementation. WHAT requires a Specification Change Request; HOW requires a Plan Change Request; a contract-preserving refactor requires neither.
4. Define or revise structured contracts, lifecycle invariants, authority, provenance, compatibility, and migration before adapter/UI behavior.
5. Write normal, denied, degraded, recovery, adversarial, and performance/evaluation fixtures.
6. Implement deterministic domain behavior and effect adapters behind ports.
7. Demonstrate machine-verifiable evidence, independent review, security checks, and documentation/traceability updates.

## Design rules

- Keep governance and domain invariants deterministic, typed, explicit, and independently testable.
- Treat Markdown, prompts, summaries, and dashboards as projections or untrusted inputs—not the sole source of truth.
- Make effects proposed, classified, policy-evaluated, capability-checked, durably intended, executed through a boundary, probed, evidenced, and recoverable.
- Preserve envelope containment as the blast-radius boundary for arbitrary toolchains; do not pretend every subprocess effect can be intercepted individually.
- Prefer bounded artifacts and references over free-form cross-component prose.
- Store large or sensitive bodies as classified artifacts; ledger/state records carry hashes, provenance, and minimum required facts.
- Model unavailable, stale, uncertain, deferred, contradicted, and partially completed states explicitly.
- Keep Brain, Hermes, builder, subagent, policy, tool, evidence, and human-decision roles separate.
- A different model may use a different implementation strategy, but the approved acceptance, architecture, security, quality, and evidence bar never changes.

## Phase, Kanban, and WIP rules

- A phase starts only when its entry conditions, dependency states, budget, skills/tools, writable scope, and risk controls are valid.
- Kanban transitions are Core-owned state transitions, not UI drag-and-drop side effects.
- One writable implementation task per isolated worktree is the default; bounded read-only research may run in parallel.
- Pending protected effects, unresolved decisions, and writable attempts count against policy-configured WIP.
- A blocked, returned, deferred, or uncertain task cannot be reported as done.
- Evidence that invalidates HOW opens controlled replanning; evidence that changes WHAT opens a new specification version.

## Skill, hook, context, and memory rules

- A skill is a versioned executable procedure with declared applicability, context, tools, capabilities, verification, failure modes, output schema, evidence, compatibility, and trust—not prompt text alone.
- Hook types declare whether they are observational, validating, blocking, or effect-producing. Blocking/effect hooks use the same Core policy and effect firewall; no hidden plugin path exists.
- Context packs record source, trust, freshness, selection reason, scope, token estimate, omissions, compiler/strategy version, and hash.
- Exact policy/spec/ADR/repository/LSP evidence outranks structural and semantic retrieval; vectors never replace authoritative sources.
- Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory remain distinct. Only the Memory Judge may consolidate eligible candidates; experiential memory never grants authority.

## Security requirements

- Never log, commit, render, or transmit secrets outside a brokered policy path.
- Treat model output, repository content, channel input, skill/pack content, memory, LSP/tool output, hooks, and external responses as untrusted at their boundaries.
- Validate identifiers, paths, refs, URLs, schemas, plans, task packets, findings, and evidence before they reach an effect or durable authoritative state.
- Add or alter a provider, network destination, sandbox control, package, LSP server, parser, vector store, hook, skill, or data flow only with threat-model review and policy documentation.
- Record denied and uncertain effects without falsely asserting execution or rollback.
- Workers/subagents never directly modify protected branches, publish packages, push, open/merge PRs, deploy, access secrets, or approve their own exception unless a separately authorized workflow expressly grants and verifies that effect.

## Review requirements

Reviewers verify correctness, authority hierarchy, traceability, failure/recovery, race/idempotency, WIP/change semantics, performance/resources, security/classification, observability, user disclosure, compatibility, documentation, and tests. A reviewer must trace a material effect from approved requirement → plan/phase/task → policy/capability/approval → execution → evidence → completion decision without relying on author or model narrative.

Builder context must not be the only basis for review. Required flow is builder → deterministic validation → independent reviewer → test/quality review → security review where risk requires → completion proposal. These outputs are evidence, never automatic approval.

## Change and compatibility management

Use small, reversible changes. Every mission/task/attempt, artifact, context pack, skill, model/profile, toolchain, and effect records its version and provenance. Contract changes declare additive versus breaking behavior, coexistence window, migration, historical-reader/upcaster needs, and rollback/repair instructions.

An approved specification is immutable. Supersession creates a new version and preserves the prior version. A retry produces new evidence linked to earlier evidence; it never overwrites a failure. Feature flags and policy presets have an owner, review/expiry, and removal plan and cannot disable hard safety gates.

## Quality and model-independent acceptance

No completion claim is accepted without the task’s required build, static, test, review, security, accessibility, performance, documentation, and evidence-completeness results. Accepted outputs from different supported models need not be byte-identical, but must be contract-equivalent across functional behavior, public contracts, architecture, security, regression behavior, evidence, and unresolved-risk thresholds.

A model that cannot converge within the approved attempts, context, and budget must escalate or stop honestly. It may not lower the quality bar. Flaky tests, opaque retries, ignored findings, unexplained regressions, evidence gaps, and non-convergence disguised as success are tracked defects.

See also: [14_Security_Architecture.md](14_Security_Architecture.md), [22_Milestones.md](22_Milestones.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), and [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md).
