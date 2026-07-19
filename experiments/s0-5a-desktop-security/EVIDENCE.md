# S0.5A Evidence Record

**EXPERIMENTAL — NEVER MERGE.** Evidence for TDR-002. Local interactive
evidence is Windows-only; Linux/macOS is CI build/test only. Compilation
is not runtime; one OS is not another.

## S05A-RUNTIME-001 — end-to-end response contract defect (PRESERVED)

This is a historical pre-fix finding. It is preserved verbatim and must
not be erased or rewritten after the fix.

```
S05A-RUNTIME-001

Environment:
Founder-controlled Windows 11 device

Observed:
The assembled Tauri application launched and rendered, but all six
operation results appeared as `undefined`.

Classification:
Blocking end-to-end response contract defect.

Security interpretation:
No conclusion may be drawn from the UI about whether the underlying
operation succeeded or was denied. Existing isolated core tests remain
separate evidence.

Disposition:
Correction required and founder runtime rerun required.
```

**Root cause (affected layer = frontend runtime interpretation).** The
Tauri host command `core_request` returns `Result<String, String>` — a
JSON *string* (the core envelope). The pre-fix `ipc.ts` called
`invoke<CoreResponse>("core_request", …)` and read `.kind` off the
result. `invoke<T>` is a compile-time-only assertion; at runtime the
value is a **string**, so `string.kind` is `undefined`, and `App.tsx`
rendered `` `${resp.kind}` `` → `"undefined"` for every operation. The
core, IPC framing, capability enforcement, and denial reasons were all
correct and unchanged — the defect was purely that the UI treated a JSON
string as an object and never parsed/validated it.

**Why typecheck/CI missed it:** `invoke<CoreResponse>` asserts a type the
compiler trusts without runtime proof; there was no runtime UI/bridge
test (headless CI cannot drive the WebView), and the core tests exercise
the core, not the JS bridge. The regression tests below close that gap.

**Correction (frontend-only; no core/host security change).** A single
canonical bridge contract (`frontend/bridge.ts`): the raw `invoke()`
result is parsed **exactly once**, validated at runtime, and normalized
into `BridgeResponse { requestId, status: ok|denied|error, code,
message, data?, sessionId? }`; a rejected `invoke()` maps to a distinct
`bridge-invoke-rejected` error (never a success or a denial). The UI
renders a deterministic sanitized line into the semantic live region and
never displays `undefined` / `null` / `[object Object]`. Core capability
grants, allowed paths, denial precedence, session semantics, frame-size
ceilings, CSP, Tauri capabilities, navigation, network, and process
isolation are unchanged.

**Regression tests:** `frontend/bridge.unit.test.mjs` (18 assertions
incl. success/denial/error, malformed, missing-field, unsupported kind,
request-id mismatch, empty reason, raw-string, double-encoded, invoke
rejection, all six real envelopes, classification, no-bad-tokens) and
`frontend/bridge.realcore.test.mjs` (spawns the REAL core, runs the six
operations, normalizes the real envelopes, asserts classification and a
stale-session denial). Both via the Node built-in test runner (no new
dependency). Local: **18/18 pass with the real core; 17 pass + 1 skip
without a core binary.** Existing core security suite: unchanged.

**Residual limitations:** this fix restores the response *contract*; it
does not by itself prove the assembled Windows UI now renders correctly
end-to-end — that requires the founder smoke rerun (Test M). Crash/
restart, accessibility, and performance remain paused until the smoke
gate passes. macOS/Linux runtime remains NOT TESTED.

**Post-fix artifact commit:** recorded in the corrected artifact run (see
"Corrected artifact inventory" in the task report / the PR checks).

## S05A-PROVENANCE-001 — artifact source-commit mismatch (PRESERVED)

Historical finding; preserved, not erased.

```
S05A-PROVENANCE-001 — ARTIFACT SOURCE COMMIT MISMATCH

Observed (founder, Windows):
The downloaded Windows artifact binaries matched the corrected-run hashes
  host: b08a21449988408930a851a739eb03d331e28e1c454f94205573eb84803027f4
  core: b6efaa75fe439a1fb4f2b0bbd374194998063f403ce04b6a0eea0dc3c270dbbc
but BUILD_INFO.txt reported
  source_commit: a7f61d7c9e40b0081166d4dc6f9f2a2fefae1036
while the reviewed PR head was
  76c55580e19e6c3c3b528d8878293063e1cf2e51
Also: BUILD_INFO em dash rendered as mojibake in Windows PowerShell.
```

**Root cause (demonstrated, not assumed).** `refs/pull/9/merge` resolves
to `a7f61d7c9e40b0081166d4dc6f9f2a2fefae1036`, a synthetic GitHub merge
commit whose two parents are `e124e293…` (base / canonical main) and
`76c5558…` (reviewed head). On `pull_request` events `github.sha` is that
merge commit and `actions/checkout`'s **default** checks out
`refs/pull/N/merge`; the old `assemble-artifact.sh` recorded
`git rev-parse HEAD` (= the merge SHA) as `source_commit`. So the label
was the merge ref, not the reviewed head.

**Identical vs different tree content.** The built content was **identical
to the reviewed head**: `git rev-parse 76c5558:experiments/s0-5a-desktop-security`
and `…a7f61d7:experiments/s0-5a-desktop-security` are the same tree
(`8c01f0ec…`), because the base does not touch the experiment paths, so
the merge's experiment subtree equals the head's. **This was a provenance
labeling defect, not a different-binary defect** — but the artifact could
not be unambiguously attributed to the reviewed branch head, which is
blocking for provenance.

**Security interpretation.** No security-semantic conclusion changes: the
binaries were byte-for-byte what the reviewed head produces. The defect is
attribution integrity, not behavior.

**Correction.** (1) The workflow now checks out
`github.event.pull_request.head.sha` explicitly (manual runs require an
explicit `source_ref`) and **every job fails unless `git rev-parse HEAD`
equals that head**. (2) A five-field provenance model is recorded
distinctly — `source_head_sha`, `checked_out_sha`, `workflow_sha`,
`base_sha`, `artifact_tree_sha` — in `BUILD_INFO.txt` and a machine-
readable `PROVENANCE.json`; `workflow_sha` is never called `source_commit`.
(3) Packaging **fails closed** (no artifact) unless
`source_head_sha == checked_out_sha`. (4) `verify-provenance.sh` gates
upload: valid JSON, all fields, source==checked, BUILD_INFO/PROVENANCE
agreement, SHA256SUMS match, ASCII-only, expected-head match; plus a
`--selftest` that proves assembly fails on a forged head. (5) Provenance
files are **ASCII** (`EXPERIMENTAL - NEVER MERGE`, hyphen not em dash),
fixing the PowerShell mojibake.

**Required founder rerun:** re-verify the fresh artifact's hashes,
`BUILD_INFO.txt`, and `PROVENANCE.json` (source_head_sha == checked_out_sha
== reviewed head), then repeat the six-operation smoke gate, process
topology, and host/core TCP check. Crash/recovery, accessibility, and
final performance remain paused until provenance passes.

## Toolchains

- rustc / cargo **1.97.0** (host). CI prints the runner default.
- node **v22.23.1**, npm **10.9.8** (host); CI node 22.
- Tauri host resolved: **tauri 2.11.5**, tauri-build 2.x (lockfile).
- Frontend: react 18.3.1, @tauri-apps/api 2.9.0, vite 5.4.20, typescript 5.9.3.

## Dependency locks

- `Cargo.lock` (core workspace): **2 packages** (the two path crates;
  zero third-party).
- `app/tauri-host/Cargo.lock`: **420 packages**.
- `app/package-lock.json`: **68 packages**.

## Reproduce

```
# core (zero-dep; builds under Application Control policies)
cargo test  --locked --manifest-path experiments/s0-5a-desktop-security/Cargo.toml
cargo clippy --locked --all-targets --manifest-path experiments/s0-5a-desktop-security/Cargo.toml -- -D warnings
cargo run   --locked --release --bin s05a-bench --manifest-path experiments/s0-5a-desktop-security/Cargo.toml
# frontend (real strict typecheck + static bundle)
cd experiments/s0-5a-desktop-security/app && npm ci --ignore-scripts && npm run typecheck && npm run build
```

## Security tests (local, Windows 11, cargo test --locked)

**23/23 passed** (19 security + 4 JSON codec). Every negative failed
closed with a typed, sanitized response.

| # | Test | Result | Observed failure-closed reason |
| --- | --- | --- | --- |
| — | positive: health / read / write / echo / explain(allow+deny) | PASS | typed Ok; explain deterministic |
| 1 | unknown IPC operation | PASS | `unknown-operation:*` (Denied) |
| 2 | malformed message | PASS | `malformed-envelope` (Error); core keeps serving |
| 3 | oversized message | PASS | `oversized-frame:*`; core exits non-zero (fail closed) |
| 4 | unsupported protocol version | PASS | `unsupported-protocol-version:*` |
| 5 | wrong/expired session | PASS | `stale-or-unknown-session` (Denied) |
| 6 | missing capability grant | PASS | `capability-not-granted` (Denied) |
| 7 | wrong capability action | PASS | `missing-or-wrong-capability-for-action:read` |
| 8 | path traversal (`..`) | PASS | `traversal-or-prefix-rejected` |
| 9 | absolute-path escape | PASS | `absolute-path-rejected` |
| 10 | symlink / junction escape | PASS | `symlink-or-reparse-rejected` (Windows junction created + rejected) |
| 11 | write outside output scope | PASS | `traversal-or-prefix-rejected` |
| 12 | forbidden `.git` access | PASS | `git-metadata-access-rejected` (read + write) |
| 13 | UI request for unexposed command | PASS | `unknown-operation:*` for shell_exec/net_connect/secret_get/env_read/db_query |
| 14 | core crash visible | PASS | channel closure observed; no hang |
| 15 | restart → fresh session | PASS | new session id ≠ old |
| 16 | stale replay after restart | PASS | `stale-or-unknown-session` |
| 17 | malformed capability object | PASS | `malformed-envelope` (structured cap rejected at the seam) |
| 18 | unexpected extra fields | PASS | `malformed-envelope` (envelope) / `malformed-params` (params) |
| J | JSON codec: roundtrip / depth / duplicate-key / control-char / escapes | PASS (4) | depth>32, dup keys, control chars, trailing bytes rejected |

Health response contains protocol/build/session/engine only — **no
environment or secret fields** (asserted by the positive test).

## Frontend (local, Windows)

- `npm run typecheck` (tsc --noEmit, strict incl. exactOptionalPropertyTypes,
  noUnusedLocals): **PASS**. (It caught a real unused-import during
  development, then passed — evidence the strict config is live.)
- `npm run build` (vite production): **PASS** — 33 modules; static bundle
  `dist/index.html` 0.94 kB, `dist/assets/index-*.js` **145.87 kB (gzip
  47.25 kB)**. No remote origins referenced.

## Performance (local, Windows 11 Home, i5-13420H, 16 GB RAM, plugged in, release)

- Core release binary size: **286,208 bytes (~0.27 MB)**.
- Core idle memory (spawned, waiting on stdin): **~3.9 MB working set,
  636 KB private**. (This is the CORE only; it is NOT the WebView/shell,
  which was not measurable on this host — see LIMITATIONS.)
- Core-process handshake (spawn → Hello): first-run **~1206 ms** (fresh
  binary; dominated by Windows Defender first-execution scan), warm
  **19.6–24.0 ms** across repeats.
- IPC echo round-trip (3000 iters/run, several runs): **p50 ≈ 0.029–0.035
  ms, p95 ≈ 0.053–0.064 ms, p99 ≈ 0.086–0.113 ms**, max < 0.82 ms.
- Malformed-frame rejection latency: **≈ 0.03–0.21 ms**.
- Interpretation: IPC overhead is ~2–3 orders of magnitude below the
  100 ms interactive budget. Cold-start-to-interactive of the assembled
  Tauri app was NOT measured (host cannot build the shell); only the core
  handshake is measured here.

## Host environment finding (recorded, not worked around)

The founder host enforces a Windows Application Control / Smart App
Control policy (state = 1) that **blocks execution of freshly compiled
`build-script-build` binaries** (`os error 4551`). Dependency-free
binaries run; serde/serde_json (which have build scripts) could not be
built locally. Consequences, all honest: (a) the core was made
zero-dependency, which is why it builds and runs locally at all; (b) the
Tauri host (proc-macro/build-script heavy) is **build-only via CI** with
no local runtime evidence. This is itself relevant TDR-002 evidence:
Tauri's build surface is large enough to be blocked by a strict endpoint
policy.

## Cross-platform matrix (CI — recorded from run 29690991358, PR #9)

CI workflow `.github/workflows/s0-5a-desktop-security.yml`. **All 7 jobs
passed.** CI build success is **not** runtime or screen-reader evidence.

| Job | ubuntu-24.04 | windows-2022 | macos-14 |
| --- | --- | --- | --- |
| core: clippy -D warnings + test --locked + release build + bench | PASS (23s) | PASS (44s) | PASS (12s) |
| frontend: strict typecheck + vite build | PASS (15s, ubuntu) | — | — |
| tauri-host: frontend build + core build + `cargo build --locked` | PASS (3m58s) | PASS (5m20s) | PASS (1m9s) |

Interpretation, honestly bounded: the **separate core + typed IPC +
security suite build and pass on all three OSes** (strong evidence for
those components); the **Tauri host compiles on all three OSes**
(build-only evidence for the shell — a headless runner does not open a
real WebView window, drive it, or run a screen reader). Runtime and
accessibility remain untested (see LIMITATIONS).

## Accessibility

Implemented in markup; **interactively verified: none (NOT TESTED on any
OS)**. See LIMITATIONS and the report's accessibility section.

## Unresolved questions

- WebView engine behavior/accessibility per OS (needs interactive runs).
- Assembled-app cold/warm start, combined idle memory, installer size.
- Whether Tauri's third-party unsafe/build-script surface passes the
  Safe Rust Standard review for production (out of scope here).
