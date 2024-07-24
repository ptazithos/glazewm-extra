use std::future::Future;

use serde_json::Value;
use tokio::select;
use tracing::info;

pub trait EventRegistry {
    fn register(&mut self, callback: fn(payload: &str));
    fn listen(&self) -> impl Future<Output = Result<(), anyhow::Error>>;
}

pub struct EffectService<T: EventRegistry> {
    pub ipc: T,
}
impl<T: EventRegistry> EffectService<T> {
    pub fn new(ipc: T) -> Self {
        let mut service = EffectService { ipc };
        service.setup_ipc_callbacks();

        service
    }

    pub fn setup_ipc_callbacks(&mut self) {
        self.ipc.register(|msg| {
            let payload: Value = serde_json::from_str(msg).unwrap();
            if let Some(response_type) = payload["data"]["type"].as_str() {
                match response_type {
                    "focus_changed" => {
                        if let Some(hwnd) = &payload["data"]["focusedContainer"]["handle"].as_i64()
                        {
                            info!("Focused window handle: {}", hwnd);
                        }
                    }
                    "window_managed" => {
                        if let Some(hwnd) = &payload["data"]["managedWindow"]["handle"].as_i64() {
                            info!("Managed window handle: {}", hwnd);
                        }
                    }
                    _ => {
                        info!("Unknown response {}", msg);
                    }
                }
            }
        });
    }

    pub async fn serve(&self) {
        let ipc_registry = &self.ipc;
        let ipc_fut = ipc_registry.listen();

        select! {
            _ = ipc_fut => {},
            _ = tokio::signal::ctrl_c() => {
                println!("Shutting down...");
            }
        }
    }
}
