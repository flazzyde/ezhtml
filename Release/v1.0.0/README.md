# Release · EZHTML v1.0.0

This folder is a **maintainer-friendly artifact pack** for publishing
the first stable release of EZHTML. Every file below goes into the
GitHub Release UI; nothing here is consumed by the compiler, editor or
extension at runtime.

## What is in this folder?

| File                                 | Purpose                                                                                  |
| ------------------------------------ | ---------------------------------------------------------------------------------------- |
| `README.md`                          | This file.                                                                               |
| `CHECKLIST.md`                       | Production-readiness gates that v1.0.0 must clear before tagging.                        |
| `TITLE.md`                           | Exact Git tag + Release title phrase. Copy/paste into the GitHub Release UI.             |
| `DESCRIPTION.md`                     | Markdown body that appears under the release title on GitHub. Female the headline.       |
| `RELEASE_NOTES.md`                   | Full release notes (highlights, fixes, install matrix, thanks). Linked from DESCRIPTION. |
| `SHA256SUMS.txt.placeholder`         | Templated SHA-256 manifest. Manually filled after the release workflow attaches files.  |
| `build-all.sh`                       | One-command *local* build script that produces every binary/asset on this machine.       |
| `verify-release.sh`                  | SHA-256 verifier + smoke compile-test every binary against the language spec.           |
| `assets/`                            | Per-asset placeholder notes explaining what each archive contains.                        |
| `assets/install.sh`                  | CURL-driven installer for Linux & macOS (idempotent, picks up the latest release).       |

## How to use this pack

1. **Run `CHECKLIST.md`** -- every gate must be green before tagging.
2. **Run `build-all.sh`** locally (or let CI do it via the
   `.github/workflows/release.yml` workflow).
3. **Verify** with `verify-release.sh`. Replace the placeholder
   `SHA256SUMS.txt.placeholder` content with the verified sums.
4. **Tag** the commit: `git tag -s v1.0.0 -m "EZHTML v1.0.0"` then
   `git push --tags`.
5. **Publish** the GitHub Release:
   - Title from `TITLE.md`
   - Short body from `DESCRIPTION.md`
   - Long notes from `RELEASE_NOTES.md`
   - Drag every file from `assets/` (the produced `.tar.gz`, `.zip`,
     `.dmg`, `.AppImage`, `.exe`, `.vsix`) into the Assets picker.
   - Drop the filled `SHA256SUMS.txt` into the Assets picker too.
   - Mark as **Latest** (not Pre-release).

## Compatibility

- **Compiler (Rust):** builds on Rust stable >= 1.75 with the usual
  cross-compile targets. Tested via the CI matrix on Linux/macOS/Windows.
- **Editor (Electron):** Node 20+, pnpm 9. Builds the renderer via
  Vite, packages the desktop binary via `electron-builder`.
- **VS Code extension:** VS Code >= 1.85. Packaged via
  `vsce package`.

## Where to go after tagging

Once the release is published:

- Mirror the 6 milestone RFC drafts to GitHub issues via
  `bash docs/RFCs/milestones/create-issues.sh` (requires
  `gh auth login`).
- Pin the v1.0.0 release in the Discord
  [@EZHTML Discord](https://discord.gg/TQs6McKJJs) `#announcements`
  channel.
- Mirror the `install.sh` to `https://ezhtml.flazzy.de/install.sh`
  so `curl | bash` keeps working.
- (Optional) Mirror the compiler to `crates.io` once the maintainer
  has a crates.io account.

## License

Everything here is MIT-licensed under the same terms as the rest of
the EZHTML project. See [`LICENSE`](../../LICENSE).
