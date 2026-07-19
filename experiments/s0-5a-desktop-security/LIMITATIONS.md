# S0.5A Limitations

**EXPERIMENTAL — NEVER MERGE.** This spike produces evidence, not
guarantees. It does not prove WePLD is secure. Read every limitation
below as a bound on what the evidence can support.

## Artifact provenance (S05A-PROVENANCE-001)

The first corrected artifact's `BUILD_INFO.txt` labeled its source as the
synthetic GitHub pull-request merge SHA (`a7f61d7…`) rather than the
reviewed head (`76c5558…`); the built content was proven identical to the
head (a labeling defect, not a different binary). The workflow now checks
out the exact head, fails closed on mismatch, and every artifact carries a
distinct five-field provenance model plus `PROVENANCE.json`. **A founder
provenance-verification rerun is required** before trusting a fresh
artifact; runtime/accessibility/performance testing stays paused until
provenance passes. CI cannot prove the founder's local hash/BUILD_INFO
verification — that step is the founder's.

## Runtime response contract (S05A-RUNTIME-001)

The founder's Windows run found every UI operation rendering `undefined`
(a frontend response-contract defect; the core was correct). A
frontend-only fix now parses the host response exactly once, validates
it, and renders a canonical typed result, with unit + real-core
regression tests. **This restores the contract but does not by itself
prove the assembled Windows UI renders correctly end-to-end** — a
founder smoke rerun on the corrected artifact is required. Crash/restart,
accessibility, and performance testing remain **paused** until that smoke
gate passes. No core/host security semantic was changed by the fix.

## Threat-scope limits

- **Full OS compromise is out of scope.** If the user's operating system
  or account is fully compromised, local confidentiality of anything the
  device can decrypt is lost, and a hostile process can impersonate or
  observe the core, the shell, and local providers. Nothing in this
  prototype changes that; it is a stated residual risk in the merged
  Security Constitution.
- **Same-user local malware** at the same privilege can read what the
  user reads and spawn what the user spawns. The capability boundary
  constrains the *prototype's own* components; it is not an anti-malware
  boundary.
- The session identifier is **non-cryptographic** (uniqueness across
  restarts only). It is sufficient for the replay/restart tests here; it
  is NOT an unguessable token and is not represented as one.

## Evidence-level limits

- **Compilation is not runtime.** A green build on an OS does not prove
  runtime behavior on that OS.
- **One OS is not another.** Local interactive evidence was collected on
  Windows only. Linux and macOS evidence is CI build/test only.
- **The Tauri host is build-only.** It could not be built on the founder
  host (Application Control policy blocks its build scripts / proc
  macros), so there is **no local runtime evidence** for the Tauri shell.
  CI attempts a headless cross-platform compile; a headless runner does
  not exercise the WebView GUI, a real window, or a screen reader.
- **Unsigned.** No code signing or notarization was performed and no
  signing credentials exist. Unsigned prototype artifacts must not be
  distributed outside founder-controlled testing.
- **No installer** was produced (`bundle.active = false`); installer size
  is therefore NOT MEASURED.

## Accessibility limits

- Accessibility is implemented in markup (semantic controls, labels, an
  `aria-live` status region, keyboard-operable buttons, an RTL/Arabic
  toggle) but **not interactively verified with a screen reader on any
  OS**. Screen-reader announcement behavior, focus order in the real
  WebView, high-contrast, reduced-motion, and text-scaling are recorded
  as **NOT TESTED / pending founder interactive hardware**. Per the task,
  TDR-002 must not be ratified while required accessibility evidence
  remains untested.

## Prototype-vs-production differences

- The capability engine is a **static two-entry table**, not a policy
  engine. It tests the boundary shape, not production policy precedence.
- The IPC transport (framed stdio) is one credible option; alternatives
  (length-prefixed local sockets, named pipes) were not implemented.
- The prototype hand-rolls JSON to reach zero dependencies; production
  would use an audited serde stack.
- No database, no provider, no secret, no updater, no worker pool, no
  telemetry — all deliberately absent.

## Not measured here

- Installer size; macOS/Linux cold-start, warm-start, and idle memory of
  the assembled shell+WebView (the local memory figure is the **core
  process only**, not the WebView); idle CPU over long durations; restart
  time of the assembled app (the core-process restart is measured
  indirectly via the test suite, not timed end-to-end in the shell).
- External independent security review (none performed).
