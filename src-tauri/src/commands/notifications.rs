use std::sync::Mutex;

use tauri::{AppHandle, Manager};
use tauri_plugin_notifications::NotificationsExt;

use crate::{constants::DASHBOARD_WINDOW_NAME, constants::MAIN_WINDOW_NAME, AppState};

/// Sends a native system notification.
/// On mobile platforms, returns an error as notifications are not yet supported.
#[tauri::command]
#[specta::specta]
pub async fn send_native_notification(
    app: AppHandle,
    title: String,
    body: Option<String>,
) -> Result<(), String> {
    let state = app.state::<Mutex<AppState>>();
    let settings = &state.lock().unwrap().settings;
    let enable_notifications = settings.enable_notifications;
    if !enable_notifications {
        tracing::info!("Notifications are disabled");
        return Ok(());
    }

    tracing::info!("Sending native notification: {title}");

    // Check if any app window is visible - if so, add sound to force notification display on macOS
    // This is needed because macOS doesn't show banner notifications when the app is in foreground
    let any_window_visible = app
        .get_webview_window(DASHBOARD_WINDOW_NAME)
        .and_then(|win| win.is_visible().ok())
        .unwrap_or(false)
        || app
            .get_webview_window(MAIN_WINDOW_NAME)
            .and_then(|win| win.is_visible().ok())
            .unwrap_or(false);

    let mut builder = app.notifications().builder().title(&title);

    if let Some(body_text) = body {
        builder = builder.body(body_text);
    }

    // On macOS, when app is in foreground (any window visible), add sound to force notification display
    #[cfg(target_os = "macos")]
    if any_window_visible {
        builder = builder.sound("default");
    }

    match builder.show() {
        Ok(_) => {
            tracing::debug!("Native notification sent successfully");
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to send native notification: {e}");
            Err(format!("Failed to send notification: {e}"))
        }
    }
}
