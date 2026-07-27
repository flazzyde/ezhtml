# EZHTML v1.0.0 Release Bundle

This folder contains everything needed to publish the `v1.0.0` GitHub release.

## Files

| File | Purpose |
|------|---------|
| `TITLE.txt` | Paste into the GitHub Release title field |
| `DESCRIPTION.md` | Paste into the GitHub Release description field |
| `RELEASE_NOTES.md` | Optional additional notes |
| `CHECKLIST.md` | Step-by-step publish checklist |
| `build-all.sh` | Script to build all compiler/editor/vsix targets locally |
| `verify-release.sh` | Self-test script to run before clicking Publish |
| `SHA256SUMS.txt` | Checksums for all assets (regenerate after adding binaries) |
| `assets/EXPECTED_ASSETS.txt` | Exact filenames expected in this release |
| `assets/install.sh` | Cross-platform installer script |
| `assets/*.gitkeep` | Slot markers for the 10 release assets |

## Quick steps

1. Drop the 10 release binaries into `assets/` (replace the `.gitkeep` placeholders).
2. Run `cd assets && sha256sum * > ../SHA256SUMS.txt`.
3. Run `bash verify-release.sh`.
4. Copy `TITLE.txt` and `DESCRIPTION.md` into the GitHub Release UI.
5. Drag all files from `assets/` into the release assets area.
6. Publish.
