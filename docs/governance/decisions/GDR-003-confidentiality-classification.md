# GDR-003 — Confidentiality Classification

- **Status:** Adopted (founder decision, 2026-07-19)
- **Prerequisites:** GDR-001, GDR-002

## Context

Trade-secret and confidentiality value depends on secrecy measures that
are actually maintained. Private repository visibility alone is an
access control, not a legal protection. The project needed practical
classifications and controls, an explicit record of known risks, and a
recorded disposition for superseded local materials.

## Decision

Adopt the four-class scheme (Restricted, Confidential, Internal,
Approved for Disclosure) and the control set in
`../CONFIDENTIALITY.md`, plus the lightweight access-control policy in
`../REPOSITORY_ACCESS_CONTROL.md`. Two specific records are made:
(1) cloud-synced working copies (OneDrive-resident clones) are an
explicit confidentiality risk requiring a future founder disposition
decision; (2) the superseded local Package A worktree and branch
(commit `23e1d755fecb624101222a3c87943519b788d056`) are retained,
untouched Internal materials pending a future founder decision — no
deletion, move, cleanup, or rewrite is authorized by this record.

## Rationale

Controls create protection; classifications make controls applicable;
honest risk records (cloud sync) beat silent exposure; and superseded
materials are dispositioned deliberately, never as a side effect.

## Consequences

Repository content defaults to Confidential; disclosure requires the
GDR-007/GDR-008 gates; two founder decisions (cloud-sync disposition,
Package A remnant disposition) are queued as future items.

## Legal-review boundary

Internal controls need no counsel to adopt. Confidentiality obligations
binding any external party exist only through counsel-reviewed
agreements (deferred; see GDR-006/GDR-007).

## Supersession rules

Classification changes or risk-disposition outcomes are recorded as
dated follow-up decisions; wholesale replacement requires a successor
GDR linking here.
