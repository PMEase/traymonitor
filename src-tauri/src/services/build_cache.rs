use std::{path::PathBuf, sync::Mutex, time::Duration};

use ringbuffer::{AllocRingBuffer, RingBuffer};
use tauri::{AppHandle, Emitter, Manager, Wry};
use tokio::time::sleep;

use crate::{
    AppState, commands::notifications::send_native_notification, path,
    services::quickbuild::QuickBuildService, types::build::Build,
};

const MAX_CACHE_SIZE: usize = 1024;
const CACHE_FILE_NAME: &str = "builds.json";

/// Builds cache structure
#[derive(Debug, Clone)]
pub struct BuildsCache {
    builds: AllocRingBuffer<Build>,
}

impl Default for BuildsCache {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildsCache {
    pub fn new() -> Self {
        Self {
            builds: AllocRingBuffer::new(MAX_CACHE_SIZE),
        }
    }

    pub fn add_builds(&mut self, new_builds: Vec<Build>) {
        tracing::debug!("Adding {} builds to cache", new_builds.len());
        for build in new_builds {
            self.builds.enqueue(build);
        }
    }

    pub fn get_all(&self) -> Vec<Build> {
        let all: Vec<Build> = self.builds.clone().into_iter().collect();
        tracing::debug!("Getting {} builds from cache", all.len());
        all
    }

    fn builds_store_path() -> Result<PathBuf, String> {
        path::config_dir()
            .map(|dir| dir.join(CACHE_FILE_NAME))
            .map_err(|e| format!("Failed to get config directory: {e}"))
    }

    pub fn get_last_notified_build_id(&self) -> Option<i64> {
        self.builds.back().map(|build| build.id)
    }

    pub fn load(&mut self) -> Result<(), String> {
        let store_path = Self::builds_store_path()?;

        if !store_path.exists() {
            tracing::info!("Builds cache file does not exist, creating new cache");
            return Ok(());
        }

        let builds: Vec<Build> = match std::fs::read_to_string(&store_path) {
            Ok(content) => match serde_json::from_str::<Vec<Build>>(&content) {
                Ok(builds) => builds,
                Err(e) => {
                    tracing::warn!("Failed to deserialize builds: {e}, creating new cache");
                    Vec::new()
                }
            },
            Err(e) => {
                tracing::warn!("Failed to read builds store: {e}, creating new cache");
                Vec::new()
            }
        };

        self.add_builds(builds);
        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {
        let store_path = Self::builds_store_path()?;

        // Ensure parent directory exists
        if let Some(parent) = store_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create builds store directory: {e}"))?;
        }

        let content = serde_json::to_string_pretty(&self.get_all())
            .map_err(|e| format!("Failed to serialize builds: {e}"))?;

        // Write to a temporary file first, then rename (atomic operation)
        let temp_path = store_path.with_extension("tmp");

        std::fs::write(&temp_path, content).map_err(|e| {
            tracing::error!("Failed to write builds store file: {e}");
            format!("Failed to write builds store file: {e}")
        })?;

        if let Err(rename_err) = std::fs::rename(&temp_path, &store_path) {
            tracing::error!("Failed to finalize builds store file: {rename_err}");
            // Clean up the temp file to avoid leaving orphaned files on disk
            if let Err(remove_err) = std::fs::remove_file(&temp_path) {
                tracing::warn!("Failed to remove temp file after rename failure: {remove_err}");
            }
            return Err(format!(
                "Failed to finalize builds store file: {rename_err}"
            ));
        }

        tracing::debug!("Saved {} builds to builds store file", self.builds.len());
        Ok(())
    }
}

pub fn load_builds_cache() -> Result<BuildsCache, String> {
    let mut cache = BuildsCache::new();
    cache.load()?;
    Ok(cache)
}

/// Start the polling thread
pub async fn start_service(app: AppHandle<Wry>) {
    tracing::info!("Starting builds polling service");

    let state = app.state::<Mutex<AppState>>();

    loop {
        let settings = state.lock().unwrap().settings.clone();

        if !settings.is_configured() {
            tracing::debug!("QuickBuild settings not configured, skipping fetching builds");
            sleep(Duration::from_secs(10)).await;
            continue;
        }

        let last_notified_build_id = state.lock().unwrap().get_last_notified_build_id();

        // Create QuickBuild service
        let quickbuild = QuickBuildService::builder()
            .host(settings.server_url.clone())
            .user(settings.user.clone())
            .token(settings.token.clone())
            .build();

        match quickbuild.get_builds(last_notified_build_id).await {
            Ok(builds) => {
                let len = builds.len();
                tracing::debug!("Fetched {} builds successfully", len);
                if len > 0 {
                    state.lock().unwrap().add_builds(builds.clone());
                    if len == 1 {
                        let build = &builds[0];
                        let title = format!("Build {} finished {}", build.version, build.status);
                        let body = format!(
                            "Build {} (#{}) for configuration {} is finished {}",
                            build.version, build.id, build.configuration_path, build.status,
                        );
                        match send_native_notification(app.clone(), title, Some(body)).await {
                            Ok(_) => {
                                tracing::debug!("Native notification sent successfully");
                            }
                            Err(e) => {
                                tracing::error!("Failed to send native notification: {e}");
                            }
                        }
                    } else {
                        match send_native_notification(
                            app.clone(),
                            "Some new builds are available".to_string(),
                            Some(format!("{} new builds notifications", len)),
                        )
                        .await
                        {
                            Ok(_) => {
                                tracing::debug!("Native notification sent successfully");
                            }
                            Err(e) => {
                                tracing::error!("Failed to send native notification: {e}");
                            }
                        }
                    }

                    let _ = app.emit("new-builds-available", ());
                }
            }
            Err(e) => {
                tracing::error!("Failed to get builds: {e}");
            }
        }

        sleep(Duration::from_secs(settings.poll_interval_in_secs as u64)).await;
    }
}
