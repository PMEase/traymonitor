// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_app_lib::{logger, path};

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    #[cfg(feature = "portable")]
    println!("Starting Tray Monitor in PORTABLE mode");

    #[cfg(not(feature = "portable"))]
    println!("Starting Tray Monitor in STANDARD mode");

    let log_dir = match path::logs_dir() {
        Ok(dir) => {
            println!("Log directory: {}", dir.display());
            dir
        }
        Err(e) => {
            eprintln!("Failed to create or access log directory: {}", e);
            println!("Starting Tray Monitor without logging ...");
            return tauri_app_lib::run();
        }
    };

    let _log_guard = match logger::setup(&log_dir) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Failed to setup logging: {}", e);
            println!("Starting Tray Monitor without logging...");
            return tauri_app_lib::run();
        }
    };

    #[cfg(feature = "portable")]
    {
        logger::info!(
            "TrayMonitor Desktop v{} starting in PORTABLE mode",
            env!("CARGO_PKG_VERSION")
        );
        if let Ok(config_dir) = path::config_dir() {
            logger::info!("Config directory (portable): {}", config_dir.display());
        }
        if let Ok(current_dir) = std::env::current_dir() {
            logger::info!("Current working directory: {}", current_dir.display());
        }
    }

    #[cfg(not(feature = "portable"))]
    {
        logger::info!(
            "TrayMonitor Desktop v{} starting in STANDARD mode",
            env!("CARGO_PKG_VERSION")
        );
        if let Ok(config_dir) = path::config_dir() {
            logger::info!("Config directory (standard): {}", config_dir.display());
        }
    }

    tauri_app_lib::run()
}
