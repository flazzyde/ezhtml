# M3 · `.ezplugin` format v0

**Target quarter:** Q3 2026
**Owner:** flazzyde
**Labels:** `roadmap`, `plugins`, `milestone-m3`
**Tracks under:** Q3 2026 · Ecosystem stream

## Goal

Let third parties extend the EZHTML language with custom elements
without forking the compiler.

## Acceptance criteria

- [ ] A short spec (`docs/RFCs/plugin-v0.md` — link follows) defines
      the `.ezplugin` file shape (TOML/JSON/YAML choice deferred).
- [ ] The compiler gains a `--plugin <file>` flag and merges plugin
      declarations with the built-in keywords.
- [ ] One example plugin ships under `examples/plugins/x-easypie/`
      with a working `.ezhtml` source that uses it.
- [ ] Errors during plugin load point users at the SPEC line that
      documents the offending key.
- [ ] A snapshot test proves a "Hello world" plugin produces the
      expected HTML output.

## Non-goals

- A central plugin registry. That's a follow-up milestone.
- Runtime hot-reload of plugins. Cold-restart compiler only for v0.

## Risks

| Risk | Mitigation |
| ---- | ---------- |
| Plugins become a vector for arbitrary code execution. | Sandbox the plugin loader: read-only access to the AST, no `unsafe`, no network. Document threat model in the SPEC. |
| Plugin name collisions. | Reserve a `x-` prefix for third-party custom elements (built-ins can use unsuffixed names). |

## Tracking

- Issues / PRs labelled `milestone-m3`.
- Discord: `#plugins` channel (new channel — created when spec lands).
- Status: `💭 considering`
