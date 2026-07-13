# IMPL-07 — Implementation Risk Register

Owner for every row: the founder. Review: at every milestone tag. Likelihood/Impact on a High/Med/Low scale; every row has a trigger (observable, not vibes) and a mitigation that exists in this program (not a wish).

## Program-level risks

| Risk | L | I | Trigger | Mitigation |
| --- | --- | --- | --- | --- |
| **Solo bus factor / burnout** | M | Critical | two consecutive weeks without a merged PR | sustainable pace baked into the timeline (4.5 d/wk, pessimistic bands); demo-Friday rhythm makes progress emotionally visible; everything is written down (this program + DECISIONS.md) so a pause isn't a loss |
| **Scope creep breaking the freeze** | H | H | a PR introduces a concept with no v2 anchor | gap-note rule (IMPL-00); dependency-cruiser + vocabulary-lock make many drifts mechanical failures; backlog file absorbs ideas without acting on them |
| **Model quality insufficient for planner/builder on real repos** | M | H | M1 missions need >2 retries or hand-holding on fixture repos | fixture repos are small by design; Manual mode is the fallback product posture; E1-lite at M5.5 informs topology; thesis readout (Phase C) is the honest arbiter — this risk is *the* research risk and cannot be engineered away, only measured early |
| **Market timing** (incumbents ship governance surfaces) | M | H | a major vendor ships mission-control + evidence gating | speed via this program's cuts; moat is the ledger/Chronicle substrate (v2-16 §5) — keep every milestone shippable so partial value exists at all times |
| **Token spend during development** | M | M | monthly bill > budget line | cassettes for all CI and most dev loops (IADR-0002); local model via openai-compat adapter for cheap iterations; budget guard is a product feature that also protects the founder |
| **better-sqlite3 / native module friction** | M | L | install failure on founder or partner machine | pinned prebuilds; `node:sqlite` fallback documented; store code isolated in `ledger` behind one interface |

## Per-milestone risks

| M | Top risks | Mitigation |
| --- | --- | --- |
| M0 | WWP plumbing edge cases (stdio buffering, zombie processes); hash-chain design mistakes ossifying early | LSP-style framing (well-trodden); process-group kill discipline from day 5; chain format reviewed against v2-06 before Day 2 ends — it is the one thing hard to change later |
| M1 | real-provider structured output flakier than cassettes suggest; context selection quality poor on first contact | schema-retry path is contract behavior (v2-07 §3); assembly-quality fixture (IMPL-06) gives an objective floor; keep tasks small — mission design is a product lever, not just a test lever |
| M2 | interrupt/batching semantics fiddly; review-isolation accidentally leaky via shared temp state | v2-10 semantics are already precise — implement the table, not intuitions; isolation asserted structurally (review pack manifest must not contain builder transcript — a test, not a hope) |
| M3 | UI time sink (the classic) | ship ugly on purpose; component budget: 3 surfaces, ~10 components; Playwright only for the golden path; a week over budget triggers scope cut, not a push |
| M4 | founder-OS sandbox reality (Seatbelt quirks / bwrap availability / WSL2 setup); envelope breaks real toolchains | canary-first development (make the test fail honestly before making it pass); S0 container fallback is the pressure valve; DEV tier remains for development itself (IADR-0003) |
| M5 | recovery classification wrong in an edge case → silent duplicate effect | the crash matrix is scripted at the *six* risk points (mid-txn, post-commit-pre-spawn, mid-phase, mid-gate, mid-merge, mid-fork); envelope confinement (ADR-0004) keeps the common case mechanically probeable |
| M6 | frame semantics churn (taste-driven rework) | generator_version discipline: churn is cheap regeneration, not migration; timebox the frame-rules iteration to one week, refine with real usage later |
| M7 | invalidation cone correctness (false carries / over-invalidation) | MVP carries nothing (ADR-0012 stance) — over-invalidation is the safe default and the golden asserts the exact set; salvage stays V1 |
| M8 | onboarding friction with strangers; second-OS surprise | install-time doctor command (checks git, node, tier prerequisites); cohort filtered to supported OS or S0 per IADR-0005 — stated openly in the invite |

## Standing unknowns (tracked, not blocked on)

Whether solo pace sustains the expected band (checkpoint: M3 tag date vs. plan — slip >2 wk forces a scope council with self); whether design partners accept fixture-repo onboarding before their own repos (checkpoint: M8 cohort feedback); whether the openai-compat local path is good enough to matter for privacy-sensitive partners (checkpoint: M1 adapter conformance results); Seatbelt deprecation horizon (checkpoint: each macOS release — S0 fallback ready per ADR-0007).
