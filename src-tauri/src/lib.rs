use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

use crate::constants::{DASHBOARD_WINDOW_NAME, MAIN_WINDOW_NAME};

mod bindings;
mod commands;
mod constants;
mod logger;
mod path;
mod settings;
mod tray;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let _log_guard = setup_logging();

    let specta_builder = bindings::generate_bindings();

    // Export TypeScript bindings in debug builds
    #[cfg(debug_assertions)]
    bindings::export_ts_bindings(&specta_builder);

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, args, _cwd| {
            tracing::debug!("Single instance invoked with args: {args:?}");
        }));
    }

    // macOS: Add NSPanel plugin for native panel behavior
    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(tauri_nspanel::init());
    }

    // Window state plugin - saves/restores window position and size
    // Note: Only applies to windows listed in capabilities (main window only, not quick-pane)
    #[cfg(desktop)]
    {
        use tauri_plugin_window_state::StateFlags;

        let flags = StateFlags::POSITION | StateFlags::SIZE;
        builder = builder.plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(flags)
                .build(),
        );
    }

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_positioner::init());
    }

    builder = builder
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_notifications::init());

    builder
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--silently"]),
        ))
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            tracing::info!("🚀 Application starting up");
            let main_win = app.get_webview_window(MAIN_WINDOW_NAME).unwrap();
            let dashboard_win = app.get_webview_window(DASHBOARD_WINDOW_NAME).unwrap();

            #[cfg(not(target_os = "macos"))]
            main_win.set_always_on_top(true);

            let _ = main_win.hide();
            let _ = dashboard_win.hide();

            let app = app.handle().clone();

            specta_builder.mount_events(&app);

            // WebviewWindowBuilder::new(&app, "dashboard", WebviewUrl::default())
            //     .title("Dashboard")
            //     .inner_size(1200.0, 800.0)
            //     .resizable(true)
            //     .decorations(true)
            //     .always_on_top(false)
            //     .build()
            //     .unwrap();

            // if let Some(dashboard) = app.get_webview_window("dashboard") {
            //     let server_url = settings::server_url(&app);
            //     let url: Url = format!("{server_url}/lite")
            //         .parse()
            //         .map_err(|e| format!("Failed to parse URL: {e}"))?;
            //     tracing::info!("Navigating to URL: {}", url);
            //     let _ = dashboard.hide();
            //     let _ = dashboard.set_title("QuickBuild Dashboard");
            //     let _ = dashboard
            //         // .eval(format!("window.location.href = '{}';", url))
            //         .navigate(url)
            //         .map_err(|e| format!("Failed to navigate to URL: {e}"));
            //     let _ = dashboard.show();
            // }

            tray::create_tray(&app)?;

            // NOTE: always force settings window to be a certain size
            // settings.set_size(LogicalSize {
            //     width: SETTINGS_WINDOW_WIDTH,
            //     height: SETTINGS_WINDOW_HEIGHT,
            // })?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app, event| {
            if let tauri::RunEvent::WindowEvent {
                event: tauri::WindowEvent::CloseRequested { api, .. },
                label,
                ..
            } = event
            {
                let win = app.get_webview_window(label.as_str()).unwrap();
                win.hide().unwrap();
                api.prevent_close();
            }
        });

    tracing::info!("Application started successfully!");
}

fn setup_logging() -> Option<logger::LogGuard> {
    let log_dir = match path::logs_dir() {
        Ok(dir) => {
            println!("Log directory: {}", dir.display());
            Some(dir)
        }
        Err(e) => {
            eprintln!("Failed to create or access log directory: {}", e);
            println!("Starting Tray Monitor without logging ...");
            None
        }
    };

    if let Some(log_dir) = log_dir {
        match logger::setup(&log_dir) {
            Ok(g) => Some(g),
            Err(e) => {
                eprintln!("Failed to setup logging: {}", e);
                println!("Starting Tray Monitor without logging...");
                None
            }
        }
    } else {
        None
    }
}
