use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegistryProviderResponse {
    pub namespace: String,
    pub name: String,
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

    pub async fn do_requests(&self, sources: Vec<String>) -> Result<Vec<RegistryProviderResponse>> {
        let mut result = Vec::new();

        for source in sources.iter() {
            let item = self.do_request(source).await?;
            result.push(item);
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
