#!/usr/bin/env bash
set -euo pipefail

VERSION="v1.0.0"
BASE_URL="https://github.com/flazzyde/ezhtml/releases/download/${VERSION}"

ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

case "$ARCH" in
  x86_64|amd64) ARCH_NAME="x86_64" ;;
  aarch64|arm64) ARCH_NAME="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

case "$OS" in
  linux) LIBC="musl"
    if ldd --version 2>/dev/null | grep -q glibc; then
      LIBC="gnu"
    fi
    FILE="ezhtml-${VERSION}-${ARCH_NAME}-unknown-linux-${LIBC}.tar.gz"
    ;;
  darwin) FILE="ezhtml-${VERSION}-${ARCH_NAME}-apple-darwin.tar.gz" ;;
  msys*|cygwin*|mingw*|windows_nt) FILE="ezhtml-${VERSION}-${ARCH_NAME}-pc-windows-msvc.zip" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

URL="${BASE_URL}/${FILE}"
echo "Downloading ${FILE}..."
curl -L --progress-bar -o "/tmp/${FILE}" "$URL"

case "$FILE" in
  *.tar.gz) tar -xzf "/tmp/${FILE}" -C /tmp ;;
  *.zip) unzip -q "/tmp/${FILE}" -d /tmp ;;
esac

echo "EZHTML ${VERSION} installed. Add /tmp/ezhtml/bin to your PATH or move the binary to a location in PATH."
