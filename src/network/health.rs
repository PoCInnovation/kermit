use anyhow::Result;
use serde_json::Value;

use crate::utils::{HttpResponse, get};

pub async fn is_network_alive(url: &str) -> Result<bool> {
    let res = get::<Value>(url, "/infos/version").await?;
    if let Some(HttpResponse { status, .. }) = res {
        Ok(status == 200)
    } else {
        Ok(false)
    }
}
