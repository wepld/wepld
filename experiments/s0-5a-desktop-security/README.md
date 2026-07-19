# S0.5A — Desktop Security Prototype

**EXPERIMENTAL — NEVER MERGE.** This directory is a throwaway technical
spike authorized solely to produce evidence for TDR-002 (desktop shell
selection). It is not product code, it is not approved for reuse, and
its pull request must be closed unmerged after founder review. No
dependency used here becomes production-approved by appearing here.

## Purpose

Evaluate, with reproducible evidence, whether Tauri 2 plus a **separate
Rust trusted-core process** behind a **typed, versioned, session-bound
IPC boundary** is a workable topology for WePLD:

```text
Untrusted React / strict-TypeScript UI
        v
Minimal Tauri host bridge
        v
Typed and versioned IPC (framed stdio, non-HTTP)
        v
Separate Rust core process
        v
Capability-scoped prototype operations
```

## What it contains

- `protocol/` — envelope types and framing (Rust, `forbid(unsafe_code)`)
- `core/` — the separate core process, a static two-entry capability
  table, path confinement, and the automated security test suite
- `app/` — minimal Tauri 2 host + React strict-TS frontend (status
  display, operation buttons, capability explanations, self-test)
- `scripts/`, `fixtures/`, `PROTOCOL.md`, `EVIDENCE.md`,
  `LIMITATIONS.md`, `DEPENDENCY_EVIDENCE.md`

## How to run

```text
# core + security tests + bench (small dependency tree)
cargo test --locked --manifest-path experiments/s0-5a-desktop-security/Cargo.toml
cargo run  --locked --release --manifest-path experiments/s0-5a-desktop-security/Cargo.toml --bin s05a-bench

# app (heavy; requires npm and platform WebView toolchain)
cd experiments/s0-5a-desktop-security/app
npm ci --ignore-scripts && npm run build
cargo build --release --locked --manifest-path tauri-host/Cargo.toml
```

## What this prototype does NOT prove

- It does not prove WePLD is secure; it exercises one proposed boundary
  under prototype conditions only.
- Compilation on a platform does not prove runtime behavior there.
- Evidence from one OS is not evidence for another.
- Configuration (CSP, capability files) is a control, not proof.
- Full OS compromise remains out of scope entirely (see
  `LIMITATIONS.md`).

## Cleanup expectations

The branch is deleted after the founder closes the PR unmerged. Any
accepted architectural result must be re-recorded through a separate
documentation-only PR from canonical main. Local build outputs
(`target/`, `node_modules/`, `frontend/dist/`, self-test logs) are
git-ignored and disposable.
