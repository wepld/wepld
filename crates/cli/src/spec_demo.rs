//! `wepld spec demo` — the end-to-end Engineering Specification vertical slice,
//! self-contained: a canonical specification object → Mission Conversion →
//! Runtime execution (Hermes, deterministic cassette) → gates → accept+merge →
//! replayable ledger. No API keys, no network. Proves Specification → Mission →
//! Runtime → Hermes → Evidence → Ledger.

use std::error::Error;
use std::path::Path;
use std::process::Command;
use wepld_contracts::command::CommandOutcome;
use wepld_providers::{cassette_key, write_cassette_entry};
use wepld_runtime::{builder_pack, Core};
use wepld_specification::{
    convert, render, ConvertInput, SpecAcceptanceCriterion, SpecificationDocument,
};

const EDITED_MAIN: &str = "fn main() {\n    const VERSION: &str = \"0.1.0\";\n    if std::env::args().any(|a| a == \"--version\") {\n        println!(\"{VERSION}\");\n        return;\n    }\n    println!(\"notes-cli\");\n}\n";

const SLUG: &str = "version-flag";

/// The canonical specification object — authored in code to show that a spec
/// is an object, not a markdown file (markdown is one serialization).
fn spec_doc() -> SpecificationDocument {
    let mut d = SpecificationDocument {
        overview: "Add a --version flag to notes-cli".to_owned(),
        user_stories: vec![
            "As a user, I can run notes-cli --version to see the version".to_owned(),
        ],
        functional_requirements: vec![
            "Print the version and exit when --version is passed".to_owned()
        ],
        acceptance_criteria: vec![SpecAcceptanceCriterion {
            id: "AC1".to_owned(),
            text: "a VERSION constant is present".to_owned(),
            verify: "gate:build".to_owned(),
        }],
        required_skills: vec!["rust".to_owned()],
        ..Default::default()
    };
    d.verification
        .insert("build".to_owned(), "grep -q VERSION src/main.rs".to_owned());
    d
}

pub fn run(worker_cmd: Vec<String>) -> Result<(), Box<dyn Error>> {
    let scratch = std::env::temp_dir().join(format!("wepld-spec-demo-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    let repo = scratch.join("notes-cli");
    let store = scratch.join("store");
    std::fs::create_dir_all(repo.join("src"))?;
    std::fs::write(
        repo.join("src/main.rs"),
        "fn main() {\n    println!(\"notes-cli\");\n}\n",
    )?;
    git(&repo, &["init", "-q", "-b", "main"])?;
    git(&repo, &["config", "user.name", "notes-cli dev"])?;
    git(&repo, &["config", "user.email", "dev@local"])?;
    git(&repo, &["add", "-A"])?;
    git(&repo, &["commit", "-q", "-m", "initial"])?;
    let repo_str = repo.to_string_lossy().into_owned();

    let doc = spec_doc();

    println!("── WePLD Engineering Specification demo ──");
    println!("scratch: {}\n", scratch.display());
    println!("── the specification (canonical object, rendered to markdown) ──");
    println!("{}", render(&doc));

    // Record the builder cassette to match what run_mission will request. The
    // brief/plan come from the same pure conversion the Runtime uses.
    record_build_cassette(&store, &doc, &repo_str)?;

    let mut core = Core::open(&store)?;
    core.set_worker_cmd(worker_cmd);

    println!("── pipeline ──");
    step(
        "spec → mission (convert)",
        &core.create_mission_from_spec(&doc, SLUG, &repo_str, "main")?,
    );
    let mission_id = format!("mis_{SLUG}_v1");
    step("approve plan", &core.approve_plan(&mission_id)?);
    step("run (Hermes build + gate)", &core.run_mission(&mission_id)?);
    step("accept --merge", &core.accept_mission(&mission_id, true)?);

    println!("\n── specification timeline ──");
    for e in core.timeline(&format!("spec_{SLUG}"))? {
        println!("{:>4}  {:<24} {}", e.seq, e.entry_type.code(), e.actor_id);
    }
    println!("\n── mission timeline ──");
    for e in core.timeline(&mission_id)? {
        println!("{:>4}  {:<24} {}", e.seq, e.entry_type.code(), e.actor_id);
    }

    let report = core.verify()?;
    println!(
        "\nchain {} — {} entries",
        if report.is_valid() {
            "VERIFIED"
        } else {
            "BROKEN"
        },
        report.total
    );
    let merged = std::fs::read_to_string(repo.join("src/main.rs"))?;
    println!(
        "primary repo now has --version: {}",
        merged.contains("--version")
    );
    println!(
        "\nSpecification → Mission → Runtime → Hermes → Evidence → Ledger — one vertical slice."
    );
    Ok(())
}

fn record_build_cassette(
    store: &Path,
    doc: &SpecificationDocument,
    repo: &str,
) -> Result<(), Box<dyn Error>> {
    // The plan comes from conversion (no planner phase); only the build phase
    // consults the brain. Compute its pack the same way the Runtime will.
    let conv = convert(ConvertInput {
        doc,
        spec_id: &format!("spec_{SLUG}"),
        version: 1,
        document_hash: "unused-for-pack",
        slug: SLUG,
        repo,
        base_branch: "main",
        paths: vec!["src/**".to_owned()],
    })
    .map_err(|e| format!("convert failed: {e:?}"))?;

    let brief_json = serde_json::to_value(&conv.brief)?;
    let task_spec = serde_json::to_value(&conv.plan.tasks[0])?;
    let pack = builder_pack(&brief_json, &task_spec);
    let key = cassette_key(
        "build",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&pack)?),
        "builder_step.v1",
        "fixture-model",
    );
    let edits =
        serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": EDITED_MAIN } ] });
    write_cassette_entry(
        &store.join("cassettes/spec.jsonl"),
        &key,
        &edits,
        "fixture-model",
    )?;
    Ok(())
}

fn step(label: &str, outcome: &CommandOutcome) {
    match outcome {
        CommandOutcome::Accepted { detail } => {
            println!("✓ {label:<26} → {}", detail["state"].as_str().unwrap_or(""));
        }
        CommandOutcome::Rejected { reason } => println!("✗ {label:<26} REJECTED: {reason}"),
        other => println!("· {label:<26} {other:?}"),
    }
}

fn git(dir: &Path, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let out = Command::new("git").args(args).current_dir(dir).output()?;
    if !out.status.success() {
        return Err(format!("git {args:?}: {}", String::from_utf8_lossy(&out.stderr)).into());
    }
    Ok(())
}
