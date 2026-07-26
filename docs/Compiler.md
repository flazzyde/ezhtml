# Compiler Architecture

The EZHTML compiler is a single Rust crate (`compiler/`) that compiles
`.ezhtml` source files into HTML5. This document walks through every
phase of the pipeline.

## Pipeline

```
            ┌──────────────┐
.ezhtml ──▶ │  Tokenizer   │   → Vec<Token>  (incl. INDENT / DEDENT)
            ├──────────────┤
            │   Parser     │   → Document (AST)
            ├──────────────┤
            │  Validator   │   → CompileReport (warnings/infos)
            ├──────────────┤
            │   Emitter    │   → String (HTML5)
            └──────────────┘
```

Each phase is a pure function on immutable inputs (except the validator,
which reads from the AST for placeholders).

## Tokenizer

Located in [`compiler/src/tokenizer.rs`](../compiler/src/tokenizer.rs).

Reads raw UTF-8, emits a flat stream of tokens:

- `Keyword(name)` — known element opener like `title`, `row`.
- `Identifier(name)` — unknown element; treated as warning later.
- `String(value)` — quoted string literal with escape support
  (`\"`, `\\`, `\n`, `\t`).
- `Bare(value)` — unquoted token.
- `Indent` / `Dedent` — explicit indentation signals so the parser can
  treat blocks like Python.
- `Newline` — end of line.
- `Directive(text)` — `!`-prefixed line.
- `Eof` — end of input.

The tokenizer also enforces that the indentation unit is consistent
within a file. The first non-zero indent becomes the unit (typically 2
or 4 spaces).

## Parser

Located in [`compiler/src/parser.rs`](../compiler/src/parser.rs).

A recursive-descent parser that consumes INDENT/DEDENT tokens. Each
keyword is matched to a [`NodeKind`](../compiler/src/ast.rs) variant:

| Keyword        | NodeKind           |
| -------------- | ------------------ |
| `title`        | `Title(String)`    |
| `subtitle`     | `Subtitle(String)` |
| `section`      | `Section(Vec<Node>)` |
| `link`         | `Link { href, children }` |
| `table`        | `Table { headers, rows }` |
| `image`        | `Image { src, alt }` |
| …              | …                  |

The parser also recognises unknown `Identifier` tokens and emits a
**warning** (severity `Warning`, code `W0200` / `W0201`) so the
surrounding document still renders. This matches the spec's
"don't break the page" philosophy.

## Validator

Located in [`compiler/src/validator.rs`](../compiler/src/validator.rs).

A second pass that collects additional diagnostics:

| Code   | Severity  | Description                                       |
| ------ | --------- | ------------------------------------------------- |
| `W0001` | Warning | Document has no top-level `title` element.        |
| `W0101` | Warning | `<img>` element missing alt text.                  |
| `W0102` | Warning | `<img>` has empty `src`.                            |
| `W0103` | Warning | `<a>` has empty `href`.                            |
| `W0104` | Warning | Table row column count mismatch.                   |
| `W0105` | Warning | Raw `html` block contains `<script>`.               |
| `I0001` | Info    | No description set – first paragraph used.          |

The validator never fails the build. It populates the
[`CompileReport`](../compiler/src/error.rs) which the editor uses
to underline problems inline.

## Emitter

Located in [`compiler/src/emitter.rs`](../compiler/src/emitter.rs).

Walks the AST and emits well-formed HTML5. The emitter:

1. Emits the DOCTYPE, `<html lang>`, `<head>` scaffold (UTF-8, viewport,
   title, description, author, keywords, theme-color, favicon, manifest,
   OpenGraph, Twitter Card).
2. For each top-level node, calls `emit_node` which knows how to render
   every `NodeKind`.
3. Escapes user content with `html_escape`.

Output is pretty-printed with 2-space indentation. The emitter optionally
falls back to `href="#"` when a button or link is missing a target.

## Project file discovery

Located in [`compiler/src/project.rs`](../compiler/src/project.rs).

When `compile_file` is called the compiler looks up `project.ez`,
`config.ez`, `settings.ez`, `site.ez.json`, or `site.ez.toml` in the
source directory. The first one that exists wins.

The native `.ez` format is a simple key/value syntax documented in
[`docs/Syntax.md`](Syntax.md#project-files).

## Build Profile

Cargo release profile is tuned for size & speed:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

The produced binary is fully static on Linux (`x86_64-unknown-linux-musl`)
and around 4 MB.

## Testing

The compiler ships with three test suites:

- `tests/integration.rs` — full pipeline through `compile`.
- `tests/tokenizer.rs` — token-level edge cases.
- `tests/ast.rs` — parsing semantic tests.

Run them with:

```bash
cargo test
```
