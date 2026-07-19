# S0.5A Manual Runtime Protocol (founder-controlled)

**EXPERIMENTAL — NEVER MERGE.** This protocol tells the founder how to
collect **interactive runtime** evidence for TDR-002 from the real
assembled application, on a founder-controlled machine. It changes no
security behavior of the prototype. Record results in
`MANUAL_RUNTIME_EVIDENCE.md`.

## Ground rules (do not violate)

- **Never disable** Smart App Control, Windows Application Control,
  Microsoft Defender, UAC, the firewall, or browser/WebView protections
  to make the artifact run. If the unsigned artifact is blocked, that is
  a valid, recordable result — classify it `BLOCKED BY ENDPOINT POLICY`
  and stop; do not bypass.
- **Environment preference:** (1) founder-controlled Windows 11 machine
  if its policy permits running an unsigned prototype; (2) otherwise a
  disposable founder-controlled Windows VM; (3) **never** a production or
  customer device.
- The artifact is **unsigned and not safe to distribute**. It exists to
  produce evidence, not to be shipped.
- Do not screenshot or paste any secret, token, or environment value
  into the evidence file (there are none in the prototype by design;
  keep it that way).

## Get the artifact

1. Open the private repo → Actions → run of workflow
   `S0.5A desktop security prototype` on branch
   `spike/s0-5a-desktop-security-prototype`.
2. Download the artifact `EXPERIMENTAL-NEVER-MERGE-wepld-s05a-windows`
   (retention is short — 1 day). Downloadable only through the private
   repo workflow; there is no Release and no tag.
3. Verify integrity before running: open `SHA256SUMS.txt` and confirm the
   binaries match, e.g. in PowerShell:
   `Get-FileHash .\s05a-tauri-host.exe -Algorithm SHA256`.
4. Read `README-EXPERIMENTAL.txt` and `BUILD_INFO.txt`; confirm
   `source_commit` matches the PR head you intend to test.

## If launch is blocked

Record under `BLOCKED BY ENDPOINT POLICY`:
- the exact on-screen message (SmartScreen / Application Control / SAC);
- whether the block was **before launch** (executable refused) or
  **during sidecar execution** (host started, core child refused);
- an Event Viewer reference if safely obtainable
  (Applications and Services Logs → Microsoft → Windows →
  `CodeIntegrity/Operational` or `AppLocker`), **read-only**;
- do not attempt any workaround. This outcome is itself TDR-002 evidence.

---

## Test A — Launch and process topology

Launch with `run.cmd` (sets scoped fixture/output roots) or by
double-clicking `s05a-tauri-host.exe`.

Verify and record PASS / PASS WITH RESERVATION / FAIL / NOT TESTED /
BLOCKED BY ENDPOINT POLICY:

1. Tauri shell window appears; title contains "EXPERIMENTAL".
2. WebView2 renders the UI (Task Manager shows `msedgewebview2`
   process(es) under the app).
3. A **separate** `s05a-core.exe` process exists as a child of
   `s05a-tauri-host.exe` (Task Manager → Details, or
   `Get-CimInstance Win32_Process | ? {$_.Name -like 's05a-*'} |
   select Name,ProcessId,ParentProcessId`).
4. **No local HTTP listener** attributable to the app:
   `Get-NetTCPConnection -State Listen | ? OwningProcess -in $pids`
   returns nothing for the app's PIDs (record the command output).
5. **No unexpected external network** connection from the app PIDs
   (`Get-NetTCPConnection | ? OwningProcess -in $pids`), read-only.

## Test B — End-to-end IPC (from the real UI)

Activate each button and record the on-screen result line:

1. **Core health** → expect `Ok` with protocol/build/session (no env).
2. **Read scoped fixture (allowed)** → `Ok`, content contains
   "S0.5A fixture".
3. **Read traversal (must deny)** → `Denied` (`traversal-or-prefix-
   rejected`).
4. **Write scoped output (allowed)** → `Ok`; confirm a file appeared
   under `%TEMP%\s05a-output\run\`.
5. **Write to .git (must deny)** → `Denied`
   (`git-metadata-access-rejected`).
6. **Unknown operation (must deny)** → `Denied` (`unknown-operation`).
7. **Capability explanation** — confirm the allowed/denied rows show a
   deterministic reason + capability + resource.
8. **Request/response correlation** — each activation updates only its
   own row and the live status region.

Absolute-path denial and oversized/malformed framing are covered by the
**automated** suite (18 negatives, `cargo test`), not by a live UI
control — the prototype deliberately exposes **no** diagnostic command
that could widen the surface. Note this in the evidence file rather than
adding such a control.

## Test C — Failure and recovery (live app)

1. In Task Manager, **End task** on `s05a-core.exe` only.
2. Activate any operation → the UI must show a sanitized failure
   ("bridge unavailable / core may have stopped"), **not hang**.
3. Close and relaunch the app (or, if a restart control exists, use it).
4. Health after relaunch → `Ok` with a **different** session id than
   before (record both).
5. Confirm the pre-restart session cannot be reused and no prior
   unauthorized request replays successfully (the core rejects a stale
   session; observe that repeating step B3/B5 still denies).
6. Confirm no partial/failed write was silently accepted (the
   `%TEMP%\s05a-output` tree contains only intended files).

## Test D — UI authority (runtime, separate from config)

Record **observed runtime behavior** and, separately, the **config
evidence** already in the repo (CSP in `tauri.conf.json`, single
`core_request` command, `core:default` capability only).

Attempt from the UI / DevTools if reachable, and confirm each is **not**
possible:

1. Browse/read an arbitrary file path.
2. Write an arbitrary file outside the scoped output.
3. Spawn a shell or launch a process.
4. Open a network connection to an arbitrary endpoint.
5. Read environment variables or any secret.
6. Invoke any Tauri command other than `core_request`
   (e.g. in DevTools console, `window.__TAURI__` invoking `plugin:fs|*`
   or `plugin:shell|*` must fail — record the error).

If DevTools is not available in the release WebView, record that and rely
on the config evidence plus the negative behavior of the buttons.

## Test E — Keyboard and accessibility (Windows)

Classify each PASS / PASS WITH RESERVATION / FAIL / NOT TESTED /
BLOCKED BY ENDPOINT POLICY. **Do not infer accessibility from markup —
verify interactively.**

1. Keyboard-only operation (unplug/ignore mouse): reach and activate
   every control with Tab/Shift+Tab.
2. Logical tab order (matches visual/reading order).
3. Visible focus indicator on every control.
4. Enter and Space both activate the focused button.
5. Status and error **announcements** occur (the live region).
6. Screen reader: **Windows Narrator** and/or **NVDA** announce button
   names, the status region, and result changes.
7. Windows **high-contrast** mode: UI remains legible/operable.
8. **Text scaling** at 125%, 150%, 200% (Windows display scaling):
   layout remains usable, no clipping that blocks operation.
9. **Reduced-motion**: no motion that ignores the OS setting (the
   prototype uses none — confirm).
10. **Arabic toggle** ("Toggle RTL") flips `dir`/`lang`.
11. **Right-to-left** layout mirrors correctly.
12. Mixed Arabic/English text renders correctly (bidi).
13. No control requires the mouse.

## Test F — Performance (assembled app, Windows)

Run `scripts/measure-windows.ps1 -ArtifactDir .` for the automatable
figures (sizes, working sets, idle CPU, and — if you place `s05a-bench`
alongside — handshake/no-op/malformed latency). Record **manually** the
GUI timings the script cannot capture:

- unpacked artifact size; shell exe size; core exe size (from script);
- **cold** launch → interactive (stopwatch, first launch after reboot or
  after Defender first-scan); **warm** launch → interactive (stopwatch,
  subsequent launches);
- core handshake latency; end-to-end **UI → core → UI** no-op latency
  (observe the health round-trip; if you want numbers, run the bundled
  `s05a-bench` which measures the core leg directly);
- combined idle working set; shell/WebView working set; core working set
  (from script);
- **idle CPU after 5 minutes** (`-IdleSeconds 300`);
- core restart time; malformed-request rejection latency (from the
  automated suite / bench).

Record hardware, Windows version, WebView2 version, power mode, number of
runs, method, raw values, median, and p95 where meaningful. **Do not**
compare these Windows numbers directly against macOS or Linux.

---

## macOS and Linux status (this task)

CI produces `EXPERIMENTAL-NEVER-MERGE-wepld-s05a-macos` and
`...-linux` artifacts, but without founder-controlled interactive
hardware they are classified:

- Build: **PASS** (see CI) — or FAIL if a packaging job fails;
- Runtime: **NOT TESTED**;
- Accessibility: **NOT TESTED**;
- Performance: **NOT MEASURED**.

Compilation is not runtime; do not ratify platform support from a build.

## Preliminary platform decision model (record after Windows testing)

- **Windows:** `RATIFY TAURI FOR WINDOWS PERSONAL ALPHA` /
  `CONTINUE WINDOWS EVALUATION` / `TRIGGER WINDOWS ELECTRON FALLBACK
  REVIEW`.
- **macOS:** `RUNTIME PROVISIONAL` / `SUPPORTED BY MANUAL EVIDENCE` /
  `NOT SUPPORTED`.
- **Linux:** `RUNTIME PROVISIONAL` / `SUPPORTED BY MANUAL EVIDENCE` /
  `NOT SUPPORTED`.

TDR-002 is **not** updated by this task; the model is filled in only
after the founder's interactive runs and reviewed separately.
