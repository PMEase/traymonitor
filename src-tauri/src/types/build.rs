use serde::{Deserialize, Serialize};
use specta::Type;
use strum::{Display, EnumString};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Type, Debug, Clone, EnumString, Display)]
#[serde(rename_all = "UPPERCASE")]
pub enum BuildStatus {
    Successful,
    Recommended,
    Failed,
    Cancelled,
    Timeout,
    Running,
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub id: i64,
    pub configuration: i64,
    #[serde(default)]
    pub configuration_path: String,
    pub master_node_address: String,
    pub requester: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceller: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceller_name: Option<String>,
    pub version: String,
    pub status: BuildStatus,
    #[serde(with = "crate::serde::four_year_iso8601")]
    pub begin_date: OffsetDateTime,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::option_four_year_iso8601"
    )]
    pub status_date: Option<OffsetDateTime>,
    pub duration: i64,
    pub wait_duration: i64,
}

impl Build {
    pub fn get_subject(&self) -> String {
        match self.status {
            BuildStatus::Successful => format!("Build {} finished successfully 🎉", self.version),
            BuildStatus::Cancelled => format!("Build {} cancelled", self.version),
            BuildStatus::Failed => format!("Build {} failed", self.version),
            BuildStatus::Timeout => format!("Build {} timed out", self.version),
            BuildStatus::Recommended => format!("Build {} was recommended", self.version),
            BuildStatus::Running => format!("Build {} is running", self.version),
        }
    }

    pub fn get_body(&self) -> Result<String, String> {
        Ok(format!(
            r#"
Configuration:    {}
Triggered by:     {}
        "#,
            self.configuration_path,
            self.requester_name.as_ref().unwrap_or(&String::new()),
        ))
    }
}
