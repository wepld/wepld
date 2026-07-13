# v2-17 — Chronicle Contracts and API

Extends the v2-07 contract set. Versioning rules identical (semver, additive minors). The Event Contract's closed vocabulary is amended to **revision 2** with: `WorkspaceSnapshotRecorded · MissionForked · DecisionRevised · MissionSuperseded` (MVP) and `InsightRecorded · AnnotationRecorded · ReplayExported` (V1) — recorded here and in v2-07 §5.

## Contracts

**C1 Frame** — v2-12 §1. `{frame_id, mission_id, seq_range, type: moment|scene|beat, title, actors[], tracks{}, state_ref, workspace_ref?, focus[], media[], duration_hint_ms}`. Deterministic per `generator_version`.

**C2 Lens** — `{lens_id, entry_types[], fold_rules, state_definition, track_style}`. The eight standard lenses ship as data; custom lenses are V2.

**C3 Replay Session** — v2-12 §2. State machine `CREATED → PAUSED ⇄ PLAYING ⇄ FOLLOWING → CLOSED` (+ transient `SEEKING`); `FOLLOWING` legal only while the mission is live; detach-on-seek.

**C4 Checkpoint** — `{mission_id, seq, state_json, fold_version, hash}`. Derived; regenerated when `fold_version` changes.

**C5 Causal Edge** — ADR-0014. `{edge_id, from_ref, to_ref, edge_type, derived_by, confidence}`. `from_ref/to_ref` address entries (`seq`), artifacts (`art_`), decisions (`dec_`), pack sections (`art_…#T2/path`).

**C6 Lineage** — `{child_mission, parent_mission, fork_seq, reason, motive: decision_revision|what_if|recovery|learning, outcome: open|adopted|superseded|archived}`.

**C7 Decision Delta** — the T1 pack section injected after a revision: `{revises, old_option, new_option, rationale, invalidated:[refs], salvageable:[refs]}`.

**C8 Comparison** — v2-15 §6. `{a:{mission,seq}, b:{mission,seq}, facets:{files, decisions, reasoning, context, evidence, policy, state, behavior, outputs}}`; every facet's rows carry evidence refs.

**C9 RCA Report** — v2-14 §4. `{failure_ref, primary:{chain:[{cause_ref, class, confidence, evidence[], counterfactual_note}]}, alternates:[{…, why_lower}], generator:{rules_version, narration: verified|unverified}}`.

**C10 Insight / Bundle** — v2-16 §1 and: `Bundle = {manifest{mission, seq_range, lenses, redaction_classes_applied, hash_chain_root}, ledger_slice, artifacts[], frames[]}` — content-addressed tar; import re-verifies the chain; export writes `ReplayExported`.

## API

Read APIs are Studio-API extensions (loopback HTTP + SSE, session-token auth, classification-filtered — nothing new in the security model). Write operations are **Commands** through the standard v2-02 pipeline: `ForkMission · ReviseDecision · AdoptBranch · ArchiveBranch · PromoteInsight · DismissInsight · AnnotateFrame · ExportReplay`.

### Replay

~~~text
POST /replay/sessions                     {mission_id, lenses?, mode?}        → Session
POST /replay/sessions/:id/play            {speed}          # 0.25–16, negative = reverse
POST /replay/sessions/:id/pause
POST /replay/sessions/:id/step            {direction: +1|-1}
POST /replay/sessions/:id/seek            {seq | frame_id | fraction}
POST /replay/sessions/:id/follow                            # live missions only
POST /replay/sessions/:id/clip            {from_seq, to_seq}
GET  /replay/sessions/:id/stream                            # SSE: frame deltas at playhead
GET  /replay/missions/:id/frames          ?lens=&from_seq=&to_seq=&v=generator_version
GET  /replay/missions/:id/state           ?at_seq=          → state_at fold
GET  /replay/missions/:id/workspace       ?at_seq=          → snapshot ref (+ materialize=true → temp worktree path)
~~~

### Forensics

~~~text
GET  /forensics/cause                     ?ref=&depth=      → cause cone (chain-ordered)
GET  /forensics/impact                    ?ref=&weight=cost|artifacts|gates
POST /forensics/investigate               {template, params} → report ref   # templates of v2-14 §2
GET  /forensics/rca                       ?mission=&failure_ref=            → C9 report (V1)
GET  /forensics/divergence                ?a=&b=&lenses=                    → per-lens first divergence (V1)
~~~

### Branching and comparison

~~~text
command ForkMission                       {mission, at_seq, reason, motive}
command ReviseDecision                    {decision_id, new_option, rationale}   # sugar: fork + DecisionRevised + replan
command AdoptBranch / ArchiveBranch       {mission}
GET  /lineage/tree                        ?root=            → Branch Graph
GET  /lineage/decision                    ?decision=        → Decision Lineage across branches
POST /compare                             {a, b, facets[]}  → C8 document
~~~

### Intelligence and queries

~~~text
GET  /intelligence/stats                  ?mission=|project=&dimension=phase|provider|decision
GET  /intelligence/insights               ?status=&class=
command PromoteInsight / DismissInsight   {insight_id, reason?}
POST /query/replay                        {mission?, lens, predicate}       # "all frames where …"
POST /query/forensic                      {graph_pattern}                   # bounded causal-pattern match (V2)
POST /query/mission                       {filter}                          # missions by outcome/cost/branch metadata
~~~

Errors reuse the v2 taxonomy; every response carries `generator/fold/rules` versions so clients can detect stale derivations and request regeneration.

## Security summary

Chronicle adds **zero new write authority**: its only writes are the standard commands above, each policy-checked like any command (revision authority = original decision authority; export requires explicit confirmation and is ledger-recorded). Reads pass the existing classification filter; bundles re-apply redaction at export and re-verify hashes at import. Branch missions inherit classification from the parent brief. Forensic narration and insight prose obey the claims discipline — the analysis layer cannot mint authority, only cite it.
