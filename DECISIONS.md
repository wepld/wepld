# DECISIONS.md — Founder log

One line per decision made under way. Anything decided under pressure gets written down cold (IMPL-08).

- **2026-07-13** — Architecture v2.0 (incl. Chronicle) FROZEN. Implementation phase begins under the Master Engineering Charter + docs/impl program.
- **2026-07-13** — Stack contradiction (charter Rust/Tauri vs. IADR-0001 TypeScript) resolved by founder: **Rust core + Tauri v2 governs** → IADR-0006 recorded, IADR-0001 superseded, program docs banner-patched.
- **2026-07-13** — Founder OS: **Windows 11** → development and M4 sandbox path via **WSL2** (IADR-0005/0006). Rust toolchain not yet installed on this machine — first item on the Day-1 checklist.
- **2026-07-13** — M0 Day 1 executed: Cargo workspace, `wepld-contracts` crate (mission, ledger entry, envelope+tiers, WWP subset, event vocabulary rev 2 with lock test), CI workflow. Local `cargo test` pending toolchain install; CI will verify on first push.
- **OPEN** — Select the two fixture repositories (needed by Day 3–4; suggestions in IMPL-00).
- **OPEN** — Obtain one hosted-provider API key (not needed until M1; Sprint 1 is cassette-only per IADR-0002).
