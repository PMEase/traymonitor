use tauri::AppHandle;

#[tauri::command]
#[specta::specta]
pub fn get_app_info(app: AppHandle) -> Result<(String, String), String> {
    let pkg = app.package_info();
    Ok((pkg.name.clone(), pkg.version.to_string()))
}
