# 24 — Repository Structure

## Three repository states must not be conflated

### 1. Canonical `main`

The canonical branch currently contains the architecture/master-plan package. No Rust implementation is merged into `main`. This planning worktree is based on `origin/main` and may change documentation only.

~~~text
wepld/
  README.md
  docs/
    01_Project_Vision.md … 35_Reference_Systems_and_Competitive_Architecture.md
    diagrams/
    adr/                    # Proposed architecture decisions
~~~

Statements such as “implementation has not started” should therefore be read precisely as **no implementation is canonical on `main`**. They must not hide the existence of a candidate implementation.

### 2. Draft PR #1 candidate layout

Draft PR #1 is open, Draft, unmerged, and unratified. It contains a candidate Rust Build Feature baseline with crates for contracts, ledger, artifacts, workspace, WWP, providers, runtime, specification, Hermes, and CLI, plus fixtures and branch-local v2/implementation documents. That layout is reference evidence, not the repository’s canonical structure and not authorization to merge.

The Build Feature Baseline Gate records `Accepted`, `Returned`, `Deferred`, or `Rejected`, then becomes **Resolved** only when an H1 prerequisite path is explicit. If the candidate contracts are accepted, the accepted head and reconciled documentation establish the starting implementation layout. Otherwise, an approved replacement-foundation plan must cover every missing prerequisite and this plan must not assume the candidate crates exist. A non-accept disposition is not a permanent H1 blocker.

### 3. Future target after gated authorization

The target below is a bounded-context map, not permission to create folders. Names may map to separate crates or internal modules after the baseline decision and relevant ADR acceptance; package boundaries should be earned by dependency and ownership needs rather than copied mechanically.

~~~text
wepld/
  apps/
    studio/                    # H9 presentation over Core APIs
    desktop-shell/             # packaging/lifecycle only, if separately selected
  crates/
    contracts/                 # domain, SOP/role/tool, context, evidence, wire schemas
    core/                      # composition root, commands, policy, durable transitions
    specification/             # H1 charter/spec/outcome/change domain
    delivery/                  # H2 qualification, SOPGraph design, plans and flow
    ledger/                    # transactional state + append-only audit facts
    artifacts/                 # evidence/context plus bounded raw tool-output bodies
    workspace/                 # Git worktrees, snapshots, proposal refs, scope checks
    wwp/                       # replaceable worker protocol
    providers/                 # provider-neutral Brain Gateway adapters
    hermes/                    # first-party WWP runtime only
    skills/                    # H3.1 built-ins/tool catalogs; H3.2 packaging if gated
    hooks/                     # H3.1 typed built-in lifecycle dispatch
    context/                   # H4.1 packs, exploration branches and compaction records
    language-intelligence/     # H4.1 rust-analyzer; H4.2 extra adapters/impact
    retrieval/                 # H4.1 exact/lexical/Git; H4.2 structural; H4.3 semantic
    loops/                     # H5 loops, typed sandbox failure and advisor experiment
    supervision/               # H6 authorized role subscriptions and bounded parallelism
    memory/                    # H7 typed memory and Memory Judge
    evaluation/                # pre-H1 spine; H8 equivalence/certification/route races
    projections/               # H9 evidence-linked execution/team views, rebuildable
    studio-api/                # shared command/query/subscription semantics
  fixtures/
    contracts/
    missions/
    repositories/
    providers/
    adversarial/
    recovery/
    equivalence/
    ablations/
  docs/
    adr/
    architecture/
    runbooks/
    security/
  tools/
    dev/
    evaluation/
    release/
~~~

## Dependency and authority rules

- Contracts and pure validation depend on no product implementation.
- WePLD Core is the sole durable mission/specification/plan/phase/task/policy/approval/budget/completion authority.
- Domain modules do not import UI, provider SDKs, Git implementations, databases, or OS adapters.
- Brains return `PlanProposal` records; deterministic compilation produces candidates, `PlanAssessment` supplies risk-tiered qualification evidence, and only an authenticated Core-recorded `PlanDecision` can create an approved `DeliveryPlan`. A producer cannot approve or serve as the sole acceptance-critical reviewer, and model voting carries no authority.
- Hermes and other workers depend on WWP and granted task/effect contracts, never on Core persistence internals.
- `SOPGraph` and `AuthorizedRoleSubscriptionGraph` are typed Core projections: roles subscribe only to authorized artifact/event classes, cannot self-subscribe, and do not gain a free shared environment or peer-chat authority.
- The Tool Schema Compiler exposes only a version-bound `CapabilityProjectedToolCatalog`. Calls still re-enter the Effect Firewall; `BoundedToolResult`, content-addressed `ToolOutputArtifact`, and typed `SandboxFailureResult` carry classification, truncation and provenance without granting a retry or new capability.
- `MissionExplorationBranch` is non-authoritative until an explicit promotion decision; `CompactionRecord` preserves source ranges, omissions, mandatory pinned authority and rehydration. H7 Memory Judge policy governs any retained learning.
- Skill, hook, LSP, retrieval, compaction, risk-advisor, memory, route-race, and subagent components cannot mint capabilities or mutate higher-authority artifacts. `ContextualRiskAdvisor` remains advisory/experimental, and `ControlledMultiRouteRace` remains an H8 isolated/read-only controlled treatment until accepted evidence says otherwise.
- Studio, CLI, MCP, and APIs use the same Core command/query/subscription semantics; presentation code owns no workflow state.
- Adapters depend inward on published ports and may not become cross-domain utility dumping grounds.
- Large/untrusted bodies remain classified artifacts; ledger facts carry minimal references and provenance.
- Test/evaluation fixtures are first-class, versioned artifacts and are not copied ad hoc into modules. The ADR-0024 evaluation spine is operational before H1/H2 and records exact cases, arms, manifests, runs, observations, deviations, and results for every H milestone; ADR-0025 H8 certification consumes that history.

## Ownership and review

| Area | Primary stewardship | Required review for material change |
| --- | --- | --- |
| Governance hierarchy and domain contracts | Core architect | product + Core + quality + security |
| Specification/outcome/change semantics | product architecture | Core + quality + security |
| Plans/phases/Kanban/WIP | delivery-method owner | product + Core + quality |
| Policy/effect/sandbox boundaries | security | Security + Core + platform |
| Hermes/WWP/skills/hooks/tool projection/subagents | runtime owner | Core + security + evaluation |
| Context/LSP/retrieval/exploration/compaction | intelligence owner | Core + security + evaluation |
| Memory and Memory Judge | knowledge owner | product + security + quality |
| Outcome equivalence/evaluation/route races | evaluation owner | product + quality + security |
| Product surfaces and visual execution/team views | product/UX | accessibility + Core contract owner + quality |
| Release/tooling | release manager | security + platform owner |

## ADR and documentation conventions

Architecture Decision Records use `docs/adr/ADR-NNNN-kebab-slug.md` and include status, date, owner, review trigger, context, decision, alternatives/reason, consequences, validation, and migration impact. Implementation-only decisions may use the repository’s IADR convention after an implementation baseline exists.

Of ADR-0015 through ADR-0026, four (0015, 0016, 0020, 0024) are **Accepted** as architecture and the rest are **Proposed**. Their numbering avoids collision with Draft PR #1’s candidate ADR-0001 through ADR-0014; it does not ratify those unmerged ADRs. No dependent Hermes implementation begins until the relevant Proposed ADR is accepted and the preceding milestone gate closes.

See also: [04_Component_Architecture.md](04_Component_Architecture.md), [22_Milestones.md](22_Milestones.md), [25_Development_Guidelines.md](25_Development_Guidelines.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), and [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md).
