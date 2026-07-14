# 23 — Technology Evaluation

## Decision criteria

Technology choices must serve local-first operation, cross-platform support, security isolation, replaceable adapters, observability, testability, reproducible context, and a small operational footprint. No framework, language server, parser, vector store, model, or agent harness is allowed to become the product’s architecture. Recommendations are hypotheses to validate at the applicable evidence gate, not production commitments.

Draft PR #1 is reference material only. Its Rust workspace, local provider adapter, and candidate contracts may inform evaluation, but the open Draft is unmerged and unratified and does not settle technology choices for H1–H9.

The official-source comparison in [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md) is the admission register for reference-derived technology ideas. Product popularity, a published benchmark, protocol compatibility, provider count, or an upstream open-source license is not an adoption decision. Each candidate needs its named RS experiment, security result, rollback path, accepted milestone gate, and component-level provenance review. Because this WePLD baseline has no repository-level license/notice policy, no upstream source reuse is currently approved; architecture work remains clean-room.

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
| Language intelligence | direct language-server coupling, shell-only analysis, normalized broker | **language-neutral LSP broker with a deliberately limited initial adapter set** | definitions, references, symbols, diagnostics, call hierarchy, rename impact, and affected files/tests remain portable; adapters earn support through fixtures |
| Structural retrieval | regex only, compiler-specific ASTs, tree-sitter | **evaluate tree-sitter as a normalized structural signal** | useful multi-language syntax coverage, but grammar quality, licensing, incremental cost, and false matches must be measured per initial language |
| Code retrieval | lexical only, semantic only, hybrid | **authority-ranked hybrid retrieval** | policy/spec/ADR and exact lexical/LSP/structural sources outrank semantic similarity; every result carries provenance, trust, freshness, and selection reason |
| Semantic index | embedded vector library, SQLite extension, external service, none | **defer library choice until H4 evaluation** | vectors are optional ranking signals, never authoritative truth; local deletion, rebuild, portability, and footprint decide the implementation |
| Evaluation harness | ad hoc benchmark scripts, provider-native evals, WePLD-owned fixtures | **WePLD-owned controlled harness** | holds mission/spec/policy/tools/environment/budget constant and measures outcome equivalence, safety, evidence, convergence, cost, and honest failure |

## Important technical cautions

SQLite WAL improves local reader/writer behavior, but it relies on shared-memory coordination and is not a distributed database or a safe multi-machine network-filesystem solution. The Core therefore remains the sole operational writer and cloud/sync storage is a later decision. SQLite’s documentation also emphasizes handling checkpointing and transient WAL/SHM files correctly. [SQLite WAL documentation](https://www.sqlite.org/wal.html)

OpenTelemetry provides a vendor-neutral framework for traces, metrics, and logs. WePLD will use those signals for component observability while keeping mission semantics in its domain event ledger. [OpenTelemetry documentation](https://opentelemetry.io/docs/)

Tauri is a plausible desktop shell candidate rather than a product architecture. Its cross-platform build, updater, plugin, webview, and permission behavior must be evaluated against WePLD’s security and accessibility acceptance fixtures before adoption. [Tauri documentation](https://tauri.app/)

LSP is a protocol boundary, not a promise that all servers expose identical quality or semantics. The broker must normalize a conservative common contract while preserving adapter-specific evidence and capability discovery. Initial support should be limited to languages represented by accepted evaluation fixtures—for example rust-analyzer first, followed only by adapters whose value and operational cost are demonstrated. Pyright, the TypeScript language server, and gopls are candidates, not a first-increment commitment.

Tree-sitter and semantic vectors solve different retrieval problems. Structural parsing may improve symbol- and syntax-aware retrieval when no reliable LSP answer exists; semantic search may improve recall for concept-level questions. Neither may supersede exact repository content, compiler/LSP diagnostics, approved specifications, ADRs, policy, or recorded evidence. An unavailable or stale index must create a visible degraded state rather than silent context loss.

## Alternatives and triggers to revisit

| Current choice | Revisit when | Likely alternative |
| --- | --- | --- |
| Modular local Core | multi-user/remote tasks need independent availability and horizontal scheduling | split bounded contexts behind existing ports; central service control plane |
| SQLite + artifact store | organization-scale concurrent collaboration / retention requires it | PostgreSQL plus object storage and migration tooling |
| Local outbox | sustained remote event distribution / fan-out becomes core | NATS JetStream or equivalent durable broker |
| Minimal workflow state machine | long-running distributed workflows/outages require mature orchestration | evaluate Temporal or comparable engine |
| Tauri shell | required native/editor/accessibility/plugin behavior fails spikes | Electron or selective native shells, preserving Studio/Core contract |
| Local semantic index | cross-project retrieval quality/scale demands a service | governed vector/graph service with deletion and access guarantees |
| Initial LSP adapter set | another language has accepted fixtures and measured planning/verification benefit | add a conformant adapter without changing the normalized contract |
| Tree-sitter structural signal | grammar gaps or maintenance cost exceed measured retrieval gain | language-native parsers or LSP-only structural evidence |
| Embedded vectors | local rebuild/deletion/portability or quality targets fail | no semantic tier, or a later governed local/service implementation |

## Technology spikes required before implementation

1. Validate an authenticated Core-to-Studio IPC transport and crash/reconnect behavior on macOS, Windows, and Linux.
2. Exercise task worktree isolation, process/resource/network controls, and report platform protection gaps.
3. Compare local and hosted brain profiles on structured-output, latency, cost, privacy, and evaluation fixtures.
4. Demonstrate SQLite event append, projection rebuild, backup/restore, and WAL lifecycle handling under abrupt termination.
5. Prototype Tauri UI accessibility, high-volume event rendering, updater/signing, and extension-host constraints.
6. Validate artifact hashing, Git worktree cleanup, and recovery after an interrupted tool action.
7. Compare LSP-off versus each candidate adapter on affected-file/test discovery, diagnostic freshness, latency, and failure behavior using fixed repositories.
8. Compare lexical-only, lexical+LSP, structural, semantic, and authority-ranked hybrid retrieval; publish inclusion/omission provenance and token-cost results.
9. Validate Context Compiler determinism, trust/classification filtering, stale-index disclosure, and pack reconstruction under fixed budgets.
10. Run controlled harness ablations for LSP, RAG, memory, loops, subagents, and skill routing before treating any component as required.
11. Execute the applicable RS-00–RS-20 reference-system experiments rather than adopting a product capability by analogy; preserve source revisions, negative results, security fault injections, license/provenance decisions, and disable criteria.

## Explicit non-decisions

No decision is made yet on a specific hosted provider, vector-index library, tree-sitter grammar set, graph database, container engine, cloud vendor, distributed queue, or enterprise identity provider. Universal language/model support is explicitly not a first-increment goal. These choices must be driven by accepted H-gate evidence and documented in ADRs rather than assumed from popularity.

See also: [03_System_Architecture.md](03_System_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [27_Performance_Goals.md](27_Performance_Goals.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), and [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md).

