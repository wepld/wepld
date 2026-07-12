# 21 — Project Backlog

## Backlog strategy

This is a product-level, dependency-ordered backlog—not a request to begin implementation. Items are prioritized by whether they reduce the core trust risk: can WePLD safely and durably coordinate a bounded engineering mission? A polished feature that cannot prove its effect ranks below a control-plane foundation.

## Priority 0 — Architecture and governance gate

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Architecture ratification | approved V1 contract | all 30 documents reviewed; open decisions assigned; ADR process adopted | this package |
| Mission/action taxonomy | consistent policy vocabulary | mission/task/action/risk/gate glossary; representative fixtures | vision, security |
| Threat-model validation | tested security assumptions | asset/boundary review; platform sandbox posture matrix | security architecture |
| Contract design | stable seams before adapters | versioned command/event/worker/brain/tool schemas reviewed | data/event/API docs |
| Evaluation charter | measurable model/worker quality | role-specific fixtures, baselines, red-team cases, score ownership | testing strategy |

## Priority 1 — Local control-plane foundation

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Core lifecycle | one authoritative local daemon | crash/restart recovery of mission projection; authenticated local connection | contracts |
| Event ledger/projections | explainable durable state | Mission and Timeline projection rebuild from events | data/event model |
| Mission/plan/task domain | bounded workflow control | invalid transitions rejected; DAG validation and revisions work | core lifecycle |
| Policy/capability enforcement | no ambient worker authority | denied action blocked at both command and tool boundary | taxonomy, security |
| Artifact/Git worktree manager | reversible source changes | every attempt gets isolated workspace and artifact hashes | policy |
| Studio foundation | evidence-first read surface | live vs stale state clear; no client-side workflow mutation | Core query/subscription |

## Priority 2 — Minimum engineering organization

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Worker adapter | Hermes-compatible execution lease | register, lease, heartbeat, cancel, artifacts, recovery tests | worker contract |
| Brain Gateway | replaceable reasoning profile | local and hosted candidate profiles pass fixture suite | policy, evaluation |
| Planner → Builder → QA/Reviewer | one complete bounded loop | plan, isolated change, build/test result, independent review | workers, worktree |
| Tool mediation | controlled local effects | action request/approval/evidence/idempotency trace | capability enforcement |
| Decision queue / Studio Messenger | executive decisions without blocking unrelated work | evidence-linked packet and authoritative resolution | mission domain |
| Manual/Limited modes | safe early autonomy | approval rules prevent gated action while routine flows continue | policy |

## Priority 3 — Reliability and organizational memory

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Scheduler/recovery | bounded parallel task execution | lease expiry, retry hypothesis, resource quotas, cancellation | reliable event ledger |
| Quality and security gates | evidence-based completion | build/test/review/scan results gate state transitions | worker/tool contracts |
| Knowledge graph | cited reusable memory | decision/claim/artifact links and authorized retrieval | artifacts, retention |
| Mission Control | operational visibility | health vector, resource/cost, blocked graph, decision alert evidence | telemetry/projections |
| Skill registry | reusable validated expertise | pinned resolution, candidate validation, revocation behavior | package model |
| Full Autonomous envelope | controlled unattended execution | hard gates remain non-bypassable; budget/stop tests pass | reliability + gates |

## Priority 4 — Studio and ecosystem readiness

| Epic | Outcome | Acceptance criteria | Dependencies |
| --- | --- | --- | --- |
| Executive/Architecture/Timeline workspaces | studio-level operational UX | users navigate decision → evidence → impact seamlessly | projections/knowledge |
| Review-oriented IDE | source-level review in context | isolated diffs, terminal attribution, Git provenance | worktree manager |
| Package registry | signed local/org capability lifecycle | install, rollback, revoke and dependency impact tested | skills/plugins |
| Channel framework | selected Messenger integrations | identity, consent, delivery/retry, disclosure rules validated | Messenger security |
| Release operations | dependable beta delivery | signed build, migration/rollback, telemetry, support runbook | release strategy |

## Priority 5 — Collaboration and enterprise

Remote workers, opt-in synchronization, organization policy servers, shared knowledge, enterprise identity/retention, a signed marketplace, and advanced IDE parity begin only after local recovery, policy, and evidence semantics are demonstrated.

## Backlog hygiene

Each backlog item must name customer outcome, bounded-context owner, data/event impact, policy/risk class, test/evaluation evidence, dependency, and definition of done. “Add model X,” “make it autonomous,” and “build integrations” are not backlog items without those constraints. Items that expand autonomy, external data transfer, or plugin scope require a security review before they can enter a milestone.

See also: [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [22_Milestones.md](22_Milestones.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), and [30_ARCHITECTURE_SUMMARY.md](30_ARCHITECTURE_SUMMARY.md).

