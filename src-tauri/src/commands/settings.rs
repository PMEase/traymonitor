use tauri::{AppHandle, Wry};

use crate::settings::AppSettings;

#[tauri::command]
#[specta::specta]
pub async fn load_settings(app: AppHandle<Wry>) -> Result<AppSettings, String> {
    tracing::debug!("Loading app settings ...");
    AppSettings::get(&app).map(|settings| settings.unwrap_or_default())
}

#[tauri::command]
#[specta::specta]
pub async fn save_settings(app: AppHandle<Wry>, settings: AppSettings) -> Result<(), String> {
    tracing::debug!("Saving app settings ...");
    let _ = settings.save(&app);
    Ok(())
}
