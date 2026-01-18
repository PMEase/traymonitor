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
    pub server_error: Option<String>,
    pub last_polling_time: Option<OffsetDateTime>,
}

impl AppState {
    pub fn new(settings: AppSettings, builds_cache: BuildStore, alert_store: AlertStore) -> Self {
        Self {
            settings,
            build_store: Arc::new(RwLock::new(builds_cache)),
            alert_store: Arc::new(RwLock::new(alert_store)),
            server_error: None,
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

        let mut builds_cache = self.build_store.write().unwrap();
        builds_cache.add_builds(builds);
        if let Err(e) = builds_cache.save() {
            tracing::error!("Failed to save builds: {e}");
            Err(format!("Failed to save builds: {e}"))
        } else {
            Ok(())
        }
    }

    pub fn get_builds(&self) -> Vec<Build> {
        let builds_cache = self.build_store.read().unwrap();
        builds_cache.get_all()
    }

    pub fn get_last_notified_build_id(&self) -> Option<i64> {
        let builds_store = self.build_store.read().unwrap();
        builds_store.get_last_notified_build_id()
    }

    pub fn get_alerts(&self) -> Vec<Alert> {
        let alerts_store = self.alert_store.read().unwrap();
        alerts_store.get_all()
    }

    pub fn get_last_notified_time(&self) -> Option<i64> {
        let alerts_store = self.alert_store.read().unwrap();
        alerts_store.get_last_notified_time()
    }

    pub fn add_alerts(&mut self, alerts: Vec<Alert>) -> Result<(), String> {
        let mut alerts_store = self.alert_store.write().unwrap();
        alerts_store.add_alerts(alerts);
        if let Err(e) = alerts_store.save() {
            tracing::error!("Failed to save alerts: {e}");
            Err(format!("Failed to save alerts: {e}"))
        } else {
            Ok(())
        }
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

            let app = app.handle().clone();
            let state = AppState::init(&app)?;
            app.manage(Mutex::new(state));

            let main_win = app.get_webview_window(MAIN_WINDOW_NAME).unwrap();
            #[cfg(not(target_os = "macos"))]
            main_win.set_always_on_top(true);

            let _ = main_win.hide();

            let dashboard_win = app.get_webview_window(DASHBOARD_WINDOW_NAME).unwrap();
            // WebviewWindowBuilder::new(&app, "dashboard", WebviewUrl::default())
            //     .title("QuickBuild Tray Monitor - Dashboard")
            //     .inner_size(1200.0, 800.0)
            //     .resizable(true)
            //     .decorations(true)
            //     .always_on_top(false)
            //     .build()?;

            let _ = dashboard_win.hide();

            let state = app.state::<Mutex<AppState>>();
            let settings = state.lock().unwrap().settings.clone();
            if settings.is_configured() {
                let _ = dashboard_win.navigate(settings.get_dashboard_url());
            }

            specta_builder.mount_events(&app);
            tray::create_tray(&app)?;
            tauri::async_runtime::spawn(poll::start(app.clone()));

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
