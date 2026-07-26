# Company / agency template

A polished agency / consultancy website with everything an actual
business needs:

- Hero with two CTAs and a subtitle
- Three services with **icon glyphs**
- A 5-step process timeline (5 cards in a row)
- A team grid with **CSS avatar placeholders** (letters, no images)
- A native HTML `<details>` FAQ accordion (works without JS too!)
- A contact form (mailto fallback)

## Files

```
company/
├── index.ezhtml
├── project.ez
├── README.md
└── assets/
    ├── css/main.css   # ~200 lines, services + timeline + team + FAQ
    └── js/main.js     # ~70 lines, contact-form validation +
                       # one-open FAQ accordion + smooth scroll
```

## Build & preview

```bash
ezhtml build company/index.ezhtml -o index.html
ezhtml preview company/index.ezhtml --port 8080
```

## Customise

- Edit the FAQ entries inline — `<details>`/`<summary>` render
  natively and work even without JavaScript.
- Replace each `data-init` letter with an actual headshot by changing
  `.avatar` to `<div class="avatar" style="background-image: url(/headshots/...)"></div>`.
- The contact form uses `mailto:` — wire it up to a backend, Netlify
  Forms, or Formspree when you're ready.
