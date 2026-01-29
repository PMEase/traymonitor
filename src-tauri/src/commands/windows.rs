use tauri::Manager;
#[cfg(target_os = "linux")]
use tauri::WebviewWindow;
// use tauri_plugin_positioner::{Position, WindowExt};

use crate::constants::{DASHBOARD_WINDOW_NAME, MAIN_WINDOW_NAME};

#[tauri::command]
#[specta::specta]
pub fn show_dashboard_window(app: tauri::AppHandle) -> Result<(), String> {
    // hide main window if it is visible
    if let Some(main) = app.get_webview_window(MAIN_WINDOW_NAME) {
        let _ = main.hide();
    }

    let window = app
        .get_webview_window(DASHBOARD_WINDOW_NAME)
        .ok_or("Dashboard window not found")
        .map_err(|e| format!("Failed to get dashboard window: {e}"))?;

    window.show().ok();
    window.unminimize().ok();
    // #[cfg(not(target_os = "linux"))]
    // window.move_window(Position::TrayCenter).ok();
    window.set_focus().ok();

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
        .ok_or("Main window not found")?;
    if let Some(title) = title {
        let _ = window.set_title(format!("QuickBuild Tray Monitor - {}", title).as_str());
    }
    window.show().ok();
    window.unminimize().ok();
    // #[cfg(not(target_os = "linux"))]
    // window.move_window(Position::TrayCenter).ok();
    window.set_focus().ok();

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
