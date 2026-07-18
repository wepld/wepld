# Event Vocabulary Map — historical baseline ↔ canonical contracts

The PR #1 golden traces and ledger facts use the historical baseline
vocabulary. The accepted ADRs define the canonical families. This map exists
so comparators remain readable and honest across reconciliation. **Old
events never gain stronger retroactive authority through this map** — a
historical fact means exactly what it meant when recorded, under the
authority it had then.

| Historical (baseline) | Canonical (accepted contracts) | Notes |
| --- | --- | --- |
| `PlanApproved` | `PlanProposalSubmitted` → `PlanAssessmentRecorded` → `PlanIndependentReviewRecorded` → `PlanDecisionRecorded` | the single historical approval maps onto the final authenticated decision of the qualification pipeline; the intermediate records have no historical counterpart |
| `MissionAccepted` | `CompletionAccepted` (via `CompletionDecision`) | same meaning: authorized acceptance against the exact validated proposal |
| `MissionReturned` | `CompletionReturned` | explicit human return; terminal, not acceptance, not rejection |
| `CompletionProposed` | `CompletionProposed` | already canonical |
| deferred/uncertain acceptance states | `CompletionDeferred` / uncertain-effect states | recoverable, not final; never flattened |
| `InsightRecorded` (direct lesson write) | `MemoryCandidate` submission (candidate-only; no admission, no retrieval) | the historical direct write is a recorded baseline nonconformance with the accepted ADR-0020 candidate-only scope |

Rules: historical facts remain hash-verifiable and readable forever; readers
must handle both families explicitly; unknown schema versions fail closed;
no ledger history is ever rewritten; the schema/version mapping is
documented in the contracts crate at reconciliation. Baseline goldens and
post-reconciliation goldens are separate file sets and are never silently
substituted for one another.
