// blog — live search, tag filter, read-time estimator.

(function () {
  "use strict";

  const POST_SELECTOR = ".row > .column > .card";
  const posts = Array.from(document.querySelectorAll(POST_SELECTOR));
  if (!posts.length) return;

  posts.forEach(function (card) {
    // The first <h1.title> in the card is the post title; everything
    // inside is searchable.
    card.classList.add("post-card");

    // Compute tags from .tag spans.
    const tagSpans = card.querySelectorAll(".tag");
    card.dataset.tags = Array.from(tagSpans).map(function (t) {
      return t.textContent.trim().toLowerCase();
    }).join(",");

    // Live-compute reading time from the .post-meta line if missing.
    const meta = card.querySelector(".post-meta");
    if (meta && !/\d+\s*min/i.test(meta.textContent)) {
      const wordy = card.querySelectorAll("p");
      let words = 0;
      wordy.forEach(function (p) { words += (p.textContent || "").split(/\s+/).length; });
      const minutes = Math.max(1, Math.round(words / 220));
      meta.textContent = meta.textContent + " " + minutes + " min ·";
    }
  });

  // ---- Toolbar (search + filter chips) ----
  const toolbar = document.createElement("div");
  toolbar.className = "toolbar";

  const search = document.createElement("input");
  search.type = "search";
  search.placeholder = "Search posts\u2026";
  search.setAttribute("aria-label", "Search posts");

  const chipBar = document.createElement("div");
  chipBar.className = "filter-bar";
  chipBar.setAttribute("role", "toolbar");
  chipBar.setAttribute("aria-label", "Filter by tag");

  toolbar.appendChild(search);
  toolbar.appendChild(chipBar);

  // Insert the toolbar just after the masthead <header>.
  const masthead = document.querySelector("header");
  if (masthead && masthead.parentNode) {
    masthead.parentNode.insertBefore(toolbar, masthead.nextSibling);
  }

  // Build chip set from discovered tags.
  const allChip = makeChip("All", null, true);
  chipBar.appendChild(allChip);
  const tagSet = new Set();
  posts.forEach(function (p) {
    (p.dataset.tags || "").split(",").filter(Boolean).forEach(function (t) {
      tagSet.add(t);
    });
  });
  Array.from(tagSet).sort().forEach(function (label) {
    chipBar.appendChild(makeChip(label, label, false));
  });

  search.addEventListener("input", applyFilters);
  chipBar.addEventListener("click", function (e) {
    if (e.target && e.target.classList && e.target.classList.contains("filter-chip")) {
      chipBar.querySelectorAll(".filter-chip").forEach(function (c) { c.classList.remove("is-active"); });
      e.target.classList.add("is-active");
      applyFilters();
    }
  });

  function makeChip(label, value, initiallyActive) {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "filter-chip" + (initiallyActive ? " is-active" : "");
    b.textContent = label;
    b.dataset.value = value == null ? "" : value;
    return b;
  }

  function activeTag() {
    const c = chipBar.querySelector(".filter-chip.is-active");
    return c ? c.dataset.value : "";
  }

  function applyFilters() {
    const term = search.value.trim().toLowerCase();
    const tag = activeTag();
    posts.forEach(function (card) {
      const inTag = !tag || (card.dataset.tags || "").split(",").includes(tag);
      const haystack = card.textContent.toLowerCase();
      const inSearch = !term || haystack.indexOf(term) !== -1;
      card.classList.toggle("is-hidden", !(inTag && inSearch));
    });
  }
})();
