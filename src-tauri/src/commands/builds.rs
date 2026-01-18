use std::sync::Mutex;

use serde::Serialize;
use specta::Type;
use tauri::State;

use crate::{AppState, types::build::Build};

#[derive(Serialize, Type, Debug, Clone)]
pub struct GetBuildsResponse {
    pub builds: Vec<Build>,
    pub error: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn get_builds(state: State<'_, Mutex<AppState>>) -> Result<GetBuildsResponse, String> {
    let mut builds = state.lock().unwrap().get_builds();
    builds.reverse();

    Ok(GetBuildsResponse {
        builds,
        error: state.lock().unwrap().server_error.clone(),
    })
}
