import { useEffect } from "react";
import { useEditorStore } from "./store";
import Toolbar from "./components/Toolbar";
import Sidebar from "./components/Sidebar";
import Editor from "./components/Editor";
import Preview from "./components/Preview";
import StatusBar from "./components/StatusBar";
import { useCompiler } from "./hooks/useCompiler";

export default function App() {
  const theme = useEditorStore((s) => s.theme);
  const layout = useEditorStore((s) => s.layout);
  useCompiler();

  useEffect(() => {
    document.documentElement.dataset.theme = theme;
  }, [theme]);

  return (
    <div className={`app theme-${theme} layout-${layout}`}>
      <Toolbar />
      <div className="main">
        <Sidebar />
        <div className="workbench">
          <Editor />
          <Preview />
        </div>
      </div>
      <StatusBar />
    </div>
  );
}
