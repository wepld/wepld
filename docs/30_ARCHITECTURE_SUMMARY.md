# 30 — Architecture Summary

## Executive decision

Build WePLD as a **desktop-first, local-first governed engineering control plane with a native specification-to-acceptance method and Hermes as its first-party Engineering Intelligence Runtime**.

WePLD provides the engineering method. The user provides the desired outcome. The Brain Agent proposes a governed delivery plan. Hermes operates bounded engineering work. Builder models execute task packets. WePLD Core governs every transition and effect. Evidence determines whether the result is acceptable.

The strategic promise is:

> **Different brains. Same engineering truth.**

Supported models may take different implementation paths. WePLD promises neither byte-identical code nor equal model capability. It promises that accepted outputs satisfy the same approved specification, Outcome Contract, policy, architecture, quality, security, regression, evidence, and unresolved-risk thresholds. A model that cannot converge within its bounds stops or escalates honestly; the quality bar is never reduced.

## Repository and candidate-baseline truth

Canonical `main` contains the architecture/master-plan package; no implementation is canonical there. Draft PR #1 is an open, Draft, unmerged, unratified candidate Build Feature baseline. It is reference evidence only. This planning package:

- does not modify, approve, merge, or mark PR #1 ready;
- does not treat its branch-local “Accepted/FROZEN/READY/M0 complete” labels as canonical decisions;
- preserves its useful candidate contracts without claiming they are merged;
- requires an independent **Build Feature Baseline Gate** before any Hermes Intelligence implementation;
- requires the relevant Proposed ADR-0015 through ADR-0024 to be accepted and the preceding H gate to close before dependent implementation.

## Authority hierarchy

~~~text
1. WePLD governance policy
2. Approved Engineering Specification
3. Outcome and Acceptance Contract
4. Approved Delivery Plan
5. Approved Phase Plan
6. Builder Task Packet
7. Tool action
~~~

No lower layer may silently redefine a higher layer.

| Actor | May do | May not do |
| --- | --- | --- |
| User/Founder | state outcomes, approve specification/plan/protected effects/completion, resolve real decisions | bypass policy/evidence through a surface shortcut |
| Core | own durable state, policy, approvals, capabilities, budgets, WIP, transitions, evidence requirements, completion, recovery | fabricate human approval or accept model prose as evidence |
| Brain Agent | propose architecture, risks, phased plan, tasks, verification, replans | approve its plan, execute effects, change approved WHAT |
| Hermes Supervisor | maintain bounded attempt state, route skills/models/subagents/context, coordinate loops | own governance truth, grant capabilities, accept completion |
| Builder | implement one bounded task packet and report artifacts/evidence | expand scope, alter acceptance, approve itself |
| Explorer/reviewer/test/security subagents | return scoped structured findings and evidence | chat as a swarm, inherit authority, mutate mission truth |
| Tool/effect boundary | enforce granted operation, probe result, record evidence | infer approval from prompt text or execute undeclared effects |

## Core architecture

~~~mermaid
flowchart LR
  H["Human executive"] --> S["CLI / Studio / MCP / API\nsame Core workflow"]
  S --> C["WePLD Core\ngovernance • state • policy • WIP\napprovals • budgets • evidence • recovery"]
  C --> G["Specification & Delivery\ncharter • outcome contract • plans\nphases • Kanban • change control"]
  C --> X["Hermes Supervisor\nskills • hooks • context • loops • subagents"]
  X --> B["Brain Agent / builder profiles\nprovider-neutral reasoning"]
  X --> T["Effect Firewall + isolated tools/worktrees"]
  C --> E["Ledger + CAS + Git\nauthoritative facts and evidence"]
  C --> M["Typed Memory + Memory Judge"]
  E --> Q["Projections / Mission Control / evaluation"]
~~~

The essential separation is: **brains reason; Hermes supervises bounded execution; builders and subagents produce artifacts; Core governs; policy authorizes; tools enforce; evidence verifies; the user decides where authority requires.**

## Native user workflow

1. Describe the desired outcome.
2. Clarify unresolved questions.
3. Review and approve the Engineering Specification and Outcome Contract.
4. Review and approve the Delivery Plan.
5. Execute tailored phases through durable Kanban/WIP control.
6. Observe evidence, risks, budgets, and flow.
7. Resolve genuine decisions and controlled change requests.
8. Review verified completion.
9. Accept, return, defer, or cancel.
10. Consolidate eligible Engineering Memory through the Memory Judge.

The user operates outcomes, decisions, risks, and reports—not low-level tool commands. Every caller uses the same Core commands and authority.

## Structured domain truth

The architecture defines versioned contracts for at least:

`MissionCharter · EngineeringSpecification · OutcomeContract · DeliveryPlan · PhasePlan · TaskPacket · RiskItem · Assumption · DecisionRequest · ChangeRequest · EvidenceRequirement · EvidenceBundle · CompletionProposal · CompletionDecision · MemoryCandidate · Retrospective`

Each declares purpose, authority, required fields, lifecycle, versioning, provenance, validation, proposer/approver, trace links, and authoritative/derived/untrusted class. Markdown is a review/export projection, never the sole source of truth.

The required trace is:

~~~text
user intent
  → specification requirement
  → Outcome Contract
  → plan phase
  → task packet
  → required evidence
  → completion decision
  → memory candidate / judge disposition
~~~

## Hermes Engineering Intelligence Runtime

Hermes is not a thin LLM wrapper and not a second control plane. Its planned internal capabilities are:

1. **Agent Kernel** — bounded objective/phase/task/hypothesis/observation/retry/confidence/next-action state.
2. **Skill Runtime and Router** — versioned executable procedures with context, tools, capabilities, verification, failures, evidence, compatibility, and trust.
3. **Context Compiler** — collect, filter, authority-rank, deduplicate, compress, provenance-label, fit, validate, and capture minimal packs.
4. **LSP Intelligence** — language-neutral normalized symbols/references/diagnostics/impact with a deliberately limited initial adapter set.
5. **Hybrid Code Retrieval** — lexical, LSP, structural AST/tree-sitter, semantic, Git, ADR/spec, evidence, and memory signals; authoritative sources outrank vectors.
6. **Controlled Loop Engine** — observe→diagnose→hypothesize→minimal action→execute→verify→compare→update→continue/replan/escalate/stop, with non-progress guards.
7. **Typed Hook Bus** — observational, validating, blocking, and effect-producing hooks; no policy-bypass plugin path.
8. **Subagent Supervisor** — bounded specialist assignments, read-only parallelism, isolated writes, structured handoffs, no free swarm.
9. **Independent Self-Review** — builder→deterministic validation→reviewer→test/quality→security where applicable→completion proposal.
10. **Effect Firewall** — proposed effect→classification→policy→capability→approval where required→durable intent→execute→probe→evidence.
11. **Typed Memory** — Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory with judge-controlled consolidation.

Durable mission truth remains in Core throughout.

## Model-independent outcome convergence

The Outcome Equivalence Contract compares accepted outputs across:

- functional behavior and acceptance criteria;
- public/API/data contracts;
- approved architecture and scope constraints;
- security policy and unsafe-effect threshold;
- build/test/review/regression quality gates;
- evidence completeness and unresolved-risk threshold.

Formatting beyond repository rules, internal strategy, attempt count, model/tool sequence, and non-contractual style need not match. The escalation ladder is improved context → specialized skill → task split → reviewer/advisor → replan → supported profile switch → clarification → human decision → safe stop.

The harness holds mission, repository commit, approved specification, policy, tools, environment, budget class, and maximum attempts constant. It varies one declared component/profile and measures equivalence, gate pass, regressions, unsafe effects, evidence, convergence, cost/time, interventions, plan changes, escalation, recovery, and honest non-convergence. Ablations cover LSP, RAG, memory, loops, subagents, and skill routing.

## Compatibility with Draft PR #1

| Classification | Candidate baseline examples | Planning treatment |
| --- | --- | --- |
| **Supporting** | command outcomes including `Deferred`; ledger/CAS/provenance; structured spec seed; mission criteria/gates/budget; WWP no-human/no-state-mutation rule; provider-neutral gateway; worktrees/snapshots/proposal refs; validated paths/ids/refs; explicit plan/completion decisions; bounded untrusted lessons | retain semantics if baseline is accepted |
| **Needs extension** | specification lifecycle; minimal plan/task; reduced WWP; empty skills; narrow context; generic evidence/decision/memory shapes | add H1–H8 contracts through accepted ADRs and migration/coexistence plans |
| **Unchanged boundary** | Core authority; Brain proposes; worker reports; no fabricated human actor; proposal ref rather than merge; memory never grants authority; evidence before completion | preserve as non-negotiable |
| **Temporary V0** | Build Feature only; no specification-approval gate; deterministic single `T1`; default `src/**`; DEV no containment/Manual/fixtures; loopback-only provider; role `stub`; empty skills; no LSP/RAG/hooks/subagents/self-review/UI; local fingerprint memory; Linux/WSL2 focus | disclose honestly; do not generalize or present as H capability |
| **Must wait** | all H1–H9 implementation and contract/vocabulary changes | baseline accepted, relevant ADR accepted, preceding gate closed, explicit next authorization |

Candidate documents that describe auto-merge, Bounded-Auto under V0, full OS sandboxing, full Context Assembly/WWP/reviewer isolation, or branch-local “frozen/approved” status must not override the final narrow V0 contract or canonical planning decisions.

## Delivery sequence

The original M0–M7 sequence remains historical lineage. Current execution gates are:

1. **Build Feature Baseline Gate:** independently accept, return, defer, or reject the candidate prerequisite; never implicit merge approval.
2. **H1:** governed specification and outcome contract.
3. **H2:** Brain planning, phase/Kanban/WIP, and controlled change.
4. **H3:** Hermes Agent/Skill Kernel and typed hooks.
5. **H4:** Context Compiler, limited LSP adapters, and authority-ranked hybrid retrieval.
6. **H5:** bounded engineering loops and escalation.
7. **H6:** supervised subagents and independent self-review.
8. **H7:** typed memory and Memory Judge.
9. **H8:** model-independent outcome convergence, profile certification, and ablations.
10. **H9:** runtime-truth-first product surfaces—Mission Control, Spec Review, Plan Review, Kanban, Decisions, Risks, Evidence, Change Requests, Completion Review.

No milestone authorizes the next merely by finishing code. Each needs accepted normal/failure/security/recovery evidence and explicit next authorization as defined in [22_Milestones.md](22_Milestones.md).

## Proposed ADR package

All are **Proposed**, not Accepted:

1. [ADR-0015 — governed specification contract](adr/ADR-0015-governed-specification-contract.md)
2. [ADR-0016 — Brain-plan/builder-execution separation](adr/ADR-0016-brain-plan-builder-execution-separation.md)
3. [ADR-0017 — phase, Kanban, and controlled change](adr/ADR-0017-phase-kanban-controlled-change.md)
4. [ADR-0018 — Hermes Skill Runtime and Hook Bus](adr/ADR-0018-hermes-skill-runtime-hook-bus.md)
5. [ADR-0019 — Context Compiler, LSP, and hybrid retrieval](adr/ADR-0019-context-compiler-lsp-hybrid-retrieval.md)
6. [ADR-0020 — typed memory and Memory Judge](adr/ADR-0020-typed-memory-memory-judge.md)
7. [ADR-0021 — bounded subagents and structured handoffs](adr/ADR-0021-bounded-subagents-structured-handoffs.md)
8. [ADR-0022 — controlled loop and escalation](adr/ADR-0022-controlled-loop-escalation.md)
9. [ADR-0023 — model-independent outcome equivalence](adr/ADR-0023-model-independent-outcome-equivalence.md)
10. [ADR-0024 — harness evaluation and provider certification](adr/ADR-0024-harness-evaluation-provider-certification.md)

Numbering avoids collision with the candidate branch’s ADR-0001–ADR-0014 and does not ratify those unmerged records.

## Scope and deferred systems

H1–H9 target one user, one local project, bounded local execution, limited initial languages/adapters/profiles, explicit approvals, and reviewable proposal refs. Open marketplace distribution, remote fleets, cloud-first control planes, universal language/model support, uncontrolled swarms, cross-customer learning, autonomous production deployment, and full IDE breadth remain deferred.

## Document map

Documents 01–20 retain product, system, component, security, data/event/API, roadmap, and risk foundations. Documents 21–30 define delivery, technology, repository, engineering, testing, performance, release, future, and summary governance. The focused extension package is:

- [31 — Governed Specification Workflow](31_Governed_Specification_Workflow.md)
- [32 — Hermes Engineering Intelligence Runtime](32_Hermes_Engineering_Intelligence_Runtime.md)
- [33 — Model-Independent Outcome Convergence](33_Model_Independent_Outcome_Convergence.md)
- [34 — Harness Evaluation Protocol](34_Harness_Evaluation_Protocol.md)
- [35 — Reference Systems and Competitive Architecture](35_Reference_Systems_and_Competitive_Architecture.md)

Document 35 supplies the evidence boundary for reference-informed decisions: Spec Kit informs the typed Delivery Protocol; Pi informs composable Hermes primitives; Zed/ACP informs future editor interoperability; Warp informs bounded execution operations; Claude Code, Codex, Cursor, OpenCode, Aider and OpenHands supply additional controlled treatments. None becomes a foundation or roadmap item by analogy. RS-00–RS-20, Core authority, license/provenance review and the H1–H9 gates decide admission.

## Architecture-gate conclusion

The architecture package is ready for review, not implementation. The next action is to return it to architecture review, disposition the Proposed ADRs, and independently decide the Build Feature Baseline Gate. No Hermes Intelligence implementation, source change, PR mutation, tag, push, release, marketplace, remote fleet, or production deployment is authorized by these documents.
