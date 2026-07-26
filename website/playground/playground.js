// Minimal client-side EZHTML "compiler" — enough to drive the playground
// while the WASM build is in progress. For full capabilities use the
// `ezhtml` CLI or the `playground-wasm` package coming in 0.2.

const STARTUP_SNIPPET = `title "EZHTML Playground"
subtitle "Type on the left, see live HTML on the right."

button "Read the docs" "https://ezhtml.flazzy.de/docs"

section
    title "Features"
    row
        card
            title "Fast"
            text "A single Rust binary."
        card
            title "Safe"
            text "Built-in validator."`;

const KEYWORDS = ["title","subtitle","text","button","image","video","link","header","footer","navbar","section","container","row","column","card","list","item","table","input","email","password","checkbox","radio","textarea","code","quote","divider","space","icon","html","headers","rows","row_"];

function escapeHtml(s) {
  return s.replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;");
}

function compile(source) {
  const lines = source.split("\n");
  const out = [
    "<!doctype html>",
    '<html lang="en">',
    "<head>",
    '  <meta charset="utf-8">',
    '  <meta name="viewport" content="width=device-width,initial-scale=1">',
    "  <title>Playground</title>",
    "</head>",
    "<body>"
  ];
  for (const raw of lines) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    const indent = raw.length - raw.trimStart().length;
    const parts = line.split(/\s+/);
    const head = parts[0];
    const rest = parts.slice(1);
    if (!KEYWORDS.includes(head)) continue;
    switch (head) {
      case "title": out.push(`  <h1 class="title">${escapeHtml(rest.join(" ").replace(/"/g,""))}</h1>`); break;
      case "subtitle": out.push(`  <h2>${escapeHtml(rest.join(" ").replace(/"/g,""))}</h2>`); break;
      case "text": out.push(`  <p>${escapeHtml(rest.join(" ").replace(/"/g,""))}</p>`); break;
      case "button": {
        const label = (rest[0] || "").replace(/"/g,"");
        const href = (rest[1] || "#").replace(/"/g,"");
        out.push(`  <p><a class="btn btn-primary" href="${escapeHtml(href)}">${escapeHtml(label)}</a></p>`);
        break;
      }
      default: out.push(`  <!-- ${line} -->`);
    }
  }
  out.push("</body>", "</html>", "");
  return out.join("\n");
}

const editor = document.getElementById("source");
const preview = document.getElementById("preview");
const status = document.getElementById("status");

editor.value = localStorage.getItem("ezhtml.pg.source") || STARTUP_SNIPPET;

function render() {
  const src = editor.value;
  localStorage.setItem("ezhtml.pg.source", src);
  const html = compile(src);
  preview.srcdoc = html;
  status.textContent = `${src.split("\n").length} lines · ${(html.length / 1024).toFixed(1)} KB preview`;
}

editor.addEventListener("input", () => {
  clearTimeout(window.__pg);
  window.__pg = setTimeout(render, 200);
});

document.getElementById("export").onclick = () => {
  const html = compile(editor.value);
  const blob = new Blob([html], { type: "text/html" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url; a.download = "playground.html"; a.click();
  URL.revokeObjectURL(url);
};

document.getElementById("reset").onclick = () => {
  editor.value = STARTUP_SNIPPET;
  render();
};

render();
