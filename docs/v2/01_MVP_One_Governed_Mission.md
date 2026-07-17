# v2-01 — MVP: One Governed Mission

**Optimization target (fixed):** prove the product thesis with the least engineering effort. Not the smallest product — the smallest *credible experiment*.

**Thesis under test:** *a professional will delegate bounded engineering work to an autonomous system — and accept its output — when the work is isolated, evidence-gated, decision-routed, and replayable.*

Every component below exists to measure that sentence. Anything that does not measure it is cut, with a named seam for its return.

## What the MVP is

One user. One local Git repository. One mission at a time. One worker runtime (Hermes, WWP-conformant) executing governed phases — Understand → Plan → Build → Validate → Review — each phase with its own role profile, sandbox envelope, and context pack. A local Core process owns state, gates, decisions, the ledger, and the Studio UI. Two autonomy modes: Manual and Bounded-Auto.

~~~mermaid
flowchart LR
  H["Human"] --> UI["Studio (local UI)\nMission • Timeline • Decisions"]
  UI --> Core["Core process\nstate machine • gates • ledger\ncontext assembly • brain gateway\ndecision queue • messenger"]
  Core -->|spawn per phase| W["Hermes worker\n(WWP over stdio)\ninside sandbox envelope"]
  W -->|brain.request via Core| P["Providers\nhosted + local"]
  Core --> DB[("SQLite\nstate + ledger")]
  Core --> A[("Artifact store\ncontent-addressed files")]
  W --> WT[("Isolated Git worktree")]
~~~

## The identity checklist — what makes this WePLD and not another agent runner

| Identity element | Present in MVP as |
| --- | --- |
| Mission, not chat | structured Mission brief (outcome, scope paths, acceptance criteria, budget, mode) is the *only* way to start work |
| Engineering organization, not a session | governed phases with role isolation (envelope + context + brain profile per phase), evidence handoffs between phases |
| Evidence before completion | Core-executed build/test gates + independent review phase; completion proposal impossible without ledger-recorded gate passes |
| Human as executive | decision queue with hard gates; interrupt budget; plan approval |
| Studio-first | three surfaces — Mission, Timeline, Decisions. **There is no editor in the MVP.** Diffs are reviewed as evidence artifacts |
| One voice | all agent-to-human text is Messenger output; workers physically lack a user channel (no WWP message reaches a human) |
| Observability & replay | hash-chained ledger; context packs and brain invocations captured; timeline and replay views rendered from ledger only |
| Local-first | everything on disk in the user's app-data directory; only brain API calls leave the machine |
| Vendor independence | two brain adapters behind one contract (one hosted API family, one local OpenAI-compatible endpoint e.g. Ollama/vLLM); swap is a config change |

## Scope table

| In MVP | Explicitly out (seam → returns in) |
| --- | --- |
| Core process: state machine, ledger, gates, decisions | Worker fleet, scheduler, parallel tasks (WWP transport swap → V2) |
| Hermes runtime, WWP v0 over stdio | Third-party runtimes (WWP conformance suite → V2) |
| Envelope enforcement + hard-gate table | Generalized policy engine (replace table with rules → V2) |
| Sandbox tiers per v2-05 | Uniform strong sandbox everywhere (impossible; tiers are permanent) |
| Context Assembly with pack capture | Semantic retrieval, cross-project knowledge (tier T3 plug-in → V2/V3) |
| Brain gateway, 2 adapters, structured output | Router/fallback sophistication, multi-profile disagreement checks (→ V2) |
| Skills as versioned, hash-pinned local packages (directory) | Registry, signing, revocation, marketplace (→ V2/V3) |
| Knowledge as typed Decision/Lesson/Finding records + FTS | Extraction pipeline, claims graph, semantic index (→ V2) |
| Decision queue + interrupt budget + Messenger narration | Channels (Telegram/Slack/email), digest schedules (→ V3) |
| Timeline + replay view | Mission Control health vector, portfolio views (→ V2) |
| Manual + Bounded-Auto | Full-Auto and Enterprise presets (same mechanism → V2) |
| Local web-served Studio with session token | Tauri packaging decision (Phase A spike S5 → V2) |
| Single mission at a time | Concurrent missions (needs footprint rules → V2) |

## The Studio in the MVP

Three surfaces, one agent identity:

1. **Mission** — brief, current phase, plan with acceptance-criteria matrix, envelope and sandbox tier on display, budget burn, interrupt budget remaining.
2. **Timeline** — the ledger rendered causally: every phase, gate, decision, artifact, and brain invocation, each expandable to its evidence. A "Replay" control walks the mission event-by-event showing what each phase saw (context pack) and produced.
3. **Decisions** — the queue: pending packets (with diffs rendered as artifacts for merge decisions), resolved history. This surface *is* Messenger in the MVP.

Served locally by Core over loopback with a per-session token (no unauthenticated localhost endpoint); the desktop-shell decision (Tauri vs. alternatives) is deliberately deferred to a Phase A spike because it is packaging, not architecture.

## What the MVP measures (thesis instrumentation)

| Metric | Question it answers | Target to justify V2 |
| --- | --- | --- |
| Mission acceptance rate | do users accept the output with only review-level changes? | ≥ 60% on fixture-class missions across cohort |
| Interruptions per mission | is governance a tax or a comfort? | median ≤ 3; users rate interrupts "worth it" ≥ 80% |
| Evidence engagement | do users actually open evidence before accepting? | ≥ 50% of acceptances follow evidence views |
| Replay usage | does replayability matter in practice? | qualitative; any organic use is signal |
| Provider swap | vendor independence real? | 100% of fixture missions pass on both adapters |
| Tier acceptance | will users tolerate honest sandbox disclosure? | no cohort attrition attributable to tier messaging |

These are decided *before* building (Phase A) so the cohort cannot be retrofitted into a success story.

## Why this cannot collapse into "another AI IDE"

The MVP contains no editor, no chat box, no file tree. Its only entry point is a mission brief; its only outputs are evidence, decisions, and an auditable timeline. The gravitational pull toward editor features (v1 risk register's top risk) is removed structurally rather than resisted culturally: there is nothing to bolt an editor onto until V2 deliberately adds a review-oriented IDE surface *as a workspace over the same ledger*.
