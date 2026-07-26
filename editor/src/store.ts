import { create } from "zustand";

export interface EditorState {
  /** Current document path (or null for "untitled"). */
  filePath: string | null;
  /** The buffer text. */
  source: string;
  /** Latest compiled HTML. */
  html: string;
  /** Last error / warning report from ezhtml doctor. */
  diagnostics: { severity: string; message: string }[];
  /** UI theme. */
  theme: "dark" | "light";
  /** Layout: side-by-side or stacked. */
  layout: "split-v" | "split-h";

  setSource: (source: string) => void;
  setHtml: (html: string) => void;
  setDiagnostics: (d: EditorState["diagnostics"]) => void;
  setFilePath: (p: string | null) => void;
  toggleTheme: () => void;
  setLayout: (l: EditorState["layout"]) => void;
}

export const useEditorStore = create<EditorState>((set) => ({
  filePath: null,
  source: `title "Hello, EZHTML"\ntext "Start writing on the left.\nSee the live preview on the right."\n`,
  html: "",
  diagnostics: [],
  theme: "dark",
  layout: "split-h",

  setSource: (source) => set({ source }),
  setHtml: (html) => set({ html }),
  setDiagnostics: (diagnostics) => set({ diagnostics }),
  setFilePath: (filePath) => set({ filePath }),
  toggleTheme: () =>
    set((s) => ({ theme: s.theme === "dark" ? "light" : "dark" })),
  setLayout: (layout) => set({ layout }),
}));
