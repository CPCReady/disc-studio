// MIT License
// Copyright (c) Destroyer 2026.
use serde::{Deserialize, Serialize};
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub code: i32,
}

/// Execute the xdsk CLI binary with the given arguments.
/// Tries the bundled sidecar first; falls back to the system PATH.
#[tauri::command]
pub async fn run_xdsk(app: tauri::AppHandle, args: Vec<String>) -> Result<CommandOutput, String> {
    let shell = app.shell();

    let command = match shell.sidecar("xdsk") {
        Ok(cmd) => cmd.args(&args),
        Err(_) => shell.command("xdsk").args(&args),
    };

    let output = command
        .output()
        .await
        .map_err(|e| format!("Failed to execute xdsk: {e}"))?;

    Ok(CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        success: output.status.success(),
        code: output.status.code().unwrap_or(-1),
    })
}

/// Check whether the xdsk binary is reachable (sidecar or PATH).
#[tauri::command]
pub async fn xdsk_available(app: tauri::AppHandle) -> bool {
    let shell = app.shell();

    // Try sidecar
    if let Ok(cmd) = shell.sidecar("xdsk") {
        if cmd.args(["--version"]).output().await.map(|o| o.status.success()).unwrap_or(false) {
            return true;
        }
    }

    // Try PATH
    shell
        .command("xdsk")
        .args(["--version"])
        .output()
        .await
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Return the xdsk binary version string.
#[tauri::command]
pub async fn xdsk_version(app: tauri::AppHandle) -> Result<String, String> {
    let shell = app.shell();

    let output = match shell.sidecar("xdsk") {
        Ok(cmd) => cmd.args(["--version"]).output().await,
        Err(_) => shell.command("xdsk").args(["--version"]).output().await,
    }
    .map_err(|e| format!("Failed to get xdsk version: {e}"))?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string())
}
