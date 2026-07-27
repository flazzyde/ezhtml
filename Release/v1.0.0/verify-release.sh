#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
ASSETS="${DIR}/assets"

ERRORS=0

echo "=== Verifying EZHTML v1.0.0 release assets ==="

while IFS= read -r line || [[ -n "$line" ]]; do
  [[ -z "$line" ]] && continue
  [[ "$line" =~ ^# ]] && continue
  asset="${ASSETS}/${line}"
  if [[ ! -f "$asset" ]]; then
    echo "MISSING: ${line}"
    ERRORS=$((ERRORS + 1))
  else
    echo "OK: ${line}"
  fi
done < "${ASSETS}/EXPECTED_ASSETS.txt"

if [[ -f "${DIR}/SHA256SUMS.txt" ]]; then
  echo "=== Verifying SHA-256 checksums ==="
  (cd "$ASSETS" && sha256sum -c "${DIR}/SHA256SUMS.txt") || ERRORS=$((ERRORS + 1))
else
  echo "WARNING: SHA256SUMS.txt not found - generate with 'cd assets && sha256sum * > ../SHA256SUMS.txt'"
fi

if [[ $ERRORS -eq 0 ]]; then
  echo "ALL CHECKS PASSED"
else
  echo "ERRORS: $ERRORS"
  exit 1
fi
