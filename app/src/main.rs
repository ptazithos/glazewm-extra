// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::setup_store;
use tauri_plugin_log::LogTarget;

mod config;
mod windows;

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout])
                .build(),
        )
        .setup(|app| {
            setup_store(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            windows::set_window_alpha,
            windows::set_window_titlebar
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
