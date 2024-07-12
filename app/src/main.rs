// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_log::LogTarget;

mod command;
mod config;
mod menu;
mod windows;

use menu::{generate_menu, menu_event_handler};

fn main() {
    tauri::Builder::default()
        .system_tray(generate_menu())
        .on_system_tray_event(menu_event_handler)
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout])
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            command::command_execute(app, argv);
        }))
        .setup(|app| {
            config::setup_store(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            windows::set_window_alpha,
            windows::set_window_titlebar,
            windows::get_window_name,
            windows::get_window_class,
            config::get_app_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
