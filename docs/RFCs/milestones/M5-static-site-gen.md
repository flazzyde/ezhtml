# M5 · Static site generator

**Target quarter:** Q4 2026
**Owner:** flazzyde
**Labels:** `roadmap`, `cli`, `site-gen`, `milestone-m5`
**Tracks under:** Q4 2026 · Ecosystem stream

## Goal

Add an `ezhtml site` sub-command that compiles an entire directory of
`.ezhtml` files into a navigable static site with index pages, RSS
and sitemap support.

## Acceptance criteria

- [ ] `ezhtml site src/ -o public/` walks a source directory tree,
      compiles each `.ezhtml` file, and copies assets verbatim.
- [ ] Per-folder `index.ezhtml` files are generated automatically if
      absent (a directory listing the contained pages).
- [ ] RSS feed (`feed.xml`) and basic sitemap (`sitemap.xml`) are
      written for projects that opt in via `project.ez`.
- [ ] One end-to-end site under `examples/site-blog/` proves the
      feature — local preview by serving `public/` over `python -m
      http.server` is documented in the example README.
- [ ] The site generator does not depend on the Markdown plugin (M3);
      they compose cleanly.

## Non-goals

- Pagination beyond ten-per-page.
- A web UI for browsing generator output.
- Hot-reload during site-gen development. Standard `ezhtml
  preview` covers local editing.

## Risks

| Risk | Mitigation |
| ---- | ---------- |
| Auto-generated indexes clash with user-authored directory indexes. | Opt-in via `project.ez` `auto_index: true`; otherwise leave the directory untouched. |
| Asset copying duplicates files shared across pages. | Symlink by default if the source allows it (Linux/macOS only); copy on Windows. |

## Tracking

- Issues / PRs labelled `milestone-m5`.
- Discord: `#compiler` and `#showcase` channels.
- Status: `🔜 scheduled`
