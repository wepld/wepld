# Pre-H1 Evaluation Package (Package A)

**Standing — read this first.** This package is **documentation and protocol
only**. **No official EvaluationRun has occurred. No results are claimed.**
Draft PR #1 remains frozen at
`d5ef318468b6c35df3c14c1c5f72beb1191baf29` and untouched. The fixture
registry is **provisional until independent provenance approval**. Package B
(the minimal Evaluation Spine implementation) is separately gated and remains
unauthorized. Nothing in this package authorizes implementation, an official
run, PR #1 reconciliation, or Native Delivery V0.

## Contents

- [WEPLD-NATIVE-V0-BASELINE-1.md](WEPLD-NATIVE-V0-BASELINE-1.md) — the
  Pre-H1 baseline `EvaluationProtocol` (Proposed for independent provenance
  approval), including the twelve versioned cases, arms, run/result
  disposition model, execution-source identity, and the RunManifest field
  contract.
- [fixture-registry.yaml](fixture-registry.yaml) — the provisional,
  hash-bound fixture/comparator registry.
- [cassette-corpus-specification.md](cassette-corpus-specification.md) —
  requirements for future path-independent committed cassette corpora.
- [event-vocabulary-map.md](event-vocabulary-map.md) — historical ↔ canonical
  event vocabulary, with authority rules.
- [assessor-policy.md](assessor-policy.md) — the independent-assessment
  boundary; assessor assignment pending founder decision.
- [cache-and-environment-procedure.md](cache-and-environment-procedure.md) —
  the mandatory clean-environment procedure for the future run.

## Authority boundaries

Evaluation records evidence; it never decides. An `EvaluationResult` cannot
authorize implementation and cannot approve completion. ADR-0024 is Accepted
as architecture; constructing the spine (Package B) and executing the
official Baseline EvaluationRun each require their own separate
authorization.
