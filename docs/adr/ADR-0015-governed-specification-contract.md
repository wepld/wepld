# ADR-0015 — Specification is the executable governance contract

**Status:** Proposed
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
