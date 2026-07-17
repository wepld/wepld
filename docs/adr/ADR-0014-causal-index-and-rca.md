# ADR-0014 — Forensics runs on a derived causal index; root causes are ranked hypotheses with evidence

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Chief Architect · **Review:** Chronicle V1 gate

## Context

Engineering Forensics (v2-14) must reconstruct causal chains — "mission failed ← verification failed ← wrong refactor ← plan defect ← missing context" — and find the earliest meaningful cause. The ledger already carries `causation_ref` and `correlation_id`, but causality in engineering is richer than command parentage: packs inform invocations, invocations produce artifacts, artifacts satisfy gates, gates block completion, decisions scope plans.

## Decision

1. Chronicle maintains a **derived causal index**: `causal_edges(from_ref, to_ref, edge_type, derived_by, confidence)`, rebuilt deterministically from the ledger. Edge types: `caused` (ledger causation), `informed` (pack section → invocation), `produced` (invocation/phase → artifact), `evidenced` (artifact → gate result), `gated` (gate → transition), `decided` (decision → plan/task scope), `revised`, `carried`. Deterministic edges have confidence 1.0; heuristic edges (e.g., "this omitted file was later implicated") carry < 1.0 and name their deriving rule in `derived_by`.
2. **Root Cause Analysis outputs ranked hypotheses, never verdicts.** An RCA result is a causal chain where every step is a claim with evidence references (the Messenger claims discipline of v2-10 §5, applied to analysis). Model-assisted narration (V1+) may *phrase* the chain; it may not *add* unevidenced links — a link with no ledger/artifact support renders as unverified, exactly like any other unverified claim.
3. "Earliest meaningful cause" is defined operationally (v2-14 §4): walk ancestors of the failure fact; a node is *meaningful* if it belongs to an actionable class (decision, context omission, plan defect, envelope denial, schema failure, retry-without-hypothesis) and passes at least one meaningfulness test (divergence vs. reference class, information-availability, first-defect). The earliest such node is the primary hypothesis; runners-up are reported with scores.

## Reason

True counterfactual causality is unknowable without re-execution; pretending otherwise would poison the product's core honesty. Ranked, evidence-linked hypotheses are what a principal engineer actually produces in a post-mortem — Chronicle mechanizes that craft rather than simulating an oracle.

## Benefits

The index makes every forensic query a graph traversal (fast, local, explainable); confidence and `derived_by` keep heuristics inspectable; the claims discipline prevents the forensic narrator from becoming a new injection/hallucination surface.

## Trade-offs

Heuristic edges will sometimes be wrong — visible, scored, and correctable (user dismissal of an edge is recorded and feeds rule tuning). The index adds a derived table to maintain — rebuildable per ADR-0011, so its failure mode is regeneration, not corruption.

## Migration impact

None to write paths: the index derives from existing entry types plus the pack selection manifests that v2-04 already captures. The manifests' *omitted-with-reason* lists — designed for context debugging — become the substrate of the "missing context" forensic class, unchanged.
