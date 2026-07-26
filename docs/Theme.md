# Theming Guide

EZHTML ships stable class names so every CSS file you write today keeps
working tomorrow.

## Built-in classes

| Class              | Where it appears                  |
| ------------------ | --------------------------------- |
| `.btn`             | every `button` element            |
| `.btn-primary`     | every `button` element            |
| `.container`       | every `container` element         |
| `.row`             | every `row` element               |
| `.col`             | every `column` element            |
| `.card`            | every `card` element              |
| `.navbar`          | every `navbar` element            |
| `.space`           | every `space` element             |
| `.title`           | every `title` element             |
| `icon`             | every `icon` element              |
| `icon-<name>`      | every `icon "name"` element       |

## Default reference theme

A minimal, dependency-free CSS theme that pairs with the editor:

```css
:root {
  --ez-bg: #ffffff;
  --ez-fg: #0f172a;
  --ez-muted: #64748b;
  --ez-accent: #0a84ff;
  --ez-card: #f8fafc;
  --ez-border: #e2e8f0;
}
@media (prefers-color-scheme: dark) {
  :root {
    --ez-bg: #0b1120;
    --ez-fg: #e2e8f0;
    --ez-muted: #94a3b8;
    --ez-accent: #38bdf8;
    --ez-card: #111827;
    --ez-border: #1f2937;
  }
}
body { background: var(--ez-bg); color: var(--ez-fg); }
.btn.btn-primary { background: var(--ez-accent); color: white; padding: 0.6rem 1rem; border-radius: 0.5rem; text-decoration: none; }
.row { display: flex; gap: 1rem; flex-wrap: wrap; }
.col { flex: 1 1 240px; }
.card { background: var(--ez-card); border: 1px solid var(--ez-border); border-radius: 0.75rem; padding: 1rem; }
.navbar { display: flex; gap: 1rem; padding: 0.75rem 1rem; border-bottom: 1px solid var(--ez-border); }
.title { font-size: 2rem; }
.icon { display: inline-block; width: 1.2em; height: 1.2em; }
table { border-collapse: collapse; width: 100%; }
th, td { padding: 0.5rem 0.75rem; border-bottom: 1px solid var(--ez-border); text-align: left; }
```

Drop this into `theme.css` at the root of your project and reference it
from your `index.ezhtml`:

```ezhtml
html "<link rel=\"stylesheet\" href=\"/theme.css\">"
```

## Creating your own theme

1. Fork the reference theme.
2. Adjust the CSS custom properties (`--ez-*`).
3. Optionally introduce your own class names and use the `html` keyword
   to attach them to specific elements.
4. Ship as a `*.css` next to your site.

## Editor themes

The editor reads the user's `VS Code` theme preference. Project files
can declare a `theme` hint (e.g. `theme midnight`) which the editor
interprets as a base colour scheme. See
[`docs/Compiler.md`](Compiler.md#directives) for the underlying
directive.
