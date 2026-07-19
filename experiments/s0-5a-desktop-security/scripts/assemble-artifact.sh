#!/usr/bin/env bash
# EXPERIMENTAL - NEVER MERGE. Assemble a runnable UNPACKED (no installer)
# founder-testable artifact from already-built release binaries, binding
# it to the REVIEWED source head (S05A-PROVENANCE-001 correction). Does
# NOT sign, publish, or weaken any security setting.
#
# Provenance model (all recorded distinctly):
#   source_head_sha  - exact reviewed PR branch head (from the caller)
#   checked_out_sha  - the commit actually built (git rev-parse HEAD)
#   workflow_sha     - GitHub event SHA (may be a synthetic merge SHA)
#   base_sha         - canonical base commit
#   artifact_tree_sha- git rev-parse HEAD^{tree}
# Packaging FAILS (no artifact) unless source_head_sha == checked_out_sha.
#
# Env (set by CI; optional locally): S05A_SOURCE_HEAD_SHA, S05A_WORKFLOW_SHA,
# S05A_BASE_SHA, S05A_WORKFLOW_RUN_ID, S05A_REPOSITORY.
# Usage: scripts/assemble-artifact.sh <os-label> <out-dir>
set -euo pipefail
cd "$(dirname "$0")/.."

OS_LABEL="${1:?os label required}"
OUT="${2:?output dir required}"
EXT=""
[ "$OS_LABEL" = "windows" ] && EXT=".exe"

CHECKED_OUT_SHA="$(git rev-parse HEAD)"
ARTIFACT_TREE_SHA="$(git rev-parse 'HEAD^{tree}')"
WORKFLOW_SHA="${S05A_WORKFLOW_SHA:-unset}"
BASE_SHA="${S05A_BASE_SHA:-unset}"
WORKFLOW_RUN_ID="${S05A_WORKFLOW_RUN_ID:-local}"
REPOSITORY="${S05A_REPOSITORY:-local}"

# Resolve the declared source head to a full commit SHA. Locally, if the
# caller did not declare one, fall back to the checked-out commit and
# mark provenance as locally-unverified (CI always declares it).
SOURCE_ORIGIN="declared"
if [ -n "${S05A_SOURCE_HEAD_SHA:-}" ]; then
  SOURCE_HEAD_SHA="$(git rev-parse "${S05A_SOURCE_HEAD_SHA}^{commit}" 2>/dev/null || echo "${S05A_SOURCE_HEAD_SHA}")"
else
  SOURCE_HEAD_SHA="$CHECKED_OUT_SHA"
  SOURCE_ORIGIN="local-unverified"
fi

# FAIL CLOSED before touching any file: a mismatch means we would have
# built (or be labeling) a commit other than the reviewed head — e.g. a
# synthetic pull-request merge ref. No artifact is produced.
if [ "$SOURCE_HEAD_SHA" != "$CHECKED_OUT_SHA" ]; then
  echo "PROVENANCE MISMATCH: source_head_sha=$SOURCE_HEAD_SHA != checked_out_sha=$CHECKED_OUT_SHA" >&2
  echo "Refusing to package. No artifact produced." >&2
  exit 3
fi

rm -rf "$OUT"
mkdir -p "$OUT/fixtures"

# Binaries (release). Host and core are siblings so the host locates the
# core next to its own executable - the UI never spawns processes.
cp "app/tauri-host/target/release/s05a-tauri-host${EXT}" "$OUT/s05a-tauri-host${EXT}"
cp "target/release/s05a-core${EXT}" "$OUT/s05a-core${EXT}"
cp "fixtures/hello.txt" "$OUT/fixtures/hello.txt"

# Launch helpers set the SCOPED fixture/output roots via env - this does
# not widen the capability model; the core still confines every path.
cat > "$OUT/run.cmd" <<'EOF'
@echo off
REM EXPERIMENTAL - NEVER MERGE. Launches the S0.5A host with scoped roots.
set "S05A_FIXTURES=%~dp0fixtures"
set "S05A_OUTPUT=%TEMP%\s05a-output"
if not exist "%S05A_OUTPUT%" mkdir "%S05A_OUTPUT%"
start "" "%~dp0s05a-tauri-host.exe"
EOF

cat > "$OUT/run.sh" <<'EOF'
#!/usr/bin/env bash
# EXPERIMENTAL - NEVER MERGE. Launches the S0.5A host with scoped roots.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
export S05A_FIXTURES="$here/fixtures"
export S05A_OUTPUT="${TMPDIR:-/tmp}/s05a-output"
mkdir -p "$S05A_OUTPUT"
exec "$here/s05a-tauri-host"
EOF
chmod +x "$OUT/run.sh" 2>/dev/null || true

# README - ASCII only (no em dash: fixes the PowerShell mojibake).
cat > "$OUT/README-EXPERIMENTAL.txt" <<EOF
EXPERIMENTAL - NEVER MERGE. NOT SAFE FOR DISTRIBUTION.

This is an unsigned S0.5A desktop-security PROTOTYPE built only to collect
TDR-002 runtime evidence on a founder-controlled machine or disposable VM.
It is not product code, not signed, not notarized, and must not be run on
a production or customer device or distributed to anyone.

Run:  Windows -> run.cmd   |   macOS/Linux -> ./run.sh
The UI holds zero authority; every effect is delegated to the separate
core process over typed IPC. Do NOT disable Smart App Control, Application
Control, Defender, UAC, the firewall, or WebView protections to run this.
If the OS blocks the unsigned binary, that is an expected, recordable
result (BLOCKED BY ENDPOINT POLICY) - do not bypass it.

Verify provenance before trusting this build: BUILD_INFO.txt and
PROVENANCE.json must show source_head_sha == checked_out_sha, and that
value must equal the reviewed PR head. workflow_sha may differ (it can be
a synthetic pull-request merge SHA) and is NOT the source of the build.
EOF

RUSTC="$(rustc --version 2>/dev/null || echo unknown)"
CARGO="$(cargo --version 2>/dev/null || echo unknown)"
NODE="$(node --version 2>/dev/null || echo unknown)"
TAURI="$(grep -A1 '^name = "tauri"$' app/tauri-host/Cargo.lock | grep version | head -1 | tr -dc '0-9.')"

# BUILD_INFO.txt - ASCII only.
cat > "$OUT/BUILD_INFO.txt" <<EOF
artifact_status: EXPERIMENTAL - NEVER MERGE
repository: ${REPOSITORY}
workflow_run_id: ${WORKFLOW_RUN_ID}
os_label: ${OS_LABEL}
source_head_sha: ${SOURCE_HEAD_SHA}
checked_out_sha: ${CHECKED_OUT_SHA}
workflow_sha: ${WORKFLOW_SHA}
base_sha: ${BASE_SHA}
artifact_tree_sha: ${ARTIFACT_TREE_SHA}
source_head_origin: ${SOURCE_ORIGIN}
rustc: ${RUSTC}
cargo: ${CARGO}
node: ${NODE}
tauri: ${TAURI}
signed: NO
installer: NO
EOF

# sha256 helper (sha256sum on Linux/git-bash, shasum on macOS).
if command -v sha256sum >/dev/null 2>&1; then HASH="sha256sum"; else HASH="shasum -a 256"; fi
sha_of() { $HASH "$1" | awk '{print $1}'; }

# Content files hashed into PROVENANCE.json (deterministic order),
# excluding PROVENANCE.json and SHA256SUMS.txt themselves.
FILES=("s05a-tauri-host${EXT}" "s05a-core${EXT}" "fixtures/hello.txt" "run.cmd" "run.sh" "README-EXPERIMENTAL.txt" "BUILD_INFO.txt")

{
  printf '{\n'
  printf '  "artifact_status": "EXPERIMENTAL - NEVER MERGE",\n'
  printf '  "repository": "%s",\n' "$REPOSITORY"
  printf '  "workflow_run_id": "%s",\n' "$WORKFLOW_RUN_ID"
  printf '  "os_label": "%s",\n' "$OS_LABEL"
  printf '  "source_head_sha": "%s",\n' "$SOURCE_HEAD_SHA"
  printf '  "checked_out_sha": "%s",\n' "$CHECKED_OUT_SHA"
  printf '  "workflow_sha": "%s",\n' "$WORKFLOW_SHA"
  printf '  "base_sha": "%s",\n' "$BASE_SHA"
  printf '  "artifact_tree_sha": "%s",\n' "$ARTIFACT_TREE_SHA"
  printf '  "source_head_origin": "%s",\n' "$SOURCE_ORIGIN"
  printf '  "toolchain": { "rustc": "%s", "cargo": "%s", "node": "%s", "tauri": "%s" },\n' "$RUSTC" "$CARGO" "$NODE" "$TAURI"
  printf '  "signed": false,\n'
  printf '  "installer": false,\n'
  printf '  "files": [\n'
  last=$(( ${#FILES[@]} - 1 ))
  for i in "${!FILES[@]}"; do
    f="${FILES[$i]}"
    h="$(cd "$OUT" && sha_of "$f")"
    sep=","
    [ "$i" -eq "$last" ] && sep=""
    printf '    { "path": "%s", "sha256": "%s" }%s\n' "$f" "$h" "$sep"
  done
  printf '  ]\n'
  printf '}\n'
} > "$OUT/PROVENANCE.json"

echo "assembled $OS_LABEL artifact at $OUT (source_head_sha=$SOURCE_HEAD_SHA)"
ls -l "$OUT"
