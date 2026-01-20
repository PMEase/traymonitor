use std::{sync::Mutex, time::Duration};

use tauri::{AppHandle, Emitter, Manager, Wry};
use time::OffsetDateTime;
use tokio::time::sleep;

use crate::{
    AppState, commands::notifications::send_native_notification,
    services::quickbuild::QuickBuildClient, types::settings::AppSettings,
};

fn get_settings(state: &Mutex<AppState>) -> Result<AppSettings, String> {
    let state_guard = state
        .lock()
        .map_err(|e| format!("Failed to acquire lock for getting settings: {e}"))?;
    Ok(state_guard.settings.clone())
}

pub async fn start(app: AppHandle<Wry>) {
    tracing::info!("Starting scheduler service");
    let state = app.state::<Mutex<AppState>>();

    loop {
        let settings = match get_settings(&state) {
            Ok(settings) => settings,
            Err(e) => {
                tracing::error!("Failed to get settings: {e}");
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        if !settings.is_configured() {
            tracing::debug!("QuickBuild settings not configured, skipping fetching notifications");
            sleep(Duration::from_secs(10)).await;
            continue;
        }

        let poll_interval = Duration::from_secs(settings.poll_interval_in_secs as u64);
        if settings.paused {
            tracing::debug!("Polling is paused, skipping fetching notifications");
            sleep(poll_interval).await;
            continue;
        }

        // Create QuickBuild service
        let client = QuickBuildClient::builder()
            .host(settings.server_url.clone())
            .user(settings.user.clone())
            .token(settings.token.clone())
            .build();

        let _ = fetch_builds(&client, app.clone(), &state).await;
        let _ = fetch_alerts(&client, app.clone(), &state).await;

        {
            let mut state_guard = match state.lock() {
                Ok(guard) => guard,
                Err(e) => {
                    tracing::error!("Failed to acquire lock for updating last polling time: {e}");
                    continue;
                }
            };

            state_guard.last_polling_time = Some(OffsetDateTime::now_utc());
        }

        sleep(poll_interval).await;
    }
}

const POLLING_FAILED_MESSAGE: &str = "Polling failed, please check your connection and try again";

async fn fetch_builds(client: &QuickBuildClient, app: AppHandle<Wry>, state: &Mutex<AppState>) {
    let last_notified_build_id = { state.lock().unwrap().get_last_notified_build_id() };
    let old_error = { state.lock().unwrap().build_polling_error.clone() };
    let should_refresh;

    match client.get_builds(last_notified_build_id).await {
        Ok(builds) => {
            let len = builds.len();
            if len > 0 {
                tracing::debug!("{} builds fetched successfully", len);
                if len == 1 {
                    let build = &builds[0];
                    let title = build.get_subject();
                    let body = build.get_body().unwrap();
                    let _ = send_native_notification(app.clone(), title, Some(body)).await;
                } else {
                    let title = format!("{} new builds are finished", len);
                    let _ = send_native_notification(app.clone(), title, None).await;
                }

                let _ = state.lock().unwrap().add_builds(builds);
            }

            should_refresh = len > 0 || old_error.is_some();
            state.lock().unwrap().build_polling_error = None;
        }
        Err(e) => {
            tracing::error!("Failed to get builds: {e}");
            tracing::info!("Old error: {old_error:?}");
            should_refresh = old_error != Some(POLLING_FAILED_MESSAGE.to_string());
            state.lock().unwrap().build_polling_error = Some(POLLING_FAILED_MESSAGE.to_string());
        }
    }

    if should_refresh {
        tracing::info!("Emitting builds-refresh-page event");
        let _ = app.emit("builds-refresh-page", ());
    }
}

async fn fetch_alerts(client: &QuickBuildClient, app: AppHandle<Wry>, state: &Mutex<AppState>) {
    let last_notified_time = { state.lock().unwrap().get_last_notified_time() };
    let old_error = { state.lock().unwrap().alert_polling_error.clone() };
    let should_refresh;

    tracing::info!(
        "Fetching alerts with last notified time: {:?}",
        last_notified_time
    );
    match client.get_alerts(last_notified_time).await {
        Ok(alerts) => {
            let len = alerts.len();
            if len > 0 {
                tracing::debug!("{} alerts fetched successfully", len);
                if len == 1 {
                    let alert = &alerts[0];
                    let title = alert.subject.clone();
                    let body = alert.alert_message.clone();
                    let _ = send_native_notification(app.clone(), title, Some(body)).await;
                } else {
                    let alert = &alerts[0];
                    let title = format!("{} and more alerts ...", alert.subject);
                    let body = format!(
                        "{}\n\n and {} more ...",
                        alert.alert_message.clone(),
                        len - 1
                    );
                    let _ = send_native_notification(app.clone(), title, Some(body)).await;
                }

                let _ = state.lock().unwrap().add_alerts(alerts);
            }

            should_refresh = len > 0 || old_error.is_some();
            state.lock().unwrap().alert_polling_error = None;
        }
        Err(e) => {
            tracing::error!("Failed to get alerts: {e}");
            should_refresh =
                old_error.is_none() || old_error != Some(POLLING_FAILED_MESSAGE.to_string());
            state.lock().unwrap().alert_polling_error = Some(POLLING_FAILED_MESSAGE.to_string());
        }
    }

    if should_refresh {
        tracing::info!("Emitting alerts-refresh-page event");
        let _ = app.emit("alerts-refresh-page", ());
    }
}
