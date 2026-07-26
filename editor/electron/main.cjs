// Electron main process (CommonJS so it works without a build step).
const { app, BrowserWindow, ipcMain, dialog, shell } = require("electron");
const path = require("node:path");
const fs = require("node:fs/promises");
const { spawn } = require("node:child_process");

let mainWindow = null;

function resolveCompilerPath() {
  // The compiler binary lives in ../compiler/target/release/ezhtml(.exe).
  // Falls back to PATH lookup if not yet built.
  const exe = process.platform === "win32" ? "ezhtml.exe" : "ezhtml";
  return path.resolve(__dirname, "..", "..", "compiler", "target", "release", exe);
}

async function compileWithRust(source, cwd) {
  const exe = resolveCompilerPath();
  return new Promise((resolve) => {
    const child = spawn(exe, ["build", "-", "-o", "-"], {
      cwd,
      stdio: ["pipe", "pipe", "pipe"],
    });
    let stdout = "";
    let stderr = "";
    child.stdout.on("data", (b) => (stdout += b.toString()));
    child.stderr.on("data", (b) => (stderr += b.toString()));
    child.on("close", (code) => {
      if (code === 0) {
        resolve({ ok: true, html: stdout, stderr });
      } else {
        resolve({ ok: false, html: stdout, stderr, code });
      }
    });
    child.stdin.end(source);
  });
}

async function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1280,
    height: 820,
    backgroundColor: "#0b1120",
    webPreferences: {
      preload: path.join(__dirname, "preload.cjs"),
      sandbox: false,
      contextIsolation: true,
    },
  });
  await mainWindow.loadURL("http://localhost:5173");
  mainWindow.on("closed", () => {
    mainWindow = null;
  });
}

app.whenReady().then(async () => {
  ipcMain.handle("file:read", async (_e, file) => {
    try {
      const data = await fs.readFile(file, "utf8");
      return { ok: true, data };
    } catch (err) {
      return { ok: false, error: String(err) };
    }
  });

  ipcMain.handle("file:write", async (_e, file, data) => {
    try {
      await fs.writeFile(file, data, "utf8");
      return { ok: true };
    } catch (err) {
      return { ok: false, error: String(err) };
    }
  });

  ipcMain.handle("file:open", async () => {
    const res = await dialog.showOpenDialog(mainWindow, {
      properties: ["openFile"],
      filters: [{ name: "EZHTML", extensions: ["ezhtml"] }],
    });
    return res.canceled ? { ok: false } : { ok: true, path: res.filePaths[0] };
  });

  ipcMain.handle("file:expose", async (_e, dir) => {
    try {
      const entries = await fs.readdir(dir, { withFileTypes: true });
      return {
        ok: true,
        entries: entries.map((e) => ({
          name: e.name,
          path: path.join(dir, e.name),
          isDir: e.isDirectory(),
        })),
      };
    } catch (err) {
      return { ok: false, error: String(err) };
    }
  });

  ipcMain.handle("ezhtml:compile", async (_e, source, cwd) => {
    return compileWithRust(source, cwd || process.cwd());
  });

  ipcMain.handle("shell:openExternal", async (_e, url) => {
    await shell.openExternal(url);
    return { ok: true };
  });

  await createWindow();
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});
