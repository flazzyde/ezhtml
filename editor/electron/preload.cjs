// Preload – exposes a minimal, typed API to the renderer.
const { contextBridge, ipcRenderer } = require("electron");

contextBridge.exposeInMainWorld("ezhtml", {
  file: {
    read: (p) => ipcRenderer.invoke("file:read", p),
    write: (p, data) => ipcRenderer.invoke("file:write", p, data),
    open: () => ipcRenderer.invoke("file:open"),
    list: (dir) => ipcRenderer.invoke("file:expose", dir),
  },
  compiler: {
    compile: (src, cwd) => ipcRenderer.invoke("ezhtml:compile", src, cwd),
  },
  shell: {
    openExternal: (url) => ipcRenderer.invoke("shell:openExternal", url),
  },
});
