//! `wepld` — CLI over the Core. Presentation only: every mutation goes
//! through `Core::submit`; every read comes from ledger-backed queries.

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
    /// Print a mission's ledger timeline
    Timeline { mission_id: String },
    /// Verify the ledger hash chain
    Verify,
}

#[derive(Subcommand)]
enum MissionCmd {
    /// Create a mission from a brief file (structured JSON, never chat)
    New {
        #[arg(short = 'f', long)]
        file: PathBuf,
    },
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
    let dir = store_dir(cli.store)?;
    let mut core = Core::open(&dir)?;

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
                match core.submit(&command)? {
                    CommandOutcome::Accepted { detail } => {
                        println!(
                            "ACCEPTED  mission {} → {}",
                            detail["mission_id"].as_str().unwrap_or("?"),
                            detail["state"].as_str().unwrap_or("?")
                        );
                    }
                    CommandOutcome::Rejected { reason } => {
                        println!("REJECTED  {reason}");
                        return Ok(ExitCode::FAILURE);
                    }
                    other => println!("{other:?}"),
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
    }
    Ok(ExitCode::SUCCESS)
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
