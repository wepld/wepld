# WePLD Intellectual Property Governance

**Status:** internally adopted (founder decision, 2026-07-19). Governance
record, not legal advice. Documents intended to bind an external party
require qualified legal review in the relevant jurisdiction(s) before use.

## Posture

WePLD is proprietary, closed-source software intended to become a
commercial software business. The repository carries an all-rights-reserved
notice (`LICENSE`) and an ownership record (`COPYRIGHT`). The owner is the
individual founder, Abdulaziz M. Alshehri; no company entity currently
exists or is represented as the owner.

## Distinct kinds of rights (do not conflate)

| Kind | What it is here | What it is not |
| --- | --- | --- |
| Copyright | Vests in the author of original works; held by the founder for founder-authored material | Not a guarantee that every artifact (for example purely AI-generated output) is copyrightable in every jurisdiction |
| Confidentiality | Created by actual secrecy controls and, for outside parties, by contract | **Not** created automatically by private repository visibility — private visibility is an access control, not a legal protection by itself |
| Trade-secret controls | Depend on secrecy measures being genuinely maintained (see `CONFIDENTIALITY.md`) | Not a label; disclosed material may lose factual secrecy regardless of labels |
| Patents | Exist only if applications are filed and granted; a lightweight invention log preserves the option (`INVENTION_LOG.md`) | Nothing in this repository is a patent or a patentability assessment |
| Trademarks | "WePLD", "Hermes", "AGILLE" are currently unregistered names; rights accrue through use and registration, both future work | Not licensed or established by this package |
| Third-party licensing | Inbound dependencies remain under their own licenses (`THIRD_PARTY_DEPENDENCIES.md`) | WePLD's proprietary posture never overrides third-party dependency, model, dataset, or API terms |
| Contractual rights | Exist only where a written agreement exists; none are drafted in this package | Contractor or employee work product does **not** automatically belong to WePLD in every jurisdiction — written assignment is required |

## Asset classes covered

Source code, documentation, schemas, protocols, evaluation fixtures and
benchmarks, prompts, research notes, decision records, and generated
artifacts authored in this repository. AI-assisted output is governed by
`AI_ASSISTED_DEVELOPMENT.md`; it is treated as proprietary material under
confidentiality controls, without claiming it is automatically
copyrightable or automatically exclusively owned in every jurisdiction.

## Governing records

Decisions are recorded as Governance Decision Records in
`decisions/` (GDR-001 through GDR-008), a series deliberately separate
from the architecture ADR series. Policy documents in this directory
implement those decisions.

## Deferred matters (not implemented, legal review required before use)

Contractor, employee, and contributor agreements; customer, evaluation,
and commercial licenses; company IP assignment; trademark registration;
patent filings; international distribution terms; regulated-data
handling; any external source or binary distribution.
