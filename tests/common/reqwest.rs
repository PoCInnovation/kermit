use anyhow::{Result, bail};
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};

pub async fn post<T: DeserializeOwned, U: Serialize>(
    url: &str,
    endpoint: &str,
    body: U,
) -> Result<Option<T>> {
    let client = Client::new();

    let url = format!("{url}{endpoint}");

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !res.status().is_success() {
        bail!("Error: {}", res.status());
    }

    let data = if res.content_length() > Some(0) {
        Some(res.json().await?)
    } else {
        None
    };

    Ok(data)
}
