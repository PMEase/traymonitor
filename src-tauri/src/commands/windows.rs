use tauri::{Manager, UserAttentionType};
use tauri_plugin_positioner::{Position, WindowExt};

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
    let _ = window.show();
    let _ = window.move_window(Position::TrayCenter);

    // Request user attention to ensure proper focus on Linux
    let _ = window.request_user_attention(Some(UserAttentionType::Informational));
    let _ = window.set_focus();

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

    let _ = window.show();
    let _ = window.move_window(Position::TrayCenter);

    // Request user attention to ensure proper focus on Linux
    // This helps with window managers that don't allow apps to steal focus
    let _ = window.request_user_attention(Some(UserAttentionType::Informational));
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
