import Editor, { loader } from "@monaco-editor/react";
import { useEditorStore } from "../store";

loader.config({
  paths: { vs: "/monaco-editor/min/vs" },
});

const EZHTML_LANG_ID = "ezhtml";

export default function EditorPane() {
  const source = useEditorStore((s) => s.source);
  const setSource = useEditorStore((s) => s.setSource);
  const theme = useEditorStore((s) => s.theme);

  return (
    <div className="editor-pane">
      <Editor
        height="100%"
        language={EZHTML_LANG_ID}
        value={source}
        onChange={(v) => setSource(v ?? "")}
        theme={theme === "dark" ? "vs-dark" : "vs"}
        options={{
          minimap: { enabled: true },
          fontSize: 14,
          wordWrap: "on",
          tabSize: 2,
          automaticLayout: true,
          smoothScrolling: true,
          cursorBlinking: "smooth",
        }}
        beforeMount={(monaco) => {
          monaco.languages.register({ id: EZHTML_LANG_ID });
          monaco.languages.setMonarchTokensProvider(EZHTML_LANG_ID, {
            keywords: [
              "title", "subtitle", "text", "button", "image", "video",
              "link", "header", "footer", "navbar", "section", "container",
              "row", "column", "card", "list", "item", "table", "input",
              "email", "password", "checkbox", "radio", "textarea", "code",
              "quote", "divider", "space", "icon", "html",
            ],
            tokenizer: {
              root: [
                [/#.*$/, "comment"],
                [/^![a-z][\w-]*/i, "tag"],
                [/"[^"]*"/, "string"],
                [/'[^']*'/, "string"],
                [/[a-z_][\w-]*/, {
                  cases: { "@keywords": "keyword", "@default": "identifier" },
                }],
                [/[{}()\[\]]/, "@brackets"],
                [/\s+/, "white"],
              ],
            },
          });
        }}
      />
    </div>
  );
}
