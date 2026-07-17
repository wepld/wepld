# IMPL-04 — Sprint 1: The Spine (M0)

> **Amended by [IADR-0006](adr/IADR-0006-rust-core-tauri.md):** day structure unchanged; tools translate per the table (Cargo workspace, serde/schemars contracts, rusqlite ledger, cargo test + proptest, Rust bins). Windows hosts develop under WSL2.

**Goal, verbatim from the program mandate:** Open WePLD. Create one Mission. Execute one Mission. Record the Ledger. Complete the Mission. Nothing more.

**Interpretation:** "Open WePLD" = the CLI (the Studio is M3). Ten working days. Everything not needed for one honest mission is deferred — no sandbox beyond the declared DEV tier, no real provider, no parallelism, no UI, no Chronicle. But **no fakes on the spine**: the ledger hash-chains for real, Hermes is a real child process over real WWP, the pack is really captured, the gate really runs a command, and the whole run replays as a golden trace forever after (retroactive replayability, ADR-0011).

## The Day-10 demo script (written first — the sprint builds backward from it)

~~~text
$ wepld init ./fixtures/repos/notes-cli        # refuses OneDrive/Dropbox paths, per v2-06
✓ store created (tier: DEV — no isolation; Manual mode only)

$ wepld mission new -f fixtures/missions/add-version-flag.json
mis_01J… DRAFT → PLANNING          # brief: outcome, scope, 2 acceptance criteria, gates [build,test]

$ wepld mission plan mis_01J…      # planner phase: hermes spawned, fixture brain, pack captured
PLAN_REVIEW  plan v1: 1 task — "add --version flag + test"
$ wepld plan approve mis_01J…

$ wepld mission run mis_01J…       # build phase in isolated worktree; then Core-run gate
task tsk_1 → build (att_01J…)      ✓ diff art_9f2… (+18/−0, 2 files)
gate build ✓ (exit 0, 3.1s)  gate test ✓ (5 passed)
COMPLETION_PROPOSED  AC1 ✓ AC2 ✓   cost $0.00 (fixture)  interrupts 0/3

$ wepld mission accept mis_01J… --merge
MissionAccepted  merge e71c…

$ wepld timeline mis_01J…          # 23 entries, chain VERIFIED, every artifact openable
$ wepld verify                     # hash chain + fold-check: state == fold(ledger) ✓
~~~

That terminal transcript — brief in, evidence out, decision explicit, history verifiable — already feels like WePLD and nothing like an AI IDE. It is also golden trace `m0-first-mission` from Day 9 onward.

## Day-by-day

| Day | Build | Proof at end of day |
| --- | --- | --- |
| **1** | Repo scaffold: pnpm workspace, tsconfig strict, vitest, dependency-cruiser rules (IMPL-02's five), CI (test+boundaries). `contracts` v0: mission, ledger entry + event enum (rev 2), envelope, WWP subset (attempt.start, heartbeat, brain.request, artifact.put, phase.result, cancel) | CI green on a schema round-trip test; vocabulary-lock test in place |
| **2** | `ledger`: open (WAL, synced-folder guard), `transact/append` with hash chain, tables (missions, plans, tasks, attempts, decisions, commands, work_queue), `verifyChain`, fold reducer v0 + fold-check test | property test: 500 random valid transitions → fold == tables; chain verifies; tampering detected |
| **3** | `cli` skeleton: `init`, `mission new` (CreateMission through a minimal command pipeline: idempotency + validation + transition), `timeline` (pretty ledger printer), `verify` | first visible artifact: create a mission, print its timeline — demo #1 recorded (yes, on day 3) |
| **4** | `artifacts` CAS (put/get/verify) + `workspace` (createWorktree, snapshot ref, diff, changedPaths, branchFrom) | unit + integration: worktree lifecycle on the fixture repo; hidden refs visible via `git for-each-ref` |
| **5** | `wwp` framing (server/client, heartbeat watchdog) + `hermes` skeleton (connect, heartbeat, echo phase.result) + runtime spawns it for a stub phase | round-trip test: spawn → attempt.start → heartbeat ×2 → phase.result → AttemptCompleted in ledger; kill hermes → AttemptUncertain |
| **6** | `brains`: gateway (validate → route → schema-check → record) + fixture adapter (cassette replay, loud miss); pack v0 = brief+task+criteria serialized, **captured to CAS, hash on the invocation row** | brain round-trip from hermes via Core; invocation row complete; pack retrievable by hash |
| **7** | Planner phase for real: pack v0 → fixture brain → PlanProposed; `plan approve` command; task creation | `wepld mission plan` + approve works; golden fragment (planning slice) asserted |
| **8** | Build phase: hermes applies the brain's structured edit script to the worktree (write files per output schema `builder_step.v1`), diff artifact, snapshot ref; gates v0: Core runs `build`/`test` commands from the brief, exit-code + log artifact → GateEvaluated | end-to-end run reaches COMPLETION_PROPOSED with real gate logs |
| **9** | `accept --merge` (git merge of mission branch — the completion hard gate), full golden trace `m0-first-mission` (normalizer + expected file), crash micro-drill: kill −9 during build → restart → AttemptUncertain + snapshot recorded (recovery *classification* only; auto-resume is M5) | golden green in CI; crash leaves an explained, verifiable ledger |
| **10** | Polish the demo script, `wepld demo` one-shot wrapper, record the video, tag `v0.0.1-m0`, write Sprint-2 notes + any gap notes | M0 DoD checklist (IMPL-08) fully ticked |

## Deferred with intent (and where each lands)

Real providers, context selection/redaction (M1) · decision packets beyond plan-approval, review phase, interrupt budget (M2) · Studio (M3) · real sandbox tier (M4) · auto-recovery, retries-with-hypothesis (M5) · frames/replay UI (M6 — but its substrate: refs, packs, chained ledger, is fully laid *this* sprint) · fork/compare (M7).

## Sprint risks (top three)

better-sqlite3 build friction on the founder's machine (buffer: Day-1 afternoon; fallback `node:sqlite` experimental API); WWP process plumbing edge cases on Windows dev machines (mitigate: founder OS decided before Day 1, IADR-0005; Windows founders develop under WSL2 from day one); scope temptation on Day 7–8 ("just add a second task type") — the demo script is frozen on Day 1; anything not in it goes to the backlog file.
