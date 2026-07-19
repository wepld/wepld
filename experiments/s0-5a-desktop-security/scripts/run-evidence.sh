#!/usr/bin/env bash
# EXPERIMENTAL — NEVER MERGE. Repeatable evidence runner for the S0.5A
# core: clippy, tests, release build, and N benchmark repetitions.
# Usage: scripts/run-evidence.sh [bench-iters] [bench-repeats]
#
# NOTE: on a host with a Windows Application Control / Smart App Control
# policy that blocks freshly compiled build-script executables, build in
# a workspace whose CARGO_TARGET_DIR is unaffected, or rely on CI. The
# core has ZERO third-party dependencies and therefore ZERO build
# scripts, which is what lets it build under such policies at all.
set -euo pipefail
cd "$(dirname "$0")/.."

ITERS="${1:-3000}"
REPEATS="${2:-4}"

echo "== clippy (deny warnings) =="
cargo clippy --locked --all-targets -- -D warnings

echo "== test (locked) =="
cargo test --locked

echo "== release build =="
cargo build --locked --release

echo "== bench x${REPEATS} (iters=${ITERS}) =="
for i in $(seq 1 "$REPEATS"); do
  S05A_BENCH_ITERS="$ITERS" cargo run --locked --release --quiet --bin s05a-bench
done
