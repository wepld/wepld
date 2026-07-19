# Dependency Register

**Status:** initial register (GDR-005, 2026-07-19). Documentation only —
this register records existing dependencies; it adds, removes, or
replaces none.

## Scope note

The canonical main branch is currently documentation-only and contains no
Rust code, so no lockfile exists on main and no automated cargo-based
license scan can run there yet. The initial review therefore covers the
frozen experimental baseline (Draft PR #1, head
`d5ef318468b6c35df3c14c1c5f72beb1191baf29`), whose dependency identity is
pinned by its `Cargo.lock`
(SHA-256 `93e5e9bbc2664a0f48cf061e99ab5fc62b507c7c894f445aac8f54155c99ae8f`).
When Rust code lands on main under its own authorization, automated
license scanning joins CI and this register is re-verified against the
live lockfile.

Canonical main itself currently has exactly one class of third-party
dependency: the SHA-pinned GitHub Actions used by CI, recorded in their
own section below. The Rust crates in the next section exist **only on
the frozen PR #1 baseline branch**, not on canonical main.

## Direct external dependencies (frozen PR #1 baseline only — not on canonical main)

License identifiers below are recorded from the upstream projects' own
declarations and are pending automated re-verification at the first
CI-integrated scan. All are permissive-class licenses; each crate remains
owned and licensed by its respective owners.

| Crate | Version req | Declared license (upstream) | Role | Register status |
| --- | --- | --- | --- | --- |
| serde | 1 | MIT OR Apache-2.0 | serialization framework | Recorded, pending scan |
| serde_json | 1 | MIT OR Apache-2.0 | JSON serialization | Recorded, pending scan |
| schemars | 1 | MIT | JSON Schema derivation | Recorded, pending scan |
| rusqlite (bundled) | 0.40.1 | MIT (bundles SQLite, public domain) | ledger storage | Recorded, pending scan |
| sha2 | 0.11 | MIT OR Apache-2.0 | content hashing | Recorded, pending scan |
| ulid | 2.0.1 | MIT | identifier generation | Recorded, pending scan |
| thiserror | 2 | MIT OR Apache-2.0 | error derivation | Recorded, pending scan |
| chrono | 0.4.45 | MIT OR Apache-2.0 | timestamps | Recorded, pending scan |
| clap | 4 | MIT OR Apache-2.0 | CLI argument parsing | Recorded, pending scan |
| tempfile | 3 | MIT OR Apache-2.0 | race-safe temporary directories | Recorded, pending scan |
| rustix | 1 (direct, Unix-target only) | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | capability-safe no-follow syscalls | Recorded, pending scan |
| ureq | 3.3.0 | MIT OR Apache-2.0 | HTTP client (providers) | Recorded, pending scan |
| url | 2 | MIT OR Apache-2.0 | standards-compliant URL parsing | Recorded, pending scan |

`rustix` declaration evidence (corrected 2026-07-19 per independent
review): it is a **direct** dependency declared in
`crates/hermes/Cargo.toml` at the frozen baseline commit
`d5ef318468b6c35df3c14c1c5f72beb1191baf29`, under the target-specific
section `[target.'cfg(unix)'.dependencies]` as
`rustix = { version = "1", features = ["fs"] }` — Unix-only scope, not a
workspace-level dependency, and present only on the frozen PR #1
baseline, not on canonical main. Its license-verification state is
unchanged: recorded from the upstream declaration, pending automated
scan.

Transitive dependencies are pinned by the baseline lockfile identified
above and enter this register individually at the first automated scan.

## CI dependencies present on canonical main (GitHub Actions)

These are the only third-party dependencies present on canonical main
(`71df22cc90f9e5b2c27f72d5289a9634bdc6df83`). Both are pinned to
immutable commit SHAs in the workflow; the SHAs are authoritative and
must not be replaced by mutable version tags.

| Action | Immutable commit SHA (as pinned) | Release label | Category | Presence | License | Verification | Workflow path | Purpose | Approval status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| actions/checkout | `11bd71901bbe5b1630ceea73d27597364c9af683` | v4.2.2 (verified: GitHub tag metadata resolves this tag to this exact commit) | CI / GitHub Action | Present on canonical main | MIT | License verified 2026-07-19 from GitHub repository metadata (SPDX: MIT) | `.github/workflows/docs-validation.yml` | repository checkout for docs validation | In use since CI adoption; recorded 2026-07-19; no change authorized by this record |
| actions/setup-python | `a26af69be951a213d495a4c3e4e4022e16d87065` | v5.6.0 (verified: GitHub tag metadata resolves this tag to this exact commit) | CI / GitHub Action | Present on canonical main | MIT | License verified 2026-07-19 from GitHub repository metadata (SPDX: MIT) | `.github/workflows/docs-validation.yml` | Python 3.12 setup for the dependency-free validator | In use since CI adoption; recorded 2026-07-19; no change authorized by this record |

## Register rules

- Every new dependency adds a row here at review time, before merge.
- A dependency in any approval-required category of
  `THIRD_PARTY_DEPENDENCIES.md` must record its founder (and, where
  required, legal) approval in its row.
- Consuming an Apache-2.0 or otherwise permissively licensed crate does
  not change WePLD's own proprietary outbound license; see
  `THIRD_PARTY_DEPENDENCIES.md`.
- Distribution-time obligations (notice preservation, attribution) are
  reviewed before any external binary or source delivery.
