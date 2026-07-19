#!/usr/bin/env bash
# EXPERIMENTAL — NEVER MERGE. Write a SHA-256 manifest over every file in
# an assembled artifact directory (excluding the manifest itself).
# Usage: scripts/make-checksums.sh <artifact-dir>
set -euo pipefail
DIR="${1:?artifact dir required}"
cd "$DIR"
rm -f SHA256SUMS.txt

# Prefer sha256sum (Linux/git-bash); fall back to shasum (macOS).
if command -v sha256sum >/dev/null 2>&1; then
  HASH="sha256sum"
else
  HASH="shasum -a 256"
fi

find . -type f ! -name SHA256SUMS.txt -print0 \
  | sort -z \
  | while IFS= read -r -d '' f; do
      $HASH "$f"
    done > SHA256SUMS.txt

echo "== SHA256SUMS.txt =="
cat SHA256SUMS.txt
