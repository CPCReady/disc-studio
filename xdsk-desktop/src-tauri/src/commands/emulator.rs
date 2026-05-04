// MIT License
// Copyright (c) Destroyer 2026.
use serde::{Deserialize, Serialize};
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorOutput {
    pub launched: bool,
    pub error: Option<String>,
}

/// Launch RetroVirtualMachine with a disk image, cross-platform.
/// On macOS: `open -a RetroVirtualMachine.app --args ...`
/// On Windows: `RetroVirtualMachine.exe ...`
/// On Linux: `RetroVirtualMachine ...`
#[tauri::command]
pub async fn launch_emulator(
    app: tauri::AppHandle,
    emulator_path: String,
    disk_path: String,
    machine: Option<String>,
    run_file: Option<String>,
) -> Result<EmulatorOutput, String> {
    if emulator_path.is_empty() {
        return Err("Emulator path is not configured".to_string());
    }

    let shell = app.shell();
    let machine_id = machine.unwrap_or_else(|| "cpc6128".to_string());

    // Build RVM args (common across platforms)
    // RVM CLI: --boot=<machine> --insert <file> [--command=<cmd>]
    let mut rvm_args: Vec<String> = vec![
        format!("--boot={}", machine_id),
        "--insert".to_string(),
        disk_path.clone(),
    ];
    
    // Add command: run"FILE"\n if file specified, otherwise CAT\n
    let command = if let Some(ref file) = run_file {
        format!("--command=run\"{}\"\n", file)
    } else {
        "--command=CAT\n".to_string()
    };
    rvm_args.push(command);

    #[cfg(target_os = "macos")]
    {
        // macOS: open -n -a <app_path> --args <rvm_args...>
        // -n forces a new instance even if one is already running
        let mut args = vec!["-n".to_string(), "-a".to_string(), emulator_path];
        args.push("--args".to_string());
        args.extend(rvm_args);
        shell
            .command("open")
            .args(&args)
            .spawn()
            .map_err(|e| format!("Failed to launch emulator: {e}"))?;
    }

    #[cfg(target_os = "windows")]
    {
        let exe = if emulator_path.ends_with(".exe") {
            emulator_path.clone()
        } else {
            format!("{}.exe", emulator_path)
        };
        shell
            .command(&exe)
            .args(&rvm_args)
            .spawn()
            .map_err(|e| format!("Failed to launch emulator: {e}"))?;
    }

    #[cfg(target_os = "linux")]
    {
        shell
            .command(&emulator_path)
            .args(&rvm_args)
            .spawn()
            .map_err(|e| format!("Failed to launch emulator: {e}"))?;
    }

    Ok(EmulatorOutput { launched: true, error: None })
}
