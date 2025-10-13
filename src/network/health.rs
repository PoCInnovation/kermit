use anyhow::{Result, bail};
use serde_json::Value;

use crate::common::get;

async fn is_network_alive(url: &str) -> Result<bool> {
    let res = get::<Value>(url, "/infos/version").await?;
    Ok(res.is_some())
}

/* See if the network is available before continuing */
pub async fn check_network(url: &str) -> Result<()> {
    if reqwest::Url::parse(&url).is_err() {
        bail!("Invalid node URL: {}", url);
    }

    if !is_network_alive(&url).await? {
        bail!("Network is not reachable: {}", url);
    }

    Ok(())
}
