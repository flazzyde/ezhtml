# M2 · `ezhtml lint` as its own command

**Target quarter:** May 2026
**Owner:** flazzyde
**Labels:** `roadmap`, `cli`, `milestone-m2`
**Tracks under:** Q2 2026 · Compiler stream

## Goal

Split "report all the issues with this file" (kept by `ezhtml doctor`)
from "are these issues violations of our style guide" (new home of
`ezhtml lint`).

## Acceptance criteria

- [ ] `ezhtml lint <input>` accepts the same positional argument and
      `--report` flag as `doctor` and `format`.
- [ ] Exit codes are standardised:
    - `0` — no findings
    - `1` — warnings only
    - `2` — errors present
- [ ] At minimum the following rules are implemented:
    - `max-line-length` (default 120)
    - `no-hard-tabs`
    - `no-trailing-whitespace`
    - `description-uniqueness` (no duplicate `description` keys in
      `project.ez`)
- [ ] `ezhtml doctor` no longer runs style rules — it is purely a
      validator (`W0001`-`W0105` codes only).
- [ ] A short table lives in `docs/CLI.md` mapping rule IDs to
      severity codes so editor tooling can colour them consistently.

## Non-goals

- A pluggable rule system. That comes with the plugin milestone (M3).
- Auto-fix. `format` stays the only auto-fixing command.

## Risks

| Risk | Mitigation |
| ---- | ---------- |
| Users on existing CI scripts (`ezhtml doctor` gates a deploy) silently miss the style regression. | Add a `--include-style` flag to `doctor` that runs both for one release; document the split in CHANGELOG. |

## Tracking

- Issues / PRs labelled `milestone-m2`.
- Discord: `#compiler` channel.
- Status: `🔜 scheduled`
