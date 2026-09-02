import * as vscode from 'vscode';
import WebSocket from 'ws';

let wsClient: WebSocket | null = null;

export function activate(context: vscode.ExtensionContext) {
    console.log('PIRTM Governed Extension Active.');

    const daemonUrl = 'ws://127.0.0.1:8090';
    try {
        wsClient = new WebSocket(daemonUrl);
        wsClient.on('open', () => {
            vscode.window.showInformationMessage('Connected to PIRTM Daemon (pirtmd) at ws://127.0.0.1:8090');
        });
    } catch (e) {
        vscode.window.showWarningMessage('Could not connect to pirtmd daemon. Start `pirtmd serve`.');
    }

    let compileCmd = vscode.commands.registerCommand('pirtm.compile', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return;

        const source = editor.document.getText();
        if (!source.trim()) {
            vscode.window.showErrorMessage('PIRTM: Active document is empty.');
            return;
        }

        // Extract theorem "..." from file or prompt user (ADR-055 fail-closed rule)
        const match = source.match(/theorem\s+["']([^"']+)["']/);
        let theoremName = match ? match[1] : '';

        if (!theoremName) {
            const input = await vscode.window.showInputBox({
                prompt: 'Enter Lean 4 Theorem Anchor Name (ADR-055 mandate)',
                placeHolder: 'e.g. Foundations.ADR.BoundedIteration.iterate_non_expansive'
            });
            if (!input || !input.trim()) {
                vscode.window.showErrorMessage('PIRTM: Compilation aborted — theorem_name anchor is required per ADR-055.');
                return;
            }
            theoremName = input.trim();
        }

        if (wsClient && wsClient.readyState === WebSocket.OPEN) {
            wsClient.send(JSON.stringify({
                id: Date.now(),
                method: 'compile',
                params: { source, name: 'vscode_module', theorem_name: theoremName }
            }));
            vscode.window.showInformationMessage(`PIRTM: Compilation request sent to daemon (Theorem: ${theoremName}).`);
        }
    });

    context.subscriptions.push(compileCmd);
}

export function deactivate() {
    if (wsClient) wsClient.close();
}
