# v2-14 — Engineering Forensics

Cinema shows **what** happened. Forensics answers **why** — as ranked, evidence-linked causal chains, never as oracle verdicts (ADR-0014). This is the capability that should become synonymous with WePLD: the post-mortem, mechanized.

## 1. The causal graph

Derived index `causal_edges` over ledger + artifacts (rebuildable). Node = any ledger entry, artifact, decision, pack section, or gate result. Edges:

| Edge | Derivation | Confidence |
| --- | --- | --- |
| `caused` | ledger `causation_ref` | 1.0 |
| `decided` | decision → plan/tasks it scoped | 1.0 |
| `informed` | pack section → brain invocation (from pack manifest) | 1.0 |
| `produced` | invocation/phase → artifact | 1.0 |
| `evidenced` | artifact → gate result | 1.0 |
| `gated` | gate result → state transition | 1.0 |
| `revised` / `carried` | branch lineage links (v2-15) | 1.0 |
| `implicated` | heuristic: omitted pack file ∩ later failure site; retry without hypothesis change; contradicted uncertainty | < 1.0, rule named in `derived_by` |

Two traversals power everything: **cause cone** (ancestors of a node) and **impact cone** (descendants, weighted by cost/artifacts/gates touched).

## 2. The question bench

Forensics is question-driven. Each template compiles to graph operations + evidence hydration:

| Question | Compilation |
| --- | --- |
| Why did this mission fail? | RCA from the terminal failure fact (§4) |
| Why did it succeed? | dominator analysis on the acceptance node: which decisions/artifacts appear on *every* path to completion — the load-bearing set |
| Where did execution diverge (A vs B)? | common prefix by lens; first differing frame per track; report per-lens divergence points |
| Which decision had the largest impact? | max impact-cone weight over decision nodes |
| Which assumption proved incorrect? | `uncertainties` recorded in phase summaries (v2-07 §2) later contradicted by gate/review evidence → `implicated` edges |
| Which evidence was missing? | acceptance criteria whose `verify` never bound to a passing check; gates passed with stale inputs |
| Which context affected this decision? | `informed` in-edges of the invocation that drafted the packet — down to the pack *section* |
| Which worker/provider produced this? | direct joins on attempts / `brain_invocations` |
| Which policy prevented execution? | envelope denial entries on the cause path |
| Who introduced this regression? | V2: bisection — run the failing check across workspace snapshot refs (ADR-0013); compute-costed and budgeted like any gate |
| What was the earliest meaningful cause? | §4 |

MVP ships the interactive **causal walk**: click "Why?" on anything → its cause cone as a navigable chain, each hop opening evidence in Cinema. V1 adds the automated report generator for the templates above.

## 3. Missing context — the signature move

Because Context Assembly records selection manifests *including omissions with reasons* (v2-04), Forensics can answer the question no competitor can: **"what did the system not know when it went wrong?"** Rule `omitted_then_implicated`: for a failure at file F / symbol S, find earlier packs whose manifest lists F (or its import neighbors) as *considered-but-omitted* (budget, ranking, or policy), and emit an `implicated` edge from that omission to the failure (confidence scaled by proximity and repetition). This turns an invisible failure class — the model literally couldn't see the constraint — into a first-class, fixable finding ("raise T2 budget for this path", "pin this file for tasks touching S"), which feeds Intelligence (v2-16).

## 4. The Root Cause engine

**Input:** a failure fact (gate fail, mission FAILED, budget exhaustion, human RETURNED). **Output:** an RCA report — a primary causal chain plus scored alternates, every step a claim with evidence refs.

Algorithm (deterministic core; model narration only phrases it):

1. **Collect** the cause cone of the failure fact (bounded depth, typically < 200 nodes at MVP scale).
2. **Classify** each ancestor into actionable classes: `decision`, `context_omission`, `plan_defect` (task under-specified vs. its criteria), `envelope_denial`, `schema_failure`, `retry_futility` (retries with unchanged hypothesis), `evidence_gap` (criterion without binding verify), `mechanical` (mere consequence).
3. **Score meaningfulness** per non-mechanical node — three tests, any may fire:
   - *Divergence test:* does a reference class exist (sibling attempts, sibling tasks, prior similar missions) where this node differs and the outcome differed? (weight by class size)
   - *Information test:* was information demonstrably absent that a later fact shows was needed? (manifest omissions, missing knowledge records)
   - *First-defect test:* is this the earliest node where a retroactively-evaluable invariant already fails (e.g., the plan's task spec never covered AC2)?
4. **Select** the earliest node with class ≠ mechanical and score above threshold → primary hypothesis; report the chain from it to the failure; list runners-up with scores and *why they scored lower*.
5. **Render** under the claims discipline: every hop `{cause, class, confidence, evidence[], counterfactual_note}`; a hop the graph can't evidence renders as unverified narration — visually demoted, exactly like any Messenger prose (v2-10 §5).

Worked shape (the canonical chain):

~~~text
MissionFailed (gate:test ✗ on AC3)                      [fact]
 ↑ gated      GateEvaluated test ✗ — 3 failures in queue_worker.rs   [fact]
 ↑ produced   refactor in att_b3 moved retry logic out of the worker  [fact]
 ↑ decided    plan v1 task T2 assumed retries were idempotent         [plan_defect — first-defect test fires: AC3 mentions at-most-once]
 ↑ informed   plan-phase pack omitted docs/queue-semantics.md (budget) [context_omission — information test fires, conf 0.8]
 ─ PRIMARY HYPOTHESIS: earliest meaningful cause = context omission at planning
   remedy candidates: pin docs/** for planning packs on paths src/queue/** ; add lesson
   runner-up: plan_defect independent of omission (score 0.44 vs 0.71 — doc existed and stated the constraint)
~~~

The user clicks any hop and Cinema seeks to that moment with the evidence drawer open — Forensics and Cinema are one navigation space (the provenance breadcrumb records the jump).

## 5. Dismissal and honesty loop

A user can mark a heuristic edge or hypothesis *dismissed with reason*; the dismissal is recorded (V1: `AnnotationRecorded`) and reported in rule-quality stats (v2-16). Forensics is allowed to be wrong; it is not allowed to be unaccountable.

## 6. Classification of capabilities

MVP: causal index (deterministic edges), causal walk, manifest-omission rule. V1: full RCA reports, question bench templates, dismissal loop, divergence finder. V2: bisection, cross-mission RCA, reference-class libraries. Future: fleet-scale pattern forensics.
