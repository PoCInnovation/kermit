use anyhow::Result;
use serde_json::Value;

use crate::utils::{HttpResponse, get};

pub async fn is_network_alive(url: &str) -> Result<bool> {
    let res: HttpResponse<Value> = get(url, "/infos/version").await?;
    Ok(res.status == 200)
}
