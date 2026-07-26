import { useEditorStore } from "../store";

export default function Toolbar() {
  const theme = useEditorStore((s) => s.theme);
  const layout = useEditorStore((s) => s.layout);
  const toggleTheme = useEditorStore((s) => s.toggleTheme);
  const setLayout = useEditorStore((s) => s.setLayout);

  return (
    <header className="toolbar">
      <div className="brand">
        <span className="logo" aria-hidden="true">⚡</span>
        <span className="brand-name">EZHTML</span>
      </div>

      <nav className="toolbar-actions">
        <button onClick={() => window.ezhtml?.file.open()}>Open</button>
        <button onClick={() => window.ezhtml?.file.write("untitled.ezhtml", useEditorStore.getState().source)}>Save</button>
        <button onClick={() => window.ezhtml?.file.write("untitled.html", useEditorStore.getState().html)}>Export HTML</button>
      </nav>

      <nav className="toolbar-toggles">
        <div role="group" aria-label="Layout">
          <button
            className={layout === "split-v" ? "active" : ""}
            onClick={() => setLayout("split-v")}
          >
            ⊟
          </button>
          <button
            className={layout === "split-h" ? "active" : ""}
            onClick={() => setLayout("split-h")}
          >
            ⊞
          </button>
        </div>
        <button onClick={toggleTheme} aria-label="Toggle theme">
          {theme === "dark" ? "☀" : "☾"}
        </button>
      </nav>
    </header>
  );
}
