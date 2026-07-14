//! The SQLite store: state tables + hash-chained append-only ledger, written
//! in the same transaction (ADR-0003). `Tx` is the single-writer surface —
//! per IMPL-02, only the runtime's transition function may hold one.

use crate::error::LedgerError;
use chrono::{SecondsFormat, Utc};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use sha2::{Digest, Sha256};
use std::path::Path;
use wepld_contracts::ledger::{ActorType, AggregateType, LedgerEntry};
use wepld_contracts::vocabulary::EventType;

/// prev_hash of the first entry.
pub const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS ledger (
  seq            INTEGER PRIMARY KEY,
  entry_id       TEXT NOT NULL UNIQUE,
  ts_utc         TEXT NOT NULL,
  entry_type     TEXT NOT NULL,
  schema_version INTEGER NOT NULL,
  aggregate_type TEXT NOT NULL,
  aggregate_id   TEXT NOT NULL,
  actor_type     TEXT NOT NULL,
  actor_id       TEXT NOT NULL,
  correlation_id TEXT NOT NULL,
  causation_ref  TEXT,
  payload_json   TEXT NOT NULL,
  payload_hash   TEXT NOT NULL,
  prev_hash      TEXT NOT NULL,
  entry_hash     TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS ledger_correlation ON ledger (correlation_id, seq);
CREATE TRIGGER IF NOT EXISTS ledger_no_update BEFORE UPDATE ON ledger
BEGIN SELECT RAISE(ABORT, 'ledger is append-only'); END;
CREATE TRIGGER IF NOT EXISTS ledger_no_delete BEFORE DELETE ON ledger
BEGIN SELECT RAISE(ABORT, 'ledger is append-only'); END;

CREATE TABLE IF NOT EXISTS missions (
  mission_id TEXT PRIMARY KEY,
  title      TEXT NOT NULL,
  state      TEXT NOT NULL,
  brief_json TEXT NOT NULL,
  revision   INTEGER NOT NULL DEFAULT 1
);
CREATE TABLE IF NOT EXISTS plans (
  plan_id     TEXT PRIMARY KEY,
  mission_id  TEXT NOT NULL,
  version     INTEGER NOT NULL,
  plan_json   TEXT NOT NULL,
  approved_by TEXT,
  approved_at TEXT
);
CREATE TABLE IF NOT EXISTS tasks (
  task_id     TEXT PRIMARY KEY,
  mission_id  TEXT NOT NULL,
  spec_json   TEXT NOT NULL,
  state       TEXT NOT NULL,
  sequence_no INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS attempts (
  attempt_id            TEXT PRIMARY KEY,
  task_id               TEXT NOT NULL,
  phase                 TEXT NOT NULL,
  role_profile          TEXT NOT NULL,
  state                 TEXT NOT NULL,
  worktree_path         TEXT,
  base_commit           TEXT,
  envelope_json         TEXT NOT NULL,
  context_pack_artifact TEXT,
  idempotency_key       TEXT NOT NULL,
  hypothesis            TEXT
);
CREATE TABLE IF NOT EXISTS decisions (
  decision_id     TEXT PRIMARY KEY,
  mission_id      TEXT NOT NULL,
  packet_json     TEXT NOT NULL,
  class           TEXT NOT NULL,
  state           TEXT NOT NULL,
  resolution_json TEXT,
  resolved_by     TEXT,
  resolved_at     TEXT
);
CREATE TABLE IF NOT EXISTS commands (
  command_id   TEXT PRIMARY KEY,
  type         TEXT NOT NULL,
  actor        TEXT NOT NULL,
  payload_hash TEXT NOT NULL,
  outcome      TEXT NOT NULL,
  processed_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS brain_invocations (
  invocation_id     TEXT PRIMARY KEY,
  attempt_id        TEXT NOT NULL,
  profile           TEXT NOT NULL,
  provider          TEXT NOT NULL,
  model             TEXT NOT NULL,
  intent            TEXT NOT NULL,
  pack_hash         TEXT NOT NULL,
  response_artifact TEXT,
  status            TEXT NOT NULL,
  tokens_in         INTEGER NOT NULL,
  tokens_out        INTEGER NOT NULL,
  cost_usd          REAL NOT NULL,
  latency_ms        INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS work_queue (
  id           INTEGER PRIMARY KEY,
  kind         TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  status       TEXT NOT NULL DEFAULT 'ready',
  not_before   TEXT,
  attempts     INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS lessons (
  lesson_id  TEXT PRIMARY KEY,
  repo       TEXT NOT NULL,
  mission_id TEXT NOT NULL,
  spec_id    TEXT,
  title      TEXT NOT NULL,
  body       TEXT NOT NULL,
  gates_json TEXT NOT NULL,
  files_json TEXT NOT NULL,
  confidence REAL NOT NULL,
  status     TEXT NOT NULL DEFAULT 'candidate',
  created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS lessons_repo ON lessons (repo, created_at);
";

pub struct LedgerStore {
    conn: Connection,
    idgen: ulid::Generator,
}

/// A pending entry; `seq`, timestamps, ids, and hashes are assigned at append.
pub struct NewEntry {
    pub entry_type: EventType,
    pub schema_version: u32,
    pub aggregate_type: AggregateType,
    pub aggregate_id: String,
    pub actor_type: ActorType,
    pub actor_id: String,
    pub correlation_id: String,
    pub causation_ref: Option<String>,
    pub payload: serde_json::Value,
}

pub struct AppendedRef {
    pub seq: i64,
    pub entry_id: String,
    pub entry_hash: String,
}

/// One recorded reasoning invocation (v2-07 §3 invocation record; the pack
/// is referenced by content hash — record once, derive forever).
#[derive(Debug, Clone)]
pub struct BrainInvocationRow {
    pub invocation_id: String,
    pub attempt_id: String,
    pub profile: String,
    pub provider: String,
    pub model: String,
    pub intent: String,
    pub pack_hash: String,
    pub response_artifact: Option<String>,
    pub status: String,
    pub tokens_in: u64,
    pub tokens_out: u64,
    pub cost_usd: f64,
    pub latency_ms: u64,
}

/// A recorded engineering lesson — durable, evidence-based memory that makes
/// Hermes better over time. Scoped to a repo so future work retrieves it.
#[derive(Debug, Clone)]
pub struct LessonRow {
    pub lesson_id: String,
    pub repo: String,
    pub mission_id: String,
    pub spec_id: Option<String>,
    pub title: String,
    pub body: String,
    /// Reusable verification recipes learned: (gate, command).
    pub gates_json: String,
    pub files_json: String,
    pub confidence: f64,
    pub status: String,
    pub created_at: String,
}

/// A task row (Orchestration-owned unit of work).
#[derive(Debug, Clone)]
pub struct TaskRow {
    pub task_id: String,
    pub mission_id: String,
    pub spec_json: String,
    pub state: String,
    pub sequence_no: i64,
}

/// Chain verification result. `broken_at == None` means every hash checks out.
pub struct ChainReport {
    pub total: u64,
    pub broken_at: Option<i64>,
}

impl ChainReport {
    pub fn is_valid(&self) -> bool {
        self.broken_at.is_none()
    }
}

/// The single-writer transactional surface (state mutation + ledger append in
/// one SQLite transaction). Constructed only by [`LedgerStore::transact`].
pub struct Tx<'a> {
    inner: &'a rusqlite::Transaction<'a>,
    idgen: &'a mut ulid::Generator,
}

impl LedgerStore {
    pub fn open(dir: &Path) -> Result<Self, LedgerError> {
        refuse_synced_paths(dir)?;
        std::fs::create_dir_all(dir)?;
        let conn = Connection::open(dir.join("wepld.db"))?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "FULL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        // If another process holds the write lock, wait rather than failing
        // instantly (the single-writer design means this is transient).
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self {
            conn,
            idgen: ulid::Generator::new(),
        })
    }

    /// Run `f` inside one immediate transaction; commit on Ok, roll back on Err.
    pub fn transact<T>(
        &mut self,
        f: impl FnOnce(&mut Tx) -> Result<T, LedgerError>,
    ) -> Result<T, LedgerError> {
        let idgen = &mut self.idgen;
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let out = {
            let mut t = Tx { inner: &tx, idgen };
            f(&mut t)
        }?;
        tx.commit()?;
        Ok(out)
    }

    pub fn last_seq(&self) -> Result<i64, LedgerError> {
        let seq: Option<i64> = self
            .conn
            .query_row("SELECT MAX(seq) FROM ledger", [], |r| r.get(0))?;
        Ok(seq.unwrap_or(0))
    }

    pub fn entries_for(&self, correlation_id: &str) -> Result<Vec<LedgerEntry>, LedgerError> {
        self.query_entries(
            "SELECT seq, entry_id, ts_utc, entry_type, schema_version, aggregate_type,
                    aggregate_id, actor_type, actor_id, correlation_id, causation_ref,
                    payload_json, payload_hash, prev_hash, entry_hash
             FROM ledger WHERE correlation_id = ?1 ORDER BY seq",
            &[correlation_id],
        )
    }

    pub fn all_entries(&self) -> Result<Vec<LedgerEntry>, LedgerError> {
        self.query_entries(
            "SELECT seq, entry_id, ts_utc, entry_type, schema_version, aggregate_type,
                    aggregate_id, actor_type, actor_id, correlation_id, causation_ref,
                    payload_json, payload_hash, prev_hash, entry_hash
             FROM ledger ORDER BY seq",
            &[],
        )
    }

    /// Re-walk the whole chain: payload hashes, link hashes, and continuity.
    pub fn verify_chain(&self) -> Result<ChainReport, LedgerError> {
        let mut stmt = self.conn.prepare(
            "SELECT seq, entry_id, payload_json, payload_hash, prev_hash, entry_hash
             FROM ledger ORDER BY seq",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, String>(5)?,
            ))
        })?;

        let mut prev = GENESIS_HASH.to_string();
        let mut total: u64 = 0;
        for row in rows {
            let (seq, entry_id, payload_json, payload_hash, prev_hash, entry_hash) = row?;
            total += 1;
            let expect_payload = hex(&Sha256::digest(payload_json.as_bytes()));
            let expect_entry = link_hash(&prev, &payload_hash, &entry_id);
            if payload_hash != expect_payload || prev_hash != prev || entry_hash != expect_entry {
                return Ok(ChainReport {
                    total,
                    broken_at: Some(seq),
                });
            }
            prev = entry_hash;
        }
        Ok(ChainReport {
            total,
            broken_at: None,
        })
    }

    /// Stored (payload_hash, outcome_json) of a processed command, if any —
    /// the idempotency lookup (v2-02 §2 step 1).
    pub fn command_record(
        &self,
        command_id: &str,
    ) -> Result<Option<(String, String)>, LedgerError> {
        Ok(self
            .conn
            .query_row(
                "SELECT payload_hash, outcome FROM commands WHERE command_id = ?1",
                [command_id],
                |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
            )
            .optional()?)
    }

    pub fn brain_invocations(
        &self,
        attempt_id: &str,
    ) -> Result<Vec<BrainInvocationRow>, LedgerError> {
        let mut stmt = self.conn.prepare(
            "SELECT invocation_id, attempt_id, profile, provider, model, intent, pack_hash,
                    response_artifact, status, tokens_in, tokens_out, cost_usd, latency_ms
             FROM brain_invocations WHERE attempt_id = ?1 ORDER BY invocation_id",
        )?;
        let rows = stmt.query_map([attempt_id], |r| {
            Ok(BrainInvocationRow {
                invocation_id: r.get(0)?,
                attempt_id: r.get(1)?,
                profile: r.get(2)?,
                provider: r.get(3)?,
                model: r.get(4)?,
                intent: r.get(5)?,
                pack_hash: r.get(6)?,
                response_artifact: r.get(7)?,
                status: r.get(8)?,
                tokens_in: r.get::<_, i64>(9)? as u64,
                tokens_out: r.get::<_, i64>(10)? as u64,
                cost_usd: r.get(11)?,
                latency_ms: r.get::<_, i64>(12)? as u64,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// Current state of an attempt row, if it exists.
    pub fn attempt_state(&self, attempt_id: &str) -> Result<Option<String>, LedgerError> {
        Ok(self
            .conn
            .query_row(
                "SELECT state FROM attempts WHERE attempt_id = ?1",
                [attempt_id],
                |r| r.get::<_, String>(0),
            )
            .optional()?)
    }

    /// The stored mission brief (as submitted), if the mission exists.
    pub fn mission_brief(
        &self,
        mission_id: &str,
    ) -> Result<Option<serde_json::Value>, LedgerError> {
        let text: Option<String> = self
            .conn
            .query_row(
                "SELECT brief_json FROM missions WHERE mission_id = ?1",
                [mission_id],
                |r| r.get(0),
            )
            .optional()?;
        match text {
            Some(t) => Ok(Some(serde_json::from_str(&t)?)),
            None => Ok(None),
        }
    }

    /// The latest plan (plan_id, plan_json) for a mission, if any.
    pub fn latest_plan(
        &self,
        mission_id: &str,
    ) -> Result<Option<(String, serde_json::Value)>, LedgerError> {
        let row: Option<(String, String)> = self
            .conn
            .query_row(
                "SELECT plan_id, plan_json FROM plans WHERE mission_id = ?1
                 ORDER BY version DESC LIMIT 1",
                [mission_id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        match row {
            Some((id, json)) => Ok(Some((id, serde_json::from_str(&json)?))),
            None => Ok(None),
        }
    }

    /// All lessons recorded for a repository, newest last — Engineering Memory
    /// retrieval for future missions.
    pub fn lessons_for_repo(&self, repo: &str) -> Result<Vec<LessonRow>, LedgerError> {
        let mut stmt = self.conn.prepare(
            "SELECT lesson_id, repo, mission_id, spec_id, title, body, gates_json, files_json,
                    confidence, status, created_at
             FROM lessons WHERE repo = ?1 ORDER BY created_at, lesson_id",
        )?;
        let rows = stmt.query_map([repo], |r| {
            Ok(LessonRow {
                lesson_id: r.get(0)?,
                repo: r.get(1)?,
                mission_id: r.get(2)?,
                spec_id: r.get(3)?,
                title: r.get(4)?,
                body: r.get(5)?,
                gates_json: r.get(6)?,
                files_json: r.get(7)?,
                confidence: r.get(8)?,
                status: r.get(9)?,
                created_at: r.get(10)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn tasks_for_mission(&self, mission_id: &str) -> Result<Vec<TaskRow>, LedgerError> {
        let mut stmt = self.conn.prepare(
            "SELECT task_id, mission_id, spec_json, state, sequence_no
             FROM tasks WHERE mission_id = ?1 ORDER BY sequence_no",
        )?;
        let rows = stmt.query_map([mission_id], |r| {
            Ok(TaskRow {
                task_id: r.get(0)?,
                mission_id: r.get(1)?,
                spec_json: r.get(2)?,
                state: r.get(3)?,
                sequence_no: r.get(4)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// Current (title, state) of a mission row, if it exists.
    pub fn mission_row(&self, mission_id: &str) -> Result<Option<(String, String)>, LedgerError> {
        Ok(self
            .conn
            .query_row(
                "SELECT title, state FROM missions WHERE mission_id = ?1",
                [mission_id],
                |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
            )
            .optional()?)
    }

    fn query_entries(&self, sql: &str, args: &[&str]) -> Result<Vec<LedgerEntry>, LedgerError> {
        let mut stmt = self.conn.prepare(sql)?;
        let mut out = Vec::new();
        let mut rows = stmt.query(rusqlite::params_from_iter(args.iter()))?;
        while let Some(r) = rows.next()? {
            let entry_type: String = r.get(3)?;
            let aggregate_type: String = r.get(5)?;
            let actor_type: String = r.get(7)?;
            let payload_text: String = r.get(11)?;
            out.push(LedgerEntry {
                seq: r.get(0)?,
                entry_id: r.get(1)?,
                ts_utc: r.get(2)?,
                entry_type: from_wire(&entry_type)?,
                schema_version: r.get(4)?,
                aggregate_type: from_wire(&aggregate_type)?,
                aggregate_id: r.get(6)?,
                actor_type: from_wire(&actor_type)?,
                actor_id: r.get(8)?,
                correlation_id: r.get(9)?,
                causation_ref: r.get(10)?,
                payload_json: serde_json::from_str(&payload_text)?,
                payload_hash: r.get(12)?,
                prev_hash: r.get(13)?,
                entry_hash: r.get(14)?,
            });
        }
        Ok(out)
    }
}

impl Tx<'_> {
    /// Append one fact; hashes chain automatically.
    pub fn append(&mut self, e: &NewEntry) -> Result<AppendedRef, LedgerError> {
        let prev_hash: String = self
            .inner
            .query_row(
                "SELECT entry_hash FROM ledger ORDER BY seq DESC LIMIT 1",
                [],
                |r| r.get(0),
            )
            .optional()?
            .unwrap_or_else(|| GENESIS_HASH.to_string());

        let entry_id = self
            .idgen
            .generate()
            .map_err(|_| LedgerError::IdGeneration)?
            .to_string();
        let ts_utc = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let payload_text = e.payload.to_string();
        let payload_hash = hex(&Sha256::digest(payload_text.as_bytes()));
        let entry_hash = link_hash(&prev_hash, &payload_hash, &entry_id);

        self.inner.execute(
            "INSERT INTO ledger (entry_id, ts_utc, entry_type, schema_version, aggregate_type,
                                 aggregate_id, actor_type, actor_id, correlation_id, causation_ref,
                                 payload_json, payload_hash, prev_hash, entry_hash)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)",
            params![
                entry_id,
                ts_utc,
                e.entry_type.code(),
                e.schema_version,
                to_wire(&e.aggregate_type),
                e.aggregate_id,
                to_wire(&e.actor_type),
                e.actor_id,
                e.correlation_id,
                e.causation_ref,
                payload_text,
                payload_hash,
                prev_hash,
                entry_hash,
            ],
        )?;
        Ok(AppendedRef {
            seq: self.inner.last_insert_rowid(),
            entry_id,
            entry_hash,
        })
    }

    pub fn insert_mission(
        &mut self,
        mission_id: &str,
        title: &str,
        state: &str,
        brief: &serde_json::Value,
    ) -> Result<(), LedgerError> {
        self.inner.execute(
            "INSERT INTO missions (mission_id, title, state, brief_json) VALUES (?1,?2,?3,?4)",
            params![mission_id, title, state, brief.to_string()],
        )?;
        Ok(())
    }

    pub fn set_mission_state(&mut self, mission_id: &str, state: &str) -> Result<(), LedgerError> {
        self.inner.execute(
            "UPDATE missions SET state = ?2, revision = revision + 1 WHERE mission_id = ?1",
            params![mission_id, state],
        )?;
        Ok(())
    }

    pub fn insert_plan(
        &mut self,
        plan_id: &str,
        mission_id: &str,
        version: i64,
        plan_json: &serde_json::Value,
    ) -> Result<(), LedgerError> {
        self.inner.execute(
            "INSERT INTO plans (plan_id, mission_id, version, plan_json) VALUES (?1,?2,?3,?4)",
            params![plan_id, mission_id, version, plan_json.to_string()],
        )?;
        Ok(())
    }

    pub fn approve_plan_row(&mut self, plan_id: &str, approver: &str) -> Result<(), LedgerError> {
        let ts = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        self.inner.execute(
            "UPDATE plans SET approved_by = ?2, approved_at = ?3 WHERE plan_id = ?1",
            params![plan_id, approver, ts],
        )?;
        Ok(())
    }

    pub fn insert_task(
        &mut self,
        task_id: &str,
        mission_id: &str,
        spec_json: &serde_json::Value,
        state: &str,
        sequence_no: i64,
    ) -> Result<(), LedgerError> {
        self.inner.execute(
            "INSERT INTO tasks (task_id, mission_id, spec_json, state, sequence_no)
             VALUES (?1,?2,?3,?4,?5)",
            params![
                task_id,
                mission_id,
                spec_json.to_string(),
                state,
                sequence_no
            ],
        )?;
        Ok(())
    }

    pub fn set_task_state(&mut self, task_id: &str, state: &str) -> Result<(), LedgerError> {
        self.inner.execute(
            "UPDATE tasks SET state = ?2 WHERE task_id = ?1",
            params![task_id, state],
        )?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_lesson(&mut self, row: &LessonRow) -> Result<(), LedgerError> {
        self.inner.execute(
            "INSERT INTO lessons (lesson_id, repo, mission_id, spec_id, title, body,
                    gates_json, files_json, confidence, status, created_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
            params![
                row.lesson_id,
                row.repo,
                row.mission_id,
                row.spec_id,
                row.title,
                row.body,
                row.gates_json,
                row.files_json,
                row.confidence,
                row.status,
                row.created_at,
            ],
        )?;
        Ok(())
    }

    pub fn insert_attempt(
        &mut self,
        attempt_id: &str,
        task_id: &str,
        phase: &str,
        role_profile: &str,
        envelope_json: &str,
        idempotency_key: &str,
    ) -> Result<(), LedgerError> {
        self.inner.execute(
            "INSERT INTO attempts (attempt_id, task_id, phase, role_profile, state, envelope_json, idempotency_key)
             VALUES (?1,?2,?3,?4,'spawned',?5,?6)",
            params![attempt_id, task_id, phase, role_profile, envelope_json, idempotency_key],
        )?;
        Ok(())
    }

    pub fn set_attempt_state(&mut self, attempt_id: &str, state: &str) -> Result<(), LedgerError> {
        self.inner.execute(
            "UPDATE attempts SET state = ?2 WHERE attempt_id = ?1",
            params![attempt_id, state],
        )?;
        Ok(())
    }

    pub fn record_brain_invocation(&mut self, row: &BrainInvocationRow) -> Result<(), LedgerError> {
        self.inner.execute(
            "INSERT INTO brain_invocations (invocation_id, attempt_id, profile, provider, model,
                    intent, pack_hash, response_artifact, status, tokens_in, tokens_out, cost_usd, latency_ms)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
            params![
                row.invocation_id,
                row.attempt_id,
                row.profile,
                row.provider,
                row.model,
                row.intent,
                row.pack_hash,
                row.response_artifact,
                row.status,
                row.tokens_in as i64,
                row.tokens_out as i64,
                row.cost_usd,
                row.latency_ms as i64,
            ],
        )?;
        Ok(())
    }

    /// Record a processed command (same transaction as its effects — v2-02 §2).
    pub fn record_command(
        &mut self,
        command_id: &str,
        command_type: &str,
        actor: &str,
        payload_hash: &str,
        outcome_json: &str,
    ) -> Result<(), LedgerError> {
        let ts = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        self.inner.execute(
            "INSERT INTO commands (command_id, type, actor, payload_hash, outcome, processed_at)
             VALUES (?1,?2,?3,?4,?5,?6)",
            params![
                command_id,
                command_type,
                actor,
                payload_hash,
                outcome_json,
                ts
            ],
        )?;
        Ok(())
    }
}

/// Hash payload text for command idempotency comparisons.
pub fn payload_hash(payload: &serde_json::Value) -> String {
    hex(&Sha256::digest(payload.to_string().as_bytes()))
}

fn link_hash(prev_hash: &str, payload_hash: &str, entry_id: &str) -> String {
    hex(&Sha256::digest(
        format!("{prev_hash}{payload_hash}{entry_id}").as_bytes(),
    ))
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn to_wire<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_value(v)
        .ok()
        .and_then(|x| x.as_str().map(str::to_owned))
        .unwrap_or_default()
}

fn from_wire<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, LedgerError> {
    Ok(serde_json::from_value(serde_json::Value::String(
        s.to_owned(),
    ))?)
}

fn refuse_synced_paths(dir: &Path) -> Result<(), LedgerError> {
    const SYNCED: [&str; 4] = ["onedrive", "dropbox", "icloud", "google drive"];
    for comp in dir.components() {
        let c = comp.as_os_str().to_string_lossy().to_lowercase();
        if SYNCED.iter().any(|s| c.contains(s)) {
            return Err(LedgerError::SyncedFolder(dir.to_path_buf()));
        }
    }
    Ok(())
}
