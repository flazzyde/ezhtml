# ![EZHTML](https://placeholder.pics/svg/150x50/E63946/FFFFFF/EZHTML) EZHTML

**Write web pages as easy as Markdown.**

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-strict-blue.svg)](https://www.typescriptlang.org/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Open Source Love](https://img.shields.io/badge/open%20source-❤-red.svg)](https://github.com)

EZHTML is a **modern, indentation-based markup language** that replaces the
boilerplate of HTML with a few simple keywords. The EZHTML compiler turns
`.ezhtml` files into clean, valid HTML5 – ready for any browser.

```ezhtml
title "EZHTML – Web pages like Markdown"
subtitle "No more closing tags."
text "With EZHTML you write web pages in minutes, not hours."
button "Learn more" "https://ezhtml.flazzy.de"

section
    title "Features"
    row
        card
            title "Easy"
            text "No boilerplate. Just content."
```

EZHTML is **not** a programming language. CSS and JavaScript stay exactly
where they are – we just remove the tag ceremony.

---

## ✨ Features

- 🚀 **Beginner-friendly** – memorable keywords instead of cryptic markup.
- 🪶 **No boilerplate** – `DOCTYPE`, `head`, `meta` are generated automatically.
- 🧱 **Indentation = hierarchy** – no closing tags, no errors from forgotten `</div>`.
- 🛡️ **Validator** – missing alt text, duplicate IDs, SEO issues are reported immediately.
- ⚡ **Fast** – Rust compiler, one single static binary.
- 🧩 **Extensible** – custom elements and themes.
- 🌐 **Open Source** – MIT license.

---

## 📦 Contents of this repository

| Folder                  | Contents                                                   |
| ----------------------- | ---------------------------------------------------------- |
| [`compiler/`](compiler/)                | Rust compiler (`ezhtml` CLI) – tokenizer, parser, emitter  |
| [`editor/`](editor/)                    | Electron + React + TypeScript desktop editor with live preview |
| [`vscode-extension/`](vscode-extension/) | Syntax highlighting, snippets, IntelliSense              |
| [`website/`](website/)                  | Static marketing site with browser playground              |
| [`examples/`](examples/)                | 30+ ready-to-compile `.ezhtml` projects                    |
| [`templates/`](templates/)              | Eight starter templates                                     |
| [`docs/`](docs/)                        | Complete reference documentation                           |
| [`.github/`](.github/)                  | CI/CD, issue and PR templates                              |

---

## 🛠️ Quick Start

### 1. Install the compiler

```bash
# from the releases (recommended)
curl -L https://ezhtml.flazzy.de/install.sh | bash

# or build from this repository
git clone https://github.com/ezhtml/ezhtml.git
cd ezhtml/compiler
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

### 2. First project

```bash
mkdir my-site && cd my-site
ezhtml init
ezhtml build
ezhtml preview
```

### 3. Run the editor

```bash
git clone https://github.com/ezhtml/ezhtml.git
cd ezhtml/editor
pnpm install
pnpm dev
```

---

## 🧑‍💻 Sample projects

See [`examples/`](examples/) – from the minimal `hello-world.ezhtml` to
complex dashboards and landing pages. Each example comes with a
realistic `.ezhtml` file and includes the generated HTML file for
reference.

---

## 🗺️ Roadmap

See [`docs/Roadmap.md`](docs/Roadmap.md). Short version:

- [x] Compiler MVP (tokenizer → parser → emitter)
- [x] Desktop editor with live preview
- [x] VS Code extension
- [x] Browser playground
- [ ] Plugin system for custom elements
- [ ] Theme generator
- [ ] Cloud build service

---

## 🤝 Contributing

We welcome pull requests, issue reports, discussions and themes.
Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) and
[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) before you get started.

---

## 📄 License

[MIT](LICENSE) – Copyright (c) 2026 flazzyde.

---

<p align="center">
  Made with ❤️ for everyone who ever forgot a closing <code>&lt;/div&gt;</code>.
</p>
