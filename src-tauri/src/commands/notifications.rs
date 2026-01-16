use tauri::AppHandle;
use tauri_plugin_notifications::NotificationsExt;

use crate::types::settings::is_enable_notifications;

/// Sends a native system notification.
/// On mobile platforms, returns an error as notifications are not yet supported.
#[tauri::command]
#[specta::specta]
pub async fn send_native_notification(
    app: AppHandle,
    title: String,
    body: Option<String>,
) -> Result<(), String> {
    let enable_notifications = is_enable_notifications(&app);
    if !enable_notifications {
        tracing::info!("Notifications are disabled");
        return Ok(());
    }

    tracing::info!("Sending native notification: {title}");

    let mut builder = app.notifications().builder().title(&title);

    if let Some(body_text) = body {
        builder = builder.body(body_text);
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
