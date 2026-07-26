# Blog template

A clean, searchable blog layout with:

- Sticky masthead with quick-link navbar
- Posts list rendered as `column → card` with date, reading time and tag chips
- Two-column "About / Subscribe" footer-CTA
- **Search box** that filters posts by title (live, instant)
- **Tag chips** that toggle post visibility
- "Read time" calculated live from the word count of each post body

## Files

```
blog/
├── index.ezhtml
├── project.ez
├── README.md
└── assets/
    ├── css/main.css   # ~160 lines, masthead + post cards + sidebar
    └── js/main.js     # ~60 lines, search + tag filter
```

## Build & preview

```bash
ezhtml build blog/index.ezhtml -o index.html
ezhtml preview blog/index.ezhtml --port 8080
```

## Customise

- Add more `card` blocks to grow the posts list.
- To pin particular tags, give them a class — chips become active when
  the `is-active` class is on them.
- Wire the subscribe form to your provider of choice via the `mailto:`
  fallback below it (or a small backend).
