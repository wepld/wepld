# WePLD Evaluation and Research Programme

**Standing:** planning only. This programme extends the ADR-0024 Evaluation
Spine and the document-34/37 discipline to the whole strategy: preregistered
arms, frozen manifests, recorded budgets, blind scoring where feasible, and
honest terminal states. **No strategy claim survives without its experiment.**

## Strategy-level comparison arms

| Arm | Configuration |
| --- | --- |
| EV-S1 | manual developer workflow (baseline) |
| EV-S2 | unstructured coding agent |
| EV-S3 | specification-driven agent (no independent review) |
| EV-S4 | WePLD without Consulting |
| EV-S5 | WePLD with Consulting |
| EV-S6 | WePLD with Committee (doc-37 arms govern the interior) |
| EV-S7 | WePLD with Project DNA/Constitution context |
| EV-S8 | WePLD with Truth Graph-backed context |
| EV-S9 | WePLD with a certified Skill vs without |
| EV-S10 | local-only Hermes |
| EV-S11 | optional Letta-backed profiles (A–E memory arms) |
| EV-S12 | governed memory (MemoryCandidate + Judge) vs bounded/self-editing memory |
| EV-S13 | mixed external providers under policy |
| EV-S14 | Imported Change Assurance entry (external-agent change + WePLD verification/Consulting) vs native delivery vs raw external-agent output |
| EV-S15 | recovery drills (crash matrix, uncertain effects, orphan states) |
| EV-S16 | Studio surface utility and decision burden |
| EV-S17 | Production Truth Loop linkage fidelity |
| EV-S18 | Adoption Gateway: manual onboarding vs no import vs deterministic import only vs deterministic import plus reviewed model-assisted extraction — measuring false rules, missed commands, unsafe imports, onboarding time, and user corrections |

Budget discipline follows document 37: same hard budget class per comparison,
actual spends recorded, absolute plus cost-normalized reporting — structural
differences are results, not noise.

## Metrics

Accepted-task success · false completion · requirements coverage ·
regressions · security defects · unsupported claims · human corrections ·
intervention count · rework · wall time · tokens · cost · context volume ·
evidence completeness · recovery success · privacy violations · plan churn ·
repeated-task improvement · decision burden · long-term maintenance burden · interruptions per phase · delayed critical decisions · bundled-decision quality · unsafe suppression rate (must be zero) · completion time per unit of human attention.

## Per-capability rejection, disable, or defer criteria (representative)

| Capability | Reject/disable/defer when |
| --- | --- |
| Committee | doc-37 rejection criteria fire (cost without material finding value; imitation convergence; worthless minority reports; churn without quality) |
| Semantic retrieval | fails ablation against exact/LSP/structural on the same budget |
| Automatic routing | increases failure or correction rate vs static profiles |
| A Skill | does not generalize beyond its fixtures — not promoted |
| Shadow Mode | false-warning rate erodes trust (measured threshold preregistered) |
| Mission Simulator / Decision Lab | predictions mislead (calibration below preregistered floor) — remains Research, never product |
| Autonomy level N | corrections or trust regress at that level — automatic rollback |
| A Studio surface | less useful than an existing integration (EV-S16) — dropped |
| Letta adapter | EV-S11 shows no repeated-task improvement or worse unsupported-claim/privacy metrics — defer or reject; the decision is Stage 7+ because arms D/E need Stage-6b foundations |
| Adoption Gateway | EV-S18 shows model-assisted extraction adds false rules or unsafe imports beyond deterministic import — restrict to deterministic importers |
| HumanAttentionBudget | any nonzero unsafe suppression, or bundling degrades decision quality — bundling disabled, invariants retained |
| Governed memory | fails EV-S12 against simpler bounded memory — simplify |
| Truth Graph surfaces | no reduction in corrections/unsupported claims — internal index only |
| Production Truth Loop | linkage unreliable for a product class — reports "unlinked", scope narrowed |
| Economics Engine | estimates worse than naive baselines — advisory display suspended |
| Any deterministic inspector | false-positive cost exceeds detection value per class — disabled per class |

Every registry row inherits this rule: **no rejection path, no admission.**

## Research register

Semantic retrieval · context-poisoning detection heuristics · Mission
Simulator/Counterfactual Decision Lab calibration · DeepLearn distillation
quality · Letta continual-learning arm · external coding-agent workers ·
diversity-informed composition (post EC-A7/EC-A8 only) · cross-repository
impact prediction (Program Mode precursor).

## Security and privacy programme (strategy-wide threat model)

Threats carried from the PR #1/#2/#3 models and extended portfolio-wide:
prompt injection through any pack; malicious repository content; model
collusion and correlated failure; model impersonation and provider
substitution (bounded identity assurance, honest unknowns); credential
leakage; excessive context; unauthorized egress; unsafe filesystem effects;
Git manipulation; migration damage; dependency compromise; poisoned memory,
Skills, or Committee reports; evidence fabrication; approval spoofing; cost
amplification; infinite retries; self-promotion; privilege persistence (leases
expire); stale authorization (approvals expire); **autonomy escalation and
evidence gaming** — an autonomy level raised without sufficient evidence, a
model manipulating metrics to earn autonomy, stale autonomy surviving a
policy/model/project change, autonomy leaking between projects, failure to
roll back after regressions, a capability lease exceeding the admitted
autonomy tier, user-interface approval spoofing, or a bundled decision hiding
an authorization request — countered by scoped and expiring autonomy,
independent evidence, automatic downgrade, no self-promotion, full audit,
human override, and the never-suppress categories; cross-project
contamination;
tenant isolation; recovery manipulation; audit deletion (append-only ledger);
production telemetry leakage (redaction + authorization). Every stage's
security acceptance in the roadmap cites this register; every new capability
must name which rows it touches before its gate.
