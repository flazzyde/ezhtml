#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# verify-release.sh — for every artifact in Release/v1.0.0/assets, verify
# its SHA-256 against Release/v1.0.0/SHA256SUMS.txt, and run a CLI smoke
# compile against the bundled templates.
#
# Usage from the EZHTML repo root:
#   ./Release/v1.0.0/verify-release.sh
#
# Portability notes:
#   - Uses `od`/`head` (not `file`) to PE-check binaries; both are present
#     on Git-Bash-minimal and Alpine by default whereas `file` may not be.
#   - Temp directories are accumulated in a `works` array and cleaned up by
#     a single EXIT trap, so multiple smoke blocks don't leak prior dirs.
# ---------------------------------------------------------------------------

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ASSETS="$ROOT/Release/v1.0.0/assets"
SUMS="$ROOT/Release/v1.0.0/SHA256SUMS.txt"

# Every mktemp-dir we create is appended here; one EXIT trap cleans up.
declare -a works=()
cleanup() {
  if [[ ${#works[@]} -gt 0 ]]; then
    rm -rf "${works[@]}" 2>/dev/null || true
  fi
}
trap cleanup EXIT

# Portable PE signature check (any PE/NE/LE binary starts with `MZ` = 4d 5a).
# Works on Git-Bash-minimal and minimal Alpine where `file` may be absent.
is_pe_binary() {
  local f="$1"
  [[ -s "$f" ]] || return 1
  local sig
  sig="$(head -c 2 "$f" 2>/dev/null | od -An -tx1 2>/dev/null | tr -d ' \n' || true)"
  [[ "$sig" == "4d5a" ]]
}

# ---- 0) Empty-tree guard ------------------------------------------
# .gitkeep placeholders don't count as real artifacts. If nothing
# built is here, print a clear hint instead of running an empty
# smoke loop that would silently print "[skip]" for every line.
real_artifact_count() {
  find "$ASSETS" -maxdepth 1 -type f \
    ! -name '*.gitkeep' \
    ! -name 'install.sh' \
    -printf '%f\n' 2>/dev/null | wc -l
}

if [[ "$(real_artifact_count)" -eq 0 ]]; then
  echo "Release/v1.0.0/assets/ contains no built artifacts yet."
  echo "Run ./Release/v1.0.0/build-all.sh first, or wait for the"
  echo "release.yml CI matrix to upload them via the v1.0.0 tag."
  echo "Nothing to verify — exiting cleanly."
  exit 0
fi

# ---- 1) SHA-256 ----------------------------------------------------

if [[ -f "$SUMS" && -s "$SUMS" ]]; then
  echo "==== SHA-256 ===="
  ( cd "$ASSETS" && sha256sum -c "$SUMS" )
else
  echo "[skip] SHA-256 manifest $SUMS missing or empty."
  echo "[skip] generate it after build-all.sh runs."
fi

# ---- 2) Smoke compile on every platform binary --------------------

smoke_template="$ROOT/templates/landing/index.ezhtml"
[[ -f "$smoke_template" ]] || { echo "no landing template at $smoke_template"; exit 1; }

declare -a bins=(
  "linux-gnu:ezhtml-v1.0.0-x86_64-unknown-linux-gnu.tar.gz"
  "linux-musl:ezhtml-v1.0.0-x86_64-unknown-linux-musl.tar.gz"
  "linux-arm:ezhtml-v1.0.0-aarch64-unknown-linux-musl.tar.gz"
  "macos-x64:ezhtml-v1.0.0-x86_64-apple-darwin.tar.gz"
  "macos-arm:ezhtml-v1.0.0-aarch64-apple-darwin.tar.gz"
)

for entry in "${bins[@]}"; do
  label="${entry%%:*}"
  arc="${entry##*:}"
  tar="$ASSETS/$arc"
  [[ -f "$tar" ]] || { echo "[skip] $label ($arc)"; continue; }

  work="$(mktemp -d)"
  works+=("$work")
  tar -xzf "$tar" -C "$work"
  bin="$work/$(basename "$tar" .tar.gz)/ezhtml"
  [[ -x "$bin" ]] || { echo "[skip] $label (no binary inside tar)"; continue; }

  out="$work/out.html"
  echo "==== smoke ($label): $bin --version ===="
  "$bin" --version
  echo "==== smoke ($label): build landing ===="
  "$bin" build "$smoke_template" -o "$out"
  grep -q "<!DOCTYPE html>" "$out" || { echo "[FAIL] $label: no DOCTYPE"; exit 1; }
  grep -q "<h1"            "$out" || { echo "[FAIL] $label: no <h1>";     exit 1; }
  echo "[ok] $label landed a valid HTML5 output"
done

# ---- 3) Windows compiler binary smoke -----------------------------

zip="$ASSETS/ezhtml-v1.0.0-x86_64-pc-windows-msvc.zip"
if [[ -f "$zip" ]]; then
  work="$(mktemp -d)"
  works+=("$work")
  unzip -q "$zip" -d "$work"
  exe="$(find "$work" -maxdepth 2 -name ezhtml.exe | head -1 || true)"
  if [[ -n "${exe:-}" && -f "$exe" ]]; then
    if is_pe_binary "$exe"; then
      echo "[ok] windows-compiler (ezhtml.exe) is a valid PE binary"
    else
      # The compiler is the byte the user actually runs; a non-MZ header
      # almost certainly means a tampered/corrupted artifact. Refuse to
      # sign-off rather than ship a broken release silently.
      echo "[FAIL] windows-compiler (ezhtml.exe) does not start with MZ — refusing to sign-off"
      exit 1
    fi
  else
    echo "[skip] windows-compiler (no ezhtml.exe inside zip)"
  fi
fi

# ---- 4) Windows editor NSIS installer smoke -----------------------
# electron-builder NSIS target ships as `ezhtml-editor-v1.0.0-setup.exe`.
# Verify it's present + PE signature matches a Windows installer.
setup="$ASSETS/ezhtml-editor-v1.0.0-setup.exe"
if [[ -f "$setup" ]]; then
  if is_pe_binary "$setup"; then
    echo "[ok] windows-editor ($setup) is a valid PE binary"
  else
    echo "[warn] windows-editor ($setup) does not start with MZ — unexpected for NSIS"
  fi
else
  echo "[skip] windows editor installer ($setup) not built yet"
fi

echo "ALL GOOD"
