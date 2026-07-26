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
title "EZHTML – Webseiten wie Markdown"
subtitle "No more closing tags."
text "Mit EZHTML schreibst du Webseiten in Minuten, nicht in Stunden."
button "Learn more" "https://ezhtml.flazzy.de"

section
    title "Features"
    row
        card
            title "Einfach"
            text "Kein boilerplate. Nur Inhalt."
```

EZHTML is **not** a programming language. CSS and JavaScript stay exactly
where they are – we just remove the tag ceremony.

---

## ✨ Features

- 🚀 **Beginner-friendly** – einpraegsame Schluesselwoerter statt kryptischem Markup.
- 🪶 **Kein boilerplate** – `DOCTYPE`, `head`, `meta` werden automatisch erzeugt.
- 🧱 **indentation = hierarchy** – keine schliessenden Tags, keine Fehler durch vergessene `</div>`.
- 🛡️ **Validator** – fehlende Alt-Texte, doppelte IDs, SEO-Probleme werden sofort gemeldet.
- ⚡ **Fast** – Rust-Compiler, eine einzige statische Binary.
- 🧩 **Extensible** – eigene Elemente und Themes.
- 🌐 **Open Source** – MIT-License.

---

## 📦 Contents of this repository

| Ordner                | Inhalt                                                        |
| --------------------- | ------------------------------------------------------------- |
| [`compiler/`](compiler/)         | Rust-Compiler (`ezhtml` CLI) – Tokenizer, Parser, Emitter     |
| [`editor/`](editor/)             | Electron + React + TypeScript Desktop-Editor mit Live-Preview |
| [`vscode-extension/`](vscode-extension/) | Syntax-Highlighting, Snippets, IntelliSense            |
| [`website/`](website/)           | Statische Marketing-Site mit Browser-Playground               |
| [`examples/`](examples/)         | 30+ fertige `.ezhtml`-Projekte                                |
| [`templates/`](templates/)       | Acht Starter-Templates                                        |
| [`docs/`](docs/)                 | Vollstaendige Referenz-documentation                           |
| [`.github/`](.github/)           | CI/CD, Issue- und PR-Templates                                |

---

## 🛠️ Quick Start

### 1. Compiler installieren

```bash
# aus den Releases (empfohlen)
curl -L https://ezhtml.flazzy.de/install.sh | bash

# or aus diesem Repo bauen
git clone https://github.com/ezhtml/ezhtml.git
cd ezhtml/compiler
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

### 2. Erstes Projekt

```bash
mkdir my-site && cd my-site
ezhtml init
ezhtml build
ezhtml preview
```

### 3. Editor starten

```bash
git clone https://github.com/ezhtml/ezhtml.git
cd ezhtml/editor
pnpm install
pnpm dev
```

---

## 🧑‍💻 Sample projects

Schau in [`examples/`](examples/) – vom minimalen `hello-world.ezhtml` bis
hin zu komplexen Dashboards und Landing-Pages. Jedes Example ist mit einer
realistischen `.ezhtml`-Datei ausgestattet und enthaelt die zugehoerige
generierte HTML-Datei zur Referenz.

---

## 🗺️ Roadmap

Siehe [`docs/Roadmap.md`](docs/Roadmap.md). Kurzfassung:

- [x] Compiler MVP (Tokenizer → Parser → Emitter)
- [x] Desktop-Editor mit Live-Preview
- [x] VS Code Extension
- [x] Browser-Playground
- [ ] Plugin-System fuer eigene Elemente
- [ ] Theme-Generator
- [ ] Cloud-Build-Service

---

## 🤝 Contributing

Wir freuen uns ueber Pull Requests, Issue-Reports, Diskussionen und Themes.
Bitte lies [`CONTRIBUTING.md`](CONTRIBUTING.md) und
[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) bevor du loslegst.

---

## 📄 License

[MIT](LICENSE) – Copyright (c) 2025 flazzyde.

---

<p align="center">
  Made with ❤️ for everyone who ever forgot a closing <code>&lt;/div&gt;</code>.
</p>
