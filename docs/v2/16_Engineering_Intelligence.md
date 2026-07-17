# v2-16 — Engineering Intelligence and Learning

Replay becomes intelligence when the record starts improving future engineering. The flywheel is a mechanism, not a slogan, because every stage already has a seam: **ledger → insight scan → lesson candidate → human approval → knowledge record → Context Assembly T3 → better packs → better missions → better ledger.**

## 1. The insight pipeline

An idle-time scanner (and on-demand run) computes findings over one or many missions and writes them to `insights` (derived, rebuildable). Every insight is a claim with evidence refs and a proposed action — the claims discipline applies to the system's opinions about itself.

~~~json
{ "schema_version": 1, "insight_id": "ins_…", "class": "recurring_context_omission",
  "statement": "3 of 4 failed missions touching src/queue/** omitted docs/queue-semantics.md from planning packs",
  "evidence": [{ "mission": "mis_A", "edge": "ce_…" }, { "mission": "mis_C", "edge": "ce_…" }, { "mission": "mis_F", "edge": "ce_…" }],
  "confidence": 0.83, "derived_by": "rule:omitted_then_implicated/agg",
  "proposed_action": { "kind": "lesson_candidate",
    "draft": { "type": "lesson", "title": "Pin docs/queue-semantics.md for planning on src/queue/**",
               "paths": ["src/queue/**"], "body_md": "…", "sources": ["ins_…"] } },
  "status": "open" }   // open | promoted | dismissed(reason)
~~~

`PromoteInsight` (human command) creates the knowledge record with the insight as source; `InsightRecorded` / dismissals land in the ledger — the system's learning is itself auditable and replayable.

## 2. Insight classes and their derivations

| Class | Derivation (all local SQL/graph — no model required) | Tier |
| --- | --- | --- |
| Recurring failure signature | failure-taxonomy class × path/task-type signature, count ≥ N across missions | V1 |
| Recurring success pattern | plan shapes / skill sets over-represented in accepted missions vs. rejected | V2 |
| Recurring context omission | aggregated `omitted_then_implicated` edges (v2-14 §3) | V1 |
| Provider reliability | per profile/intent: schema-failure rate, retry rate, cost, latency, downstream gate pass rate — from `brain_invocations` joins | V1 |
| Worker-role reliability | phase outcome rates by role profile version | V1 |
| Decision latency | requested→resolved gaps; which packet classes stall missions | V1 |
| Slow/high-cost phases | phase duration and cost distributions; outliers flagged with their frames | MVP (stats panel) |
| High-risk phases | phases with elevated envelope-extension and escalation density | V1 |
| Retry futility | retries whose hypothesis field didn't change or whose outcome didn't improve | V1 |
| Knowledge gap | RCA chains ending in `context_omission` with no matching knowledge record | V1 |
| Architecture drift | declared scope patterns vs. actually-touched paths across missions; undocumented hot dependencies | V2 |
| Strategy effectiveness | branch comparisons where one strategy systematically beats another (needs branch volume) | V2 |

MVP ships only the stats panel (cost/time/interrupts/gates per phase and per mission) — deliberately humble, immediately useful. V1 ships the scanner with the five highest-signal classes. Everything upgrades by adding rules, never by re-architecting: an insight class is a query plus an evidence template.

## 3. Guardrails (what Intelligence must never become)

- **No silent self-modification.** Insights change nothing by themselves; promotion is human, recorded, and reversible by supersession — the v1 skill/knowledge evolution rule, preserved.
- **No surveillance framing.** Metrics aggregate *missions*, not humans; decision-latency insights describe packet classes, not the person (the v1 doc-29 "evidence, not surveillance" guardrail, applied inward).
- **No cross-project leakage.** Insights compute within a project scope; cross-project and cross-org learning are Future, gated on the consent/provenance research agenda of v1 doc 29.
- **No confidence theater.** Every insight carries its rule, sample size, and confidence; small-N findings render as anecdotes, not laws.

## 4. Learning — the record as teacher

The same substrate teaches humans:

| Artifact | What it is | Tier |
| --- | --- | --- |
| Mission lesson | promoted insight or manually authored lesson with frame/evidence sources | MVP (records exist) |
| Decision summary | auto-drafted "what was decided and why" per mission from beat frames — Messenger prose over ledger facts | V1 |
| Failure/success analysis | the RCA report (v2-14) as a shareable document | V1 |
| Mission comparison | the facet diff (v2-15 §6) as the "what changed / why was this better" explainer | MVP |
| Annotated replay | user notes pinned to frames (`AnnotationRecorded` → artifacts); a reviewed mission becomes a commentary track | V1 |
| Replay bundle | exportable signed clip (ledger slice + artifacts + frames, redaction re-applied) — the portable unit of engineering teaching | V1 |
| Interactive tutorial | curated bundle with stops and questions ("what would you have decided?") — onboarding that plays real missions | V2 |
| "What would happen if…" | **a real branch**: fork the point, change the input, run under budget, compare (v2-15). Chronicle's what-if is never a simulation — it is inexpensive honest experiments | MVP (as branching) / V2 (guided assistant) |

## 5. The compounding claim, stated precisely

WePLD's long-term moat is not that it runs agents; it is that after a year of use, a project's Chronicle contains: every decision with rationale and revision history, every failure with a ranked cause, every recurring omission converted into pinned context, provider reliability measured on *this* codebase, and a library of replayable precedents. None of that can be cold-started by a competitor's model upgrade — it accrues only through governed operation. Chronicle is where the operating system's memory becomes leverage.
