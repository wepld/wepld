# S0.5A Manual Runtime Evidence (to be filled by the founder)

**EXPERIMENTAL — NEVER MERGE.** This file is an empty evidence template.
Nothing here is verified yet. Every status starts `NOT TESTED` and must
be changed only from a real interactive run on a founder-controlled
machine. Do not paste secrets or environment values.

## Run header

- artifact tested: `EXPERIMENTAL-NEVER-MERGE-wepld-s05a-windows`
- source_commit (from BUILD_INFO.txt): `__________`
- SHA-256 of s05a-tauri-host.exe (verified): `__________`
- SHA-256 of s05a-core.exe (verified): `__________`
- hardware: `__________`
- Windows version: `__________`
- WebView2 version: `__________`
- power mode: `__________`
- environment: [ ] founder machine [ ] disposable VM (never a customer device)
- date/tester: `__________`

## Endpoint-policy outcome

- launch result: [ ] launched  [ ] BLOCKED BY ENDPOINT POLICY
- if blocked — exact message: `__________`
- block phase: [ ] before launch  [ ] during sidecar (core) execution
- Event Viewer reference (read-only, optional): `__________`
- (no bypass attempted): confirm `__________`

## Test A — Launch and process topology

| Check | Result | Notes (PID/observation) |
| --- | --- | --- |
| A1 shell window appears | NOT TESTED | |
| A2 WebView2 renders | NOT TESTED | |
| A3 separate core child process | NOT TESTED | host PID __ / core PID __ / parent __ |
| A4 no local HTTP listener (app PIDs) | NOT TESTED | command output: |
| A5 no unexpected external network | NOT TESTED | |

## Test B — End-to-end IPC (from the UI)

| Check | Expected | Result line observed |
| --- | --- | --- |
| B1 health | Ok (no env) | NOT TESTED |
| B2 read allowed | Ok, "S0.5A fixture" | NOT TESTED |
| B3 read traversal | Denied traversal-or-prefix-rejected | NOT TESTED |
| B4 write allowed | Ok + file in %TEMP%\s05a-output\run | NOT TESTED |
| B5 write .git | Denied git-metadata-access-rejected | NOT TESTED |
| B6 unknown op | Denied unknown-operation | NOT TESTED |
| B7 capability explanation | deterministic reason/cap/resource | NOT TESTED |
| B8 request/response correlation | only own row updates | NOT TESTED |

## Test C — Failure and recovery

| Check | Result | Notes |
| --- | --- | --- |
| C1 kill core only | NOT TESTED | |
| C2 UI shows sanitized failure, no hang | NOT TESTED | |
| C3 relaunch | NOT TESTED | |
| C4 fresh session id (old __ / new __) | NOT TESTED | |
| C5 stale session cannot be reused | NOT TESTED | |
| C6 no partial write silently accepted | NOT TESTED | |

## Test D — UI authority (runtime)

| Attempt (must be impossible) | Result | Observed error |
| --- | --- | --- |
| D1 browse arbitrary file | NOT TESTED | |
| D2 write arbitrary file | NOT TESTED | |
| D3 spawn shell | NOT TESTED | |
| D4 launch process | NOT TESTED | |
| D5 connect arbitrary endpoint | NOT TESTED | |
| D6 read env/secret | NOT TESTED | |
| D7 invoke non-`core_request` Tauri command | NOT TESTED | |

Config evidence (already in repo, restate): CSP `default-src 'none'`;
single command `core_request`; capability `core:default` only.

## Test E — Keyboard and accessibility (Windows)

| Item | Result | Notes (Narrator/NVDA, scaling %, etc.) |
| --- | --- | --- |
| E1 keyboard-only operation | NOT TESTED | |
| E2 logical tab order | NOT TESTED | |
| E3 visible focus | NOT TESTED | |
| E4 Enter + Space activate | NOT TESTED | |
| E5 status/error announcements | NOT TESTED | |
| E6 Narrator / NVDA | NOT TESTED | |
| E7 high-contrast mode | NOT TESTED | |
| E8 text scaling 125/150/200% | NOT TESTED | |
| E9 reduced-motion | NOT TESTED | |
| E10 Arabic toggle | NOT TESTED | |
| E11 RTL layout | NOT TESTED | |
| E12 mixed Arabic/English (bidi) | NOT TESTED | |
| E13 no mouse-only interaction | NOT TESTED | |

## Test F — Performance (assembled app, Windows)

| Metric | Value | Method/notes |
| --- | --- | --- |
| unpacked artifact size | NOT MEASURED | measure-windows.ps1 |
| shell exe size | NOT MEASURED | |
| core exe size | NOT MEASURED | |
| cold launch → interactive | NOT MEASURED | manual stopwatch |
| warm launch → interactive | NOT MEASURED | manual stopwatch |
| core handshake latency | NOT MEASURED | s05a-bench (core leg) |
| e2e UI→core→UI no-op | NOT MEASURED | observed |
| combined idle working set | NOT MEASURED | |
| shell/WebView working set | NOT MEASURED | |
| core working set | NOT MEASURED | |
| idle CPU after 5 min | NOT MEASURED | -IdleSeconds 300 |
| core restart time | NOT MEASURED | |
| malformed-request reject latency | NOT MEASURED | automated suite/bench |

Raw runs / median / p95: `__________`
Do not compare these Windows numbers with macOS or Linux.

## macOS / Linux (this task)

- macOS: Build = (see CI), Runtime = NOT TESTED, Accessibility = NOT
  TESTED, Performance = NOT MEASURED.
- Linux: Build = (see CI), Runtime = NOT TESTED, Accessibility = NOT
  TESTED, Performance = NOT MEASURED.

## Preliminary platform decision (fill after Windows runs)

- Windows: `__________` (RATIFY FOR WINDOWS PERSONAL ALPHA / CONTINUE
  WINDOWS EVALUATION / TRIGGER WINDOWS ELECTRON FALLBACK REVIEW)
- macOS: `RUNTIME PROVISIONAL` (until manual evidence)
- Linux: `RUNTIME PROVISIONAL` (until manual evidence)

TDR-002 is not changed here; this evidence feeds a later, separate
review.
