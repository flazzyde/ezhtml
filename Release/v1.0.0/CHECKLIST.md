# Production-readiness checklist for v1.0.0

Every gate below is mapped to a CI workflow run (`main` branch) and / or
a local command. **All boxes must be green** before tagging `v1.0.0`.

## Compiler (Rust)

- [ ] `cd compiler && cargo fmt --all -- --check` — exit code 0
- [ ] `cd compiler && cargo clippy --all-targets -- -D warnings` — exit code 0
- [ ] `cd compiler && cargo test --all --locked` — exit code 0
      (also exercised by `.github/workflows/ci.yml :: rust-compiler`)
- [ ] `cd compiler && cargo build --release --locked` — produces a
      statically-linked `target/release/ezhtml` binary.
- [ ] `ezhtml version` prints `ezhtml 1.0.0`.
- [ ] `ezhtml build examples/01-hello-world.ezhtml -o /tmp/out.html`
      produces a valid HTML5 document (lint via `tidy -e` if needed).
- [ ] Smoke test against every template:
      `for t in templates/*/index.ezhtml; do ezhtml build "$t" -o /tmp/"$(basename "$(dirname "$t")")".html; done`.
- [ ] Version field in `compiler/Cargo.toml` is `1.0.0`.

## Editor (Electron + React + TS)

- [ ] `cd editor && pnpm lint` — exit code 0
- [ ] `cd editor && pnpm test` — exit code 0
- [ ] `cd editor && pnpm build` — populates `editor/dist/`.
- [ ] `cd editor && pnpm package:dir` — produces an unpacked Electron
      bundle.
- [ ] `cd editor && pnpm package` — produces `.AppImage`, `.dmg` and
      `.exe` bundles.
- [ ] Version field in `editor/package.json` is `1.0.0`.
- [ ] Manual smoke: open `editor/dist/index.html` in a browser and
      load `templates/landing/index.ezhtml` into the editor. Verify
      live preview renders within 250 ms after a keystroke.

## VS Code extension

- [ ] `cd vscode-extension && npm install && npm run build` — exit 0
- [ ] `cd vscode-extension && npx vsce package --no-dependencies` —
      produces `ezhtml-1.0.0.vsix`.
- [ ] Version field in `vscode-extension/package.json` is `1.0.0`.
- [ ] Manual: install the `.vsix` in a fresh VS Code 1.85+ instance
      and verify the `.ezhtml` TextMate grammar kicks in.

## Templates

- [ ] All 8 templates build without warnings.
- [ ] Each template's `assets/css/main.css` is self-contained (no
      external CDN, no build step).
- [ ] Each template's `assets/js/main.js` (when present) runs without
      console errors in a Chrome incognito window.
- [ ] Browser test: serve `templates/landing/dist/index.html` via
      `python3 -m http.server` and visually confirm the navbar toggle,
      theme toggle and smooth-scroll links work.

## Website

- [ ] `website/` renders without broken links or 404s.
- [ ] Discord link appears in `website/index.html`,
      `website/roadmap/index.html`, `website/blog/index.html` and
      `website/docs/index.html` footers plus the homepage CTA card.
- [ ] Gantt chart on `/roadmap/` renders the 6 milestone bars across
      Q2/Q3/Q4 2026 with the `pill-considering` colour for out-of-
      quarter items.
- [ ] Lighthouse audit (manual): score >= 95 for Performance and
      Accessibility on `/` and `/roadmap/`.

## Community & docs

- [ ] 10 strategic files carry the Discord invite (`discord.gg/TQs6McKJJs`).
- [ ] `docs/Roadmap.md` M1-M6 each link to their `docs/RFCs/milestones/M*.md` draft.
- [ ] `SECURITY.md` routes private disclosures to `ezhtml@flazzy.de` and
      non-sensitive follow-ups to Discord `#security`.
- [ ] `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `LICENSE` (MIT) all
      present at repo root.
- [ ] `CHANGELOG.md` has a `[1.0.0]` entry dated today.

## GitHub Release workflow

- [ ] `.github/workflows/release.yml` builds a 4-target matrix
      (linux-musl, macOS x86_64, macOS aarch64, Windows MSVC) plus the
      VS Code extension `.vsix`. Verified on a dry-run tag
      (`v0.99.0-rc1`) before cutting the real tag.
- [ ] Pin actions by SHA (e.g. `actions/checkout@v4` → commit SHA),
      not by tag, to harden supply chain.
- [ ] `softprops/action-gh-release@v2` is configured to upload by
      `${{ env.ASSET }}` so each matrix leg uploads its own tarball.

## Final maintainer steps

- [ ] `git tag -s v1.0.0 -m "EZHTML v1.0.0"`
- [ ] `git push --tags`
- [ ] Confirm CI artifacts uploaded to the GitHub Release draft
- [ ] Replace `SHA256SUMS.txt.placeholder` with verified sums and
      upload that file too
- [ ] Mark the Release **Latest** (not Pre-release)
- [ ] Pin a Discord announcement in `#announcements` of
      <https://discord.gg/TQs6McKJJs>
- [ ] Update the README "Quick Start" curl-bash URL to point at the
      v1.0.0 release asset (or keep `$LATEST` semantics on the web)
