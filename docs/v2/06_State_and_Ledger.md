# v2-06 — State and Ledger

**Decision (ADR-0003):** transactional state tables + append-only audit ledger, written in the same SQLite transaction by a single transition function. Current state lives in tables; history, audit, timeline, replay, and Messenger narration live in the ledger. Full event sourcing is a documented promotion path, not an MVP cost.

## Storage layout

One SQLite database (WAL, `synchronous=FULL`, local filesystem only — never a synced folder; Core refuses to initialize its store inside detected OneDrive/Dropbox/iCloud paths and says why), plus a content-addressed artifact directory (`objects/ab/cdef…`, write-once, hash-verified on read).

### State tables (owners in parentheses)

| Table | Key columns beyond id/timestamps |
| --- | --- |
| `missions` (mission-state) | brief_json, state, autonomy_mode, budget_json, envelope_json, sandbox_tier, revision |
| `plans` | mission_id, version, plan_json (tasks, criteria matrix), approved_by, approved_at |
| `tasks` | mission_id, plan_version, spec_json, state, sequence_no |
| `attempts` | task_id, phase, role_profile, state, worktree_path, base_commit, envelope_json, context_pack_artifact, idempotency_key, hypothesis (retries) |
| `decisions` | mission_id, packet_json, class, state, resolution_json, resolved_by, resolved_at |
| `artifacts` | hash, kind, media_type, size, classification, producer (attempt/core), path |
| `brain_invocations` | attempt_id, profile, provider, model, pack_hash, response_artifact, tokens_in, tokens_out, cost_estimate, latency_ms, outcome |
| `knowledge_records` | type (decision/lesson/finding), title, body, tags, source_artifacts, supersedes, status |
| `skills` | name, version, hash, path, enabled |
| `commands` | command_id (unique), type, actor, payload_hash, outcome, processed_at |
| `work_queue` | kind, payload_json, status, not_before, attempts |

FTS5 virtual tables index `knowledge_records` and ledger payload text for search.

### Ledger

~~~sql
CREATE TABLE ledger (
  seq            INTEGER PRIMARY KEY,          -- authoritative local order
  entry_id       TEXT NOT NULL UNIQUE,         -- ULID
  ts_utc         TEXT NOT NULL,
  entry_type     TEXT NOT NULL,                -- past-tense fact, closed vocabulary (v2-07)
  schema_version INTEGER NOT NULL,
  aggregate_type TEXT NOT NULL,                -- mission|task|attempt|decision|system
  aggregate_id   TEXT NOT NULL,
  actor_type     TEXT NOT NULL,                -- human|core|worker|brain_adapter
  actor_id       TEXT NOT NULL,
  correlation_id TEXT NOT NULL,                -- mission-level trace
  causation_ref  TEXT,                         -- command_id or parent entry_id
  payload_json   TEXT NOT NULL,                -- minimal fact; big bodies by artifact ref
  payload_hash   TEXT NOT NULL,
  prev_hash      TEXT NOT NULL,                -- hash chain: H(prev_hash || payload_hash || entry_id)
  entry_hash     TEXT NOT NULL
);
~~~

Properties: **append-only** (no UPDATE/DELETE grants in application code; a trigger rejects both), **hash-chained** for tamper evidence (a background verifier and every export re-walk the chain), **envelope-complete** (all v1 event-envelope fields present), **body-light** (payloads reference artifacts by hash; nothing large or secret enters the ledger — redaction happens upstream in Context Assembly and artifact classification).

## The single-writer transition function

`apply(current, input) → (mutations, entries[])` is the only code path that writes state tables or ledger, executed in one transaction. Consequences:

- State and narrative cannot diverge silently: a **fold-checker** replays the ledger through a pure reducer and diffs the result against tables — run in CI on every scenario fixture and on daemon startup (fast: single mission scale). Divergence is a `SEV-1` defect and a startup warning, never auto-repaired quietly.
- Crash consistency is SQLite's transaction guarantee; there is no window where state changed but history didn't.

## Reads

- **Timeline** = ledger scan by correlation_id with artifact hydration on demand.
- **Studio live updates** = SSE stream of ledger tail (client keeps `last_seq`; reconnect resumes from cursor; cursor too old → full refetch signal — v1's resync semantics, trivially).
- **Messenger narration** and **replay** read the ledger only — they cannot know anything the audit trail doesn't, which is exactly the honesty property v1 wanted from projections.

## Retention and deletion

Artifacts carry classification and retention class; deletion tombstones an artifact (content removed, hash + tombstone reason retained) so the chain stays verifiable while the body is gone — v1's tombstone semantics at MVP scale. The ledger itself is retained; its payloads were body-light by construction.

## Promotion path to full event sourcing (when multi-writer/sync demands it)

1. Declare `ledger` authoritative per aggregate; add `expected_revision` enforcement per aggregate (column exists on missions already).
2. Generate state tables as projections via the existing fold-checker reducer (it *is* the projection function, already tested by CI).
3. Add an outbox consumer for remote distribution (the `work_queue` pattern generalizes).
4. Upcasters begin at that point — not before — with history fully preserved because the envelope fields were there from day one.

What is *not* promised: re-deriving pre-promotion state from events alone. Pre-promotion history is attested (hash-chained facts) rather than re-executable. That is sufficient for audit, timeline, and replay — the properties the principles actually demand.
