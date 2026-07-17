# IMPL-07 — Implementation Risk Register

Owner for every row: the founder. Review: at every milestone tag. Likelihood/Impact on a High/Med/Low scale; every row has a trigger (observable, not vibes) and a mitigation that exists in this program (not a wish).

## Program-level risks

| Risk | L | I | Trigger | Mitigation |
| --- | --- | --- | --- | --- |
| **Solo bus factor / burnout** | M | Critical | two consecutive weeks without a merged PR | sustainable pace baked into the timeline (4.5 d/wk, pessimistic bands); demo-Friday rhythm makes progress emotionally visible; everything is written down (this program + DECISIONS.md) so a pause isn't a loss |
| **Scope creep breaking the freeze** | H | H | a PR introduces a concept with no v2 anchor | gap-note rule (IMPL-00); dependency-cruiser + vocabulary-lock make many drifts mechanical failures; backlog file absorbs ideas without acting on them |
| **Model quality insufficient for planner/builder on real repos** | M | H | M1 missions need >2 retries or hand-holding on fixture repos | fixture repos are small by design; Manual mode is the fallback product posture; E1-lite at M5.5 informs topology; thesis readout (Phase C) is the honest arbiter — this risk is *the* research risk and cannot be engineered away, only measured early |
| **Market timing** (incumbents ship governance surfaces) | M | H | a major vendor ships mission-control + evidence gating | speed via this program's cuts; moat is the ledger/Chronicle substrate (v2-16 §5) — keep every milestone shippable so partial value exists at all times |
| **Token spend during development** | M | M | monthly bill > budget line | cassettes for all CI and most dev loops (IADR-0002); local model via openai-compat adapter for cheap iterations; budget guard is a product feature that also protects the founder |
| **better-sqlite3 / native module friction** | M | L | install failure on founder or partner machine | pinned prebuilds; `node:sqlite` fallback documented; store code isolated in `ledger` behind one interface |

## Per-milestone risks

| M | Top risks | Mitigation |
| --- | --- | --- |
| M0 | WWP plumbing edge cases (stdio buffering, zombie processes); hash-chain design mistakes ossifying early | LSP-style framing (well-trodden); process-group kill discipline from day 5; chain format reviewed against v2-06 before Day 2 ends — it is the one thing hard to change later |
| M1 | real-provider structured output flakier than cassettes suggest; context selection quality poor on first contact | schema-retry path is contract behavior (v2-07 §3); assembly-quality fixture (IMPL-06) gives an objective floor; keep tasks small — mission design is a product lever, not just a test lever |
| M2 | interrupt/batching semantics fiddly; review-isolation accidentally leaky via shared temp state | v2-10 semantics are already precise — implement the table, not intuitions; isolation asserted structurally (review pack manifest must not contain builder transcript — a test, not a hope) |
| M3 | UI time sink (the classic) | ship ugly on purpose; component budget: 3 surfaces, ~10 components; Playwright only for the golden path; a week over budget triggers scope cut, not a push |
| M4 | founder-OS sandbox reality (Seatbelt quirks / bwrap availability / WSL2 setup); envelope breaks real toolchains | canary-first development (make the test fail honestly before making it pass); S0 container fallback is the pressure valve; DEV tier remains for development itself (IADR-0003) |
| M5 | recovery classification wrong in an edge case → silent duplicate effect | the crash matrix is scripted at the *six* risk points (mid-txn, post-commit-pre-spawn, mid-phase, mid-gate, mid-merge, mid-fork); envelope confinement (ADR-0004) keeps the common case mechanically probeable |
| M6 | frame semantics churn (taste-driven rework) | generator_version discipline: churn is cheap regeneration, not migration; timebox the frame-rules iteration to one week, refine with real usage later |
| M7 | invalidation cone correctness (false carries / over-invalidation) | MVP carries nothing (ADR-0012 stance) — over-invalidation is the safe default and the golden asserts the exact set; salvage stays V1 |
| M8 | onboarding friction with strangers; second-OS surprise | install-time doctor command (checks git, node, tier prerequisites); cohort filtered to supported OS or S0 per IADR-0005 — stated openly in the invite |

## V0 Build Feature slice risks (post-review, PR #1)

| Risk | L | I | Trigger | Mitigation |
| --- | --- | --- | --- | --- |
| **Prompt injection via Engineering Memory** | M | H | a lesson's observation contains instruction-like text | *residual risk, not eliminated:* memory is structurally separated (labelled `untrusted-context`, structured fields, `MEMORY_POLICY` system instruction, schema-validated output); it grants no capability and changes no acceptance criteria; every effect stays behind independent approval + gates ([[09_Engineering_Memory_Contract]] §5) |
| **DEV tier read as sandboxed** | M | H | a mission runs on a non-throwaway repo assuming isolation | no OS containment is *enforced under DEV*; caps are Manual-only + fixtures-only + explicit `--i-understand-dev-tier` override, all disclosed and ledger-recorded; the Envelope is descriptive, not enforced ([[10_V0_Governance_Safety_and_Limits]] §3) |
| **Repo-identity edge cases** | L | M | a reused path or exotic filesystem confuses memory scope | identity = canonical common dir + root commit; relative/absolute/case resolve alike; reinit → new identity (no inherited lessons); it is a local fingerprint, not a global GUID — no "never leaks" claim |
| **Credential exposure via provider adapter** | L | H | an API key or HTTPS endpoint is configured | local-loopback-only build: keys over HTTP and all HTTPS are refused with a typed error; no key reaches a request/log/Debug; hosted/keyed support deferred to a verified-TLS build |
| **Acceptance effect/ledger inconsistency on crash** | L | H | a crash between acceptance decision and effect/record | intent-before-effect + idempotent proposal-ref + probe + explicit uncertain state; fault-injection tests cover both crash points; no false `MissionAccepted`, no base mutation |

## V0 security-boundary risks (final remediation)

| Risk | L | I | Trigger | Mitigation |
| --- | --- | --- | --- | --- |
| **Model edit path escapes the worktree** | M | H | a builder edit uses `..`, an absolute path, a symlink, or a concurrent path swap | edit paths are validated `WorkspaceRelativePath` (Component-based, not substring) **and** written through a handle-relative, no-follow capability boundary (`openat` + `O_NOFOLLOW`/`O_DIRECTORY`, `mkdirat`, `fstat` regular-file check) — not a metadata check plus path open, so a concurrent swap cannot redirect the write; non-Unix fails closed ([[10_V0_Governance_Safety_and_Limits]] §5) |
| **Untrusted id becomes path/ref syntax** | M | H | a slug/mission/task/base value contains separators, `..`, `@{`, or a leading `-` | central validation contracts reject them as deterministic recorded rejections before persistence; plan output is validated semantically; workspace independently refuses unsafe attempt ids and resolves base refs to a commit with `--end-of-options` |
| **Recoverable acceptance mistaken for failure** | L | M | a proposal-ref conflict or interrupted acceptance | preserved as `RecipeOutcome::Deferred` (durable decision, not final, no merge) — never flattened to `Rejected`; the CLI surfaces the distinction |
| **Silent canonicalization fallback** | L | M | a fixtures root or repo cannot be canonicalized | `set_fixtures_root` and `project_fingerprint` fail closed — a failed update never clears/weakens the prior authorization, and an unborn repo yields `NoRootCommit` |

## V0 integrity & resource-safety risks (final remediation)

| Risk | L | I | Trigger | Mitigation |
| --- | --- | --- | --- | --- |
| **DEV override authorized but unrecorded (or lost on restart)** | L | H | a storage failure during override activation, or a process restart after a grant | activation is **ledger-atomic**: the override fact commits first and in-memory authorization is set only after commit; a failed transaction leaves authorization unchanged (fault seam proves it); `restore_dev_override` reconstructs the latest recorded override at `Core::open` for the authenticated grantor only ([[10_V0_Governance_Safety_and_Limits]] §3) |
| **A returned completion silently reported as completed** | L | H | a reviewer declines completion (`approve=false`) | the return path is a first-class `RecipeOutcome::Returned` recording `MissionReturned` — it never falls through to a completion report, creates no proposal ref, and records no lesson; Accepted→Returned, Rejected→Rejected, Deferred→Deferred ([[10_V0_Governance_Safety_and_Limits]] §6) |
| **Resource exhaustion via oversized model payload** | M | M | a model emits a flood of edits, an enormous file, or a huge plan | deterministic caps (`MAX_EDITS_PER_STEP`, `MAX_BYTES_PER_EDIT`, `MAX_TOTAL_EDIT_BYTES`, `MAX_PLAN_TASKS`, `MAX_TASK_TITLE_BYTES`, `MAX_SATISFIES_PER_TASK`, `MAX_TOTAL_PLAN_BYTES`) reject the payload at the boundary; an edit batch is prevalidated in full before the first write (no partial application); a plan is bounded before persistence ([[10_V0_Governance_Safety_and_Limits]] §5) |
| **Partial edit application on a rejected batch** | L | M | edit *k* is valid, edit *k+1* escapes or exceeds a bound | the whole batch is prevalidated (count, per-edit + overflow-checked aggregate bytes, duplicate normalized paths) **before any write**, so a batch that fails validation writes nothing |

## Standing unknowns (tracked, not blocked on)

Whether solo pace sustains the expected band (checkpoint: M3 tag date vs. plan — slip >2 wk forces a scope council with self); whether design partners accept fixture-repo onboarding before their own repos (checkpoint: M8 cohort feedback); whether the openai-compat local path is good enough to matter for privacy-sensitive partners (checkpoint: M1 adapter conformance results); Seatbelt deprecation horizon (checkpoint: each macOS release — S0 fallback ready per ADR-0007).
