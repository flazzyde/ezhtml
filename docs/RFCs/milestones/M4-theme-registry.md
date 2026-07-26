# M4 · Theme registry MVP

**Target quarter:** Q3 2026
**Owner:** flazzyde
**Labels:** `roadmap`, `themes`, `milestone-m4`
**Tracks under:** Q3 2026 · Ecosystem stream

## Goal

Make it trivially easy to switch the look-and-feel of any EZHTML
output without touching project source.

## Acceptance criteria

- [ ] A new `theme "<name>"` directive (top-level keyword, same family
      as `!lang`) is parsed and documented.
- [ ] Three reference themes ship under `themes/`:
      `midnight`, `ocean`, `paper`.
- [ ] `ezhtml build --theme <name>` resolves the theme via:
      1. `./themes/<name>.css`
      2. `$EZHTML_THEMES_DIR/<name>.css`
      3. `<exe-dir>/../share/ezhtml/themes/<name>.css`
- [ ] A CI smoke test runs the landing template against each theme
      and uploads a screenshot per theme to the build artefact.
- [ ] The website theme picker (currently in the editor) starts to
      pull from the central registry.

## Non-goals

- A web marketplace. Just a directory layout for v0.
- Per-element theming. Themes swap the global CSS file, not partial
  overrides.

## Risks

| Risk | Mitigation |
| ---- | ---------- |
| Theme CSS variable names drift away from the compiler-emitted class names. | Lock the class names in `docs/Theme.md` and have CI check chosen themes against the canonical test page. |

## Tracking

- Issues / PRs labelled `milestone-m4`.
- Discord: `#themes` channel.
- Status: `💭 considering`
