#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod ipc;
mod service;
mod tray;
mod windows;

use ipc::IPCEventRegistry;
use service::EffectService;
use single_instance::SingleInstance;
use tracing::error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_mutex = SingleInstance::new("glazewm-extra").unwrap();
    if !app_mutex.is_single() {
        error!("Another instance is already running.");
        return;
    }

    let app_config = config::parse_config();

    let ipc_registry = IPCEventRegistry::new();
    let tray_registry = tray::TrayEventRegistry::new();

    let mut app_service = EffectService::new(app_config, ipc_registry, tray_registry);
    app_service.serve().await;
}
