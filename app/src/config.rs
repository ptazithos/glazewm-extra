use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{App, Manager};

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowRule {
    pub command: String,
    pub match_process_name: Option<String>,
    pub match_class_name: Option<String>,
    pub match_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    window_rules: Vec<WindowRule>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            window_rules: vec![
                WindowRule {
                    command: "set transparent 220".to_string(),
                    match_process_name: Some("*".to_string()),
                    match_class_name: None,
                    match_title: None,
                },
                WindowRule {
                    command: "set title false".to_string(),
                    match_process_name: Some("*".to_string()),
                    match_class_name: None,
                    match_title: None,
                },
            ],
        }
    }
}

pub fn setup_store(app: &mut App) {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config");
    config_path.push("glazewm-extra.toml");

    match fs::metadata(&config_path) {
        Ok(_) => {
            let config_str = fs::read_to_string(&config_path).unwrap();

            if let Ok(app_config) = toml::from_str::<AppConfig>(&config_str) {
                app.manage(app_config);
            } else {
                app.manage(AppConfig::default());
            }
        }
        Err(_) => {
            let config_str = toml::to_string(&AppConfig::default()).unwrap();
            let _ = fs::write(&config_path, config_str);
            app.manage(AppConfig::default());
        }
    }
}

#[tauri::command]
pub fn get_app_config(state: tauri::State<AppConfig>) -> &AppConfig {
    state.inner()
}
