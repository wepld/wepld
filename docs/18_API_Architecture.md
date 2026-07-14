# 18 — API Architecture

## One Core workflow

WePLD exposes stable, versioned ports rather than internal storage. CLI, Studio, MCP, Messenger/integrations, and future public APIs call the same Core command/query/subscription workflow and receive the same authority, policy, revision, evidence, and transition semantics. No surface owns a shortcut implementation or a second source of truth.

V1 is local-first: approved clients and adapters communicate with the long-lived Core over authenticated local IPC/RPC. A network public API is deferred, but domain contracts are transport-neutral. Structured schemas and Core records—not Markdown payloads, UI state, or database tables—are authoritative.

## Authority invariant

Core alone commits durable policy, approvals, capabilities, budgets, governance/workflow state, effect intent/result, completion, and recovery. The Brain Agent proposes specifications/plans; Hermes requests supervised transitions and routing; builders/subagents return actions, artifacts, findings, and evidence; tool/provider boundaries return observed effects. API verbs preserve those distinctions: `Propose`, `Submit`, `Request`, `Approve`, `Issue`, `Report`, `Validate`, and `Decide` are not interchangeable.

Every executable request resolves the version chain:

`Policy > approved EngineeringSpecification > approved OutcomeContract > approved DeliveryPlan > approved PhasePlan > authorized TaskPacket > ToolAction`

## Interface classes

| Interface | Consumers | Semantics |
| --- | --- | --- |
| Command API | CLI, Studio, MCP adapter, Messenger, authorized automation | submit typed intent; return accepted/rejected/awaiting/deferred, never optimistic mutation |
| Query API | all authorized surfaces/reporting | read field-filtered projections with versions, authority, freshness, and gaps |
| Event subscription | CLI/Studio streams, projection workers, Messenger | resumable authorization-filtered stream of committed Core facts/deltas |
| Governed-artifact port | Brain Agent, review surfaces, Core validators | propose/validate/review/approve/version charter, spec, outcome, plans, changes and completion |
| Hermes supervisor port | Hermes runtime | obtain approved phase/task envelopes; request leases/transitions; report routing, loops and structured results |
| Worker/subagent protocol | approved execution hosts | register, accept lease, heartbeat, propose actions, return artifacts/findings/attempt result; never write task/mission state |
| Brain/builder port | Brain Gateway adapters | provider-neutral structured request/result with context/evidence/budget contract |
| Intelligence ports | Skill Runtime, Hook Bus, Context Compiler, LSP/retrieval/memory adapters | typed scoped input/output and provenance; no direct governance or effect write |
| Tool/effect port | mediated tool/provider boundaries | validate capability, execute exact intent, probe postcondition, report result/evidence to Core |
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
- `ProposeDeliveryPlan`, `ValidateDeliveryPlan`, `ApproveDeliveryPlan`, `ApprovePhasePlan`;
- `SubmitSpecificationChangeRequest`, `SubmitPlanChangeRequest`, `DecideChangeRequest`;
- `ProposeTaskPacket`, `RequestTaskTransition`, `RequestPhaseTransition`, `SetWipPolicy` (Core validates and authorizes the packet);
- `OpenDecisionRequest`, `AnswerDecisionRequest`, `SubmitRiskItem`, `InvalidateAssumption`;
- `ProposeToolAction`, `DecideEffectApproval`, `ReportEffectObservation`, `SubmitEvidenceBundle`, `ValidateEvidenceBundle`;
- `SubmitCompletionProposal`, `DecideCompletion`, `SubmitMemoryCandidate`, `RecordMemoryJudgment`;
- `RegisterEvaluationRun`, `RecordEvaluationMetric`, `ChangeProfileCertification`.

Only the role named by policy may invoke approval, issuance, validation, or decision commands. Brain Agent cannot call its own plan-approval path; Hermes cannot approve phase/task authority; executors cannot call task/phase/mission completion transitions; a tool boundary can report only the exact effect tied to its active capability.

Commands never expose raw database queries, direct ledger append, client-assigned authoritative state, or generic “run arbitrary action.” An offline surface may queue an untrusted command envelope but cannot reserve authority, WIP, budget, or perform effects before Core accepts it.

## Query and subscription model

Queries are projection-specific, field-level authorization-filtered, paginated, and return schema/version, governing record versions, cursor, freshness, classification, and source availability. Contract views expose trace links from user intent through requirement, outcome, phase, task, evidence, completion decision, and memory candidate.

Subscriptions resume from a durable cursor and are filtered by principal and scope. Clients receive explicit `ResyncRequired`, `CursorExpired`, `SchemaIncompatible`, or `AccessChanged` signals. A client never fabricates intermediate state from an event it is not authorized to see; Core may provide a redacted transition projection.

## Capability and effect protocol

Human/API callers authenticate to identity scope and are authorized by role, project, artifact authority, and policy. Hermes, workers, subagents, hooks, plugins, and tool/provider boundaries use short-lived Core-issued capabilities binding subject, exact action, resources, classification, conditions, budget, governing versions, task/correlation context, and expiry.

The API preserves the Effect Firewall sequence:

`propose → classify → policy → capability → approval → durable intent → execute → probe → evidence`

The execution boundary validates the active capability immediately before effect, then reports typed observed postconditions and evidence to Core. It cannot exchange a token for a broader lease, append an event, assert workflow success, or retry an uncertain non-idempotent effect without new Core direction.

## Errors, versions, and deprecation

Errors are typed, non-sensitive, correlated, and actionable: validation, authentication, authorization, policy denial, approval required, stale governing version, conflict, WIP/readiness, unavailable dependency, quota/budget, timeout, uncertain effect, evidence failure, or internal fault.

API schemas use semantic versions with additive minor changes. Breaking changes require a new major port/version and coexistence window. Adapters declare supported versions and capability sets at registration. Deprecation emits a structured warning, replacement target, and deadline. Artifact schema migration never mutates an approved historical record; it creates a validated representation/version with provenance.

## External integrations, webhooks, and MCP

External channels and tools communicate through adapters, not internet-exposed Core-domain endpoints. Inbound payloads authenticate origin, normalize identity, classify and rate-limit content, preserve provenance, and enter as untrusted intent. A chat message cannot assert an approval, evidence result, or completion without the relevant authenticated command and authority.

Outbound calls use a transactional delivery queue, idempotency/deduplication, classification/redaction, retries, and receipts. No callback is trusted to assert execution success without a probe/evidence contract. MCP resources and tool calls pass through the same authorization and Effect Firewall; MCP is a surface/adapter, not an alternate control plane.

ACP and comparable editor/terminal-agent protocols follow the same rule. Session IDs and plans are projections; capability negotiation describes a client envelope but cannot mint a Core capability. Filesystem, terminal, MCP and other tool requests are denied unless losslessly mapped to an authorized Core action. Transport, identity or data-flow adoption requires a new Proposed ADR under H9.

See [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [05_Worker_Architecture.md](05_Worker_Architecture.md), [06_Brain_Architecture.md](06_Brain_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), [16_Data_Model.md](16_Data_Model.md), and [17_Event_System.md](17_Event_System.md). Proposed ADRs 0015–0024 define these authority and interface decisions and remain non-authorizing while Proposed.
