# Templates

Each template is a **mini-project** — a folder with everything you need to
ship a real website: the EZHTML source, a full CSS theme, an optional
JavaScript file for interactivity, a metadata file, and a per-template
README.

```
templates/<name>/
├── index.ezhtml       # the page (this is the file `ezhtml build` runs on)
├── project.ez         # metadata (title, description, theme_color, …)
├── README.md          # what's inside, how to customise
└── assets/
    ├── css/main.css   # full theme, vanilla CSS, dark-mode aware
    └── js/main.js     # interactivity wired up (and `defer`-attached)
```

## Available templates

| Name         | Best for                                          | Has JS? |
| ------------ | ------------------------------------------------- | ------- |
| `blank`      | the smallest viable starter                       | no      |
| `minimal`    | a slightly richer static page                     | no      |
| `landing`    | hero + features + testimonials + CTA + footer    | yes     |
| `portfolio`  | filterable work grid + about + contact form      | yes     |
| `blog`       | masthead + posts + tag chips + search             | yes     |
| `dashboard`  | sidebar nav + sortable table + inline SVG chart   | yes     |
| `docs`       | search + copy-code + scroll-spy                   | yes     |
| `company`    | services + timeline + team + FAQ accordion + form | yes     |

## How `ezhtml init` finds them

When you run

```bash
ezhtml init my-site --template landing
```

the CLI walks the following lookup order and copies the **whole folder**
into `my-site/`:

1. `$EZHTML_TEMPLATES_DIR/landing/`          — env var override (CI, packagers)
2. `<exe-dir>/../templates/landing/`        — FHS-style install (`/usr/bin/ezhtml`)
3. `<exe-dir>/templates/landing/`           — portable install
4. `./templates/landing/`                   — developer mode (running from repo)

If none match, the CLI prints the exact path it was looking for and a
short error message. To fix: either set `EZHTML_TEMPLATES_DIR` or place
the templates directory next to your `ezhtml` binary.

## Build any template

```bash
ezhtml build templates/landing/index.ezhtml -o dist/index.html
```

The produced HTML inlines references to `assets/css/main.css` and
`assets/js/main.js` so a fresh `git clone && ezhtml build && python -m
http.server` always works — no node_modules, no build step.

## Adding your own template

1. Create `templates/<my-name>/` with the structure above.
2. Add `index.ezhtml` using the documented syntax.
3. Add a `project.ez` for accurate OpenGraph metadata.
4. Theme it from `assets/css/main.css` (use the existing templates as
   reference for class names).
5. Wire interactivity in `assets/js/main.js`. The HTML already includes
   it via `html "<script src=\"assets/js/main.js\" defer></script>"`
   at the top.

Open a PR — we welcome new templates!

## Get unstuck on Discord

If a template won't compile, a CSS variable misbehaves, or you just want
to share what you built — drop into the [EZHTML Discord](https://discord.gg/TQs6McKJJs).
The `#showcase` channel is for finished work, `#help` for build failures,
`#roadmap` for what should ship next. Office hours every first Wednesday
of the quarter.
