# WePLD Desktop Architecture (Provisional)

**Status:** approved direction (S0-A, founder decision, 2026-07-19);
process-topology and shell clauses are provisional pending S0.5A
prototype evidence. This document authorizes no implementation and no
prototype.

WePLD is a cross-platform, local-first desktop application — not
primarily a website. Personal Mode works offline, without a WePLD
account, with authoritative state on the user's device. CoWork is the
built-in multi-user and multi-agent collaboration mode inside the same
application (not a separate client); its detailed architecture is
deferred to S0-B and appears here only as constraints.

## Provisional process topology

```text
Tauri/WebView UI process
        | typed IPC
Separate Rust trusted-core process
        |
Capability-scoped execution workers
```

- **UI process** — renders and requests; untrusted; zero authority.
- **Trusted-core process** — hosts the TCB (`../security/TRUSTED_COMPUTING_BASE.md`):
  policy engine, capability broker, secret mediator, ledger writer,
  provider gateway, workspace confinement. Provisionally a process
  separate from the WebView shell so a renderer compromise faces a
  process boundary; this separation is subject to S0.5A validation.
- **Workers** — short-lived, per-task processes for builds, tests, and
  agent file operations; confined to capability-scoped paths; no
  network by default; killable for cancellation.
- Local providers (for example Ollama) are external processes, never
  bundled.

## Trust boundaries and IPC

- Every UI→core command is typed, versioned, and schema-validated; the
  core authorizes each call against capabilities before any effect.
- Every IPC call carries process identity and session binding; requests
  without a valid session fail closed.
- Responses to the UI are data, never code, and are redacted: secret
  handles only, never values.
- Raw shell, filesystem, HTTP, or database APIs are not exposed to the
  WebView; the shell's API surface is the typed command set alone.
- **No embedded local HTTP server** may be introduced (for IPC, UI
  serving, or anything else) unless separately approved by a TDR — a
  listening socket is an attack surface that requires its own decision.

## Provider and filesystem access

All provider calls and all filesystem effects flow through the core:
classification, destination, and budget checks precede every provider
request; workspace IO uses the no-follow, path-confined layer. The UI
and agents never hold direct access.

## Personal Mode constraints

- Offline-capable and account-optional; no mandatory telemetry; no
  mandatory synchronization.
- Local SQLite is the authoritative store; workspace files belong to
  the user; secrets live in the OS keychain via the secret mediator.
- Cloud-synced folder locations are detected and warned about, with
  defaults outside synced directories.

## Updates — founder-only Personal Alpha rule

- The initial founder-only Personal Alpha ships **no automatic
  updater**.
- Every build is tied to a known commit with recorded hashes and build
  evidence.
- **Platform code signing is mandatory before any build is distributed
  outside founder-controlled devices.**
- **Secure update verification is mandatory before external
  evaluation.**
- **Key-rotation and TUF-style hardening are mandatory before customer
  distribution.**
- These thresholds apply to externally distributed builds without
  exception and are never weakened.

## Future evidence gates (separately authorized; not created now)

**S0.5A — Desktop Security Prototype.** Must evaluate: Tauri
sidecar/core-process IPC; capability mediation through the IPC
boundary; Windows, macOS, and Linux WebView behavior; keyboard and
screen-reader accessibility; startup time; memory use; installer size;
rendering fidelity; failure and recovery behavior. Its outcome ratifies
or reopens TDR-002; failure triggers fallback review (Electron), not
silent continuation.

**S6.5 — CoWork Sync Prototype.** Future, separately authorized, not
part of S0-A. Must evaluate: the SQLite operation log; PostgreSQL
authority; retries; idempotency; authorization at apply time; conflict
handling; offline revocation behavior; representative conflict load.
Recorded here only as a dependency of the deferred CoWork design.

## Fallback

If S0.5A shows Tauri failing its acceptance criteria on any tier-1
platform, Electron is the reviewed fallback: the React UI and the
separate Rust core survive the swap, because the UI/authority split —
not the shell — is the load-bearing decision.
