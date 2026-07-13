# M1 — GitHub Spec Kit Integration Design

**Status:** DESIGN — awaiting founder approval. **No implementation until approved** (per the adoption directive). · **Date:** 2026-07-13 · **Author:** Lead Software Engineer
**Scope:** adopt GitHub Spec Kit ([github/spec-kit](https://github.com/github/spec-kit)) as WePLD's official specification workflow. Implementation decision, not an architectural redesign. Architecture v2 and Chronicle remain frozen; the Charter governs.

---

## 1. What Spec Kit is (study summary)

Spec Kit is GitHub's **Spec-Driven Development (SDD)** toolkit: a methodology + templates + a `specify` CLI that makes the *specification* the primary artifact and code its generated expression. Seven phases:

| Phase | Command | Produces | Nature |
| --- | --- | --- | --- |
| Constitution | `/speckit.constitution` | `.specify/memory/constitution.md` | project governing principles |
| Specify | `/speckit.specify` | `specs/NNN-feature/spec.md` | WHAT/WHY: user stories, functional reqs, acceptance checklist, `[NEEDS CLARIFICATION]` markers |
| Clarify | `/speckit.clarify` | refines `spec.md` in place (+ Clarifications section) | ambiguity resolution |
| Plan | `/speckit.plan` | `plan.md` + `research.md`, `data-model.md`, `contracts/`, `quickstart.md` | HOW: tech strategy, architecture |
| Tasks | `/speckit.tasks` | `tasks.md` | ordered, dependency-aware, `[P]` parallel markers |
| Analyze | `/speckit.analyze` | consistency/coverage report | cross-artifact validation |
| **Implement** | `/speckit.implement` | **application source code** (unmediated AI agent) | **execution** |

Layout: `.specify/` (memory, scripts, templates, extensions, presets) + numbered `specs/NNN-feature/`. Extension points: extensions (new commands), presets (template overrides), bundles (role sets), and a template-resolution stack.

---

## 2. The governing boundary (the one decision everything follows from)

> **Spec Kit owns specification. WePLD owns execution. Spec Kit's `/implement` phase is NOT adopted — the WePLD Runtime replaces it.**

Spec Kit's phases 1–6 (constitution → … → analyze) are *specification* work and are adopted. Phase 7 (`/implement`) drives an **unmediated AI agent to write code directly** — this fundamentally conflicts with WePLD's first invariant: *the Runtime is the only execution authority; Hermes is the engineering runtime; every effect is evidence-gated and ledger-recorded.* We do not modify WePLD to accommodate `/implement`; we **decline to adopt that one phase** and substitute the WePLD Runtime. This is not a Gap Note — it is the natural division of labor and the entire value proposition ("WePLD owns execution"). Everything below implements the target flow:

```
Developer → Specification → Plan → Tasks   [Spec Kit methodology, WePLD-hosted]
          → Mission Conversion             [new adapter: wepld-spec]
          → Runtime → Hermes → WWP → Workers → Evidence → Ledger → Chronicle   [unchanged]
```

Also **not adopted** (out of scope / superseded): `taskstoissues` (GitHub issues — WePLD's integration gateway handles external channels later), the bash agent-scripts (WePLD's Hermes *is* the agent), and `bundles/presets/extensions` marketplace machinery (WePLD has its own registry/skills lifecycle).

---

## 3. Architecture mapping (Spec Kit → WePLD, reusing existing primitives)

| Spec Kit concept | WePLD home | Mechanism |
| --- | --- | --- |
| `constitution.md` (project principles) | **CAS artifact** + Context Assembly T-tier | Per-*project* governance (the user's project). Distinct from the WePLD Charter (which governs WePLD itself). Informs gate selection (e.g. test-first → require a test gate) |
| `spec.md` (WHAT/WHY) | **CAS artifact** (`kind: specification`), versioned | The new first-class artifact. Human-editable working copy in repo `specs/NNN/`; immutable hashed snapshot in CAS on each validate/convert |
| `[NEEDS CLARIFICATION]` markers | **`wepld spec validate`** gate | Deterministic completeness check; conversion refuses a spec with unresolved markers |
| `plan.md` + design docs | **CAS artifacts** (`kind: spec_plan`, `spec_design`) | Generated via the WePLD gateway (Hermes reasoning) or imported; fed to Context Assembly for the builder |
| `tasks.md` (ordered, `[P]`) | **WePLD `PlanDoc.tasks[]`** | Mission Conversion maps Spec Kit tasks → WePLD `TaskSpec` (id, title, satisfies, deps, parallel) |
| Numbered `specs/NNN-feature/` | repo working dir + spec aggregate | Spec Kit's convention kept for the editable source; CAS holds the immutable versions |
| `/specify`,`/plan`,`/tasks` (reasoning) | **Provider Gateway** (existing) | Reasoning goes through the Runtime's gateway → captured, replayable, evidence-backed (strictly better than Spec Kit's untracked agent calls) |
| `/implement` | **Mission → Runtime → Hermes** | Replaced entirely; the whole point |
| Spec/plan/tasks lifecycle | **Ledger + Chronicle** | No second history system; spec revisions are ledger facts + CAS snapshots |

---

## 4. Reused components (no new subsystems)

- **CAS** — stores every spec/plan/tasks/constitution version, content-addressed and immutable. *This is the specification store.*
- **Ledger** — records the spec lifecycle as durable facts (spec created/revised, plan/tasks generated, mission derived). Hash-chained, verifiable, replayable.
- **Provider Gateway** — spec plan/tasks *generation* is reasoning; it flows through the existing gateway (fixture-first; deterministic under cassettes; real via the M1-A OpenAI-compatible adapter).
- **Context Assembly** (v0, coming in M1) — spec + constitution + design docs become high-tier context packs for the builder. The spec is *the* context source a real project needs.
- **Mission Runtime** — spec-derived missions run through the unchanged M0 lifecycle (create → plan → approve → run → gates → completion → accept).
- **Chronicle** — replays and forensically analyzes the full lifecycle from specification through execution.

---

## 5. Required adapter: `wepld-spec` (the Mission Conversion layer)

A new **pure-transform crate** `crates/spec` (`wepld-spec`). Depends on `contracts` only (+ a small markdown/front-matter parser). **Owns no state, executes nothing, calls no gateway, writes no ledger** — it transforms data. The CLI/runtime orchestrate the ledger/gateway side.

```
wepld-spec
├── model.rs        SpecDoc, PlanArtifact, TasksArtifact, Constitution (parsed structures)
├── parse.rs        markdown + machine-readable anchors → typed structures
├── validate.rs     completeness gate: no [NEEDS CLARIFICATION]; every acceptance criterion
│                   has a verify; every task maps to ≥1 criterion; required sections present
├── convert.rs      THE Mission Conversion: (SpecDoc, plan, tasks, constitution)
│                   → { MissionBrief, PlanDoc, SpecProvenance, ValidationReport }  — pure, deterministic
└── templates.rs    WePLD-flavored spec/plan/tasks/constitution templates (adopted from Spec Kit,
                    with structured anchors so parsing is robust, not freeform-markdown-brittle)
```

**Conversion contract (`convert`)** — deterministic function, same inputs (same hashes) → same Mission:

| Spec source | → Mission field |
| --- | --- |
| feature name + number | `title` |
| spec `## Goal`/`## Outcome` | `outcome` |
| spec functional reqs + acceptance checklist | `acceptance_criteria[]` (each with a `verify` → gate) |
| plan repo/scope | `scope` (repo, base_branch, paths, forbidden_paths) |
| plan testing strategy + constitution test-first | `gates_required` + `gate_commands` |
| tasks.md (ordered, `[P]`) | `PlanDoc.tasks[]` (id, title, satisfies, sequence, parallel) |
| spec/plan/design doc hashes | `SpecProvenance` (recorded on `MissionCreated`) |

---

## 6. Repository changes

- New crate `crates/spec` (added to the workspace).
- New CLI commands (§7) in `crates/cli`.
- Committed WePLD spec templates under `fixtures/spec-templates/` (constitution, spec, plan, tasks) — adopted from Spec Kit with structured anchors.
- A `specs/` directory convention in *user* repos (created by `wepld spec init`), mirroring Spec Kit's `specs/NNN-feature/`.
- New golden trace `spec-to-mission` (the full spec → conversion → execution flow) under `fixtures/golden/`.
- `ureq` already added (M1-A). One small parser dep (e.g. `pulldown-cmark` or hand-rolled anchor parser — TBD, minimal).

Existing crates, contracts (except the additive changes in §9), and golden traces are untouched.

---

## 7. CLI changes (all additive; existing mission commands unchanged)

| Command | Behavior | Reasoning? |
| --- | --- | --- |
| `wepld spec init` | scaffold `specs/` + templates + a project `constitution.md` in the current repo | none |
| `wepld spec new <name>` | create `specs/NNN-name/spec.md` from template (developer fills WHAT/WHY) | none |
| `wepld spec validate <spec>` | deterministic completeness gate (§5 validate) | none |
| `wepld spec plan <spec>` | generate `plan.md` + design docs via the gateway; snapshot to CAS; record lifecycle | gateway (fixture-first) |
| `wepld spec tasks <spec>` | generate `tasks.md` via the gateway; snapshot to CAS; record | gateway (fixture-first) |
| `wepld mission create --from-spec <spec>` | **Mission Conversion**: validate → convert → capture spec artifacts to CAS → record spec lineage → create the Mission (existing command path) with spec provenance → PlanProposed derived from tasks | none (reasoning already happened) |

`wepld mission new -f <brief.json>` and all other M0/M1 mission commands are **unchanged**. `--from-spec` is a new, separate creation path.

---

## 8. Runtime changes (minimal, additive)

- New orchestration method `create_mission_from_spec(spec_refs)`: captures the spec/plan/tasks artifacts to CAS, records spec lineage, builds `MissionBrief` + `PlanDoc` via `wepld-spec::convert`, then creates the Mission and records a **spec-derived `PlanProposed`** (provenance: spec hashes; no planner *phase* — the plan came from the spec, analogous to a deterministic/zero-reasoning path per IADR-0007 §1). Human approval gate preserved.
- New method `reason_for_spec(spec_id, intent, schema)`: calls the **existing gateway** for spec plan/tasks generation, stores the output artifact to CAS, records a `BrainInvoked` + artifact under the spec aggregate. Reuses the gateway; adds no execution path.
- No change to the mission state machine, phase engine, WWP, or Hermes. Spec-derived missions execute through the identical M0 lifecycle.

---

## 9. Chronicle integration + the additive contract decision (needs approval)

The founder's Chronicle capture list — *spec version, spec revision, plan revision, task generation, mission creation, mission completion, spec updates, mission forks, mission comparisons* — maps to durable ledger facts. Mission creation/completion/forks/comparisons **already exist**. The new capture needs are the spec-lifecycle events.

**Decision point (choose one; recommendation below):**

- **Option A — Reuse-only (zero vocabulary change).** Model specs as `ArtifactRecorded{kind: specification|spec_plan|spec_tasks, version, supersedes}` under a spec correlation, and `MissionCreated{source: spec_kit, spec_refs}`. **Pros:** no contract change at all; maximally conservative. **Cons:** Chronicle's spec lens must filter artifact-kinds rather than event-types; weaker semantic clarity for forensics/cinema.
- **Option B — Minimal additive extension (recommended).** Add one aggregate type `Specification` and a small event set to vocabulary **rev 3** (additive, exactly as Chronicle added rev 2): `SpecificationCreated · SpecificationRevised · SpecPlanGenerated · SpecTasksGenerated · SpecValidated · MissionDerivedFromSpec`. **Pros:** first-class spec traceability the directive explicitly wants; clean Chronicle spec lens; every revision individually addressable. **Cons:** an additive contract extension (version bump + lock-test update).

**Recommendation: Option B.** It is the *sanctioned additive path* the architecture defines for exactly this (v2-07 additive minors; Chronicle set the precedent at rev 2), and it delivers the traceability the directive requires. It is **not** a freeze violation — it adds vocabulary without changing any existing meaning. But because it touches a frozen contract, it is **explicitly flagged for your approval** here rather than done unilaterally.

Chronicle then gains (read-side, later per its own roadmap) a **Specification lens**: spec revisions, plan/tasks generation, and the derive-to-mission edge become nodes/edges in the causal index — so a mission's forensics can trace back to *which spec revision* produced it, and a spec's impact cone shows *which missions* it spawned.

---

## 10. Determinism & replay

- `convert` is a pure function; identical spec/plan/tasks hashes → identical Mission. Guaranteed replayable.
- Spec plan/tasks *generation* reasoning is deterministic under cassettes (fixture-first, IADR-0002); the record/replay harness (M1-A) already covers it.
- Spec artifacts are content-addressed in CAS; the Mission records their exact hashes; Chronicle reconstructs the whole chain.
- New golden trace `spec-to-mission` pins the normative event sequence (spec create → validate → plan → tasks → mission derive → run → accept).

---

## 11. Migration strategy

Purely additive, greenfield. Existing `mission new -f` missions, all M0/M1 data, and existing golden traces are unaffected — spec-derived missions are a new parallel path. No data migration. If Option B is approved, the vocabulary bump to rev 3 is additive (old ledgers remain valid; the lock test gains the new variants). Teams already using Spec Kit proper can `wepld spec` import externally-authored artifacts (ingested as versioned CAS artifacts with recorded provenance).

---

## 12. Compatibility risks

| Risk | Mitigation |
| --- | --- |
| Spec Kit is an evolving external project | We adopt *concepts + templates*, not their code/scripts; their releases can't break us |
| Freeform markdown parsing is brittle | WePLD templates use **structured anchors** (front-matter / fenced machine-readable blocks) so conversion parses reliably, not by guessing markdown |
| Additive vocabulary/aggregate change (Option B) | Flagged for approval; additive-only; lock-test updated; precedent = Chronicle rev 2 |
| Users expecting Spec Kit's `/implement` code-gen | Documented boundary: WePLD executes via the Runtime instead; `wepld spec` output flows into a Mission, not a code generator |
| Constitution vs. Charter confusion | Explicit doc: *project* constitution (user's rules, per-repo) ≠ WePLD *Charter* (governs WePLD); the project constitution never overrides WePLD's architecture |

---

## 13. Gap Notes

**None required.** The only structural tension — Spec Kit's `/implement` vs. WePLD's execution authority — is resolved by *not adopting that phase* and substituting the Runtime (an adapter/boundary decision, not a WePLD modification). All other Spec Kit mechanisms map onto existing WePLD primitives (CAS, Ledger, Gateway, Context Assembly, Runtime, Chronicle) via the `wepld-spec` adapter. Option B's vocabulary extension is additive-by-design, not a conflict.

---

## 14. Proposed implementation slices (only after approval)

1. **Spec-A:** `wepld-spec` crate — templates, parse, validate, `convert` (pure, unit-tested). `wepld spec init/new/validate`. No runtime/ledger yet.
2. **Spec-B:** `wepld mission create --from-spec` — Mission Conversion wired to the runtime (CAS capture + spec lineage + spec-derived PlanProposed). Golden `spec-to-mission`. Runs through the unchanged execution lifecycle.
3. **Spec-C:** `wepld spec plan/tasks` — gateway-backed generation (fixture-first + record mode), captured to CAS/ledger.
4. **Spec-D (if Option B approved):** vocabulary rev 3 + `Specification` aggregate + Chronicle spec-lineage edges.
5. **(Later, M3+ Studio):** the Specification workspace UI (v0-generated) — *not* a chat window; the engineering specification interface.

Each slice: small commits, passing tests, fmt + clippy clean, evidence-based verification, fixture-first determinism.

---

## 15. Open decisions for founder approval

1. **Approve the overall design** (Spec Kit phases 1–6 adopted; `/implement` replaced by the Runtime).
2. **Chronicle capture: Option A (reuse-only, zero contract change) or Option B (recommended: additive vocabulary rev 3 + `Specification` aggregate).**
3. **Spec plan/tasks generation authority:** WePLD-hosted via the gateway (recommended — captured & replayable), and/or import of externally-authored Spec Kit artifacts (both supported; confirm priority).
4. **Slice order** (Spec-A first, then B) — confirm.

No code will be written until you approve. Protect the Runtime; Spec Kit improves the specification workflow; WePLD remains the Operating System for Autonomous Engineering.
