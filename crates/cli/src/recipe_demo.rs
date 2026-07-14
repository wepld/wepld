//! `wepld recipe demo` — the Build Feature recipe, end-to-end and
//! self-contained. The user states a feature; Hermes reasons a specification,
//! converts it to a mission, executes it, and returns an evidence-derived
//! Engineering Completion Report. Deterministic (cassettes); no keys, no
//! network. The user never sees the words specification, plan, or task.

use std::error::Error;
use std::path::Path;
use std::process::Command;
use wepld_runtime::{builder_pack, BuildFeatureReport, Core, RecipeOutcome};
use wepld_specification::{convert, ConvertInput, SpecAcceptanceCriterion, SpecificationDocument};

const REQUEST: &str = "Add a --version flag to notes-cli";
const SLUG: &str = "version-flag";
const EDITED_MAIN: &str = "fn main() {\n    const VERSION: &str = \"0.1.0\";\n    if std::env::args().any(|a| a == \"--version\") { println!(\"{VERSION}\"); return; }\n    println!(\"notes-cli\");\n}\n";

/// The specification Hermes "reasons" from the request (the specify cassette).
fn reasoned_spec() -> SpecificationDocument {
    let mut d = SpecificationDocument {
        overview: "Add a --version flag to notes-cli".to_owned(),
        user_stories: vec!["As a user, I can run notes-cli --version".to_owned()],
        functional_requirements: vec!["Print the version and exit on --version".to_owned()],
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
    let scratch = std::env::temp_dir().join(format!("wepld-recipe-demo-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    let repo = scratch.join("notes-cli");
    let store = scratch.join("store");
    std::fs::create_dir_all(repo.join("src"))?;
    std::fs::write(
        repo.join("src/main.rs"),
        "fn main() {\n    println!(\"notes-cli\");\n}\n",
    )?;
    for args in [
        &["init", "-q", "-b", "main"][..],
        &["config", "user.name", "dev"],
        &["config", "user.email", "dev@local"],
        &["add", "-A"],
        &["commit", "-q", "-m", "initial"],
    ] {
        git(&repo, args)?;
    }
    let repo_str = repo.to_string_lossy().into_owned();

    record_cassettes(&store, &repo_str)?;

    let mut core = Core::open(&store)?;
    core.set_worker_cmd(worker_cmd);

    println!("── WePLD · Engineering Recipe: Build Feature ──\n");
    println!("  You:    \"{REQUEST}\"\n");
    println!("  Hermes is engineering this feature…\n");

    match core.run_build_feature(REQUEST, SLUG, &repo_str, "main")? {
        RecipeOutcome::Completed(bf) => print_report(&bf, REQUEST),
        RecipeOutcome::NeedsClarification { questions, .. } => {
            println!("  Hermes needs clarification:");
            for q in questions {
                println!("    • {q}");
            }
        }
        RecipeOutcome::Rejected(reason) => println!("  Could not complete: {reason}"),
    }

    let merged = std::fs::read_to_string(repo.join("src/main.rs"))?;
    println!(
        "\n  (the feature is now on main: --version present = {})",
        merged.contains("--version")
    );
    Ok(())
}

fn print_report(bf: &BuildFeatureReport, feature: &str) {
    let r = &bf.report;
    let ck = |b: bool| if b { "✓" } else { "✗" };
    println!("  ┌─ Mission Complete ───────────────────────────");
    println!("  │ Feature        {feature}");
    println!(
        "  │ Specification  {} {}",
        r.spec_id.as_deref().unwrap_or("-"),
        ck(r.spec_id.is_some())
    );
    println!(
        "  │ Mission        {} {}",
        r.mission_id,
        ck(r.state == "accepted")
    );
    println!("  │ Implementation {}", ck(r.evidence_artifacts > 0));
    for (gate, passed) in &r.gates {
        println!("  │ Gate {:<9} {}", gate, ck(*passed));
    }
    println!(
        "  │ Gates          {}/{} passed",
        r.gates_passed(),
        r.gates.len()
    );
    println!(
        "  │ Criteria       {}/{} met",
        r.criteria_met(),
        r.criteria.len()
    );
    println!(
        "  │ Evidence       {} artifact(s), chain {}",
        r.evidence_artifacts,
        if r.chain_verified {
            "VERIFIED"
        } else {
            "BROKEN"
        }
    );
    println!("  │ Reasoning      {} brain call(s)", r.brain_calls);
    println!(
        "  │ Replay         {}",
        if r.replay_available {
            "Available"
        } else {
            "—"
        }
    );
    println!(
        "  │ Confidence     {:.0}% (evidence-derived)",
        r.confidence * 100.0
    );
    println!(
        "  │ Eng. Memory    ✓ {} learned · {} applied · {} total",
        bf.lessons_learned, bf.prior_lessons_applied, bf.total_memory
    );
    println!("  └──────────────────────────────────────────────");
    println!("\n  The codebase, Hermes, and the Engineering Memory are all better than before.");
}

fn record_cassettes(store: &Path, repo: &str) -> Result<(), Box<dyn Error>> {
    let doc = reasoned_spec();

    // specify: request → specification document (empty memory on first run).
    let specify_pack = serde_json::json!({
        "schema_version": 1, "intent": "specify", "request": REQUEST,
        "engineering_memory": []
    });
    let specify_key = wepld_providers::cassette_key(
        "specify",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&specify_pack)?),
        "specification.v1",
        "fixture-model",
    );
    wepld_providers::write_cassette_entry(
        &store.join("cassettes/recipe.jsonl"),
        &specify_key,
        &serde_json::to_value(&doc)?,
        "fixture-model",
    )?;

    // build: the builder pack from the converted brief + task.
    let conv = convert(ConvertInput {
        doc: &doc,
        spec_id: &format!("spec_{SLUG}"),
        version: 1,
        document_hash: "unused-for-pack",
        slug: SLUG,
        repo,
        base_branch: "main",
        paths: vec!["src/**".to_owned()],
    })
    .map_err(|e| format!("convert failed: {e:?}"))?;
    let pack = builder_pack(
        &serde_json::to_value(&conv.brief)?,
        &serde_json::to_value(&conv.plan.tasks[0])?,
    );
    let build_key = wepld_providers::cassette_key(
        "build",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&pack)?),
        "builder_step.v1",
        "fixture-model",
    );
    wepld_providers::write_cassette_entry(
        &store.join("cassettes/recipe.jsonl"),
        &build_key,
        &serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": EDITED_MAIN } ] }),
        "fixture-model",
    )?;
    Ok(())
}

fn git(dir: &Path, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let out = Command::new("git").args(args).current_dir(dir).output()?;
    if !out.status.success() {
        return Err(format!("git {args:?}: {}", String::from_utf8_lossy(&out.stderr)).into());
    }
    Ok(())
}
