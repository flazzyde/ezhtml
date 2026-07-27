# Release Notes · EZHTML v1.0.0

> "Markdown for HTML" — EZHTML ships its first stable release.
> Indentation defines the structure. Closing tags stay home.

## Highlights

- **First stable compiler (`ezhtml` CLI).** The full pipeline
  Tokenizer → Parser → Validator → Emitter ships frozen. All 28
  documented elements (title, subtitle, text, button, image, video,
  link, header, footer, navbar, section, container, row, column, card,
  list, item, table, input, email, password, checkbox, radio,
  textarea, code, quote, divider, space, icon, html) compile to valid,
  pretty-printed HTML5.
- **Project-file discovery.** A `project.ez` (or `.json`, `.toml`,
  `config.ez`, `settings.ez`) next to your `.ezhtml` source feeds the
  full `<head>` scaffold: charset, viewport, title, description,
  author, keywords, theme colour, favicon, manifest, OpenGraph and
  Twitter Card meta tags. Generated automatically.
- **Eight starter templates** under `templates/` ship as full
  folder-based projects — each one has an `index.ezhtml`, a
  `project.ez`, a `README.md` and a self-contained `assets/` tree
  with CSS and (where useful) vanilla JavaScript. Build a real site in
  one `ezhtml init` command.
- **Desktop editor** (`Electron + React + TypeScript + Vite`)
  bundles a Monaco-backed editor, a debounced live preview, a file
  explorer with tabs, dark/light theme toggle and a snippet picker.
- **VS Code extension** ships a TextMate grammar, a snippet pack,
  formatter and a "Open Documentation" command.
- **Website & playground** under `website/` serve a static
  marketing landing, a docs hub, a browser playground and a roadmap
  page with a visual swimlane Gantt and clickable RFC links.
- **Documentation** under `docs/` covers every keyword, the
  compiler architecture, the CLI surface and the theming guide.

## Improvements & polish

- Validator surfaces **39+ diagnostics** for image alt-text, link
  href, table column consistency, raw HTML safety and more.
- Indentation units of 2 or 4 spaces are both accepted and inferred
  per file.
- CLI is fully colorised when stdout is a TTY (`EZHTML_NO_COLOR` to
  opt out).
- `ezhtml init --template <name>` walks four filesystem locations
  (`$EZHTML_TEMPLATES_DIR` → `<exe-dir>/../templates/` →
  `<exe-dir>/templates/` → `./templates/`) and copies the WHOLE
  template folder, refusing to scaffold into a non-empty directory.
- The roadmap page renders as a horizontal Gantt (`Q2 / Q3 / Q4 2026`)
  + a 7-step phase stepper + a Discord CTA card.
- Discord is now placed in **10 strategic files** (README badge +
  Contributing section, docs/Roadmap, SECURITY, vscode-extension, blog,
  docs page, templates/README, website/index + website/roadmap).
- Six milestone RFC drafts (`docs/RFCs/milestones/M1-M6.md`) link
  each roadmap entry to acceptance criteria, non-goals and risks.

## Bug fixes since 0.1.x

- Fixed: `<section>` outside any other block no longer eats a
  trailing `<hr>` divider.
- Fixed: blank URL in a `link` keyword no longer produces a literal
  `href=""` (validator warning W0103 is now confirmed).
- Fixed: `ezhtml build -o` no longer fails on Windows paths that
  contain spaces.
- Fixed: long input strings no longer overflow the formatter.

## Breaking changes

None. v1.0.0 is the first stable tag. The CLI argument surface,
emitted HTML structure and CSS class names are now frozen for the
`1.x` line; future changes will go through an RFC in
`docs/RFCs/` first.

## Install

### Compiler (`ezhtml` CLI)

| Operating system | One-liner                                                                                       |
| ----------------- | ----------------------------------------------------------------------------------------------- |
| Linux (glibc)     | `curl -L https://ezhtml.flazzy.de/install.sh \| bash`                                          |
| Linux (musl)      | Download `ezhtml-v1.0.0-x86_64-unknown-linux-musl.tar.gz` from the Assets, extract, run.       |
| macOS (Apple)     | `brew install ezhtml`                                                                          |
| macOS (Intel)     | Download `ezhtml-v1.0.0-x86_64-apple-darwin.tar.gz` from the Assets, extract, run.             |
| Windows (x64)     | Download `ezhtml-v1.0.0-x86_64-pc-windows-msvc.zip` from the Assets, unzip, add to PATH.       |
| crates.io mirror  | `cargo install ezhtml-cli --version 1.0.0`                                                     |

### Editor

Download the appropriate bundle from the Assets:

- `ezhtml-editor-v1.0.0.AppImage` (Linux)
- `ezhtml-editor-v1.0.0.dmg` (macOS)
- `ezhtml-editor-v1.0.0.exe` (Windows)

The editor finds the bundled `ezhtml` binary automatically.

### VS Code extension

In VS Code, run:

```
ext install flazzyde.ezhtml
```

…or grab `ezhtml-vscode-v1.0.0.vsix` from the Assets and
`code --install-extension ezhtml-vscode-v1.0.0.vsix`.

## Verifying the binaries

```bash
curl -L https://github.com/flazzyde/ezhtml/releases/download/v1.0.0/SHA256SUMS.txt -o SHA256SUMS.txt
sha256sum -c SHA256SUMS.txt
```

…or use the bundled `verify-release.sh` from this folder. Any
mismatch means tampering -- do NOT run an unverified binary.

## Acknowledgements

EZHTML v1.0.0 is the work of **flazzyde** with help from the
contributors listed by `git log --shortlog`. Early reporting,
template art and copy-editing by the community at
<https://discord.gg/TQs6McKJJs>.

Special thanks to:

- the **Discord community** for 90 days of RFC discussions;
- every **bug reporter** who shipped a `.ezhtml` reproducer;
- every **template author** who shared what they built.

## See also

- Roadmap: [`docs/Roadmap.md`](../../docs/Roadmap.md)
- Roadmap mirror: <https://ezhtml.flazzy.de/roadmap>
- CHANGELOG: [`CHANGELOG.md`](../../CHANGELOG.md)
- Project home: <https://ezhtml.flazzy.de>
- Discord: <https://discord.gg/TQs6McKJJs>

---

🪶 *Made with EZHTML for everyone who ever forgot a closing
`</div>`.*
