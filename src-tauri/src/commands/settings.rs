use std::sync::Mutex;

use tauri::{AppHandle, Manager, State, Wry};

use crate::{AppState, types::settings::AppSettings};

#[tauri::command]
#[specta::specta]
pub fn load_settings(state: State<'_, Mutex<AppState>>) -> Result<AppSettings, String> {
    tracing::info!("Loading app settings ...");
    let settings = state.lock().unwrap().settings.clone();
    Ok(settings)
}

#[tauri::command]
#[specta::specta]
pub fn save_settings(app: AppHandle<Wry>, settings: AppSettings) -> Result<(), String> {
    tracing::info!("Saving app settings ...");
    let state = app.state::<Mutex<AppState>>();
    settings.save(&app)?;
    state.lock().unwrap().update_settings(settings);

    Ok(())
}
