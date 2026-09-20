use std::collections::HashMap;

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegistryProviderResponse {
    pub version: String,
}

pub struct RegistryClient {
    client: Client,
}

impl RegistryClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn do_requests(
        &self,
        sources: Vec<String>,
    ) -> Result<HashMap<String, RegistryProviderResponse>> {
        let mut result = HashMap::new();

        for source in sources.iter() {
            let item = self.do_request(source).await?;
            result.insert(source.to_string(), item);
        }

        Ok(result)
    }

    async fn do_request(&self, source: &str) -> Result<RegistryProviderResponse> {
        let (namespace, name) = source.split_once('/').unwrap();
        let url = format!("https://registry.terraform.io/v1/providers/{namespace}/{name}");
        let result: RegistryProviderResponse = self.client.get(url).send().await?.json().await?;

        Ok(result)
    }
}
