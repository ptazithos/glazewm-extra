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

            let _daemon_window = tauri::WindowBuilder::new(
                app,
                "daemon",
                tauri::WindowUrl::App("daemon.html".into()),
            )
            .visible(false)
            .build()?;

            let _overview_window = tauri::WindowBuilder::new(
                app,
                "overview",
                tauri::WindowUrl::App("overview.html".into()),
            )
            .visible(false)
            .decorations(false)
            .transparent(true)
            .build()?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            windows::set_window_alpha,
            windows::set_window_titlebar,
            windows::set_window_rounded,
            windows::get_window_title,
            windows::get_window_class,
            windows::get_window_process_name,
            windows::capture_window,
            config::get_app_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
