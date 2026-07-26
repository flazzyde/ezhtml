#!/usr/bin/env bash
# Requires bash >= 4.0 (associative arrays). macOS ships bash 3.2 by default;
# install bash via `brew install bash` and run with that binary, or invoke
# the script via `bash docs/RFCs/milestones/create-issues.sh`.
# ---------------------------------------------------------------------------
# create-issues.sh — open the six roadmap milestone issues on GitHub.
#
# Usage:
#   gh auth login                         # one-time, never committed
#   ./docs/RFCs/milestones/create-issues.sh
#
# Each issue body is the contents of the matching M*.md file in this
# directory (everything after the first `---`-style separator is kept
# in the issue comment as a "context" block — adjust to taste).
#
# Idempotent in spirit: the script does NOT check whether an issue
# already exists; running twice produces duplicates. Re-run only after
# deleting previously created issues.
# ---------------------------------------------------------------------------

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
RFC_DIR="${REPO_ROOT}/docs/RFCs/milestones"
LABELS=("roadmap")

if ! command -v gh >/dev/null 2>&1; then
  echo "gh CLI not found. Install: https://cli.github.com/" >&2
  exit 1
fi

if ! gh auth status >/dev/null 2>&1; then
  echo "Not authenticated with gh. Run 'gh auth login' first." >&2
  exit 1
fi

# Map RFC file -> milestone short tag used in the title.
declare -A TITLES=(
  ["M1-v0.3-rc1.md"]="M1 · v0.3.0-rc.1 — WASM build of the compiler"
  ["M2-ezhtml-lint.md"]="M2 · \`ezhtml lint\` as its own command"
  ["M3-plugin-format.md"]="M3 · \`.ezplugin\` format v0"
  ["M4-theme-registry.md"]="M4 · Theme registry MVP"
  ["M5-static-site-gen.md"]="M5 · Static site generator"
  ["M6-lsp-prototype.md"]="M6 · LSP prototype"
)

# Per-milestone label suffix (merged with the `roadmap` label).
declare -A EXTRA_LABELS=(
  ["M1-v0.3-rc1.md"]="compiler,wasm,milestone-m1"
  ["M2-ezhtml-lint.md"]="cli,milestone-m2"
  ["M3-plugin-format.md"]="plugins,milestone-m3"
  ["M4-theme-registry.md"]="themes,milestone-m4"
  ["M5-static-site-gen.md"]="cli,site-gen,milestone-m5"
  ["M6-lsp-prototype.md"]="lsp,editor,milestone-m6"
)

for file in M1-v0.3-rc1.md M2-ezhtml-lint.md M3-plugin-format.md \
            M4-theme-registry.md M5-static-site-gen.md M6-lsp-prototype.md; do
  body_file="${RFC_DIR}/${file}"
  title="${TITLES[$file]}"
  labels="${LABELS[*]},${EXTRA_LABELS[$file]}"

  echo "+ creating: ${title}"
  gh issue create \
    --repo "$(gh repo view --json nameWithOwner -q .nameWithOwner)" \
    --title "${title}" \
    --label "${labels}" \
    --body-file "${body_file}"
done

echo
echo "All six milestone issues created. Edit the roadmap:"
echo "  - docs/Roadmap.md   — replace #[RFC](milestones/M1-...md) with #42-style GitHub links"
echo "  - website/roadmap/  — same swap for the Roadmap page"
