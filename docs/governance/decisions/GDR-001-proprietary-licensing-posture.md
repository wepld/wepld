# GDR-001 — Proprietary Licensing Posture

- **Status:** Adopted (founder decision, 2026-07-19)
- **Prerequisites:** none (series root)

## Context

The prior Apache-2.0 outbound licensing decision was revoked by founder
emergency decision on 2026-07-18; the repository was made Private, the
superseded licensing pull request (PR #6) was closed unmerged, and its
remote branch was deleted. WePLD is proprietary, closed-source, and
intended to become a commercial software business, currently owned by the
individual founder. The repository needed an explicit outbound posture so
that "no license file" ambiguity could not recur.

## Decision

WePLD adopts a minimal all-rights-reserved proprietary posture, recorded
as a short repository-level `LICENSE` notice that: identifies WePLD as
proprietary; reserves all rights; states that access grants no rights to
use, copy, modify, distribute, sublicense, disclose, commercialize,
reverse engineer, train models on, or create derivative works; states
that no license arises from access, cloning, viewing, or possession;
requires explicit written authorization for external permissions; and
leaves third-party dependencies under their own licenses. No open-source
grant, patent grant, or Apache-2.0-derived text appears in WePLD-owned
licensing files.

## Rationale

A short reservation notice is unambiguous, creates no premature grant,
and avoids drafting unreviewed contract terms while there is no external
grantee. Fuller instruments (customer, evaluation, commercial licenses)
are deferred to qualified counsel when real counterparties exist.

## Consequences

The revoked Apache-2.0 posture is replaced repository-wide; any future
open-source or source-available release would be a deliberate reversal
requiring its own founder decision and legal review; the documentation
validator's changed-scope allowlist admits the root governance files.

## Legal-review boundary

The notice is internally adopted and must receive qualified legal review
in the relevant jurisdiction(s) before external contractual reliance. It
is not legal advice and not a complete commercial agreement.

## Supersession rules

Superseded only by a later GDR that explicitly names this record,
records the founder decision, and undergoes review; the superseding
record links back here. This record is never edited to change its
decision retroactively.
