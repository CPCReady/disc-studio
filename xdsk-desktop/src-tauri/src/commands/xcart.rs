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

#[tauri::command]
pub async fn run_xcart(app: tauri::AppHandle, args: Vec<String>) -> Result<CommandOutput, String> {
    let shell = app.shell();

    let command = match shell.sidecar("xcart") {
        Ok(cmd) => cmd.args(&args),
        Err(_) => shell.command("xcart").args(&args),
    };

    let output = command
        .output()
        .await
        .map_err(|e| format!("Failed to execute xcart: {e}"))?;

    Ok(CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        success: output.status.success(),
        code: output.status.code().unwrap_or(-1),
    })
}

#[tauri::command]
pub async fn xcart_available(app: tauri::AppHandle) -> bool {
    let shell = app.shell();

    if let Ok(cmd) = shell.sidecar("xcart") {
        if cmd.args(["--version"]).output().await.map(|o| o.status.success()).unwrap_or(false) {
            return true;
        }
    }

    shell
        .command("xcart")
        .args(["--version"])
        .output()
        .await
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[tauri::command]
pub async fn xcart_version(app: tauri::AppHandle) -> Result<String, String> {
    let shell = app.shell();

    let output = match shell.sidecar("xcart") {
        Ok(cmd) => cmd.args(["--version"]).output().await,
        Err(_) => shell.command("xcart").args(["--version"]).output().await,
    }
    .map_err(|e| format!("Failed to get xcart version: {e}"))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[tauri::command]
pub fn xcart_roms_ready(path: String) -> bool {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return false;
    }

    let base = std::path::Path::new(trimmed);
    if !base.is_dir() {
        return false;
    }

    ["os.rom", "basic.rom", "amsdos.rom"]
        .iter()
        .all(|name| base.join(name).is_file())
}
