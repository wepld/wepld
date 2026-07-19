# SDR-004 — Safe Rust Standard

- **Status:** Adopted (founder decision, 2026-07-19)
- **Prerequisites:** SDR-001, SDR-002; TDR-001

## Context

Rust is the authority-bearing language (TDR-001). Memory safety is a
default worth enforcing mechanically, but FFI and platform syscalls
sometimes require unsafe code, and build scripts and procedural macros
execute third-party code at build time.

## Decision

Adopt `../SAFE_RUST_STANDARD.md`: `#![forbid(unsafe_code)]` as the
default for WePLD-owned crates; exceptions only in dedicated boundary
crates with written justification, per-block `SAFETY` documentation, an
unsafe-code register, independent review, focused tests, and a
founder-approved exception decision; no hand-written cryptography;
build scripts and procedural macros treated as supply-chain execution
surfaces at dependency admission; a verification ladder (Miri, fuzzing,
property testing, sanitizers, advisory scanning) adopted as future CI
gates; performance exceptions only with benchmark evidence.

## Rationale

The frozen PR #1 baseline demonstrated both the value and the
manageability of this discipline (a single documented rustix exception
behind a safe wrapper) — cited as historical evidence, not merged code.
Mechanical defaults plus registered exceptions beat case-by-case
judgment.

## Consequences

Future crates start safe by construction; the unsafe register gives the
founder and future auditors a complete inventory; dependency admission
gains explicit build-time-execution review.

## Legal-review boundary

None.

## Supersession rules

Rule changes require a successor SDR linking here; new tooling in the
verification ladder may be added by dated amendment.
