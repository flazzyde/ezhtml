import { useEditorStore } from "../store";

export default function Preview() {
  const html = useEditorStore((s) => s.html);
  const diagnostics = useEditorStore((s) => s.diagnostics);

  return (
    <section className="preview">
      <header className="preview-header">
        <span>Preview</span>
        <span className="preview-meta">
          {diagnostics.length === 0
            ? "✓ no issues"
            : `${diagnostics.length} issue(s)`}
        </span>
      </header>
      <iframe
        title="Live preview"
        srcDoc={html || defaultPreview()}
        sandbox="allow-same-origin allow-scripts"
      />
    </section>
  );
}

function defaultPreview() {
  return `<!doctype html><html><body style="font-family: sans-serif; padding:2rem; color:#94a3b8;">
    <p>Start typing &lt;title&gt;, &lt;text&gt;, &lt;section&gt; ...</p>
  </body></html>`;
}
