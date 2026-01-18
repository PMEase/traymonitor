use std::sync::Mutex;

use tauri::{AppHandle, Manager, State, Wry};

use crate::{AppState, constants::DASHBOARD_WINDOW_NAME, types::settings::AppSettings};

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
    settings.save(&app)?;
    let win = app.get_webview_window(DASHBOARD_WINDOW_NAME).unwrap();
    let _ = win.navigate(settings.get_dashboard_url());
    let state = app.state::<Mutex<AppState>>();
    state.lock().unwrap().reload_settings(&app)
}
