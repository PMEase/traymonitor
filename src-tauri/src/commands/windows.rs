use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

use crate::constants::{DASHBOARD_WINDOW_NAME, MAIN_WINDOW_NAME};

#[tauri::command]
#[specta::specta]
pub fn show_dashboard_window(app: tauri::AppHandle) -> Result<(), String> {
    // hide main window if it is visible
    if let Some(main) = app.get_webview_window(MAIN_WINDOW_NAME) {
        let _ = main.hide();
    }

    // let state = app.state::<Mutex<AppState>>();
    // let settings = state.lock().unwrap().settings.clone();
    // let url = settings.server_url.clone();
    // let url: Url = format!("{}/lite", url)
    //     .parse()
    //     .map_err(|e| format!("Failed to parse URL: {e}"))
    //     .unwrap();

    let window = app
        .get_webview_window(DASHBOARD_WINDOW_NAME)
        .ok_or("Dashboard window not found")
        .unwrap();
    let _ = window.move_window(Position::TrayCenter);
    // let _ = window.eval(format!("window.location.href = '{}';", url));
    let _ = window.show();
    window.set_focus().unwrap();

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn close_dashboard_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(dashboard) = app.get_webview_window(DASHBOARD_WINDOW_NAME) {
        let _ = dashboard.hide();
    }

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn show_main_window(app: tauri::AppHandle, title: Option<&str>) -> Result<(), String> {
    // hide dashboard window if it is visible
    if let Some(dashboard) = app.get_webview_window(DASHBOARD_WINDOW_NAME) {
        let _ = dashboard.hide();
    }

    let window = app
        .get_webview_window(MAIN_WINDOW_NAME)
        .ok_or("Main window not found")
        .unwrap();
    if let Some(title) = title {
        let _ = window.set_title(format!("QuickBuild Tray Monitor - {}", title).as_str());
    }
    let _ = window.move_window(Position::TrayCenter);
    let _ = window.show();
    let _ = window.set_focus();

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn close_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(main) = app.get_webview_window(MAIN_WINDOW_NAME) {
        let _ = main.hide();
    }

    Ok(())
}
