#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# install.sh — one-liner installer for Linux + macOS.
#
# This is a copy of the same script that gets mirrored to
#   https://ezhtml.flazzy.de/install.sh
# so `curl -L https://ezhtml.flazzy.de/install.sh | bash` keeps working
# across releases.
#
# Defaults to the latest GitHub release; override with $VERSION.
#   VERSION=v1.0.0 curl -L https://ezhtml.flazzy.de/install.sh | bash
# ---------------------------------------------------------------------------

set -euo pipefail

REPO="${EZHTML_REPO:-flazzyde/ezhtml}"
VERSION="${VERSION:-latest}"
PREFIX="${EZHTML_PREFIX:-$HOME/.local}"

# Resolve VERSION=latest to the real tag from the GitHub API so the
# asset filename below matches what the release actually published.
# If the user pinned a version, require it matches vX.Y.Z exactly.
if [[ "$VERSION" == "latest" ]]; then
  api_url="https://api.github.com/repos/$REPO/releases/latest"
  if ! api_body="$(curl -fsSL -H 'User-Agent: ezhtml-installer' "$api_url")"; then
    if curl -sS -H 'User-Agent: ezhtml-installer' -o /dev/null -w '%{http_code}' "$api_url" 2>/dev/null \
        | grep -q '^403$'; then
      echo "GitHub anon API rate-limited — retry in about an hour, or pin VERSION=v1.0.0." >&2
    else
      echo "could not reach $api_url — check network, or pin VERSION=v1.0.0." >&2
    fi
    exit 1
  fi
  VERSION="$(printf '%s' "$api_body" \
    | grep -m1 '"tag_name"' \
    | sed -E 's/.*"tag_name":\s*"([^"]+)".*/\1/')"
  [[ -n "$VERSION" && "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+([-+].*)?$ ]] \
    || { echo "could not resolve latest release tag (got: $VERSION)" >&2; exit 1; }
else
  [[ "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+([-+].*)?$ ]] \
    || { echo "VERSION must look like v1.0.0 (got: $VERSION)" >&2; exit 1; }
fi

uname_s="$(uname -s)"
uname_m="$(uname -m)"

case "$uname_s" in
  Linux)
    case "$uname_m" in
      x86_64)            asset="ezhtml-${VERSION}-x86_64-unknown-linux-musl.tar.gz" ;;
      aarch64 | arm64)   asset="ezhtml-${VERSION}-aarch64-unknown-linux-musl.tar.gz" ;;
      *) echo "unsupported architecture: $uname_m" >&2; exit 1 ;;
    esac
    url_kind=tar
    ;;
  Darwin)
    case "$uname_m" in
      x86_64)            asset="ezhtml-${VERSION}-x86_64-apple-darwin.tar.gz" ;;
      arm64)             asset="ezhtml-${VERSION}-aarch64-apple-darwin.tar.gz" ;;
      *) echo "unsupported architecture: $uname_m" >&2; exit 1 ;;
    esac
    url_kind=tar
    ;;
  *)
    echo "ezhtml install.sh does not support $uname_s; grab the Windows zip manually." >&2
    exit 1
    ;;
esac

base_url="https://github.com/$REPO/releases/download/$VERSION"
download_url="$base_url/$asset"

work="$(mktemp -d)"
trap "rm -rf $work" EXIT

echo "+ downloading $download_url"
curl -fsSL "$download_url" -o "$work/$asset"

case "$url_kind" in
  tar)
    tar -xzf "$work/$asset" -C "$work"
    bin="$(find "$work" -maxdepth 2 -name ezhtml -type f | head -1)"
    ;;
  zip)
    unzip -q "$work/$asset" -d "$work"
    bin="$(find "$work" -maxdepth 2 -name 'ezhtml.exe' | head -1)"
    ;;
esac
[[ -n "${bin:-}" ]] || { echo "could not extract binary" >&2; exit 1; }

mkdir -p "$PREFIX/bin"
install -m 0755 "$bin" "$PREFIX/bin/ezhtml"

cat <<EOF

Installed:
  $PREFIX/bin/ezhtml

Add to your PATH (if not already):
  export PATH="$PREFIX/bin:\$PATH"

Verify:
  ezhtml version

Project home:  https://ezhtml.flazzy.de
Discord:       https://discord.gg/TQs6McKJJs
EOF
