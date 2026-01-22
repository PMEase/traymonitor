use crate::{constants::MAX_STORE_ROWS, path, types::build::Build};
use ringbuffer::{AllocRingBuffer, RingBuffer};

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

    pub fn add_builds(&mut self, mut new_builds: Vec<Build>) {
        tracing::debug!("Adding {} builds to cache", new_builds.len());

        new_builds.sort_by_key(|build| build.id);
        self.builds.extend(new_builds);
    }

    pub fn clear(&mut self) {
        tracing::debug!("Clearing builds cache");
        self.builds.clear();
    }

    pub fn get_all(&self) -> Vec<Build> {
        let mut all: Vec<Build> = self.builds.clone().into_iter().collect();
        all.sort_by_key(|build| build.id);
        all.reverse();
        tracing::debug!("Getting {} builds from store", all.len());
        all
    }

    pub fn get_last_notified_build_id(&self) -> Option<i64> {
        self.builds.back().map(|build| build.id)
    }

    pub fn load(&mut self) -> Result<(), String> {
        let store_path = path::builds_store_path()?;

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

        self.builds.clear();
        self.add_builds(builds);

        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {
        let store_path = path::builds_store_path()?;

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
