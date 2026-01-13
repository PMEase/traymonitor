use serde::{Deserialize, Serialize};
use serde_json::json;
use specta::Type;
use tauri::{AppHandle, Wry};
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
            server_url: "http://quickbuild:8810".to_string(),
            user: "user".to_string(),
            token: "token".to_string(),
            poll_interval_in_secs: default_poll_interval_in_secs(),
        }
    }
}

const STORE_FILE_NAME: &str = "settings.json";

impl AppSettings {
    pub fn get(app: &AppHandle<Wry>) -> Result<Option<Self>, String> {
        match app.store(STORE_FILE_NAME).map(|s| s.get("settings")) {
            Ok(Some(store)) => match serde_json::from_value(store) {
                Ok(settings) => Ok(Some(settings)),
                Err(e) => Err(format!("Failed to deserialize app settings: {e}")),
            },
            _ => Ok(None),
        }
    }

    // pub fn update(app: &AppHandle<Wry>, update: impl FnOnce(&mut Self)) -> Result<(), String> {
    //     let Ok(store) = app.store(STORE_FILE_NAME) else {
    //         return Err("App settings store not found".to_string());
    //     };

    //     let mut settings = Self::get(app)?.unwrap_or_default();
    //     update(&mut settings);
    //     store.set("settings", json!(settings));
    //     store
    //         .save()
    //         .map_err(|e| format!("Failed to save app settings: {e}"))
    // }

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

pub fn is_enable_notifications(app: &AppHandle<Wry>) -> bool {
    AppSettings::get(app)
        .map(|settings| settings.unwrap_or_default().enable_notifications)
        .unwrap_or_default()
}

pub fn server_url(app: &AppHandle<Wry>) -> String {
    AppSettings::get(app)
        .map(|settings| settings.unwrap_or_default().server_url.clone())
        .unwrap_or_default()
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
