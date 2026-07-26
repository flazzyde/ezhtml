# Landing page template

A polished marketing landing page with hero, three feature cards, a
testimonials row, a CTA with a code snippet, and a multi-link footer.

## What's included

- Hero with two CTAs and one subtitle.
- Three feature cards rendered via `row → column → card` with inline `icon`s.
- A 3-card testimonial row using `quote` + `text`.
- A live code snippet in a `<pre><code>` block.
- Sticky navbar with smooth-scroll anchor links (click `Features`, `Testimonials`, …).
- Mobile hamburger toggle.
- **Theme toggle** (light ↔ dark) that remembers the user's choice via `localStorage`.

## Files

```
landing/
├── index.ezhtml
├── project.ez
├── README.md
└── assets/
    ├── css/main.css   # ~180 lines, full theme system
    └── js/main.js     # ~50 lines, three behaviours wired up
```

## Build & preview

```bash
ezhtml build landing/index.ezhtml -o index.html
ezhtml preview landing/index.ezhtml --port 8080
```

## Customise

- Edit `project.ez` — your `title` and `description` are emitted into
  `<title>` and `<meta name="description">` for free.
- Replace `rocket` / `shield` / `sparkles` in `index.ezhtml` with any
  [Lucide](https://lucide.dev/) icon name to swap the visuals.
- Change `--accent` in `assets/css/main.css` to rebrand instantly.
