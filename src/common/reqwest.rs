use anyhow::{Result, bail};
use reqwest::Client;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Deserialize)]
struct Error {
    detail: String,
}

/// Perform a GET request to the given URL
pub async fn get<T: DeserializeOwned>(url: &str, endpoint: &str) -> Result<Option<T>> {
    let client = Client::new();
    let url = format!("{url}{endpoint}");

    let res = client
        .get(url)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        if let Ok(err) = res.json::<Error>().await {
            bail!(err.detail);
        }
        bail!(status);
    }

    let data = if res.content_length() > Some(0) {
        let data = res.json().await?;
        Some(data)
    } else {
        None
    };

    Ok(data)
}

/// Perform a POST request to the given URL
pub(crate) async fn post<T: DeserializeOwned, U: Serialize>(
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
        let status = res.status();
        if let Ok(err) = res.json::<Error>().await {
            bail!(err.detail);
        }
        bail!(status);
    }

    let data = if res.content_length() > Some(0) {
        let data = res.json().await?;
        Some(data)
    } else {
        None
    };

    Ok(data)
}

/// Perform a PUT request to the given URL
pub(crate) async fn put<T: DeserializeOwned, U: Serialize>(
    url: &str,
    endpoint: &str,
    body: U,
) -> Result<Option<T>> {
    let client = Client::new();

    let url = format!("{url}{endpoint}");

    let res = client
        .put(url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        if let Ok(err) = res.json::<Error>().await {
            bail!(err.detail);
        }
        bail!(status);
    }

    let data = if res.content_length() > Some(0) {
        let data = res.json().await?;
        Some(data)
    } else {
        None
    };

    Ok(data)
}

/// Perform a DELETE request to the given URL
pub(crate) async fn delete<T: DeserializeOwned>(url: &str, endpoint: &str) -> Result<Option<T>> {
    let client = Client::new();

    let url = format!("{url}{endpoint}");

    let res = client
        .delete(url)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        if let Ok(err) = res.json::<Error>().await {
            bail!(err.detail);
        }
        bail!(status);
    }

    let data = if res.content_length() > Some(0) {
        let data = res.json().await?;
        Some(data)
    } else {
        None
    };

    Ok(data)
}
