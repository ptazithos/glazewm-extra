use std::future::Future;

use serde_json::Value;
use tokio::select;
use tracing::{error, info};

use crate::config::AppConfig;

pub trait EventRegistry {
    fn register<F>(&mut self, closure: F)
    where
        F: Fn(&str) + Send + 'static;
    fn listen(&self) -> impl Future<Output = Result<(), anyhow::Error>>;
}

pub struct EffectService<T: EventRegistry> {
    ipc: T,
    config: AppConfig,
}
impl<T: EventRegistry> EffectService<T> {
    pub fn new(config: AppConfig, ipc: T) -> Self {
        info!("Init EffectService with config: {:?}", config);
        let mut service = EffectService { config, ipc };

        service.setup_ipc_callbacks();

        service
    }

    fn setup_ipc_callbacks(&mut self) {
        let config = self.config.clone();
        self.ipc.register(move |msg| {
            if let Ok(payload) = serde_json::from_str(msg) as Result<Value, _> {
                if let Some(response_type) = payload["data"]["type"].as_str() {
                    match response_type {
                        "focus_changed" => {
                            if let Some(hwnd) =
                                payload["data"]["focusedContainer"]["handle"].as_i64()
                            {
                                info!("Focused window handle: {:?}", config);
                                handle_fucous_change(&config, hwnd.try_into().unwrap());
                            }
                        }
                        "window_managed" => {
                            if let Some(hwnd) = &payload["data"]["managedWindow"]["handle"].as_i64()
                            {
                                info!("Managed window handle: {}", hwnd);
                            }
                        }
                        _ => {
                            info!("Unknown response {}", msg);
                        }
                    }
                }
            } else {
                error!("Failed to parse message: {}", msg);
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

fn handle_fucous_change(config: &AppConfig, hwnd: isize) {
    let focused_rules = match &config.focused_window_rules {
        Some(value) => value,
        None => &Vec::new(),
    };

    let blur_rules = match &config.unfocused_window_rules {
        Some(value) => value,
        None => &Vec::new(),
    };
}

fn apply_window_effect() {}
