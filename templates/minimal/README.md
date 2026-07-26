# Minimal template

Slightly richer than `blank` — adds a `subtitle`, a `divider` and a real
`footer`. Still **no JavaScript**, so it's served exactly the same in
every browser without `noscript` fallbacks.

## Contents

```
minimal/
├── index.ezhtml
├── project.ez
├── README.md
└── assets/
    └── css/main.css   # base typography + button styling
```

## Build

```bash
ezhtml build minimal/index.ezhtml -o index.html
ezhtml preview minimal/index.ezhtml
```

## Customise checklist

- [ ] Replace the title / subtitle / button copy in `index.ezhtml`
- [ ] Swap `--accent` in `assets/css/main.css` to match your brand
- [ ] Edit `project.ez` for accurate OpenGraph metadata
