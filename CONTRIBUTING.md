# Contributing to EZHTML

Thank you for your interest in contributing to **EZHTML** – the modern
markup language that makes writing web pages as easy as writing Markdown.

## Code of Conduct

This project and everyone participating in it is governed by the
[Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected
to uphold this code.

## How Can I Contribute?

### Reporting Bugs

- Use the **bug report** issue template.
- Include a minimal `.ezhtml` reproduction.
- Include the compiler version (`ezhtml version`).
- Include the operating system.

### Suggesting Features

- Use the **feature request** issue template.
- Describe the use case and the user-facing benefit.
- Add a minimal example if applicable.

### Pull Requests

1. Fork the repository.
2. Create a feature branch: `git checkout -b feat/amazing-thing`.
3. Make your changes.
4. Add or update tests.
5. Run `cargo test --workspace`, `pnpm test`, and `pnpm lint`.
6. Commit using [Conventional Commits](https://www.conventionalcommits.org/).
7. Push your branch and open a Pull Request.

### Commit Messages

```
feat(compiler): support custom doctype declaration
fix(editor): enforce dark-mode color tokens
docs(syntax): document `card` element
chore(release): bump to 0.2.0
```

## Development Setup

```bash
# Compiler
cd compiler
cargo build
cargo test

# Editor
cd editor
pnpm install
pnpm dev

# VS Code extension
cd vscode-extension
pnpm install
pnpm package
```

## Project Structure

```
compiler/          Rust compiler (tokenizer → parser → AST → validator → emitter)
editor/            Electron + React + TypeScript editor
vscode-extension/  VS Code / Cursor language support
website/           Marketing site + browser playground
examples/          30+ .ezhtml examples
templates/         Starter templates
docs/              Reference documentation
```

## Coding Style

- **Rust:** `cargo fmt` and `cargo clippy` clean.
- **TypeScript:** Prettier + ESLint (`pnpm lint`).
- Tests are required for all new compiler logic.
- Comments in English.
- Public APIs must be documented.

## Adding a New EZHTML Element

1. Tokenize / parse the element in `compiler/src/parser.rs`.
2. Add AST node to `compiler/src/ast.rs`.
3. Add validation rules to `compiler/src/validator.rs`.
4. Add HTML emission to `compiler/src/emitter.rs`.
5. Add tests in `compiler/tests/`.
6. Document the element in `docs/Syntax.md`.
7. Add a snippet to `vscode-extension/snippets/`.

## License

By contributing, you agree that your contributions will be licensed under
the [MIT License](LICENSE).
