# DECISIONS.md — Founder log

One line per decision made under way. Anything decided under pressure gets written down cold (IMPL-08).

- **2026-07-13** — Architecture v2.0 (incl. Chronicle) FROZEN. Implementation phase begins under the Master Engineering Charter + docs/impl program.
- **2026-07-13** — Stack contradiction (charter Rust/Tauri vs. IADR-0001 TypeScript) resolved by founder: **Rust core + Tauri v2 governs** → IADR-0006 recorded, IADR-0001 superseded, program docs banner-patched.
- **2026-07-13** — Founder OS: **Windows 11** → development and M4 sandbox path via **WSL2** (IADR-0005/0006). Rust toolchain not yet installed on this machine — first item on the Day-1 checklist.
- **2026-07-13** — M0 Day 1 executed: Cargo workspace, `wepld-contracts` crate (mission, ledger entry, envelope+tiers, WWP subset, event vocabulary rev 2 with lock test), CI workflow. Local `cargo test` pending toolchain install; CI will verify on first push.
- **2026-07-13** — Fixture repositories: **locally generated**, not vendored OSS (no downloads, no licenses, fully deterministic). Workspace tests build a tiny `notes-cli` repo inline in temp dirs; a committed `fixtures/repos/notes-cli` source tree lands on Day 7 when missions first consume it. A real OSS repo for M1 realism remains an open founder pick.
- **2026-07-13** — Lifecycle orchestration (plan/approve/run/accept) implemented as `Core` methods, not `submit` commands, because they spawn worker phases (not pure transactions). `create_mission` remains the canonical `submit` pipeline example. Command-level idempotency for lifecycle ops is deferred (not an M0 DoD item); each still records durable ledger facts.
- **2026-07-13** — **M0 COMPLETE.** Tagged `v0.0.1-m0`. DoD verified:
  - golden `m0-first-mission` green (21-entry trace pinned in fixtures/golden, asserted in CI) ✓
  - chain-verify + fold==tables both tested ✓
  - WWP over a real child process (hermes spawned in every lifecycle/build/golden test) ✓
  - context pack captured v0 (stored once to CAS, referenced by hash) ✓
  - DEV tier recorded at init and displayed (`wepld init`, demo) ✓
  - IADR-0001…0008 merged ✓
  - `wepld demo` runs the full loop self-contained (create→plan→approve→run→accept→timeline→verify), chain VERIFIED, edit merged to main ✓
  - 45 tests, fmt + clippy `-D warnings` clean.
- **2026-07-13** — **M0 accepted; M1 authorized.** Repository publish tasks (push commits/tags, GitHub Release) are **BLOCKED**: the authenticated GitHub account `IamShehri` lacks write access to `wepld/wepld` (push → 403). Needs founder action: grant write access, push from an authorized account, or repoint `origin` to a fork. 14 commits + tags `v0.0.1-m0`, `v0.0.1-m0-gate` are staged locally. M1 proceeds locally in the meantime.
- **2026-07-13** — M1 first real adapter is **OpenAI-compatible over HTTP** (Ollama/LM Studio/vLLM/llama.cpp) — local-first per the charter ("Hermes + Ollama"), no TLS/API-key dependency, vendor-neutral. `ureq` (no-default-features + json) is the blocking HTTP client. Hosted HTTPS (OpenAI/Anthropic) is a later TLS-enabled slice; the adapter contract is unchanged by it. Record mode (`RecordingAdapter`) turns real interactions into replayable cassettes — the record/replay harness, delivered as a side effect of the first real adapter (IADR-0002).
- **OPEN** — Obtain one hosted-provider API key (needed for the hosted HTTPS adapter slice; local Ollama needs none).
- **OPEN (M1)** — Pick a real OSS fixture repo for realism beyond the generated `notes-cli`.
