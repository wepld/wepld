# TDR-001 — Technology Constitution

- **Status:** Adopted (founder decision, 2026-07-19), **except
  prototype-dependent clauses** (desktop shell and process topology),
  which are governed by TDR-002 and remain provisional
- **Prerequisites:** SDR-001

## Context

The read-only Security and Technology Architecture Gate evaluated Rust,
Go, Java, Kotlin, C++, Swift, C#/.NET, TypeScript, Python, and
WebAssembly against memory safety, performance, cross-platform
distribution, auditability, binary size, startup, supply-chain control,
and the founder's v0-based UI workflow. The founder provisionally
approved the overall direction while explicitly not freezing the full
stack.

## Decision

Adopt `../TECHNOLOGY_CONSTITUTION.md`: Rust as the sole
authority-bearing shipped language; strict TypeScript confined to the
presentation layer with zero authority; Python confined to non-shipping
research and evaluation tooling; Go, Java, Kotlin, C#, C++, Swift,
Dart, embedded Python runtimes, and other shipped languages excluded
initially, with any addition requiring a new founder-approved TDR;
SQLite as the local embedded database direction; PostgreSQL reserved
for the future CoWork server and never embedded in the desktop client;
dependency admission with committed lockfiles, `--locked` builds, and
SHA-pinned CI actions; no hand-written cryptography; no plugin system
in Alpha; all exceptions via recorded TDRs.

## Rationale

The smallest coherent language set that covers the authority path
(memory-safe, auditable, small binaries) and the founder's UI workflow
is two shipped languages. Every additional runtime multiplies
supply-chain surface, audit scope, and distribution weight without a
compensating capability.

## Consequences

Product code is Rust or presentation TypeScript, nothing else; local
persistence questions are settled in SQLite's favor for the client;
future proposals to add languages, servers, or plugins face a recorded
decision rather than drift.

## Legal-review boundary

None for internal adoption; third-party license obligations remain
governed by the merged dependency policy (GDR-005).

## Supersession rules

Any change to the language set, persistence split, or exception process
requires a successor TDR linking here. TDR-002's prototype outcome does
not amend this record; it resolves the clauses this record explicitly
excludes.
