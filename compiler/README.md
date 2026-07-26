# EZHTML Compiler

A modern, fast Rust compiler for the EZHTML markup language. Transforms
`.ezhtml` files into clean, valid HTML5 – including automatic generation
of `<!DOCTYPE>`, `<html>`, `<head>`, meta tags, OpenGraph, Twitter Cards
and more.

## Architecture

```
.ezhtml source
    │
    ▼
┌─────────────┐
│  Tokenizer  │   raw chars → sequence of tokens (incl. INDENT/DEDENT)
└─────┬───────┘
      ▼
┌─────────────┐
│   Parser    │   tokens → Abstract Syntax Tree
└─────┬───────┘
      ▼
┌─────────────┐
│ Validator   │   AST → multi-error diagnostic report
└─────┬───────┘
      ▼
┌─────────────┐
│  Emitter    │   AST → pretty-printed HTML5
└─────┬───────┘
      ▼
   index.html
```

## CLI

```bash
ezhtml build input.ezhtml -o output.html
ezhtml run input.ezhtml                     # build + serve + open browser
ezhtml preview input.ezhtml                 # watch & live-reload
ezhtml init                                 # scaffold a new project
ezhtml doctor input.ezhtml                  # validation report
ezhtml format input.ezhtml                  # canonical formatting
ezhtml lint input.ezhtml                    # style + best-practice checks
ezhtml version
```

## Library Use

```rust
use ezhtml::{compile, CompileOptions};

let source = r#"title "Hello"
section
    text "World"
"#;

let html = compile(source, &CompileOptions::default())?;
println!("{}", html);
```

## Tests

```bash
cargo test
cargo insta review   # interactive snapshot review
```

## Status

The compiler pipeline is fully implemented. See
[`docs/Syntax.md`](../docs/Syntax.md) for the supported element set.
