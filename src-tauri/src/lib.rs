use crate::{
    constants::{DASHBOARD_WINDOW_NAME, MAIN_WINDOW_NAME},
    services::{
        alert_store::{AlertStore, create_alert_store},
        build_store::{BuildStore, create_build_store},
        poll,
    },
    types::{alert::Alert, build::Build, settings::AppSettings},
};
use std::sync::{Arc, Mutex, RwLock};
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_autostart::MacosLauncher;
use time::OffsetDateTime;

mod bindings;
mod commands;
mod constants;
mod logger;
mod path;
mod serde;
mod services;
mod tray;
mod types;
mod utils;

pub struct AppState {
    pub settings: AppSettings,
    pub build_store: Arc<RwLock<BuildStore>>,
    pub alert_store: Arc<RwLock<AlertStore>>,
    pub build_polling_error: Option<String>,
    pub alert_polling_error: Option<String>,
    pub last_polling_time: Option<OffsetDateTime>,
}

impl AppState {
    pub fn new(settings: AppSettings, builds_cache: BuildStore, alert_store: AlertStore) -> Self {
        Self {
            settings,
            build_store: Arc::new(RwLock::new(builds_cache)),
            alert_store: Arc::new(RwLock::new(alert_store)),
            build_polling_error: None,
            alert_polling_error: None,
            last_polling_time: None,
        }
    }

    pub fn init(app: &AppHandle<Wry>) -> Result<Self, String> {
        let settings = AppSettings::get(app)?;
        let build_store = create_build_store()?;
        let alert_store = create_alert_store()?;
        Ok(Self::new(settings, build_store, alert_store))
    }

    pub fn reload_settings(&mut self, app: &AppHandle<Wry>) -> Result<(), String> {
        let settings = AppSettings::get(app)?;
        self.settings = settings;
        Ok(())
    }

    pub fn add_builds(&mut self, builds: Vec<Build>) -> Result<(), String> {
        if builds.is_empty() {
            return Ok(());
        }

        let mut builds_store = self
            .build_store
            .write()
            .map_err(|e| format!("Failed to acquire write lock for build store: {e}"))?;
        builds_store.add_builds(builds);
        builds_store
            .save()
            .map_err(|e| format!("Failed to save builds: {e}"))
    }

    pub fn clear_builds(&mut self) -> Result<(), String> {
        let mut builds_store = self
            .build_store
            .write()
            .map_err(|e| format!("Failed to acquire write lock for build store: {e}"))?;
        builds_store.clear();
        builds_store
            .save()
            .map_err(|e| format!("Failed to save builds: {e}"))
    }

    pub fn get_builds(&self) -> Vec<Build> {
        self.build_store
            .read()
            .map(|store| store.get_all())
            .unwrap_or_else(|e| {
                tracing::error!("Failed to acquire read lock for build store: {e}");
                Vec::new()
            })
    }

    pub fn get_last_notified_build_id(&self) -> Option<i64> {
        self.build_store
            .read()
            .ok()
            .and_then(|store| store.get_last_notified_build_id())
    }

    pub fn get_alerts(&self) -> Vec<Alert> {
        self.alert_store
            .read()
            .map(|store| store.get_all())
            .unwrap_or_else(|e| {
                tracing::error!("Failed to acquire read lock for alert store: {e}");
                Vec::new()
            })
    }

    pub fn get_last_notified_time(&self) -> Option<i64> {
        self.alert_store
            .read()
            .ok()
            .and_then(|store| store.get_last_notified_time())
    }

    pub fn add_alerts(&mut self, alerts: Vec<Alert>) -> Result<(), String> {
        if alerts.is_empty() {
            return Ok(());
        }

        let mut alerts_store = self
            .alert_store
            .write()
            .map_err(|e| format!("Failed to acquire write lock for alert store: {e}"))?;
        alerts_store.add_alerts(alerts);
        alerts_store
            .save()
            .map_err(|e| format!("Failed to save alerts: {e}"))
    }

    pub fn clear_alerts(&mut self) -> Result<(), String> {
        let mut alerts_store = self
            .alert_store
            .write()
            .map_err(|e| format!("Failed to acquire write lock for alert store: {e}"))?;
        alerts_store.clear();
        alerts_store
            .save()
            .map_err(|e| format!("Failed to save alerts: {e}"))
    }
}

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

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
    }

    builder = builder
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_notifications::init());

    builder
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--silently"]),
        ))
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            tracing::info!("🚀 Application starting up");

            let app_handle = app.handle().clone();

            #[cfg(target_os = "macos")]
            {
                if let Err(e) = app_handle.set_activation_policy(tauri::ActivationPolicy::Accessory) {
                    tracing::warn!("Failed to hide dock icon: {e}");
                }
            }

            specta_builder.mount_events(&app_handle);

            let state = AppState::init(&app_handle)?;
            app_handle.manage(Mutex::new(state));

            let main_win = app_handle
                .get_webview_window(MAIN_WINDOW_NAME)
                .ok_or("Main window not found")?;
            #[cfg(not(target_os = "macos"))]
            let _ = main_win.set_always_on_top(true);

            if let Err(e) = main_win.hide() {
                tracing::warn!("Failed to hide main window: {e}");
            }

            let dashboard_win = app_handle
                .get_webview_window(DASHBOARD_WINDOW_NAME)
                .ok_or("Dashboard window not found")?;
            // WebviewWindowBuilder::new(&app, "dashboard", WebviewUrl::default())
            //     .title("QuickBuild Tray Monitor - Dashboard")
            //     .inner_size(1200.0, 800.0)
            //     .resizable(true)
            //     .decorations(true)
            //     .always_on_top(false)
            //     .build()?;

            let _ = dashboard_win.hide();

            let state = app_handle.state::<Mutex<AppState>>();
            let settings = {
                let state_guard = state
                    .lock()
                    .map_err(|e| format!("Failed to acquire lock for settings: {e}"))?;
                state_guard.settings.clone()
            };

            if settings.is_configured()
                && let Err(e) = dashboard_win.navigate(settings.get_dashboard_url())
            {
                tracing::warn!("Failed to navigate dashboard window: {e}");
            }

            tray::create_tray(&app_handle)?;
            tauri::async_runtime::spawn(poll::start(app_handle.clone()));

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
                if let Some(win) = app.get_webview_window(label.as_str())
                    && let Err(e) = win.hide()
                {
                    tracing::warn!("Failed to hide window {}: {e}", label);
                }
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
