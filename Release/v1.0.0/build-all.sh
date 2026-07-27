#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# build-all.sh — produce every artifact the v1.0.0 GitHub Release needs,
# entirely on this machine. The .github/workflows/release.yml workflow
# is the CI equivalent.
#
# Usage from the EZHTML repo root:
#
#   ./Release/v1.0.0/build-all.sh
#
# Requirements:
#   - Rust stable + rustup + cross support for the targets below.
#   - Node 20 + pnpm 9.
#   - python3 (only for the install.sh helper script).
# ---------------------------------------------------------------------------

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="$ROOT/Release/v1.0.0/assets"
mkdir -p "$OUT"

VERSION="1.0.0"

# ---- 1) Compiler cross-compile matrix -------------------------------

cross_targets=(
  "x86_64-unknown-linux-gnu"
  "x86_64-unknown-linux-musl"
  "aarch64-unknown-linux-musl"
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
  "x86_64-pc-windows-msvc"
)

cd "$ROOT/compiler"

for tgt in "${cross_targets[@]}"; do
  echo "==== compiler build: $tgt ===="
  cargo build --release --target "$tgt" --locked
done

cd "$ROOT"

# ---- 2) Tar/zip + binary packaging ----------------------------------
for tgt in "${cross_targets[@]}"; do
  case "$tgt" in
    *windows*)
      bin="ezhtml.exe"
      arc="ezhtml-v${VERSION}-${tgt}.zip"
      tmp="$OUT/ezhtml-v${VERSION}-${tgt}"
      mkdir -p "$tmp"
      cp "compiler/target/$tgt/release/ezhtml.exe" "$tmp/"
      (cd "$OUT" && zip -qr "$arc" "$(basename "$tmp")")
      rm -rf "$tmp"
      ;;
    *)
      bin="ezhtml"
      arc="ezhtml-v${VERSION}-${tgt}.tar.gz"
      tmp="$OUT/ezhtml-v${VERSION}-${tgt}"
      mkdir -p "$tmp"
      cp "compiler/target/$tgt/release/ezhtml" "$tmp/"
      chmod +x "$tmp/ezhtml"
      tar -C "$OUT" -czf "$arc" "$(basename "$tmp")"
      rm -rf "$tmp"
      ;;
  esac
done

# ---- 3) Desktop editor bundles -------------------------------------
cd "$ROOT/editor"
pnpm install --frozen-lockfile
pnpm lint
pnpm test            # strict under `set -e`: missing vitest or failing tests abort the release build
pnpm build
pnpm package        # builds AppImage / dmg / nsis .exe based on platform
cd "$ROOT"

# The electron-builder output depends on the host OS:
#   Linux  -> dist/electron/EZHTML Editor-1.0.0.AppImage
#   macOS  -> dist/electron/EZHTML Editor-1.0.0.dmg
#   Windows-> dist/electron/EZHTML Editor Setup 1.0.0.exe
# Rename so the GitHub Release assets have stable filenames.
for f in "$ROOT/editor/dist/electron/"*.AppImage "$ROOT/editor/dist/electron/"*.dmg "$ROOT/editor/dist/electron/"*.exe; do
  [ -e "$f" ] || continue
  case "$f" in
    *.AppImage) cp "$f" "$OUT/ezhtml-editor-v${VERSION}.AppImage" ;;
    *.dmg)      cp "$f" "$OUT/ezhtml-editor-v${VERSION}.dmg" ;;
    *.exe)      cp "$f" "$OUT/ezhtml-editor-v${VERSION}.exe" ;;
  esac
done

# ---- 4) VS Code extension -------------------------------------------
cd "$ROOT/vscode-extension"
npm install
npm run build
npx vsce package --no-dependencies
cp ezhtml-${VERSION}.vsix "$OUT/ezhtml-vscode-v${VERSION}.vsix"
cd "$ROOT"

# ---- 5) SHA-256 manifest --------------------------------------------
( cd "$OUT" && sha256sum * ) > "$OUT/../SHA256SUMS.txt"
echo
echo "OK. Artifacts in $OUT:"
ls -lh "$OUT"
echo
echo "Manifest written to $(realpath "$OUT/../SHA256SUMS.txt")"
