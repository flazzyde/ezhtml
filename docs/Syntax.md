# EZHTML – Syntax Reference

A complete reference for every keyword, attribute and pattern supported by
the EZHTML compiler.

Source files end in `.ezhtml`. Comments start with `#`. Indentation uses
2 or 4 spaces (be consistent within a file).

---

## Core ideas

1. **No closing tags.** Indentation is the structure.
2. **No doctype/head boilerplate.** It's generated for you.
3. **Strings are first class.** Quoted with `"..."` or `'...'`.

---

## Head elements

### `title "Hello"`

Renders to `<h1 class="title">Hello</h1>` and is also picked up as the
`<title>` tag in the page `<head>`.

### `subtitle "Subhead"`

Renders to `<h2>`.

### `text "Paragraph"`

Renders to `<p>`. The first `text` element in the document is also used as
the `description` meta tag if you didn't set one in `project.ez`.

---

## Inline elements

### `button "Click me" "https://example.com"`

Renders to `<a class="btn btn-primary" href="…">Click me</a>`.

### `image "./logo.png" "Company logo"`

Renders to `<img src="…" alt="…">`. The compiler warns if you forget the
alt text — accessibility matters.

### `video "./intro.mp4"`

Renders to `<video controls src="…"></video>`.

### `link "https://…" "Anchor text"`

Renders to `<a href="…">Anchor text</a>`. Block form supported too:

```ezhtml
link "#section"
    text "Jump to section"
```

---

## Containers

### `header`, `footer`, `navbar`, `section`

Semantic landmark wrappers. Rendered as the matching HTML5 element with
no class.

`navbar` adds `class="navbar"` automatically.

### `container` / `row` / `column` / `card`

Layout primitives. All render as `<div>` (or `<article class="card">`)
with stable class names so your CSS works everywhere.

```ezhtml
row
    column
        card
            title "Card title"
            text "Card body"
    column
        card
            title "Another card"
            text "With another body."
```

### `list` / `item`

```ezhtml
list
    item
        text "First"
    item
        text "Second"
```

---

## Tables

```ezhtml
table
    headers "Name", "Score"
    rows
        row_ "Alice", "42"
        row_ "Bob",   "37"
```

- `headers` takes one quoted header per entry, separated by any whitespace.
- `row_` (with the trailing underscore, so it stays a valid identifier)
  starts a new row.

---

## Forms

### `input "name" "Placeholder"`

```html
<input type="text" name="name" placeholder="Placeholder">
```

### `email`, `password`

Same shape, different `type` attribute.

### `checkbox "name" "Label text"`

```html
<label class="checkbox">
  <input type="checkbox" name="name"> Label text
</label>
```

### `radio "group" "value" "Label"`

Same as `checkbox` but with `type="radio"` and a `value` attribute.

### `textarea "name" "Placeholder"`

Standard `<textarea>` with a `placeholder` attribute.

---

## Misc

### `code "print("hello")" "python"`

Renders to `<pre><code class="language-python">…</code></pre>`.

### `quote "Body" "Source"`

`<blockquote cite="Source">Body</blockquote>`.

### `divider`

`<hr>`.

### `space`

Empty paragraph for vertical whitespace.

### `icon "rocket"`

`<i class="icon icon-rocket">`. Pair with an icon font such as
[Lucide](https://lucide.dev/).

### `html "<div>…any HTML…</div>"`

Raw HTML pass-through. The compiler warns if it contains `<script>`.

---

## Directives

Lines starting with `!` are directives. They are exposed in the AST as
HTML comments so the source of truth is preserved across builds.

```ezhtml
!lang de
!doctype html5
!theme dark
```

The following are recognised today:

- `!lang <bcp47>` sets `PageSettings.language` (e.g. `!lang de`, `!lang fr-CA`).
- `!theme <name>` is consumed by the editor only (it picks the stylesheet).
- `!doctype html5` is the default; reserved for future variants.

---

## Comments

Anything after `#` on a line is ignored. They never reach the output.

---

## Project files

A project is configured via one of (first match wins):

- `project.ez`
- `config.ez`
- `settings.ez`
- `site.ez.json`
- `site.ez.toml`

Keys (`.ez` key/value form, one per line):

```
title "My Site"
description "A short tagline."
author "Me"
theme_color "#0a84ff"
language "en"
favicon "/icon.svg"
manifest "/manifest.webmanifest"
image "/og.png"
url "https://example.com"
keyword "rust", "compiler", "web"
```

JSON form:

```json
{
  "metadata": {
    "title": "My Site",
    "description": "A short tagline.",
    "keywords": ["rust", "compiler", "web"]
  },
  "page_settings": {
    "language": "en"
  }
}
```

TOML form:

```toml
[metadata]
title = "My Site"
description = "A short tagline."

[page_settings]
language = "en"
```
