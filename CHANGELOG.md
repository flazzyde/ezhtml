# Changelog

All notable changes to **EZHTML** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
