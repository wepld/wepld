# 18 — API Architecture

## One Core workflow

WePLD exposes stable, versioned ports rather than internal storage. CLI, Studio, MCP, Messenger/integrations, and future public APIs call the same Core command/query/subscription workflow and receive the same authority, policy, revision, evidence, and transition semantics. No surface owns a shortcut implementation or a second source of truth.

V1 is local-first: approved clients and adapters communicate with the long-lived Core over authenticated local IPC/RPC. A network public API is deferred, but domain contracts are transport-neutral. Structured schemas and Core records—not Markdown payloads, UI state, or database tables—are authoritative.

## Authority invariant

Core alone commits durable policy, approvals, capabilities, budgets, governance/workflow state, effect intent/result, completion, and recovery. The Brain Agent proposes specifications and `PlanProposal` records; deterministic Core services compile normalized candidates and initial assessments; independent reviewers record separate immutable review evidence; Core finalizes a Ready `PlanAssessment`; an authorized user issues a `PlanDecision`; Hermes requests supervised transitions and routing; builders/subagents return actions, artifacts, findings, and evidence; tool/provider boundaries return observed effects. API verbs preserve those distinctions: `Propose`, `Submit`, `Compile`, `Assess`, `Request`, `Approve`, `Issue`, `Report`, `Validate`, and `Decide` are not interchangeable.

Every executable request resolves the version chain:

`Policy > approved EngineeringSpecification > approved OutcomeContract > approved DeliveryPlan > approved PhasePlan > authorized TaskPacket > ToolAction`

## Interface classes

| Interface | Consumers | Semantics |
| --- | --- | --- |
| Command API | CLI, Studio, MCP adapter, Messenger, authorized automation | submit typed intent; return accepted/rejected/awaiting/deferred, never optimistic mutation |
| Query API | all authorized surfaces/reporting | read field-filtered projections with versions, authority, freshness, and gaps |
| Event subscription | CLI/Studio streams, projection workers, Messenger | resumable authorization-filtered stream of committed Core facts/deltas |
| Governed-artifact port | Brain Agent, review surfaces, Core validators | propose/version artifacts; deterministically compile candidates; record structural validation and initial assessments; record separate independent reviews; finalize Ready assessments and authenticated decisions; no producer-side approval |
| Hermes supervisor port | Hermes runtime | obtain approved phase/task envelopes; request leases/transitions; report routing, loops and structured results |
| Worker/subagent protocol | approved execution hosts | register, accept lease, heartbeat, propose actions, return artifacts/findings/attempt result; never write task/mission state |
| SOP compiler/projection port | Core SOP compiler, Hermes requests, RoleNodes | deterministically compile/validate exact approved parents; authorize subscriptions; Core-project role inputs; validate outputs/control edges; no role ledger query or peer channel |
| Exploration/context-continuity port | Hermes, Brain/reviewer/explorer roles, Core context service | authorize read-only MissionExplorationBranches, dispose contributions, record compaction and rehydrate current authority before resume |
| Brain/builder port | Brain Gateway adapters | provider-neutral structured request/result with context/evidence/budget contract |
| Intelligence ports | Skill Runtime, Hook Bus, Context Compiler, LSP/retrieval/memory adapters | typed scoped input/output and provenance; no direct governance or effect write |
| Tool/effect port | mediated tool/provider/sandbox boundaries | project capability-bounded ToolCatalogManifest; validate capability; execute exact intent; return BoundedToolResult/ToolOutputArtifact/SandboxFailureResult and probe evidence |
| Evaluation/visual-evidence port | evaluation harness, approved capture/comparison boundaries, independent reviewers | govern ControlledMultiRouteRace and evaluation linkage; record reproducible visual capture/comparison without self-acceptance |
| Registry port | package hosts/admin surfaces | discover, stage, evaluate, request activation, health, quarantine, revoke |
| Integration/MCP port | channels, service adapters, MCP servers | normalize untrusted inbound intent; mediated outbound delivery and credential use |
| Agent/client adapter port | ACP or other editor/terminal agent clients | negotiated projections and typed requests mapped to Core commands/events; no direct effect or authority |

## Transport and schema decision

V1 uses versioned local IPC/RPC over OS-appropriate Unix domain sockets or named pipes, with authenticated per-user sessions and strict filesystem permissions. Loopback TCP is development-only and still authenticated. No unauthenticated localhost control endpoint is acceptable.

JSON-Schema-compatible contracts are the portable interchange representation. An implementation may use a compact typed transport internally only when generated/validated schemas, error types, enum values, and compatibility tests remain authoritative. Future remote APIs may add mTLS/OIDC and a gateway while preserving the same commands, queries, events, approvals, and capabilities. Transport never leaks into domain authority.

## Command model

Commands include named intent, idempotency key, authenticated caller, project/mission scope, expected revision where relevant, payload schema version, client correlation ID, and exact governing artifact versions. Core returns:

| Outcome | Meaning |
| --- | --- |
| `Accepted` | command is durably accepted; resulting facts will appear through events/projections |
| `Rejected` | schema, identity, authority, policy, governing-version, WIP, budget, or concurrency failure, with safe reason |
| `AwaitingApproval` | command is valid but an exact `DecisionRequest` must be answered by named authority |
| `Deferred` | command is valid but capacity/dependency/readiness prevents processing; no effect occurred |

`Accepted` does not mean approved, executed, verified, or completed unless the named command and returned event explicitly state that fact.

Representative workflow commands include:

- `DescribeMission`, `SubmitClarification`, `SubmitSpecificationForReview`, `ApproveEngineeringSpecification`, `ApproveOutcomeContract`;
- `SubmitPlanProposal`, `CompilePlanCandidate`, `ValidatePlanCandidate`, `RecordPlanAssessment`, `SubmitIndependentPlanReview`, `DecidePlan`, `ApprovePhasePlan`;
- `SubmitSpecificationChangeRequest`, `SubmitPlanChangeRequest`, `DecideChangeRequest`;
- `ProposeTaskPacket`, `RequestTaskTransition`, `RequestPhaseTransition`, `SetWipPolicy` (Core validates and authorizes the packet);
- `CompileSOPGraph`, `ValidateSOPGraph`, `ActivateSOPGraph`, `AuthorizeInputSubscription`, `RevokeInputSubscription`, `ProjectRoleInput`, `SubmitRoleOutput`, `RecordSOPControlEdgeState`;
- `AuthorizeMissionExplorationBranch`, `SubmitExplorationFindings`, `DecideExplorationContribution`, `RecordCompaction`, `RehydrateCompactedAuthority`;
- `OpenDecisionRequest`, `AnswerDecisionRequest`, `SubmitRiskItem`, `InvalidateAssumption`;
- `ProjectToolCatalogManifest`, `RevokeToolCatalogManifest`, `ProposeToolAction`, `DecideEffectApproval`, `ReportBoundedToolResult`, `RecordToolOutputArtifact`, `ReportSandboxFailure`, `ReportEffectObservation`, `SubmitEvidenceBundle`, `ValidateEvidenceBundle`;
- `RecordVisualEvidenceCapture`, `SubmitVisualComparisonResult`, `ValidateVisualComparisonResult`;
- `SubmitCompletionProposal`, `DecideCompletion`, `SubmitMemoryCandidate`, `RecordMemoryJudgment`;
- `ApproveEvaluationCase`, `ProposeControlledMultiRouteRace`, `AuthorizeControlledMultiRouteRace`, `ReportRaceRouteResult`, `CancelRaceRoute`, `AssessControlledMultiRouteRace`, `FreezeTreatmentArm`, `FreezeRunManifest`, `RegisterEvaluationRun`, `StartEvaluationRun`, `RecordMetricObservation`, `ValidateMetricObservation`, `RecordProtocolDeviation`, `DispositionProtocolDeviation`, `ReportEvaluationRunTerminal`, `AssessEvaluationRun`, `FinalizeEvaluationResult`, `ChangeProfileCertification`.

Only the role named by policy may invoke approval, issuance, validation, or decision commands. The plan API enforces `Brain Agent → PlanProposal → deterministic compiler/normalization → candidate DeliveryPlan → structural validation → initial PlanAssessment → independent review when policy requires → finalized Ready PlanAssessment → authenticated PlanDecision → approved DeliveryPlan`. A low-risk candidate may proceed from deterministic validation to a `Ready` assessment and authorized user decision when exact policy requires no review set. Medium/high risk first enters `ReviewRequired`; independent architecture, quality and security reviewers named by the exact policy/risk-tier version create separate immutable review records, then Core finalizes a new assessment version bound to their exact IDs/versions/hashes. The decision command binds that final assessment and the same exact reviews. The proposal producer cannot decide or be the sole acceptance-critical reviewer. Model votes are findings, not authority. Alternative-proposal commands become required only when risk, uncertainty, a material architectural choice or failed assessment triggers them; the API does not require multiple plans routinely.

Evaluation commands enforce `EvaluationCase → TreatmentArm + RunManifest → EvaluationRun → MetricObservation/ProtocolDeviation → EvaluationResult`. A run cannot start without a frozen manifest containing exact fixture/source and repository hashes, governing versions, provider/model/profile/adapter/settings, prompt/context/skill/tool versions, environment, supported seed, budgets, and manifest creation/freeze/allocation timestamps. Observed run start/end are recorded on `EvaluationRun`, not predicted in the frozen manifest. `ReportEvaluationRunTerminal` accepts only the typed `Completed`, `Failed`, or `Aborted` outcome with evidence; `AssessEvaluationRun` follows required observation validation and deviation disposition. A result cannot finalize with undisposed deviations, unvalidated required observations, or an unassessed run.

Brain Agent cannot call its own plan-decision path; Hermes cannot approve phase/task authority; executors cannot call task/phase/mission completion transitions; a tool boundary can report only the exact effect tied to its active capability.

SOP compilation/validation/activation, role input projection, control-edge state, capability-catalog projection, contribution disposition, and authority rehydration are internal Core operations even when another component requests them. The API binds exact approved parent hashes and compiler/schema versions. A role receives only `RoleInputProjection` values permitted by its active `InputSubscription`, submits only its `OutputContract`, and has no verb for self-subscription, peer broadcast/free chat, shared-environment observation, direct ledger/artifact query, or contract widening.

Exploration commands bind a read-only parent/objective/ContextPack hash, permissions and budget; contribution acceptance names the exact findings/evidence admitted downstream. Compaction resume fails unless Core rehydrates current governing authority and exposes source hash plus omissions. Controlled race commands bind isolation, fixed outcome/context, budgets, cancellation, selection metric, independent evaluator and complete evaluation-spine refs; candidate selection never approves a plan, task, effect, evidence bundle or completion.

Commands never expose raw database queries, direct ledger append, client-assigned authoritative state, or generic “run arbitrary action.” An offline surface may queue an untrusted command envelope but cannot reserve authority, WIP, budget, or perform effects before Core accepts it.

## Query and subscription model

Queries are projection-specific, field-level authorization-filtered, paginated, and return schema/version, governing record versions, cursor, freshness, classification, and source availability. Contract views expose trace links from user intent through requirement, outcome, PlanProposal/candidate/assessment/decision, phase, task, evidence, completion decision, and memory candidate. Evaluation views expose case, treatment arm, frozen manifest, run, metric observations, deviations, result, baseline/regression comparisons and downstream policy decisions without dropping provenance.

A RoleNode is not a general Query API consumer. It receives immutable Core-produced `RoleInputProjection` batches for an active authorized `InputSubscription`; each batch carries source refs/hashes, cursor/range, redactions, omissions and governing versions. Revocation or stale parents/capabilities stops delivery. A `ToolCatalogManifest` query returns only the current capability-projected catalog for that exact subject/task and remains derived, revocable and non-authorizing.

Subscriptions resume from a durable cursor and are filtered by principal and scope. Clients receive explicit `ResyncRequired`, `CursorExpired`, `SchemaIncompatible`, or `AccessChanged` signals. A client never fabricates intermediate state from an event it is not authorized to see; Core may provide a redacted transition projection.

Role replay cannot exceed its authorized cursor/range or become ledger/artifact-store access. A compacted session receives its `CompactionRecord` plus freshly rehydrated policy/specification/outcome/plan/phase/task/decision/capability/SOP refs; missing or mismatched authority produces `ResyncRequired` rather than prompt-based continuation.

## Capability and effect protocol

Human/API callers authenticate to identity scope and are authorized by role, project, artifact authority, and policy. Hermes, workers, subagents, hooks, plugins, and tool/provider boundaries use short-lived Core-issued capabilities binding subject, exact action, resources, classification, conditions, budget, governing versions, task/correlation context, and expiry.

For one RoleNode and task, Core may project a content-hashed `ToolCatalogManifest` from the intersection of its `ActionContract` and active capabilities. The manifest is discovery metadata, not a bearer token or permission; stale, revoked or omitted entries fail closed.

The API preserves the Effect Firewall sequence:

`propose → classify → policy → capability → approval → durable intent → execute → probe → evidence`

The execution boundary validates the active capability immediately before effect, then reports a size-bounded `BoundedToolResult` with truncation, resource/timing, postcondition, errors and artifact refs. Large/binary content is a content-addressed `ToolOutputArtifact`. Denial, violation, timeout, crash, resource exhaustion or possible partial effect is a `SandboxFailureResult` and may enter `Uncertain`; it cannot exchange a token for a broader lease, append an event, assert workflow success, hide failure/truncation, or retry an uncertain non-idempotent effect without new Core direction.

## Errors, versions, and deprecation

Errors are typed, non-sensitive, correlated, and actionable: validation, authentication, authorization, policy denial, approval required, reviewer-independence failure, incomplete assessment, SOP non-determinism/stale parent, subscription/projection denial, compaction rehydration mismatch, stale governing version, conflict, WIP/readiness, unavailable dependency, quota/budget, sandbox failure/timeout/resource exhaustion, uncertain effect, race isolation/allocation failure, visual-provenance failure, evaluation-provenance/deviation failure, evidence failure, or internal fault.

API schemas use semantic versions with additive minor changes. Breaking changes require a new major port/version and coexistence window. Adapters declare supported versions and capability sets at registration. Deprecation emits a structured warning, replacement target, and deadline. Artifact schema migration never mutates an approved historical record; it creates a validated representation/version with provenance.

## External integrations, webhooks, and MCP

External channels and tools communicate through adapters, not internet-exposed Core-domain endpoints. Inbound payloads authenticate origin, normalize identity, classify and rate-limit content, preserve provenance, and enter as untrusted intent. A chat message cannot assert an approval, evidence result, or completion without the relevant authenticated command and authority.

Outbound calls use a transactional delivery queue, idempotency/deduplication, classification/redaction, retries, and receipts. No callback is trusted to assert execution success without a probe/evidence contract. MCP resources and tool calls pass through the same authorization and Effect Firewall; MCP is a surface/adapter, not an alternate control plane.

ACP and comparable editor/terminal-agent protocols follow the same rule. Session IDs and plans are projections; capability negotiation describes a client envelope but cannot mint a Core capability. Filesystem, terminal, MCP and other tool requests are denied unless losslessly mapped to an authorized Core action. Transport, identity or data-flow adoption requires a new Proposed ADR under H9.

See [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [05_Worker_Architecture.md](05_Worker_Architecture.md), [06_Brain_Architecture.md](06_Brain_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), [16_Data_Model.md](16_Data_Model.md), [17_Event_System.md](17_Event_System.md), and [36_Engineering_Committee.md](36_Engineering_Committee.md) — whose Committee commands (request, cancel, inspect) are ordinary Core commands producing advisory artifacts and can never authorize an effect. Proposed ADRs 0015–0026 define these authority and interface decisions and remain non-authorizing while Proposed.
