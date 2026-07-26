// docs — search highlight + copy-code buttons + scroll-spy.

(function () {
  "use strict";

  // ---- 1) Inject the search toolbar -----------------------------------
  const navbar = document.querySelector(".navbar");
  if (navbar) {
    const bar = document.createElement("div");
    bar.className = "toolbar";
    const search = document.createElement("input");
    search.type = "search";
    search.placeholder = "Search the docs\u2026";
    search.setAttribute("aria-label", "Search documentation");
    bar.appendChild(search);
    navbar.parentNode.insertBefore(bar, navbar.nextSibling);

    // Run highlight whenever the search changes.
    search.addEventListener("input", function () {
      highlight(search.value.trim().toLowerCase());
    });
  }

  function highlight(term) {
    const sections = Array.from(document.querySelectorAll("section"));
    sections.forEach(function (s) { s.classList.remove("is-faded"); });
    if (!term) return;

    sections.forEach(function (s) {
      const body = s.textContent.toLowerCase();
      if (body.indexOf(term) === -1) {
        s.classList.add("is-faded");
      }
    });
  }

  // ---- 2) Copy-code buttons -------------------------------------------
  document.querySelectorAll("pre").forEach(function (pre) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "copy-btn";
    btn.textContent = "Copy";
    btn.setAttribute("aria-label", "Copy code to clipboard");
    btn.addEventListener("click", function () {
      const code = pre.querySelector("code");
      const text = code ? code.textContent : pre.textContent;
      navigator.clipboard.writeText(text).then(function () {
        btn.textContent = "Copied";
        btn.classList.add("is-copied");
        setTimeout(function () {
          btn.textContent = "Copy";
          btn.classList.remove("is-copied");
        }, 1500);
      }).catch(function () {
        btn.textContent = "Failed";
      });
    });
    pre.appendChild(btn);
  });

  // ---- 3) Scroll-spy: highlight nearest section in the navbar --------
  const navLinks = Array.from(document.querySelectorAll(".navbar a"));
  const linkByHash = {};
  navLinks.forEach(function (a) { linkByHash[a.getAttribute("href")] = a; });

  const sections = Array.from(document.querySelectorAll("section"));
  function nearest() {
    const offset = window.scrollY + 120;
    let chosen = null;
    for (const s of sections) {
      if (s.offsetTop <= offset) chosen = s;
    }
    if (!chosen) return;
    navLinks.forEach(function (a) { a.classList.remove("is-active"); });
    const link = linkByHash["#" + chosen.id];
    if (link) link.classList.add("is-active");
  }

  if ("IntersectionObserver" in window) {
    const obs = new IntersectionObserver(nearest, { rootMargin: "-120px 0px -70% 0px" });
    sections.forEach(function (s) { obs.observe(s); });
  } else {
    window.addEventListener("scroll", nearest, { passive: true });
  }
  nearest();
})();
