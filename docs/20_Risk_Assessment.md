# 20 — Risk Assessment

## Risk posture and authority

WePLD combines governed specifications, autonomous execution, proprietary source, external models, retrieval and memory, skills/hooks/plugins, and executive decisions. Uncertainty is durable operational state: risks have provenance, owners, triggers, exposure, controls, residual decisions, evidence, review dates, and affected contract versions.

Risk ownership is not approval authority. An owner monitors and mitigates; only the principal named by governance policy may approve residual risk, policy exceptions, specification/acceptance changes, protected effects, or completion. Core records those decisions and alone commits transitions, budgets, capabilities, effect intent/result, completion, and recovery.

## Top risk register

| Risk | Likelihood / impact | Early trigger | Primary controls | Owner |
| --- | --- | --- | --- | --- |
| Product collapses into an AI IDE or chat | Medium / High | roadmap dominated by editor/chat breadth | H1–H8 runtime gates before H9; outcome/decision/evidence-first surfaces | Product |
| Draft PR #1 is treated as canonical or implied approval | Medium / Critical | planning cites candidate code as merged truth | explicit Baseline Gate, branch provenance, independent decision, no cross-branch implementation | Architecture |
| Baseline disposition deadlocks H1 | Medium / Critical | a returned, deferred, or rejected PR #1 is treated as a permanent prerequisite failure | close the gate as `Resolved`; require either accepted candidate prerequisite contracts or an approved replacement-foundation plan covering every gap | Architecture/Core |
| Authority leakage or self-approval | High / Critical | Brain/Hermes/worker/tool claims approval, transition or completion | formal hierarchy, distinct verbs/records, Core-only writer, adversarial authority tests | Core/Security |
| Approved specification is silently changed | Medium / Critical | task/plan differs from WHAT without change record | immutable approved versions, spec-vs-plan Change Requests, trace validation | Product/Core |
| Incorrect change-request classification | Medium / High | HOW request alters criteria/scope or WHAT request bypasses impact | typed impact analysis, Core rejection, authorized review | Architecture |
| Unsafe tool execution or prompt injection | High / Critical | undeclared action, capability mismatch, egress attempt | isolated worktrees, complete Effect Firewall, default deny network, probes | Security |
| Real-world effect is duplicated or left uncertain | Medium / Critical | timeout/crash between execution and receipt | durable intent, idempotency, observed postcondition probe, `Uncertain`, no unsafe retry | Core |
| Sensitive context leaks to provider/channel | Medium / Critical | classification/destination/redaction mismatch | context minimization, brokered credentials, egress policy, provenance, audit | Security/Privacy |
| Context Compiler omits or mistranks critical evidence | Medium / High | missed impacted symbol/test or unexplained ranking | exact/LSP/policy precedence, selection reasons, validation, reproducible packs | Hermes/Quality |
| Semantic RAG or prompt content overrides authority | Medium / Critical | semantic result conflicts with spec/policy/source | trust ranking, contradiction detection, authoritative-source pinning | Security/Hermes |
| LSP evidence is incomplete or wrong | High / High | unsupported language, stale index, divergent diagnostics | support tiers, freshness, fallback exact analysis, uncertainty/escalation | Tooling |
| Skill or hook becomes a policy escape path | Medium / Critical | undeclared tool, effect-producing hook, reentrancy/deadlock | signed contracts, isolation, typed hook class, recursion/time limits, re-enter firewall | Security/Hermes |
| Unbounded subagent swarm or write conflict | Medium / High | free-form chatter, duplicate writes, budget fan-out | one objective, structured handoff, bounded read parallelism, isolated writes, WIP | Hermes/Core |
| SOP or role topology becomes a shadow control plane | Medium / Critical | role self-subscribes, free chat/shared environment changes work, or a peer message is treated as authority | Core-projected typed `SOPGraph` and `AuthorizedRoleSubscriptionGraph`; authorized artifact/event subscriptions only; one-writer/effect rules; no free shared environment | Core/Hermes |
| Capability-projected tool schema and actual grant diverge | Medium / Critical | a model sees or invokes an operation/resource absent from its task capability | compile `CapabilityProjectedToolCatalog` from exact grants; version-bind schema, task and capability; Effect Firewall revalidates every call; fail closed on drift | Security/Core |
| Controlled loop oscillates or makes no progress | High / High | repeated action, no state change, rising diagnostics/schema failures | hypothesis ledger, loop guards, retry/budget caps, replan/escalation/stop | Hermes/Quality |
| Sandbox denial or risk advice is mistaken for authority | Medium / Critical | denial text causes privilege-seeking retry, or contextual advice overrides policy/risk decision | typed `SandboxFailureResult` with no capability gain; advisory-only, evaluated `ContextualRiskAdvisor`; Core policy and authenticated decisions remain final | Security/Core |
| Model unreliability causes incorrect change | High / High | schema failures, disagreement, rework, unsupported confidence | structured outputs, deterministic checks, independent review, fixed gates | Quality |
| Structurally valid but unfit plan is approved | Medium / Critical | schema/DAG checks pass while architecture, risk, evidence, recovery, or proportionality is weak | durable `PlanAssessment`; risk-tier review; reviewer independence; authenticated `PlanDecision`; producer cannot approve or be sole critical reviewer | Architecture/Quality |
| False outcome equivalence lowers quality for a model | Medium / Critical | profile-specific waiver or superficial pass | fixed Outcome Contract, independent evidence, same gates, unresolved-risk threshold | Quality/Product |
| Model non-convergence is hidden | Medium / High | repeated retries, optimistic summary, unexplained profile switch | explicit convergence state, escalation ladder, honest-stop metric | Quality/Hermes |
| Provider/model drift invalidates certification | High / High | behavior/cost/schema/safety shift | exact profile fingerprints, continuous fixtures, quarantine/recertification | Quality/Registry |
| Memory poisoning, staleness, or authority confusion | Medium / Critical | uncited lesson, cross-scope leak, Governance Memory treated as advice | typed memory, Memory Judge, scope/freshness/expiry/contradiction/supersession | Knowledge/Security |
| Exploration or compaction corrupts governed context | Medium / Critical | an exploration branch is promoted silently, mandatory authority disappears, or a summary cannot be traced to raw inputs | non-authoritative `MissionExplorationBranch`; explicit promotion gate; `CompactionRecord`; mandatory-source pin/rehydration; H7 Memory Judge for retained learning | Hermes/Knowledge |
| Evidence is fabricated, stale, incomplete, or circular | Medium / Critical | producer self-validates, missing environment/hash, same context only | Evidence Requirements/Bundles, independent validation, provenance, freshness | Quality |
| Tool output floods context, injects instructions, or leaks data | High / Critical | unbounded stdout/result enters a prompt, bypasses classification, or loses raw provenance | bounded `BoundedToolResult`; classified content-addressed `ToolOutputArtifact`; sanitized excerpts and truncation metadata; exact producer/task/effect links | Security/Hermes |
| Premature phase/mission completion | Medium / Critical | `Done` or worker success interpreted as acceptance | distinct task/phase/proposal/decision states, Core gate validation | Core/Product |
| Kanban/WIP starvation or decision backlog | Medium / High | growing blocked/approval queues, idle critical path | policy WIP, aging/escalation, capacity/budget signals, explicit unblock decisions | Delivery |
| Runaway model/cost/resource consumption | Medium / High | burn rate, fan-out or CPU above envelope | reservations, quotas, WIP, rate limits, scheduler backpressure, mandatory stop | Core/Hermes |
| State loss or corrupt recovery | Medium / Critical | projection mismatch, lost events/artifacts, uncertain leases | ledger/snapshots, integrity checks, backups, replay and recovery scenarios | Core |
| Plugin/skill/model supply-chain compromise | Medium / Critical | advisory, signature mismatch, permission change | signing/hash, isolation, SBOM/advisories, evaluation, quarantine/revocation | Security/Registry |
| Reference code/fixture enters without provenance or compatible license | Medium / Critical | copied example, unknown/mixed license, missing revision/notice | RS-00, clean-room default, repository licensing policy, component-level provenance review | Legal/Architecture |
| ACP or another client protocol becomes an alternate control plane | Medium / Critical | client plan/permission/tool call mutates state or executes directly | Core-mediated adapter, fail-closed semantic mapping, RS-06/RS-20, new Proposed ADR at H9 | Core/Security |
| Cross-platform sandbox guarantees diverge | High / High | control unavailable on supported OS | posture tests, support tiers, reduced autonomy, visible gaps | Core/Security |
| Harness evaluation is contaminated or gamed | Medium / High | fixture leak, uncontrolled variable, benchmark-only optimization | fixed manifests, blinded/held-out scenarios, ablation, safety/evidence metrics | Evaluation |
| Evaluation is irreproducible | Medium / High | missing commit/tool/profile/config/seed | immutable run manifest, artifact hashes, environment capture, rerun policy | Evaluation |
| Evaluation arrives too late to establish a trustworthy baseline | Medium / Critical | H8 retrofits telemetry after H1–H7 choices and cannot reconstruct exact provenance | operational ADR-0024 spine before H1/H2; versioned cases/arms/manifests/runs/observations/deviations/results; baseline and regression evidence at every H milestone | Evaluation/Core |
| Multi-route race multiplies effects, cost, or selection bias | Medium / Critical | routes share writable state, exceed budget, leak findings, or a producer cherry-picks an unevaluated winner | H8-only `ControlledMultiRouteRace`; fixed contract/allocation/join; isolated or read-only routes; Effect Firewall; deterministic scoring; cancellation and complete loser evidence | Evaluation/Hermes |
| Platform breadth outruns measured need and governance proof | Medium / High | registries, third-party packages, broad LSP/AST/semantic retrieval, or signing infrastructure enter the minimum kernel | H3.1 built-ins before optional H3.2 packaging; H4.1 reproducible exact/lexical/Git/rust-analyzer baseline before optional H4.2/H4.3; preregistered benefit and rollback gates | Architecture/Security |
| UI overstates progress, team activity, freshness, or authority | Medium / High | visual team/route status or green completion lacks source version and evidence | H9 projection-only Execution Console and Engineering-Team UX; every view resolves to `EvidenceBundle`, Core event and artifact provenance; staleness/uncertainty visible | Studio/Quality |
| Markdown and structured state diverge | Medium / High | exported spec edited and treated as canonical | Core state banner/version/hash, one-way projection/import validation | Core/Documentation |
| Sync/enterprise complexity overwhelms local proof | Medium / High | remote work precedes recovery/authority tests | defer sync, local single writer, later explicit gate | Architecture |
| Regulatory, residency, or retention mismatch | Medium / High | provider/storage/export conflicts with policy | classification, configurable retention, legal review, enterprise policy | Legal/Privacy |

## Risk classification by action

| Class | Examples | Default treatment |
| --- | --- | --- |
| Low | read authorized local metadata, analyze existing evidence, draft/propose a plan | autonomous proposal within scope; never autonomous approval |
| Moderate | isolated worktree edit, local deterministic validation, Memory Candidate | Core-issued capability, evidence and WIP control; risk may rise with repository trust |
| High | dependency/toolchain change, network/model egress, database migration, push/PR proposal, budget expansion | protected policy path and usually explicit approval |
| Critical | secrets, destructive Git/filesystem action, merge/public release, production deployment, policy exception, acceptance-bar/specification change | named human/enterprise decision; no autonomy-mode bypass |

Classification is contextual. A read may expose Restricted data; a local test may execute an untrusted repository; a “documentation” change may alter policy. Core determines risk from action, resource, reversibility, data, environment, actor, governing versions, mission mode, and effect scope. Autonomy mode changes routing, never the underlying authority hierarchy or final quality bar.

## Risk propagation and operational response

A `RiskItem` traces to affected specification requirements, outcome criteria, plan phases, Task Packets, evidence requirements, effects, decisions, and completion. If its trigger fires, Core may block task admission, phase closure, effect dispatch, plan validity, or completion according to policy. Hermes may recommend mitigation or replanning but cannot accept residual risk.

Invalidated assumptions and execution evidence can challenge a plan. They do not silently edit it: WHAT impact opens a Specification Change Request; HOW-only impact opens a Plan Change Request. Until the replacement is approved, affected work becomes blocked, returned, deferred, or uncertain.

Risk with no credible mitigation blocks the affected capability or contract, not necessarily unrelated work. A non-converging model stops or escalates without a lowered gate. A suspected compromise revokes affected capabilities and preserves recovery evidence.

## Residual decisions and milestone gates

Before H1 entry, named authorities decide supported platform/sandbox claims, candidate PR #1 disposition, local trust boundary, initial provider data handling, default effect envelope, resource ceilings, and minimum baseline evidence. The Baseline Gate becomes **Resolved** only when it records both the disposition and an executable prerequisite path: accepted candidate contracts, or an approved replacement-foundation plan covering every missing prerequisite. `Return`, `Defer`, or `Reject` cannot become a permanent H1 blocker.

Before H1/H2 implementation, Proposed ADR-0024 must be accepted and the Early Evaluation Spine must capture exact `EvaluationCase`, `TreatmentArm`, `RunManifest`, `EvaluationRun`, `MetricObservation`, `ProtocolDeviation`, and `EvaluationResult` provenance. Every H milestone establishes a versioned baseline, reports regressions, and preserves compatible runs. Before each later implementation increment, its applicable Proposed ADR and preceding gate must close. Decisions include: specification and completion authority; PlanProposal qualification and PlanDecision review tiers; H2 `SOPGraph` and authorized role-subscription design; Phase/Kanban/WIP defaults; hosted-profile classification limits; H3.1 built-in skill/hook trust, capability-projected tool catalogs and bounded output artifacts; any later H3.2 packaging boundary; H4.1 initial context/rust-analyzer support, exploration branches and compaction records; any later H4.2 structural or H4.3 semantic expansion; H5 typed sandbox failure semantics and whether the Contextual Risk Advisor experiment earns any use; H6 role/subagent parallelism; memory admission; loop budgets; and outcome-equivalence thresholds. H8 certification under Proposed ADR-0025 consumes accumulated spine evidence and adds controlled cross-model, repeated/randomized, ablation, multi-route-race, and independent-review evidence; it does not invent provenance retrospectively. H9 visual team/execution views remain projections of Core events, artifacts, and `EvidenceBundle` records.

H9 requires runtime state, evidence, degraded behavior, accessibility, and authority semantics to be stable enough for product surfaces. Enterprise, sync, marketplace, and production deployment require later separate decisions.

## Review cadence and escalation

Owners review high/critical risks before each gate/release, when a new external/authority boundary is introduced, after an incident, on provider/package drift, when evidence invalidates an assumption, and when a change request affects exposure. Messenger may route an urgent redacted notice but cannot disclose restricted detail or resolve the risk.

Suppression, exception, and residual acceptance record rationale, authority, expiry, affected versions, and evidence. They do not delete findings or retroactively change past gate truth.

## Acceptance criteria

- Every material risk has identity, source, owner, trigger, exposure, controls, residual state, review date, and linked contract/control evidence.
- Risk owner, policy evaluator, approval authority, executor, and completion decision-maker remain distinguishable.
- Autonomy modes map to explicit action-risk rules; drafting/proposing never implies approval.
- New model, provider, skill, hook, retriever, compactor, exploration branch, SOP/role topology, capability-projection rule, memory type, subagent role, route racer, plugin, worker, sync, effect class, or execution/team view cannot ship without threat-model, risk-register, and evaluation updates.
- Completion cannot be accepted with a blocking risk, missing evidence, unresolved uncertain effect, or a profile-specific reduction in the Outcome Contract.
- PR #1 remains candidate, unmerged, and unratified until its own authorized disposition; if not accepted, H1 proceeds only through an approved replacement-foundation plan rather than treating that disposition as a permanent blocker.

See [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [02_Product_Principles.md](02_Product_Principles.md), [10_Loop_Engineering.md](10_Loop_Engineering.md), [14_Security_Architecture.md](14_Security_Architecture.md), [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [22_Milestones.md](22_Milestones.md), and [28_Release_Strategy.md](28_Release_Strategy.md). Proposed ADRs 0015–0025 remain risk-reduction proposals, not implementation authority.
