# Dashboard template

A read-only data dashboard with sidebar nav, KPI cards, a sorted
customers table, an inline SVG bar chart and an activity feed.

## Features

- **Sidebar nav** (injected by JS) with smooth-scrolling to each section.
- **KPI cards** rendered with `row → column → card`.
- **Bar chart** drawn as inline SVG — no external libraries, no images.
  Hook the data in `assets/js/main.js` to your own `Number[]` source.
- **Sortable table** — clicking any column header sorts the rows; the
  numeric columns are detected automatically so "1,200" sorts correctly.
- **Activity feed** as an unordered list with `list → item → text`.

## Files

```
dashboard/
├── index.ezhtml
├── project.ez
├── README.md
└── assets/
    ├── css/main.css   # ~220 lines, sidebar + KPI + table + chart
    └── js/main.js     # ~120 lines, sidebar/sort/chart pipeline
```

## Build & preview

```bash
ezhtml build dashboard/index.ezhtml -o index.html
ezhtml preview dashboard/index.ezhtml --port 8080
```

## Customise

- Tweak the `TRAFFIC` data array in `assets/js/main.js` to feed the chart.
- Edit the table rows directly in `index.ezhtml`.
- Replace the `+4.2%` deltas with your own metrics — they're plain `text`
  blocks.
