# Documentation site template

Three-column docs layout with:
- **Sticky top navbar** linking to every chapter.
- **Search box** that highlights matching sections live.
- **Copy-to-clipboard button** on every code block.
- **Scroll-spy** that highlights the nearest section heading in the nav.
- A clean prose column with code blocks (typography tuned for long
  reading sessions).

## Files

```
docs/
├── index.ezhtml
├── project.ez
├── README.md
└── assets/
    ├── css/main.css   # ~180 lines, prose typography + code styling
    └── js/main.js     # ~70 lines, search + scroll-spy + copy-code
```

## Build & preview

```bash
ezhtml build docs/index.ezhtml -o index.html
ezhtml preview docs/index.ezhtml --port 8080
```

## Customise

- Add new chapters by appending another `section` block — the sidebar
  updates itself by walking the rendered DOM.
- Replace the code blocks (`code` keyword) with whatever tutorials you
  want — every block auto-receives a copy button.
