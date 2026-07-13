# WePLD Engineering Specification System — Final Design

**Status:** DESIGN v4 (APPROVED) — incorporates all founder decisions incl. the platform elevation (Recipes as a first-class capability with SDK + Marketplace; true typed Specification Graph; Engineering/Organization Packs + Expert Profiles). Foundation implementation (Spec-A) authorized and begun. · **Date:** 2026-07-13
**Frozen:** Architecture v2, Chronicle engine, WWP, Hermes Runtime, single execution authority. This work is **additive** — new domains built on the frozen substrate; nothing frozen is modified (only sanctioned additive contract extensions, flagged).

> **Founding principle.** Code is an implementation artifact. **Engineering Specifications are the engineering truth.** Chronicle records history; the Runtime governs execution; Hermes performs work; Knowledge captures learning; Skills preserve capability. WePLD evolves Spec Kit's philosophy into a permanent Engineering Specification System — Spec Kit is the starting point, not the destination. WePLD is an Engineering Operating System, not an AI IDE.

---

## PART I — Principles & boundary

**Spec Kit adopted as methodology; `/implement` not adopted — the Runtime replaces it.** Phases 1–6 (constitution → specify → clarify → plan → tasks → analyze) become WePLD specification work. Execution is always Mission → Runtime → Hermes → WWP → Evidence → Ledger → Chronicle.

**The Specification is a canonical object, not a markdown file.** Markdown (Spec Kit's `spec.md`/`plan.md`/`tasks.md`) is *one serialization*. The canonical `SpecificationDocument` is a structured, typed object stored in CAS; markdown is a bidirectional representation (`parse ⇄ render`) for human editing and Spec Kit compatibility. **Every WePLD component interacts with the object, never with markdown.**

**Frozen vs. additive.** "Frozen" = the internals and contracts of the existing systems are unchanged. "Additive" = new domains (Specification, Recipes) + sanctioned additive contract extensions (vocabulary rev 3, `Specification` aggregate) + additive Chronicle derivation rules. This is the same additive path Chronicle used (rev 2). No frozen system is redesigned.

**Boundary rules (CI-enforced):** Specifications never execute code, never bypass the Runtime, never talk to Hermes directly, never bypass the Ledger, never own execution state. The Runtime is the single execution *and persistence* authority.

**Specification is now a core domain** — a first-class peer of Mission, Runtime, Chronicle, Knowledge, Skill, Decision, ADR, Artifact, Worker, Provider, Context. Not an adapter, not a plugin. Part of the language of WePLD.

---

## PART II — The Specification domain

### II.1 Canonical object model (`crates/contracts/src/specification.rs`, contracts → 0.5.0)

```rust
Specification {                       // living identity (state row)
  spec_id, number, slug, status: SpecStatus, author, current_version, created_at, updated_at
}
SpecStatus = Draft|Clarifying|Researching|Planned|Tasked|Active|Revising|Superseded|Archived

SpecVersion {                         // immutable snapshot (append-only)
  spec_id, version, revision, timestamp, author,
  document_hash,                      // CAS: canonical SpecificationDocument (JSON) — the truth
  render_hashes { markdown, plan_md, tasks_md, … },  // derived representations (also CAS)
  plan_hash?, tasks_hash?, research_hash?, data_model_hash?, contracts_hashes[], constitution_hash?,
  supersedes: Option<u32>, reason, quality: SpecQuality
}

SpecificationDocument {               // canonical structured content (serialization-independent)
  overview, user_stories[], functional_requirements[], acceptance_criteria[]  (each: text + verify),
  non_functional[], edge_cases[], constraints[], dependencies[], required_skills[],
  success_metrics[], clarifications[]  (question→answer), open_questions[]  (NEEDS CLARIFICATION)
}

SpecLink { spec_id, kind: Mission|Adr|Knowledge|Skill|Spec|Context|Recipe, target_ref, relation }
SpecFinding { spec_id, version, class: FindingClass, severity, evidence_refs[], disposition }
SpecRevisionProposal { proposal_id, spec_id, from_version, proposed_by, trigger, diff_ref,
                       rationale, evidence_refs[], status: Proposed|Approved|Rejected }
SpecQuality { completeness, consistency, ambiguity, coverage, risk, maintainability,
              review_status, verification_status, missing_sections[], evidence_refs[] }
FindingClass = MissingAcceptanceCriteria|HiddenAssumption|ArchitectureContradiction|DependencyConflict
  |MissingRollback|MissingBenchmark|MissingSecurity|MissingMigration|MissingTesting
  |MissingDeployment|MissingPerformance|MissingObservability|MissingRecovery|MissingOperational
```

### II.2 Living lifecycle (never finishes; supersession, never deletion)

```
Draft → Clarifying → Researching → Planned → Tasked → Active
  ↑ (derived missions execute; evidence & verification accrue)          │
  └── Revising ← revision approved ← (reverse-sync proposal | manual) ──┘
Active/Revising → Superseded (a later spec replaces it) → Archived (retained, replayable)
```

### II.3 Versioning & storage (reuse CAS + Ledger; no second history)

Canonical document + all renders/design-docs → **CAS** (immutable, hashed). Domain rows (`specifications`, `spec_versions`, `spec_links`, `spec_findings`, `spec_revision_proposals`) → ledger state tables, written **only** through the Runtime's single-writer transaction with the matching rev-3 event. Correlation id = `spec_id` → each spec has its own hash-chained, independently-replayable history.

---

## PART III — The Engineering Graph (a true typed graph)

A **true typed graph** — a first-class query surface with an explicit node/edge type schema, a derived typed-edge store, and a traversal/query API. It is *derived* (rebuildable from ledger link facts, never a second source of truth) yet *first-class* (a real graph model, not a flat link list).

**Node types** (each a stable ref into its owning domain): `Specification · SpecVersion · Mission · Task · Knowledge · Skill · ADR · Context · Recipe · Pack · ExpertProfile · Artifact · Decision`.

**Edge types** (typed, directed, each carrying provenance + confidence): `depends_on` (spec→spec), `creates` (recipe→spec, spec→mission), `derives` (spec_version→mission), `produces` (mission→knowledge/artifact/adr), `references` (spec→adr/knowledge), `requires` (spec→skill, mission→context, recipe→pack), `provides` (pack→template/recipe/skill, expert→recipe/skill), `reverse_syncs` (mission→spec_proposal), `supersedes` (spec→spec), `governs` (org_pack→spec/recipe).

**Storage:** a derived `graph_edges` table (from, to, type, provenance_ref, confidence), rebuilt deterministically from ledger link facts (`SpecLinked`, `MissionDerivedFromSpec`, produce/require facts) + Chronicle causal edges. **Query API** in `wepld-specification::graph`: typed traversal, impact cone ("what does revising this spec affect?"), dependency resolution (multi-spec, pack requirements), and shortest-provenance path — the substrate for Diff (Part IV), Recipes (Part VI), and Packs. Rebuildable at any time; `DROP TABLE` + re-derive is the recovery model (per ADR-0011's projection discipline). This is the engineering knowledge graph the platform reasons over.

---

## PART IV — Lifecycle intelligence (Intelligence · Quality · Reverse-Sync · Diff · Replay · DNA)

**Specification Intelligence** (`wepld spec review`) — Hermes (Gateway reasoning, Runtime-orchestrated) emits evidence-based `SpecFinding`s across the fourteen `FindingClass`es. Every finding cites the spec section (or its absence). Never invents, never hallucinates, never auto-modifies. Recorded (`SpecReviewed`). Deterministic under cassettes.

**Specification Quality** — per-version scores (completeness, consistency, ambiguity, coverage, risk, maintainability, review/verification status), each citing its evidence. `validate` computes deterministic scores; `review` adds reasoning findings. A spec below policy threshold cannot be converted to a mission.

**Reverse Synchronization** — during execution Hermes may find reality diverged (architecture/API/DB/ADR/dependency/perf/security assumptions changed). It raises a `SpecRevisionProposal` (evidence + proposed diff) routed through the **existing decision/approval machinery**. It **never** edits the spec; founder approval is mandatory. Approve → `ReviseSpec` → new version + `SpecRevisionResolved`.

**Specification Diff** (`wepld spec compare A B`) — pure `diff(v_a, v_b) → SpecDiff`: changed requirements/architecture/assumptions/tasks + affected missions/ADRs/knowledge/skills/runtime-history (from the graph + Chronicle edges).

**Specification Replay** — Chronicle replays the full spec lifecycle (creation → clarification → research → planning → task/mission generation → execution → evidence → verification → revisions → comparisons → forks) as a session on a **Specification lens**. Reuses Chronicle's frozen engine.

**Specification DNA** — Chronicle insight classes discover cross-spec patterns ("this team omits observability," "this project underestimates testing"). Evidence-based, **advisory only**, promoted by a human into lessons that improve future templates/reviews. Never auto-modifies.

---

## PART V — Engineering Memory & Skills

**Engineering Memory** — approved specifications become **Knowledge**; **Context Assembly** includes relevant specs in future mission packs; future specs, mission planning, and skill discovery reference them via the graph. Specifications are a primary memory type in the compounding-memory thesis.

**Skills integration** — a `SpecificationDocument.required_skills[]` (e.g. Rust, Tokio, PostgreSQL, Security, FHIR, Networking) flows spec → derived mission → worker profile skill pins → Hermes loads the pinned skill packages at mission execution, via the **existing v2-09 skill-resolution machinery**. *Sequencing:* the Skill *registry/packages* are a later milestone; until then `required_skills` is recorded, surfaced, and graph-linked but not auto-loaded (no conflict — a data field ahead of its consumer).

---

## PART VI — Engineering Recipes: a first-class platform capability

**Recipes are executable engineering knowledge** — reusable engineering workflows that orchestrate Specifications, Missions, Runtime, Hermes, Chronicle, Knowledge, and Skills. They are a **first-class platform capability and a plugin ecosystem** (SDK + Marketplace), not merely a UX veneer. **The primary UX is one-click Recipes; users never learn internal workflows.**

### VI.1 The Recipe object & SDK

A Recipe is a versioned, signed **package** (reusing the v2-15 plugin lifecycle: identity, semver, integrity hash, signature, capabilities, trust tier, revocation). Its manifest is a declarative pipeline — **not executable code that touches the Runtime**; the Runtime interprets it:

```
Recipe {
  id, version, publisher, trust_tier,
  action: "build-feature" | "fix-bug" | …,        // the one-click identity
  inputs[],                                        // minimal user prompts (typed)
  template_ref,                                    // Specification Template to instantiate
  pipeline: [ specify, clarify?, research?, plan, tasks, convert, execute, verify ],  // declarative steps
  required_skills[], required_packs[],             // resolved before execution
  gates[], autonomy_default, capability_requests[] // policy-mediated, never ambient
}
```

**Recipe SDK** = the authoring contract: the manifest schema + template/step vocabulary + a validator + a local test harness (fixture-first: a recipe is proven against recorded cassettes before publication). Third parties author recipes against the SDK; the Runtime is the only interpreter. A recipe **owns no execution and no state** — it declares intent; the Runtime + Hermes perform, Chronicle records.

### VI.2 Recipe Marketplace (plugin ecosystem)

Recipes are distributed through the **registry/marketplace** (v2-15 stance): local + organization registries first; signed community publication later, with publisher verification, permission transparency, evaluations, ratings that cannot override policy, and rapid revocation. Installing a recipe is a policy-governed transaction; a recipe can request no capability it wasn't granted. **No recipe bypasses the Runtime, Ledger, or policy** — the marketplace distributes *declarative engineering knowledge*, never execution authority.

### VI.3 Initial recipes (one-click)

✨ Build Feature · 🐞 Fix Bug · ♻ Refactor Module · ⚡ Optimize Performance · 🔒 Security Audit · 📚 Understand Repository · 🧪 Generate Tests · 🚀 Prepare Release · 📦 Upgrade Dependencies · 🏗 Architecture Review. Each instantiates a template → spec → (clarify/research/plan/tasks) → Mission → execution → evidence, with the workflow hidden. Recipes are graph nodes (`recipe → creates → spec`, `recipe → requires → pack`).

## PART VI-B — Engineering Packs, Organization Packs, Expert Profiles

All three are **signed, versioned packages** in the same registry/marketplace lifecycle (v2-15) — curated, distributable engineering assets. Graph nodes with `provides`/`governs`/`requires` edges.

| Pack type | Contents | Purpose |
| --- | --- | --- |
| **Engineering Pack** | curated templates + recipes + skills + knowledge for a domain/stack (e.g. "Rust Backend", "FHIR Healthcare", "React Frontend") | one install = a ready engineering capability set |
| **Organization Pack** | an org's constitution, conventions, approved templates/recipes/skills, policy, gate standards | the organization's engineering standards as a versioned, `governs`-linked, distributable artifact (enterprise governance) |
| **Expert Profile** | a curated bundle capturing an expert's approach — preferred recipes, review criteria, spec patterns, skills, brain-profile preferences | "apply an expert's engineering judgment" — the human-expertise-marketplace thesis (v2-29), as installable, policy-bounded assets |

These reuse the frozen registry/skills/knowledge machinery; they add **no** execution path. An Organization Pack's `governs` edges let policy/Context Assembly apply org standards to every spec/recipe/mission in that org.

## PART VI-C — Quick Actions & Project Bootstrap

**Quick Actions** — the Studio Home surface for Recipes (one-click: Build Feature, Understand Project, Fix Bug, Review Architecture, Generate ADR, Upgrade Dependencies, Security/Performance Audit, Generate Tests, Release Build). One click; the Runtime orchestrates everything. Studio, M3+ (reserved).

**Project Bootstrap** (strategic horizon) — a Recipe: Import Repository → Understand Repository (analysis missions) → Generate Engineering Specifications → Architecture → ADRs → Knowledge → Skills → Mission Backlog → Ready. Turns an existing codebase into a fully-specified WePLD project. Designed here; implemented Future.

**Specification Templates** — reusable structured spec skeletons per project type (REST API, CLI, Rust Library, Desktop App, DB Migration, Microservice, FHIR Capability, SDK, AI Feature, Infrastructure Change), pre-populating the canonical document's sections + default gates/skills; versioned assets (CAS + registry), shippable inside Engineering Packs. `wepld spec new --template rest-api`.

---

## PART VII — Substrate integration

**Contracts (rev 3, additive; approved Option B extended):** `Specification` aggregate; `specification.rs` (Part II); vocabulary events — `SpecificationCreated · SpecificationRevised · SpecClarified · SpecResearched · SpecPlanGenerated · SpecTasksGenerated · SpecValidated · SpecReviewed · MissionDerivedFromSpec · SpecRevisionProposed · SpecRevisionResolved · SpecLinked · SpecStatusChanged`. Lock test extended; old ledgers valid.

**Chronicle (engine frozen; additive derivation rules only):** a Specification lens, the new graph/causal edges, a spec-diff comparison facet, and a DNA insight class — all new readers/rules over the same substrate, exactly as Chronicle was designed to grow (v2-11). No frozen Chronicle contract changes.

**Runtime (minimal, additive):** a `spec` orchestration module — `create_spec, clarify_spec, research_spec, generate_spec_plan, generate_spec_tasks, validate_spec, review_spec, revise_spec, create_mission_from_spec, propose_spec_revision, resolve_spec_revision`. Each records its rev-3 event + spec state in one transaction; reasoning ones call the existing Gateway. **No change to the mission state machine, phase engine, Hermes, or WWP.**

**CLI (additive; existing mission commands unchanged):** `wepld spec init|new|clarify|validate|plan|tasks|review|replay|compare` and `wepld mission create --from-spec`. Recipes later: `wepld recipe run <recipe>`. `wepld mission new -f` remains the lower-level primitive conversion builds on.

**Mission Conversion (deterministic, multi-spec):** pure `convert(specs[], plan, tasks, constitution) → { MissionBrief, PlanDoc, SpecProvenance, ValidationReport }`; same hashes → same Mission. A mission may derive from multiple specs; it records all source refs and the spec→mission graph edges. Execution runs the unchanged M0 lifecycle.

---

## PART VIII — Long-term repository vision (guidance only; no restructuring now)

Target domain crates as the platform matures (the founder's domain language): `runtime · chronicle · specification · mission · knowledge · skills · recipes · providers · workspace · ledger · studio` (+ current `contracts · artifacts · wwp · hermes · cli`). **Architectural guidance only — existing code is not restructured in this work.** Naming for the new subsystem: dir `crates/specification/`, package `wepld-specification` (the `wepld-` prefix retained solely for workspace package-namespace consistency — the strong architectural reason the directive allows); `crates/recipes/` → `wepld-recipes`.

---

## PART IX — Determinism · Migration · Compatibility · Gap Notes

- **Determinism/replay:** canonical objects + conversion + diff are pure; reasoning is deterministic under cassettes (record/replay, M1-A); versions immutable in CAS; Chronicle reconstructs the full chain. Golden `spec-to-mission` pins the normative sequence.
- **Migration:** additive/greenfield. Existing missions, data, and goldens unaffected. Spec-first (via Recipes) becomes the standard workflow; `mission new -f` stays as the primitive. Rev-3 is additive.
- **Compatibility:** Spec Kit evolves — we adopt concepts+templates, not code; brittle markdown → canonical object + structured render/parse; additive contract change → flagged & lock-tested; `/implement` expectations → documented boundary; project constitution ≠ WePLD Charter.
- **Gap Notes:** **none.** `/implement` resolved by substitution; every mechanism maps onto existing primitives (CAS, Ledger, Gateway, Context, Chronicle, Runtime, skill-resolution, decision machinery); rev-3 and Chronicle rules are additive-by-design; Skills registry & Bootstrap are *sequencing*, not conflicts.

---

## PART X — Implementation phases (only after go-ahead)

| Phase | Delivers |
| --- | --- |
| **Spec-A** | `wepld-specification` crate: canonical model, markdown parse⇄render, templates v0, `validate`, `convert`, `quality`, `diff` (pure, unit-tested). Contracts rev-3 + `Specification` aggregate. `wepld spec init/new/validate`. |
| **Spec-B** | Runtime spec orchestration (create/revise/status) + `wepld mission create --from-spec` (conversion, CAS capture, spec↔mission graph links, `MissionDerivedFromSpec`). Golden `spec-to-mission`. Runs the unchanged lifecycle. |
| **Spec-C** | Gateway-backed `clarify/research/plan/tasks/review` (fixture-first + record mode); Spec Quality + Intelligence findings. |
| **Spec-D** | Reverse synchronization (proposal → decision → approved revision) + `spec compare` diff. |
| **Spec-E** | Specification Graph API; Chronicle Specification lens + `spec replay`; DNA insight class; Knowledge/Context-Assembly integration; Skills required-skills flow. |
| **Recipe-A** | `wepld-recipes` domain + Recipe object/manifest + core recipes (Build Feature, Fix Bug) orchestrating the full pipeline through the Runtime; `wepld recipe run`. |
| **Recipe-B** | Recipe SDK (manifest schema + validator + fixture test harness) + local/org registry install lifecycle. |
| **Ecosystem** | Engineering Packs, Organization Packs, Expert Profiles (registry packages, `provides`/`governs` edges); Marketplace distribution (signed community publication) — reuses v2-15 lifecycle. |
| **Graph** | typed `graph_edges` store + query/traversal API (impact cone, dependency resolution). |
| **Studio / Bootstrap** | Specification workspace + Quick Actions (M3+); Project Bootstrap (Future). |

Each phase: small commits, passing tests, fmt + clippy clean, fixture-first determinism, evidence-based verification, no drift.

## PART XI — Status: APPROVED; foundation authorized

The founder approved the design and elevated the vision (this v4). **Confirmed:** crate names `wepld-specification` (`crates/specification`) and `wepld-recipes` (`crates/recipes`) with the `wepld-` namespace prefix; the rev-3 event set; the phase order. **Spec-A (the stable foundation — contracts rev-3 + the pure `wepld-specification` canonical model, validate, quality) is authorized and begun.** The higher ecosystem layers (Recipe SDK/Marketplace, Packs, Expert Profiles, Bootstrap) remain design-only until their phases; they are additive and build on this foundation without churn.

Protect the Runtime. Protect the Architecture. The Engineering Specification is the product; Recipes are executable engineering knowledge.
