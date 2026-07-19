# WePLD Trusted Computing Base

**Status:** adopted (SDR-002, founder decision, 2026-07-19). The
process-separation clause is provisional pending prototype evidence
(S0.5A; see `../architecture/decisions/TDR-002-desktop-shell-selection.md`).

## Definition

The Trusted Computing Base (TCB) is the set of components whose
correctness the security objective depends on directly: a defect inside
the TCB can defeat authorization, evidence, or confidentiality without
any other failure. Everything outside the TCB is designed so that its
compromise is survivable.

## Trusted components (each justified as unavoidable)

| Component | Why trust is unavoidable |
| --- | --- |
| Capability broker | issues, validates, and revokes every authority grant; nothing else may mint authority |
| Policy engine | evaluates precedence (including explicit deny and emergency freeze) for every effect; a wrong answer is an unauthorized effect |
| Identity and session verification | binds every action to an actor and device; broken binding breaks attribution and authorization |
| Canonical serialization | authorization decisions and hashes depend on unambiguous encoding; malleable encoding forges evidence |
| Cryptographic verification | signature and hash checking for updates, artifacts, and the ledger; the verifier must be correct even when everything it checks is hostile |
| Secure-update verifier | the supply-chain gate at install time; deferred from the founder-only Alpha (no updater exists there) but TCB-resident the day it exists |
| Ledger writer | append-only, hash-chained evidence; a corrupt writer silently rewrites history |
| Secret mediator | the only component that touches the OS keychain or vault; everything else sees handles |
| Workspace confinement layer | the no-follow, path-scoped filesystem primitives every worker uses; a hole here is arbitrary file access |
| IPC authorization layer | the choke point where every UI/agent request meets the policy engine |

## Less-trusted and untrusted components

- **Less trusted:** execution workers (capability-scoped, killable), the
  future CoWork server as seen from the client (server compromise must
  not silently rewrite local history), platform WebView and OS
  facilities (relied on, not vouched for).
- **Untrusted:** the entire UI; Hermes and all agents; model output and
  provider responses; plugins (none exist in Alpha); imported files and
  all workspace content; remote APIs; user automation; third-party
  integrations.

## Size and dependency discipline

The TCB is targeted to be **small enough for a single competent reviewer
to audit in full** — a discipline, not a marketing number; no line-count
target is claimed without measurement once the crates exist. Rules:

- TCB code lives in dedicated `wepld-tcb-*` crates linked only by the
  trusted-core process; no other process links them.
- TCB crates carry `#![forbid(unsafe_code)]` (exceptions per
  `SAFE_RUST_STANDARD.md` boundary-crate rules), minimal dependencies,
  and the highest review tier: fuzzing for every decoder, property
  tests for policy precedence, Miri on unit tests.
- Adding a dependency to a TCB crate requires its own justification in
  the dependency admission record.

## Process boundaries (provisional pending S0.5A)

```text
Tauri/WebView UI process        (untrusted)
        | typed, versioned IPC
Rust trusted-core process       (TCB lives here)
        | capability handoff
Capability-scoped workers       (less trusted, confined, killable)
```

The core runs as a process separate from the WebView shell so that a
renderer compromise faces a process boundary, not only a language
boundary. This separation is the provisional design; S0.5A must validate
its IPC ergonomics, performance, and failure behavior before it is
frozen. The UI/core separation itself — zero authority in the UI — is
**not** provisional and holds under any shell outcome.
