# v2-12 — The Replay Engine

The engine that turns the ledger into something watchable, seekable, and queryable. Everything here is derived and rebuildable (ADR-0011).

## 1. Core abstractions

### Frame

The unit of replay is not a ledger entry — entries are facts; frames are *moments*. A frame groups one or more entries into a coherent beat a human can watch:

~~~json
{ "schema_version": 1, "frame_id": "frm_mis…F_00041",
  "mission_id": "mis_…F", "seq_range": [3013, 3015], "type": "scene",
  "title": "Builder requests the governor crate and hits the key-vs-IP ambiguity",
  "actors": [{"type":"worker","role":"builder","attempt":"att_b1"}],
  "tracks": { "execution": true, "decisions": true, "brain": true },
  "state_ref": { "at_seq": 3015 },
  "workspace_ref": "refs/wepld/mis…F/att_b1/build-snap",
  "focus": [
    { "kind": "pack_section", "artifact": "art_cp2", "section": "T2" },
    { "kind": "escalation", "entry": 3015 } ],
  "media": [{ "kind": "diff_partial", "artifact": "art_…" }],
  "duration_hint_ms": 4000 }
~~~

Frame types: `moment` (one significant entry), `scene` (a folded span — a phase's inner work condensed, heartbeats and routine artifact puts summarized), `beat` (human interaction: decision delivery/resolution). Generation is **deterministic and versioned**: a pure function `frames(entries[], generator_version) → frames[]` driven by per-entry-type rules (significant types → `moment`/`beat`; noise types fold into the enclosing `scene`). Regenerating with the same version yields identical frames — cache-safe, test-safe.

### Lens

A lens is a **typed view definition**, not just a filter: which entry types it includes, how they fold into frames, and what "state" means on its track.

| Lens | Includes | Its notion of state |
| --- | --- | --- |
| Decisions | DecisionRequested/Resolved/Revised, PlanApproved, EnvelopeExtension* | the open-decision set + interrupt budget |
| Evidence | GateEvaluated, ArtifactRecorded(kind∈evidence), CheckCompleted | the acceptance-criteria matrix status |
| Execution | Task/Attempt/Phase transitions, Recovery* | the mission/task state machine |
| Brain | BrainInvoked (+pack refs) | cumulative cost/tokens; provider mix |
| Context | pack manifests per invocation | what was visible/omitted at this moment |
| Code | WorkspaceSnapshotRecorded, diffs | the workspace at the snapshot ref |
| Policy | envelope grants/denials, tier detection, redactions | the effective envelope |
| User | commands by human principal, MessageSent | what the human saw and did |

MVP ships the first four as scrubber tracks; the rest exist as filters and graduate to tracks in V1. Lenses compose: the player renders the union of enabled tracks; each track can also be replayed alone (the Part-1 "replay only X" requirement is exactly a single-lens session).

### Checkpoint and `state_at`

`checkpoints(mission, seq, state_json, fold_version, hash)` written by the engine (not the runtime) every 500 entries and at phase boundaries. `state_at(mission, seq)` = nearest checkpoint ≤ seq, fold forward with the **same reducer the v2-06 fold-checker uses** — one reducer, three consumers (consistency check, projection promotion path, replay). Workspace at seq = nearest `WorkspaceSnapshotRecorded` ≤ seq (ADR-0013).

## 2. Replay sessions

A session is server-side state so any surface (Studio now, channels later) can drive playback consistently:

~~~json
{ "session_id": "rps_…", "mission_id": "mis_…F", "mode": "historical",  // historical | following
  "playhead": { "frame": "frm_…_00041", "seq": 3015 },
  "status": "paused",            // paused | playing | seeking | following | closed
  "speed": 1.0,                  // 0.25–16; negative = reverse playback
  "lenses": ["decisions","evidence","execution","brain"],
  "range": null }                // optional clip bounds
~~~

**Session state machine:** `CREATED → PAUSED ⇄ PLAYING ⇄ FOLLOWING → CLOSED`, with `SEEKING` as a transient from any active state. Rules: `FOLLOWING` is only reachable when the mission is live — the playhead locks to the ledger tail (SSE) and advances as facts commit; any manual seek **detaches** to `PAUSED` (the mission keeps running; a `LIVE +n` chip shows drift; "Jump to live" re-attaches). `PLAYING` advances frame-by-frame on `duration_hint × 1/speed` timers; reverse playback iterates frames backward — free by construction (ADR-0011). Historical and live missions therefore share one player, one session model, one frame stream.

Playback verbs (full API in v2-17): `play(speed)`, `pause()`, `step(±1)`, `seek(seq | frame | fraction)`, `follow()`, `clip(from,to)`.

## 3. Storage (all derived, all rebuildable)

| Table | Contents | Rebuild cost |
| --- | --- | --- |
| `frames` | cached generator output, keyed (mission, generator_version) | seconds per mission |
| `checkpoints` | folded state snapshots | seconds |
| `causal_edges` | forensic graph (v2-14) | seconds |
| `lineage` | fork tree: child, parent, fork_seq, reason | *not* derived — written at fork commit, but reconstructible from MissionForked entries |
| `insights` | intelligence findings (v2-16) | minutes (cross-mission scan) |
| `annotations` | user notes pinned to frames (V1) | source data — artifacts referenced from `AnnotationRecorded` entries |

Startup/repair rule: any derived table failing its version or hash check is dropped and regenerated. There is no "corrupt replay" support scenario — only "regenerating."

## 4. Performance and memory

Budget (MVP scale: ≤ 5k entries, ≤ 40 brain calls, ≤ 200 artifacts per mission):

| Operation | Target | Mechanism |
| --- | --- | --- |
| Cold open of a mission in Cinema | < 2 s | generate-and-cache frames; lazy hydrate |
| `state_at` anywhere | < 50 ms | checkpoint spacing ≤ 500 entries |
| Scrub (frame render) | < 100 ms | hydration window: playhead ± 20 frames materialized; outside window, track glyphs only |
| Historical workspace checkout | < 2 s typical | `git worktree add --detach` on snapshot ref |
| Live follow latency | ≤ SSE latency + frame fold | incremental generation on tail entries |

Memory: frames are metadata (< 2 KB each); artifacts stream from CAS on demand and are never held wholesale; large diffs render windowed. Large-repository strategy: Chronicle never scans repositories — it reads snapshot *refs* (Git materializes), pack *excerpts* (already bounded by v2-04), and artifacts (already content-addressed); repo size affects Chronicle only through Git's own performance.

## 5. Consistency, privacy, failure

- **Reads committed facts only.** A frame can never show state the ledger hasn't durably recorded; live following is eventually consistent with the tail and says so (`LIVE` chip semantics).
- **Classification enforcement unchanged:** Chronicle reads through the same authorized query layer as every Studio surface; a pack section or artifact above the viewer's clearance renders as the same policy-omission marker Context Assembly wrote — replay does not become a side door (secrets never entered the substrate anyway: body-light ledger + upstream redaction).
- **Export is a decision:** replay bundles (V1) are content-addressed tars {ledger slice, referenced artifacts, frames, manifest} with the hash chain re-verified at import; export re-applies redaction classes, requires explicit confirmation, and writes `ReplayExported` to the ledger — exporting history is itself history.
- **Offline:** everything is local; Chronicle has zero network dependencies. Synchronization (Future) ships bundles, not databases — content addressing makes them idempotent to merge into another Chronicle's CAS.
- **Failure recovery:** sessions are ephemeral (recreate at last playhead); derived stores regenerate; fork commits are single transactions in the Core (v2-15); a crash mid-export leaves a partial file that fails hash verification — safe by construction.
