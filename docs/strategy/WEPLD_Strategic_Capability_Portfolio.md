# WePLD Strategic Capability Portfolio

**Standing:** planning only. Every capability below is `implementation_status:
Planned` or `Research` and `implementation_authorized: false`, except where the
Existing column cites Draft PR #1/#2/#3 evidence — and even those citations
describe Draft, unmerged work. Nothing here is authority to build.

## Capability taxonomy

Every capability is exactly one of:

Core authority service · AGILLE delivery service · Hermes runtime service ·
Agent role · Advisory deliberation system · Skill · Deterministic inspector ·
Execution infrastructure · Context/knowledge system · Evidence/assurance
system · Recovery/operations system · Studio surface · Integration ·
Deployment mode · Enterprise capability · Research experiment · Commercial
service.

Category confusion is a defect. Canonical examples: Mastermind is a **role**;
the Plan Compiler is a **deterministic Core service**; the Committee is an
**advisory deliberation system**; rust-analyzer is an **integrated
deterministic tool** behind a broker; Engineering Memory is a **governed
context/knowledge service**; SkillHouse is a **registry and evaluation
system**; Letta is a possible **external stateful-agent runtime** behind the
Universal Agent Gateway — never Core authority. A role is never a service; a
Skill is never authority; a Studio surface is never an engine.

## The CapabilityRecord contract

Every registry entry resolves this record. Group tables carry the fields that
vary; the following defaults hold for every entry unless its row says
otherwise:

```text
CapabilityRecord {
    capability_id, name, category, purpose, user_value, primary_users,
    problem_solved, authority_owner, durable_data_owner, required_inputs,
    typed_outputs, dependencies, prerequisite_decisions, security_boundary,
    privacy_boundary, failure_modes, recovery_semantics, evaluation_method,
    success_metrics, rejection_or_disable_criteria, v0_scope, mature_scope,
    build_buy_integrate_decision, open_source_boundary, commercial_boundary,
    delivery_stage, implementation_status, implementation_authorized
}
```

Registry-wide defaults:

- `authority_owner`: WePLD Core (an authenticated principal for decisions);
  never a model, agent, Committee, score, vote, or reputation.
- `durable_data_owner`: Core ledger/artifact/evidence stores.
- `security_boundary`: Effect Firewall + Capability Engine; all model output
  is untrusted until structurally validated and evidence-linked.
- `privacy_boundary`: classification-scoped projections; redaction before any
  provider egress; no credentials in model-visible content.
- `recovery_semantics`: durable terminal states, idempotent retry,
  supersession — never silent relabeling (per the PR #1 recovery discipline).
- `evaluation_method`: a preregistered arm or inspector check in the
  [Evaluation and Research Programme](WEPLD_Evaluation_and_Research_Programme.md);
  every capability has a rejection, disable, or defer path there.
- `implementation_status`: Planned (or Research where marked);
  `implementation_authorized`: **false**.
- `delivery_stage`: per the [Staged Delivery Roadmap](WEPLD_Staged_Delivery_Roadmap.md).

Priorities: **P0** Foundation · **P1** Differentiation · **P2** Scale ·
**P3** Ecosystem · **R** Research · **C** Conditional · **X**
Rejected/Replaced/Deferred (disposition stated).

## Group 1 — Core governance and AGILLE delivery

Nothing in this group may be delegated: it is the authority spine. Existing
evidence: the Draft PR #1 slice implements early forms of several rows
(ledger, CAS, staged approvals, completion decisions, honest
Returned/Deferred outcomes); PR #2 documents the target contracts.

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| WePLD Core | Core authority service | 0–1 | P0 | Foundation; seed exists in Draft PR #1 |
| AGILLE Delivery Engine | AGILLE delivery service | 1 | P0 | Foundation; the governed method engine over Core |
| Mission Charter | AGILLE delivery service (typed artifact) | 1 | P0 | Foundation |
| Engineering Specification | AGILLE delivery service (typed artifact) | 1 | P0 | Foundation; PR #2 doc 31 contract |
| Outcome Contract | AGILLE delivery service (typed artifact) | 1 | P0 | Foundation |
| Specification versioning + change control | Core authority service | 1 | P0 | Foundation; supersession, never edits |
| PlanProposal | AGILLE delivery service (typed artifact) | 1 | P0 | Foundation |
| Deterministic Plan Compiler | Core authority service | 1 | P0 | Foundation; deterministic, not a model |
| PlanAssessment + plan qualification | AGILLE delivery service | 1 | P0 | Foundation; independent of the proposer |
| PlanDecision | Core authority service (typed artifact) | 1 | P0 | Foundation; authenticated only |
| DeliveryPlan | AGILLE delivery service (typed artifact) | 1 | P0 | Foundation; versioned, exact history |
| Phases | AGILLE delivery service | 1–2 | P0 | Foundation |
| Kanban | AGILLE delivery service | 2 | P0 | Foundation; Core-enforced transitions |
| WIP limits | AGILLE delivery service | 2 | P0 | Foundation |
| TaskPacket compiler | Core authority service | 1 | P0 | Foundation; least-knowledge packets |
| Completion proposal | AGILLE delivery service (typed artifact) | 1 | P0 | Foundation; exists in slice form (PR #1) |
| CompletionDecision | Core authority service | 1 | P0 | Foundation; human-authenticated |
| Convergence assessment | Evidence/assurance system | 4 | P1 | Near-term; honest `NonConvergent` is a valid outcome |
| Finite controlled loops | Hermes runtime service | 4 | P0 | Foundation; budgets + no-progress detection |
| Governance Policy | Core authority service | 1 | P0 | Foundation |
| Policy Engine | Core authority service | 1 | P0 | Foundation; deterministic evaluation, separate decision records |
| Capability Engine | Core authority service | 2 | P0 | Foundation; issues/revokes scoped capabilities |
| Effect Firewall | Core authority service | 1 | P0 | Foundation; the only effect path |
| Budget Controller | Core authority service | 2 | P0 | Foundation; hard ceilings, durable exhaustion |
| Secret Manager | Core authority service | 2 | P0 | Foundation; credentials never model-visible |
| Identity and authorization | Core authority service | 1 | P0 | Foundation; authenticated principals only |
| Durable event ledger | Core authority service | 0–1 | P0 | Foundation; hash-chained seed exists (PR #1) |
| Artifact + evidence stores | Core authority service | 0–1 | P0 | Foundation; CAS seed exists (PR #1) |
| Provenance | Core authority service | 1 | P0 | Foundation; every artifact carries it |
| Idempotency + duplicate-effect control | Core authority service | 1 | P0 | Foundation; PR #1 acceptance CAS is the pattern |

## Group 2 — Hermes and Universal Agent Gateway

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Hermes Engineering Intelligence Runtime | Hermes runtime service | 1–5 | P0 | Foundation; PR #2 doc 32 architecture |
| Agent Kernel | Hermes runtime service | 2 | P0 | Foundation |
| Provider Gateway | Hermes runtime service | 1 | P0 | Foundation; seed exists (PR #1 gateway + cassette adapter) |
| Universal Agent Gateway | Hermes runtime service | 2 | P0 | Foundation; the Provider Gateway's Stage-2 evolution — **one** gateway, two protocol families; not a second engine |
| Intelligence Provider Protocol | Open protocol | 2 | P0 | Foundation; brains speak this |
| Engineering Worker Protocol | Open protocol | 2 | P0 | Foundation; WWP (PR #1) is the seed |
| Role routing | Hermes runtime service | 2 | P1 | Near-term; policy-scoped, never authority |
| Model/profile admission | Evidence/assurance system | 2 | P1 | Near-term; ADR-0025 certification path |
| Capability manifests | Hermes runtime service | 2 | P0 | Foundation; projected tool catalogs |
| Tool Router | Hermes runtime service | 2 | P0 | Foundation; behind capability manifests |
| Hook Bus | Hermes runtime service | 3 | P1 | Later; ADR-0018 |
| Subagent Supervisor | Hermes runtime service | 6 | P1 | Later; ADR-0021 bounded subagents |
| Provider health monitoring | Recovery/operations system | 2 | P1 | Near-term |
| Local / remote / enterprise / hybrid runtimes | Deployment mode | 2–8 | P0–P2 | Local first; remote under policy; enterprise Stage 8 |
| OpenAI-compatible adapters | Integration | 2 | P0 | Foundation; loopback seed exists (PR #1); hosted deferred to verified-TLS build |
| Anthropic-compatible adapters | Integration | 2 | P1 | Near-term; behind the same gateway contract |
| Kimi/Moonshot-compatible adapters | Integration | 2 | P2 | Later |
| Gemini-compatible adapters | Integration | 2 | P2 | Later |
| Local model adapters | Integration | 2 | P0 | Foundation; local-only mode depends on them |
| Human worker adapters | Integration | 8 | P2 | Later; humans as governed workers via TaskPackets |
| External coding-agent adapters | Integration | 6+ | C/R | Conditional research: external agents as governed workers behind the Engineering Worker Protocol; admitted per profile only after evaluation |

## Group 3 — Agent Hive roles

Roles are **invocation contracts**, not services and not authority. Researcher
discovers. Wisdom assesses knowledge. Mastermind designs. Builder constructs.
Consulting assures. DeepLearn distills verified experience. Core governs. No
role may approve its own work by renaming or re-invocation; independence is
structural (distinct identity, context trail, and no authorship of the
artifact under review — the PR #3 rule).

| Role | Stage | Pri | Boundary |
| --- | --- | --- | --- |
| Mastermind Agent | 1 | P0 | designs specifications/plans; proposals only; never qualifies or approves its own plan |
| Researcher Agents | 3 | P1 | discovery into typed ResearchPackets; no effects |
| Wisdom Agent | 3 | P1 | assesses research; chairs Committee synthesis (PR #3); never final authority |
| Builder Agents | 1 | P0 | construct in isolated worktrees; evidence-producing; never self-accepting (exists in slice form, PR #1) |
| Consulting / Independent Assurance Agent | 1 | P0 | independent assessment of delivery and plan changes; independence is structural |
| Test Agent | 4 | P1 | plans/extends verification; deterministic runners do the proving |
| Security Agent | 4 | P1 | threat-driven review; findings, not authority |
| Evidence Agent | — | X | **Replaced**: primary evidence must come from the deterministic Evidence Producer and Core stores; a narrative role would invite fabricated evidence. Advisory report-drafting only, folded into Consulting |
| Recovery Agent | 4 | P1 | drives Recovery Room probes under approval; never silent repair |
| DeepLearn Agent | 6 | P1 | distills verified experience into Memory/Skill candidates; cannot promote them |
| Justified domain specialists | 6+ | C | Conditional: admitted per Domain Pack evidence, not by default |
| Human engineers and contractors | 8 | P2 | governed workers with TaskPackets, leases, and evidence duties — same rules as AI workers |

## Group 4 — Engineering Committee

The corrected PR #3 architecture (documents 36–37, ADR-0026) is adopted as-is:
three-or-more selected members; local/Hermes/API/hybrid membership; immutable
`CommitteePack`; per-member projections; independent first round; bounded
challenge rounds; Wisdom synthesis; canonical `MinorityReport` +
`MinorityReportProjection`; `ModelIdentityEvidence` assurance tiers; typed
`LineageEvidence` with honest unknowns; cost/context ceilings; quorum; durable
dispositions; presets; performance records; user-triggered V0; no model-voting
authority; no direct plan mutation; `PlanChangeProposal` through normal
qualification. Nothing in this portfolio redesigns or weakens those
boundaries; the Committee stays an **advisory deliberation system** at Stage
6, priority P1, gated by the document-37 admission rule (terminal EC-A1,
EC-A2, EC-A3, EC-A5, EC-A6; EC-A7/EC-A8 before any diversity-based claims or
routing).

## Group 5 — Project knowledge and engineering truth

Clarified identities (see the consolidation map): **Project DNA** is the
relatively stable project identity and conventions; the **Truth Graph** is
traceability and authoritative relationships; the **Digital Twin** is a
derived operational/simulation representation and never a source of truth;
the **Change Passport** is one proof-carrying change record.

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Engineering Truth Graph | Context/knowledge system | 3 | P1 | Near-term; traceability spine |
| Project DNA | Context/knowledge system | 3 | P1 | Near-term |
| Project Constitution Compiler | Core authority service | 3 | P1 | Near-term; compiles DNA + policy into enforceable rules |
| Architecture rules | Deterministic inspector | 3 | P1 | Near-term; Constitution output |
| Repository map | Context/knowledge system | 3 | P1 | Near-term |
| Module + ownership graph | Context/knowledge system | 3 | P1 | Near-term |
| Symbol graph | Context/knowledge system | 3 | P1 | Near-term; LSP-derived |
| Dependency + call graph | Context/knowledge system | 3 | P1 | Near-term |
| Project Digital Twin | Context/knowledge system | 7 | P2 | Later; derived from Truth Graph + operational observations |
| Runtime topology | Context/knowledge system | 7 | P2 | Later; Twin input |
| Decision memory | Context/knowledge system | 3 | P1 | Near-term; projections of the decision ledger |
| Ask Why / Explain This Project | Studio surface | 5 | P1 | A Truth Graph query surface, not an engine |
| Stakeholder-specific truth views | Studio surface | 5 | P2 | Later; projections only |
| Project replay | Recovery/operations system | 4 | P2 | A Flight Recorder projection (see consolidation) |
| Evidence-linked search | Context/knowledge system | 3 | P1 | Near-term |
| Technical-debt ledger | Evidence/assurance system | 4 | P2 | Later; feeds Economics Engine |

## Group 6 — Context and impact intelligence

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Context Compiler | Hermes runtime service | 3 | P0 | Foundation; ADR-0019 |
| ContextPack | Typed artifact | 3 | P0 | Foundation |
| Context projections | Context/knowledge system | 3 | P0 | Foundation; per-audience, hash-recorded (PR #3 pattern) |
| Provider-specific context | Context/knowledge system | 3 | P0 | Foundation; egress-scoped |
| Redaction + secret removal | Deterministic inspector | 3 | P0 | Foundation; before any egress |
| Relevant-file / relevant-symbol selection | Context/knowledge system | 3 | P1 | Near-term |
| Git evidence | Context/knowledge system | 3 | P0 | Foundation |
| ADR + policy selection | Context/knowledge system | 3 | P1 | Near-term |
| Test impact | Context/knowledge system | 4 | P1 | Near-term |
| Architecture impact | Context/knowledge system | 3 | P1 | Near-term |
| Exclusion records | Evidence/assurance system | 3 | P1 | Near-term; what was deliberately left out, and why |
| Context hashes + freshness | Evidence/assurance system | 3 | P0 | Foundation |
| Poisoning detection | Research experiment | 6 | R | Research; heuristics must beat false-positive costs before admission |
| Language-neutral LSP Broker | Execution infrastructure | 3 | P0 | Foundation; wraps rust-analyzer first |
| AST/symbol intelligence | Context/knowledge system | 3 | P1 | Near-term |
| Structural analysis | Context/knowledge system | 3 | P1 | Near-term |
| Semantic retrieval | Research experiment | 4+ | C | **Conditional**: admitted only after ablation proves it beats exact/LSP/structural retrieval (doc 34 discipline) |

## Group 7 — Engineering Memory, SkillHouse and learning

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Governed Engineering Memory | Context/knowledge system | 6 | P1 | Advisory only; slice seed exists (PR #1, repo-scoped lessons) |
| MemoryCandidates | Typed artifact | 6 | P1 | The only admission path into memory |
| Memory Judge | Core authority service | 6 | P1 | ADR-0020; decides admission/retrieval eligibility |
| Freshness + expiry | Context/knowledge system | 6 | P1 | Near-term |
| Contradiction handling | Context/knowledge system | 6 | P1 | Near-term |
| Private project memory | Context/knowledge system | 6 | P1 | Default scope |
| Organization memory | Enterprise capability | 8 | P2 | Later; opt-in |
| Built-in Skill Kernel | Hermes runtime service | 3 | P0 | Foundation; ADR-0018 |
| Project Skills | Skill | 6 | P1 | Later |
| Organization Skills | Skill | 8 | P2 | Later |
| Certified global SkillHouse | Commercial service + registry | 9 | P3 | Ecosystem; only after certification foundations |
| SkillPackage / versioning / signing / compatibility / provenance | Registry contracts | 6 | P1 | Later |
| Skill evaluation / canary / promotion / suspension / revocation / rollback | Evidence/assurance system | 6 | P1 | Later; no automatic self-promotion, ever |
| LearningEpisodes | Typed artifact | 6 | P1 | Later |
| DeepLearn SkillCandidates | Typed artifact | 6 | P1 | Later; candidates only |
| Poisoning defense | Evidence/assurance system | 6 | P1 | Later; required before any sharing tier |
| License analysis (skills) | Deterministic inspector | 6 | P1 | Later |
| Opt-in global contribution | Enterprise capability | 9 | P3 | Ecosystem; **opt-in only, never default** |

## Group 8 — Letta / MemGPT

Studied as a reference system and optional external runtime — full assessment
in the [Tooling and Integration Map](WEPLD_Tooling_and_Integration_Map.md).
Disposition summary: **adopt concepts now; optional adapter later;
never authority.** Letta is not required for V0, does not replace Hermes or
Core; Letta memory is not Engineering Memory (it enters as `MemoryCandidate`);
Letta procedures enter as `SkillCandidate`s; Letta sits behind the Universal
Agent Gateway; shared writable Letta memory cannot be authoritative project
state and cannot be shared between independent Consulting or Committee roles;
Committee use requires a frozen, hash-bound memory snapshot; cloud use obeys
data-egress policy; local use obeys sandbox and Effect Firewall rules.

## Group 9 — Verification and independent assurance

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Verification Lab | Evidence/assurance system | 4 | P0 | Foundation engine of proof |
| Proof Gap Detector | Deterministic inspector | 4 | P1 | **Inside Verification Lab** (consolidated, not a separate engine) |
| Claim-to-evidence mapping | Evidence/assurance system | 4 | P0 | Foundation |
| Acceptance coverage | Evidence/assurance system | 4 | P0 | Foundation; slice seed exists (PR #1 criteria↔gates) |
| Evidence freshness | Evidence/assurance system | 4 | P1 | Near-term |
| Deterministic tests / focused selection / regression planning | Deterministic inspector | 4 | P0/P1 | Integrate existing runners; never rebuild them |
| Property testing / fault injection | Deterministic inspector | 4 | P2 | Later; integrate proptest/cargo-mutants-class tools |
| Static analysis / compiler + LSP diagnostics | Deterministic inspector | 4 | P0 | Integrate (clippy, rust-analyzer) |
| Security scanning / secret scanning / vulnerability scanning | Deterministic inspector | 4 | P1 | Integrate (audit/gitleaks-class tools) behind adapters |
| License analysis / SBOM / provenance | Deterministic inspector | 7 | P2 | Adopt standards (SPDX/CycloneDX, SLSA-class provenance) |
| Migration verification | Evidence/assurance system | 7 | P2 | Later; Migration Lab consumes it |
| Accessibility / performance / visual regression / compatibility / cross-platform | Deterministic inspector | 7 | P2 | Later; integrate per stack |
| Independent Consulting findings | Evidence/assurance system | 1 | P0 | Foundation; V0 requirement |
| Residual-risk assessment | Evidence/assurance system | 4 | P1 | Near-term; honest remainder, never hidden |

## Group 10 — Architecture and consistency intelligence

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Architecture Drift Radar | Deterministic inspector | 4 | P1 | Consumes Constitution policies — one rules engine, two consumers |
| Ambiguity detection | Deterministic inspector | 3 | P1 | A specification-workflow inspector, not a product |
| Contradiction detection | Deterministic inspector | 3 | P1 | Same inspector family |
| Task identity consistency | Deterministic inspector | 2 | P0 | Foundation; exists in seed form (PR #1 validators) |
| Specification/plan contradiction | Deterministic inspector | 3 | P1 | Near-term |
| Documentation/code contradiction | Deterministic inspector | 4 | P2 | Later |
| Unauthorized dependency directions | Deterministic inspector | 3 | P1 | Constitution rule |
| Business-logic / database boundary checks | Deterministic inspector | 3 | P1 | Constitution rules |
| Policy drift | Deterministic inspector | 4 | P1 | Near-term |
| Governance Diff | Core authority service | 4 | P1 | Near-term; downstream effects of policy/spec/contract/plan changes |
| Plan staleness | Deterministic inspector | 4 | P1 | Near-term |
| ADR consistency | Deterministic inspector | 0 | P0 | Exists for docs (validator); extends to code later |
| Authorized deviations | Core authority service | 4 | P1 | Recorded exceptions, never silent ones |

## Group 11 — Execution and environment safety

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Environment Capsules | Execution infrastructure | 4 | P1 | Later; reproducible run identity |
| Reproducible toolchains | Execution infrastructure | 4 | P1 | Integrate (rustup/lockfiles/containers) |
| Worktree Manager | Execution infrastructure | 1 | P0 | Foundation; exists in seed form (PR #1) |
| One writer per worktree | Execution infrastructure | 1 | P0 | Foundation; PR #1 invariant |
| Sandbox Manager | Execution infrastructure | 4 | P0 | Foundation; DEV tier honesty until real containment lands |
| Local / container / VM / enterprise / remote runners | Execution infrastructure | 4–8 | P0–P2 | Local first; container next; VM/enterprise Stage 8 |
| Filesystem grants / network policy / resource limits | Execution infrastructure | 4 | P0 | Foundation; envelope becomes enforced, not descriptive |
| Capability Leasing | Core authority service | 2 | P0 | Foundation; scoped, task/actor/repo/path/effect-bound, time-limited, auto-expiring, durably recorded |
| Automatic expiration | Core authority service | 2 | P0 | Foundation; no privilege persistence |
| Safe Git operations | Execution infrastructure | 1 | P0 | Foundation; exists in seed form (PR #1 `--end-of-options`, CAS refs) |
| Environment identity + compatibility matrices | Execution infrastructure | 7 | P2 | Later |

## Group 12 — Recovery and operational truth

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Engineering Flight Recorder | Recovery/operations system | 4 | P1 | The ledger's operational projection; one fact store |
| Recovery Room | Recovery/operations system | 4 | P1 | Consumes Flight Recorder facts; guided reconciliation |
| Crash reconciliation | Recovery/operations system | 4 | P0 | Foundation; PR #1 acceptance-recovery is the pattern |
| Uncertain-effect investigation | Recovery/operations system | 4 | P0 | Foundation; `Uncertain` is first-class |
| Duplicate-effect detection | Recovery/operations system | 4 | P0 | Foundation |
| Orphan-worktree / stale-lock recovery | Recovery/operations system | 4 | P1 | Near-term; PR #1 documents the orphan case honestly |
| Interrupted-provider recovery | Recovery/operations system | 4 | P1 | Near-term |
| Evidence reconstruction | Recovery/operations system | 4 | P1 | Near-term |
| Retry safety + recovery probes | Recovery/operations system | 4 | P0 | Foundation |
| Time-travel replay | Recovery/operations system | 4 | P2 | A Flight Recorder projection (consolidated) |
| Incident Commander | Recovery/operations system | 7 | P2 | Later; coordinated incident roles + evidence |
| Postmortems / runbooks / production rollback evidence | Recovery/operations system | 7 | P2 | Later |

## Group 13 — Planning and decision intelligence

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Mission Simulator | Research experiment | 5 | R | Research; predictions are model output, clearly labeled, never guaranteed outcomes |
| Counterfactual Decision Lab | Research experiment | 5 | R | Research; same engine family as the Simulator (consolidated "Decision Lab") |
| Plan comparison (A/B) | AGILLE delivery service | 5 | P1 | Later; same evidence, same constraints |
| Expected-effect / cost / schedule estimates | AGILLE delivery service | 5 | P2 | Later; estimates labeled as estimates |
| Reversibility / rollback / dependency simulation | AGILLE delivery service | 5 | P2 | Later |
| Committee trigger recommendation | Advisory deliberation system | 6 | C | Conditional on Committee admission + policy |
| Decision Inbox | Studio surface | 5 | P1 | A surface over the Core decision queue, not an engine |
| Material-decision filtering | Core authority service | 5 | P1 | Near-term; what deserves human attention |
| Approval expiry + supersession | Core authority service | 5 | P1 | Near-term; stale authorization dies |

## Group 14 — Studio (summary)

Studio begins as **Engineering Mission Control**, not a full IDE. Full surface
inventory, staging, and rejection criteria live in
[Studio and User Experience](WEPLD_Studio_and_User_Experience.md). Every
surface is a projection of Core truth; no surface owns state or authority.

## Group 15 — Trust adoption and autonomy (summary)

Shadow Mode, advisory/supervised/bounded modes, the seven-level Autonomy
Ladder (0 Observe → 6 governed programme autonomy), evidence-based promotion,
rollback, capability- and risk-specific autonomy, expiry, human override, and
**no universal full-autonomy switch** — detailed in
[Studio and User Experience](WEPLD_Studio_and_User_Experience.md). Stage 5,
P1.

## Group 16 — Change, release and production truth

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Change Passport | Evidence/assurance system | 4 | P1 | One proof-carrying change record; Truth Graph edge bundle |
| Proof-carrying changes | Evidence/assurance system | 4 | P1 | Near-term |
| Release Guardian | Evidence/assurance system | 7 | P2 | Consumes Change Passports + Verification evidence (consolidated) |
| Versioning / compatibility / changelog / signing | Evidence/assurance system | 7 | P2 | Later; integrate standards |
| SBOM + provenance | Deterministic inspector | 7 | P2 | Adopt standards |
| Release candidates / feature flags / canary / smoke tests | Execution infrastructure | 7 | P2 | Later; integrate existing platforms |
| Post-release verification + rollback readiness | Evidence/assurance system | 7 | P2 | Later |
| Production Truth Loop | Evidence/assurance system | 7 | P2 | Later; a green release is not automatically a successful product outcome |
| Runtime telemetry as evidence | Evidence/assurance system | 7 | P2 | Later; redaction first; no raw sensitive telemetry reaches model context without authorization |
| SLOs + quality-budget monitoring | Evidence/assurance system | 7 | P2 | Later |
| User-outcome validation / post-release Outcome Contract assessment | Evidence/assurance system | 7 | P2 | Later |
| Production drift + observability redaction | Evidence/assurance system | 7 | P2 | Later |

## Group 17 — Data, migration and supply-chain safety

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Migration and Data Safety Lab | Evidence/assurance system | 7 | P2 | Later; applied-history checks, destructive-change detection, expand-and-contract analysis, backward/forward compatibility, rollback feasibility, backup/restore drills, data-loss detection, data classification |
| Dependency and Supply-Chain Guardian | Evidence/assurance system | 7 | P1 | Near-term seed (Lean Solution Gate) then full guardian |
| Dependency justification + Lean Solution Gate | Core authority service | 2 | P0 | Foundation; the PR #1 boundary-rule discipline generalized |
| Transitive analysis / maintenance health / license compatibility / build-script risk | Deterministic inspector | 7 | P2 | Integrate (cargo-audit/deny-class tools) |
| Pinning / checksums / trusted registries / offline mirrors | Execution infrastructure | 7 | P2 | Later; sovereignty prerequisite |

## Group 18 — Quality and engineering economics

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Quality Attribute Budgets | Evidence/assurance system | 7 | P2 | Later; security/reliability/latency/memory/startup/bundle/build-test-duration/accessibility/cloud-cost budgets as typed gates |
| Engineering Economics Engine | Evidence/assurance system | 7 | P2 | Later; implementation/maintenance/complexity/provider cost, debt interest, rework, Skill-reuse savings, value-versus-risk. Shares CostRecord data with cost routing but serves **decisions**, not invocations (consolidated) |

## Group 19 — Feedback and product requirements

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Customer Feedback Compiler | Context/knowledge system | 7 | P2 | Later; provenance-carrying intake, duplicate clustering, evidence separation, complaint-versus-requirement distinction, problem candidates, requirement proposals, specification-change requests, impact estimates. **No automatic backlog or specification authority; popularity is not authority** |

## Group 20 — Evaluation and trust registry

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Benchmark Arena | Evidence/assurance system | 6 | P1 | The ADR-0024 spine, productized |
| Model and Agent Trust Registry | Evidence/assurance system | 6 | P1 | Certification states (ADR-0025) + Committee reputation (consolidated home) |
| Task-family / role-specific / language / environment / risk-tier evaluation | Evidence/assurance system | 6 | P1 | Later |
| Model/profile fingerprinting | Evidence/assurance system | 6 | P1 | PR #3 `ModelIdentityEvidence` is the contract |
| Committee reputation / minority-finding value / unsupported claims / defect detection / false positives / plan churn / cost / latency / human corrections / fallback performance | Evidence/assurance system | 6 | P1 | Doc-37 metric families feeding the registry |
| Universal best-model claims | — | X | **Rejected permanently**; only scoped, evidence-bound claims exist |

## Group 21 — Team, enterprise and programme mode

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Organizations / users / roles / membership | Enterprise capability | 8 | P2 | Later |
| Human–AI workforce + contractor mode | Enterprise capability | 8 | P2 | Later; same governed-worker rules |
| Separation of duties + approval delegation (time-bounded) | Core authority service | 8 | P2 | Later |
| Enterprise policy / audit retention / data residency | Enterprise capability | 8 | P2 | Later |
| Private gateways + private SkillHouse | Enterprise capability | 8 | P2 | Later |
| Program Mode | Enterprise capability | 9 | P3 | Ecosystem; cross-repository Outcome Contracts, coordinated schema changes, synchronized releases, compatibility matrices, programme risks, programme Committee. **No uncontrolled shared writable workspace — rejected permanently** |

## Group 22 — Sovereignty, deployment and exit

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Local-only / self-hosted / private cloud / enterprise cluster / hybrid / air-gapped / sovereign | Deployment mode | 2–8 | P0–P2 | Local-only is the founding mode; the rest stage upward |
| No-telemetry mode | Deployment mode | 2 | P0 | Foundation; telemetry is opt-in |
| Private artifact storage / internal signing / internal model registry / offline mirrors | Enterprise capability | 8 | P2 | Later |
| Sovereignty and Exit Pack | Core authority service | 8 | P1 | Complete open-format export of project engineering truth: events, artifacts, evidence, decisions, memory, skills; provider/agent/skill/evidence portability. **No lock-in through proprietary engineering truth** |

## Group 23 — Open protocol and ecosystem

| Capability | Category | Stage | Pri | Disposition / notes |
| --- | --- | --- | --- | --- |
| Open WePLD Protocol SDK | Open protocol | 9 | P3 | Ecosystem; provider/worker adapters, deterministic inspectors, Skill SDK, policy packs, Committee perspectives, evidence collectors |
| Conformance tests + fixtures + version compatibility | Evidence/assurance system | 9 | P3 | Ecosystem |
| Extension signing + revocation | Core authority service | 9 | P3 | Ecosystem |
| Open artifact/evidence schemas | Open protocol | 2 | P0 | Foundation — published early; they are the exit guarantee |
| AGILLE as independent methodology | Open protocol | 9 | P1 | The method stands alone; WePLD is the reference governed implementation |

## Group 24 — Domain Packs

Healthcare, FinTech, SaaS, Rust Security, Mobile, Data Platform, Regulated
Systems, and organization-defined packs — Stage 9, P3, all **Planned**. A pack
may contain policies, specifications, threat models, evidence requirements,
Committee perspectives, Skills, templates, and fixtures. **No pack may claim
automatic legal compliance**; a pack supplies evidence machinery, and
compliance remains a human, organizational judgment.

## Group 25 — Integrations and public/commercial surfaces

Decision detail in the [Tooling and Integration Map](WEPLD_Tooling_and_Integration_Map.md):
source control (GitHub/GitLab/Bitbucket/local Git), planning systems
(Jira/Linear/GitHub Issues), communication (Slack/Teams/email/webhooks),
CI/CD (GitHub Actions/GitLab CI/Jenkins/enterprise), runtimes
(Docker/Kubernetes/VM/local/enterprise), standards (MCP/ACP/LSP/OpenAPI/JSON
Schema/SARIF/SBOM/provenance), identity providers, billing, artifact stores,
observability. All integrations remain behind WePLD authority and capability
boundaries. Commercial/web boundary: Studio is a custom application; Core is
not implemented in WordPress; WordPress is marketing/content only if
operationally useful; WooCommerce is not a Core dependency and may only be
evaluated later for a simple marketplace — never for mission state, billing
authority, or engineering truth; SaaS billing uses a dedicated provider
abstraction (Stripe-class); provider usage metering and WePLD entitlement stay
separate concerns.

## Additional strategic systems (all included above)

Engineering Digital Twin (Group 5) · Production Truth Loop (Group 16) ·
Capability Leasing (Group 11) · Governance Diff (Group 10) · Counterfactual
Decision Lab (Group 13) · Sovereignty and Exit Pack (Group 22) · Customer
Feedback Compiler (Group 19) · Engineering Economics Engine (Group 18).

## Exemplar full CapabilityRecords

Twelve majors carry the full record in
[Product Architecture Map — exemplar records](WEPLD_Product_Architecture_Map.md);
every other row resolves the same contract through its group table plus the
registry-wide defaults above.

## Rejected / Replaced register (nothing silently omitted)

| Concept | Disposition |
| --- | --- |
| Universal full-autonomy switch | Rejected permanently — autonomy is capability- and risk-scoped, earned, expiring |
| Model/Committee vote as authority | Rejected permanently (PR #3 boundary) |
| Popularity-based backlog/specification authority | Rejected permanently |
| Consumer-subscription workarounds, cookie capture, chat-session automation | Rejected permanently |
| Uncontrolled shared writable workspace (Program Mode or otherwise) | Rejected permanently |
| Universal best-model claims | Rejected permanently |
| Evidence Agent (LLM as primary evidence source) | Replaced by deterministic Evidence Producer + Core stores |
| Time-Travel Replay as a separate engine | Replaced — Flight Recorder projection |
| Proof Gap Detector as a separate product | Replaced — Verification Lab component |
| Cost Router as an economics engine | Replaced — routing stays in Hermes; economics in the Economics Engine |
| Letta as Core/Hermes replacement or authoritative memory | Rejected; concepts adopted, optional adapter Conditional |
| WooCommerce for mission state/billing authority/engineering truth | Rejected; marketplace evaluation only, later |
| Full IDE as the founding Studio | Deferred — Mission Control first; editor breadth reconsidered at Stage 9 with ACP/editor interop |
| WordPress as any part of Core | Rejected; marketing/content only |
