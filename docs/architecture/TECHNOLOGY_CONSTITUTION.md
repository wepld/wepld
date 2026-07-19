# WePLD Technology Constitution

**Status:** adopted (TDR-001, founder decision, 2026-07-19), **except**
prototype-dependent clauses, which are marked provisional and governed
by `decisions/TDR-002-desktop-shell-selection.md`. The technology stack
is a founder-approved direction; it is **not fully frozen**.

## Languages

1. **Rust** is the authority-bearing product language: trusted core,
   workers, future backend services, CLI.
2. **Strict TypeScript** is limited to the presentation layer. UI code
   holds zero authority (see the prohibition below).
3. **Python** is limited to non-shipping research and evaluation
   tooling; it never ships in the product, never runs inside it, and
   has no product-data access by default.
4. Go, Java, Kotlin, C#, C++, Swift, Dart, embedded Python runtimes,
   and all other shipped languages are **excluded initially**.
5. Adding any shipped language requires a separately approved
   Technology Decision Record (TDR) with founder approval.

## UI authority prohibition

The user interface must never directly control: filesystem, shell,
network, secrets, databases, policy, capabilities, ledger, providers,
updates, or agent execution. All effects flow through the typed IPC
boundary into the Rust trusted core, where authorization happens. This
rule is constitutional and survives any change of desktop shell.

## Desktop shell (provisional)

Tauri 2 is the provisional primary shell and Electron the provisional
fallback. This clause is **Provisionally Approved — Prototype Evidence
Required**: it becomes final only after the separately authorized S0.5A
prototype validates the acceptance criteria in TDR-002. No product
implementation or prototype is authorized by this document.

## Persistence

- **SQLite** is the approved local embedded database direction for the
  desktop client.
- **PostgreSQL** is reserved for the future CoWork server. It must not
  be embedded in, installed by, or shipped with the desktop client.
- CoWork synchronization, isolation, and realtime design are deferred
  to the S0-B package and are constraints here, not decisions.

## Dependencies

- Admission per the merged third-party dependency policy
  (`../governance/THIRD_PARTY_DEPENDENCIES.md`): register entry,
  license check, unsafe-code and build-script review before merge.
- Lockfiles are committed and builds run `--locked`.
- CI actions are pinned to immutable commit SHAs (already the practice
  on canonical main and recorded in the dependency register).
- No git dependencies without a decision record.
- npm dependencies are minimized; generated UI code may not introduce
  dependencies outside the admission process.

## Cryptography

No hand-written cryptography. Cryptographic crates are selected by a
dedicated future TDR based on audit status and maintenance evidence.

## Plugins

No plugin system exists in Alpha. Any future plugin or extension
mechanism requires its own decision records and security review before
design freeze.

## Exceptions

Any deviation from this constitution — a new language, an embedded
server, a UI authority exception, an unpinned dependency — requires a
recorded, founder-approved TDR with an expiry or revisit date.
Exceptions are records, not habits.
