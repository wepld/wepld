# Assessor Policy — Pre-H1 Baseline EvaluationRun

assessor_assignment_status: PendingFounderDecision

No assessor is named in this package; assignment is a founder decision
recorded before assessment begins.

## Boundary rules

- The runner cannot be the sole assessor.
- The RunManifest hash and every run-record hash freeze **before**
  assessment begins.
- The assessor receives immutable evidence bundles (records, hashes, logs) —
  never mutable workspace state.
- Every `ProtocolDeviation` remains visible to the assessor; none may be
  hidden or summarized away.
- Missing evidence is never treated as a pass; it yields `NotAssessable` or
  `Fail` per the protocol's evidence checklists.
- The deterministic fixture-adapter arm (ARM-BASE/1) requires no provider
  independence — there is no provider.
- Role, session, and context independence remains mandatory wherever any
  model-assisted review participates in assessment.
- The finalized `EvaluationResult` cannot authorize implementation and
  cannot approve completion; downstream authorization is a separate
  authenticated decision that merely *requires* an assessed acceptable
  result.
