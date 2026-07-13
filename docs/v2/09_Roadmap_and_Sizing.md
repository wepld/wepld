# v2-09 — Roadmap and Sizing

Principle preserved from v1: phases exit on evidence, not calendar. Change from v1: every epic carries an effort estimate so the plan is falsifiable (gate finding H6). Estimates are in **engineer-weeks (ew)**, ±50% honesty band, for a senior team; they exist to expose order-of-magnitude mismatches, not to promise dates.

**Team plan:** 4 engineers (Core ×2, Worker/Sandbox ×1, Studio ×1) + fractional product/design and security review. Below 3 engineers, timelines roughly double; the scope does not shrink further without losing the thesis instruments.

## Phase A — Proof and freeze (~5–7 weeks, runs partly in parallel)

| ID | Deliverable | Exit evidence | Est |
| --- | --- | --- | --- |
| S1 | Linux sandbox spike (namespaces+Landlock+cgroups, canary self-test) | forbidden read/write/egress observably denied | 2 ew |
| S2 | macOS Seatbelt spike + Windows S3/WSL2 probe | tier table (v2-05) confirmed or amended per OS | 3 ew |
| S3 | SQLite state+ledger spike: crash mid-transaction, fold-checker, 10k-entry timeline render | recovery drill green; render < 1 s | 2 ew |
| S4 | Brain structured-output evaluation (2 adapter families × plan/build/review fixtures) | schema-validity ≥ target; cost/latency baseline published | 3 ew |
| S5 | Studio transport spike (loopback+token+SSE) and shell decision input (Tauri vs. browser) | decision recorded as ADR-0011 | 1 ew |
| **E1** | **Orchestration-thesis experiment** (ADR-0002): ≥20 fixture missions on 3 OSS repos; Arm A single-runtime role-switching with context isolation; Arm B same runtime *without* review isolation; Arm C split-context full handoffs | **pre-registered decision rule:** keep review isolation if Arm A ≥ Arm B on acceptance-rate without >25% cost overhead; revisit ADR-0002 only if Arm C beats A on quality *and* cost | 4 ew |
| P1 | Positioning + business-model hypothesis (vs. Claude Code, Cursor, Devin, OpenHands, GitHub agents) | reviewed; wedge messaging (ADR-0010) confirmed or amended | 2 ew |
| C1 | Contract freeze v0 (the nine contracts, WWP fixtures, ledger vocabulary) | conformance fixture suite exists and runs | 3 ew |

**Phase A gate:** all spikes green or tier table amended honestly; E1 decision applied; contracts frozen. ~20 ew total ⇒ ~5–7 calendar weeks at 4 engineers.

## Phase B — Build the MVP (~16–20 weeks)

| Epic | Contents | Est |
| --- | --- | --- |
| B1 Core skeleton | process lifecycle, command pipeline, work queue, state machine, ledger + fold-checker | 6 ew |
| B2 Artifacts + worktrees | CAS store, Git worktree manager, scope re-verification, snapshot/recovery | 4 ew |
| B3 Sandbox envelopes | S1 + S2 + S3/S2W launchers, tier detection/self-test, envelope grants & extensions | 6 ew |
| B4 Hermes + WWP | runtime phase loop, stdio transport, conformance suite green | 6 ew |
| B5 Brain gateway | 2 adapters, profiles, budget/cost ledger, failure taxonomy handling | 4 ew |
| B6 Context Assembly | tiers, selection+manifest, compression, redaction, pack capture | 6 ew |
| B7 Gates + phases | Core-run build/test checks, review gate, retry-with-hypothesis | 4 ew |
| B8 Decisions + Messenger | packet lifecycle, batching, interrupt budget, claims rendering | 4 ew |
| B9 Studio | Mission / Timeline / Decisions surfaces, SSE live state, replay view | 8 ew |
| B10 Knowledge + skills (MVP) | typed records + FTS; skill dir resolution into packs | 3 ew |
| B11 Hardening | the eight v1 critical acceptance scenarios (doc 26) adapted to MVP; chaos drills (kill −9 matrix) | 5 ew |

~56 ew ⇒ ~16–20 calendar weeks at 4 engineers with integration overhead. **Phase B gate = v1's M1–M3 proofs compressed:** crash recovery drill, deny-path drill, one real mission end-to-end matching v2-08 including the crash variant.

**Phase B+ — Chronicle MVP (+~10–11 ew, may begin once B1/B2 land).** The Engineering Intelligence pillar (v2-11) layers onto the substrate as a read-side: snapshot refs, checkpoints/state_at, frames + player + scrubber (absorbing the B9 Timeline surface — net UI cost is partially shared), causal walk, fork + decision revision, two-point comparison, stats panel. Sizing per capability in [v2-11](11_Chronicle_Overview.md); solo-founder buildable because it adds zero write-path complexity beyond ADR-0013's phase-boundary commits.

## Phase C — Design partners (~8 weeks, engineering continues at half throttle)

10–20 solo-professional users (ADR-0010), 3+ missions each on their own repos. Instrumentation per v2-01 §metrics; weekly cohort review. **Phase C gate = the thesis readout:** acceptance rate, interrupts/mission, evidence engagement, provider-swap fixture pass. This readout — not enthusiasm — authorizes V2 investment.

## V2 — Earn-back (triggered, not scheduled)

Each deferred v1 system returns when its trigger fires, through its named seam:

| Deferred system | Trigger | Seam |
| --- | --- | --- |
| Worker fleet + parallel tasks | missions demonstrably serialized on independent tasks; users ask for throughput | WWP over socket; footprint-disjointness rule: parallel only when declared `scope.paths` of ready tasks are disjoint, else serialize — merge conflicts stay a non-event by construction |
| Generalized policy engine | ≥3 cohort requests for envelope rules the hard-gate table can't express | replace table with rule evaluation at the same two enforcement points |
| Full-Auto / Enterprise presets | cohort trust data supports it | new presets over ADR-0009 mechanism |
| Skill registry/signing/evolution | first third-party or cross-project skill demand | fields added to v2-07 §9 descriptor |
| Knowledge extraction + semantic retrieval | T3 hit-rate measurably poor; record volume grows | new ranking signals inside v2-04 step 4 |
| Review-oriented IDE surface | users leave Studio to inspect diffs in editors (measured) | new workspace over the same ledger/artifact reads |
| Second seat (reviewer/approver link) | design partners request human co-review | Decision authority field already generalizes |
| Channels (Telegram/Slack/email) | pull from cohort for away-from-desk decisions | Messenger contract already channel-shaped |
| Event-sourcing promotion, sync, remote workers | multi-device/team demand | v2-06 promotion path; envelope signing seam in v2-07 §2 |
| Tauri/native shell packaging | ADR-0011 (from S5) | Studio is transport-decoupled already |

## What is deliberately absent from all phases above

Marketplace, cross-customer learning, autonomous production deployment, portfolio intelligence, simulation/forecasting — H4/H5 horizons in v1 doc 29, untouched and unscheduled, exactly as the vision documents them. The road exists; it is simply not paved before the first mile is driven.
