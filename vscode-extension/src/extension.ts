import * as vscode from "vscode";
import { execFile } from "child_process";
import * as path from "path";
import * as fs from "fs";
import { createPanel } from "./panel";

let output = vscode.window.createOutputChannel("Meika-256");

export function activate(context: vscode.ExtensionContext) {
    // Command: Encrypt via quick picker
    const encryptCmd = vscode.commands.registerCommand(
        "meika256.encrypt",
        () => runQuick("encrypt")
    );

    // Command: Decrypt via quick picker
    const decryptCmd = vscode.commands.registerCommand(
        "meika256.decrypt",
        () => runQuick("decrypt")
    );

    // Command: Open Web UI panel
    const panelCmd = vscode.commands.registerCommand(
        "meika256.openPanel",
        () => createPanel(context)
    );

    context.subscriptions.push(encryptCmd, decryptCmd, panelCmd);
}

/* ---------------- CLI quick mode ---------------- */

async function runQuick(action: "encrypt" | "decrypt") {
    const files = await vscode.window.showOpenDialog({
        canSelectMany: false
    });

    if (!files || files.length === 0) {
        return;
    }

    const password = await vscode.window.showInputBox({
        prompt: "Enter password",
        password: true
    });

    if (!password) {
        return;
    }

    const inputPath = files[0].fsPath;
    const outputPath =
        action === "encrypt"
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
        vscode.window.showErrorMessage(
            "Meika CLI not found. Run `cargo build --release -p meika256-cli`"
        );
        return;
    }

    output.clear();
    output.show(true);
    output.appendLine(`${action.toUpperCase()} ${inputPath}`);

    const args = [action, inputPath, outputPath, password];

    vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title: `Meika-256 ${action}ing`,
            cancellable: false
        },
        () =>
            new Promise<void>((resolve, reject) => {
                const proc = execFile(cliPath, args, (err) => {
                    if (err) {
                        output.appendLine(`ERROR: ${err.message}`);
                        vscode.window.showErrorMessage("Operation failed");
                        reject(err);
                    } else {
                        vscode.window.showInformationMessage(
                            `${action.toUpperCase()} complete`
                        );
                        resolve();
                    }
                });

                proc.stdout?.on("data", (d) =>
                    output.append(d.toString())
                );
                proc.stderr?.on("data", (d) =>
                    output.append(`ERROR: ${d}`)
                );
            })
    );
}

export function deactivate() {}
