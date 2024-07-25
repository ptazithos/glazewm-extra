use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::windows::{get_window_info, set_window_alpha, set_window_rounded, set_window_titlebar};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawWindowRule {
    pub command: String,
    pub match_process_name: Option<String>,
    pub match_class_name: Option<String>,
    pub match_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub window_rules: Option<Vec<RawWindowRule>>,
    pub focused_window_rules: Option<Vec<RawWindowRule>>,
    pub unfocused_window_rules: Option<Vec<RawWindowRule>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            window_rules: Some(vec![
                RawWindowRule {
                    command: "set title false".to_string(),
                    match_process_name: Some(".*".to_string()),
                    match_class_name: None,
                    match_title: None,
                },
                RawWindowRule {
                    command: "set rounded false".to_string(),
                    match_process_name: Some(".*".to_string()),
                    match_class_name: None,
                    match_title: None,
                },
            ]),
            focused_window_rules: Some(vec![RawWindowRule {
                command: "set translucent 255".to_string(),
                match_process_name: Some(".*".to_string()),
                match_class_name: None,
                match_title: None,
            }]),
            unfocused_window_rules: Some(vec![RawWindowRule {
                command: "set translucent 220".to_string(),
                match_process_name: Some(".*".to_string()),
                match_class_name: None,
                match_title: None,
            }]),
        }
    }
}

#[derive(Debug, Clone)]
enum CommandType {
    Set,
}

#[derive(Debug, Clone)]
enum CommandCategory {
    Translucent(u8),
    Titlebar(bool),
    Rounded(bool),
}

#[derive(Debug, Clone)]
pub struct WindowRule {
    command: (CommandType, CommandCategory),
    match_process_name: Option<String>,
    match_class_name: Option<String>,
    match_title: Option<String>,
}

impl WindowRule {
    fn from_raw(rule: &RawWindowRule) -> Option<Self> {
        let elements = rule.command.split_whitespace().collect::<Vec<&str>>();
        if let Some(command_type) = elements.get(0) {
            if *command_type == "set" {
                if let Some(category) = elements.get(1) {
                    match *category {
                        "translucent" => {
                            let value = elements
                                .get(2)
                                .unwrap_or(&"255")
                                .parse::<u8>()
                                .unwrap_or(255);

                            Some(WindowRule {
                                command: (CommandType::Set, CommandCategory::Translucent(value)),
                                match_class_name: rule.match_class_name.clone(),
                                match_process_name: rule.match_process_name.clone(),
                                match_title: rule.match_title.clone(),
                            })
                        }
                        "title" => {
                            let value = elements
                                .get(2)
                                .unwrap_or(&"true")
                                .parse::<bool>()
                                .unwrap_or(true);

                            Some(WindowRule {
                                command: (CommandType::Set, CommandCategory::Titlebar(value)),
                                match_class_name: rule.match_class_name.clone(),
                                match_process_name: rule.match_process_name.clone(),
                                match_title: rule.match_title.clone(),
                            })
                        }
                        "rounded" => {
                            let value = elements
                                .get(2)
                                .unwrap_or(&"true")
                                .parse::<bool>()
                                .unwrap_or(true);

                            Some(WindowRule {
                                command: (CommandType::Set, CommandCategory::Rounded(value)),
                                match_class_name: rule.match_class_name.clone(),
                                match_process_name: rule.match_process_name.clone(),
                                match_title: rule.match_title.clone(),
                            })
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn apply(&self, hwnd: isize) {
        let window_info = get_window_info(hwnd);

        let is_title_matched = self.match_title.clone().map_or(false, |match_title| {
            Regex::new(&match_title)
                .map(|re| re.is_match(&window_info.title))
                .unwrap_or(false)
        });

        let is_class_name_matched =
            self.match_class_name
                .clone()
                .map_or(false, |match_class_name| {
                    Regex::new(&match_class_name)
                        .map(|re| re.is_match(&window_info.class))
                        .unwrap_or(false)
                });

        let is_process_name_matched =
            self.match_process_name
                .clone()
                .map_or(false, |match_process_name| {
                    Regex::new(&match_process_name)
                        .map(|re| re.is_match(&window_info.process_name))
                        .unwrap_or(false)
                });

        if is_title_matched | is_class_name_matched | is_process_name_matched {
            match self.command.1 {
                CommandCategory::Translucent(value) => set_window_alpha(hwnd, value),
                CommandCategory::Titlebar(value) => set_window_titlebar(hwnd, value),
                CommandCategory::Rounded(value) => set_window_rounded(hwnd, value),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub window_rules: Vec<WindowRule>,
    pub focused_window_rules: Vec<WindowRule>,
    pub unfocused_window_rules: Vec<WindowRule>,
}

impl AppConfig {
    fn from_config(config: &Config) -> Self {
        AppConfig {
            window_rules: config
                .window_rules
                .clone()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|rule| WindowRule::from_raw(&rule))
                .collect(),
            focused_window_rules: config
                .focused_window_rules
                .clone()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|rule| WindowRule::from_raw(&rule))
                .collect(),
            unfocused_window_rules: config
                .unfocused_window_rules
                .clone()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|rule| WindowRule::from_raw(&rule))
                .collect(),
        }
    }
}

pub fn parse_config() -> AppConfig {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config");
    config_path.push("glazewm-extra.toml");

    match fs::metadata(&config_path) {
        Ok(_) => {
            let config_str = fs::read_to_string(&config_path).unwrap();

            if let Ok(config) = toml::from_str::<Config>(&config_str) {
                AppConfig::from_config(&config)
            } else {
                AppConfig::from_config(&Config::default())
            }
        }
        Err(_) => {
            let config_str = toml::to_string(&Config::default()).unwrap();
            let _ = fs::write(&config_path, config_str);
            AppConfig::from_config(&Config::default())
        }
    }
}
