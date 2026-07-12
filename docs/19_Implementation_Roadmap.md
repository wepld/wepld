# 19 — Implementation Roadmap

## Roadmap principle

Build a trustworthy narrow vertical slice before platform breadth. WePLD’s greatest early risk is a convincing interface over an unreliable control plane. Each phase therefore exits only with measurable evidence, not a collection of partially connected features.

## Phase 0 — Architecture approval and discovery

**Objective:** convert this plan into an approved engineering contract without writing production features.

- Review the 30 architecture documents, resolve strategic open questions, and record initial ADRs.
- Produce threat-model review, action/risk taxonomy, core event/command schemas, and representative mission acceptance fixtures.
- Validate desktop sandbox feasibility on macOS, Windows, and Linux through non-product technical spikes.
- Evaluate one local-capable and one hosted brain adapter using non-sensitive fixtures.

**Exit:** scope lock for V1, approved data/policy contract, initial threat model, architecture sign-off, and an explicitly prioritized backlog.

## Phase 1 — Local control-plane foundation

**Objective:** establish durable local state and safe seams before autonomous work.

- Core Daemon lifecycle, local authenticated RPC, operational database/event ledger, artifact store, and projection framework.
- Mission/task domain model, policy decision point/enforcement points, capability tokens, budget accounting, and audit timeline.
- Git/worktree manager and sandbox posture abstraction; Studio shell with read-only Mission Control/Timestamp projections.
- Contract test harnesses for worker, brain, tool, and plugin ports.

**Exit:** a daemon restart can reconstruct mission state; an unauthorized action is denied at enforcement; Studio does not own mutable workflow state.

## Phase 2 — The single-project vertical slice

**Objective:** complete one bounded engineering mission locally and safely.

- One project, one user, one local repository, Planner → Builder → QA/Reviewer workflow.
- One Hermes-compatible worker adapter, one local-capable brain profile, and one approved hosted profile where policy permits.
- Isolated worktree effects, action evidence, basic build/test gate, decision queue, Messenger in Studio, and Timeline.
- Manual and Limited Approval modes; full provenance from mission brief to completion proposal.

**Exit:** a user can submit a small mission; see/approve its plan; receive a reviewable isolated change with test evidence; and recover from a killed worker/daemon without unexplained state.

## Phase 3 — Operational depth and engineering loops

**Objective:** make autonomous loops reliable and observable.

- Task DAG parallelism, leases/retries/recovery, resource scheduler, cancellation, budget controls, and failure classification.
- Quality/security/review/benchmark gates, coverage baseline, secret/dependency checks, and security-finding lifecycle.
- Knowledge ingestion/retrieval with citations, skill resolver/validation, health vector, richer Mission Control, and async Messenger reports.
- Full Autonomous mode within declared envelopes and explicit hard gates.

**Exit:** bounded parallel missions produce evidence-backed results under cost/resource limits; policy and health states are explainable; no direct worker-to-user route exists.

## Phase 4 — Studio breadth and ecosystem foundations

**Objective:** turn the vertical slice into a credible engineering studio.

- Executive, Architecture, Knowledge, and review-oriented IDE workspaces.
- Local registry, signed core/org skills/plugins, package lifecycle, additional worker/brain profiles, and integration framework.
- Accessibility, offline/degraded UX, document/report exports, release workflow, telemetry exporter, and recovery drills.

**Exit:** the product supports a team’s local engineering workflow with consistent evidence, UI accessibility, and package governance.

## Phase 5 — Collaboration, enterprise, and scale-out

**Objective:** add distributed capability only after local semantics are proven.

- Opt-in sync, organization identity/policy, remote workers, shared knowledge, organization audit retention, and selected channel adapters.
- Enterprise deployment, signed marketplace, remote artifact storage, compliance controls, and scale-out orchestration evaluation.

**Exit:** multi-user/remote execution preserves the same authority, audit, isolation, and policy semantics as local V1.

## Sequencing dependencies

Policy/event/artifact/worktree foundations precede workers; workers precede autonomous modes; evidence gates precede full autonomy; registry trust precedes marketplace; local recovery precedes sync; and role-based workspaces precede broad editor parity. No phase may skip its exit criteria to satisfy a visual demo.

## Team shape

Initial work needs a product/CTO owner, desktop/Studio engineer, Core/infra engineer, security engineer, quality/evaluation engineer, and UX researcher/designer. Specialists can be fractional in early phases, but security, test/evaluation, and product ownership cannot be deferred to a final hardening sprint.

See also: [21_Project_Backlog.md](21_Project_Backlog.md), [22_Milestones.md](22_Milestones.md), [23_Technology_Evaluation.md](23_Technology_Evaluation.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).

