use anyhow::{bail, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize)]
struct Error {
    detail: String,
}

/// Perform a GET request to the given URL
pub async fn get<T: DeserializeOwned>(url: &str, method: &str) -> Result<T> {
    let client = Client::new();

    let url = format!("{}{}", url, method);

    println!("{}", url);

    let res = client
        .get(&url)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if !res.status().is_success() {
        let err: Error = res.json().await?;
        bail!(err.detail);
    }

    let data = res.json().await?;

    Ok(data)
}
