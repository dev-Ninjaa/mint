use serde::Deserialize;
use reqwest::Client;
use anyhow::Result;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct PyPiResponse {
    #[allow(dead_code)]
    pub info: Value,
    pub releases: Value,
}

impl PyPiResponse {
    pub fn releases(&self) -> Option<&serde_json::Map<String, Value>> {
        self.releases.as_object()
    }
}

pub async fn fetch_package_metadata(client: &Client, package: &str) -> Result<PyPiResponse> {
    let url = format!("https://pypi.org/pypi/{}/json", package);
    let resp: PyPiResponse = client.get(&url).send().await?.json().await?;
    println!("✅ Fetched metadata for {}", package);
    Ok(resp)
}
