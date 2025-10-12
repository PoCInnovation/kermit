use anyhow::Result;
use serde_json::Value;

use crate::common::{get};

pub async fn is_network_alive(url: &str) -> Result<bool> {
    let res = get::<Value>(url, "/infos/version").await?;
    Ok(res.is_some())
}
