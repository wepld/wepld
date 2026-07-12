# 24 — Repository Structure

## Current state

This repository deliberately contains planning artifacts only. No source folders, build configuration, dependency manifests, CI configuration, or generated code should be created until the Architecture Gate (M0) is approved.

## Current documentation layout

~~~text
wepldProject/
  README.md
  docs/
    01_Project_Vision.md … 30_ARCHITECTURE_SUMMARY.md
    diagrams/
    adr/
~~~

## Proposed implementation monorepo after approval

~~~text
wepldProject/
  apps/
    studio/                 # desktop UI shell and workspace presentation
  crates/
    domain/                 # pure mission/task/policy types and invariants
    core-daemon/            # composition root, lifecycle, local RPC
    orchestration/          # DAGs, leases, retries, scheduling
    policy-security/        # policy evaluation, capabilities, secret interfaces
    event-ledger/           # event persistence, outbox, projection runtime
    artifacts-workspace/    # Git/worktree, artifact content store, sandbox ports
    worker-runtime/         # worker registry, adapters, host protocol
    brain-gateway/          # provider-neutral routing and adapters
    quality/                # checks, gates, review/security evidence
    knowledge/              # graph records, ingestion, retrieval indexes
    messenger/              # report/decision composition and channel ports
    registry/               # skills/plugins/packages and trust lifecycle
    observability/          # traces, logs, metrics, health
  adapters/
    brain/                  # one package per provider family
    worker/                 # Hermes-compatible and future adapters
    integrations/           # channels/MCP/third-party adapters
    toolchains/             # approved tool execution adapters
  packages/
    contracts/              # schema source, generated client types where needed
    ui-design-system/       # shared Studio visual primitives
    test-fixtures/          # mission, policy, provider, and evaluation fixtures
  docs/
    adr/
    architecture/
    runbooks/
    security/
  tools/
    dev/                    # non-product developer workflow helpers
    release/
  tests/
    contract/
    integration/
    e2e/
    adversarial/
    performance/
~~~

This structure is a target map, not permission to create the folders now.

## Boundary rules

- `domain` has no UI, database, provider, OS, or network dependency.
- Each core bounded context depends on domain/contracts and ports, never another context’s storage implementation.
- `adapters` depend inward on published ports; adapters never become a shared utility dumping ground.
- `apps/studio` only uses command/query/subscription contracts; it does not import Core persistence or worker execution modules.
- `packages/contracts` is versioned carefully and may generate types, but it does not own domain behavior.
- Test fixtures are first-class artifacts shared by contract/evaluation/security suites, never copied ad hoc into product modules.

## Ownership and review

| Area | Primary stewardship | Required review for material change |
| --- | --- | --- |
| Domain, event, API contracts | Core architect | Core + quality + security |
| Policy/security | Security division | Security + product owner |
| Worker/brain adapters | platform owner | Core + security + evaluation owner |
| Studio | product/UX owner | UX + accessibility + Core contract owner |
| Registry/plugins | ecosystem owner | security + compatibility owner |
| Release/tooling | release manager | security + platform owner |

## ADR and documentation conventions

Architecture Decision Records live in `docs/adr/` after M0, using status (proposed/accepted/superseded/rejected), context, options, decision, consequences, validation, owner, and review date. No ADR is required for a local refactor that preserves a contract; any change to a boundary, authority, data classification, event schema, permission model, or irreversible technology commitment requires one.

See also: [04_Component_Architecture.md](04_Component_Architecture.md), [18_API_Architecture.md](18_API_Architecture.md), [25_Development_Guidelines.md](25_Development_Guidelines.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).

