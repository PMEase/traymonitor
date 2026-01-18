use std::{sync::Mutex, time::Duration};

use tauri::{AppHandle, Emitter, Manager, Wry};
use time::OffsetDateTime;
use tokio::time::sleep;

use crate::{
    AppState, commands::notifications::send_native_notification,
    services::quickbuild::QuickBuildService,
};

pub async fn start(app: AppHandle<Wry>) {
    tracing::info!("Starting scheduler service");
    let state = app.state::<Mutex<AppState>>();

    loop {
        let settings = state.lock().unwrap().settings.clone();
        if !settings.is_configured() {
            tracing::debug!("QuickBuild settings not configured, skipping fetching notifications");
            sleep(Duration::from_secs(10)).await;
            continue;
        }

        if settings.paused {
            tracing::debug!("Polling is paused, skipping fetching notifications");
            sleep(Duration::from_secs(10)).await;
            continue;
        }

        // Create QuickBuild service
        let quickbuild = QuickBuildService::builder()
            .host(settings.server_url.clone())
            .user(settings.user.clone())
            .token(settings.token.clone())
            .build();

        let old_error = state.lock().unwrap().server_error.clone();
        let mut current_error = None;
        let mut should_refresh_page = false;

        let fetch_builds_result = fetch_builds(&quickbuild, app.clone(), &state).await;
        let fetch_alerts_result = fetch_alerts(&quickbuild, app.clone(), &state).await;

        match fetch_builds_result {
            Ok(len) => {
                should_refresh_page = len > 0;
            }
            Err(e) => {
                current_error = Some(e.clone());
            }
        }

        match fetch_alerts_result {
            Ok(len) => {
                should_refresh_page = len > 0;
            }
            Err(e) => {
                current_error = Some(e.clone());
            }
        }

        if !should_refresh_page {
            should_refresh_page = old_error != current_error;
        }

        state.lock().unwrap().server_error = current_error;

        if should_refresh_page {
            app.emit("refresh-page", ()).unwrap();
        }

        state.lock().unwrap().last_polling_time = Some(OffsetDateTime::now_utc());

        sleep(Duration::from_secs(settings.poll_interval_in_secs as u64)).await;
    }
}

async fn fetch_builds(
    quickbuild: &QuickBuildService,
    app: AppHandle<Wry>,
    state: &Mutex<AppState>,
) -> Result<usize, String> {
    let last_notified_build_id = state.lock().unwrap().get_last_notified_build_id();
    match quickbuild.get_builds(last_notified_build_id).await {
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

            Ok(len)
        }
        Err(e) => {
            tracing::error!("Failed to get builds: {e}");
            Err(format!("Polling failed: {e}"))
        }
    }
}

async fn fetch_alerts(
    quickbuild: &QuickBuildService,
    app: AppHandle<Wry>,
    state: &Mutex<AppState>,
) -> Result<usize, String> {
    let last_notified_time = state.lock().unwrap().get_last_notified_time();
    tracing::info!(
        "Fetching alerts with last notified time: {:?}",
        last_notified_time
    );
    match quickbuild.get_alerts(last_notified_time).await {
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
                    let title = format!("{} new alerts are available", len);
                    let _ = send_native_notification(app.clone(), title, None).await;
                }

                let _ = state.lock().unwrap().add_alerts(alerts);
            }

            Ok(len)
        }
        Err(e) => {
            tracing::error!("Failed to get alerts: {e}");
            Err(format!("Polling failed: {e}"))
        }
    }
}
