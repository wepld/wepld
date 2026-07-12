# 23 — Technology Evaluation

## Decision criteria

Technology choices must serve local-first operation, cross-platform support, security isolation, replaceable adapters, observability, testability, and a small operational footprint. No framework is allowed to become the product’s architecture. Each recommendation is a hypothesis to validate in Phase 0 spikes, not a production commitment.

## Recommended starting stack

| Concern | Options considered | Recommendation | Rationale / decision boundary |
| --- | --- | --- | --- |
| Desktop shell | Tauri, Electron, native per OS | **Tauri 2 shell** with a web UI | aligns a Rust Core with a cross-platform shell; validate plugin, accessibility, update, and sandbox needs before lock-in |
| Studio UI | React/TypeScript, Vue, Svelte | **React + TypeScript** | mature component/testing ecosystem and broad hiring; keep UI behind query/command ports |
| Local Core | Rust, TypeScript/Node, Go | **Rust daemon** | strong fit for resource-aware local process, native integration, and shared shell language; domain must remain framework-independent |
| Local storage | SQLite, embedded KV, local Postgres | **SQLite operational store + artifact files** | transactional, portable local state; single Core writer; no network-share assumption |
| Source truth | custom diff store, Git | **Git plus per-task worktrees** | preserves familiar history/review and keeps operational events from duplicating source history |
| Knowledge | graph DB, vector DB, relational + indexes | **relational typed links + full text + optional local vector index** | governance/query simplicity before graph scale is proven |
| Workflow engine | bespoke durable state machine, Temporal, distributed scheduler | **minimal durable state machine in Core** | V1 is local and single-user; reassess distributed engine when remote execution becomes real |
| Messaging | in-process ledger/outbox, NATS JetStream, Kafka | **transactional outbox + local projection workers** | avoids operating a broker locally; later evaluate NATS for remote delivery |
| Observability | proprietary-only, OpenTelemetry, ad hoc logs | **OpenTelemetry-compatible traces/metrics/logs plus domain events** | vendor-neutral telemetry; domain ledger remains authoritative for workflow truth |
| Local IPC | loopback HTTP, Unix/named pipe RPC, direct DB | **authenticated OS-local IPC/RPC** | avoids exposing an unauthenticated local web API and preserves process boundary |
| Secrets | config files, environment variables, OS/enterprise store | **OS-backed vault / enterprise secret broker** | no secret material in prompts, skills, plugins, or ordinary logs |

## Important technical cautions

SQLite WAL improves local reader/writer behavior, but it relies on shared-memory coordination and is not a distributed database or a safe multi-machine network-filesystem solution. The Core therefore remains the sole operational writer and cloud/sync storage is a later decision. SQLite’s documentation also emphasizes handling checkpointing and transient WAL/SHM files correctly. [SQLite WAL documentation](https://www.sqlite.org/wal.html)

OpenTelemetry provides a vendor-neutral framework for traces, metrics, and logs. WePLD will use those signals for component observability while keeping mission semantics in its domain event ledger. [OpenTelemetry documentation](https://opentelemetry.io/docs/)

Tauri is a plausible desktop shell candidate rather than a product architecture. Its cross-platform build, updater, plugin, webview, and permission behavior must be evaluated against WePLD’s security and accessibility acceptance fixtures before adoption. [Tauri documentation](https://tauri.app/)

## Alternatives and triggers to revisit

| Current choice | Revisit when | Likely alternative |
| --- | --- | --- |
| Modular local Core | multi-user/remote tasks need independent availability and horizontal scheduling | split bounded contexts behind existing ports; central service control plane |
| SQLite + artifact store | organization-scale concurrent collaboration / retention requires it | PostgreSQL plus object storage and migration tooling |
| Local outbox | sustained remote event distribution / fan-out becomes core | NATS JetStream or equivalent durable broker |
| Minimal workflow state machine | long-running distributed workflows/outages require mature orchestration | evaluate Temporal or comparable engine |
| Tauri shell | required native/editor/accessibility/plugin behavior fails spikes | Electron or selective native shells, preserving Studio/Core contract |
| Local semantic index | cross-project retrieval quality/scale demands a service | governed vector/graph service with deletion and access guarantees |

## Technology spikes required before implementation

1. Validate an authenticated Core-to-Studio IPC transport and crash/reconnect behavior on macOS, Windows, and Linux.
2. Exercise task worktree isolation, process/resource/network controls, and report platform protection gaps.
3. Compare local and hosted brain profiles on structured-output, latency, cost, privacy, and evaluation fixtures.
4. Demonstrate SQLite event append, projection rebuild, backup/restore, and WAL lifecycle handling under abrupt termination.
5. Prototype Tauri UI accessibility, high-volume event rendering, updater/signing, and extension-host constraints.
6. Validate artifact hashing, Git worktree cleanup, and recovery after an interrupted tool action.

## Explicit non-decisions

No decision is made yet on a specific hosted provider, vector-index library, graph database, container engine, cloud vendor, distributed queue, or enterprise identity provider. Those choices must be driven by actual Phase 0 results and documented in ADRs rather than assumed from popularity.

See also: [03_System_Architecture.md](03_System_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), and [27_Performance_Goals.md](27_Performance_Goals.md).

