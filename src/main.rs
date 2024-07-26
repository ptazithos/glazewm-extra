mod config;
mod ipc;
mod service;
mod tray;
mod windows;

use ipc::IPCEventRegistry;
use service::EffectService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app_config = config::parse_config();
    let ipc_registry = IPCEventRegistry::new();

    let tray_registry = tray::TrayEventRegistry::new();

    let mut app_service = EffectService::new(app_config, ipc_registry, tray_registry);
    app_service.serve().await;
}
