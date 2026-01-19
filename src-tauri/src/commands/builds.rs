use std::sync::Mutex;

use serde::Serialize;
use specta::Type;
use tauri::State;
use time::OffsetDateTime;

use crate::{AppState, types::build::Build};

#[derive(Serialize, Type, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetBuildsResponse {
    pub builds: Vec<Build>,
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
pub async fn get_builds(state: State<'_, Mutex<AppState>>) -> Result<GetBuildsResponse, String> {
    let state = state.lock().unwrap();
    let mut builds = state.get_builds();
    builds.reverse();

    Ok(GetBuildsResponse {
        builds,
        error: state.build_polling_error.clone(),
        last_polling_time: state.last_polling_time,
    })
}
