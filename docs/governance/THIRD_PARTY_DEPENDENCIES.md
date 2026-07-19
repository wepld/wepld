# Third-Party Dependency and Asset Policy

**Status:** internally adopted (GDR-005, 2026-07-19). Internal policy;
not legal advice. This is a documentation-only policy: no dependency is
added, removed, or replaced by its adoption.

## Outbound versus inbound licensing — the core distinction

WePLD's **outbound** license (the proprietary `LICENSE` notice) governs
what others may do with WePLD's own material. **Inbound** dependency
licenses govern what WePLD may do with third-party material. They are
independent: consuming a permissively licensed dependency does not change
WePLD's outbound posture, and WePLD's proprietary posture never overrides
any third-party dependency, model, dataset, or API terms.

**Specifically: consuming an Apache-2.0-licensed Rust crate does not
license WePLD itself under Apache-2.0.** Apache-2.0 is a permissive
license that permits use inside proprietary software subject to its
notice and attribution conditions. The crate remains Apache-2.0; WePLD's
own code remains proprietary.

## Allowed with registration (permissive)

MIT, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, Unicode, and Apache-2.0
dependencies may be used, subject to:

- an entry in `DEPENDENCY_REGISTER.md` (name, version, license, role);
- attribution and notice compliance at distribution time;
- compatibility review when a dependency's terms interact with others;
- distribution-time review before any external binary or source delivery
  (permissive obligations such as notice preservation travel with
  distributed artifacts).

## Explicit founder and legal approval required before use

- GPL, AGPL, or other strong-copyleft licenses;
- non-commercial licenses;
- research-only licenses;
- source-available licenses with field-of-use restrictions (for example
  BUSL- or SSPL-class terms) where WePLD would embed or distribute the
  component;
- models or datasets whose terms restrict commercial use or impose
  output-sharing obligations;
- copied code snippets with unclear or incompatible provenance
  (StackOverflow-era snippets commonly carry CC BY-SA terms and are
  effectively prohibited in proprietary code absent legal review).

## Process

- Every dependency addition records a register entry at review time.
- The initial dependency review covers the Rust dependencies recorded in
  the frozen experimental baseline's `Cargo.lock` (see
  `DEPENDENCY_REGISTER.md`); automated license scanning joins CI when
  Rust code lands on the canonical main branch, since main is currently
  documentation-only and repository tooling there cannot run a cargo
  scan.
- Weak-copyleft licenses (for example MPL-2.0, LGPL) require founder
  review before adoption; their file- or library-scoped obligations are
  assessed at distribution time.
- No working dependency is removed or replaced as part of this policy's
  adoption.
