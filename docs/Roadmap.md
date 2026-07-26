# Roadmap

The living roadmap for **EZHTML** — what's shipped, what's coming next,
and how the community shapes what comes after.

## Status legend

| Symbol | Meaning                                                      |
| ------ | ------------------------------------------------------------ |
| ✅     | Shipped and available in a tagged release.                    |
| 🚧     | In progress — code in a branch, RFC open, or sprint scoped.   |
| 🔜     | Scheduled — accepted into the current quarter's milestone.    |
| 💭     | Considering — open discussion, no commitment yet.             |
| ❌     | Dropped or replaced by a different approach.                  |

Quarters roll over on the **first Monday** of January, April, July and
October. Milestone targets are realistic, not promises.

---

## What's next · Q2 2026

Three streams are running in parallel. Pick the one most relevant to
what you want to ship:

### 🧪 Compiler stream — "EZHTML v0.3"

- 🚧 **WASM build of the compiler** so the browser playground compiles
  entirely client-side, no network round-trip.
- 🚧 **Snapshot test harness** stabilised around `insta` so PRs cannot
  regress output without an explicit review.
- 🔜 **`ezhtml lint` differentiates from `ezhtml doctor`** — style checks
  (line length, hard-tabs, duplicate `description`) become their own
  command with `errors/warnings/info knobs`.
- 🔜 **Richer `project.ez`** — multiple author profiles, RSS/Atom
  feed metadata, sitemap URLs.

### 🛠️ Editor stream — "Live, faster, prettier"

- 🚧 **Faster preview** — replace the file-system polling with a
  `notify` watcher; expected <50 ms roundtrip from save to browser.
- 🔜 **Inline diagnostics gutter** in Monaco using the existing
  `CompileReport`.
- 🔜 **Per-template starter picker on `File → New`** so users can
  scaffold from the editor without dropping to the terminal.

### 🌐 Ecosystem stream — "Beyond the binary"

- 🔜 **`.ezplugin` file format** — define a custom element with
  `keyword "x-easypie"`, maps to `<div class="x-easypie">` plus a hook.
- 🔜 **Theme registry** — opt-in `theme "midnight"` directive chooses a
  pre-bundled CSS file at build time; themes live in a separate
  `themes/` repo and are consumed via git subtree or download.
- 🔜 **Static site generator** — `ezhtml site src/ -o public/` walks
  a directory, compiles each `.ezhtml` and writes a navigable site.

---

## Phase history

### Phase 0 · Foundation ✅

- [x] Repository skeleton (`LICENSE`, `README`, `CONTRIBUTING`, etc.)
- [x] `.gitignore` covering secrets and build outputs
- [x] Issue & PR templates
- [x] CI / CD workflows for Rust, Node and VS Code extension

### Phase 1 · Compiler MVP ✅  (v0.1)

- [x] Full pipeline: Tokenizer → Parser → Validator → Emitter
- [x] `<head>` scaffold (UTF-8, viewport, OpenGraph, Twitter Cards)
- [x] All 28 documented elements
- [x] Validator: alt text, link `href`, table columns, raw-html safety
- [x] CLI: `build`, `run`, `preview`, `init`, `doctor`, `format`,
      `lint`, `version`
- [x] Project files (`project.ez`, JSON, TOML)
- [x] Formatter (whitespace collapse)
- [x] Tests (unit, integration, parser, tokenizer, AST)
- [x] 32 example projects
- [x] 8 starter templates (folder-based, with assets and JS)

### Phase 2 · Desktop editor ✅  (v0.2)

- [x] Electron + React + TypeScript shell
- [x] Live preview side panel
- [x] File explorer & tabs
- [x] Dark mode / Light mode toggle
- [x] Monaco-based code editor
- [x] IntelliSense autocomplete
- [x] Settings panel
- [x] Theme picker (default, midnight, ocean)

### Phase 3 · VS Code extension ✅  (v0.2)

- [x] TextMate grammar (syntax highlighting)
- [x] Snippet pack
- [x] Bracket pair colourisation
- [x] Minimap icons
- [x] Branded editor icon

### Phase 4 · Website ✅  (v0.1)

- [x] Landing page (`index.html`)
- [x] Browser playground (`/playground`)
- [x] Download page for the compiler binary
- [x] Documentation hub (`/docs`)
- [x] Roadmap mirror (`/roadmap`)
- [x] Blog placeholder (`/blog`)
- [x] Open Graph + Twitter Cards
- [x] Discord link in the footer

### Phase 5 · Ecosystem (next)

- [ ] Plugin system: define custom elements via `.ezplugin` files
- [ ] Theme registry / marketplace (CSS drops shipped as git subtrees)
- [ ] Cloud build service (free tier + paid hosted previews)
- [ ] Cursor / Windsurf support (separate snippet pack + LSP)
- [ ] Storybook integration (emit stories from `*.ezhtml`)
- [ ] Static site generator (`ezhtml site src/ -o public/`)

### Phase 6 · Long-term

- [ ] WASM build of the compiler (playground + edge functions)
- [ ] Language Server Protocol for any editor
- [ ] Macro language for repeating sections (`for each post in posts:`)
- [ ] Component imports (a la JSX): `import Nav from "components/nav"`)
- [ ] Visual diff between two `.ezhtml` files

---

## Milestones · next 90 days

Each milestone has an **acceptance criterion** so we know when it's
done — not "vibes".

### M1 · v0.3.0-rc.1 · [RFC](milestones/M1-v0.3-rc1.md)  (target: end of April 2026)

- WASM build passes the compiler test suite (`cargo test --target
  wasm32-unknown-unknown`).
- New `ezhtml.wasm` npm package published.
- Browser playground loads the WASM with no server round-trip.
- Two templates (`landing`, `docs`) re-recorded as snapshot tests.

### M2 · `ezhtml lint` is its own command · [RFC](milestones/M2-ezhtml-lint.md)  (target: May 2026)

- `lint` accepts the same input arguments as `doctor` and `format`.
- Output is exit-coded (0 = clean, 1 = warnings, 2 = errors).
- New rules: line length, hard-tab usage, duplicate `description`,
  unused `project.ez` keys.

### M3 · Plugin format v0 · [RFC](milestones/M3-plugin-format.md)  (target: June 2026)

- Spec for `.ezplugin` written; reference implementation merged.
- One example plugin (`x-easypie`) shipped.
- Error messages point users to the SPEC file when a plugin fails to
  load.

### M4 · Theme registry MVP · [RFC](milestones/M4-theme-registry.md)  (target: June 2026)

- `theme "name"` directive documented.
- Three reference themes committed (`midnight`, `ocean`, `paper`).
- CI smoke-tests each theme against the landing template.

### M5 · Static site generator · [RFC](milestones/M5-static-site-gen.md)  (target: Q3 2026)

- `ezhtml site src/ -o public/` walks a directory tree.
- Generates `index.html`, per-folder index, RSS feed.
- One site in `examples/` proves the feature end-to-end.

### M6 · LSP prototype · [RFC](milestones/M6-lsp-prototype.md)  (target: Q3 2026)

- `ezhtml-lsp` binary speaks the LSP protocol.
- Hover, go-to-definition and completion work in VS Code.
- Released as a separate `vscode-extension-lsp` package.

---

## How to influence the roadmap

The roadmap is **community-driven**. Three places to make your voice
heard:

1. **Discord.** Drop into the
   [`#roadmap`](https://discord.gg/TQs6McKJJs) channel — quick async
   discussion, weekly office hours, no waiting on a GitHub notification.
2. **GitHub issues.** Tag your request `roadmap` and we'll triage it
   into the next milestone review.
3. **RFCs** — for anything breaking or large, open a pull request
   against [`docs/RFCs/`](RFCs/) with context, alternatives and a
   decision record.

We pick the next 90 days of work during the last week of each
quarter. Anyone can observe — Discord link below.

---

## 📣 Join the community

The fastest way to keep up with progress, ask questions, and shape the
roadmap is the EZHTML Discord:

> 🔗 **<https://discord.gg/TQs6McKJJs>**

The `#roadmap`, `#help`, `#showcase` and `#compiler` channels cover the
vast majority of day-to-day discussion. There's a quarterly office hour
in `#general` the first Wednesday of each new quarter.

---

## Release cadence

- **Patch releases** (`v0.2.1`, `v0.2.2`): as needed for regressions.
- **Minor releases** (`v0.3`, `v0.4`): quarterly, scoped to the
  milestone list above.
- **Major releases** (`v1.0`): once the API surface is stable for 6
  months and the validator covers the HTML5 spec.

There is no commitment to a strict calendar — the milestone *list*
above is what we plan around, not dates. We move items between
milestones when reality requires it, and announce those moves on
Discord before the next release is cut.
