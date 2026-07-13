# IMPL-01 — Final Repository Layout

> **Amended by [IADR-0006](adr/IADR-0006-rust-core-tauri.md):** Cargo workspace, `crates/wepld-*` instead of `packages/*`, Rust toolchain per the translation table. Structure, ownership, and dependency rules below are unchanged.

pnpm workspace monorepo (IADR-0001, superseded — see banner). Fourteen packages — deliberately few; a solo founder pays ceremony tax per package. Internal module folders (enforced by dependency-cruiser, IMPL-02) provide finer boundaries without more packages. Every path below maps to a frozen-architecture element; nothing exists without a v2 anchor.

~~~text
wepld/
├── package.json  pnpm-workspace.yaml  tsconfig.base.json
├── .dependency-cruiser.cjs         # the architecture, as a lint rule
├── .github/workflows/ci.yml        # test + golden traces + boundaries on every PR
├── DECISIONS.md                    # founder log: OS choice, fixture repos, key dates
├── docs/                           # frozen architecture (v1, v2, adr) + this program (impl)
├── fixtures/
│   ├── repos/                      # the two sample repositories (git submodules or vendored)
│   ├── cassettes/                  # recorded brain interactions (IADR-0002)
│   ├── missions/                   # brief JSONs for golden traces
│   └── golden/                     # expected ledger traces (IADR-0004)
├── packages/
│   ├── contracts/                  # L0 — the shared truth
│   ├── ledger/                     # L1 — SQLite store: tables, ledger, fold, checkpoints, work queue
│   ├── artifacts/                  # L1 — content-addressed store (CAS)
│   ├── workspace/                  # L1 — git worktrees, snapshot refs, diffs, scope checks
│   ├── sandbox/                    # L1 — tier detection, envelope launchers, canary self-test
│   ├── wwp/                        # L2 — WWP JSON-RPC framing: server (Core side) + client (worker side)
│   ├── providers/                  # L2 — reasoning-provider gateway, profiles, adapters: fixture | anthropic | openai-compat (renamed from brains, IADR-0007)
│   ├── context/                    # L2 — Context Assembly: tiers, selection, manifest, redaction, capture
│   ├── runtime/                    # L3 — the mission engine (the Core's beating heart)
│   ├── chronicle/                  # L4 — frames, lenses, sessions, state_at, causal walk, fork/compare
│   ├── studio-api/                 # L4 — loopback HTTP + SSE + session token; command/query/stream routes
│   ├── hermes/                     # app — reference WWP worker (bin: hermes)
│   ├── studio/                     # app — React SPA: Mission / Decisions / Cinema surfaces
│   └── cli/                        # app — `wepld` bin: daemon, mission ops, timeline, demo
└── tools/
    ├── record-cassettes.ts         # refresh fixture cassettes against real providers
    └── new-golden.ts               # run a scenario, normalize, write expected trace
~~~

## Why each package exists (and its v2 anchor)

| Package | Exists because | v2 anchor |
| --- | --- | --- |
| `contracts` | one source of truth for every schema, event type, and WWP message; the freeze made executable | v2-07, v2-17 |
| `ledger` | single-writer transactional state + hash-chained append-only history is the product's spine | v2-06, ADR-0003 |
| `artifacts` | evidence and packs must be immutable, hash-addressed, classification-tagged | v2-07 §7 |
| `workspace` | isolation and time travel live in Git: worktrees, hidden snapshot refs, scope re-verification | v2-02 §4, ADR-0013 |
| `sandbox` | envelopes and honest tiers are the security story | v2-05, ADR-0004/0007 |
| `wwp` | the worker boundary is a protocol, not a function call — runtime replaceability | v2-03, ADR-0005 |
| `providers` | provider neutrality (the architecture's Brain Gateway role); credentials never leave Core; fixture adapter is the test spine; reasoning is optional per IADR-0007 §1 | v2-03, IADR-0002, IADR-0007 |
| `context` | the highest-leverage quality subsystem; packs captured for replay | v2-04, ADR-0006 |
| `runtime` | commands, state machine, phase engine, gates, decisions, messenger, recovery, budgets — the Core | v2-02 |
| `chronicle` | replay/forensics/branching as a read-side over ledger + CAS + refs | v2-11…15, ADR-0011 |
| `studio-api` | the only ingress: auth token, commands in, projections/SSE out | v2-02 §1, v2-17 |
| `hermes` | the reference worker must be an actually-separate program or replaceability is fiction | ADR-0005 |
| `studio` | the product surface: Mission, Decisions, Cinema — studio-first identity | v2-01, v2-13 |
| `cli` | founder's daily driver, demo vehicle, and the M0 UI | IMPL-04 |

## Package dependency graph

~~~mermaid
flowchart TB
  subgraph L0
    contracts
  end
  subgraph L1["L1 — substrate"]
    ledger --> contracts
    artifacts --> contracts
    workspace --> contracts
    sandbox --> contracts
  end
  subgraph L2["L2 — services"]
    wwp --> contracts
    providers --> contracts
    context --> contracts
    context --> artifacts
  end
  subgraph L3["L3 — engine"]
    runtime --> ledger & artifacts & workspace & sandbox & wwp & providers & context
  end
  subgraph L4["L4 — read-side & ingress"]
    chronicle --> ledger & artifacts & workspace & contracts
    studioapi["studio-api"] --> runtime & chronicle
  end
  subgraph Apps
    hermes --> wwp
    cli --> runtime & studioapi & chronicle
    studio -. "HTTP/SSE only" .-> studioapi
  end
~~~

Rules the graph encodes (enforced in CI, IMPL-02): `contracts` imports nothing; L1 packages import only `contracts`; L2 imports L1 + contracts, never each other (exception: `context → artifacts`, declared); `runtime` is the only package that composes L1+L2; `chronicle` never imports `runtime` — it reads stores and *submits commands through studio-api's command port* like any other caller (forks are commands, per v2-17); **`hermes` imports only `wwp` (+ contracts transitively)** — this single rule *is* worker-runtime replaceability, and CI fails if it ever gains another edge; `studio` has no workspace dependencies at all — it speaks HTTP, proving the UI/Core boundary. Clarification on chronicle: it *computes* plans (e.g., `ForkPlan`); mutations are commands submitted by callers (CLI/Studio) through studio-api to the runtime — chronicle itself never submits.

## Ownership

One founder owns everything — so "ownership" here means *blast-radius labels* used in PR titles and the risk register: `spine` (contracts, ledger, runtime — changes here need golden-trace review), `substrate` (artifacts, workspace, sandbox), `intelligence` (context, brains, chronicle), `surface` (studio-api, studio, cli, hermes). A future second engineer inherits a label, not a mystery.

## Services

There are exactly **two long-lived processes** (Core daemon incl. studio-api; and per-phase Hermes children) plus the browser. No other services exist in the MVP. Anything resembling a third service requires a gap note first.
