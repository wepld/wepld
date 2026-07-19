# WePLD Safe Rust Standard

**Status:** adopted (SDR-004, founder decision, 2026-07-19). This
standard defines rules for future WePLD Rust code; it authorizes no
implementation and requires no tool installation today.

## Default

Every WePLD-owned crate carries `#![forbid(unsafe_code)]`. Safety is a
default enforced by the compiler, not a review aspiration.

## Exceptions: unsafe code and FFI

Unavoidable `unsafe` (typically FFI or platform syscalls) is permitted
only under all of the following:

1. **Dedicated boundary crate** — the unsafe code lives in its own
   wrapper crate exposing a safe API; product crates never contain
   inline `unsafe`.
2. **Written justification** — why no safe alternative serves, recorded
   in the crate and in the unsafe-code register.
3. **Per-block `// SAFETY:` documentation** — every unsafe block states
   the invariants it relies on; `deny(unsafe_op_in_unsafe_fn)` applies.
4. **Unsafe-code register** — a maintained record (crate, block,
   justification, reviewer, revisit date) covering all WePLD unsafe
   code.
5. **Independent review** — someone other than the author approves the
   unsafe block.
6. **Focused tests** — targeted tests (and sanitizers where relevant)
   exercise the boundary.
7. **Founder-approved exception decision** — a recorded decision admits
   the exception; the frozen PR #1 baseline's documented rustix
   exception is the historical template (cited as precedent, not
   merged).

## Cryptography

No hand-written cryptography, ever. Only established, audited
cryptographic crates selected through a dedicated technology decision
record; constant-time and misuse-resistance claims are taken from
upstream documentation and advisories, never assumed.

## Dependency safety review

Admission review for third-party crates (per the Technology Constitution
and the merged third-party dependency policy) includes: unsafe-code
density and quality signals; maintenance and advisory status;
**build scripts (`build.rs`) and procedural macros reviewed as
supply-chain execution surfaces** — they run at build time with
developer privileges and are inspected before admission; git
dependencies prohibited without a decision record.

## Verification ladder (adopted as future CI requirements)

- **Miri** on TCB crate unit tests.
- **Fuzzing** for every parser, IPC decoder, and path-canonicalization
  routine.
- **Property testing** for policy-precedence evaluation and any merge
  or reconciliation logic.
- **Sanitizers** where FFI exists.
- Advisory scanning (`cargo audit` / `cargo deny` class tooling) in CI.

These become mandatory gates when the corresponding code exists; this
document does not install or run them.

## Performance exceptions

A performance-motivated exception to any rule above requires benchmark
evidence demonstrating the need, a decision record, and an unsafe-code
register entry — never assertion alone.
