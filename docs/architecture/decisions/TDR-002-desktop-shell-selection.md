# TDR-002 — Desktop Shell Selection

- **Status:** **Provisionally Approved — Prototype Evidence Required**
  (founder decision, 2026-07-19)
- **Prerequisites:** TDR-001, SDR-002

## Context

WePLD is a cross-platform, local-first desktop application — not
primarily a website. The architecture gate compared Tauri 2, Electron,
Flutter, native per-platform UI, Qt, and Avalonia against security
boundaries, WebView risk, binary size, memory, accessibility,
cross-platform behavior, update security, maintainability, and
compatibility with the v0-generated React UI. Tauri 2 won on
size, startup, and Rust-boundary fit; its principal open risk is
per-platform WebView variance (behavior, patch cadence, accessibility),
which no document can resolve.

## Decision

- **Tauri 2 is the primary candidate** for the desktop shell.
- **Electron is the fallback**, preserving the React UI and the
  separate Rust trusted core if Tauri fails its acceptance criteria.
- The trusted core provisionally runs as a **separate process** from
  the WebView shell; this topology is likewise prototype-dependent.
- **No product implementation is authorized** by this record, and no
  prototype is authorized by the S0-A package that contains it.
- This decision becomes **Accepted** only after the separately
  authorized **S0.5A Desktop Security Prototype** validates, at
  minimum: Tauri sidecar/core-process IPC; capability mediation across
  the IPC boundary; Windows, macOS, and Linux WebView behavior;
  keyboard and screen-reader accessibility; startup time; memory use;
  installer size; rendering fidelity; and failure and recovery
  behavior.
- **Failure of the prototype triggers fallback review** (an explicit
  founder decision over Electron or another evaluated option) — never
  silent continuation with a failed candidate.

## Rationale

Committing documentation-level direction now unblocks S0-B design work,
while withholding Accepted status until prototype evidence prevents the
project from freezing an authority-adjacent choice on assertion — the
failure mode WePLD's governance history exists to prevent.

## Consequences

S0-B documents may assume the UI/authority split and typed IPC (which
hold under either shell) but must not hard-code Tauri-only mechanisms
as final; the S0.5A outcome is recorded as a dated amendment moving
this record to Accepted or reopening it for fallback review.

## Legal-review boundary

None; third-party framework licenses flow through the dependency
policy.

## Supersession rules

Only the S0.5A outcome amendment or a successor TDR (founder-approved,
linking here) may change this record's status.
