use crate::{
    constants::{BUILD_STORE_FILE_NAME, MAX_STORE_ROWS},
    path,
    types::build::Build,
};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::path::PathBuf;

/// Build store structure
#[derive(Debug, Clone)]
pub struct BuildStore {
    builds: AllocRingBuffer<Build>,
}

impl Default for BuildStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildStore {
    pub fn new() -> Self {
        Self {
            builds: AllocRingBuffer::new(MAX_STORE_ROWS),
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
            .map(|dir| dir.join(BUILD_STORE_FILE_NAME))
            .map_err(|e| format!("Failed to get config directory: {e}"))
    }

    pub fn get_last_notified_build_id(&self) -> Option<i64> {
        self.builds.back().map(|build| build.id)
    }

    pub fn load(&mut self) -> Result<(), String> {
        let store_path = Self::builds_store_path()?;

        if !store_path.exists() {
            tracing::info!("Builds store file does not exist, creating new store");
            return Ok(());
        }

        let builds: Vec<Build> = match std::fs::read_to_string(&store_path) {
            Ok(content) => match serde_json::from_str::<Vec<Build>>(&content) {
                Ok(builds) => builds,
                Err(e) => {
                    tracing::warn!("Failed to deserialize builds: {e}, creating new store");
                    Vec::new()
                }
            },
            Err(e) => {
                tracing::warn!("Failed to read builds store: {e}, creating new store");
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

pub fn create_build_store() -> Result<BuildStore, String> {
    let mut store = BuildStore::new();
    store.load()?;
    Ok(store)
}
