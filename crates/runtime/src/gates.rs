//! Gate execution (v2-10 gate evidence). A gate is a shell command run in the
//! task worktree; its exit code and captured output are the evidence. Under
//! the DEV tier the Core runs gates directly (IADR-0003, disclosed); the real
//! sandbox at M4 will run them in a validator envelope without changing this
//! result shape.

use std::path::Path;
use std::process::Command;

pub struct GateResult {
    pub passed: bool,
    pub exit_code: Option<i32>,
    pub log: String,
}

/// Run one gate command with the worktree as its working directory. A missing
/// command is a failed gate, not a panic (the mission under-specified it).
pub fn run_gate(worktree: &Path, command: &str) -> GateResult {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(worktree)
        .output();
    match output {
        Ok(out) => {
            let mut log = String::from_utf8_lossy(&out.stdout).into_owned();
            log.push_str(&String::from_utf8_lossy(&out.stderr));
            GateResult {
                passed: out.status.success(),
                exit_code: out.status.code(),
                log,
            }
        }
        Err(e) => GateResult {
            passed: false,
            exit_code: None,
            log: format!("gate command failed to launch: {e}"),
        },
    }
}
