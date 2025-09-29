use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    account::address::Address, config::config_contracts::Asset, contracts_funcs::compile_project::compile_project_values::RalphValue, utils::get
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractState {
    pub address: String,
    pub bytecode: String,
    pub code_hash: String,
    pub initial_state_hash: Option<String>,
    pub imm_fields: Vec<RalphValue>,
    pub mut_fields: Vec<RalphValue>,
    pub asset: Asset,
}

pub async fn state(url: &str, contract_id: &str) -> Result<Value> {
    let endpoint = format!("/contracts/{}/state", contract_id);
    Ok(get::<Value>(url, endpoint.as_str()).await?.context("Empty reply")?.data)
}

pub async fn code(url: &str, code_hash: &str) -> Result<Value> {
    let endpoint: String = format!("/contracts/{code_hash}/code");
    Ok(get::<Value>(url, endpoint.as_str()).await?.context("Empty reply")?.data)
}

pub async fn parent(url: &str, address: &Address) -> Result<Value> {
    let address = &address.key;
    let endpoint: String = format!("/contracts/{address}/code");
    Ok(get::<Value>(url, endpoint.as_str()).await?.context("Empty reply")?.data)
}

pub async fn sub_contracts(url: &str, address: &Address, start: i32, limit: Option<i32>) -> Result<Value> {
    let address = &address.key;
    let endpoint: String = format!("/contracts/{address}/sub-contracts?start={start}");
    let endpoint = if let Some(limit) = limit {
        format!("{endpoint}&limit={limit}")
    } else {
        endpoint
    };
    Ok(get::<Value>(url, endpoint.as_str()).await?.context("Empty reply")?.data)
}

pub async fn sub_contracts_current_count(url: &str, address: &Address) -> Result<Value> {
    let address = &address.key;
    let endpoint: String = format!("/contracts/{address}/sub-contracts/current-count");
    Ok(get::<Value>(url, endpoint.as_str()).await?.context("Empty reply")?.data)
}