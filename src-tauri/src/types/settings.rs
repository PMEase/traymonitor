use serde::{Deserialize, Serialize};
use serde_json::json;
use specta::Type;
use tauri::{AppHandle, Url, Wry};
use tauri_plugin_store::StoreExt;

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum AppTheme {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AppSettings {
    #[serde(default = "default_enable_notifications")]
    pub enable_notifications: bool,
    #[serde(default = "default_notifications_total")]
    pub notifications_total: u32,
    #[serde(default = "default_theme")]
    pub theme: AppTheme,
    #[serde(default)]
    pub server_url: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub token: String,
    #[serde(default = "default_poll_interval_in_secs")]
    pub poll_interval_in_secs: u32,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::option_u64_as_string"
    )]
    pub last_notified_build_id: Option<u64>,
    #[serde(default)]
    pub paused: bool,
}

fn default_enable_notifications() -> bool {
    true
}

fn default_notifications_total() -> u32 {
    100
}

fn default_theme() -> AppTheme {
    AppTheme::System
}

fn default_poll_interval_in_secs() -> u32 {
    10
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            enable_notifications: default_enable_notifications(),
            notifications_total: default_notifications_total(),
            theme: default_theme(),
            server_url: "".to_string(),
            user: "".to_string(),
            token: "".to_string(),
            poll_interval_in_secs: default_poll_interval_in_secs(),
            last_notified_build_id: None,
            paused: false,
        }
    }
}

const STORE_FILE_NAME: &str = "settings.json";

impl AppSettings {
    pub fn is_configured(&self) -> bool {
        !self.server_url.is_empty() && !self.user.is_empty() && !self.token.is_empty()
    }

    pub fn get_dashboard_url(&self) -> Url {
        format!("{}/lite", self.server_url).parse().unwrap()
    }

    pub fn get(app: &AppHandle<Wry>) -> Result<Self, String> {
        match app.store(STORE_FILE_NAME).map(|s| s.get("settings")) {
            Ok(Some(store)) => match serde_json::from_value(store) {
                Ok(settings) => Ok(settings),
                Err(e) => Err(format!("Failed to deserialize app settings: {e}")),
            },
            _ => Ok(Self::default()),
        }
    }

    pub fn update(app: &AppHandle<Wry>, update: impl FnOnce(&mut Self)) -> Result<(), String> {
        let Ok(store) = app.store(STORE_FILE_NAME) else {
            return Err("App settings store not found".to_string());
        };

        let mut settings = Self::get(app)?;
        update(&mut settings);
        store.set("settings", json!(settings));
        store
            .save()
            .map_err(|e| format!("Failed to save app settings: {e}"))
    }

    pub fn save(&self, app: &AppHandle<Wry>) -> Result<(), String> {
        let Ok(store) = app.store(STORE_FILE_NAME) else {
            return Err("App settings store not found".to_string());
        };

        store.set("settings", json!(self));
        store
            .save()
            .map_err(|e| format!("Failed to save app settings: {e}"))
    }
}

// pub fn init(app: &AppHandle<Wry>) {
//     tracing::info!("Initializing app settings");
//     let store = match AppSettings::get(app) {
//         Ok(Some(store)) => store,
//         Ok(None) => AppSettings::default(),
//         Err(e) => {
//             tracing::error!("Failed to deserialize general settings store: {}", e);
//             AppSettings::default()
//         }
//     };

//     if let Err(e) = store.save(app) {
//         tracing::error!("Failed to save general settings: {}", e);
//     }

//     tracing::info!("App settings initialized");
// }
