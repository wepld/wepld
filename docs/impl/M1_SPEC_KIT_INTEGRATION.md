# M1 — Engineering Specification Subsystem (Spec Kit adoption)

**Status:** DESIGN v2 — incorporates the founder's approval + extensions. **Awaiting approval; no implementation until approved.** · **Date:** 2026-07-13
**Founder decisions folded in:** Spec Kit adopted as the *official* specification workflow; Option B approved and extended; specifications become **first-class, permanent, living engineering objects**; a **dedicated permanent Specification subsystem** (not a temporary adapter); reverse synchronization, specification intelligence, quality, diff, replay, and DNA. Architecture v2, Chronicle, Hermes, and WWP remain **frozen** — this subsystem is **additive**, reusing the frozen substrate without modifying it.

---

## 0. Founding principle (the frame for every decision)

> Code is an implementation artifact. **The Engineering Specification is the product.** Hermes executes specifications; Chronicle remembers them; Knowledge grows from them; Skills evolve from them. WePLD is an Engineering Operating System, not an AI IDE.

Everything below serves that: a specification is a durable, versioned, replayable, reviewable, comparable object that lives for the life of the project and drives — but never performs — execution.

## 1. Governing boundary (unchanged, reaffirmed)

**Spec Kit owns specification. WePLD owns execution.** Spec Kit phases 1–6 (constitution → specify → clarify → plan → tasks → analyze) are adopted as methodology; **phase 7 (`/implement`) is not adopted — the Runtime replaces it.** The flow:

```
Specify → Clarify → Research → Plan → Tasks → [Mission Conversion]
  → Runtime → Hermes → WWP → Execution → Evidence → Verification → Ledger → Chronicle
  → (Revision → Replay → Comparison → Future Specification)   ← the spec keeps living
```

**Hard boundary rules (enforced in code + CI):** specifications never execute code, never bypass the Runtime, never communicate with Hermes directly, never bypass the Ledger, never own execution state. The Runtime is the single execution *and persistence* authority; the Specification subsystem is a domain that flows through it.

---

## 2. The Specification Subsystem (permanent bounded context)

A new **permanent** bounded context — a peer of Mission, Orchestration, Quality, Knowledge — with its own domain model and lifecycle, reusing the frozen substrate:

| It owns (domain) | It reuses (frozen substrate) | It never does |
| --- | --- | --- |
| Specification objects, versions, links, status, quality, findings, diffs, revision proposals | **CAS** (content), **Ledger** (lifecycle facts + state tables), **Gateway** (reasoning), **Context Assembly** (feeds specs to missions), **Chronicle** (replay/diff/DNA), **Runtime** (the single writer + execution authority) | execute code, hold `ledger::Tx`, call Hermes/WWP, own mission/execution state |

**Crate:** directory `crates/specification/`, package **`wepld-specification`**. *Naming note:* the meaningful, permanent name is `specification` (per the directive); the `wepld-` package prefix is retained only for workspace namespace consistency (all crates are `wepld-*`; cargo needs collision-free package names). This is the "strong architectural reason" the directive allows. **Confirm or override in §Open Decisions.**

**Layering:** `wepld-specification` is a pure domain library (depends on `contracts` + a structured-markdown parser). It performs no I/O to the ledger, no gateway calls, no execution. The **Runtime** hosts spec *orchestration* (commands, gateway reasoning, ledger writes) by composing `wepld-specification` — exactly as it composes the Mission domain. This preserves the single-writer boundary the M0 release gate verified: the Runtime remains the only `ledger::Tx` holder.

---

## 3. First-class Specification domain model

Contracts (`crates/contracts/src/specification.rs`, additive, contracts → 0.5.0):

```rust
Specification {                       // the living object (state table row)
  spec_id: String,                    // "spec_01J…" stable identity
  number: u32,                        // NNN sequential (Spec Kit convention)
  slug: String,                       // "user-authentication"
  status: SpecStatus,
  author: String,                     // authoring principal
  current_version: u32,
  created_at, updated_at,
}
SpecStatus = Draft | Clarifying | Planned | Tasked | Active | Revising | Superseded | Archived

SpecVersion {                         // immutable snapshot (append-only)
  spec_id, version, revision, timestamp, author,
  spec_hash,                          // CAS: spec.md content
  plan_hash?, tasks_hash?, research_hash?, data_model_hash?, contracts_hashes[], constitution_hash?,
  supersedes: Option<u32>, reason: String,
  quality: SpecQuality,               // evidence-based (§7)
}
SpecLink { spec_id, kind: Mission|Adr|Knowledge|Skill|Spec, target_ref, relation }
SpecFinding {                         // Specification Intelligence output (§6)
  spec_id, version, class: FindingClass, severity, evidence_refs[], disposition
}
SpecRevisionProposal {                // Reverse Sync (§8) — proposal only
  proposal_id, spec_id, from_version, proposed_by, trigger, diff_ref, rationale,
  evidence_refs[], status: Proposed|Approved|Rejected
}
SpecQuality {                         // §7 — every score cites its evidence
  completeness, consistency, ambiguity_count, risk, coverage,
  missing_sections[], review_status, verification_status, evidence_refs[]
}
FindingClass = MissingAcceptanceCriteria | HiddenAssumption | ArchitectureContradiction
  | MissingRollback | MissingBenchmark | MissingSecurity | MissingMigration
  | MissingTesting | MissingDeployment | MissingPerformance | MissingObservability | MissingRecovery
```

**Links are first-class** (`spec_links` table): a spec ↔ missions derived from it, ADRs it aligns to, knowledge records it sources, skills it invokes, and other specs it depends on (multi-spec). Chronicle links are implicit via ledger correlation.

---

## 4. Living lifecycle & state machine

A specification never "ends." Status transitions (each a recorded fact; supersession, never deletion):

```
Draft ──specify──▶ Clarifying ──clarify──▶ Planned ──plan──▶ Tasked ──tasks──▶ Active
   ▲                                                                              │
   │                                       derived missions execute ─────────────┘
   └────── Revising ◀── revision approved ◀── (reverse-sync proposal | manual edit)
Active/Revising ──supersede──▶ Superseded    (a later spec replaces it)
any ──archive──▶ Archived                     (retained, replayable, never deleted)
```

Chronicle replays the entire lifecycle (§10). Every transition is a ledger fact under the spec's correlation.

---

## 5. Versioning & storage (reuse CAS + Ledger; no second history)

- **Content** (spec.md, plan.md, tasks.md, research.md, data-model.md, contracts/, constitution.md) → **CAS**, immutable, content-addressed. A revision is a *new* `SpecVersion` referencing new hashes and `supersedes` the prior — the working copy in the repo `specs/NNN/` is mutable and human-editable; CAS holds the frozen versions.
- **Domain rows** (`specifications`, `spec_versions`, `spec_links`, `spec_findings`, `spec_revision_proposals`) → the ledger DB state tables, written **only** through the Runtime's single-writer transaction alongside the corresponding spec event.
- **Lifecycle facts** → the ledger (rev-3 events, §14). Correlation id = `spec_id`, so a spec has its own hash-chained history independent of any mission, and Chronicle can replay it standalone.

Immutability, chain verification, and replay are inherited from the M0 substrate — no new history system.

---

## 6. Specification Intelligence (`wepld spec review`)

Hermes (reasoning via the Gateway, orchestrated by the Runtime) reviews a spec version and emits `SpecFinding`s across the twelve `FindingClass`es (missing acceptance criteria, hidden assumptions, architecture contradiction, missing rollback/benchmark/security/migration/testing/deployment/performance/observability/recovery). **Every finding is evidence-based** — it cites the spec section (or its absence) that produced it; nothing is invented (the v2-10 claims discipline applied to specs). Findings are recorded (`SpecReviewed`). Intelligence **never modifies** the spec — it informs the author and feeds Quality (§7). Deterministic under cassettes (fixture-first).

## 7. Spec Quality model (evidence-based, never invented)

`SpecQuality` per version, computed from structure + findings:

| Score | Derivation |
| --- | --- |
| Completeness | required sections present ÷ expected (template-driven) |
| Consistency | contradiction findings (arch/assumption) → inverse |
| Ambiguity | count of unresolved `[NEEDS CLARIFICATION]` markers |
| Risk | severity-weighted open findings |
| Coverage | acceptance criteria with a machine-checkable `verify` ÷ total |
| Missing sections | explicit list (the "missing_*" finding classes) |
| Review / Verification status | last review event / downstream mission gate evidence |

Each score references the findings/evidence that produced it. Surfaced by `wepld spec validate` (deterministic checks) and `wepld spec review` (reasoning findings). Quality gates conversion: a spec below a policy threshold (e.g. unresolved clarifications, coverage < 100%) **cannot** be converted to a mission.

## 8. Reverse Synchronization (proposal-only; approval mandatory)

During execution, Hermes may discover reality diverges from the spec (API/DB/architecture/ADR/dependency changed). It **never** edits the spec. It raises a `SpecRevisionProposal` — recorded as `SpecRevisionProposed` with the divergence evidence and a proposed `SpecDiff` — routed to the founder through the **existing decision/approval machinery** (a decision packet). The founder approves → a `ReviseSpec` command applies it → new `SpecVersion` + `SpecRevisionResolved{approved}`. Rejected → recorded, spec unchanged. This reuses WePLD's "human decides / evidence before assertion" invariants exactly; no new approval mechanism.

## 9. Specification Diff (`wepld spec compare`)

Pure function `diff(version_a, version_b) → SpecDiff`:

| Facet | Source |
| --- | --- |
| Changed requirements / assumptions / architecture | spec.md section diff |
| Changed tasks | tasks.md diff |
| Affected missions | `spec_links` (missions derived from either version) |
| Affected ADRs / skills / knowledge | `spec_links` |
| Affected runtime history | Chronicle causal edges from the spec to executed missions |

Analogous to Chronicle's mission comparison (v2-15 §6), applied to specifications. Deterministic; renders as a comparison document.

## 10. Specification Replay (Chronicle)

Chronicle replays a spec's full lifecycle — creation → clarifications → research → plan → tasks → derived missions → execution → completion → revisions → comparisons → forks — as a session scoped to the spec's ledger correlation. Reuses Chronicle's frame/session engine unchanged; the spec events become frames on a **Specification lens**. `wepld spec replay <spec>`.

## 11. Specification DNA (Chronicle intelligence)

Chronicle's insight pipeline (v2-16) gains spec-pattern insight classes computed across specs/projects: *"this project omits testing strategy 60% of the time," "this team writes strong rollback plans," "specs here average 2 clarification rounds."* Evidence-based, aggregated, **advisory** — promoted by a human into lesson candidates that improve future spec templates/reviews. **Never auto-modifies.** Reuses the existing insight → lesson → knowledge flywheel.

## 12. Engineering Memory integration

Specifications become **Knowledge**: approved specs and their lessons are knowledge records; **Context Assembly** includes relevant specs in future missions' packs (a spec is the highest-value context a real project has); skills and future specs reference them via `spec_links`. This is the compounding-memory thesis (v2-16 §5) with specifications as a primary memory type.

---

## 13. Mission Conversion (deterministic; multi-spec)

Pure transform in `wepld-specification`: `convert(specs[], plan, tasks, constitution) → { MissionBrief, PlanDoc, SpecProvenance, ValidationReport }`. Same inputs (same hashes) → same Mission (replayable). A mission may derive from **multiple** specs (dependency_links); the mission records all source spec+version refs. Field mapping as in v1 §5 (outcome, scope, acceptance_criteria→gates, tasks→PlanDoc). The Runtime then creates the mission via the existing command path, recording `MissionDerivedFromSpec` and the spec→mission link. Execution proceeds through the unchanged M0 lifecycle. Reverse-sync findings during execution flow back to the spec (§8).

---

## 14. Contract changes (rev 3 — additive; approved as extended Option B)

- **Aggregate type:** add `Specification`.
- **Vocabulary rev 3** (additive, precedent = Chronicle rev 2): `SpecificationCreated · SpecificationRevised · SpecClarified · SpecPlanGenerated · SpecTasksGenerated · SpecValidated · SpecReviewed · MissionDerivedFromSpec · SpecRevisionProposed · SpecRevisionResolved · SpecLinked · SpecStatusChanged`.
- **New contract module** `specification.rs` (§3 types), contracts → 0.5.0, lock test extended.

All additive; existing meanings unchanged; old ledgers remain valid. Flagged as touching a frozen contract — proceeding on the founder's Option-B approval.

## 15. Chronicle integration (additive derivation rules — engine frozen)

Chronicle's **engine and substrate stay frozen**; it grows exactly as designed (v2-11: "every item is a new reader or frame/edge rule over the same substrate"). Additive derivation rules: a **Specification lens** (spec events → frames), new causal edges (`derived` spec→mission, `reverse_sync` mission→spec-proposal, `depends` spec→spec), a comparison facet (spec diff), and a DNA insight class. No frozen Chronicle contract changes — it consumes the rev-3 events through its existing projection.

## 16. Repository & crate changes

- New crate `crates/specification/` (`wepld-specification`) — permanent subsystem, pure domain.
- `contracts`: `specification.rs` + rev-3 vocabulary + `Specification` aggregate (0.5.0).
- `ledger`: spec state tables + read/write methods (writes only via Runtime Tx).
- `runtime`: `spec` orchestration module (commands, gateway reasoning, conversion wiring).
- `cli`: `wepld spec *` commands.
- `fixtures/spec-templates/` (constitution, spec, plan, tasks — adopted from Spec Kit with structured anchors for robust parsing) + `fixtures/golden/spec-to-mission.trace`.
- Existing crates, frozen contracts (beyond the additive rev-3), Hermes, WWP, and existing golden traces: **untouched**.

## 17. CLI (additive; existing mission commands unchanged)

`wepld spec init | new <name> | clarify <spec> | validate <spec> | plan <spec> | tasks <spec> | review <spec> | replay <spec> | compare <a> <b>` and `wepld mission create --from-spec <spec…>`. `validate` and `compare` are deterministic/pure; `clarify/plan/tasks/review` use the Gateway (fixture-first, record/replay). `wepld mission new -f` remains as the lower-level primitive the conversion builds on.

## 18. Runtime changes (minimal, additive)

New `spec` orchestration: `create_spec`, `revise_spec`, `clarify_spec`, `generate_spec_plan`, `generate_spec_tasks`, `review_spec`, `validate_spec` (pure), `create_mission_from_spec`, `propose_spec_revision`, `resolve_spec_revision`. Each records the rev-3 event + spec state in one transaction; reasoning ones call the existing Gateway. No change to the mission state machine, phase engine, Hermes, or WWP.

## 19. Studio (reserved, M3+)

When Studio begins, **Specification is its own workspace** — an engineering specification environment (spec objects, versions, quality, findings, diff, replay), v0-generated per IADR-0008. Not a chat, not a markdown editor. Design reserves it; out of scope now.

---

## 20. Determinism, migration, risks

- **Determinism/replay:** conversion + diff are pure; reasoning is deterministic under cassettes; spec versions are immutable in CAS; Chronicle reconstructs the full chain. Golden `spec-to-mission` pins the normative sequence.
- **Migration:** purely additive/greenfield. Existing missions, data, and goldens unaffected. Spec-first becomes the *standard* workflow; `mission new -f` remains the internal primitive. Rev-3 bump is additive.
- **Compatibility risks:** Spec Kit evolves — we adopt concepts+templates, not code; brittle markdown → WePLD templates use structured anchors; additive contract change → flagged & lock-tested; users expecting `/implement` code-gen → documented boundary (Runtime executes); project constitution ≠ WePLD Charter (never overrides architecture).
- **Gap Notes:** none. The `/implement` tension is resolved by substitution; everything else maps onto existing primitives; rev-3 is additive-by-design.

## 21. Implementation slices (only after approval)

1. **Spec-A:** `wepld-specification` crate (model, structured templates, parse, `validate`, `convert`, `quality`, `diff` — pure, unit-tested) + `wepld spec init/new/validate`. Contracts rev-3 + `Specification` aggregate.
2. **Spec-B:** Runtime spec orchestration (create/revise/status) + `wepld mission create --from-spec` (conversion, CAS capture, spec↔mission links, `MissionDerivedFromSpec`) + golden `spec-to-mission`. Executes through the unchanged lifecycle.
3. **Spec-C:** gateway-backed `spec clarify/plan/tasks/review` (fixture-first + record mode) + Spec Quality + Intelligence findings.
4. **Spec-D:** reverse synchronization (proposal → decision → approved revision) + `spec compare` diff.
5. **Spec-E:** Chronicle Specification lens/edges + `spec replay`; DNA insight class; Knowledge/Context-Assembly integration.
6. **(M3+):** Specification Studio workspace.

Each slice: small commits, passing tests, fmt + clippy clean, fixture-first determinism, evidence-based verification, no drift.

## 22. Open decisions for approval

1. **Approve this updated design** (permanent Specification subsystem, all models).
2. **Crate name:** `wepld-specification` (dir `crates/specification/`) — confirm, or prefer a non-`wepld-` package name (e.g. `engineering-specification`) despite the workspace-namespace consistency cost.
3. **Rev-3 event set** (§14) — confirm the twelve events (or trim).
4. **Slice order** (Spec-A → E) — confirm.

No code will be written until you approve. Protect the Runtime. The Engineering Specification is the product.
