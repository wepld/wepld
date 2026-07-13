# IMPL-03 — Implementation Order, Milestones, Timeline

## Implementation order (and why)

> **IADR-0007 §3:** within every milestone the build order is Contracts → Core → Runtime → Storage → Verification → API → UI. The UI is always the final consumer; never begin a milestone with it.

The order is *thin versions first, in dependency order, until the spine lives — then deepen by demo value*:

1. **`contracts`** — everything imports it; transcribing v2-07 into zod is also the final contract review.
2. **`ledger`** — the spine; nothing meaningful demos without durable facts. Its fold/verify tests are the program's first safety net.
3. **`cli` (skeleton)** — immediately makes the ledger *visible* (`wepld timeline`). Visibility before features, always.
4. **`artifacts` + `workspace`** — evidence and isolation; snapshot refs from day 4 so every later mission is Chronicle-ready retroactively (ADR-0011's retroactivity is free only if refs exist from the start).
5. **`wwp` + `hermes` (skeleton)** — the process boundary must exist before habits form around in-process shortcuts.
6. **`brains` (fixture adapter)** — deterministic reasoning unblocks everything downstream without keys or cost.
7. **`runtime` (thin)** — command pipeline, minimal state machine, one-task phase engine, trivial gate. **← M0 demo lives here.**
8. Then by milestone: `context` v0 + real adapters (M1) → gates/review/decisions (M2) → `studio-api` + `studio` (M3) → `sandbox` real tier (M4) → recovery hardening (M5) → `chronicle` (M6–M7) → packaging (M8).

Rationale for the two most debatable calls: **Studio (M3) before sandbox (M4)** — demos and validation compound earlier, and IADR-0003 makes the interim honest; **Chronicle last (M6–M7)** — it is read-side over data that must exist first, and its substrate (refs, packs, invocations) is being laid correctly from M0, so nothing is lost by sequencing it late.

## Milestones

Every milestone: ends with something **executable, observable, testable, demonstrable**; has golden-trace coverage; is a git tag with a recorded demo.

| M | Name | Weeks (exp.) | Demo at exit | Definition of Done |
| --- | --- | --- | --- | --- |
| **M0** | The Spine | 1–2 | `wepld demo` runs a fixture mission end-to-end in the terminal: brief → plan → approve → build → gate → accept → timeline | golden `m0-first-mission` green; chain-verify + fold-check in CI; WWP over real child process; pack captured (v0); DEV tier recorded & displayed; all IADRs merged |
| **M1** | Real Brain, Real Repo | 3–4 | mission "add a function + its test" on fixture repo with a *hosted* model; timeline shows packs, invocations, live cost | both real adapters pass cassette-recorded conformance; context v0 (T0/T1/T2-seeds + manifest + capture + T0-overflow loud); budget projection blocks over-spend; cassettes refreshed via `tools/record-cassettes` |
| **M2** | Gates, Review, Decisions | 5–6 | mission hits a real ambiguity → decision packet in CLI inbox → resolve → continues → independent review phase → completion blocked until evidence | Core-run build/test gates (exit-code + log artifacts); review phase in fresh process w/ context isolation (no builder transcript — asserted by test); interrupt budget + batching per v2-10; `envelope.extend` flow; golden `v2-08-rate-limiting` (happy path) green |
| **M3** | The Studio | 7–9 | **Browser demo:** create mission in Studio form, watch live status via SSE, resolve decisions in inbox, accept with evidence links | studio-api token auth + routes per v2-17 subset; Mission/Decisions/Contact-Sheet surfaces; claims rendering (verified chips vs. unverified prose); no client-side state mutation; Playwright E2E of the full flow |
| **M4** | Honest Sandbox | 10–11 | same mission, now inside a real envelope: a planted `curl` in the fixture repo is *observably denied*; tier shown in Studio | founder-OS tier implemented + canary self-test; ADR-0007 caps enforced; DEV-tier default removed (IADR-0003 DoD); deny-path golden trace; envelope quotas (mem/procs/time) kill runaway fixture |
| **M5** | Survives Failure | 12–13 | kill −9 the Core mid-build on stage; restart; recovery classifies, resumes, timeline explains itself | crash matrix (kill at 6 scripted points) green incl. golden `v2-08` crash variant; work-queue at-least-once verified; adversarial golden `injection` green; **E1-lite topology experiment run on cassettes, result recorded in DECISIONS.md** |
| **M6** | Cinema (Replay) | 14–16 | **North Star Demo** — full flow + replay: scrub the mission, watch packs/decisions/evidence as frames; live-follow a running mission, detach, jump-to-live | checkpoints + `state_at`; frames deterministic (regeneration byte-identical); 4 lenses; player/scrubber absorbing the Timeline surface; session state machine incl. FOLLOWING/detach |
| **M7** | Fork & Why | 17–19 | revise a past decision live: fork → invalidation report → re-plan → compare A/B side-by-side; click "Why?" on the gate result and walk its causes | golden `v2-18-decision-edit` green; causal walk (deterministic edges); ForkMission/ReviseDecision commands; comparison doc (files/decisions/cost/evidence facets); lineage + branch listing; stats panel |
| **M8** | Design-Partner Preview | 20–22 | a stranger installs WePLD in <10 min and completes their first mission on a fixture repo, then their own repo | install script + docs + onboarding mission; second sandbox tier *or* S0 container fallback (cohort-OS driven, IADR-0005); provider-swap fixture green on both adapters; crash-recovery drill doc; feedback instrumentation (v2-01 metrics, local, consented); tag `v0.1.0-preview` |

## Build timeline

Solo, sustainable pace (~4.5 productive days/week). Ranges are honest, not decorative:

| Phase | Optimistic | Expected | Pessimistic |
| --- | --- | --- | --- |
| M0 | 2 wk | 2 wk | 3 wk |
| M1–M2 | 3 wk | 4 wk | 6 wk |
| M3 | 2 wk | 3 wk | 5 wk (UI always slips) |
| M4–M5 | 3 wk | 4 wk | 7 wk (sandbox is the risk pool) |
| M6–M7 | 4 wk | 6 wk | 8 wk |
| M8 | 2 wk | 3 wk | 5 wk |
| **Total to preview** | **16 wk** | **~22 wk** | **34 wk** |

Consistency with the frozen roadmap: v2-09 sized the 4-engineer Phase B at ~56 ew including three OS tiers, full hardening, and team overhead; this program reaches preview at ~22 solo ew by cutting to one OS tier (IADR-0005), cassette-first testing (IADR-0002), and a narrower Studio — cuts the cohort can absorb. Phase C (design partners, ~8 wk) follows M8 unchanged from v2-09; the thesis readout remains the program's finish line.

**Value ratchet check** (the "every milestone increases product value" requirement): M0 proves the spine exists → M1 proves it reasons → M2 proves it governs → M3 makes it usable → M4 makes it safe → M5 makes it trustworthy → M6 makes it explainable → M7 makes it editable → M8 makes it shareable. No milestone is invisible infrastructure; each is a sentence a design partner would care about.
