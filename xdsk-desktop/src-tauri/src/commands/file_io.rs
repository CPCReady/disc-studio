// MIT License
// Copyright (c) Destroyer 2026.

#[tauri::command]
pub fn write_text_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| format!("Failed to write file '{}': {}", path, e))
}
