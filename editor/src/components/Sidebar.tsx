import { useEditorStore } from "../store";

export default function Sidebar() {
  const source = useEditorStore((s) => s.source);
  const setSource = useEditorStore((s) => s.setSource);

  const snippets: { label: string; text: string }[] = [
    {
      label: "Section + Card",
      text: "section\n    title \"Section\"\n    card\n        title \"Card\"\n        text \"Body\"\n",
    },
    {
      label: "Form",
      text: "section\n    input \"name\"\n    email \"email\"\n    button \"Send\" \"#\"\n",
    },
    {
      label: "Table",
      text: "table\n    headers \"Name\", \"Score\"\n    rows\n        row_ \"Alice\", \"42\"\n",
    },
    {
      label: "List",
      text: "list\n    item\n        text \"First\"\n    item\n        text \"Second\"\n",
    },
  ];

  async function openFile() {
    const res = await window.ezhtml?.file.open();
    if (res?.ok && res.path) {
      const file = await window.ezhtml?.file.read(res.path);
      if (file?.ok) {
        setSource(file.data);
        useEditorStore.getState().setFilePath(res.path);
      }
    }
  }

  return (
    <aside className="sidebar">
      <section className="sidebar-section">
        <h3>Files</h3>
        <button onClick={openFile}>Open .ezhtml…</button>
        <button
          onClick={async () => {
            await window.ezhtml?.file.write(
              "new.ezhtml",
              source
            );
          }}
        >
          Save current
        </button>
      </section>

      <section className="sidebar-section">
        <h3>Snippets</h3>
        <ul>
          {snippets.map((s) => (
            <li key={s.label}>
              <button onClick={() => setSource(source + "\n" + s.text)}>
                + {s.label}
              </button>
            </li>
          ))}
        </ul>
      </section>

      <section className="sidebar-section">
        <h3>Help</h3>
        <ul>
          <li>
            <a
              href="#"
              onClick={(e) => {
                e.preventDefault();
                window.ezhtml?.shell.openExternal("https://ezhtml.org/docs");
              }}
            >
              Documentation ↗
            </a>
          </li>
        </ul>
      </section>
    </aside>
  );
}
