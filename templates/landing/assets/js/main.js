// Landing page — three behaviours, no library.
//
// 1) On <720px viewports show a hamburger toggle that expands the navbar.
// 2) Theme toggle that remembers the user's choice (localStorage key:
//    "ezhtml:theme"). Initial value follows prefers-color-scheme.
// 3) Smooth-scroll for in-page anchors (CSS scroll-behavior handles most
//    of it; JS just keeps focus management clean).

(function () {
  "use strict";

  const ROOT = document.documentElement;
  const LS_KEY = "ezhtml:theme";

  // --- 1) Hamburger ---------------------------------------------------
  const navbar = document.querySelector(".navbar");
  if (navbar && !document.querySelector(".nav-toggle")) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "nav-toggle";
    btn.setAttribute("aria-label", "Toggle navigation");
    btn.textContent = "\u2630";   /* ☰ */
    btn.addEventListener("click", function () {
      navbar.classList.toggle("nav-open");
    });
    navbar.appendChild(btn);
  }

  // --- 2) Theme toggle -----------------------------------------------
  const stored = localStorage.getItem(LS_KEY);
  if (stored === "light" || stored === "dark") {
    ROOT.classList.add("theme-" + stored);
  }

  const toggle = document.createElement("button");
  toggle.type = "button";
  toggle.className = "theme-toggle";
  toggle.setAttribute("aria-label", "Toggle theme");
  toggle.textContent = currentGlyph();

  toggle.addEventListener("click", function () {
    const goingDark = !ROOT.classList.contains("theme-dark");
    ROOT.classList.remove("theme-light", "theme-dark");
    ROOT.classList.add(goingDark ? "theme-dark" : "theme-light");
    localStorage.setItem(LS_KEY, goingDark ? "dark" : "light");
    toggle.textContent = goingDark ? "\u2600" : "\u263d"; // ☀ / ☾
  });

  navbar && navbar.appendChild(toggle);

  function currentGlyph() {
    if (ROOT.classList.contains("theme-dark")) return "\u2600"; // ☀
    return "\u263d";                                          // ☾
  }

  // --- 3) Smooth scroll ----------------------------------------------
  document.querySelectorAll('a[href^="#"]').forEach(function (a) {
    a.addEventListener("click", function (ev) {
      const id = a.getAttribute("href");
      if (!id || id === "#") return;
      const target = document.querySelector(id);
      if (!target) return;
      ev.preventDefault();
      target.scrollIntoView({ behavior: "smooth", block: "start" });
      history.replaceState(null, "", id);
      navbar && navbar.classList.remove("nav-open");
    });
  });
})();
