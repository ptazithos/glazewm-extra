use std::future::Future;

use serde_json::Value;
use tokio::select;
use tracing::{error, info};

use crate::{config::AppConfig, windows::get_visible_windows};

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
                                handle_focus_change(&config, hwnd.try_into().unwrap());
                            }
                        }
                        "window_managed" => {
                            if let Some(hwnd) = payload["data"]["managedWindow"]["handle"].as_i64()
                            {
                                info!("Managed window handle: {}", hwnd);
                                handle_window_managed(&config, hwnd.try_into().unwrap());
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

fn handle_focus_change(config: &AppConfig, focused_hwnd: isize) {
    let windows = get_visible_windows();

    windows.iter().for_each(|hwnd| {
        if *hwnd == focused_hwnd {
            config.focused_window_rules.iter().for_each(|rule| {
                rule.apply(*hwnd);
            });
        } else {
            config.unfocused_window_rules.iter().for_each(|rule| {
                rule.apply(*hwnd);
            });
        }
    });
}

fn handle_window_managed(config: &AppConfig, hwnd: isize) {
    config.window_rules.iter().for_each(|rule| rule.apply(hwnd))
}
