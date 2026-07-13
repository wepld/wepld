//! Contract round-trip tests: the normative v2-07 examples must parse and
//! re-serialize losslessly. Value-level equality catches missing fields,
//! renamed fields, and spelling drift in one assertion.

use wepld_contracts::mission::MissionBrief;
use wepld_contracts::wwp::WwpMessage;

const MISSION_FIXTURE: &str = include_str!("fixtures/mission_rate_limiting.json");

#[test]
fn mission_brief_roundtrips_losslessly() {
    let original: serde_json::Value = serde_json::from_str(MISSION_FIXTURE).unwrap();
    let brief: MissionBrief = serde_json::from_value(original.clone()).unwrap();
    let back = serde_json::to_value(&brief).unwrap();
    assert_eq!(original, back);
}

#[test]
fn wwp_heartbeat_roundtrips() {
    let raw = serde_json::json!({
        "method": "heartbeat",
        "params": { "attempt_id": "att_01J8R0AB0000000000000000", "progress": "implementing token bucket" }
    });
    let msg: WwpMessage = serde_json::from_value(raw.clone()).unwrap();
    let back = serde_json::to_value(&msg).unwrap();
    assert_eq!(raw, back);
}

#[test]
fn wwp_phase_result_roundtrips() {
    let raw = serde_json::json!({
        "method": "phase.result",
        "params": {
            "attempt_id": "att_01J8R0AB0000000000000000",
            "status": "succeeded",
            "outputs": [{ "artifact": "art_01", "kind": "diff" }],
            "evidence": [{ "artifact": "art_02", "kind": "worklog" }],
            "summary": { "schema": "phase_summary.v1", "what": "added middleware" }
        }
    });
    let msg: WwpMessage = serde_json::from_value(raw.clone()).unwrap();
    let back = serde_json::to_value(&msg).unwrap();
    assert_eq!(raw, back);
}

#[test]
fn json_schema_export_works() {
    let schema = schemars::schema_for!(MissionBrief);
    let v = serde_json::to_value(&schema).unwrap();
    assert!(v.is_object(), "exported JSON-Schema must be an object");
}
