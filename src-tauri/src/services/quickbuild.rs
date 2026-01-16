use std::collections::HashMap;

use bon::bon;
use reqwest::Client;

use crate::types::build::Build;

pub struct QuickBuildService {
    user: String,
    token: String,
    host: String,
    client: Client,
}

#[bon]
impl QuickBuildService {
    #[builder]
    pub fn new(user: String, token: String, host: String) -> Self {
        Self {
            user,
            token,
            host,
            client: Client::new(),
        }
    }

    pub async fn get_builds(
        &self,
        last_notified_build_id: Option<i64>,
    ) -> anyhow::Result<Vec<Build>> {
        let mut builder = self
            .client
            .get(format!("{}/rest/notifications", self.host))
            .basic_auth(self.user.as_str(), Some(self.token.as_str()))
            .header("Accept", "application/json");
        if let Some(build_id) = last_notified_build_id {
            builder = builder.query(&[("last_notified_build_id", build_id.to_string())]);
        }

        let response = builder.send().await?;
        let mut builds = response.json::<Vec<Build>>().await?;
        let mut map = HashMap::<i64, String>::new(); // configuration id -> configuration path
        for build in &mut builds {
            let path: String = if let std::collections::hash_map::Entry::Vacant(e) =
                map.entry(build.configuration)
            {
                let path = self.get_configuration_path(build.configuration).await?;
                e.insert(path.clone());
                path.clone()
            } else {
                map.get(&build.configuration).unwrap().clone()
            };
            build.configuration_path = path
        }

        Ok(builds)
    }

    async fn get_configuration_path(&self, id: i64) -> anyhow::Result<String> {
        let response = self
            .client
            .get(format!("{}/rest/configurations/{}/path", self.host, id))
            .basic_auth(self.user.as_str(), Some(self.token.as_str()))
            .send()
            .await?;
        let path = response.text().await?;
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_builds() {
        let service = QuickBuildService::builder()
            .host("http://localhost:8810".to_string())
            .user("admin".to_string())
            .token("admin".to_string())
            .build();
        let builds = service.get_builds(Some(12)).await.unwrap();
        println!("{}", serde_json::to_string_pretty(&builds).unwrap());
    }
}
