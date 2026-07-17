# v2-18 — Chronicle Worked Examples

Three normative walkthroughs. Ledger entries shown as `seq · type · essentials`. Example 2 is the canonical decision-edit contract: an implementation that cannot produce that trace does not conform.

---

## Example 1 — Watching a live mission, scrubbing back, jumping to live

Mission `mis_R` ("add rate limiting", the v2-08 mission) is executing. Ana opens Cinema.

1. Cinema opens a session `{mission: mis_R, mode: following}` — status `FOLLOWING`, playhead locked to the ledger tail. The Screen shows the current frame: Builder mid-phase, cost odometer ticking, envelope chips visible.
2. A decision beat lands (`dec_1`/`dec_2` batched). The Decision Camera holds on the packet. Ana resolves it in the same surface (Decisions is one tab away — Cinema and the decision queue share the playhead).
3. Curious how the builder got here, Ana drags the scrubber back 12 frames → session auto-detaches to `PAUSED`; the orientation bar shows `⬤ LIVE +9 frames` drift. The mission keeps running; nothing about viewing is coupled to execution.
4. She opens the Context drawer on the plan frame: the pack manifest shows `docs/api-conventions.md — included (seed)`; she steps (`+1`) through the plan invocation, seeing intent, budget, structured output.
5. She clicks **Jump to live** → `FOLLOWING` again, catching the validate phase's gates landing as facts.

Total new machinery exercised beyond the MVP engine: none — following, detach, seek, drawers, and frames are the same session primitives.

---

## Example 2 — Decision editing: PostgreSQL → SQLite (the Part-5 contract)

Mission `mis_Q` ("persist the job queue"), Bounded-Auto, reached seq 4042 (task 3 of 4, "wire retry metrics"). At seq 4018, decision `dec_2` chose **PostgreSQL** (options: postgres | sqlite | defer; rationale then: "expected multi-node"). Ana has changed her mind: deployment is single-node.

**Locate.** Cinema, Decision lens solo. The track shows 3 beats; she clicks the second (seq 4018 beat frame). The Screen holds on the original packet — options, her old rationale. Action rail: **Revise ⑂**.

**Fork + revise.** She picks `sqlite`, rationale "single-node deployment confirmed; ops burden not justified." Core executes `ReviseDecision` (sugar for fork + revision):

~~~text
— in mis_Q's ledger (parent untouched except the fact it was forked):
4101 · MissionForked            child=mis_Q2, at_seq=4017, motive=decision_revision

— mis_Q2's ledger begins:
1 · MissionCreated              forked_from={mis_Q, 4017}, brief inherited, budget fresh
2 · WorkspaceSnapshotRestored?  — no: worktree branched from ref refs/wepld/mis_Q/att_b2/build-end   (recorded in MissionCreated payload)
2 · DecisionRevised             revises=dec_2(mis_Q), old=postgres, new=sqlite, rationale="single-node…", authority=principal_local
~~~

**Invalidation report** (computed from dec_2's impact cone in mis_Q; shown to Ana before re-planning):

~~~text
Will NOT carry into mis_Q2 (14 outcomes downstream of PostgreSQL):
  art_schema.sql (pg DDL) · art_compose (postgres service) · sqlx-pg pool code in T1/T2 diffs ·
  GateEvaluated build/test ×2 (validated pg code) · kno_11 "queue uses pg advisory locks"
Independent of the decision (salvageable — V1; not carried in MVP):
  art_metrics_design (T3 plan artifact) · docs updates in T4
~~~

**Re-plan.** mis_Q2's planning pack carries the `DecisionDelta` section (C7). Planner emits plan v2: 3 tasks (drops the "provision postgres in CI" task). Bounded-Auto still requires plan approval — Ana approves.

~~~text
5 · BrainInvoked   plan, pack=art_cp1' (DecisionDelta present, manifest logged)
6 · PlanProposed   v2: T1 sqlite schema+pool · T2 tests · T3 docs
7 · PlanApproved   by=principal_local
~~~

**Continue.** Execution proceeds normally; mis_Q2 completes at its own seq 3067 with all gates green.

**Explain.** Chronicle renders the comparison `Compare{a:{mis_Q,4042}, b:{mis_Q2,3067}}`:

~~~text
DECISIONS   dec_2 revised: postgres → sqlite (rationales side-by-side)
PLAN        4 tasks → 3 (dropped CI provisioning)
FILES       git diff refs: −schema.sql −docker-compose.yml +queue.db migrations; pool code Δ
REASONING   plan invocation pack Δ: DecisionDelta section present; provider/model identical
EVIDENCE    both branches: build ✓ test ✓ review ✓ — B adds "single-file backup" test
COST        A spent $3.90 to seq 4042; B spent $2.10 total; A's post-fork spend not carried
~~~

**Preserve.** Ana adopts B: `AdoptBranch{mis_Q2}` → mis_Q gets `MissionSuperseded{by=mis_Q2}` and is archived *unmodified*; both remain fully replayable; the Branch Graph shows Q ─⑂→ Q2 with motive and outcome. Decision Lineage on dec_2 shows both resolutions with both rationales — the organization now remembers not just what it chose, but that it once chose otherwise, and why both times.

---

## Example 3 — Forensics: why did mission `mis_F` fail?

`mis_F` ("make queue delivery at-most-once") failed: gate `test` red on the third build attempt, retry budget exhausted, mission `FAILED`. Ana clicks **Why?** on the failure banner.

**MVP (causal walk):** the cause cone renders as a chain; each hop seeks Cinema to the moment with evidence open:

~~~text
MissionFailed
 ← gated     GateEvaluated test ✗ (3 failures, queue_worker.rs) ......... [art_testlog3]
 ← produced  att_b3 refactor moved retry loop out of delivery guard ..... [art_diff3]
 ← caused    att_b2 retry (hypothesis: "restructure for testability") ... [hypothesis unchanged from b1 → flag]
 ← decided   plan v1 T2: "retries are idempotent, safe to extract" ...... [art_plan1]
 ← informed  plan pack T2 omitted docs/queue-semantics.md (budget rank 17/14) [art_cp1 manifest]
~~~

**V1 (RCA report, C9):**

~~~text
PRIMARY (score 0.74) — context_omission at planning:
  docs/queue-semantics.md states deliveries are NOT idempotent; it was considered and omitted
  (budget). Information test: fires (constraint provably existed & was needed).
  First-defect test: fires at plan T2 (spec contradicts AC "at-most-once").
  Remedy candidates: pin docs/** for planning on src/queue/**; lesson candidate drafted.
ALTERNATE (0.41) — retry_futility: b2/b3 retried without changed hypothesis; would not
  have fixed the plan defect (why lower: divergence test finds no sibling where retry style
  differed and outcome improved).
Narration: verified (all hops evidence-linked).
~~~

The insight scanner later aggregates this with two prior missions into the `recurring_context_omission` insight (v2-16 §1); Ana promotes it; the lesson enters Knowledge; Context Assembly pins the file for future queue work. **The failure is now infrastructure.** Next quarter, a new mission on `src/queue/**` plans with the constraint in-pack — and its Chronicle will show the lesson being *used*, closing the loop the platform exists to close.
