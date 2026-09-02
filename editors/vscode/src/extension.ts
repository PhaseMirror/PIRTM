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

    let compileCmd = vscode.commands.registerCommand('pirtm.compile', () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return;

        const source = editor.document.getText();
        if (wsClient && wsClient.readyState === WebSocket.OPEN) {
            wsClient.send(JSON.stringify({
                id: Date.now(),
                method: 'compile',
                params: { source, name: 'vscode_module', theorem_name: 'Foundations.ADR.BoundedIteration.iterate_non_expansive' }
            }));
            vscode.window.showInformationMessage('PIRTM: Compilation request sent to daemon.');
        }
    });

    context.subscriptions.push(compileCmd);
}

export function deactivate() {
    if (wsClient) wsClient.close();
}
