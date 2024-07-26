use std::future::Future;

use serde_json::Value;
use tokio::select;
use tracing::{error, info};

use crate::{
    config::AppConfig,
    windows::{set_window_alpha, set_window_rounded, set_window_titlebar},
};

pub trait EventRegistry {
    fn register<F>(&mut self, closure: F)
    where
        F: Fn(&str, &Vec<isize>) + Send + 'static;
    fn listen(&mut self) -> impl Future<Output = Result<(), anyhow::Error>>;
}

pub struct EffectService<M: EventRegistry, N: EventRegistry> {
    ipc: M,
    tray: N,
    config: AppConfig,
}
impl<M: EventRegistry, N: EventRegistry> EffectService<M, N> {
    pub fn new(config: AppConfig, ipc: M, tray: N) -> Self {
        info!("Init EffectService with config: {:?}", config);
        let mut service = EffectService { config, ipc, tray };

        service.setup_ipc_callbacks();
        service.setup_tray_callbacks();
        service
    }

    fn setup_ipc_callbacks(&mut self) {
        let config = self.config.clone();
        self.ipc.register(move |msg, hwnds| {
            if let Ok(payload) = serde_json::from_str(msg) as Result<Value, _> {
                if let Some(response_type) = payload["data"]["type"].as_str() {
                    match response_type {
                        "focus_changed" => {
                            if let Some(focused_hwnd) =
                                payload["data"]["focusedContainer"]["handle"].as_i64()
                            {
                                info!("Focused window: {}", msg);
                                hwnds.iter().for_each(|hwnd| {
                                    if *hwnd == isize::try_from(focused_hwnd).unwrap() {
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
                        }
                        "window_managed" => {
                            if let Some(hwnd) = payload["data"]["managedWindow"]["handle"].as_i64()
                            {
                                info!("Managed window: {}", msg);
                                config
                                    .window_rules
                                    .iter()
                                    .for_each(|rule| rule.apply(isize::try_from(hwnd).unwrap()))
                            }
                        }
                        _ => {
                            info!("Unknown response {}", msg);
                        }
                    }
                } else {
                    if let Some(response_type) = payload["messageType"].as_str() {
                        match response_type {
                            "client_response" => {
                                hwnds.iter().for_each(|hwnd| {
                                    config
                                        .window_rules
                                        .iter()
                                        .for_each(|rule| rule.apply(*hwnd))
                                });
                            }
                            _ => {
                                error!("Unknown message type: {}", msg);
                            }
                        }
                    } else {
                        error!("Parsed message no data type: {}", msg);
                    }
                }
            } else {
                error!("Failed to parse message: {}", msg);
            }
        });
    }

    fn setup_tray_callbacks(&mut self) {
        self.tray.register(|msg, hwnds| match msg {
            "quit" => {
                hwnds.iter().for_each(|hwnd| {
                    set_window_alpha(*hwnd, 255);
                    set_window_rounded(*hwnd, true);
                    set_window_titlebar(*hwnd, true);
                });

                std::process::exit(0);
            }
            _ => {
                info!("Unknown tray event: {}", msg);
            }
        })
    }

    pub async fn serve(&mut self) {
        let tray_fut = self.tray.listen();

        let ipc_fut = self.ipc.listen();

        select! {
            res = tray_fut => { error!("Tray error: {:?}", res) },
            res = ipc_fut => {error!("IPC error: {:?}", res)},
            _ = tokio::signal::ctrl_c() => {
                println!("Shutting down...");
            }
        }
    }
}
