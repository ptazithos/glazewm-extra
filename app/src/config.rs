use std::fs;
use tauri::{App, Manager};

struct AppStore {
    pub translucent_window: TranslucentWindowConfig,
}

struct TranslucentWindowConfig {
    pub enable: bool,
    pub alpha: u8,
}

pub fn setup_store(app: &mut App) {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config");
    config_path.push("glazewm-extra.toml");

    match fs::metadata(config_path.as_path()) {
        Ok(_) => {
            println!("Config file exists")
        }
        Err(_) => {
            println!("Config file doesn't exist")
        }
    }
    app.manage(AppStore {
        translucent_window: TranslucentWindowConfig {
            enable: false,
            alpha: 240,
        },
    });
}
