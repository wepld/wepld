# v2-04 — Context Assembly

**Status:** first-class Core subsystem (ADR-0006). The highest-leverage quality mechanism in the product, and therefore owned, versioned, tested, and captured — never improvised inside workers or adapters.

## Port

`assemble(mission_id, task_id, phase, role_profile, token_budget) → ContextPack`

Workers request packs; they never read raw sources into prompts themselves. Every pack is serialized, hashed, stored as an artifact, and referenced by every brain invocation that used it. That artifact — not a vibe — is what "replayability" means for reasoning: for any historical decision, the exact model input is retrievable.

## Pack structure — tiers with fixed priorities

| Tier | Content | Budget stance |
| --- | --- | --- |
| **T0 Pinned** | mission brief, acceptance criteria, phase instructions, role profile rules, resolved skill content, output schema | never truncated; if T0 > 30% of budget, assembly **fails loudly** (`ENVIRONMENT`) — an over-stuffed pinned tier is a configuration bug, not something to silently degrade |
| **T1 Task state** | approved plan, current task spec, prior phase results (summaries), gate outcomes so far, open uncertainties | summarized forms preferred; full artifacts by reference |
| **T2 Repository** | repo map (tree with sizes/languages), selected file contents, current diff | the main flexible budget consumer |
| **T3 Knowledge** | Decision/Lesson/Finding records matching task tags/paths | small, capped share |
| **T4 History** | compressed summary of earlier attempts of this task (what was tried, what failed, why) | present only on retries |

Default budget split after T0: T1 20%, T2 60%, T3 10%, T4 10%, with unused share flowing to T2. Splits are role-profile parameters, not code.

## Selection mechanics (T2, the hard part)

MVP selection is deliberately deterministic and explainable — no embeddings:

1. **Seeds:** files named in the task spec; files under `scope.paths`; files changed in the current diff.
2. **Expansion:** for each seed, add direct import/include neighbors (language-aware where a parser exists, regex-import fallback otherwise), one hop.
3. **Mentions:** exact-identifier search (ripgrep-class) for symbols named in the task's acceptance criteria.
4. **Ranking:** seed > diff-adjacent > neighbor > mention; within rank, smaller and more recently modified first.
5. **Packing:** whole files while they fit; beyond that, targeted excerpts (matched regions ± context) with elided-marker honesty; the repo map always includes entries for files that were *considered but omitted*, so the model knows what it cannot see.

Every pack embeds its own **selection manifest** (files included/excerpted/omitted and why). This makes context quality debuggable and makes E1-style experiments (v2-09) comparable. Semantic retrieval, when it earns its way in (V2), becomes an additional ranking signal inside step 4 — the pack format does not change.

## Compression

Prior phases and attempts are stored twice at write time: the full artifact, and a short structured summary (produced by the phase itself as part of `phase.result`, schema-enforced — not an extra model call at assembly time). Assembly uses summaries by default and full artifacts only when budget allows. The Review phase's pack uses the diff and evidence, never the Builder transcript (ADR-0002 isolation).

## Redaction and trust labeling

Before a pack leaves Core toward any brain adapter:

1. **Secret scan:** known token formats (cloud keys, PATs, private key blocks) + high-entropy heuristics over every T2 inclusion; hits are replaced with `⟦REDACTED:kind⟧` and logged (`RedactionApplied`) — the ledger records *that* redaction occurred, never the secret.
2. **Classification filter:** artifacts whose classification exceeds the brain profile's allowance are excluded and listed in the manifest as policy-omitted, so degraded context is visible rather than silent.
3. **Trust labeling:** repository-derived and tool-output content is wrapped in fenced sections labeled as untrusted data. Phase instructions state that such content is evidence to analyze, never instructions to follow. This is the prompt-injection stance at the substrate level; the human-facing layer is v2-10.

## Capture and replay

`ContextPack = {pack_id, schema_version, tier_sections[], selection_manifest, redaction_log_ref, hash}` — stored content-addressed. `brain_invocations` rows reference pack hash + response artifact. Replay (v2-08 §Replay) is honest about its nature: it **reconstructs** — exactly what was seen, asked, answered, done, decided — and does not claim deterministic *re-execution* against nondeterministic providers. Reconstruction is what audit, debugging, and trust actually require.

## Ownership and evolution

The pack format and capture pipeline are fixed contracts; selection strategies are pluggable per role profile. Future strategies (semantic retrieval, learned selection, cross-project knowledge in T3) compete inside the same measured harness: same pack schema, same manifests, same evaluation fixtures. This subsystem is where much of WePLD's long-term quality moat should accrete — treat its evaluation suite (Phase A deliverable) as a first-class product asset.
