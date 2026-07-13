# v2-02 — System Design: Mechanisms

This document answers "how does it actually work," not "what should it do." Contracts referenced here are specified concretely in [v2-07](07_Contracts.md); a full worked run is in [v2-08](08_Worked_Example.md).

## 1. Process model

| Process | Lifetime | Responsibilities | Trust |
| --- | --- | --- | --- |
| **Core** | long-lived, one per user | state machine, ledger, gates, decision queue, context assembly, brain gateway (holds provider credentials), messenger, artifact store, Studio HTTP server (loopback + session token) | trusted |
| **Worker (Hermes)** | one per phase, spawned by Core as a child in a sandbox | executes the phase loop: request context → reason via Core-routed brain calls → act inside envelope → produce artifacts/evidence → report phase result | untrusted execution |
| **Studio UI** | browser/webview page | render projections; submit commands | untrusted input |

Workers are child processes in a process group: if Core dies, workers die with it (no orphan effects); if a worker dies, Core observes `SIGCHLD`/exit and runs recovery (§7). Worker↔Core transport is WWP (JSON-RPC 2.0 over stdio). UI↔Core transport is HTTP + server-sent events on loopback, authenticated by a token minted at Core start.

**Module map inside Core** (each behind a port; these are tomorrow's process boundaries): `mission-state`, `ledger`, `gates`, `decisions`, `context-assembly`, `brain-gateway`, `sandbox`, `artifacts`, `knowledge`, `messenger`, `studio-api`. No module reads another's tables; the composition root wires ports. This is principle 15 with teeth: the fleet split (V2) moves `sandbox`+worker spawn behind a socket without touching any other module.

## 2. Command handling

Every mutation enters as a **Command** (v2-07): `{command_id, type, actor, payload, expected_revision?}`. Processing is one SQLite transaction:

1. Idempotency check: `command_id` already in `commands` table → return prior outcome.
2. Authorization: actor is the local principal (MVP) — checked, not assumed, so multi-seat is additive.
3. Validation against the state machine (§4). Invalid transition → `REJECTED{reason}`; no ledger noise.
4. Apply: mutate state tables, append ledger entries, record command outcome — same transaction.
5. Post-commit: notify subscribers (UI event stream), schedule side effects (spawn worker, run gate) via the `work_queue` table (§3).

Outcomes are exactly v1's four: `ACCEPTED`, `REJECTED`, `AWAITING_APPROVAL`, `DEFERRED`.

## 3. Side effects and the work queue

Side effects (spawn a phase, run a gate check, call a provider, send a notification) never run inside the command transaction. The transaction enqueues a row in `work_queue` (`{id, kind, payload, not_before, attempts, status}`); a Core scheduler loop claims rows (single process — a simple `UPDATE … WHERE status='ready'` claim), executes, and records completion in a new transaction. Crash between commit and execution ⇒ the row is still `ready` at restart ⇒ at-least-once with idempotency keys downstream. This is the outbox pattern reduced to its local minimum.

## 4. State machine

Single transition function `apply(state, command|worker_report) → (state', ledger_entries[])` — the **only** writer of both state tables and ledger (ADR-0003 consistency argument).

**Mission:** `DRAFT → PLANNING → PLAN_REVIEW → RUNNING ⇄ WAITING_DECISION → VERIFYING → COMPLETION_PROPOSED → ACCEPTED | RETURNED(→RUNNING) | CANCELLED | FAILED`

**Task (sequential in MVP):** `PENDING → READY → RUNNING → SUCCEEDED | FAILED | BLOCKED_DECISION(→READY)`

**Attempt:** `SPAWNED → RUNNING → SUCCEEDED | FAILED | CANCELLED | UNCERTAIN`

Transition guards worth stating precisely:

- `RUNNING → VERIFYING` requires every task `SUCCEEDED`.
- `VERIFYING → COMPLETION_PROPOSED` requires: every required gate has a `GateEvaluated{pass}` ledger fact **produced by Core** (§6); review-phase artifact exists with disposition `approve` or all findings dispositioned; budget not exceeded; diff confined to scope paths (Core re-checks with `git diff --name-only` against `scope.paths` — not trusted from the worker).
- `COMPLETION_PROPOSED → ACCEPTED` requires a human `AcceptMission` command. Always. No autonomy mode bypasses this in the MVP.

## 5. Phase engine

A mission's approved plan yields tasks; each task runs phases from its declared phase set (default: Build → Validate → Review; Understand/Plan run once at mission level). For each phase, Core:

1. Requests a **context pack** from Context Assembly (v2-04) for `(task, phase, role)`.
2. Computes the **envelope** for the role (v2-05): builder gets worktree write; validator gets worktree read + execute; reviewer gets read-only + diff artifacts.
3. Spawns Hermes with `attempt.start{attempt_id, role_profile, phase, context_pack_ref, envelope, gates, budgets, idempotency_key}`.
4. Streams `heartbeat`, `brain.request` (routed to the gateway; invocation recorded), `artifact.put`, `escalation.raise`, `envelope.extend` messages.
5. Receives `phase.result{status, outputs[], evidence[], uncertainties[], next_hint}` and advances the state machine.

Phase retries follow v1's rule verbatim: a retry requires a named hypothesis, consumes budget, and links to the prior attempt.

## 6. Gates — evidence the model cannot fake

A gate is a **Core-executed check**, not a worker claim. For `build` and `test` gates, Core itself runs the project's declared commands (from the mission brief or repo config) inside a *validator envelope* — a sandboxed execution identical in kind to a worker phase but driven by Core — capturing exit codes, stdout/stderr artifacts, and durations. The resulting `GateEvaluated{gate, pass, check_artifacts[]}` ledger entry is therefore rooted in Core-observed process results. The review gate is satisfied by the review phase's structured findings artifact (schema-validated), which the human can always open. Model prose is never parsed for gate status. This is principle 6 as a mechanism.

## 7. Recovery — concrete algorithm

On Core startup (or worker exit without `phase.result`):

1. Select attempts in `SPAWNED|RUNNING`. Mark `UNCERTAIN`; ledger `AttemptUncertain{reason}`.
2. **Probe observable state** instead of guessing: `git -C <worktree> status --porcelain` and `git log` vs. the attempt's `base_commit`; enumerate declared output artifacts present in the store.
3. Classify:
   - *No worktree changes, no artifacts* → safe automatic retry (fresh attempt, causation-linked).
   - *Changes present* → snapshot the worktree state as an artifact (`RecoverySnapshotRecorded`), then: Bounded-Auto with a reversible phase → retry from snapshot; otherwise → decision packet "resume / discard / inspect."
   - *Non-idempotent external effect possible* (only if the envelope allowed network, dependency install, etc.) → always a decision packet; never silent retry.
4. Ledger `RecoveryPerformed{disposition}`. The mission never shows a state the ledger cannot explain.

Because effects are confined to the worktree by the envelope in the common case, recovery classification is usually mechanical — this is the quiet payoff of ADR-0004.

## 8. Budgets and cost attribution

Every brain invocation records `{profile, provider, model, tokens_in/out, cost_estimate, latency, attempt_id}` (table `brain_invocations`). Mission budget = Σ estimates, checked *before* dispatching each request with a projected cost (prompt size × price sheet); crossing the budget is a hard gate, not a post-hoc report. Retries and gate runs attribute to the attempt that caused them. The price sheet is versioned data, refreshed manually in MVP.

## 9. Failure taxonomy

Every failure is classified into exactly one of: `TRANSIENT` (provider 5xx/timeout → bounded automatic retry), `SCHEMA` (invalid structured output → one reformat retry, then escalate), `ENVELOPE` (attempted crossing → denied, logged, escalate on repeat), `BUDGET`, `EVIDENCE_CONFLICT` (checks disagree → never auto-resolved, always escalate), `ENVIRONMENT` (sandbox/toolchain integrity → pause mission), `UNCERTAIN_EFFECT` (§7). The taxonomy is closed: adding a class is a contract change. Retry/escalate semantics live in one table, not scattered in call sites.

## 10. Degraded modes

Provider down → mission enters visible `WAITING(provider)` with the failed invocation on the timeline; never fabricated progress. Local model unavailable → profile-level substitution only if the mission's classification allows the fallback (no silent upgrade of data egress — v1 rule preserved). Studio closed → Core continues; on reconnect the UI resumes from the ledger cursor. Disk pressure → new missions refused before running missions are corrupted (watermark check in the scheduler loop).

## 11. Security posture summary

Trust boundaries: UI input untrusted; worker untrusted (sandboxed, no credentials, no user channel); repo content untrusted (labeled in context packs — v2-04 §Redaction; quoted, never obeyed); providers untrusted (structured outputs validated; proposals ≠ effects). Secrets: provider keys in the OS keychain, read only by the brain gateway; workers never receive them (brain calls route through Core). The signed-lease/key-management question from the gate review dissolves in the MVP: Core spawns workers directly, so the envelope's authenticity is the parent-child relationship plus the per-attempt token embedded in the spawn arguments; asymmetric signing becomes necessary only when workers go remote (V2 seam, noted in v2-07 §Worker Contract).
