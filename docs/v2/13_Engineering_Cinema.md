# v2-13 — Engineering Cinema

Not a timeline. A **screening room for engineering**: the place where a mission plays as the organization actually lived it — what was known, seen, decided, built, proven — with the controls of an editing suite and the discipline of a flight recorder. The v2 Timeline surface is *absorbed*: it survives inside Cinema as the **Contact Sheet** (the dense forensic list, one click away), but the primary experience is the screening.

Design honesty rule (inherited from ADR-0011 and v2-10): every pixel renders a recorded fact or is visibly labeled as narration. Cinema dramatizes real data through focus and pacing — never through invention. "Watching the organization think" is literal: the actual pack sections a brain saw, the actual structured output it returned, the actual envelope it worked inside.

## 1. The stage — layout

~~~text
┌────────────────────────────────────────────────────────────────────┐
│ ORIENTATION BAR   mis_…F "Add rate limiting" · branch: main-line   │
│ ▸ scene 41/58 · seq 3015 · phase BUILD · ⬤ HISTORICAL (ended 61m)  │
│ here because: you clicked "Why?" on the failed gate → cause #2     │
├──────────────────────────────┬─────────────────────────────────────┤
│                              │  STATE INSPECTOR                    │
│         THE SCREEN           │  mission: RUNNING → WAITING_DECISION│
│  (current frame rendering)   │  interrupts: 1/3 · cost: $1.12      │
│                              │  criteria: AC1 ◐  AC2 ○  AC3 ○      │
│                              ├─────────────────────────────────────┤
│                              │  CONTEXT / EVIDENCE / KNOWLEDGE     │
│                              │  (drawer tabs for the frame)        │
├──────────────────────────────┴─────────────────────────────────────┤
│ ◀◀  ◀  ⏸  ▶  ▶▶   speed 1×   ⦿ follow-live    clip [ ]  fork ⑂   │
│ SCRUBBER                                                           │
│ Decisions   ──────♦────────♦♦───────────────♦──────────♦────────   │
│ Evidence    ────────▮───▮─────────▮▮────▮──────────▮▮──────▮────   │
│ Execution   ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬   │
│ Brain       ──⚡───⚡⚡────⚡──────⚡───⚡─────⚡⚡──────⚡─────────   │
└────────────────────────────────────────────────────────────────────┘
~~~

### The Screen (MVP)

Renders the current frame as a composed shot: **who** (actor chip: "Builder · att_b1 · envelope: worktree-rw, net-deny"), **what** (the frame's focus — a pack section coming into view, a diff hunk appearing, a decision packet opening, a gate verdict landing), **with what authority** (envelope/tier chips), **at what cost** (running cost/token odometer). Scene frames play their folded interior as a paced montage of their significant sub-moments. Beat frames (decisions) hold on the packet — options, recommendation, the human's recorded rationale — because decisions are the dramatic core of the medium.

### The Scrubber (MVP)

A layered track bar, one row per enabled lens (v2-12), glyphs positioned by sequence with phase-boundary tick marks. Interactions: click = seek; drag = scrub with live Screen preview; brush a range = range summary chip (duration, cost, decisions, gate outcomes — the seed of V2 heatmaps); toggle tracks to solo a lens (this *is* "replay only decisions/evidence/…"). Zoom: mission → phase → frame.

### State Inspector (MVP)

Always-current `state_at(playhead)`: mission/task state machine positions, acceptance-criteria matrix, open decisions, budget odometers, effective envelope. This is the "digital twin at time t" — the same fold the runtime itself trusts, so it cannot disagree with reality.

### Drawers (MVP)

Context Viewer (the pack of the focused invocation, selection manifest included — including what was *omitted and why*), Evidence Viewer (artifacts with hash/provenance), Knowledge Viewer (records cited in the frame). All reuse existing Studio components.

## 2. Cameras (MVP: manual · V2: auto-director)

A **camera is a focus policy** — a rule for what the Screen emphasizes as frames advance:

| Camera | Follows | Typical viewer |
| --- | --- | --- |
| Mission Camera | the state machine: phases, gates, budget — the executive's shot | "is this on track?" |
| Engineering Camera | artifacts and code: diffs growing, tests appearing, refs advancing | "what is being built?" |
| Decision Camera | the human loop: packets, deliberation gaps, resolutions | "where was I needed?" |
| Reasoning Camera | packs and invocations: what the brains saw and returned | "why did it think that?" |

MVP: cameras are presets over lens weights + drawer defaults, switched manually. V2's **auto-director** scores each frame's tracks and cuts to the most informative camera — a pure read-side heuristic, safely deferred.

## 3. Maps and graphs (V1 unless noted)

- **Mission Map** — the organization as a stage: stations for Planner / Builder / Validator / Reviewer / Gates / Decision Queue / Human; the active attempt is a token moving between stations as frames advance; envelope changes redraw the station's boundary. This is the digital-twin view, derived entirely from real transitions.
- **Mission Graph** — the plan's task DAG with live status, linkable from any frame (MVP ships a static version inside the Mission surface already).
- **Decision Graph** — decisions as nodes, causal in/out edges from the index; deliberation time rendered as node weight.
- **Branch Graph** — the lineage tree (v2-15): forks, their reasons, their outcomes, adoption marks.
- **Contact Sheet** (MVP) — the dense sequential list of frames/entries with full filters: the forensic fallback that guarantees no information is *only* cinematic.

## 4. Split view and comparison (V2; two-point compare is MVP)

Two players locked to a shared logical clock: pre-fork, aligned by sequence; post-fork, aligned by (task index, phase). Divergence markers appear on the scrubber where lenses first differ (the v2-15 divergence finder feeds this). MVP ships the static two-point comparison (v2-15 §6); the synchronized dual *playback* is the V2 theatrical upgrade of the same data.

## 5. The six questions — answered structurally

The orientation contract: at any instant, without navigation, a viewer can answer —

| Question | Answered by |
| --- | --- |
| Where am I? | Orientation bar: mission · branch · scene/seq · phase · live/historical |
| Why am I here? | the **provenance breadcrumb**: every navigation records its cause ("via RCA hypothesis #2", "via decision dec_2", "via divergence marker") — Cinema applies causation discipline to its own UI |
| What happened? | the Screen's current shot + scene title |
| What changed? | the frame's delta chips (state transitions, files, budget) against the previous frame |
| Why did it change? | the frame's `caused-by` link — one click walks the causal edge (v2-14) |
| How do I continue? | the action rail: resume live · fork here ⑂ · revise this decision · open forensics · export clip |

## 6. What Cinema refuses to do

No invented animation of "thinking" beyond recorded packs and outputs; no synthetic confidence theater (uncertainty fields render as uncertainty); no green that the ledger can't back (v2-10 rules apply to every badge); no hiding of the Contact Sheet — the cinematic layer is a lens on the record, never a replacement for it.
