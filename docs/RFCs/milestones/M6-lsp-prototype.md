# M6 · LSP prototype

**Target quarter:** Q4 2026
**Owner:** flazzyde
**Labels:** `roadmap`, `lsp`, `editor`, `milestone-m6`
**Tracks under:** Q4 2026 · Editor + Ecosystem stream

## Goal

Bring IntelliSense-grade EZHTML support to every editor that speaks
the Language Server Protocol — not just VS Code.

## Acceptance criteria

- [ ] A new `ezhtml-lsp` crate + binary lives next to the
      `ezhtml` compiler crate, sharing the lexer/parser/vm.
- [ ] The server implements the LSP protocol via the official
      `lsp-server` crate.
- [ ] At minimum these capabilities are reported and used:
      - Hover over a keyword → keyword reference
      - Go-to-definition for `link "#"` and `link "./"` targets
      - Completion of the 28 built-in keywords
      - Diagnostics that mirror the `CompileReport` from the CLI
- [ ] A second VS Code extension (the existing one plus LSP wiring)
      is published as `vscode-extension-lsp` with a minimum-CI smoke
      test.
- [ ] Cursor and Windsurf can use the same LSP via the standard
      extension.

## Non-goals

- Rename refactoring across files.
- Multi-file AST analysis (e.g. "show all uses of `x-easypie`").
  That's an M7 candidate.

## Risks

| Risk | Mitigation |
| ---- | ---------- |
| LSP coupling pulls parser changes into a major-version-bump cycle. | Keep the LSP server a thin shell that reuses `ezhtml`'s public API. |
| Test surface explodes — every editor is a moving target. | Ship tests against the `lsp-server` crate's in-memory transport, not VS Code itself. |

## Tracking

- Issues / PRs labelled `milestone-m6`.
- Discord: `#editor` channel.
- Status: `🔜 scheduled`
