import { useEditorStore } from "../store";

export default function StatusBar() {
  const filePath = useEditorStore((s) => s.filePath);
  const diagnostics = useEditorStore((s) => s.diagnostics);

  const errors = diagnostics.filter((d) => d.severity === "error").length;
  const warnings = diagnostics.filter((d) => d.severity === "warning").length;

  return (
    <footer className="status-bar">
      <span>{filePath ?? "untitled.ezhtml"}</span>
      <span className="status-spacer" />
      <span className={errors > 0 ? "text-error" : ""}>
        {errors} {errors === 1 ? "error" : "errors"}
      </span>
      <span className={warnings > 0 ? "text-warning" : ""}>
        {warnings} {warnings === 1 ? "warning" : "warnings"}
      </span>
      <span>EZHTML 0.1.0</span>
    </footer>
  );
}
