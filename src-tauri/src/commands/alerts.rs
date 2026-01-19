use std::sync::Mutex;

use serde::Serialize;
use specta::Type;
use tauri::State;
use time::OffsetDateTime;

use crate::{AppState, types::alert::Alert};

#[derive(Serialize, Type, Debug, Clone)]
pub struct GetAlertsResponse {
    pub alerts: Vec<Alert>,
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_polling_time: Option<OffsetDateTime>,
}

#[tauri::command]
#[specta::specta]
pub async fn get_alerts(state: State<'_, Mutex<AppState>>) -> Result<GetAlertsResponse, String> {
    let state = state.lock().unwrap();
    let mut alerts = state.get_alerts();
    alerts.reverse();

    Ok(GetAlertsResponse {
        alerts,
        error: state.alert_polling_error.clone(),
        last_polling_time: state.last_polling_time,
    })
}
