# Changelog

All notable changes to **EZHTML** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-07-27

### Added

- **Stable Rust compiler (`ezhtml` CLI) 1.0.0.** The full pipeline
  Tokenizer -> Parser -> Validator -> Emitter ships frozen. All 28
  documented elements compile to valid HTML5.
- **Project-file discovery** (`project.ez`, `config.ez`,
  `settings.ez`, `site.ez.json`, `site.ez.toml`) feeding the full
  `<head>` scaffold (UTF-8, viewport, title, description, theme color,
  favicon, manifest, OpenGraph, Twitter Card meta tags) automatically.
- **CLI commands**: `build`, `run`, `preview`, `init`, `doctor`,
  `format`, `lint`, `version` (8 sub-commands, all implemented).
- **Templates** (`templates/`): blank, minimal, landing, blog,
  portfolio, dashboard, docs, company. Ship as folder projects with
  a self-contained `assets/` tree (CSS + JS where applicable).
- **Examples** (`examples/`): 32 ready-to-compile projects plus the
  newly added `31-team-page` and `32-invoice` examples.
- **Documentation** (`docs/`): Syntax, Compiler, CLI, Roadmap, FAQ,
  Theme, plus six Milestone RFC drafts under `docs/RFCs/milestones/`.
- **Desktop editor** (`editor/`): Electron + React + TypeScript + Vite
  with Monaco, debounced live preview, dark/light mode, file explorer.
- **VS Code extension** (`vscode-extension/`): TextMate grammar,
  snippet pack, formatter, Open-Documentation command.
- **Website** (`website/`): landing page, browser playground, docs
  hub, download page, blog placeholder, roadmap mirror with a visual
  Gantt swimlane.
- **Discord community hub**: invite placed in 10 strategic files
  (README, contributing, roadmap, security, vscode-extension README,
  blog, docs, templates README, homepage CTA, roadmap CTA).

### Changed

- Version bumped from `0.1.0` to `1.0.0` in `compiler/Cargo.toml`,
  `editor/package.json` and `vscode-extension/package.json`.
- CLI surface, emitted HTML structure and CSS class names are now
  frozen for the `1.x` line. Future changes require an RFC.
- Migration to MIT-licensed, Discord-first community workflow.

### Fixed

- `section` outside any block no longer eats a trailing `<hr>` divider.
- Blank URL in `link` keyword no longer produces literal `href=""`.
- `ezhtml build -o` no longer fails on Windows paths with spaces.
- Long input strings no longer overflow the formatter.

### Security

- Path traversal in `ezhtml init` is mitigated: target directory must
  be empty, symlinks in template folders are skipped.
- All Discord external anchors carry `rel="noopener noreferrer"`.

---

## [0.1.0] – 2026-XX-XX

### Added

- **Rust compiler** (`compiler/`): single-binary CLI implementing the full
  pipeline (Tokenizer → Parser → Validator → Emitter). All 28 elements
  documented in `docs/Syntax.md` are supported.
- **Project-file discovery** (`project.ez`, `config.ez`, `settings.ez`,
  `site.ez.json`, `site.ez.toml`) feeding `<head>` metadata
  automatically (UTF-8, viewport, OpenGraph, Twitter Cards, theme-color,
  favicon, manifest).
- **CLI commands**: `build`, `run`, `preview`, `init`, `doctor`, `format`,
  `lint`, `version`.
- **Templates** (`templates/`): blank, minimal, landing, blog, portfolio,
  dashboard, docs, company.
- **Examples** (`examples/`): 32 ready-to-compile projects, from
  `01-hello-world.ezhtml` to `32-invoice.ezhtml`.
- **Documentation** (`docs/`): Syntax, Compiler, CLI, Roadmap, FAQ,
  Theme.
- **Desktop editor** (`editor/`): Electron + React + TypeScript + Vite.
  Explorer, tabs, Monaco-based code editor, live preview via the Rust
  compiler (debounced), dark/light mode, split layout.
- **VS Code extension** (`vscode-extension/`): TextMate grammar, snippet
  pack, whitespace formatter, command palette commands.
- **Website** (`website/`): landing page, browser playground, docs hub,
  download page, blog placeholder, roadmap mirror.
- **GitHub configuration** (`.github/`): CI for Rust + Node + extension,
  release workflow for binaries & `.vsix`, issue & PR templates.
- **Icons** (`icons/`): primary logo, monochrome variant, mark only.
- **`.gitignore`** that protects secrets, build outputs and runtime
  caches (`env`, `node_modules`, `target`, `dist`, …).
- Initial `LICENSE` (MIT), `README`, `SECURITY`, `CONTRIBUTING`,
  `CODE_OF_CONDUCT`.

### Known caveats

- The first release of the prebuilt binaries is not yet published.
  Build from source with `cargo build --release` in `compiler/`.
- The browser playground runs a minimal client-side shim. WASM
  compilation of the full Rust pipeline is scheduled for 0.2.
