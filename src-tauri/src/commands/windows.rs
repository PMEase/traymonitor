use tauri::Manager;
#[cfg(target_os = "linux")]
use tauri::WebviewWindow;
use tauri_plugin_positioner::{Position, WindowExt};

use crate::constants::{DASHBOARD_WINDOW_NAME, MAIN_WINDOW_NAME};

/// Force window focus on Linux using the "always on top" trick
/// This temporarily sets the window to always on top, then removes it
#[cfg(target_os = "linux")]
fn force_focus_linux(window: &WebviewWindow) {
    // Trick 1: Always on top toggle - forces window manager to bring window to front
    let _ = window.set_always_on_top(true);
    let _ = window.set_always_on_top(false);

    // Trick 2: Unminimize in case window is minimized
    let _ = window.unminimize();

    // Trick 3: Set focus after the tricks
    let _ = window.set_focus();
}

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

    #[cfg(not(target_os = "linux"))]
    {
        let _ = window.move_window(Position::TrayCenter);
        let _ = window.set_focus();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = window.move_window(Position::Center);
        force_focus_linux(&window);
    }

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

    #[cfg(not(target_os = "linux"))]
    {
        let _ = window.move_window(Position::TrayCenter);
        let _ = window.set_focus();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = window.move_window(Position::Center);
        force_focus_linux(&window);
    }

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
