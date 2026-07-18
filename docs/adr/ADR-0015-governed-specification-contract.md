# ADR-0015 — Specification is the executable governance contract

**Status:** Accepted
**Date:** 2026-07-14
**Owner:** Architecture Review Board
**Review:** Before H1 implementation authorization

## Context

The current architecture moves from a mission brief to a task plan and places scope, success criteria, and acceptance on one Mission record. That does not provide a native clarification and approval method, separate WHAT from HOW, or prevent execution evidence or a planner from silently redefining the requested outcome.

## Decision

Adopt the hierarchy and artifact contracts in [31_Governed_Specification_Workflow.md](../31_Governed_Specification_Workflow.md). `EngineeringSpecification` and `OutcomeContract` are structured, versioned Core records. An approved specification defines WHAT and is immutable; a `ChangeRequest(kind=SpecificationChange)` creates a new version. Delivery and phase plans define HOW; HOW-only changes use `ChangeRequest(kind=PlanChange)`.

Markdown is a review/export projection, not the operational source of truth. Core records approval, supersession, trace edges, evidence requirements, and affected descendants.

## Reason

The product must provide its own engineering method. A durable approved contract gives the Brain Agent, Hermes, builders, reviewers, and users one common engineering truth while making scope, change, and evidence disputes mechanically visible.

## Benefits

- Explicit clarification, review, and approval.
- Immutable approved meaning with controlled adaptation.
- Requirement-to-evidence traceability.
- One semantic workflow across CLI, Studio, MCP, and API surfaces.

## Trade-offs

- Additional domain types and review steps.
- Mandatory version, supersession, and impact analysis.
- Material ambiguity prevents casual prompts from jumping directly to implementation.

## Migration

H1 evidence must prove Draft → clarification → review → approval; immutable approved versions; specification-versus-plan change classification; complete outcome/evidence bindings; denial of silent edits; and replayable approval provenance.

Draft PR #1's `SpecificationDocument`, version/provenance/link types, CAS, ledger, and spec-to-mission conversion are candidate seeds. The candidate lacks a separate approval contract and immediately advances a Draft specification; it is neither conformant nor canonical unless the baseline is accepted and explicitly migrated. This ADR does not authorize that merge.

## Acceptance disposition (2026-07-18)

Accepted by founder and Mastermind with the following V0 scope. This
acceptance records architecture only; it does not authorize PR #1 changes,
PR #1 reconciliation, or H1 implementation.

1. `MissionCharter` is the durable intake artifact for user intent, recorded
   by Core and never model-altered.
2. `EngineeringSpecification` is the approved, immutable, versioned WHAT.
3. Acceptance criteria reside in the Specification.
4. `OutcomeContract` is a separate versioned record binding each criterion to
   its verification method, evidence requirement, and completion
   interpretation.
5. Open questions block approval: a Draft specification cannot advance while
   material clarification is outstanding.
6. Models may propose drafts but cannot approve or mutate an approved
   version; change is a new version through `ChangeRequest`.
7. Core owns all three records, their version links, and the authenticated
   approval facts.
8. All three artifacts carry content hashes and supersession links.
9. Draft PR #1's `SpecificationDocument` remains only a migration seed; it
   becomes conformant only by adding the separate approval contract and
   immutable versioning under later, separately authorized reconciliation.
