# v2-08 — Worked Example: one mission, end to end

The rate-limiting mission from v2-07, run in **Bounded-Auto** on an S1 (Linux) machine. Ledger entries shown as `seq · type · essentials`; timestamps and hashes elided. This walkthrough is normative: an implementation that cannot produce this trace does not conform.

## 1. User creates the mission

Ana fills the Mission brief (outcome, scope `src/api/**`, three acceptance criteria, gates build/test/review, Bounded-Auto, $5 / 90 min / 3 interrupts). Studio submits `CreateMission` (command `cmd_001`).

~~~text
3001 · SandboxTierDetected     tier=S1, self_test=passed
3002 · MissionCreated          mis_…F, mode=bounded_auto, budget={5.00,90m,3int}, by=principal_local
~~~

Mission state: `DRAFT → PLANNING`. Core enqueues a planning phase in `work_queue`.

## 2. Mission is planned

Core assembles a **plan-phase context pack** (T0 brief+criteria+planner skill; T2 repo map + middleware/config/test seeds; T3 one knowledge record tagged `api`), spawns Hermes with the *planner* role (read-only envelope), which issues one `brain.request(intent=plan)`.

~~~text
3003 · AttemptSpawned          att_p1, phase=plan, role=planner, envelope=read_only
3004 · PhaseStarted            att_p1
3005 · BrainInvoked            profile=default-plan, pack=art_cp1, tokens=11k/2.1k, cost=$0.29
3006 · ArtifactRecorded        art_plan1, kind=plan
3007 · PhaseCompleted          att_p1, status=succeeded
3008 · PlanProposed            plan v1: T1 "middleware + config" · T2 "tests" · T3 "docs update"; AC matrix attached
~~~

State: `PLANNING → PLAN_REVIEW`. Bounded-Auto still requires plan approval (ADR-0009). Messenger delivers the plan as a `completion`-class-free report; Studio shows the task list against the acceptance-criteria matrix. **Ana approves** (`ApprovePlan`, cmd_002).

~~~text
3009 · PlanApproved            plan v1, by=principal_local
3010 · TaskStarted             tsk_1 "middleware + config"
~~~

## 3. Worker executes — and hits a real ambiguity

Build phase: worktree created from `main@9ac31f2`; builder envelope (worktree rw, network deny, no secrets).

~~~text
3011 · AttemptSpawned          att_b1, phase=build, role=builder, worktree=…/wt-att_b1, base=9ac31f2
3012 · PhaseStarted            att_b1
3013 · BrainInvoked            implement_step, pack=art_cp2 (manifest: 9 files full, 3 excerpted, 14 omitted-listed)
~~~

Hermes needs the `governor` crate → `envelope.extend(dependency_install, reversible=true)`. The mission's declared envelope said `dependency_install: "ask"` → hard gate → decision packet `dec_1` (class `blocking`, interrupt 1/3). Independently it raises the key-vs-IP ambiguity → Core classifies it `blocking` too, but **batches** both packets into one delivery (v2-10 batching rule): one interruption, two questions. Interrupts consumed: 2? No — batching counts one delivery = **1 interrupt** (the budget meters human attention, not question count).

~~~text
3014 · EnvelopeExtensionRequested  att_b1, dependency_install "cargo add governor@0.6"
3015 · EscalationRaised            att_b1, ambiguity key-vs-ip
3016 · DecisionRequested           dec_1 (dependency), class=blocking
3017 · DecisionRequested           dec_2 (key-vs-ip), class=blocking, batched_with=dec_1
3018 · MessageSent                 decision_delivery, claims=[2 verified refs], interrupts 1/3
~~~

Mission: `RUNNING → WAITING_DECISION`. **Ana replies** in the Decisions surface: approve the dependency; choose `key`, rationale recorded.

~~~text
3019 · DecisionResolved        dec_1 approve, by=principal_local
3020 · DecisionResolved        dec_2 option=key, rationale="AC1 wording governs"
3021 · EnvelopeExtensionResolved   granted: cargo add governor@0.6 (network: crates.io class, one-shot)
~~~

## 4. Mission continues

Build completes; the builder's `phase.result` carries the diff artifact and a schema-enforced summary. Validate phase (fresh process, read+execute envelope): **Core itself** runs `cargo build` and `cargo test` as gate checks (v2-02 §6).

~~~text
3022 · ArtifactRecorded        art_diff1 (4 files, +212/−9) · 3023 PhaseCompleted att_b1 succeeded
3024 · AttemptSpawned          att_v1, phase=validate, role=validator
3025 · GateEvaluated           gate=build, pass=true, check=art_buildlog1 (exit 0, 41s)
3026 · GateEvaluated           gate=test, pass=true, check=art_testlog1 (12 passed / 0 failed, includes 5 new)
3027 · PhaseCompleted          att_v1 succeeded
~~~

Review phase: fresh process, **read-only envelope, independent brain profile**, pack contains brief + criteria + diff + test evidence — *not* the builder transcript (ADR-0002).

~~~text
3028 · AttemptSpawned          att_r1, phase=review, role=reviewer, brain=independent-review
3029 · BrainInvoked            review, pack=art_cp4, provider≠build provider
3030 · ArtifactRecorded        art_rev1, kind=review_findings: 1 minor (config name), 0 major, disposition=approve-with-note
3031 · PhaseCompleted          att_r1 succeeded
3032 · TaskCompleted           tsk_1
~~~

Tasks 2–3 run the same loop without interrupts (nothing crosses the envelope). Budget so far: $2.10, 44 min, 1/3 interrupts.

## 5. Mission completes

All tasks succeeded; `VERIFYING` re-checks: every required gate has a Core-produced pass; review disposition recorded; `git diff --name-only` ⊆ scope paths (Core re-verifies — worker claims don't count); budget OK.

~~~text
3040 · CompletionProposed      AC1✓(test ref) AC2✓(test ref) AC3✓(gate ref), diff=art_final, cost=$2.85, 61 min
3041 · MessageSent             completion_proposal, claims all verified w/ ledger refs
~~~

Studio shows the acceptance matrix — each criterion linked to its evidence artifact. Ana opens the test log (evidence engagement metric ticks), then **accepts**. The isolated worktree branch `wepld/mis_…F` is merged per her merge preference (fast-forward into `main` was pre-approved in the brief? No — merge is always a hard gate: the accept dialog includes the merge decision explicitly).

~~~text
3042 · DecisionResolved        dec_3 completion: accept + merge to main, by=principal_local
3043 · MissionAccepted         mis_…F, merge_commit=e71c40a
~~~

A knowledge record is offered (not silently written): "Rate limiting is per API key" (from dec_2). Ana keeps it → `kno_…` with sources `{dec_2, art_rev1}`.

## 6. Timeline

The Timeline surface renders seq 3001–3043 causally: mission → plan → approval → task → attempts → decisions → gates → completion. Every node expands to its evidence; nothing on the screen lacks a ledger ancestor. Filter by "decisions only" shows Ana's four touchpoints — the entire human cost of the mission, inspectable.

## 7. Replay

Replay walks the same entries with hydration: at 3013 it shows the **exact context pack** (art_cp2, selection manifest included) the builder saw; at 3029, what the reviewer saw (and that the builder transcript was absent); at every `BrainInvoked`, request/response artifacts, cost, latency. Honest definition (v2-04): reconstruction of what was seen/asked/answered/done/decided — not deterministic re-execution. Ana can answer, three months later: *why is the limit per-key?* → 3017 → 3020 → her own recorded rationale.

## 8. The crash variant (recovery contract in action)

Suppose Core dies at seq 3022½ (diff written to worktree, `phase.result` never sent). On restart:

~~~text
3023' · AttemptUncertain          att_b1, reason=core_restart
3024' · RecoverySnapshotRecorded  art_snap1 (worktree state, git status: 4 modified)
3025' · RecoveryPerformed         disposition=resume_from_snapshot (phase=build, reversible, envelope had one-shot network already consumed → no external uncertainty)
3026' · AttemptSpawned            att_b2, causation=att_b1, hypothesis="resume after crash; work preserved"
~~~

Had the envelope permitted an un-probeable external effect, disposition would be a decision packet instead. Either way the timeline explains itself — the property the entire architecture exists to guarantee.
