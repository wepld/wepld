//! Engineering Memory — how Hermes accumulates engineering experience (not
//! prompts). When a mission is accepted, an evidence-based **lesson** is
//! extracted from its own ledger facts (the verification recipes that passed,
//! the files it touched, its confidence), stored durably, and made available
//! to future missions on the same repo. This closes the loop the founder
//! requires: every completed mission leaves the codebase, Hermes, and the
//! Engineering Memory better. Deterministic and evidence-based — never
//! invented (the lesson cites the mission's own facts).

use crate::{Core, RuntimeError};
use wepld_contracts::ledger::{ActorType, AggregateType};
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::{LessonRow, NewEntry};

#[derive(Debug, Clone)]
pub struct RecordedLesson {
    pub lesson_id: String,
    pub title: String,
    pub gates_learned: usize,
    pub files_touched: usize,
}

impl Core {
    /// Extract and record an engineering lesson from an accepted mission.
    /// Returns `None` for missions that did not reach acceptance (no lesson to
    /// learn yet). Idempotent per mission (the lesson id is derived).
    pub fn record_engineering_experience(
        &mut self,
        mission_id: &str,
    ) -> Result<Option<RecordedLesson>, RuntimeError> {
        let report = self.engineering_report(mission_id)?;
        if report.state != "accepted" {
            return Ok(None);
        }
        let brief = match self.store.mission_brief(mission_id)? {
            Some(b) => b,
            None => return Ok(None),
        };
        let repo = brief["scope"]["repo"].as_str().unwrap_or("").to_owned();

        // Evidence: the verification recipes that passed, and the diff.
        let mut gates_learned: Vec<(String, String)> = Vec::new();
        let mut diff_hash: Option<String> = None;
        for e in &self.timeline(mission_id)? {
            match e.entry_type {
                EventType::GateEvaluated if e.payload_json["status"] == "passed" => {
                    gates_learned.push((
                        e.payload_json["gate"].as_str().unwrap_or("").to_owned(),
                        e.payload_json["command"].as_str().unwrap_or("").to_owned(),
                    ));
                }
                EventType::ArtifactRecorded if e.payload_json["kind"] == "diff" => {
                    diff_hash = e.payload_json["hash"].as_str().map(str::to_owned);
                }
                _ => {}
            }
        }
        let files_touched = match diff_hash {
            Some(h) => parse_changed_files(&self.artifact(&h)?),
            None => vec![],
        };

        let gate_names: Vec<&str> = gates_learned.iter().map(|(g, _)| g.as_str()).collect();
        let body = format!(
            "Feature \"{}\" was implemented and verified in this project. \
             Files touched: {}. Verified by gate(s): {}. Confidence {:.0}% (evidence-derived).",
            report.title,
            if files_touched.is_empty() {
                "-".to_owned()
            } else {
                files_touched.join(", ")
            },
            if gate_names.is_empty() {
                "-".to_owned()
            } else {
                gate_names.join(", ")
            },
            report.confidence * 100.0
        );
        let lesson_id = format!("lesson_{mission_id}");
        let row = LessonRow {
            lesson_id: lesson_id.clone(),
            repo,
            mission_id: mission_id.to_owned(),
            spec_id: report.spec_id.clone(),
            title: format!("{}: implemented & verified", report.title),
            body: body.clone(),
            gates_json: serde_json::to_string(&gates_learned)?,
            files_json: serde_json::to_string(&files_touched)?,
            confidence: report.confidence,
            status: "candidate".to_owned(),
            created_at: now_millis(),
        };

        let body_ref = self.cas().put(body.as_bytes())?;
        let mid = mission_id.to_owned();
        let lid = lesson_id.clone();
        let title = row.title.clone();
        let payload = serde_json::json!({
            "lesson_id": lid, "title": title, "confidence": report.confidence,
            "gates_learned": gates_learned, "files_touched": files_touched,
            "body_ref": body_ref.hash, "status": "candidate"
        });
        self.store_mut().transact(|tx| {
            tx.insert_lesson(&row)?;
            tx.append(&NewEntry {
                entry_type: EventType::InsightRecorded,
                schema_version: 1,
                aggregate_type: AggregateType::Mission,
                aggregate_id: mid.clone(),
                actor_type: ActorType::Core,
                actor_id: "core".to_owned(),
                correlation_id: mid,
                causation_ref: None,
                payload,
            })?;
            Ok(())
        })?;

        Ok(Some(RecordedLesson {
            lesson_id,
            title: row.title,
            gates_learned: gates_learned.len(),
            files_touched: files_touched.len(),
        }))
    }

    /// Lessons recorded for a repo — Engineering Memory for future missions.
    pub fn lessons_for_repo(&self, repo: &str) -> Result<Vec<LessonRow>, RuntimeError> {
        Ok(self.store.lessons_for_repo(repo)?)
    }
}

/// Extract the changed file paths from a unified git diff (the `+++ b/…` lines).
fn parse_changed_files(diff: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(diff);
    let mut files = Vec::new();
    for line in text.lines() {
        if let Some(p) = line.strip_prefix("+++ b/") {
            let p = p.trim().to_owned();
            if !files.contains(&p) {
                files.push(p);
            }
        }
    }
    files
}

fn now_millis() -> String {
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    // Zero-padded for lexicographic ordering in the lessons index.
    format!("{ms:020}")
}

#[cfg(test)]
mod tests {
    use super::parse_changed_files;

    #[test]
    fn parses_changed_files_from_a_diff() {
        let diff = b"diff --git a/src/main.rs b/src/main.rs\n--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1 +1 @@\n+x\n";
        assert_eq!(parse_changed_files(diff), vec!["src/main.rs".to_owned()]);
    }
}
