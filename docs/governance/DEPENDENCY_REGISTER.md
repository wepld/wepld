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

## Direct external dependencies (frozen baseline)

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
| rustix | (via workspace) | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | capability-safe no-follow syscalls | Recorded, pending scan |
| ureq | 3.3.0 | MIT OR Apache-2.0 | HTTP client (providers) | Recorded, pending scan |
| url | 2 | MIT OR Apache-2.0 | standards-compliant URL parsing | Recorded, pending scan |

Transitive dependencies are pinned by the baseline lockfile identified
above and enter this register individually at the first automated scan.

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
