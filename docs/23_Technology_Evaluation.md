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
| Knowledge | graph DB, vector DB, relational + indexes | **H4.1 relational typed links + exact/full text/Git + one rust-analyzer adapter; conditional H4.2 structural and H4.3 local vector treatments** | governance/query simplicity first; structural expands only after conformance, and vector/semantic remains disabled until H4.3 ablation proves value and safety |
| Workflow engine | bespoke durable state machine, Temporal, distributed scheduler | **minimal durable state machine in Core** | V1 is local and single-user; reassess distributed engine when remote execution becomes real |
| Messaging | in-process ledger/outbox, NATS JetStream, Kafka | **transactional outbox + local projection workers** | avoids operating a broker locally; later evaluate NATS for remote delivery |
| Observability | proprietary-only, OpenTelemetry, ad hoc logs | **OpenTelemetry-compatible traces/metrics/logs plus domain events** | vendor-neutral telemetry; domain ledger remains authoritative for workflow truth |
| Local IPC | loopback HTTP, Unix/named pipe RPC, direct DB | **authenticated OS-local IPC/RPC** | avoids exposing an unauthenticated local web API and preserves process boundary |
| Secrets | config files, environment variables, OS/enterprise store | **OS-backed vault / enterprise secret broker** | no secret material in prompts, skills, plugins, or ordinary logs |
| H3 resource loading | compiled/built-in resources, repository manifests, dynamic package install | **H3.1 repository-owned static manifests and exact hashes** | delivers typed skills/hooks and internal events without creating an installer, registry, marketplace, general signing service or third-party executable surface |
| Skill/hook packaging | local catalogue, organization registry, public marketplace, none | **defer to conditional H3.2** | only measured H3.1 benefit plus RS-19 governance proof can justify staging, provenance/signature verification, atomic activation, rollback and revocation |
| Tool-schema exposure | expose every tool, prompt-only allowlist, capability-projected schemas | **deterministic capability-projected catalogue** | intersect packet, policy, capability, role, scope, compatibility and availability; schema visibility reduces context but never substitutes for enforcement |
| Tool/resource catalogue context | eager full catalogue, lazy opaque search, budgeted projection/discovery | **explicit item/token budgets with audited omissions** | mandatory authority wins; future MCP discovery is bounded and provenance-labelled and cannot auto-connect, auto-trust or grant authority |
| Large tool results | unbounded prompt text, silent truncation, temporary files, content-addressed artifacts | **bounded excerpt plus governed `ToolOutputArtifact`** | retain exact raw hashes/provenance when policy permits, expose truncation, and authorize every later range read; summary/path alone is not evidence |
| Long-session compaction | transcript deletion, opaque summary, raw-only replay, event-referenced compaction | **verified `CompactionRecord` over retained raw chronology** | rehydrate mandatory authority from Core, bind source spans/hashes/omissions, detect corruption, and treat summaries as disposable projections |
| Alternative exploration | mutate main thread, unbounded side chat, read-only mission branch | **bounded `MissionExplorationBranch`** | pin parent versions and budgets; default read-only; promote findings through normal review and memory only through H7 Memory Judge |
| Language intelligence | direct language-server coupling, shell-only analysis, normalized broker | **H4.1 language-neutral broker with `rust-analyzer` only** | prove one conservative read-only contract first; additional language adapters are H4.2 candidates and enter one at a time through conformance/value fixtures |
| Structural retrieval | regex only, compiler-specific ASTs, tree-sitter | **defer tree-sitter/AST to H4.2** | H4.1 does not need a parser graph; grammar quality, licensing, incremental cost, impact accuracy, and false matches must be measured before structural expansion |
| Code retrieval | exact/lexical, LSP, structural, semantic, hybrid | **stage authority-ranked retrieval** | H4.1 uses exact path/identifier, lexical, Git and `rust-analyzer`; H4.2 may add structural signals; H4.3 may add semantic signals only after ablation |
| Semantic index | embedded vector library, SQLite extension, external service, none | **disabled through H4.1/H4.2; evaluate at H4.3** | vectors are optional ranking signals, never authoritative truth; measured benefit with no authority, security, freshness, quality, cost or token harm precedes any default use |
| Evaluation harness | ad hoc benchmark scripts, provider-native evals, WePLD-owned fixtures | **WePLD-owned controlled harness** | holds mission/spec/policy/tools/environment/budget constant and measures outcome equivalence, safety, evidence, convergence, cost, and honest failure |

## Important technical cautions

SQLite WAL improves local reader/writer behavior, but it relies on shared-memory coordination and is not a distributed database or a safe multi-machine network-filesystem solution. The Core therefore remains the sole operational writer and cloud/sync storage is a later decision. SQLite’s documentation also emphasizes handling checkpointing and transient WAL/SHM files correctly. [SQLite WAL documentation](https://www.sqlite.org/wal.html)

OpenTelemetry provides a vendor-neutral framework for traces, metrics, and logs. WePLD will use those signals for component observability while keeping mission semantics in its domain event ledger. [OpenTelemetry documentation](https://opentelemetry.io/docs/)

Tauri is a plausible desktop shell candidate rather than a product architecture. Its cross-platform build, updater, plugin, webview, and permission behavior must be evaluated against WePLD’s security and accessibility acceptance fixtures before adoption. [Tauri documentation](https://tauri.app/)

LSP is a protocol boundary, not a promise that all servers expose identical quality or semantics. H4.1 therefore proves the conservative broker contract with `rust-analyzer` only while preserving adapter-specific evidence and capability discovery. Pyright, the TypeScript language server, `gopls`, and any other server are H4.2 candidates, not first-slice commitments, and each must demonstrate conformance, isolation, provenance, freshness, failure behavior, operational cost, and task benefit independently.

Tree-sitter and semantic vectors solve different retrieval problems and are not prerequisites for a useful compiler. H4.1 first establishes exact path/identifier, lexical, Git and `rust-analyzer` evidence plus reproducible selection manifests. H4.2 may add structural parsing and impact/test inference when its conformance suite passes. H4.3 may evaluate semantic search for concept-level recall, but it remains off by default unless controlled ablation proves material benefit without authority, security, freshness, quality, latency, cost, or token-budget harm. Neither signal may supersede exact repository content, compiler/LSP diagnostics, approved specifications, ADRs, policy, or recorded evidence. An unavailable or stale index must create a visible degraded state rather than silent context loss.

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
7. For H3.1, compare the built-in minimal kernel/skill/hook runtime with the fixed harness while proving static manifest/hash resolution, typed I/O, declared resource denial, required evidence, internal-event conformance, and absence of external loading. Record measured value before proposing H3.2.
8. For conditional H3.2, run RS-19 against only isolated fake packages and staging: provenance/signature verification, capability/license review, atomic activation, rollback and revocation. Do not build the package surface if its benefit or governance gate fails.
9. For H4.1, compare exact/lexical/Git context with and without the read-only `rust-analyzer` adapter; validate selection-manifest completeness, deterministic pack reconstruction, authority ordering, diagnostics freshness, latency, no-write behavior, and explicit degradation.
10. For H4.2, test tree-sitter/AST impact and affected-test inference, then each additional LSP adapter independently. No treatment enters the default route without conformance and measurable benefit over H4.1.
11. For H4.3, compare semantic-off and semantic-on treatments under fixed tasks, profiles, tools, budgets, and environments; publish retrieval quality, outcome, authority retention, leakage, freshness, token, latency, and cost results. Reject or keep semantic disabled on security harm, quality regression, inconclusive value, or economic harm.
12. Run controlled harness ablations for memory, loops, subagents, and skill routing before treating any later component as required.
13. Execute every applicable named experiment in the RS-00–RS-30 register rather than adopting a product capability by analogy; preserve source revisions, negative results, security fault injections, license/provenance decisions, and disable criteria.
14. Run RS-23 with full, prompt-only-filtered, and deterministic capability-projected tool-schema arms; measure tokens, correct tool choice, schema errors, omission recovery, and actual-boundary denial under stale/forged projections.
15. Run RS-24 against main-path-only exploration using bounded read-only branches from fixed parent versions; measure material alternative discovery, decision quality, cost, branch contamination, and attempted mutation/effect/memory promotion.
16. Run RS-25 with raw history, opaque summary, and governed `CompactionRecord` arms; inject omitted/reversed policy, stale source IDs, corrupt hashes, split turns, and repeated compaction, then verify reconstruction and mandatory-authority retention.
17. Run RS-26 with bounded outputs and content-addressed artifacts; inject huge lines, secret-bearing output, disappearing temporary paths, hash substitution, range abuse, and retention expiry. Reject silent truncation, provenance loss, leakage, or material task-quality harm.
18. Run RS-21/RS-22 against manual coordination and contained synthetic broadcast controls; prove deterministic exact-parent SOP compilation, required-input recall, information reduction, revocation, and zero self-subscription, peer-chat, shared-environment or authority-bearing input leakage.
19. Run RS-27 against unstructured denial strings; measure identical-denial repetition and safe recovery while injecting false retryability, unsafe alternatives, forged evidence, uncertain effects, and unchanged-action loops.
20. Run RS-28 with deterministic policy held fixed; measure false allow/block, flapping, latency, interruption reduction, task quality and unsafe-effect escape. Reject any grant, deny override, protected-effect approval, policy mutation, sole-boundary dependence or safety escape.
21. Run RS-29 only for preregistered high-risk/uncertain strata with exact contract/commit/policy/tools/environment/budget/gates held fixed; retain every arm and require every selected candidate to pass independently. Reject vote, rank, visual preference, arm leakage or unjustified cost.
22. Run RS-30 against the canonical plain diff/event/evidence view; inject stale/forged visual evidence, hidden console/network failures, inaccessible states, secret-bearing traces and projection drift, then measure review time and defect detection without appearance-based acceptance.

## Explicit non-decisions

No decision is made yet on a specific hosted provider, vector-index library, tree-sitter grammar set, graph database, container engine, cloud vendor, distributed queue, or enterprise identity provider. Universal language/model support is explicitly not a first-increment goal. These choices must be driven by accepted H-gate evidence and documented in ADRs rather than assumed from popularity.

See also: [03_System_Architecture.md](03_System_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [27_Performance_Goals.md](27_Performance_Goals.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), and [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md).

