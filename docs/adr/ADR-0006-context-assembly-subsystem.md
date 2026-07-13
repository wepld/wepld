# ADR-0006 — Context Assembly is a first-class Core subsystem

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Core owner · **Review:** after first design-partner cohort

## Context

v1 said brain requests carry "context references, not unbounded prompt history" but assigned prompt construction to no bounded context. The gate review (H2) identified context assembly — deciding what enters the model's window, under what budget, with what compression and redaction — as the dominant quality lever in agentic coding and the most likely place for hidden coupling if left to individual workers or adapters.

## Decision

Context Assembly is a named Core subsystem with an owned port: `assemble(mission, task, phase, role, budget) → ContextPack`. Workers never build their own prompts from raw sources; they request packs. Every pack is serialized, hashed, stored as an artifact, and referenced by the brain invocation record. Design in [v2-04](../v2/04_Context_Assembly.md).

Responsibilities: tiered selection (pinned brief/criteria/skills → task state → repository content → knowledge → compressed history), token budgeting with loud failure when pinned content overflows, summarization of prior phases and attempts, secret/classification redaction before egress, and full capture for replay.

## Reason

Centralizing the highest-leverage quality mechanism makes it testable, comparable across brain profiles, and auditable (principle 11: replayability requires knowing exactly what the model saw). It also gives the redaction requirement of v1 doc 06 an actual enforcement point.

## Benefits

One implementation instead of N hidden ones; context quality becomes a measurable, versioned artifact (`pack_schema_version`); replay and cost attribution get their missing substrate; the review phase's independence (ADR-0002) is enforced here by construction.

## Trade-offs

A central assembler can become a bottleneck for experimentation; mitigated by making selection strategies pluggable per role profile while the pack format and capture pipeline stay fixed.

## Migration impact

None to other contracts: WWP workers already receive `context_pack_ref` in `attempt.start`. Future retrieval improvements (semantic index, cross-project knowledge) plug into tier T3 without changing the pack format.
