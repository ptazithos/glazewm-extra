mod ipc;
mod service;

use ipc::IPCEventRegistry;
use service::EffectService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let ipc_registry = IPCEventRegistry::new();

    let app_service = EffectService::new(ipc_registry);
    app_service.serve().await;
}
