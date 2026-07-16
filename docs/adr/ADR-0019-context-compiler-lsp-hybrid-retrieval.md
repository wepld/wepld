# ADR-0019 — Context Compiler combines LSP and hybrid retrieval under authority-first ranking

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H4 implementation authorization

## Context

Model quality depends heavily on context selection. The existing plan has general retrieval and a candidate pack reference, but no complete reproducible compiler, language-neutral LSP contract, or ordering rule preventing semantic similarity from outranking policy, specifications, exact code, or structural facts.

## Decision

Create the governed Context Compiler and `ContextItem` provenance contract in [32_Hermes_Engineering_Intelligence_Runtime.md](../32_Hermes_Engineering_Intelligence_Runtime.md). Core owns authorization and captured truth; Hermes operates compilation inside the approved TaskPacket.

Deliver the target in three governed slices:

1. **H4.1:** the authority-first Context Compiler, exact path/identifier reads, lexical search, Git evidence, exact source and Core records, one read-only `rust-analyzer` adapter, and a complete reproducible `ContextSelectionManifest`.
2. **H4.2:** AST/tree-sitter relationships, impact and affected-test inference, and additional LSP adapters only after each passes normalized-contract, isolation, provenance, freshness, failure, fallback, and value conformance.
3. **H4.3:** semantic retrieval or embeddings only after controlled ablation shows material benefit without authority, security, freshness, outcome-quality, latency, cost, or token-budget harm. Semantic remains optional and removable.

The long-term language-neutral LSP broker may normalize definitions, references, symbols, implementations, call/type hierarchy, diagnostics, types, rename impact, affected files, and affected tests. The long-term retrieval set may combine lexical, LSP, AST/tree-sitter, semantic, Git, ADR/specification, evidence, and typed-memory sources, but listing a signal here does not admit it before its slice gate.

H4 also defines three context-governance records:

- `MissionExplorationBranch` pins its parent mission/plan/packet/context versions, read-only question/scope, eligible tool projection, budgets, expiry and stop condition. It produces candidates only. It cannot mutate, approve, alter WIP, perform an external effect, or admit memory; promotion requires normal validation/review/decision, and any H7 memory use passes through `MemoryCandidate` and the Memory Judge.
- `CompactionRecord` binds the raw source event span, prior compaction, first retained event, source/context/summary hashes, token/item counts, producer profile/config, rehydrated mandatory authority, retained/omitted IDs and reasons, tool-output artifact IDs, verification and supersession. Raw chronology remains recoverable and the summary is never authority, accepted evidence by itself, or automatic memory.
- `ToolOutputArtifact` binds a complete policy-allowed raw result to action/attempt/capability/tool/version IDs, hashes, exit/error state, size counts, excerpt strategy, classification/redaction/retention and provenance. The model receives only the budgeted excerpt plus an explicit artifact reference; retrieving another range is a new authorized context decision. When policy forbids complete capture, the record says so.

H3 transports the typed compaction lifecycle and bounded tool-result records; H4 compiles and verifies them. Neither a temporary file path nor an LLM summary satisfies durable provenance.

Ranking is authority before relevance, exact before approximate, current before stale, structural before semantic, and verified before inferred. Semantic vectors are recall aids only. Every pack is authorized, budgeted, provenance-labelled, content-addressed, and reproducible.

## Reason

This gives planning, implementation, review, and forensics the smallest useful context while preserving engineering truth and making omissions and retrieval behavior measurable.

## Benefits

- Impact-aware planning and affected-test mapping.
- Reduced context waste and reproducible invocations.
- Explicit degraded, partial, and stale states.
- Replaceable retrieval and language adapters.
- A useful, reproducible H4.1 that does not depend on multi-language parsing or embeddings.
- Bounded alternative exploration without silent mission mutation or automatic memory.
- Reconstructable compaction and full-output provenance without flooding model context.

## Trade-offs

- LSP coverage is incomplete by language and toolchain.
- Index and diagnostic freshness require explicit controls.
- Structural and semantic indexes add optional dependencies and evaluation cost.
- Initial language support remains deliberately limited.

## Migration

H4.1 evidence must prove selection-manifest completeness and reconstruction; deterministic pack hashes; authority/source precedence; policy omission behavior; exact path/identifier and lexical fixtures; Git evidence provenance; `rust-analyzer` isolation, freshness, diagnostics, unsupported/degraded behavior and no-write enforcement; injection fencing; and benefit against the preregistered exact/lexical baseline.

H4.2 evidence must separately prove AST/tree-sitter and impact/affected-test correctness and the full conformance suite for each added LSP adapter. A failed adapter or structural treatment is removed without changing H4.1.

H4.3 evidence must compare semantic-off and semantic-on treatments under fixed packs, tasks, profiles, tools, budgets and environments. Admission requires the stated benefit threshold with mandatory-authority retention and exact-source precedence at 100%, cross-scope leak and language-server write at zero, stale-result acceptance below its threshold, and no harmful outcome, latency, cost, or token regression. Failure or inconclusive value leaves semantic retrieval disabled.

Before H4.1 exit, RS-24 must show that read-only exploration improves ambiguity/alternative discovery without effect, write, scope, authority or memory leakage; RS-25 must prove raw-span reconstruction, mandatory-authority retention and compaction-corruption detection; and RS-26 must prove deterministic output budgets, artifact hash/retrieval integrity, redaction/retention enforcement and no silent truncation. A failure disables the treatment and falls back respectively to the main mission path, uncompacted raw references, or a smaller direct tool query.

Draft PR #1's context hash/reference, CAS, gateway, repository validation, and narrow planner/builder packs can be extended only if the baseline is accepted. Its V0 packs are not the proposed compiler, and no LSP or hybrid-RAG conformance is inferred from them.
