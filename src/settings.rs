use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

const CONFIG_FILE: &str = "settings/settings.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlertConfig {
    pub enabled: bool,
    pub telegram_bot_token: Option<String>,
    pub telegram_chat_id: Option<String>,
    pub webhook_url: Option<String>,
    pub custom_message: Option<String>,
    
    // New Services
    pub pushover_user_key: Option<String>,
    pub pushover_api_token: Option<String>,
    pub gotify_url: Option<String>,
    pub gotify_token: Option<String>,
    pub slack_webhook_url: Option<String>,
    pub discord_webhook_url: Option<String>,
    pub teams_webhook_url: Option<String>,
    pub ntfy_url: Option<String>,
    pub ntfy_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ThresholdType {
    MaxUsed,
    MinRemaining,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonitoredPath {
    pub path: String,
    pub threshold_type: ThresholdType,
    pub threshold_value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub paths: Vec<MonitoredPath>,
    pub check_interval_minutes: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub language: String,
    pub monitoring: MonitoringConfig,
    pub alerts: AlertConfig,
    pub layout: Option<serde_json::Value>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            monitoring: MonitoringConfig {
                enabled: false,
                paths: vec![MonitoredPath {
                    path: "/".to_string(),
                    threshold_type: ThresholdType::MaxUsed,
                    threshold_value: 100.0,
                }],
                check_interval_minutes: 60,
            },
            alerts: AlertConfig {
                enabled: false,
                telegram_bot_token: None,
                telegram_chat_id: None,
                webhook_url: None,
                custom_message: None,
                pushover_user_key: None,
                pushover_api_token: None,
                gotify_url: None,
                gotify_token: None,
                slack_webhook_url: None,
                discord_webhook_url: None,
                teams_webhook_url: None,
                ntfy_url: None,
                ntfy_token: None,
            },
            layout: Some(default_layout()),
        }
    }
}

pub fn default_layout() -> serde_json::Value {
    serde_json::json!([
        { "h": 10, "id": "widget-largedirs", "w": 6, "x": 0, "y": 0 },
        { "h": 10, "id": "widget-browser", "w": 6, "x": 6, "y": 0 },
        { "h": 7, "id": "widget-filetypes", "w": 6, "x": 0, "y": 10 },
        { "h": 7, "id": "widget-toptypes", "w": 6, "x": 6, "y": 10 },
        { "h": 3, "id": "widget-diskspace", "w": 12, "x": 0, "y": 17 }
    ])
}

pub type SharedSettings = Arc<Mutex<Settings>>;

pub fn load_settings() -> Settings {
    if Path::new(CONFIG_FILE).exists() {
        match fs::read_to_string(CONFIG_FILE) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(settings) => return settings,
                Err(e) => println!("Error parsing settings: {}", e),
            },
            Err(e) => println!("Error reading settings file: {}", e),
        }
    }
    Settings::default()
}

pub fn save_settings(settings: &Settings) -> std::io::Result<()> {
    if let Some(parent) = Path::new(CONFIG_FILE).parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            println!("Failed to create settings directory '{}': {}", parent.display(), e);
            return Err(e);
        }
    }
    let content = serde_json::to_string_pretty(settings)?;
    if let Err(e) = fs::write(CONFIG_FILE, content) {
        println!("Failed to write settings file '{}': {}", CONFIG_FILE, e);
        return Err(e);
    }
    Ok(())
}
