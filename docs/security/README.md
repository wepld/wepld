# WePLD Security Documentation

S0-A core security package (founder decision, 2026-07-19).
Documentation only — no implementation is authorized by these files.
The technology stack is approved as a direction and is not fully
frozen; prototype-dependent clauses are labeled provisional.

| Document | Subject |
| --- | --- |
| `SECURITY_CONSTITUTION.md` | mission, principles, red lines, residual risk |
| `THREAT_MODEL.md` | present, future, deferred-CoWork, and residual threats |
| `TRUSTED_COMPUTING_BASE.md` | trusted/untrusted components, process boundaries |
| `CAPABILITY_MODEL.md` | capability schema, policy precedence, Work Contracts |
| `SAFE_RUST_STANDARD.md` | forbid(unsafe_code), FFI rules, verification ladder |

## Decision records (SDR series)

Separate from the architecture TDR series (`../architecture/decisions/`),
the legal-governance GDR series (`../governance/decisions/`), and the
validator-locked architecture ADR series (`../adr/`).

| Record | Decision | Status |
| --- | --- | --- |
| `decisions/SDR-001-security-constitution.md` | security constitution | Adopted |
| `decisions/SDR-002-trusted-computing-base.md` | TCB principles | Adopted (process clause provisional) |
| `decisions/SDR-003-capability-and-policy-model.md` | capability and policy model | Adopted |
| `decisions/SDR-004-safe-rust-standard.md` | safe Rust standard | Adopted |
