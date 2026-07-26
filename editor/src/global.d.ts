// Type declarations for the Electron preload bridge.
// These describe the surface that `electron/preload.cjs` exposes via
// `contextBridge.exposeInMainWorld("ezhtml", …)`. Keep this file in sync
// with `electron/preload.cjs`.

export interface CompileResult {
  /** Whether the compile call completed without an IPC exception. */
  ok: boolean;
  /** Compiled HTML output (empty when ok is false). */
  html?: string;
  /** Process stderr captured by the Rust compiler binary. */
  stderr?: string;
}

export interface FileOpenResult {
  ok: boolean;
  path?: string;
  cancelled?: boolean;
}

export interface FileReadResult {
  ok: boolean;
  data?: string;
  error?: string;
}

export interface EzhtmlBridge {
  file: {
    open: () => Promise<FileOpenResult>;
    read: (path: string) => Promise<FileReadResult>;
    write: (path: string, data: string) => Promise<{ ok: boolean; error?: string }>;
    save: (path: string, data: string) => Promise<{ ok: boolean; error?: string }>;
  };
  shell: {
    openExternal: (url: string) => Promise<{ ok: boolean; error?: string }>;
  };
  compiler: {
    /**
     * Path of the bundled `ezhtml` binary. Returned by the preload so the
     * renderer never hard-codes it.
     */
    binaryPath: () => Promise<string>;
    /** Compile the given source. Returns html + stderr diagnostics. */
    compile: (source: string, projectContent: string) => Promise<CompileResult>;
    /** Validate without producing HTML. */
    doctor: (source: string, projectContent: string) => Promise<CompileResult>;
  };
  platform: "win32" | "darwin" | "linux" | "browser";
}

declare global {
  interface Window {
    ezhtml?: EzhtmlBridge;
  }
}

export {};
