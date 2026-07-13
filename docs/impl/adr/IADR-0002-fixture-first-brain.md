# IADR-0002 — The fixture brain adapter is built first and is the testing spine

**Status:** Accepted · **Date:** 2026-07-13 · **Scope:** implementation only

## Context

Real providers are nondeterministic, slow, and cost money — poison for a solo founder's CI and demo loop. The Brain Contract (v2-07 §3) makes providers pluggable; nothing says the first adapter must be a real one.

## Decision

Adapter #1 is `fixture`: a Brain adapter that **records and replays** provider interactions. Record mode proxies a real adapter and writes `{request_hash → response}` JSONL cassettes into `fixtures/cassettes/`; replay mode serves them deterministically (keyed by pack hash + intent + schema id; miss = loud failure, never improvisation). All golden-ledger tests, CI runs, demos-without-keys, and the Sprint-1 end-to-end run on cassettes. Real adapters (Anthropic-family, OpenAI-compatible-local) land in M1 *behind* the same gateway, and every recorded session with them refreshes cassettes.

## Why

- CI is free, fast, and deterministic from day 3.
- The v2 mandate for a record/replay harness (gate-review improvement #18) is fulfilled as a *side effect of the first adapter*, not as extra infrastructure.
- Demos never die on stage from a provider outage.
- Cassettes double as the evaluation fixtures Phase A demanded: schema-validity and behavior regressions are diffs on recorded traffic.

## Trade-offs

Cassettes go stale as prompts evolve — mitigated by a `--record` refresh flow and hashing packs (a changed pack fails loudly, prompting re-record). Cassettes may contain repo excerpts — they inherit fixture-repo licensing only; never record against private repos.

## Migration impact

None. The fixture adapter is a permanent citizen (testing + offline demo), not scaffolding to delete.
