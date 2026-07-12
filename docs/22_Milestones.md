# 22 — Milestones

## Milestone philosophy

Milestones are proof points, not calendar promises. They prove a product capability under realistic failure and policy conditions. Dates should be assigned only after Phase 0 estimates, team capacity, platform-spike results, and prioritization are approved.

| Milestone | Proof | Definition of done |
| --- | --- | --- |
| M0 — Architecture Gate | organization agrees what it is building | 30 documents complete and consistent; open decisions recorded; V1 scope approved; no production implementation started |
| M1 — Durable Local Core | the product survives normal failure | Core restarts with correct mission/task state; command/event contracts and local auth verified; projection rebuild passes |
| M2 — Safe Task Effect | a worker can make a controlled change | isolated worktree; capability-mediated tool action; artifact/evidence trail; deny-path tested |
| M3 — First Mission Loop | the organization produces a reviewable bounded outcome | planner → builder → QA/reviewer loop with required test evidence and decision queue |
| M4 — Governed Autonomy | unattended work remains controllable | leases/retries/budget/stop conditions; Manual/Limited/Full mode tests; hard gates cannot be bypassed |
| M5 — Organizational Memory | past work improves future work safely | sourced decisions/findings/lessons retrievable with classification and freshness |
| M6 — Studio Beta | leaders can operate rather than inspect logs | Mission Control, Executive, Timeline, Architecture, review IDE, accessibility baseline |
| M7 — Ecosystem/Enterprise Readiness | extension and remote growth preserve trust | signed package lifecycle, selected integrations, policy/retention baseline, release operations |

## Entry and exit evidence

### M0 — Architecture Gate

Entry: this planning package exists. Exit: product/engineering/security leaders resolve the strategic decision list; architecture traceability is checked; V1 acceptance fixtures and ownership are approved; implementation authorization is explicit.

### M1 and M2 — Foundation proof

Entry: approved contracts and a supported platform test plan. Exit: simulated crash, conflicting command, expired lease, denied capability, and corrupted/absent projection scenarios are demonstrably recoverable or safely visible. A task action can be traced from intent through evidence without a manual forensic reconstruction.

### M3 — Vertical-slice proof

Entry: M1/M2 evidence. Exit: one user completes a low-risk mission on one local repository using one supported worker adapter and brain profile; no primary worktree mutation occurs; build/test/review evidence and user decision are visible in Timeline.

### M4 and M5 — Trust proof

Entry: reliable slice and evaluation baseline. Exit: parallel dependencies, retries, cancellation, budget limits, security gates, and knowledge retrieval are exercised by acceptance scenarios. Full autonomy remains limited to declared envelopes; a security/policy attempt is blocked with a clear reason.

### M6 and M7 — Product/operational proof

Entry: operational data is trustworthy. Exit: usability/accessibility research confirms executive and engineer workflows; supported packages can be installed, rolled back, and revoked; beta releases can migrate/rollback safely; any enterprise/remote feature meets the same contract tests as local use.

## Milestone metrics

| Metric | M3 baseline | M4/M6 target principle |
| --- | --- | --- |
| State recovery | all scenario fixtures reconstruct correctly | regression suite remains 100% |
| Action provenance | 100% of actions traced to task/policy/evidence | 100% in release telemetry |
| Gate truthfulness | no completion with missing mandatory evidence | audited continuously |
| Unsafe-effect prevention | deny-path scenarios pass | adversarial test suite expands |
| Mission usability | user can explain mission state and next decision | usability research improves task comprehension |
| Provider portability | supported profile swap preserves contract behavior | expanded provider benchmark coverage |

## Milestone governance

Advancement requires evidence review by product, Core, quality, and security owners. A visual demo or a successful happy-path run is insufficient. Material scope changes require a revised milestone acceptance matrix, risk review, and potentially a new ADR.

See also: [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), [21_Project_Backlog.md](21_Project_Backlog.md), and [28_Release_Strategy.md](28_Release_Strategy.md).

