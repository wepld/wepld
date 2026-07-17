//! Day-2 integration tests: chain integrity, fold==tables consistency,
//! append-only enforcement, tamper detection, synced-folder guard.

use wepld_contracts::ledger::{ActorType, AggregateType};
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::{fold_mission, LedgerError, LedgerStore, LessonRow, NewEntry};

fn open_store() -> (tempfile::TempDir, LedgerStore) {
    let dir = tempfile::tempdir().unwrap();
    let store = LedgerStore::open(dir.path()).unwrap();
    (dir, store)
}

fn entry(mission_id: &str, et: EventType, payload: serde_json::Value) -> NewEntry {
    NewEntry {
        entry_type: et,
        schema_version: 1,
        aggregate_type: AggregateType::Mission,
        aggregate_id: mission_id.to_owned(),
        actor_type: ActorType::Human,
        actor_id: "principal_local".to_owned(),
        correlation_id: mission_id.to_owned(),
        causation_ref: None,
        payload,
    }
}

fn create_mission(store: &mut LedgerStore, mission_id: &str, title: &str) {
    let e = entry(
        mission_id,
        EventType::MissionCreated,
        serde_json::json!({ "title": title }),
    );
    let t = title.to_owned();
    let id = mission_id.to_owned();
    store
        .transact(move |tx| {
            tx.insert_mission(&id, &t, "draft", &serde_json::json!({ "title": t }))?;
            tx.append(&e)?;
            Ok(())
        })
        .unwrap();
}

/// Forward mission transitions, in order. (EventType, resulting table state.)
const STEPS: [(EventType, &str); 4] = [
    (EventType::PlanProposed, "plan_review"),
    (EventType::PlanApproved, "running"),
    (EventType::CompletionProposed, "completion_proposed"),
    (EventType::MissionAccepted, "accepted"),
];

fn advance(store: &mut LedgerStore, mission_id: &str, step: usize) {
    let (et, state) = STEPS[step];
    let e = entry(mission_id, et, serde_json::json!({}));
    let id = mission_id.to_owned();
    store
        .transact(move |tx| {
            tx.append(&e)?;
            tx.set_mission_state(&id, state)?;
            Ok(())
        })
        .unwrap();
}

#[test]
fn full_lifecycle_chain_is_valid_and_folds_correctly() {
    let (_dir, mut store) = open_store();
    create_mission(&mut store, "mis_1", "Add version flag");
    for step in 0..STEPS.len() {
        advance(&mut store, "mis_1", step);
    }

    let report = store.verify_chain().unwrap();
    assert!(report.is_valid(), "chain must verify");
    assert_eq!(report.total, 5);

    let entries = store.entries_for("mis_1").unwrap();
    let folded = fold_mission(&entries).unwrap();
    assert_eq!(folded.state, "accepted");
    assert_eq!(folded.title, "Add version flag");

    let (title, state) = store.mission_row("mis_1").unwrap().unwrap();
    assert_eq!(
        (title.as_str(), state.as_str()),
        ("Add version flag", "accepted")
    );
}

#[test]
fn fold_matches_tables_over_randomized_histories() {
    let (_dir, mut store) = open_store();

    // Deterministic LCG — no rand dependency needed for a Day-2 property test.
    let mut rng: u64 = 42;
    let mut next = move || {
        rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (rng >> 33) as usize
    };

    let mut expected: Vec<(String, String)> = Vec::new();
    for i in 0..50 {
        let id = format!("mis_{i}");
        create_mission(&mut store, &id, &format!("Mission {i}"));
        let steps = next() % (STEPS.len() + 1); // 0..=4 forward transitions
        for s in 0..steps {
            advance(&mut store, &id, s);
        }
        let state = if steps == 0 {
            "draft"
        } else {
            STEPS[steps - 1].1
        };
        expected.push((id, state.to_owned()));
    }

    for (id, want) in &expected {
        let folded = fold_mission(&store.entries_for(id).unwrap()).unwrap();
        assert_eq!(&folded.state, want, "fold state for {id}");
        let (_, table_state) = store.mission_row(id).unwrap().unwrap();
        assert_eq!(&table_state, want, "table state for {id}");
        assert_eq!(folded.state, table_state, "fold == tables for {id}");
    }

    assert!(store.verify_chain().unwrap().is_valid());
}

#[test]
fn ledger_rejects_update_and_delete() {
    let (dir, mut store) = open_store();
    create_mission(&mut store, "mis_1", "t");

    // Out-of-band connection, as an attacker or buggy code would use.
    let raw = rusqlite::Connection::open(dir.path().join("wepld.db")).unwrap();
    let upd = raw.execute("UPDATE ledger SET payload_json = '{}' WHERE seq = 1", []);
    assert!(upd.is_err(), "append-only trigger must reject UPDATE");
    let del = raw.execute("DELETE FROM ledger WHERE seq = 1", []);
    assert!(del.is_err(), "append-only trigger must reject DELETE");
}

#[test]
fn tampering_is_detected_by_chain_verification() {
    let (dir, mut store) = open_store();
    create_mission(&mut store, "mis_1", "t");
    advance(&mut store, "mis_1", 0);

    // Simulate out-of-band tampering: drop the guard triggers, then mutate.
    let raw = rusqlite::Connection::open(dir.path().join("wepld.db")).unwrap();
    raw.execute_batch("DROP TRIGGER ledger_no_update;").unwrap();
    raw.execute(
        "UPDATE ledger SET payload_json = '{\"title\":\"forged\"}' WHERE seq = 1",
        [],
    )
    .unwrap();

    let report = store.verify_chain().unwrap();
    assert_eq!(report.broken_at, Some(1), "tampered entry must be detected");
}

#[test]
fn failed_transaction_rolls_back_atomically() {
    let (_dir, mut store) = open_store();
    create_mission(&mut store, "mis_1", "t");
    let before = store.last_seq().unwrap();

    // A transaction that appends and mutates, then errors, must persist nothing.
    let result: Result<(), LedgerError> = store.transact(|tx| {
        tx.set_mission_state("mis_1", "running")?;
        tx.append(&entry(
            "mis_1",
            EventType::PlanApproved,
            serde_json::json!({}),
        ))?;
        Err(LedgerError::IdGeneration)
    });
    assert!(result.is_err());
    assert_eq!(store.last_seq().unwrap(), before, "append rolled back");
    assert_eq!(
        store.mission_row("mis_1").unwrap().unwrap().1,
        "draft",
        "state mutation rolled back"
    );
    assert!(store.verify_chain().unwrap().is_valid());
}

fn lesson_row(id: &str, repo: &str) -> LessonRow {
    LessonRow {
        lesson_id: id.to_owned(),
        repo: repo.to_owned(),
        mission_id: "mis_1".to_owned(),
        spec_id: Some("spec_1".to_owned()),
        title: "t".to_owned(),
        body: "b".to_owned(),
        gates_json: "[]".to_owned(),
        files_json: "[]".to_owned(),
        confidence: 1.0,
        status: "candidate".to_owned(),
        created_at: "00000000000000000001".to_owned(),
        created_seq: 0,
    }
}

/// Record a lesson the way the runtime does: append the `InsightRecorded` fact,
/// stamp the row with that fact's ledger sequence, insert — one transaction.
fn record_lesson(store: &mut LedgerStore, id: &str, repo: &str) {
    let mut row = lesson_row(id, repo);
    store
        .transact(|tx| {
            let appended = tx.append(&entry(
                "mis_1",
                EventType::InsightRecorded,
                serde_json::json!({ "lesson_id": id }),
            ))?;
            row.created_seq = appended.seq;
            tx.insert_lesson(&row)?;
            Ok(())
        })
        .unwrap();
}

#[test]
fn lessons_persist_across_store_reopen_and_are_repo_scoped() {
    let dir = tempfile::tempdir().unwrap();
    {
        let mut store = LedgerStore::open(dir.path()).unwrap();
        record_lesson(&mut store, "lesson_mis_1", "repoA");
    } // store dropped — the database handle is closed.

    // A brand-new handle on the same directory still sees the lesson: durable
    // across process restarts, with provenance intact and scoped to its repo.
    let store = LedgerStore::open(dir.path()).unwrap();
    let a = store.lessons_for_repo("repoA").unwrap();
    assert_eq!(a.len(), 1);
    assert_eq!(a[0].lesson_id, "lesson_mis_1");
    assert!(
        a[0].created_seq > 0,
        "creation ledger seq persisted (provenance)"
    );
    assert!(store.lesson("lesson_mis_1").unwrap().is_some());
    assert!(
        store.lessons_for_repo("repoB").unwrap().is_empty(),
        "a lesson never leaks to another repository"
    );
}

#[test]
fn lesson_and_event_roll_back_together_on_failure() {
    let (_dir, mut store) = open_store();
    let before = store.last_seq().unwrap();
    let mut row = lesson_row("lesson_x", "repoA");

    // A record that fails after appending and inserting must persist neither.
    let res: Result<(), LedgerError> = store.transact(|tx| {
        let appended = tx.append(&entry(
            "mis_1",
            EventType::InsightRecorded,
            serde_json::json!({}),
        ))?;
        row.created_seq = appended.seq;
        tx.insert_lesson(&row)?;
        Err(LedgerError::IdGeneration)
    });
    assert!(res.is_err());
    assert_eq!(
        store.last_seq().unwrap(),
        before,
        "InsightRecorded rolled back"
    );
    assert!(
        store.lesson("lesson_x").unwrap().is_none(),
        "lesson row rolled back"
    );
    assert!(store.verify_chain().unwrap().is_valid());
}

#[test]
fn duplicate_lesson_id_leaves_no_orphan_event() {
    let (_dir, mut store) = open_store();
    record_lesson(&mut store, "lesson_dup", "repoA");
    let after_first = store.last_seq().unwrap();

    // Re-recording the same (derived) lesson id: the primary key rejects the
    // row and the event appended in the same transaction rolls back with it —
    // no duplicate lesson, no orphaned InsightRecorded.
    let mut row = lesson_row("lesson_dup", "repoA");
    let res: Result<(), LedgerError> = store.transact(|tx| {
        let appended = tx.append(&entry(
            "mis_1",
            EventType::InsightRecorded,
            serde_json::json!({}),
        ))?;
        row.created_seq = appended.seq;
        tx.insert_lesson(&row)?;
        Ok(())
    });
    assert!(res.is_err(), "duplicate lesson id must be rejected");
    assert_eq!(
        store.last_seq().unwrap(),
        after_first,
        "no orphan InsightRecorded"
    );
    assert_eq!(store.lessons_for_repo("repoA").unwrap().len(), 1);
    assert!(store.verify_chain().unwrap().is_valid());
}

#[test]
fn synced_folders_are_refused() {
    let err = LedgerStore::open(std::path::Path::new("/tmp/OneDrive/wepld-store"))
        .err()
        .unwrap();
    assert!(matches!(err, LedgerError::SyncedFolder(_)));
    let err = LedgerStore::open(std::path::Path::new("/tmp/Dropbox/x"))
        .err()
        .unwrap();
    assert!(matches!(err, LedgerError::SyncedFolder(_)));
}
