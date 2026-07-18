# EvaluationProtocol — WEPLD-NATIVE-V0-BASELINE/1

**Protocol ID:** `WEPLD-NATIVE-V0-BASELINE/1`
**Protocol status:** Proposed for independent provenance approval.
**Standing:** documentation only. **No official EvaluationRun has occurred**
under this protocol, and no results are claimed or implied.

## Purpose

Record what Draft PR #1 at exact frozen head
`d5ef318468b6c35df3c14c1c5f72beb1191baf29` actually does under frozen
synthetic inputs — the pre-H1 foundation comparator required by Accepted
ADR-0024.

## Claims boundary

This protocol produces a **foundation/provenance comparator only**. It makes
no claim that accepted ADR contracts are implemented by the baseline; it is
not certification; it is not a production-readiness claim; and it carries no
implementation or completion authority. The EvaluationResult cannot authorize
implementation and cannot approve completion.

## Hypotheses (preregistered, falsifiable)

- H-B1: every valid case reaches its preregistered expected disposition.
- H-B2: refusal/failure cases do not produce false completion.
- H-B3: repeated runs produce deterministic event-type agreement.

## Arms

**ARM-BASE/1** — the only arm executable in the future run. Repository
`d5ef318468b6c35df3c14c1c5f72beb1191baf29`, components exactly as shipped
(single-approval plan path, direct lesson write, deterministic fixture
adapter). Two repetitions per baseline case. Independent variable: none —
this is a single-arm comparator.

**ARM-RECON/1** — declared here for future identity only; **ARM-RECON/1 is
declared but not executable** until PR #1 reconciliation lands under its own
authorization. It will later measure the accepted Specification approval
boundary, deterministic PlanAssessment, Consulting independence, canonical
completion vocabulary, MemoryCandidate-only emission, the provider
pre-materialization response cap, and Evaluation Spine integration.

**No pooled or like-for-like aggregate may ever combine the two arms.** The
only sanctioned comparison is per-case disposition deltas explicitly labelled
baseline-versus-reconciled.

## Fixture inclusion and exclusion

Included: exactly the assets listed in
[fixture-registry.yaml](fixture-registry.yaml) at the registry hash frozen in
the RunManifest. Excluded: everything else — unregistered files, live
providers, network resources, regenerated goldens, and any asset whose
registry state is `TestSupportOnly` or `Rejected`. Network mode is **deny**.
Zero automatic retries. Pinned Unix environment class. Hard per-case wall
and invocation budgets copied from the baseline phase-runner limits, plus a
hard whole-run ceiling. No manual intervention outside scripted case steps.

## Run validity versus result — two separate dispositions

`EvaluationRunDisposition` (was the run *executed correctly*?):

- `FinalizedValid`
- `InvalidatedByProtocolDeviation`
- `AbortedBeforeEvidenceComplete`

`EvaluationResultDisposition` (what did a valid run *show*?):

- `Pass`
- `Fail`
- `Inconclusive`
- `NotAssessable`

Rules:

1. Missing blocking manifest identity, unauthorized fixture substitution, or
   material uncontrolled contamination may invalidate a run.
2. False completion, false acceptance, unsafe effects, failed recovery, or
   other product failures **do not invalidate a correctly executed run** —
   product failure is a result, not a run defect.
3. A valid run that exposes a product failure produces a failing
   `EvaluationResult` and remains durable evidence.
4. A rerun is never performed merely to erase a negative result.
5. A rerun requires a recorded reason: protocol defect, contaminated
   environment, or a new protocol/fixture version.
6. Downstream authorization requires an assessed acceptable result, but the
   `EvaluationResult` itself grants nothing — authorization is a separate
   authenticated decision.

## Execution-source identity

The deterministic fixture adapter is an execution source, not a model:

```text
ExecutionSourceIdentity:
  kind: FixtureAdapter
  adapter_id: wepld-fixture-adapter
  adapter_version: as recorded from the frozen head at run time
  source_or_binary_hash: required at run time
  cassette_corpus_hash: required at run time
  deterministic: true

ModelIdentityEvidence:
  status: NotApplicable
  reason: No model or provider serves the deterministic fixture-adapter arm.
```

Model/provider identity evidence is reserved for a future arm that actually
invokes a model or provider.

## EvaluationCases (id / version = PH1-Cnn/1)

All twelve are `PRE_RECONCILIATION_BASELINE_CASE` class; the "reusable"
column states whether the same case definition is re-runnable
post-reconciliation (as a `POST_RECONCILIATION_ACCEPTANCE_CASE` re-instance
under new goldens). EV-S1–EV-S5 comparative arms are future cases,
preregistered only at slice authorization.

| Case | Name | Fixtures | Initial state / fault seams | Expected evidence | Expected terminal state | Prohibited false success | Security invariant | Metrics | Reusable |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| PH1-C01/1 | low-risk Build Feature success | fx-notes-cli, fx-mission-add-version-flag, cc-corpus-* | clean instantiated repo; no seams | spec, plan, approvals, gate logs, snapshot, diff, proposal ref | completion proposed → accepted; base branch untouched | acceptance without evidence | no-merge; worktree isolation | terminal correctness, criteria coverage, evidence completeness | yes |
| PH1-C02/1 | specification clarification required | cc corpus with open questions | clean; no seams | clarification stop recorded; no mission created | needs-clarification stop | silent advance to execution | specification stop condition | terminal correctness | yes |
| PH1-C03/1 | invalid or incomplete plan | cc corpus with oversized/uncovered plan | mission created; planning runs | recorded rejection; zero task rows; draft state | plan rejected | plan materialized or executed | payload/coverage bounds | terminal correctness | yes |
| PH1-C04/1 | unsafe filesystem edit | cc corpus with escape/symlink/hard-link edits | build phase reaches edit application | batch rejection or contained failure; filesystem sweep clean | failed/contained attempt | any write outside the worktree | no-follow capability boundary | unsafe-path escape count (must be 0) | yes |
| PH1-C05/1 | worker failure | fx-notes-cli; hermes failure-mode seam (recorded) | worker dies/mutes mid-phase | failed/uncertain attempt fact; worktree destroyed; no snapshot | failed attempt, mission not promoted | snapshot or promotion of failed work | failed-attempt containment | terminal correctness, recovery metrics | yes |
| PH1-C06/1 | fixture/provider failure | missing cassette key (recorded seam) | reasoning requested, no cassette | honest rejection recorded | rejected: no reasoning provider | fabricated specification or plan | fail-closed gateway | terminal correctness | yes |
| PH1-C07/1 | verification-gate failure | cc corpus + failing gate command | gates run and fail | gate logs with failure; executed-not-accepted report | not accepted | completion proposal on red gates | evidence-bound completion | false-completion rate | yes |
| PH1-C08/1 | completion returned | PH1-C01 state to proposal, approve=false | reviewer returns | return fact with real reviewer; no ref; no lesson | returned (terminal) | reported as completed | return semantics | false-acceptance rate | yes |
| PH1-C09/1 | uncertain acceptance effect | PH1-C01 state; **crash fault seam during the acceptance effect** | the acceptance effect begins and its outcome becomes unknown mid-flight | recorded intent, attempted effect, probe results, reconciliation decision | acceptance-pending/uncertain, then exactly-once completion on reconciliation | duplicate or phantom acceptance | intent-before-effect + probe reconciliation | uncertain-effect reconciliation accuracy | yes |
| PH1-C10/1 | pre-existing proposal-ref conflict | PH1-C01 state; **a conflicting ref planted before the CAS update** | conflicting ref exists ahead of time; no crash seam | deterministic conflict detection; refusal fact; proof the conflicting ref is byte-unchanged | acceptance deferred/uncertain by refusal | overwrite or force-replacement | compare-and-swap refusal; **no uncertain-effect probe path is used** | proposal-ref conflict handling | yes |
| PH1-C11/1 | restart recovery | PH1-C09-style seams + store reopen | crash, process restart, retry | idempotent completion; exactly one acceptance; valid hash chain | recovered, single acceptance | duplicate acceptance | ledger replay + idempotency | restart recovery success | yes |
| PH1-C12/1 | baseline direct-memory-write nonconformance | PH1-C01 accepted mission | acceptance triggers lesson path | the direct lesson write is observed and recorded **as a known baseline nonconformance** with respect to the accepted ADR-0020 candidate-only scope | accepted mission + recorded nonconformance observation | describing the baseline as conformant with the candidate-only scope | visibility of the direct-write path | provenance completeness | no — superseded by the candidate-emission case post-reconciliation |

**Cases PH1-C09 and PH1-C10 are mechanically distinct.** C09 injects a
crash/fault seam *during* the acceptance effect so the initial effect state
is genuinely unknown; probes and reconciliation determine the durable
outcome, and its evidence must include intent, attempted effect, probe, and
reconciliation records. C10 plants a *known* conflicting ref *before* the
compare-and-swap update; the conflict is detected deterministically, the
overwrite is refused, no probe path executes, and its evidence proves the
pre-existing ref remains unchanged. They share neither setup nor expected
evidence.

## RunManifest field contract (future run)

Blocking (run may not start when missing): protocol id/version/hash; case
id/version/hash; arm id/version/hash; repository commit; fixture-registry
hash; per-fixture identities and hashes; cassette-corpus identities and
hashes; configuration hashes; contracts version; Rust toolchain;
dependency-lock hash; `ExecutionSourceIdentity`; model/provider identity
evidence **or** its explicit NotApplicable reason; time/request budgets;
retry policy (zero); network mode (deny); operator identity; assessor
identity (assigned before assessment); active fault seams per case.

Optional/recordable-as-Unknown: OS kernel patch level; architecture
micro-revision; token budgets (fixture adapter consumes none); ContextPack
lineage for phases that compile no packs; random-seed applicability
(deterministic run → recorded `NotApplicable`).

Explicitly NotApplicable (with reason recorded, never silently blank):
model/provider identity for ARM-BASE; random seed.

Also recorded: invocation ids; environment deviations (list, possibly
empty); ContextPack lineage where packs exist.

## Metrics

As defined in the qualification package: terminal correctness,
false-completion rate (must be 0), false-acceptance rate (must be 0),
criteria coverage, evidence completeness, provenance completeness (must be
1.0 over blocking fields), unauthorized effect count (0), unsafe-path escape
count (0), uncertain-effect reconciliation accuracy, restart recovery
success, proposal-ref conflict handling, deterministic replay agreement
across the two repetitions, operator interventions (0 outside scripts),
elapsed time, adapter invocations, budget consumption. Denominator: executed
case-repetitions unless a metric names its own.

## ProtocolDeviation rules

Any retry, unscripted manual step, environment drift, fixture substitution,
budget breach, or unexpected seam activation is recorded with cause, scope,
contamination impact, authorizing decision if any, and an explicit
inclusion/exclusion disposition. Deviations never disappear. A deviation
affecting frozen inputs moves the affected case-run toward
`InvalidatedByProtocolDeviation` — product failures never do (see the
disposition rules above).

## Rejection criteria and rollback consequence

The **run** is invalidated only by the run-validity rules above. The
**protocol or fixtures** are rejected/superseded when: blocking identity
cannot be satisfied; registry hashes fail independent re-verification; or a
protocol defect is found — producing a new protocol/fixture version with a
supersession link. A failing or inconclusive `EvaluationResult` leaves the
baseline exactly where it is: PR #1 stays a retained experimental baseline,
downstream packages may not cite the result as acceptable, and nothing is
erased — all records are append-only.

## Assessor independence

Per [assessor-policy.md](assessor-policy.md): the runner cannot be the sole
assessor; hashes freeze before assessment; missing evidence is never a pass;
the deterministic fixture-adapter arm needs no provider independence;
role/session/context independence is mandatory wherever model-assisted
review participates. Assessor assignment: PendingFounderDecision.
