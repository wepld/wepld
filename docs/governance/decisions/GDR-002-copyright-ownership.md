# GDR-002 — Copyright Ownership

- **Status:** Adopted (founder decision, 2026-07-19)
- **Prerequisites:** GDR-001

## Context

WePLD has a single individual owner and no company entity. The ownership
record must be accurate today and must not invent an entity, while
leaving a clean path for a future assignment.

## Decision

The repository records ownership in a root `COPYRIGHT` file:
`Copyright © 2026 Abdulaziz M. Alshehri` / `All rights reserved.` The
record states that the current owner is the individual founder, that no
company entity is represented as owner, that any future transfer to a
company requires a separate written assignment with its own legal
review, and that third-party components remain owned and licensed by
their respective owners.

## Rationale

An accurate individual-ownership record is the correct legal posture
today and the cleanest starting point for a comprehensive
founder-to-entity assignment at incorporation. Misrepresenting a
nonexistent entity as owner would damage, not protect, the position.

## Consequences

Copyright notices across the package name the founder; the year extends
to a range as years accrue; at incorporation, this record is updated
with the executed assignment reference rather than rewritten.

## Legal-review boundary

The record itself documents facts and needs no counsel to adopt. The
future company assignment is a mandatory legal-review event: assignment
formalities differ across jurisdictions and must not be improvised.

## Supersession rules

Updated (append-style, with dated notes) upon an executed assignment;
otherwise superseded only by an explicit successor GDR linking here.
