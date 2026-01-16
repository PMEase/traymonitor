use std::sync::Mutex;

use tauri::State;

use crate::{AppState, types::build::Build};

#[tauri::command]
#[specta::specta]
pub async fn get_builds(state: State<'_, Mutex<AppState>>) -> Result<Vec<Build>, String> {
    let mut builds = state.lock().unwrap().get_builds();
    builds.reverse();
    Ok(builds)
    // tracing::info!("Getting builds since id: {:?}", last_notified_build_id);
    // let settings = AppSettings::get(&app).unwrap_or_default();
    // let quickbuild_service = QuickBuildService::builder()
    //     .user(settings.user)
    //     .token(settings.token)
    //     .host(settings.server_url)
    //     .build();

    // let builds = quickbuild_service
    //     .get_builds(last_notified_build_id)
    //     .await
    //     .map_err(|e| format!("Failed to get builds: {e}"))?;

    // tracing::debug!("Builds loaded successfully: {:?}", builds);

    // Ok(builds)
}
