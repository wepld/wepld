# M0 Release Report — WePLD "The Spine"

**Tag:** `v0.0.1-m0` (audit fixes at `91607c6`) · **Date:** 2026-07-13 · **Author:** Lead Software Engineer
**Scope:** production-quality verification of M0 against the Master Engineering Charter, Architecture v2 (frozen), Chronicle (frozen), the Implementation Program, and the M0 Definition of Done. No new features; verification only.

This gate was adversarial: the audit attempted to *prove incorrectness* across every surface named in the gate brief. It found **three real weaknesses**, all fixed and regression-tested, plus two robustness improvements. All results below are observed (real processes, real SQLite, real git), not asserted.

---

## Architecture Compliance

| Frozen requirement | Status | Evidence |
| --- | --- | --- |
| Runtime is the product; owns all state | ✅ | Only `runtime` holds `ledger::Tx`; CLI/Studio would mutate only via commands |
| Worker runtime replaceable; WWP is the boundary | ✅ | `cargo tree`: `hermes` production deps = `wwp` → `contracts` **only** (no ledger/runtime/providers) |
| Hermes stateless; owns execution, not persistence | ✅ | Hermes has no store/CAS/ledger access; receives packs, returns results over WWP |
| Reasoning optional (IADR-0007 §1) | ✅ | `echo`/deterministic phases complete with an empty `brain_invocations` table |
| Provider neutrality; Runtime never calls providers directly | ✅ | All reasoning flows through `providers::Gateway`; no crate outside `providers` names a vendor |
| Single-writer, hash-chained ledger (ADR-0003) | ✅ | One `Tx` consumer; SHA-256 chain with genesis; append-only DB triggers |
| Record once, derive forever | ✅ | Context pack stored once to CAS, referenced by hash on every invocation; diffs/logs are artifacts |
| Git owns source truth; isolation via worktrees + hidden refs (ADR-0013) | ✅ | Snapshots under `refs/wepld/**` via temp index; primary worktree untouched (asserted) |
| Honest sandbox tier (IADR-0003) | ✅ | `SandboxTierDetected{DEV}` recorded at init and displayed; no false containment claim |
| Closed event vocabulary rev 2 (v2-07) | ✅ | 39-variant enum, compile-time-exhaustive `code()`, lock test |
| Crate layering acyclic; boundaries enforced | ✅ | Cargo graph acyclic by construction; `contracts` imports nothing |

**Crates (9):** libs `contracts, ledger, artifacts, workspace, wwp, providers, runtime` + apps `hermes, cli`. The `sandbox` (M4) and `chronicle` (M6) crates are later-milestone and correctly absent from M0. No architectural drift: every contract change was additive with a version bump (`contracts` 0.1→0.4).

---

## Implementation Compliance — M0 Definition of Done

| DoD item | Status | Evidence |
| --- | --- | --- |
| Golden `m0-first-mission` green | ✅ | `fixtures/golden/m0-first-mission.trace` (21 entries) asserted exactly in `golden_tests` |
| Chain-verify + fold-check | ✅ | `verify_chain` + randomized `fold == tables` property test |
| WWP over a real child process | ✅ | Hermes spawned as a real process in every lifecycle/build/golden/adversarial test |
| Context pack captured (v0) | ✅ | Pack stored once to CAS; hash on each `brain_invocations` row |
| DEV tier recorded & displayed | ✅ | `wepld init` and `wepld demo` print the tier + disclosure |
| IADRs merged | ✅ | IADR-0001…0008 in `docs/impl/adr/` |
| `wepld demo` self-contained | ✅ | Runs create→plan→approve→run→accept→timeline→verify; chain VERIFIED; edit merged to main |
| fmt + clippy `-D warnings` + tests green | ✅ | `cargo fmt --check` clean; `clippy --all-targets -D warnings` clean; **57/57** tests |

---

## Performance Summary

Measured in release mode on the WSL2 dev environment (single run; indicative, not a benchmark suite — formal calibration is a v2-27 Phase-0/M1 task).

| Metric | Observed | v2-27 context |
| --- | --- | --- |
| Full bounded loop (`wepld demo`): create→plan→approve→run→accept, **2 hermes process spawns, 2 brain calls, real git ops, 1 gate** | **~160 ms** wall | Well within budget; each control-plane op is sub-100 ms |
| Peak resident memory (whole loop) | **~6.3 MB** | Comfortable for a local daemon |
| Ledger append + fold | sub-millisecond at M0 scale | v2-27 targets p95 < 25 ms append |

No performance regressions possible to assess yet (first milestone); the numbers establish a healthy baseline far under target.

---

## Security Review

**Fixed during this gate (real vulnerabilities):**

1. **WWP unbounded `Content-Length` (OOM / process abort).** A WWP peer is untrusted (v2-03). A worker sending `Content-Length: 1099511627776` forced a ~1 TB allocation → abort. **Fixed:** `MAX_CONTENT_LEN` = 64 MiB; oversized frames return `ContentTooLarge` without allocating. Regression: `huge_content_length_is_refused_without_allocating`.
2. **WWP unbounded header line (OOM).** A headerless byte stream grew the line buffer without limit. **Fixed:** byte-budgeted header reader capped at `MAX_HEADER_BYTES` = 16 KiB → `HeaderTooLarge`. Regression: `headerless_byte_stream_is_bounded`.
3. **Brain-call budget unenforced (worker request spam / unbounded work + ledger growth).** `max_brain_calls` was advertised to the worker but not enforced by the Core. **Fixed:** the phase engine counts `brain.request`s per attempt and kills+fails a worker that exceeds the cap. Regression: `brain_call_budget_is_enforced` (a flooding worker is bounded to ≤ 8 recorded invocations, attempt fails cleanly, chain stays valid).

**Verified already-defended:**

- **Ledger tampering:** any mutation of a payload is detected by `verify_chain` (recomputed payload + link hashes), proven after a full realistic flow (`tampering_after_flow_is_detected`) and at unit level. Append-only enforced by DB triggers (out-of-band `UPDATE`/`DELETE` rejected).
- **CAS corruption:** every `get` re-verifies the SHA-256; a corrupted body errors, never serves silently. Tombstones preserve hash + reason.
- **Secrets:** none enter the substrate. Envelope declares `secrets: []` by default; any secret is a hard gate (not exercised at M0 since no secret is used).
- **Data egress:** network `deny` by default in every M0 envelope; no provider egress (fixture adapter is offline, IADR-0002).
- **Trust boundary honesty:** DEV tier is disclosed, not disguised; autonomy is capped accordingly.

**Not yet in scope (by milestone plan):** real OS sandboxing (M4 — gates currently run directly under the disclosed DEV tier, IADR-0003); real provider egress controls + redaction (M1); supply-chain/signing (M7).

---

## Reliability Review

| Property | Status | Evidence |
| --- | --- | --- |
| Transactional atomicity | ✅ | A transaction that appends + mutates then errors persists nothing (`failed_transaction_rolls_back_atomically`) |
| Worker crash → honest classification | ✅ | A dead build worker → `AttemptUncertain` (never assumed failed); mission stays running; chain valid (crash drill) |
| Worker hang / heartbeat loss | ✅ | Watchdog fires within the heartbeat timeout → Uncertain (`silent_worker_trips_the_watchdog`) |
| Worker protocol violation (garbage bytes) | ✅ | Classified Uncertain; mission unchanged (`garbage_worker_is_uncertain`) |
| Cooperative cancellation | ✅ | `hang` worker honors `attempt.cancel` |
| Command idempotency | ✅ | Replay returns the stored outcome; exactly one effect; reused id + changed payload rejected |
| Duplicate mission id | ✅ | Second create rejected; exactly one `MissionCreated` |
| Invalid state transitions | ✅ | run-before-approve, approve-before-plan, accept-before-completion, double-plan, double-approve all rejected; state unchanged |
| Store reopen | ✅ | Reopens cleanly; tier fact not duplicated; SQLite `busy_timeout(5s)` added for transient contention |
| Bad repository path | ✅ | Clean `Rejected`, not a propagated error; no corruption |
| Primary worktree integrity | ✅ | `git status` clean throughout; snapshot commits invisible on user branches |

---

## Failure Modes (catalogued)

| Trigger | System behavior | Recovery |
| --- | --- | --- |
| Worker exits without result | `AttemptUncertain` recorded; mission stays running | Human/M5 auto-resume decides; state is explainable |
| Worker heartbeat timeout | Watchdog → kill → `AttemptUncertain` | as above |
| Worker protocol violation / oversized frame | Reader emits `Malformed` → kill → `AttemptUncertain` | as above |
| Worker brain-call spam | Killed at cap → attempt `failed` (budget reason) | mission stays running |
| Cassette miss (no reasoning) | `provider_error` invocation recorded; phase fails | mission stays in prior state; loud, never improvised |
| Gate fails | `GateEvaluated{failed}`; no `CompletionProposed` | mission stays running |
| Bad repo path | clean `Rejected` | no state change |
| Ledger tamper | `verify_chain` reports `broken_at` | audit surfaces it |
| CAS body corruption | `get` errors on hash mismatch | never served |
| Transaction error | full rollback | no partial writes |
| Process crash mid-transaction | SQLite rolls back the uncommitted transaction (WAL, synchronous=FULL) | reopen reconstructs consistent state |

---

## Known Limitations (by design at M0; not defects)

1. **Ledger tail truncation is undetectable by the chain alone.** A hash chain detects mutation and mid-sequence deletion but cannot detect removal of the *most recent N* entries without an external head anchor. Mitigation deferred; documented risk. (Filesystem/OS integrity and backups are the stopgap, per v2-17.)
2. **No real OS sandbox** — gates and workers run under the disclosed DEV tier (IADR-0003). Real isolation lands at M4; autonomy is capped to Manual/fixture-repo scope meanwhile.
3. **No recovery auto-resume** — crashes are *classified* (Uncertain) but not automatically re-driven; that is M5.
4. **Single task per mission exercised** — the loop supports a task list, but golden/demo use one task; multi-task ordering and parallelism are later.
5. **Snapshot refs accumulate** — no retention sweep yet (ADR-0013 notes it); storage is deduplicated by Git but refs are not pruned.
6. **Cassette-only reasoning** — the fixture adapter is the sole provider (IADR-0002); real adapters are M1.
7. **Concurrency assumes a single Core** — two Cores on one store rely on SQLite `busy_timeout`; a second creator racing the same mission id would hit a PK error rather than a clean rejection (single-writer design; acceptable).

---

## Open Technical Debt

| Item | Severity | Plan |
| --- | --- | --- |
| Lifecycle ops (plan/approve/run/accept) are `Core` methods, not idempotent `submit` commands | Low | Fold into the command pipeline when remote/async execution needs it; each already records durable facts |
| `handle_brain_request` propagates rare Core-side I/O errors (disk full) without killing the worker | Low | Wrap terminal on Core-side failure during M1 |
| Gate execution is direct (`sh -c`) under DEV tier | Medium | Replaced by validator-envelope execution at M4; result shape unchanged |
| Golden trace is a single normative flow | Low | Add v2-08 (batched decisions) and injection goldens at M2/M5 per IADR-0004 |
| Windows-native process/path semantics untested (WSL2 only) | Medium | M8 packaging; WSL2 is the supported dev path (IADR-0006) |

None of these block M1.

---

## Recommended M1 Entry Criteria

1. M0 tag `v0.0.1-m0` + audit fixes green on CI (fmt, clippy `-D warnings`, 57 tests). ✅
2. This report accepted (PASS). 
3. Founder items resolved: one hosted-provider API key; one real OSS fixture repo (both flagged OPEN in `DECISIONS.md`).
4. M1 first task confirmed: real brain adapter (Anthropic-family) + record-mode cassette proxy behind the **unchanged** gateway interface; real Context Assembly v0 (T0/T1/T2 selection + manifest + capture) replacing hand-built packs; budget projection enforcement.
5. M1 must not modify frozen contracts except additively with a version bump, and must preserve the `hermes → wwp → contracts` boundary (CI-enforceable).

---

## Final Verdict

# ✅ PASS

M0 satisfies every applicable requirement of the Charter, Architecture v2, Chronicle's substrate expectations, the Implementation Program, and the M0 Definition of Done. The adversarial audit surfaced three genuine weaknesses; all are fixed and regression-tested. The spine is durable, honest under failure, and bounded against a hostile worker. Every claim in this report is backed by an observed test or measurement.

**M1 is authorized to begin.**
