// company — FAQ accordion (one item open at a time) +
// contact form validation + smooth-scroll for navbar anchors.

(function () {
  "use strict";

  // ---- 1) FAQ accordion -----------------------------------------------
  // Native <details> tags already give us open/close behaviour. We add
  // a small extra: opening a new one closes the others.
  const details = Array.from(document.querySelectorAll(".faq details"));
  details.forEach(function (d) {
    d.addEventListener("toggle", function () {
      if (!d.open) return;
      details.forEach(function (other) {
        if (other !== d) other.open = false;
      });
    });
  });

  // ---- 2) Contact form -----------------------------------------------
  // Submits via mailto: with a fresh "brief" string. Replace the handler
  // with fetch(...) to talk to a real backend without touching the markup.
  const contactSection = Array.from(document.querySelectorAll(".title"))
    .find(function (h) { return /project/i.test(h.textContent); });
  const form = contactSection ? contactSection.parentNode.querySelector("button[href^='mailto']") : null;

  if (form) {
    form.addEventListener("click", function (ev) {
      ev.preventDefault();
      const emailEl = contactSection.parentNode.querySelector("input[type='email']");
      const companyEl = contactSection.parentNode.querySelector("input[name='company']");
      const brief = contactSection.parentNode.querySelector("textarea");
      const errs = [];

      [emailEl, companyEl, brief].forEach(function (el) {
        if (el && el.classList) el.classList.remove("is-invalid");
      });

      if (!emailEl || !emailEl.value.trim() || !/^[^\s@]+@[^\s@]+$/.test(emailEl.value)) {
        if (emailEl) emailEl.classList.add("is-invalid");
        errs.push("Please enter a valid email.");
      }
      if (!brief || brief.value.trim().length < 10) {
        if (brief) brief.classList.add("is-invalid");
        errs.push("Tell us a bit about the project (10+ chars).");
      }
      if (errs.length) {
        showFieldError(contactSection.parentNode, errs.join(" "));
        return;
      }

      clearFieldError(contactSection.parentNode);
      const subject = encodeURIComponent("Project brief from " + (companyEl && companyEl.value ? companyEl.value : emailEl.value));
      const body = encodeURIComponent(
        "Email: " + emailEl.value + "\n" +
        "Company: " + (companyEl && companyEl.value ? companyEl.value : "-") + "\n\n" +
        "Brief:\n" + brief.value
      );
      const href = form.getAttribute("href");
      const target = href.split("?")[0];
      window.location.href = target + "?subject=" + subject + "&body=" + body;
    });
  }

  function showFieldError(scope, msg) {
    let el = scope.querySelector(".field-error");
    if (!el) {
      el = document.createElement("p");
      el.className = "field-error";
      scope.appendChild(el);
    }
    el.textContent = msg;
  }
  function clearFieldError(scope) {
    const el = scope.querySelector(".field-error");
    if (el) el.remove();
  }

  // ---- 3) Smooth scroll for the navbar --------------------------------
  document.querySelectorAll('.navbar a[href^="#"]').forEach(function (a) {
    a.addEventListener("click", function (ev) {
      const id = a.getAttribute("href");
      if (!id || id === "#") return;
      const target = document.querySelector(id);
      if (!target) return;
      ev.preventDefault();
      target.scrollIntoView({ behavior: "smooth", block: "start" });
      history.replaceState(null, "", id);
    });
  });
})();
