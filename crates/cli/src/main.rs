//! `wepld` — CLI over the Core. Presentation only: every mutation goes
//! through the Core; every read comes from ledger-backed queries.

mod demo;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;
use wepld_contracts::command::{Command, CommandOutcome};
use wepld_runtime::{command_id_for, Core};

#[derive(Parser)]
#[command(
    name = "wepld",
    version,
    about = "WePLD — the Operating System for Autonomous Engineering"
)]
struct Cli {
    /// Store directory (default: $WEPLD_HOME or ~/.wepld)
    #[arg(long, global = true)]
    store: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create (or open) the operational store
    Init,
    /// Mission operations
    Mission {
        #[command(subcommand)]
        cmd: MissionCmd,
    },
    /// Plan operations
    Plan {
        #[command(subcommand)]
        cmd: PlanCmd,
    },
    /// Print a mission's ledger timeline
    Timeline { mission_id: String },
    /// Verify the ledger hash chain
    Verify,
    /// Run the full M0 bounded loop on a bundled fixture (self-contained)
    Demo,
}

#[derive(Subcommand)]
enum MissionCmd {
    /// Create a mission from a brief file (structured JSON, never chat)
    New {
        #[arg(short = 'f', long)]
        file: PathBuf,
    },
    /// Run the planner phase for a draft mission
    Plan { mission_id: String },
    /// Execute a running mission's tasks (build + gates)
    Run { mission_id: String },
    /// Accept a proposed completion (optionally merging into the base branch)
    Accept {
        mission_id: String,
        /// Merge the final snapshot into the base branch
        #[arg(long)]
        merge: bool,
    },
}

#[derive(Subcommand)]
enum PlanCmd {
    /// Approve the proposed plan and materialize its tasks
    Approve { mission_id: String },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<ExitCode, Box<dyn std::error::Error>> {
    // The demo is self-contained (its own scratch store); handle it before
    // touching the default operational store.
    if matches!(cli.cmd, Cmd::Demo) {
        demo::run(locate_hermes())?;
        return Ok(ExitCode::SUCCESS);
    }

    let dir = store_dir(cli.store)?;
    let mut core = Core::open(&dir)?;
    core.set_worker_cmd(locate_hermes());

    match cli.cmd {
        Cmd::Init => {
            let entries = core.all_entries()?;
            let tier = entries
                .first()
                .map(|e| e.payload_json["tier"].as_str().unwrap_or("?").to_owned())
                .unwrap_or_else(|| "?".to_owned());
            println!("store ready at {}", dir.display());
            println!(
                "sandbox tier: {tier} — no isolation; Manual mode and fixture repositories only"
            );
        }
        Cmd::Mission { cmd } => match cmd {
            MissionCmd::New { file } => {
                let payload: serde_json::Value =
                    serde_json::from_str(&std::fs::read_to_string(&file)?)?;
                let command = Command {
                    command_id: command_id_for("create_mission", &payload),
                    command_type: "create_mission".to_owned(),
                    actor: "principal_local".to_owned(),
                    payload,
                };
                let outcome = core.submit(&command)?;
                if !report(&outcome, "mission") {
                    return Ok(ExitCode::FAILURE);
                }
            }
            MissionCmd::Plan { mission_id } => {
                let outcome = core.plan_mission(&mission_id)?;
                if !report(&outcome, "plan") {
                    return Ok(ExitCode::FAILURE);
                }
            }
            MissionCmd::Run { mission_id } => {
                let outcome = core.run_mission(&mission_id)?;
                if !report(&outcome, "run") {
                    return Ok(ExitCode::FAILURE);
                }
            }
            MissionCmd::Accept { mission_id, merge } => {
                let outcome = core.accept_mission(&mission_id, merge)?;
                if !report(&outcome, "acceptance") {
                    return Ok(ExitCode::FAILURE);
                }
            }
        },
        Cmd::Plan { cmd } => match cmd {
            PlanCmd::Approve { mission_id } => {
                let outcome = core.approve_plan(&mission_id)?;
                if !report(&outcome, "approval") {
                    return Ok(ExitCode::FAILURE);
                }
            }
        },
        Cmd::Timeline { mission_id } => {
            let entries = core.timeline(&mission_id)?;
            if entries.is_empty() {
                println!("no entries for {mission_id}");
                return Ok(ExitCode::FAILURE);
            }
            for e in &entries {
                let mut summary = e.payload_json.to_string();
                if summary.len() > 72 {
                    summary.truncate(69);
                    summary.push_str("...");
                }
                println!(
                    "{:>5}  {:<26} {:<16} {}",
                    e.seq,
                    e.entry_type.code(),
                    e.actor_id,
                    summary
                );
            }
        }
        Cmd::Verify => {
            let report = core.verify()?;
            match report.broken_at {
                None => println!("chain VERIFIED — {} entries", report.total),
                Some(seq) => {
                    println!("chain BROKEN at seq {seq} (of {} entries)", report.total);
                    return Ok(ExitCode::FAILURE);
                }
            }
        }
        Cmd::Demo => unreachable!("handled before store open"),
    }
    Ok(ExitCode::SUCCESS)
}

/// Print a command outcome; return false if it was not accepted.
fn report(outcome: &CommandOutcome, noun: &str) -> bool {
    match outcome {
        CommandOutcome::Accepted { detail } => {
            let state = detail["state"].as_str().unwrap_or("");
            let extra = if detail.get("task_count").is_some() {
                format!(" ({} task(s))", detail["task_count"])
            } else {
                String::new()
            };
            println!("ACCEPTED  {noun} → {state}{extra}");
            true
        }
        CommandOutcome::Rejected { reason } => {
            println!("REJECTED  {reason}");
            false
        }
        other => {
            println!("{other:?}");
            true
        }
    }
}

/// The `hermes` binary lives next to `wepld` (same Cargo target dir / install).
fn locate_hermes() -> Vec<String> {
    let exe = std::env::current_exe().ok();
    if let Some(dir) = exe.as_ref().and_then(|e| e.parent()) {
        let candidate = dir.join(if cfg!(windows) {
            "hermes.exe"
        } else {
            "hermes"
        });
        if candidate.exists() {
            return vec![candidate.to_string_lossy().into_owned()];
        }
    }
    vec!["hermes".to_owned()]
}

fn store_dir(explicit: Option<PathBuf>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(d) = explicit {
        return Ok(d);
    }
    if let Ok(d) = std::env::var("WEPLD_HOME") {
        return Ok(PathBuf::from(d));
    }
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| "cannot determine home directory; pass --store")?;
    Ok(PathBuf::from(home).join(".wepld"))
}
