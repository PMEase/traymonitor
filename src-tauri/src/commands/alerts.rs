use std::sync::Mutex;

use serde::Serialize;
use specta::Type;
use tauri::State;
use time::OffsetDateTime;

use crate::{AppState, types::alert::Alert};

#[derive(Serialize, Type, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAlertsResponse {
    pub alerts: Vec<Alert>,
    pub error: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::option_four_year_iso8601"
    )]
    pub last_polling_time: Option<OffsetDateTime>,
}

#[tauri::command]
#[specta::specta]
pub async fn get_alerts(state: State<'_, Mutex<AppState>>) -> Result<GetAlertsResponse, String> {
    let state_guard = state
        .lock()
        .map_err(|e| format!("Failed to acquire lock for getting alerts: {e}"))?;

    let alerts = state_guard.get_alerts();

    Ok(GetAlertsResponse {
        alerts,
        error: state_guard.alert_polling_error.clone(),
        last_polling_time: state_guard.last_polling_time,
    })
}
