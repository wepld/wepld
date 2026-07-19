# GDR-004 — AI-Assisted Development Policy

- **Status:** Adopted (founder decision, 2026-07-19)
- **Prerequisites:** GDR-001, GDR-003

## Context

WePLD is developed with substantial AI assistance. Copyright treatment
of AI-generated output varies by jurisdiction and is unsettled;
providers' terms bind usage; and generated code can carry third-party
provenance. The project needed a policy that maximizes the ownership
position without overclaiming it.

## Decision

Adopt `../AI_ASSISTED_DEVELOPMENT.md`: one named human owner per
AI-assisted change; human review and understanding before merge;
provenance records and tool disclosure where practical; derivative-code
checks; license and dependency scanning; provider restrictions for
confidential material; prohibition on submitting customer or regulated
data without authorization and contractual basis; human authorship,
selection, arrangement, and editing discipline; and passage through all
existing verification gates. AI coding agents are tools under a
responsible human's authority, not independent contributors.

## Rationale

Human creative control plus provenance records is the strongest
available copyright posture for AI-assisted work; treating output as
proprietary material under confidentiality and contract protects it even
where copyrightability is uncertain. The policy explicitly does not
claim AI output is automatically copyrightable or automatically
exclusively owned.

## Consequences

Every AI-assisted change has an accountable human; provider terms are
treated as part of the dependency surface; confidential material flows
only to approved providers.

## Legal-review boundary

Internal adoption needs no counsel. Any ownership-critical claim or
dispute involving AI-generated material is a legal-escalation event.

## Supersession rules

Policy revisions are dated in the policy file; a change to the
decision's substance requires a successor GDR linking here.
