# GDR-007 — Distribution and Commercialization Gates

- **Status:** Adopted (founder decision, 2026-07-19)
- **Prerequisites:** GDR-001, GDR-003, GDR-005, GDR-006

## Context

WePLD intends to become a commercial software business, but no
counsel-reviewed external instruments exist. The largest realistic loss
of rights would come from external sharing before instruments exist.
Cumulative gates make correct sequencing structural.

## Decision

Adopt the following cumulative gates. Each tier requires everything
listed for it, and no tier may be entered early:

| Tier | Preconditions |
| --- | --- |
| Internal access (founder, future staff) | GDR-006 instruments executed; least-privilege grant |
| Contractor sharing | counsel-reviewed contractor agreement executed before access; scoped access; register entry |
| Prospect demonstration | confidentiality instrument or demonstration limited to Approved-for-Disclosure material; no source exposure |
| Evaluation access | counsel-reviewed evaluation license |
| Customer deployment or hosted service | counsel-reviewed commercial terms; security-disclosure policy; privacy and telemetry terms; jurisdiction check |
| Binary distribution | commercial license plus third-party notice-compliance audit (GDR-005) |
| Source delivery | founder decision plus counsel-drafted source terms; highest bar |
| Any open-source or source-available release | explicit founder reversal decision of GDR-001 plus legal review — never incidental |

## Rationale

Gates encode the rule that rights leave only through counsel-reviewed
instruments, and that publication or distribution is always a deliberate
founder decision rather than drift.

## Consequences

External sharing of any kind is blocked until its tier's instruments
exist; demonstration content must be explicitly classified; every
distribution event triggers the GDR-005 compliance audit.

## Legal-review boundary

Every externally binding instrument named above requires qualified
legal review in the relevant jurisdiction(s) before use. The gate table
itself is internal governance and binds no outside party.

## Supersession rules

Tier changes require a successor GDR linking here; individual gate
passages are recorded as dated decisions without superseding this
record.
