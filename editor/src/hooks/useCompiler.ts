import { useEffect } from "react";
import { useEditorStore } from "../store";

const DEBOUNCE_MS = 250;

export function useCompiler() {
  const source = useEditorStore((s) => s.source);
  const setHtml = useEditorStore((s) => s.setHtml);
  const setDiagnostics = useEditorStore((s) => s.setDiagnostics);

  useEffect(() => {
    const handle = setTimeout(async () => {
      try {
        const res = await window.ezhtml?.compiler.compile(source, "");
        if (res?.ok) {
          setHtml(res.html || "");
          setDiagnostics(parseDiagnostics(res.stderr));
        } else {
          setDiagnostics([
            { severity: "error", message: res?.stderr || "compile failed" },
          ]);
        }
      } catch (err) {
        setDiagnostics([
          { severity: "error", message: String(err) },
        ]);
      }
    }, DEBOUNCE_MS);

    return () => clearTimeout(handle);
  }, [source, setHtml, setDiagnostics]);
}

function parseDiagnostics(stderr: string | undefined) {
  if (!stderr) return [];
  return stderr
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .map((line) => ({ severity: "info", message: line }));
}
