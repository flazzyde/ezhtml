# CLI Reference

The `ezhtml` binary implements eight sub-commands that map 1-to-1 onto
the eight entry points in the spec.

```bash
ezhtml build <input> [-o <output>] [--report]
ezhtml run <input> [--port 8080]
ezhtml preview <input> [--port 8080]
ezhtml init [dir] [--template <name>]
ezhtml doctor <input>
ezhtml format <input> [--write]
ezhtml lint <input>
ezhtml version
```

## `build`

Compile one `.ezhtml` file to HTML.

```bash
ezhtml build index.ezhtml -o dist/index.html
# Defaults to <input>.html in the same directory.
ezhtml build index.ezhtml
# Print diagnostics alongside the build.
ezhtml build index.ezhtml --report
```

Exit code is non-zero when the parser returns a hard error.

## `run`

Compile and open the result in the default browser.

```bash
ezhtml run landing.ezhtml
ezhtml run landing.ezhtml --port 8080
```

On Windows / macOS / Linux the command uses `rundll32`, `open` and
`xdg-open` respectively to spawn the browser.

## `preview`

Watch the input file and rebuild on save.

```bash
ezhtml preview landing.ezhtml
ezhtml preview landing.ezhtml --port 8080
```

Implementation note: a production preview server should use the
`notify` crate to watch the filesystem and serve from a tiny HTTP
server. The MVP rebuilds whenever the user re-runs the command, which
is enough for most authoring loops.

## `init`

Scaffold a new EZHTML project.

```bash
ezhtml init my-site
ezhtml init --template blog
ezhtml init --template landing
```

Available templates: `blank`, `minimal`, `landing`, `blog`, `portfolio`,
`dashboard`, `docs`, `company`.

`init` creates `index.ezhtml` (from the template) and a starter
`project.ez` with sensible defaults you can edit.

## `doctor`

Print a validation report for a single file. `doctor` is non-destructive
and never overwrites your file.

```bash
ezhtml doctor landing.ezhtml
```

Output is colour-coded by `severity` (info = blue, warning = yellow,
error = red).

## `format`

Canonical-format the source. Indentation is preserved; redundant blank
lines are collapsed; trailing whitespace is trimmed.

```bash
# Print to stdout
ezhtml format landing.ezhtml
# Write back to the file
ezhtml format landing.ezhtml --write
```

## `lint`

Style / best-practice checks. Currently catches hard-tabs and lines
over 120 characters. Surfaces duplicate `description` keywords as
warnings.

```bash
ezhtml lint landing.ezhtml
```

## `version`

```bash
ezhtml version
# ezhtml 0.1.0
# commit  local
# target  x86_64
# rustc   rustc 1.79.0 (and later)
```

## Environment variables

| Variable                | Purpose                                                  |
| ----------------------- | -------------------------------------------------------- |
| `EZHTML_TELEMETRY`      | Opt-in anonymous crash/usage telemetry (default: false). |
| `EZHTML_DEV_PORT`       | Default port for `preview`/`run`.                        |
| `EZHTML_NO_COLOR`       | Disable coloured output even when stdout is a TTY.       |

## Exit codes

| Code | Meaning                                                  |
| ---- | -------------------------------------------------------- |
| 0    | Build succeeded (warnings may still be present).         |
| 1    | Build failed (parse error, I/O error).                   |
| 2    | User error (bad flags, missing arguments, …).            |
| 3    | Internal compiler panic.                                 |
