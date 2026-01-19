use std::collections::HashMap;

use bon::bon;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;

use crate::{
    constants::TRAY_MONITOR_NOTIFICATION_TYPE,
    types::{alert::Alert, build::Build},
};

pub struct QuickBuildClient {
    user: String,
    token: String,
    host: String,
    client: Client,
}

#[bon]
impl QuickBuildClient {
    #[builder]
    pub fn new(user: String, token: String, host: String) -> Self {
        Self {
            user,
            token,
            host,
            client: Client::new(),
        }
    }

    /// Generic GET method that deserializes JSON response to any type that implements Deserialize
    /// Supports both complex types (structs, Vec, etc.) and primitive types (String, i64, f64, etc.)
    async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        query: Vec<(&str, String)>,
    ) -> Result<T, String> {
        let text = self.get_raw(url, query).await?;
        let result = serde_json::from_str::<T>(&text)
            .map_err(|e| format!("Failed to deserialize response: {e}"))?;
        Ok(result)
    }

    /// Get raw response as String (for cases where you need the raw text)
    async fn get_raw(&self, url: &str, query: Vec<(&str, String)>) -> Result<String, String> {
        let full_url = format!("{}/{}", self.host, url);
        tracing::debug!("Getting raw data from {}", full_url);

        let mut builder = self
            .client
            .get(&full_url)
            .basic_auth(self.user.as_str(), Some(self.token.as_str()))
            .header("Accept", "application/json");
        builder = builder.query(&query);

        let response = builder.send().await.map_err(|e| e.to_string())?;
        match response.error_for_status() {
            Ok(r) => {
                let text = r.text().await.map_err(|e| e.to_string())?;
                Ok(text)
            }
            Err(e) => Err(format!(
                "Failed to get data: {e}, status: {}",
                e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            )),
        }
    }

    pub async fn get_builds(
        &self,
        last_notified_build_id: Option<i64>,
    ) -> Result<Vec<Build>, String> {
        let mut queries = Vec::<(&str, String)>::new();
        if let Some(last_notified_build_id) = last_notified_build_id {
            queries.push(("last_notified_build_id", last_notified_build_id.to_string()));
        }
        let mut builds: Vec<Build> = self.get("rest/notifications", queries).await?;
        let mut configuration_path_map = HashMap::<i64, String>::new(); // configuration id -> configuration path
        let mut requester_name_map = HashMap::<i64, String>::new(); // requester id -> requester name

        // update configuration path and requester name
        for build in &mut builds {
            let path: String = if let std::collections::hash_map::Entry::Vacant(e) =
                configuration_path_map.entry(build.configuration)
            {
                let path = self.get_configuration_path(build.configuration).await?;
                e.insert(path.clone());
                path.clone()
            } else {
                configuration_path_map
                    .get(&build.configuration)
                    .unwrap()
                    .clone()
            };
            build.configuration_path = path;
            let requester_name = if let std::collections::hash_map::Entry::Vacant(e) =
                requester_name_map.entry(build.requester)
            {
                let name = self.get_user_diplay_name(build.requester).await?;
                e.insert(name.clone());
                name.clone()
            } else {
                requester_name_map.get(&build.requester).unwrap().clone()
            };
            build.requester_name = Some(requester_name);
        }
        Ok(builds)
    }

    pub async fn get_alerts(&self, last_notified_time: Option<i64>) -> Result<Vec<Alert>, String> {
        let mut queries = Vec::<(&str, String)>::new();
        queries.push(("notifier_type", TRAY_MONITOR_NOTIFICATION_TYPE.to_string()));
        if let Some(last_notified_time) = last_notified_time
            && last_notified_time > 0
        {
            queries.push(("last_notified_time", last_notified_time.to_string()));
        }

        let alerts: Vec<Alert> = self.get("rest/notifications/alerts", queries).await?;
        Ok(alerts)
    }

    async fn get_configuration_path(&self, id: i64) -> Result<String, String> {
        self.get_raw(&format!("rest/configurations/{id}/path"), vec![])
            .await
    }

    async fn get_user_diplay_name(&self, id: i64) -> Result<String, String> {
        self.get_raw(&format!("rest/users/{id}/display_name"), vec![])
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_builds() {
        let service = QuickBuildClient::builder()
            .host("http://localhost:8810".to_string())
            .user("admin".to_string())
            .token("admin".to_string())
            .build();
        let builds = service.get_builds(Some(12)).await.unwrap();
        println!("{}", serde_json::to_string_pretty(&builds).unwrap());
    }

    #[tokio::test]
    async fn test_get_alerts() {
        let service = QuickBuildClient::builder()
            .host("http://localhost:8810".to_string())
            .user("admin".to_string())
            .token("admin".to_string())
            .build();
        let alerts = service.get_alerts(None).await.unwrap();
        println!("{}", serde_json::to_string_pretty(&alerts).unwrap());
    }
}
