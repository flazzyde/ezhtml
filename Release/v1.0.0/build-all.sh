#!/usr/bin/env bash
set -euo pipefail

VERSION="1.0.0"
OUTDIR="$(cd "$(dirname "$0")" && pwd)/assets"

# Placeholder for CI/local cross-build of the compiler.
# In a real setup this runs `cargo build --release --target ...` for each target
# and `pnpm package` for the editor / vsix.

echo "Building EZHTML release ${VERSION}..."
echo "Output directory: ${OUTDIR}"

# Example target list - real build steps would go here.
for target in x86_64-unknown-linux-gnu x86_64-unknown-linux-musl aarch64-unknown-linux-musl x86_64-apple-darwin aarch64-apple-darwin x86_64-pc-windows-msvc; do
  echo "  - ${target}"
done

echo "Editor bundles:"
echo "  - ezhtml-editor-v${VERSION}.AppImage"
echo "  - ezhtml-editor-v${VERSION}.dmg"
echo "  - ezhtml-editor-v${VERSION}-setup.exe"
echo "VS Code extension:"
echo "  - ezhtml-vscode-v${VERSION}.vsix"

echo "Done. Run 'cd assets && sha256sum * > ../SHA256SUMS.txt' after binaries are in place."
