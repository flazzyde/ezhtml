// dashboard — sidebar nav, sortable table, inline SVG bar chart.

(function () {
  "use strict";

  // ---- Helpers ---------------------------------------------------------
  function parseNum(s) {
    if (!s) return NaN;
    const cleaned = String(s).replace(/[^0-9.\-]/g, "");
    const n = parseFloat(cleaned);
    return Number.isFinite(n) ? n : NaN;
  }

  // ---- 1) Sidebar nav -------------------------------------------------
  const sections = Array.from(document.querySelectorAll("section"));
  if (sections.length) {
    const nav = document.createElement("aside");
    nav.className = "sidebar-nav";
    nav.innerHTML = "<h3>Sections</h3><ul></ul>";
    const ul = nav.querySelector("ul");
    sections.forEach(function (s) {
      const heading = s.querySelector(".title") || s.querySelector("h1, h2");
      if (!heading) return;
      // Give the section an id so the anchor works.
      const slug = heading.textContent.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
      if (slug) s.id = slug;
      const li = document.createElement("li");
      const a = document.createElement("a");
      a.href = "#" + slug;
      a.textContent = heading.textContent.trim();
      li.appendChild(a);
      ul.appendChild(li);
    });
    document.body.insertBefore(nav, document.body.firstChild);

    // Scroll-spy: highlight active nav link as you scroll.
    nav.addEventListener("click", function (e) {
      if (e.target && e.target.tagName === "A") {
        ul.querySelectorAll("a").forEach(function (a) { a.classList.remove("is-active"); });
        e.target.classList.add("is-active");
      }
    });
    // Mark the first as active initially.
    if (ul.firstChild) ul.firstChild.querySelector("a").classList.add("is-active");
  }

  // ---- 2) Sortable table ----------------------------------------------
  const tables = Array.from(document.querySelectorAll("table"));
  tables.forEach(function (table) {
    const thead = table.querySelector("thead");
    const tbody = table.querySelector("tbody");
    if (!thead || !tbody) return;

    const headers = Array.from(thead.querySelectorAll("th"));
    const original = Array.from(tbody.querySelectorAll("tr"));

    headers.forEach(function (th, idx) {
      const arrow = document.createElement("span");
      arrow.className = "arrow";
      arrow.textContent = "\u2195";   // ↕
      th.appendChild(arrow);

      th.addEventListener("click", function () {
        const dir = th.classList.contains("is-sorted-asc") ? "desc" : "asc";
        headers.forEach(function (h) { h.classList.remove("is-sorted-asc", "is-sorted-desc"); });
        th.classList.add("is-sorted-" + dir);
        arrow.textContent = dir === "asc" ? "\u2191" : "\u2193";

        // Detect numeric column — if at least half the rows parse as numbers, treat as numeric.
        const sample = original.slice(0, Math.min(original.length, 5));
        const numericCount = sample.filter(function (r) {
          const c = r.children[idx];
          return c && !Number.isNaN(parseNum(c.textContent));
        }).length;
        const isNumeric = numericCount * 2 >= sample.length;

        const sorted = original.slice().sort(function (a, b) {
          const av = a.children[idx] ? a.children[idx].textContent : "";
          const bv = b.children[idx] ? b.children[idx].textContent : "";
          if (isNumeric) {
            const an = parseNum(av), bn = parseNum(bv);
            return dir === "asc" ? an - bn : bn - an;
          }
          return dir === "asc"
            ? av.localeCompare(bv, undefined, { sensitivity: "base" })
            : bv.localeCompare(av, undefined, { sensitivity: "base" });
        });
        sorted.forEach(function (row) { tbody.appendChild(row); });
      });
    });
  });

  // ---- 3) Bar chart ----------------------------------------------------
  // Edit this single source of truth to feed the chart.
  const TRAFFIC = [
    { day: "Mon", value: 1240 },
    { day: "Tue", value: 1580 },
    { day: "Wed", value: 1320 },
    { day: "Thu", value: 2210 },
    { day: "Fri", value: 2890 },
    { day: "Sat", value: 1760 },
    { day: "Sun", value: 2030 }
  ];

  const chart = document.getElementById("traffic-chart");
  if (chart) {
    const W = chart.clientWidth || 600;
    const H = chart.clientHeight || 200;
    const padL = 36, padR = 12, padT = 8, padB = 28;
    const max = Math.max.apply(null, TRAFFIC.map(function (d) { return d.value; })) || 1;
    const barW = (W - padL - padR) / TRAFFIC.length;

    const svgNS = "http://www.w3.org/2000/svg";
    const svg = document.createElementNS(svgNS, "svg");
    svg.setAttribute("viewBox", "0 0 " + W + " " + H);
    svg.setAttribute("preserveAspectRatio", "none");

    // Y-axis gridlines + labels at 0, 25, 50, 75, 100%.
    [0, 0.25, 0.5, 0.75, 1].forEach(function (frac) {
      const y = padT + (H - padT - padB) * (1 - frac);
      const grid = document.createElementNS(svgNS, "line");
      grid.setAttribute("x1", padL); grid.setAttribute("x2", W - padR);
      grid.setAttribute("y1", y); grid.setAttribute("y2", y);
      grid.setAttribute("stroke", "currentColor");
      grid.setAttribute("stroke-opacity", "0.08");
      svg.appendChild(grid);
      const label = document.createElementNS(svgNS, "text");
      label.setAttribute("x", padL - 6);
      label.setAttribute("y", y + 3);
      label.setAttribute("text-anchor", "end");
      label.textContent = Math.round(max * frac);
      svg.appendChild(label);
    });

    TRAFFIC.forEach(function (d, i) {
      const ratio = d.value / max;
      const h = (H - padT - padB) * ratio;
      const x = padL + i * barW + barW * 0.15;
      const y = H - padB - h;
      const w = barW * 0.7;

      const rect = document.createElementNS(svgNS, "rect");
      rect.setAttribute("x", x); rect.setAttribute("y", y);
      rect.setAttribute("width", w); rect.setAttribute("height", h);
      rect.setAttribute("rx", "3");
      svg.appendChild(rect);

      const lbl = document.createElementNS(svgNS, "text");
      lbl.setAttribute("x", x + w / 2);
      lbl.setAttribute("y", H - padB + 14);
      lbl.setAttribute("text-anchor", "middle");
      lbl.textContent = d.day;
      svg.appendChild(lbl);
    });

    chart.appendChild(svg);
  }

  // ---- 4) Wire the KPI deltas to status colour -------------------------
  document.querySelectorAll(".card").forEach(function (card) {
    const ps = card.querySelectorAll("p");
    if (ps.length < 2) return;
    const delta = ps[ps.length - 1].textContent.trim();
    if (/^\+/.test(delta) || /up|more|grew/i.test(delta)) ps[ps.length - 1].classList.add("delta-pos");
    if (/^[-]/.test(delta) || /down|fewer|fell|failed/i.test(delta)) ps[ps.length - 1].classList.add("delta-neg");
  });
})();
