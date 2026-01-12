use std::io;
use std::path::PathBuf;

use crate::constants::APP_ID;

fn config_dir() -> io::Result<PathBuf> {
    let path = platform_config_dir();
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    Ok(path)
}

pub fn logs_dir() -> io::Result<PathBuf> {
    let path = platform_logs_dir();
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    Ok(path)
}

// pub fn log_file_path() -> PathBuf {
//     platform_logs_dir().join(format!("{}.log", APP_NAME))
// }

#[tauri::command]
#[specta::specta]
pub fn get_config_dir() -> Result<String, String> {
    config_dir()
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn get_logs_dir() -> Result<String, String> {
    logs_dir()
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|err| err.to_string())
}

// Follows how `tauri` does this
// see: https://github.com/tauri-apps/tauri/blob/dev/crates/tauri/src/path/desktop.rs
fn platform_config_dir() -> PathBuf {
    if cfg!(feature = "portable") {
        return std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir().join(APP_ID));
    }

    #[cfg(target_os = "macos")]
    {
        use crate::constants::APP_ID;

        dirs::home_dir()
            .map(|dir| dir.join("Library/Application Support").join(APP_ID))
            .unwrap_or_else(|| std::env::temp_dir().join(APP_ID))
    }

    #[cfg(target_os = "windows")]
    {
        dirs::config_dir()
            .map(|dir| dir.join(APP_ID))
            .unwrap_or_else(|| std::env::temp_dir().join(APP_ID))
    }

    #[cfg(target_os = "linux")]
    {
        dirs::config_dir()
            .map(|dir| dir.join(APP_ID))
            .unwrap_or_else(|| std::env::temp_dir().join(APP_ID))
    }

    // Fallback for others
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        std::env::temp_dir().join(APP_ID)
    }
}

// Follows how `tauri` does this
// see: https://github.com/tauri-apps/tauri/blob/dev/crates/tauri/src/path/desktop.rs
fn platform_logs_dir() -> PathBuf {
    if cfg!(feature = "portable") {
        return std::env::current_dir()
            .unwrap_or_else(|_| std::env::temp_dir())
            .join("logs");
    }

    #[cfg(target_os = "macos")]
    {
        dirs::home_dir()
            .map(|dir| dir.join("Library/Logs").join(APP_ID))
            .unwrap_or_else(|| std::env::temp_dir().join(APP_ID).join("logs"))
    }

    // Also fallback for others
    #[cfg(not(target_os = "macos"))]
    {
        dirs::data_local_dir()
            .map(|dir| dir.join(APP_ID).join("logs"))
            .unwrap_or_else(|| std::env::temp_dir().join(APP_ID).join("logs"))
    }
}
