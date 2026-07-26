# EZHTML Icons

This directory holds icons for:

- The desktop editor (`editor/build/`)
- The VS Code extension (`vscode-extension/`)
- The website (`website/`)

## Files

- `ezhtml.svg` — primary logo (SVG, scalable).
- `favicon.ico` — 16/32/48 multi-resolution favicon (binary).
- `ezhtml-256.png` — 256×256 raster for store listings and PWA icons.
- `ezhtml-mono.svg` — monochrome variant for use on dark surfaces.
- `ezhtml-mark.svg` — mark only (the ⚡ glyph extracted).

## Generating raster icons

If you have ImageMagick:

```bash
magick -background none -density 1200 icons/ezhtml.svg -resize 256x256 icons/ezhtml-256.png
magick -background none -density 1200 icons/ezhtml.svg -resize 32x32 favicon.ico
```

## Logo usage

The EZHTML wordmark and ⚡ glyph are licensed under MIT, same as the
code. Use them freely in articles, talks, and derivative tools — credit
appreciated but not required.
