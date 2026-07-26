# Blank template

The smallest viable EZHTML starter. Use this when you want to write
everything from scratch.

## Contents

```
blank/
├── index.ezhtml       # page content
├── project.ez         # metadata (title, description, theme, …)
├── README.md          # this file
└── assets/
    └── css/main.css   # minimal CSS reset
```

No JavaScript — the page is intentionally static so you can wire up
only what you actually need.

## Build & preview

From the **template folder**:

```bash
ezhtml build index.ezhtml -o ../index.html
```

From one level up:

```bash
ezhtml build blank/index.ezhtml -o index.html
ezhtml preview blank/index.ezhtml --port 8080
```

Once compiled, open `index.html` in any browser — it pulls the local
`assets/css/main.css` for a sane default look.

## Customise checklist

- [ ] Edit `index.ezhtml` and add a `subtitle`, more `text` blocks
- [ ] Update `project.ez` with your real title & description
- [ ] Optionally swap the accent colour in `assets/css/main.css`
  (`--accent`).
