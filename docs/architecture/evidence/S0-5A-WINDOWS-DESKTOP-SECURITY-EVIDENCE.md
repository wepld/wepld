# S0.5A — Windows Desktop-Security Evidence Record

**Status:** documentation-only closeout of the S0.5A Desktop Security
Prototype (founder decision, 2026-07-20). This record captures evidence
honestly and supports the platform-scoped amendment to
[TDR-002](../decisions/TDR-002-desktop-shell-selection.md). It authorizes
no implementation.

## Provenance

- Evidence pull request: **#9** (`spike/s0-5a-desktop-security-prototype`)
  — **EXPERIMENTAL — NEVER MERGE**; not merged and not copied into
  product paths.
- Final experimental head:
  `ffbb1a26881bbd8b9479e88e7d621f7cbc2190c4`.
- Canonical base: `e124e293a46b960589cf3d2b37adefe8d6353eaf`.
- Final provenance-bound workflow run: `29702954386`.
- Prototype code remains under `experiments/s0-5a-desktop-security/` on
  PR #9 only; nothing here reuses it.

## How to read this record

Evidence is separated into: **automated CI**, **founder manual (attested)**,
**inferred conclusions**, **unmeasured areas**, and **residual risks**.
CI builds are never treated as runtime or accessibility evidence, and
single-OS results are never presented as cross-platform facts.

## A. Automated CI evidence

Run `29702954386` on head `ffbb1a2…` — all jobs green:

- **core** (ubuntu-24.04 / windows-2022 / macos-14): clippy `-D warnings`,
  `cargo test --locked` (security suite), release build, and bench — green
  on all three OSes.
- **frontend**: strict typecheck + static bundle, plus the real-core
  bridge tests **18 pass, 0 skip** (spawns the actual core and normalizes
  real success/denial/error envelopes).
- **tauri-host build** (ubuntu / windows / macos): cross-platform compile
  green.
- **package** (windows / linux / macos): assemble → checksums → provenance
  verify → upload, each green.
- **Exact-head checkout + fail-closed provenance:** every job checks out
  the reviewed PR head and fails unless `git rev-parse HEAD` equals it;
  packaging refuses to produce an artifact unless
  `source_head_sha == checked_out_sha`; a provenance self-test proves the
  fail-closed path.
- **Security behaviors exercised by the core suite / typed-IPC tests:**
  traversal, absolute-path, symlink/junction, `.git`, unknown-operation,
  and stale-session requests are all denied fail-closed with typed,
  sanitized reasons; oversized/malformed/unsupported/wrong-session inputs
  fail closed.
- **Fresh artifacts** are bound to the reviewed head (`ffbb1a2…`) with
  ASCII `BUILD_INFO.txt` + `PROVENANCE.json` + `SHA256SUMS.txt`.
- The response-contract and provenance corrections changed **no**
  security semantic (verified by path audit: core/host/CSP/capability
  files untouched).

**These are build/test facts. They are NOT runtime, WebView, or
accessibility evidence.**

## B. Founder manual evidence — MANUAL VERIFIED — FOUNDER ATTESTATION

Collected interactively by the founder on a founder-controlled Windows 11
machine, from the provenance-bound artifact. This is founder attestation,
not an independent audit.

### Runtime

- The unsigned experimental application launched under **normal Windows
  protections**; **no protection was disabled or bypassed**.
- WebView2 rendered the UI successfully; the UI remained responsive.

### End-to-end operations (corrected runtime)

- `Core health: OK — protocol s05a/1, session established`
- `Read scoped fixture: OK — completed within capability scope (61 bytes)`
- `Read traversal: DENIED — traversal-or-prefix-rejected`
- `Write scoped output: OK — completed within capability scope (2 bytes)`
- `Write to .git: DENIED — git-metadata-access-rejected`
- `Unknown operation: DENIED — unknown-operation:shell_exec`

No result showed `undefined`, `null`, or `[object Object]`.

### Process topology

One verified run showed the Tauri host PID distinct from the core PID, the
core's `ParentProcessId` equal to the host `ProcessId`, and WebView2
descendants beneath the host — demonstrating a **separate core process on
Windows**. (The specific PIDs were transient and are **not** recorded as
architectural constants.)

### Network sample

The founder's `Get-NetTCPConnection` query for the Tauri host and core
returned no rows:
**No host/core TCP listener or external TCP connection observed during the
captured sample.** This is a narrow observation of one sample; it is **not**
proof that no network activity can ever occur.

### Crash and recovery

- Terminating the core did **not** terminate the host; the host UI
  remained available.
- No unexpected partial files or `.git` material were observed.
- Core health was restored after the accepted recovery/relaunch
  interaction; a fresh session was established.
- **An always-on background auto-restart supervisor was NOT established as
  a product requirement by this prototype.** Production UX must provide
  explicit **core-unavailable** and **restart** controls. No complete
  production supervisor was implemented.

### Accessibility (founder attestation)

The Windows prototype passed, per founder manual testing: keyboard-only
navigation; logical tab order; visible focus; Enter and Space activation;
no observed mouse-only operation; Windows **Narrator** labels and
operation-result announcements; Arabic/RTL switching and return to LTR;
text scaling; high contrast; reduced-motion compatibility.

**Limits:** this is founder manual evidence, **not** an independent
accessibility audit; **macOS VoiceOver and Linux Orca were not tested**;
the production UI will be substantially more complex and must be retested.

## C. Performance evidence

### Artifact size — PASS (lightweight hypothesis)

- Unpacked artifact: **8.49 MB**
- Tauri host executable: **8.22 MB**
- Rust core executable: **0.27 MB**

`PASS for the S0.5A lightweight artifact hypothesis.` These are **not**
final product sizes.

### Memory — SUPPORTED WITH RESERVATION

Founder measurement of the host descendant tree:

| Metric | Value |
| --- | --- |
| process count | 9 |
| combined Working Set | 404.13 MB |
| combined Private Memory | 186.83 MB |
| Rust core Working Set | ~3.98 MB |
| Rust core Private Memory | ~0.61 MB |
| Tauri host Working Set | ~26.70 MB |
| Tauri host Private Memory | ~4.79 MB |

Interpretation: the original **≤250 MB combined Working Set** hypothesis
was **not met** under this measurement; **private memory stayed below
200 MB**; the descendant Working Set includes multiple WebView2 processes
and may count shared pages repeatedly, so **404.13 MB must not be claimed
as unique physical memory**. **No target is silently redefined.** A refined
Windows memory methodology and an optimization gate are **mandatory before
Beta**; memory does **not** block founder-controlled Personal Alpha.
Classification: **SUPPORTED WITH RESERVATION.**

### Idle CPU — PASS (measured sample)

Valid independent 60-second idle sample: **0.0938 s** CPU time,
**0.013%** average system CPU → `PASS — effectively idle during the
measured sample.` (A later `0.039%` figure reused the original baseline
without a fresh 60-second sample and is **not** treated as an independent
sample.)

### Startup timing — NOT MEASURED

No exact numeric cold/warm launch timings were recorded. The founder
reported startup behavior as acceptable, but the **numeric startup target
remains `NOT MEASURED`**; numeric launch evidence is required before Beta.
No launch times are invented here.

## D. Preserved historical findings

### S05A-RUNTIME-001 (preserved)

All six UI operations originally rendered `undefined`. Root cause:
`invoke<CoreResponse>` treated a runtime JSON **string** as an object, so
`.kind` was `undefined`; the **core envelopes themselves were correct**.
Correction: parse exactly once, validate, normalize, and render
deterministic typed responses; real-core bridge regression tests pass.
This failure remains historical evidence and is **not erased**.

### S05A-PROVENANCE-001 (preserved)

An earlier artifact labeled a synthetic pull-request merge SHA
(`a7f61d7…`) as the source commit, because the default `pull_request`
checkout used `refs/pull/9/merge`. The experiment subtree was
**byte-identical** to the reviewed head (`8c01f0ec…`), making this a
**labeling defect, not a different-binary defect**. Correction: the
workflow explicitly checks out the PR head; `source_head_sha ==
checked_out_sha` is enforced fail-closed; `workflow_sha` is recorded
separately; `PROVENANCE.json` and ASCII `BUILD_INFO.txt` are produced; and
provenance validation gates artifact upload.

### Execution deviation (disclosed, non-blocking)

The provenance-correction authorization requested **one** correction
commit; execution produced **two** because the first commit's new workflow
guard exposed a Windows shell-default defect (a job-level `defaults.run`
had dropped `shell: bash`, so the guard ran under PowerShell). Amendment
and force-push were prohibited, so the second commit was **workflow-only**
and preserved history; **no security or product semantics changed.** The
deviation is disclosed here, not concealed.

## E. Accepted architecture conclusions (founder-approved)

1. Tauri 2 is suitable for founder-controlled Windows Personal Alpha.
2. The UI-zero-authority architecture is retained.
3. The separate Rust core process is retained.
4. Typed framed local IPC is retained as the current direction.
5. The frontend must treat every runtime value as untrusted and validate
   it.
6. Artifact provenance must bind builds to reviewed source heads.
7. Human-readable authorization explanations are mandatory.
8. Electron fallback is not triggered.
9. macOS and Linux runtime support remain provisional.
10. Working-set optimization and refined measurement are required before
    Beta.
11. Production core recovery requires explicit human-visible controls.
12. No prototype implementation is approved for automatic reuse.

## F. Unmeasured areas and residual risks

- **Unmeasured:** numeric cold/warm startup timing; a refined
  unique-physical-memory methodology; installer size (no installer built);
  macOS/Linux runtime, accessibility, and performance; signed-build
  behavior.
- **Residual risks (honest):** unsigned artifacts may be blocked by
  endpoint policy (recordable, never bypassed); a single network sample is
  not a guarantee; founder accessibility attestation is not an independent
  audit; per-platform WebView variance remains the principal open risk off
  Windows; full-OS-compromise remains out of scope per the Security
  Constitution.

## G. Non-authorization statement

This closeout package is **documentation only**. It does **not** authorize:
product implementation; copying prototype code into product paths; S1;
S0-B; CoWork; providers; SQLite schemas; PostgreSQL; GitHub integration;
Agents; the Studio Shell; updates; plugins; Package A; the Evaluation
Spine; an official EvaluationRun; or Native Delivery V0. PR #9 remains
never-merge and is not closed by this package.
