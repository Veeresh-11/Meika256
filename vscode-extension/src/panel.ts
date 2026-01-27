import * as vscode from "vscode";
import { execFile } from "child_process";
import * as path from "path";
import * as fs from "fs";

export function createPanel(context: vscode.ExtensionContext) {
    const panel = vscode.window.createWebviewPanel(
        "meikaPanel",
        "Meika-256",
        vscode.ViewColumn.One,
        { enableScripts: true }
    );

    panel.webview.html = getHtml();

    panel.webview.onDidReceiveMessage(async (msg) => {
        if (msg.command !== "encrypt" && msg.command !== "decrypt") {
            return;
        }

        const files = await vscode.window.showOpenDialog({
            canSelectMany: false
        });

        if (!files || files.length === 0) {
            vscode.window.showWarningMessage("No file selected");
            return;
        }

        const inputPath = files[0].fsPath;
        const outputPath =
            msg.command === "encrypt"
                ? inputPath + ".meika"
                : inputPath.replace(/\.meika$/, ".decrypted");

        const workspace = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
        if (!workspace) {
            vscode.window.showErrorMessage("Open a workspace first");
            return;
        }

        const exeName =
            process.platform === "win32" ? "meika256-cli.exe" : "meika256-cli";

        const cliPath = path.join(
            workspace,
            "cli",
            "target",
            "release",
            exeName
        );

        if (!fs.existsSync(cliPath)) {
            vscode.window.showErrorMessage("Meika CLI not found. Build it first.");
            return;
        }

        const args = [msg.command, inputPath, outputPath, msg.password];

        vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: `Meika-256 ${msg.command}ing`,
                cancellable: false
            },
            () =>
                new Promise<void>((resolve, reject) => {
                    execFile(cliPath, args, (err) => {
                        if (err) {
                            vscode.window.showErrorMessage("Operation failed");
                            reject(err);
                        } else {
                            vscode.window.showInformationMessage(
                                `${msg.command.toUpperCase()} complete`
                            );
                            resolve();
                        }
                    });
                })
        );
    });
}

function getHtml(): string {
    return `
<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<style>
body { font-family: sans-serif; padding: 16px; }
button { margin-right: 8px; }
</style>
</head>
<body>
<h3>Meika-256 Encryption</h3>

<input id="password" type="password" placeholder="Password" />
<br><br>

<button onclick="encrypt()">Encrypt File</button>
<button onclick="decrypt()">Decrypt File</button>

<script>
const vscode = acquireVsCodeApi();

function encrypt() {
    const pwd = document.getElementById("password").value;
    vscode.postMessage({ command: "encrypt", password: pwd });
}

function decrypt() {
    const pwd = document.getElementById("password").value;
    vscode.postMessage({ command: "decrypt", password: pwd });
}
</script>

</body>
</html>`;
}
