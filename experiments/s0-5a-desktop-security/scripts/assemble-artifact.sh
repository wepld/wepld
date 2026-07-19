#!/usr/bin/env bash
# EXPERIMENTAL — NEVER MERGE. Assemble a runnable UNPACKED (no installer)
# founder-testable artifact from already-built release binaries. Does NOT
# sign, publish, or weaken any security setting. Usage:
#   scripts/assemble-artifact.sh <os-label> <out-dir>
# where <os-label> is windows|macos|linux. Run from the experiment root.
set -euo pipefail
cd "$(dirname "$0")/.."

OS_LABEL="${1:?os label required}"
OUT="${2:?output dir required}"
COMMIT="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
EXT=""
[ "$OS_LABEL" = "windows" ] && EXT=".exe"

rm -rf "$OUT"
mkdir -p "$OUT/fixtures"

# Binaries (release). Host and core are siblings so the host locates the
# core next to its own executable — the UI never spawns processes.
cp "app/tauri-host/target/release/s05a-tauri-host${EXT}" "$OUT/s05a-tauri-host${EXT}"
cp "target/release/s05a-core${EXT}" "$OUT/s05a-core${EXT}"
cp "fixtures/hello.txt" "$OUT/fixtures/hello.txt"

# Launch helpers set the SCOPED fixture/output roots via env — this does
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

cat > "$OUT/README-EXPERIMENTAL.txt" <<EOF
EXPERIMENTAL — NEVER MERGE. NOT SAFE FOR DISTRIBUTION.

This is an unsigned S0.5A desktop-security PROTOTYPE built only to collect
TDR-002 runtime evidence on a founder-controlled machine or disposable VM.
It is not product code, not signed, not notarized, and must not be run on
a production or customer device or distributed to anyone.

Run:  Windows -> run.cmd   |   macOS/Linux -> ./run.sh
The UI holds zero authority; every effect is delegated to the separate
core process over typed IPC. Do NOT disable Smart App Control, Application
Control, Defender, UAC, the firewall, or WebView protections to run this.
If the OS blocks the unsigned binary, that is an expected, recordable
result (BLOCKED BY ENDPOINT POLICY) — do not bypass it.
EOF

cat > "$OUT/BUILD_INFO.txt" <<EOF
S0.5A EXPERIMENTAL ARTIFACT — NEVER MERGE
os_label:        ${OS_LABEL}
source_commit:   ${COMMIT}
rustc:           $(rustc --version 2>/dev/null || echo unknown)
cargo:           $(cargo --version 2>/dev/null || echo unknown)
node:            $(node --version 2>/dev/null || echo unknown)
tauri (locked):  $(grep -A1 '^name = "tauri"$' app/tauri-host/Cargo.lock | grep version | head -1 | tr -d ' ')
signed:          NO
installer:       NO (unpacked runnable)
contents:        s05a-tauri-host${EXT}, s05a-core${EXT}, fixtures/hello.txt,
                 run.cmd, run.sh, README-EXPERIMENTAL.txt, BUILD_INFO.txt,
                 SHA256SUMS.txt
EOF

echo "assembled $OS_LABEL artifact at $OUT"
ls -l "$OUT"
