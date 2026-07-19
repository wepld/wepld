# AI-Assisted Development Policy

**Status:** internally adopted (GDR-004, 2026-07-19). Internal policy;
not legal advice.

AI coding agents are tools operated under a responsible human's
authority. They are not independent contributors, hold no repository
access of their own, and their output enters the repository only under
the operating human's obligations.

## Requirements

1. **Named human owner.** Every AI-assisted change has exactly one named
   human owner who is responsible for it.
2. **Human review before merge.** The owner reviews and understands
   AI-assisted output before it merges. Unreviewed generation never
   lands.
3. **Provenance records.** Material AI-assisted work keeps a provenance
   record: what was AI-assisted, when, and by whom.
4. **Tool disclosure.** The tool or model used is disclosed where
   practical (the repository's commit-trailer practice implements this
   for commits).
5. **Derivative-code checks.** Generated output is checked for copied or
   suspiciously derivative third-party code; anything resembling
   third-party material is handled under `THIRD_PARTY_DEPENDENCIES.md`.
6. **License and dependency scanning.** AI-assisted changes pass the same
   license and dependency review as any other change.
7. **Provider restrictions.** Confidential WePLD material may be
   submitted only to model providers approved under `CONFIDENTIALITY.md`,
   with retention or training opt-outs applied where offered.
8. **Customer and regulated data.** Customer data or regulated data is
   never submitted to any model provider without explicit authorization
   and a contractual basis. (No such data exists in the repository today;
   the rule binds future work.)
9. **Human authorship discipline.** Humans specify, select, arrange, and
   edit; records should reflect human creative control. This policy does
   not claim that AI-generated output is automatically copyrightable or
   automatically exclusively owned — copyright treatment of AI output
   varies by jurisdiction and remains unsettled.
10. **Verification gates.** All AI-assisted work passes the repository's
    existing verification gates (builds, tests, validators) without
    exception.

## Provider terms

Model-provider and API terms of use govern WePLD's use of those
services regardless of WePLD's own proprietary posture. WePLD's outbound
license never overrides a provider's inbound terms.
