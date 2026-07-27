## The first stable release of the EZHTML markup language

EZHTML turns indented, Markdown-style source into clean, valid HTML5 - no closing tags, no boilerplate, no `<div>` soup. v1.0.0 freezes the syntax, the compiler, the editor and the toolchain.

### What's in this release

- **Compiler (`ezhtml`)** - single statically-linked binary for Linux (glibc/musl, x86_64 + aarch64), macOS (Intel + Apple Silicon) and Windows. Sub-millisecond compile, ~3 MB binary.
- **Editor** - Electron + React + Monaco desktop app with live preview, dark/light toggle, file explorer with tabs, snippet picker and IntelliSense for all 28 elements.
- **VS Code extension** - TextMate grammar, snippets, formatter, "Open Documentation" command.
- **Templates** - 8 starter projects (blank, minimal, landing, portfolio, blog, dashboard, docs, company). Each ships full assets/css + assets/js. Real websites in one `ezhtml init` command.
- **Website & playground** - static marketing landing, docs hub, browser playground and a roadmap page with Q2/Q3/Q4 Gantt.
- **Documentation** - every keyword, the compiler architecture, the CLI surface and the theming guide under `docs/`.

### Install

```bash
# Linux
curl -L https://ezhtml.flazzy.de/install.sh | bash

# macOS
brew install ezhtml

# Windows
# download ezhtml-v1.0.0-x86_64-pc-windows-msvc.zip from Assets below
```

### Highlights

- **28 documented elements** compile to valid HTML5 - title, subtitle, text, button, image, video, link, header, footer, navbar, section, container, row, column, card, list, item, table, input, email, password, checkbox, radio, textarea, code, quote, divider, space, icon, html.
- **Auto-generated `<head>`** - charset, viewport, title, description, author, keywords, theme color, favicon, manifest, OpenGraph, Twitter Cards. Driven by `project.ez` next to your `.ezhtml` source.
- **39+ validator diagnostics** - alt-text, link href, table column consistency, raw-HTML safety.
- **Live preview** debounced to ≤250 ms after each keystroke.
- **Gantt roadmap** at https://ezhtml.flazzy.de/roadmap with M1–M6 RFCs already drafted under `docs/RFCs/milestones/`.

### Breaking changes

None. v1.0.0 is the first stable tag. The CLI surface, emitted HTML structure and CSS class names are frozen for the `1.x` line - future changes go through an RFC in `docs/RFCs/` first.

### Community

Discord: <https://discord.gg/TQs6McKJJs>
Project home: <https://ezhtml.flazzy.de>

---

🪶 *Made with EZHTML for everyone who ever forgot a closing `</div>`.*
