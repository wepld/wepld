# 24 — Repository Structure

## Three repository states must not be conflated

### 1. Canonical `main`

The canonical branch currently contains the architecture/master-plan package. No Rust implementation is merged into `main`. This planning worktree is based on `origin/main` and may change documentation only.

~~~text
wepld/
  README.md
  docs/
    01_Project_Vision.md … 34_Harness_Evaluation_Protocol.md
    diagrams/
    adr/                    # Proposed architecture decisions
~~~

Statements such as “implementation has not started” should therefore be read precisely as **no implementation is canonical on `main`**. They must not hide the existence of a candidate implementation.

### 2. Draft PR #1 candidate layout

Draft PR #1 is open, Draft, unmerged, and unratified. It contains a candidate Rust Build Feature baseline with crates for contracts, ledger, artifacts, workspace, WWP, providers, runtime, specification, Hermes, and CLI, plus fixtures and branch-local v2/implementation documents. That layout is reference evidence, not the repository’s canonical structure and not authorization to merge.

If the Build Feature Baseline Gate accepts the candidate, the accepted head and its reconciled documentation establish the starting implementation layout. If it is returned or replaced, this plan must not assume those crates exist.

### 3. Future target after gated authorization

The target below is a bounded-context map, not permission to create folders. Names may map to separate crates or internal modules after the baseline decision and relevant ADR acceptance; package boundaries should be earned by dependency and ownership needs rather than copied mechanically.

~~~text
wepld/
  apps/
    studio/                    # H9 presentation over Core APIs
    desktop-shell/             # packaging/lifecycle only, if separately selected
  crates/
    contracts/                 # versioned domain, wire, evidence, and validation schemas
    core/                      # composition root, commands, policy, durable transitions
    specification/             # H1 charter/spec/outcome/change domain
    delivery/                  # H2 plans, phases, Kanban, WIP, task packets
    ledger/                    # transactional state + append-only audit facts
    artifacts/                 # content-addressed evidence and context bodies
    workspace/                 # Git worktrees, snapshots, proposal refs, scope checks
    wwp/                       # replaceable worker protocol
    providers/                 # provider-neutral Brain Gateway adapters
    hermes/                    # first-party WWP runtime only
    skills/                    # H3 manifests, procedures, routing, conformance
    hooks/                     # H3 typed lifecycle hook dispatch
    context/                   # H4 Context Compiler and provenance manifests
    language-intelligence/     # H4 normalized LSP adapter boundary
    retrieval/                 # H4 lexical/LSP/structural/semantic ranking
    loops/                     # H5 bounded loop and escalation policy
    supervision/               # H6 assignments, handoffs, bounded parallelism
    memory/                    # H7 typed memory and Memory Judge
    evaluation/                # H8 outcome-equivalence and ablation harness
    projections/               # Timeline/Mission Control/read models, rebuildable
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
- Brains return structured proposals; they do not execute or approve.
- Hermes and other workers depend on WWP and granted task/effect contracts, never on Core persistence internals.
- Skill, hook, LSP, retrieval, memory, and subagent components cannot mint capabilities or mutate higher-authority artifacts.
- Studio, CLI, MCP, and APIs use the same Core command/query/subscription semantics; presentation code owns no workflow state.
- Adapters depend inward on published ports and may not become cross-domain utility dumping grounds.
- Large/untrusted bodies remain classified artifacts; ledger facts carry minimal references and provenance.
- Test/evaluation fixtures are first-class, versioned artifacts and are not copied ad hoc into modules.

## Ownership and review

| Area | Primary stewardship | Required review for material change |
| --- | --- | --- |
| Governance hierarchy and domain contracts | Core architect | product + Core + quality + security |
| Specification/outcome/change semantics | product architecture | Core + quality + security |
| Plans/phases/Kanban/WIP | delivery-method owner | product + Core + quality |
| Policy/effect/sandbox boundaries | security | Security + Core + platform |
| Hermes/WWP/skills/hooks/subagents | runtime owner | Core + security + evaluation |
| Context/LSP/retrieval | intelligence owner | Core + security + evaluation |
| Memory and Memory Judge | knowledge owner | product + security + quality |
| Outcome equivalence/evaluation | evaluation owner | product + quality + security |
| Product surfaces | product/UX | accessibility + Core contract owner |
| Release/tooling | release manager | security + platform owner |

## ADR and documentation conventions

Architecture Decision Records use `docs/adr/ADR-NNNN-kebab-slug.md` and include status, date, owner, review trigger, context, decision, alternatives/reason, consequences, validation, and migration impact. Implementation-only decisions may use the repository’s IADR convention after an implementation baseline exists.

ADR-0015 through ADR-0024 in this planning package are **Proposed**. Their numbering avoids collision with Draft PR #1’s candidate ADR-0001 through ADR-0014; it does not ratify those unmerged ADRs. No dependent Hermes implementation begins until the relevant Proposed ADR is accepted and the preceding milestone gate closes.

See also: [04_Component_Architecture.md](04_Component_Architecture.md), [22_Milestones.md](22_Milestones.md), [25_Development_Guidelines.md](25_Development_Guidelines.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), and [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md).
