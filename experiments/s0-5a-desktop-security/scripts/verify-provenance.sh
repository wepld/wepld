#!/usr/bin/env bash
# EXPERIMENTAL - NEVER MERGE. Dependency-free provenance validator for an
# assembled S0.5A artifact (S05A-PROVENANCE-001). Fails non-zero on any
# violation, so CI can gate upload on it. Uses only shell + python3 (JSON
# parsing / ASCII check) - no new package dependency.
#
#   scripts/verify-provenance.sh <artifact-dir> [expected_source_head_sha]
#   scripts/verify-provenance.sh --selftest
set -euo pipefail
cd "$(dirname "$0")/.."

if command -v sha256sum >/dev/null 2>&1; then HASH="sha256sum"; else HASH="shasum -a 256"; fi
sha_of() { $HASH "$1" | awk '{print $1}'; }
fail() { echo "PROVENANCE-CHECK FAIL: $*" >&2; exit 1; }

# Portable Python (CI runners have python3; some local shells only have
# python, and Windows ships a non-functional Store alias). Probe for one
# that actually EXECUTES. Standard tooling; no package dependency added.
PY=""
for cand in python3 python py; do
  if command -v "$cand" >/dev/null 2>&1 && "$cand" -c "import sys" >/dev/null 2>&1; then PY="$cand"; break; fi
done
[ -n "$PY" ] || fail "no working python3/python available for JSON/ASCII validation"

# --- self-test: assemble must fail closed on a source/checkout mismatch,
# --- producing no artifact. Runs before any real binaries exist.
if [ "${1:-}" = "--selftest" ]; then
  tmp="$(mktemp -d)"
  set +e
  S05A_SOURCE_HEAD_SHA="0000000000000000000000000000000000000000" \
    bash scripts/assemble-artifact.sh windows "$tmp/out" >/dev/null 2>&1
  rc=$?
  set -e
  [ "$rc" -ne 0 ] || fail "assemble did NOT fail on a forged source_head_sha"
  [ ! -e "$tmp/out/BUILD_INFO.txt" ] || fail "assemble wrote BUILD_INFO despite mismatch"
  [ ! -e "$tmp/out/PROVENANCE.json" ] || fail "assemble wrote PROVENANCE despite mismatch"
  rm -rf "$tmp"
  echo "selftest OK: packaging fails closed on provenance mismatch (rc=$rc, no artifact)"
  exit 0
fi

DIR="${1:?artifact dir required}"
EXPECTED="${2:-}"

[ -f "$DIR/BUILD_INFO.txt" ]   || fail "missing BUILD_INFO.txt"
[ -f "$DIR/PROVENANCE.json" ]  || fail "missing PROVENANCE.json"
[ -f "$DIR/SHA256SUMS.txt" ]   || fail "missing SHA256SUMS.txt"

# 1. PROVENANCE.json is valid JSON with all required fields.
$PY - "$DIR/PROVENANCE.json" <<'PY' || fail "PROVENANCE.json invalid or missing fields"
import json,sys
d=json.load(open(sys.argv[1],encoding="utf-8"))
req=["artifact_status","repository","workflow_run_id","os_label","source_head_sha",
     "checked_out_sha","workflow_sha","base_sha","artifact_tree_sha","signed","installer","files"]
missing=[k for k in req if k not in d]
assert not missing, f"missing {missing}"
assert isinstance(d["files"],list) and d["files"], "files empty"
assert d["signed"] is False and d["installer"] is False, "signed/installer must be false"
PY

# helper: read a "key: value" line from BUILD_INFO.txt
bi() { sed -n "s/^$1: //p" "$DIR/BUILD_INFO.txt" | head -1; }
# helper: read a top-level string field from PROVENANCE.json
pj() { $PY -c "import json,sys;print(json.load(open(sys.argv[1],encoding='utf-8')).get(sys.argv[2],''))" "$DIR/PROVENANCE.json" "$1"; }

BI_SRC="$(bi source_head_sha)"; BI_CHK="$(bi checked_out_sha)"; BI_WF="$(bi workflow_sha)"
BI_BASE="$(bi base_sha)"; BI_TREE="$(bi artifact_tree_sha)"
PJ_SRC="$(pj source_head_sha)"; PJ_CHK="$(pj checked_out_sha)"; PJ_WF="$(pj workflow_sha)"
PJ_BASE="$(pj base_sha)"; PJ_TREE="$(pj artifact_tree_sha)"

# 3/4. required fields present and source==checked in BOTH files.
[ -n "$BI_SRC" ] && [ -n "$BI_CHK" ] || fail "BUILD_INFO missing sha fields"
[ "$BI_SRC" = "$BI_CHK" ] || fail "BUILD_INFO source_head_sha != checked_out_sha ($BI_SRC vs $BI_CHK)"
[ "$PJ_SRC" = "$PJ_CHK" ] || fail "PROVENANCE source_head_sha != checked_out_sha"

# 7. BUILD_INFO and PROVENANCE agree on every SHA field.
[ "$BI_SRC" = "$PJ_SRC" ]   || fail "source_head_sha disagreement BUILD_INFO vs PROVENANCE"
[ "$BI_CHK" = "$PJ_CHK" ]   || fail "checked_out_sha disagreement"
[ "$BI_WF"  = "$PJ_WF"  ]   || fail "workflow_sha disagreement"
[ "$BI_BASE" = "$PJ_BASE" ] || fail "base_sha disagreement"
[ "$BI_TREE" = "$PJ_TREE" ] || fail "artifact_tree_sha disagreement"

# 8. no synthetic merge SHA labeled as source head: the source head must
# be the built commit, and (when a distinct workflow_sha is recorded) must
# not equal it.
if [ "$BI_WF" != "unset" ] && [ "$BI_WF" = "$BI_SRC" ]; then
  # Allowed only when the event SHA legitimately equals the head (e.g. a
  # push run). For a merge-ref this would be a mislabel; we cannot tell
  # the event here, so we require checked_out==source_head (already true)
  # and rely on the CI guard. Emit a note, do not fail.
  echo "note: workflow_sha == source_head_sha (push-style run)"
fi

# 5. expected source head, when provided by CI, must match.
if [ -n "$EXPECTED" ]; then
  EXP_FULL="$(git rev-parse "${EXPECTED}^{commit}" 2>/dev/null || echo "$EXPECTED")"
  [ "$BI_SRC" = "$EXP_FULL" ] || fail "source_head_sha ($BI_SRC) != expected reviewed head ($EXP_FULL)"
fi

# 6. SHA256SUMS matches the packaged binaries, and PROVENANCE agrees.
# Resolve the real on-disk basename by listing (avoids a Windows quirk
# where `test -f name` / `sha256sum name` auto-resolve to name.exe).
for b in s05a-tauri-host s05a-core; do
  f="$(cd "$DIR" && ls | grep -E "^${b}(\.exe)?$" | head -1)"
  [ -n "$f" ] || fail "missing binary $b"
  actual="$(cd "$DIR" && sha_of "$f")"
  insums="$(grep -F "./$f" "$DIR/SHA256SUMS.txt" | awk '{print $1}' | head -1)"
  [ -n "$insums" ] && [ "$actual" = "$insums" ] || fail "SHA256SUMS mismatch for $f ($actual vs $insums)"
  inprov="$($PY -c "import json,sys;d=json.load(open(sys.argv[1],encoding='utf-8'));print(next((x['sha256'] for x in d['files'] if x['path']==sys.argv[2]),''))" "$DIR/PROVENANCE.json" "$f")"
  [ "$actual" = "$inprov" ] || fail "PROVENANCE hash mismatch for $f ($actual vs $inprov)"
done

# 9. ASCII-only provenance files (fixes the PowerShell em-dash mojibake).
for f in BUILD_INFO.txt PROVENANCE.json README-EXPERIMENTAL.txt; do
  $PY -c "import sys;d=open(sys.argv[1],'rb').read();sys.exit(1 if any(b>127 for b in d) else 0)" "$DIR/$f" \
    || fail "$f contains non-ASCII bytes"
done

echo "PROVENANCE-CHECK OK: source_head_sha=$BI_SRC checked_out_sha=$BI_CHK workflow_sha=$BI_WF"
echo "  files verified against SHA256SUMS.txt and PROVENANCE.json; ASCII clean."
