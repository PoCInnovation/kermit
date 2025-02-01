use anyhow::{bail, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize)]
struct Error {
    detail: String,
}

/// Perform a GET request to the given URL
pub async fn get<T: DeserializeOwned>(url: &str, endpoint: &str) -> Result<T> {
    let client = Client::new();

    let url = format!("{}{}", url, endpoint);

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

/// Perform a POST request to the given URL
pub async fn post<T: DeserializeOwned, U: Serialize>(
    url: &str,
    endpoint: &str,
    body: U,
) -> Result<T> {
    let client = Client::new();

    let url = format!("{}{}", url, endpoint);

    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !res.status().is_success() {
        let err: Error = res.json().await?;
        bail!(err.detail);
    }

    let data = res.json().await?;

    Ok(data)
}

/// Perform a PUT request to the given URL
pub async fn put<T: DeserializeOwned, U: Serialize>(
    url: &str,
    endpoint: &str,
    body: U,
) -> Result<T> {
    let client = Client::new();

    let url = format!("{}{}", url, endpoint);

    let res = client
        .put(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !res.status().is_success() {
        let err: Error = res.json().await?;
        bail!(err.detail);
    }

    let data = res.json().await?;

    Ok(data)
}

/// Perform a DELETE request to the given URL
pub async fn delete(url: &str, endpoint: &str) -> Result<()> {
    let client = Client::new();

    let url = format!("{}{}", url, endpoint);

    let res = client
        .delete(&url)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if !res.status().is_success() {
        let err: Error = res.json().await?;
        bail!(err.detail);
    }

    Ok(())
}
