# S0.5A Dependency Evidence

**EXPERIMENTAL — NEVER MERGE.** No dependency becomes production-approved
through this document. Inclusion here records what the *prototype* uses,
not what WePLD product may use. Production dependency admission remains
governed by the merged `docs/governance/THIRD_PARTY_DEPENDENCIES.md`.

## Toolchain (host, recorded at run time)

- rustc / cargo: 1.97.0 (host); CI uses the runner default and prints it.
- node: v22.x (host and CI).
- Tauri CLI/host crates resolved (see below).

## Rust — core workspace (`protocol` + `core`)

**Zero third-party dependencies. Zero build scripts. Zero procedural
macros.** `cargo tree` is trivial: the two path crates only. This is a
deliberate result: the founder host runs a Windows Application Control
policy that blocks execution of freshly compiled `build-script-build`
binaries (`os error 4551`), which made serde/serde_json unbuildable
locally. Rather than depend on build-script crates, the prototype
hand-rolls a small depth-limited JSON codec (`protocol/src/json.rs`).
This both (a) unblocks local build/test/bench and (b) minimizes the
supply-chain surface to nothing for the part under security evaluation.

| Crate | Version | Purpose | License | Source | Build script | Proc-macro | Unsafe (WePLD-owned) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| s05a-protocol | 0.0.1 (path) | framed-stdio IPC + mini-JSON | proprietary (this repo) | in-tree | none | none | `#![forbid(unsafe_code)]` |
| s05a-core | 0.0.1 (path) | separate core process + tests + bench | proprietary (this repo) | in-tree | none | none | `#![forbid(unsafe_code)]` |

Production note: production WePLD would almost certainly use an audited
serde stack rather than a hand-rolled codec. The hand-rolled parser is
**prototype-only** and is not proposed for product use.

## Rust — Tauri host (`app/tauri-host`, standalone lockfile)

Resolved lockfile: **420 packages**, `tauri 2.11.5`, `tauri-build 2.x`.
Direct declared dependencies:

| Crate | Version (req) | Purpose | License (upstream) | Build script | Proc-macro | Unsafe |
| --- | --- | --- | --- | --- | --- | --- |
| tauri | 2 (→2.11.5) | desktop shell / WebView host | MIT OR Apache-2.0 | yes (many transitive) | yes (transitive) | third-party unsafe present (webview/OS FFI) — NOT WePLD-owned |
| tauri-build | 2 | build-time codegen (build-dependency) | MIT OR Apache-2.0 | yes (this IS a build script) | — | third-party |
| s05a-protocol | path | shared IPC types | proprietary | none | none | forbid(unsafe) |

**Third-party unsafe:** the Tauri dependency tree contains unsafe code
(WebView bindings, OS integration, FFI). This is reported honestly as
**third-party** unsafe; it is NOT WePLD-owned unsafe and is NOT counted
against the Safe Rust Standard's `forbid(unsafe_code)` rule for
WePLD crates. A production decision to adopt Tauri would require
reviewing this surface under the Safe Rust Standard and dependency
policy — out of scope for this spike.

**Build scripts / proc-macros:** numerous, across the Tauri tree. This is
exactly the supply-chain execution surface the Safe Rust Standard flags,
and exactly what the founder host's Application Control policy blocks —
which is why the Tauri host is **build-only via CI** and was not built on
the founder host.

## JavaScript — frontend (`app`, `package-lock.json`)

Resolved: 68 packages. Direct dependencies:

| Package | Version | Purpose | License (upstream) | Install scripts |
| --- | --- | --- | --- | --- |
| react | 18.3.1 | UI runtime | MIT | none |
| react-dom | 18.3.1 | UI runtime | MIT | none |
| @tauri-apps/api | 2.9.0 | typed `invoke` bridge to host | MIT OR Apache-2.0 | none |
| @types/react | 18.3.12 | types (dev) | MIT | none |
| @types/react-dom | 18.3.1 | types (dev) | MIT | none |
| @vitejs/plugin-react | 4.3.4 | JSX build (dev) | MIT | none |
| typescript | 5.9.3 | strict typecheck (dev) | Apache-2.0 | none |
| vite | 5.4.20 | bundler (dev) | MIT | none (esbuild ships a signed prebuilt binary) |

Installed with `npm ci --ignore-scripts`; no install-time scripts were
executed. License identifiers are upstream declarations pending
independent verification; none is production-approved here.

## No secrets, no network dependencies

No API keys, credentials, tokens, or remote runtime assets exist
anywhere in the prototype. No git dependencies. No unpinned remote
assets. All locked: `Cargo.lock` (core, 2 pkgs), `app/tauri-host/Cargo.lock`
(420 pkgs), `app/package-lock.json` (68 pkgs).
