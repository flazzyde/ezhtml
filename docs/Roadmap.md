# Roadmap

This is the living roadmap for **EZHTML**. Items are grouped by phase.
A ✅ means shipped in `0.1.0`; a 🚧 means in progress; a 🔜 means
scheduled; an empty checkbox is a future idea we haven't prioritised
yet.

## Phase 0 — Foundation ✅

- [x] Repository skeleton (LICENSE, README, CONTRIBUTING, etc.)
- [x] `.gitignore` covering secrets and build outputs
- [x] Issue & PR templates
- [x] CI / CD workflows for Rust, Node and VS Code extension

## Phase 1 — Compiler MVP ✅

- [x] Full pipeline: Tokenizer → Parser → Validator → Emitter
- [x] `<head>` scaffold (UTF-8, viewport, OpenGraph, Twitter Cards)
- [x] All 28 documented elements
- [x] Validator: alt text, link href, table columns, raw-html safety
- [x] CLI: build, run, preview, init, doctor, format, lint, version
- [x] Project files (`project.ez`, JSON, TOML)
- [x] Formatter (whitespace collapse)
- [x] Tests (unit, integration, parser, tokenizer, AST)
- [x] 32 example projects
- [x] 8 starter templates

## Phase 2 — Editor ✅

- [x] Electron + React + TypeScript shell
- [x] Live preview side panel
- [x] File explorer & tabs
- [x] Dark mode / Light mode toggle
- [x] Monaco-based code editor
- [x] IntelliSense autocomplete
- [x] Settings panel
- [x] Theme picker (default, midnight, ocean)

## Phase 3 — VS Code Extension ✅

- [x] TextMate grammar (syntax highlighting)
- [x] Snippet pack
- [x] Bracket pair colourisation
- [x] Minimap icons
- [x] Branded editor icon

## Phase 4 — Website ✅

- [x] Landing page (`index.html`)
- [x] Browser playground (`/playground`)
- [x] Download page for the compiler binary
- [x] Documentation hub (`/docs`)
- [x] Roadmap mirror (`/roadmap`)
- [x] Blog placeholder (`/blog`)
- [x] Open Graph + Twitter Cards

## Phase 5 — Ecosystem (next)

- [ ] Plugin system: define custom elements via `.ezplugin` files
- [ ] Theme marketplace (CSS drops shipped as git submodules)
- [ ] Cloud build service (free tier + paid hosted previews)
- [ ] Cursor / Windsurf support (separate snippet pack + LSP)
- [ ] Storybook integration (emit stories from `*.ezhtml`)
- [ ] Static site generator (`ezhtml site src/ -o public/`)

## Phase 6 — Long-term

- [ ] WASM build of the compiler for the playground and edge functions
- [ ] Language Server Protocol for any editor
- [ ] Macro language for repeating sections
- [ ] Component imports (like JSX)
- [ ] Visual diff between two `.ezhtml` files

## How to influence the roadmap

Open an issue tagged `roadmap` — or comment on the
[tracking discussion](#). We pick the next 90 days of work during the
last week of each quarter.
