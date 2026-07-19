# S0.5A Experiment Protocol

**Status:** experimental evidence protocol for TDR-002. Never merge.

## Research questions

- RQ1: Can a Tauri 2 shell host an untrusted React/strict-TS UI whose
  every effect flows through a minimal host bridge into a **separate**
  Rust core process over typed, versioned, session-bound, non-HTTP IPC?
- RQ2: Do capability-scoped operations fail closed under adversarial
  input (traversal, symlinks, absolute paths, malformed/oversized/
  stale/unknown requests)?
- RQ3: Is the topology's overhead compatible with the preliminary
  budgets (cold start ~2 s, warm ~1 s, combined idle memory ~250 MB,
  IPC overhead far below a 100 ms interaction budget)?
- RQ4: What is the honest per-OS evidence level for build, runtime,
  and accessibility?

## Hypotheses

- H1: all listed negative requests are rejected with typed, sanitized
  denials and no partial effects.
- H2: core restart invalidates prior sessions; stale replays are
  denied.
- H3: IPC no-op round-trip p95 is under 5 ms locally.
- H4: the frontend cannot read/write files, spawn processes, reach the
  network, read secrets/environment, or invoke undeclared commands.

## Test matrix

18 automated negative security tests (unknown op; malformed message;
oversized frame; unsupported version; wrong session; missing
capability; wrong capability action; `..` traversal; absolute path;
symlink/junction escape; write outside scope; `.git` access; unexposed
command; core crash visibility; restart fresh-session; stale replay;
malformed capability object; unexpected extra fields) plus positive
controls (health, scoped read, scoped write, echo, explain allow/deny)
on every OS the toolchain runs; app build evidence per OS via the
dedicated CI workflow; local interactive runtime evidence on the
founder Windows machine via an automated in-app self-test.

## Measurement method

- Bench client (`s05a-bench`) spawns the core exactly as the host
  does, records handshake latency, then N echo round-trips; reports
  p50/p95/p99 and malformed-rejection latency; JSON output.
- Cold start: wall time from app-process spawn to the in-app self-test
  completion marker file (release build, `S05A_AUTOEXIT=1`).
- Memory/CPU: OS process counters for shell and core at idle.
- Sizes: release binary sizes on disk; no installer is produced
  (bundling/signing out of scope).

## Evidence classification

Per OS and per area: `AUTOMATED` / `MANUAL VERIFIED` / `BUILD ONLY` /
`NOT TESTED`. CI build success is never runtime or accessibility
evidence. Single-OS numbers are never presented as cross-platform.

## Acceptance criteria (evidence hypotheses, not product promises)

Security boundary and functional boundary items are mandatory
(PASS/FAIL); performance items are classified PASS / PASS WITH
RESERVATION / FAIL / NOT MEASURED against the preliminary budgets;
accessibility evidence may remain honestly pending where interactive
hardware is unavailable, and TDR-002 must not be ratified while it is.

## Stop conditions

Stop and record (rather than work around): required unsafe code;
secret material appearing anywhere; a boundary violation that cannot
fail closed; framework requirements that would exceed the authorized
paths.
