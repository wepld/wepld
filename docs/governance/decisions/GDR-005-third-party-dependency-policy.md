# GDR-005 — Third-Party Dependency Policy

- **Status:** Adopted (founder decision, 2026-07-19)
- **Prerequisites:** GDR-001, GDR-004

## Context

WePLD's proprietary outbound posture coexists with inbound third-party
dependencies (predominantly permissively licensed Rust crates in the
frozen experimental baseline). The two directions were at risk of being
conflated — especially after the revoked Apache-2.0 outbound decision,
since Apache-2.0 remains a common and acceptable *inbound* dependency
license.

## Decision

Adopt `../THIRD_PARTY_DEPENDENCIES.md` and the initial
`../DEPENDENCY_REGISTER.md`. Permissive dependencies (MIT, BSD, ISC,
Zlib, Unicode, Apache-2.0) are allowed subject to registration,
attribution and notice compliance, compatibility review, and
distribution-time review. Consuming an Apache-2.0 Rust crate does not
license WePLD itself under Apache-2.0. GPL, AGPL, and other strong
copyleft; non-commercial; research-only; field-of-use-restricted
source-available licenses; commercially restricted models or datasets;
and unclear-provenance code snippets all require explicit founder and
legal approval before use. The initial review covers the frozen
baseline's `Cargo.lock` identity; automated scanning joins CI when Rust
code lands on main. No working dependency is removed or replaced by
this record.

## Rationale

Register-at-review-time plus distribution-time audit is the lightest
process that prevents both silent copyleft adoption and
distribution-time notice violations, while keeping the outbound/inbound
distinction explicit.

## Consequences

Every dependency addition carries a register entry; distribution events
gain a mandatory license-compliance step; third-party terms are never
claimed to be overridden by WePLD's proprietary license.

## Legal-review boundary

Internal adoption needs no counsel. First external distribution
(binary or source), and any approval-required-category dependency,
are legal-review events.

## Supersession rules

Category changes require a successor GDR linking here; register rows
evolve continuously under the policy without superseding this record.
