use std::future::Future;

use regex::Regex;
use serde_json::Value;
use tokio::select;
use tracing::{error, info};

use crate::{
    config::{AppConfig, WindowRule},
    windows::{
        get_visible_windows, get_window_info, set_window_alpha, set_window_rounded,
        set_window_titlebar,
    },
};

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
    let focused_rules = match &config.focused_window_rules {
        Some(value) => value,
        None => &Vec::new(),
    };

    let blur_rules = match &config.unfocused_window_rules {
        Some(value) => value,
        None => &Vec::new(),
    };

    let windows = get_visible_windows();

    windows.iter().for_each(|hwnd| {
        if *hwnd == focused_hwnd {
            focused_rules.iter().for_each(|rule| {
                match_window_rule(rule, *hwnd);
            });
        } else {
            blur_rules.iter().for_each(|rule| {
                match_window_rule(rule, *hwnd);
            });
        }
    });
}

fn handle_window_managed(config: &AppConfig, hwnd: isize) {
    let managed_rules = match &config.window_rules {
        Some(value) => value,
        None => &Vec::new(),
    };

    managed_rules.iter().for_each(|rule| {
        match_window_rule(rule, hwnd);
    });
}

fn match_window_rule(rule: &WindowRule, hwnd: isize) {
    let window_info = get_window_info(hwnd);

    if let Some(match_title) = &rule.match_title {
        let re = Regex::new(match_title).unwrap();

        if re.is_match(&window_info.title) {
            info!("Matched rule '{}' for window {}", match_title, hwnd);
            apply_window_effect(rule, hwnd);
        } else {
            info!(
                "Skipping rule for window {} because title does not match",
                hwnd
            );
        }
        return;
    }

    if let Some(match_class) = &rule.match_class_name {
        let re = Regex::new(match_class).unwrap();

        if re.is_match(&window_info.class) {
            info!("Matched rule '{}' for window {}", match_class, hwnd);
            apply_window_effect(rule, hwnd);
        } else {
            info!(
                "Skipping rule for window {} because class does not match",
                hwnd
            );
        }
        return;
    }

    if let Some(match_process) = &rule.match_process_name {
        let re = Regex::new(match_process).unwrap();

        if re.is_match(&window_info.process_name) {
            info!("Matched rule '{}' for window {}", match_process, hwnd);
            apply_window_effect(rule, hwnd);
        } else {
            info!(
                "Skipping rule for window {} because process does not match",
                hwnd
            );
        }
        return;
    }
}

fn apply_window_effect(rule: &WindowRule, hwnd: isize) {
    let rule_elements = rule.command.split_whitespace().collect::<Vec<&str>>();
    if let Some(kind) = rule_elements.get(1) {
        match *kind {
            "translucent" => {
                let value = rule_elements
                    .get(2)
                    .unwrap_or(&"255")
                    .parse::<u8>()
                    .unwrap_or(255);
                set_window_alpha(hwnd, value);
                info!("Setting window {} to translucent {}", hwnd, value);
            }
            "title" => {
                let value = rule_elements
                    .get(2)
                    .unwrap_or(&"true")
                    .parse::<bool>()
                    .unwrap_or(true);
                set_window_titlebar(hwnd, value);
                info!("Setting window {} title to {}", hwnd, value);
            }
            "rounded" => {
                let value = rule_elements
                    .get(2)
                    .unwrap_or(&"true")
                    .parse::<bool>()
                    .unwrap_or(true);
                info!("Setting window {} rounded to {}", hwnd, value);
                set_window_rounded(hwnd, value)
            }
            _ => {
                error!("Unknown rule: {}", rule.command);
            }
        }
    }
}
