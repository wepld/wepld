# S0.5A Evidence Record

**EXPERIMENTAL — NEVER MERGE.** Evidence for TDR-002. Local interactive
evidence is Windows-only; Linux/macOS is CI build/test only. Compilation
is not runtime; one OS is not another.

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
