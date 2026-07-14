# ADR-0019 — Context Compiler combines LSP and hybrid retrieval under authority-first ranking

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H4 implementation authorization

## Context

Model quality depends heavily on context selection. The existing plan has general retrieval and a candidate pack reference, but no complete reproducible compiler, language-neutral LSP contract, or ordering rule preventing semantic similarity from outranking policy, specifications, exact code, or structural facts.

## Decision

Create the governed Context Compiler and `ContextItem` provenance contract in [32_Hermes_Engineering_Intelligence_Runtime.md](../32_Hermes_Engineering_Intelligence_Runtime.md). Core owns authorization and captured truth; Hermes operates compilation inside the approved TaskPacket.

Add a language-neutral LSP broker normalizing definitions, references, symbols, implementations, call/type hierarchy, diagnostics, types, rename impact, affected files, and affected tests. Hybrid retrieval combines lexical, LSP, AST/tree-sitter, semantic, Git, ADR/specification, evidence, and typed-memory sources.

Ranking is authority before relevance, exact before approximate, current before stale, structural before semantic, and verified before inferred. Semantic vectors are recall aids only. Every pack is authorized, budgeted, provenance-labelled, content-addressed, and reproducible.

## Reason

This gives planning, implementation, review, and forensics the smallest useful context while preserving engineering truth and making omissions and retrieval behavior measurable.

## Benefits

- Impact-aware planning and affected-test mapping.
- Reduced context waste and reproducible invocations.
- Explicit degraded, partial, and stale states.
- Replaceable retrieval and language adapters.

## Trade-offs

- LSP coverage is incomplete by language and toolchain.
- Index and diagnostic freshness require explicit controls.
- Structural and semantic indexes add optional dependencies and evaluation cost.
- Initial language support remains deliberately limited.

## Migration

H4 evidence must prove selection-manifest completeness; deterministic pack hashes; authority/source precedence; policy omissions; LSP freshness and failure disclosure; exact symbol and impact fixtures; semantic non-override; injection fencing; affected-test evidence; and measurable benefit in controlled ablations.

Draft PR #1's context hash/reference, CAS, gateway, repository validation, and narrow planner/builder packs can be extended only if the baseline is accepted. Its V0 packs are not the proposed compiler, and no LSP or hybrid-RAG conformance is inferred from them.
