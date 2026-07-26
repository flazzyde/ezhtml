import * as vscode from "vscode";

const EZHTML_LANGUAGE_ID = "ezhtml";

export function activate(context: vscode.ExtensionContext) {
  // The MVP focuses on declarative contributions in package.json. This
  // extension module is reserved for adding LSP bridge, formatters and live
  // diagnostics once the `ezhtml` LSP is shipped.

  const formatCommand = vscode.commands.registerCommand(
    "ezhtml.format",
    async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== EZHTML_LANGUAGE_ID) {
        return;
      }
      // Simple whitespace normalisation; the authoritative formatter is
      // `ezhtml format` in the CLI.
      const edits: vscode.TextEdit[] = [];
      for (let i = 0; i < editor.document.lineCount; i++) {
        const line = editor.document.lineAt(i);
        const trimmed = line.text.replace(/\s+$/g, "");
        if (trimmed !== line.text) {
          edits.push(vscode.TextEdit.replace(line.range, trimmed));
        }
      }
      await editor.edit((b) => edits.forEach((e) => b.replace(e.range, e.newText)));
    }
  );

  const openDocsCommand = vscode.commands.registerCommand(
    "ezhtml.openDocs",
    () => vscode.env.openExternal(vscode.Uri.parse("https://ezhtml.org/docs"))
  );

  context.subscriptions.push(formatCommand, openDocsCommand);
}

export function deactivate() { }
