use crate::{constants::MAX_STORE_ROWS, path, types::alert::Alert};
use ringbuffer::{AllocRingBuffer, RingBuffer};

#[derive(Debug, Clone)]
pub struct AlertStore {
    alerts: AllocRingBuffer<Alert>,
}

impl Default for AlertStore {
    fn default() -> Self {
        Self::new()
    }
}

impl AlertStore {
    pub fn new() -> Self {
        Self {
            alerts: AllocRingBuffer::new(MAX_STORE_ROWS),
        }
    }

    pub fn add_alerts(&mut self, mut new_alerts: Vec<Alert>) {
        tracing::debug!("Adding {} alerts to store", new_alerts.len());
        new_alerts.sort_by_key(|alert| alert.id);
        self.alerts.extend(new_alerts);
    }

    pub fn clear(&mut self) {
        tracing::debug!("Clearing alerts cache");
        self.alerts.clear();
    }

    pub fn get_all(&self) -> Vec<Alert> {
        let mut all: Vec<Alert> = self.alerts.clone().into_iter().collect();
        all.sort_by_key(|alert| alert.ctime);
        all.reverse();
        tracing::debug!("Getting {} alerts from store", all.len());
        all
    }

    pub fn get_last_notified_time(&self) -> Option<i64> {
        self.alerts.back().map(|alert| alert.ctime)
    }

    pub fn load(&mut self) -> Result<(), String> {
        let store_path = path::alerts_store_path()?;

        if !store_path.exists() {
            tracing::info!("Alerts store file does not exist, creating new store");
            return Ok(());
        }

        let alerts: Vec<Alert> = match std::fs::read_to_string(&store_path) {
            Ok(content) => match serde_json::from_str::<Vec<Alert>>(&content) {
                Ok(alerts) => alerts,
                Err(e) => {
                    tracing::warn!("Failed to deserialize alerts: {e}");
                    Vec::new()
                }
            },
            Err(e) => {
                tracing::warn!("Failed to read alerts store: {e}");
                Vec::new()
            }
        };

        self.alerts.clear();
        self.add_alerts(alerts);

        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {
        let store_path = path::alerts_store_path()?;
        // Ensure parent directory exists
        if let Some(parent) = store_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create builds store directory: {e}"))?;
        }

        let alerts = self.get_all();
        tracing::debug!("Saving {} alerts to alerts store file", alerts.len());

        let content = serde_json::to_string_pretty(&alerts)
            .map_err(|e| format!("Failed to serialize alerts: {e}"))?;

        // Write to a temporary file first, then rename (atomic operation)
        let temp_path = store_path.with_extension("tmp");

        std::fs::write(&temp_path, content).map_err(|e| {
            tracing::error!("Failed to write alerts store file: {e}");
            format!("Failed to write alerts store file: {e}")
        })?;

        if let Err(rename_err) = std::fs::rename(&temp_path, &store_path) {
            tracing::error!("Failed to finalize alerts store file: {rename_err}");
            if let Err(remove_err) = std::fs::remove_file(&temp_path) {
                tracing::warn!(
                    "Failed to remove alerts temp file after rename failure: {remove_err}"
                );
            }
            return Err(format!(
                "Failed to finalize alerts store file: {rename_err}"
            ));
        }

        tracing::debug!("Saved {} alerts to alerts store file", self.alerts.len());
        Ok(())
    }
}

pub fn create_alert_store() -> Result<AlertStore, String> {
    let mut store = AlertStore::new();
    store.load()?;
    Ok(store)
}
