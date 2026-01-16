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
    pub version: String,
    pub status: BuildStatus,
    #[serde(with = "time::serde::iso8601")]
    pub begin_date: OffsetDateTime,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::option_time_iso8601"
    )]
    pub status_date: Option<OffsetDateTime>,
    pub duration: i64,
    pub wait_duration: i64,
}
