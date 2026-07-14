# ADR-0020 — Separate typed memory and govern consolidation through a Memory Judge

**Status:** Proposed
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H7 implementation authorization

## Context

The current Knowledge plan and Draft PR #1's narrow lesson loop establish useful provenance foundations, but “memory” still risks conflating temporary reasoning, mission facts, repository lessons, measured skill/model behavior, and binding governance. Unverified or stale lessons could become hidden authority.

## Decision

Separate Working, Mission, Engineering, Skill, Provider/Model Performance, and Governance Memory as defined in [32_Hermes_Engineering_Intelligence_Runtime.md](../32_Hermes_Engineering_Intelligence_Runtime.md). Governance Memory is authoritative and mandatory when applicable; other classes cannot override it.

All durable learning begins as a `MemoryCandidate`. The Memory Judge validates evidence/provenance, authorization, classification, scope, deduplication, contradictions, confidence, freshness/expiry, supersession, and security before approving, rejecting, quarantining, merging as a new version, or superseding. Required human or policy review depends on memory class.

## Reason

Typed memory lets verified experience improve later missions without turning model output or retrieval scores into policy. Explicit consolidation and contradiction semantics keep the compounding loop auditable and reversible.

## Benefits

- Safer, scoped reuse of verified lessons.
- Measurable skill and provider learning.
- Explicit stale, contradictory, expiry, and supersession behavior.
- Preserved governance precedence.

## Trade-offs

- Conservative consolidation reduces memory volume.
- Contradiction and scope review add work.
- Confidence remains evidence-relative rather than universal truth.
- Cross-project learning remains deferred.

## Migration

H7 evidence must prove candidate-only ingestion; evidence/source requirements; deduplication; contradiction quarantine; freshness/expiry; supersession; scope/classification isolation; prompt-injection and authority-confusion defenses; governance precedence; and improvement on a later mission without lowering its quality bar.

Draft PR #1's candidate lesson status, project fingerprint, bounded selection, untrusted labelling, and atomic ledger row are useful candidate inputs. Its Build Feature-only lesson model is not generalized Engineering Memory, and no automatic promotion or conformance is inherited.
