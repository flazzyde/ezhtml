// portfolio — filter chips + read-time estimator for the about blurb.

(function () {
  "use strict";

  // --- 1) Filter chips ----------------------------------------------
  // Build the chip bar from the unique tags found across all thumbs.
  const cards = Array.from(document.querySelectorAll(".card"));
  const tags = new Set();
  cards.forEach(function (card) {
    card.classList.add("project-card");
    const tagSpans = card.querySelectorAll(".tag");
    tagSpans.forEach(function (t) { tags.add(t.textContent.trim()); });
    card.dataset.tags = Array.from(tagSpans).map(function (t) {
      return t.textContent.trim();
    }).join(",");
  });

  // Decide where to inject the filter bar: inside the "Selected work"
  // section, right after the descriptive text.
  const heading = Array.from(document.querySelectorAll(".title"))
    .find(function (h) { return /selected work/i.test(h.textContent); });
  if (heading && tags.size) {
    const bar = document.createElement("div");
    bar.className = "filter-bar";
    bar.setAttribute("role", "toolbar");
    bar.setAttribute("aria-label", "Filter projects by tag");

    const all = makeChip("All", function () { filter(null); }, true);
    bar.appendChild(all);
    Array.from(tags).sort().forEach(function (label) {
      bar.appendChild(makeChip(label, function () { filter(label); }, false));
    });

    heading.parentNode.insertBefore(bar, heading.nextSibling);
  }

  function makeChip(label, onClick, initiallyActive) {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "filter-chip" + (initiallyActive ? " is-active" : "");
    b.textContent = label;
    b.addEventListener("click", function () {
      document.querySelectorAll(".filter-chip").forEach(function (c) {
        c.classList.remove("is-active");
      });
      b.classList.add("is-active");
      onClick();
    });
    return b;
  }

  function filter(tag) {
    cards.forEach(function (card) {
      if (!tag || (card.dataset.tags || "").split(",").includes(tag)) {
        card.classList.remove("is-hidden");
      } else {
        card.classList.add("is-hidden");
      }
    });
  }

  // --- 2) Read-time estimator --------------------------------------
  // Picks the first <p> inside the About section and appends a small
  // "5 min read" badge. Does nothing if no candidate is found.
  const aboutHeading = Array.from(document.querySelectorAll(".title"))
    .find(function (h) { return /^about$/i.test(h.textContent.trim()); });
  if (aboutHeading) {
    const section = aboutHeading.closest("section");
    if (!section) return;
    const para = section.querySelector("p");
    if (!para) return;
    const words = (para.textContent || "").trim().split(/\s+/).length;
    const minutes = Math.max(1, Math.round(words / 220));
    const badge = document.createElement("span");
    badge.className = "tag";
    badge.style.marginLeft = "0.5rem";
    badge.textContent = minutes + " min read";
    aboutHeading.appendChild(badge);
  }
})();
