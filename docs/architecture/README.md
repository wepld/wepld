# WePLD Architecture Documentation

S0-A core technology package (founder decision, 2026-07-19), amended by
the S0.5A outcome (2026-07-20). Documentation only — no implementation or
prototype is authorized by these files. The technology stack is an
approved direction, **not a finished cross-platform freeze**: after
S0.5A, Tauri 2 and the separate-process Rust core are **ratified for
founder-controlled Windows Personal Alpha only**; macOS/Linux runtime
remains unverified.

| Document | Subject |
| --- | --- |
| `TECHNOLOGY_CONSTITUTION.md` | languages, persistence, dependencies, exceptions |
| `DESKTOP_ARCHITECTURE.md` | process topology, IPC, update rules, evidence gates |
| `evidence/S0-5A-WINDOWS-DESKTOP-SECURITY-EVIDENCE.md` | S0.5A automated + founder-manual desktop-security evidence |

## Decision records (TDR series)

Separate from the security SDR series (`../security/decisions/`), the
legal-governance GDR series (`../governance/decisions/`), and the
validator-locked architecture ADR series (`../adr/`).

| Record | Decision | Status |
| --- | --- | --- |
| `decisions/TDR-001-technology-constitution.md` | technology constitution | Adopted (prototype-dependent clauses excluded) |
| `decisions/TDR-002-desktop-shell-selection.md` | desktop shell selection | Partially Ratified — Windows Personal Alpha; macOS/Linux Runtime Evidence Required |

Deferred to S0-B and later packages: CoWork architecture and
authorization, synchronization model, provider architecture, plugin
architecture, secure-update implementation, performance budgets,
incident response, assurance case.
