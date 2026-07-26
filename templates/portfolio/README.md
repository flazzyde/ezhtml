# Portfolio template

Personal portfolio with a filterable project grid, an about section and
a contact form.

## Features

- Three project cards rendered as `column → card` blocks with inline
  `thumb` placeholders (gradient swatches generated in CSS — no real
  images required).
- A `tag` row per card (used by the JavaScript filter).
- An "About" sidebar with skills list.
- A working contact form (mailto: fallback — wire it up to a backend if
  you need real submissions).
- **Project filter** — clickable chips (Rust, TypeScript, Canvas, …) hide
  non-matching cards in pure CSS via data attributes + the `is-hidden`
  class toggled from JS.

## Files

```
portfolio/
├── index.ezhtml
├── project.ez
├── README.md
└── assets/
    ├── css/main.css   # ~200 lines, visual grid + thumbs + animations
    └── js/main.js     # ~70 lines, filter behaviour
```

## Build & preview

```bash
ezhtml build portfolio/index.ezhtml -o index.html
ezhtml preview portfolio/index.ezhtml --port 8080
```

## Customise

- Replace each project's `thumb-*` class with your own CSS to swap the
  visual placeholder, or replace the `html "<div class=\"thumb …\">"`
  block with a real `<img>` once you have screenshots.
- Edit the `data-tags="…"` list per project to control the filter chips.
