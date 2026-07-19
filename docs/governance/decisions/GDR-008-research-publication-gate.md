# GDR-008 — Research Publication Gate

- **Status:** Adopted (founder decision, 2026-07-19)
- **Prerequisites:** GDR-003, GDR-007

## Context

WePLD's technical material (architecture, evaluation methodology,
protocol designs) has potential research value, but public disclosure
can destroy trade-secret status immediately and can create patent bars
in many jurisdictions. Disclosure must therefore be gated, not
incidental.

## Decision

Any publication, paper, talk, blog post, demonstration recording, or
other public disclosure of WePLD technical material requires, in order:
(1) an invention-log review under `../INVENTION_LOG.md` — entries with
potential novelty and `NotDisclosed` status require an explicit founder
decision, with qualified patent counsel consulted where warranted,
before disclosure; (2) reclassification of the exact disclosed material
to Approved for Disclosure under GDR-003; (3) a recorded founder
approval naming the venue, scope, and date. Disclosed material is then
factually no longer secret, and records must not pretend otherwise.
A lightweight invention log is adopted at `../INVENTION_LOG.md`; it is
not a patentability assessment.

## Rationale

A pre-publication review is cheap; recovering from a disclosure-created
patent bar or a destroyed trade secret is often impossible. The gate
preserves options without blocking legitimate publication.

## Consequences

Publication becomes a three-step recorded process; the invention log
becomes a standing pre-disclosure checklist; academic or research
collaboration requests route through this gate plus GDR-007 tiers.

## Legal-review boundary

The gate is internal. Patent filings, patentability opinions, and any
publication agreement with an external venue or institution are
legal-review events in the relevant jurisdiction(s).

## Supersession rules

Gate changes require a successor GDR linking here; individual
publication approvals are dated records under, not amendments to, this
record.
