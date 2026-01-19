use serde::{Deserialize, Serialize};
use specta::Type;
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Type, Debug, Clone, EnumString, Display)]
#[serde(rename_all = "UPPERCASE")]
pub enum AlertPriority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Type, Debug, Clone, EnumString, Display)]
#[serde(rename_all = "UPPERCASE")]
pub enum AlertCategory {
    System,
    Metric,
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: i64,
    pub subject: String,
    pub priority: AlertPriority,
    pub category: AlertCategory,
    pub alert_message: String,
    pub trigger: String,
    pub fixed: bool,
    pub ctime: i64,
    pub ack_time: i64,
}
