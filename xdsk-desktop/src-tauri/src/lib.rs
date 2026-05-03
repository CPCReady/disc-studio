// MIT License
// Copyright (c) Destroyer 2026.
mod commands;

use tauri_plugin_window_state::StateFlags;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(StateFlags::SIZE | StateFlags::POSITION)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::xdsk::run_xdsk,
            commands::xdsk::xdsk_available,
            commands::xdsk::xdsk_version,
            commands::xcart::run_xcart,
            commands::xcart::xcart_available,
            commands::xcart::xcart_version,
            commands::emulator::launch_emulator,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
