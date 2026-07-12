# 18 — API Architecture

## API philosophy

WePLD exposes stable, versioned ports rather than internal storage. V1 is a local product: the Studio, local worker hosts, and approved adapters communicate with the Core through authenticated local RPC. A network public API is explicitly deferred, but API semantics are designed now so future remote control planes do not require rewriting domain logic.

## Interface classes

| Interface | Consumers | Semantics |
| --- | --- | --- |
| Command API | Studio, Messenger, authorized automation | submit intent; return accepted/rejected/pending, never optimistic mutation |
| Query API | Studio workspaces, reporting | read authorized projections with version/freshness |
| Event subscription | Studio, projection workers, Messenger | resumable filtered stream of domain events/deltas |
| Worker protocol | worker host adapters | registration, compatible lease, heartbeat, artifacts, proposed actions, terminal outcomes |
| Brain port | Brain Gateway adapters | provider-neutral structured request/result |
| Tool port | Worker hosts/toolchains | policy-bound action request, execution evidence, cancellation |
| Plugin/registry port | package hosts and admin UI | discovery, install, resolve, health, revoke |
| Integration port | Messenger/channel/MCP adapters | normalized inbound intent, outbound delivery, credential mediation |

## Transport and schema decision

V1 uses a versioned local IPC/RPC transport over OS-appropriate Unix domain sockets or named pipes, with an authenticated per-user session and strict filesystem permissions. JSON-schema-compatible contracts are the portable interchange representation; implementation may use a compact typed transport internally so long as generated/validated schemas remain authoritative. Loopback TCP is a development fallback only and requires local authentication. No unauthenticated localhost control endpoint is acceptable.

Future remote APIs can add mTLS/OIDC and a gateway while preserving command/query/event semantics. REST-like HTTP is suitable for administrative/external integrations; streaming RPC is suitable for worker heartbeats and event delivery. Transport choice must not leak into the domain layer.

## Command model

Commands are named intent, include an idempotency key, caller identity, project scope, expected record revision where relevant, payload schema version, and client correlation ID. The Core returns one of:

| Outcome | Meaning |
| --- | --- |
| Accepted | durable command recorded; outcome will appear as events/projection |
| Rejected | invalid schema, authorization, policy, or concurrency conflict with reason |
| Awaiting approval | valid but requires a decision packet/authorized approval |
| Deferred | valid but unavailable dependency/capacity prevents immediate processing |

Commands never expose a raw database query or a generic “run arbitrary action” endpoint. Examples include CreateMission, ReviseScope, ApprovePlan, PauseMission, ResolveDecision, ProposeToolAction, InstallPackage, and AcknowledgeFinding.

## Query and subscription model

Queries are projection-specific, authorization-filtered, paginated, and return schema/version/freshness metadata. Clients request fields by documented view contracts, not database-table shapes. Subscriptions begin from a durable cursor and can reconnect/replay; clients receive an explicit resync signal when a cursor is expired or incompatible.

## Authorization and capability tokens

Human/API callers authenticate to identity scope and are authorized by role, project, and policy. Worker and plugin calls use short-lived signed capability tokens that bind subject, action, resources, data classification, conditions, expiry, and correlation/task context. Tokens are validated by the enforcement point and cannot be used to obtain a broader lease or direct mission-state write.

## Errors, versions, and deprecation

Errors are typed, non-sensitive, correlated, and actionable: validation, policy denial, authorization, conflict, unavailable dependency, quota, timeout, or internal fault. API schemas use semantic versions with additive minor changes; breaking changes require a new major port/version and coexistence window. Adapters declare supported versions at registration. Deprecated versions emit a structured warning and migration target; package/API removal follows the release policy.

## External integrations and webhooks

Channels and third-party tools communicate through adapters, not Core-domain endpoints exposed to the internet. Inbound webhooks validate origin/authentication, normalize payloads, classify content, rate-limit, and enqueue a command. Outbound calls use a transactional delivery queue, idempotency/deduplication, redaction policy, retries, and audit receipts. No external callback is trusted to assert execution success without evidence.

See also: [05_Worker_Architecture.md](05_Worker_Architecture.md), [06_Brain_Architecture.md](06_Brain_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), [16_Data_Model.md](16_Data_Model.md), and [17_Event_System.md](17_Event_System.md).

