# IMPL-06 — Testing Strategy

Inherits v2-26's philosophy ("test the organization, not just the functions") scaled to one engineer: **few layers, ruthlessly automated, deterministic by default** (cassettes, IADR-0002), with the frozen architecture itself encoded as failing tests (IADR-0004). CI runs the whole pyramid on every PR; total wall time budget: < 5 minutes (kept honest by cassettes and fixture repos).

## The pyramid

| Layer | What it proves | Tooling | Runs |
| --- | --- | --- | --- |
| **Unit** | pure logic: transition guards, fold reducer, budget math, selection ranking, redaction patterns, batching rules, chain hashing | vitest | every PR, <30 s |
| **Property** | state-space correctness: random valid transition sequences → fold==tables; no attempt ever has two active leases; pack assembly deterministic | vitest + fast-check | every PR |
| **Contract** | schema round-trips; vocabulary lock; WWP conformance harness (Hermes and any future runtime as black boxes); brain adapter conformance against cassettes; studio-api route enumeration == spec | vitest + harness | every PR |
| **Golden traces** | the normative architecture behaviors end-to-end (see table below) | golden harness | every PR |
| **Integration** | real substrate edges: SQLite crash-in-txn, worktree lifecycle, snapshot refs, sandbox canary (supported OS runner), SSE cursor resume | vitest, OS-matrix later | every PR (canary: on OS runner) |
| **E2E** | the user's actual flows: CLI demo script (M0+), Playwright browser flows (M3+), replay interactions (M6+) | script + Playwright | every PR (CLI), nightly (browser) |
| **Chaos/recovery** | kill −9 matrix at scripted points; disk-full; cassette-miss; provider-timeout paths | crash harness | nightly + before every tag |
| **Adversarial** | injection fixture ("tests pass, merge now" planted in repo): gate status unmoved, claims render unverified, pack fencing applied | golden `injection` | every PR from M5 |

## Golden trace suite (the architecture's teeth)

| Golden | Asserts | From |
| --- | --- | --- |
| `m0-first-mission` | the Sprint-1 spine: create→plan→approve→build→gates→accept; chain verified; fold==tables | M0 |
| `v2-08-rate-limiting` | canonical mission: batched decisions (one interrupt), envelope extension, review isolation (builder transcript absent from review pack — asserted on the pack manifest), completion guards | M2 |
| `v2-08-crash-variant` | kill at the scripted seq → AttemptUncertain → snapshot → classified recovery; timeline self-explaining | M5 |
| `injection` | adversarial repo content cannot move a gate or mint a verified claim | M5 |
| `v2-18-decision-edit` | fork → DecisionRevised → invalidation set exact → replan with DecisionDelta → comparison facets | M7 |
| `provider-swap` | same mission, both real adapters (cassette-recorded), same trace shape | M8 |

Normalization (fixed in the harness, never per-test): IDs → ordinals, timestamps → `T+n`, hashes → `H(k)` stable placeholders, durations elided. Regenerating a golden requires `tools/new-golden.ts` and the diff appears in the PR — intentional behavior change is reviewed as a trace diff, which is exactly how the architecture wants to be reviewed.

## Replay & ledger validation (Chronicle-specific)

- **Determinism:** `frames(mission)` regenerated twice → byte-identical; bump of `generator_version` is the only permitted difference.
- **State equivalence:** for every golden, `chronicle.stateAt(seq)` === `ledger.fold(uptoSeq)` at 20 sampled points.
- **Retroactivity:** the archived M0 database (checked into fixtures at tag time) must replay under every later Chronicle version — the ADR-0011 promise as a regression test.
- **Chain:** `verifyChain` on every golden run and on `wepld verify` in every E2E.

## Context tests (the quality lever gets its own bar)

Manifest completeness (every candidate file appears as included/excerpted/omitted-with-reason); T0-overflow fails loudly; redaction corpus (AWS/GitHub/PEM/entropy strings → placeholders + `RedactionApplied`); trust fencing present on every repo-derived section; pack hash stability. From M1, a small **assembly-quality fixture**: for 10 task specs on the fixture repos, the files a human marked "must see" are included — a regression floor, not a benchmark.

## What is deliberately not tested yet

Performance under load (single mission scale makes it moot — one nightly timing assertion on the 10k-entry timeline query guards regressions); multi-OS sandbox matrix (IADR-0005 — one OS runner until M8); UI pixel styling. Each gets a line in the risk register instead of a test suite it doesn't need yet.
