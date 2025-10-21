use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    common::get, config::config_contracts::Asset,
    contracts_funcs::compile_project::compile_project_values::RalphValue,
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
    let endpoint = format!("/contracts/{contract_id}/state");
    get(url, &endpoint).await?.context("Empty reply")
}

pub async fn code(url: &str, code_hash: &str) -> Result<Value> {
    let endpoint = format!("/contracts/{code_hash}/code");
    get(url, &endpoint).await?.context("Empty reply")
}

pub async fn parent(url: &str, contract_id: &str) -> Result<Value> {
    let endpoint = format!("/contracts/{contract_id}/parent");
    get(url, &endpoint).await?.context("Empty reply")
}

pub async fn sub_contracts(
    url: &str,
    contract_id: &str,
    start: i32,
    limit: Option<i32>,
) -> Result<Value> {
    let mut endpoint = format!("/contracts/{contract_id}/sub-contracts?start={start}");
    if let Some(limit) = limit {
        endpoint.push_str(&format!("&limit={limit}"));
    }

    get(url, &endpoint).await?.context("Empty reply")
}

pub async fn sub_contracts_current_count(url: &str, contract_id: &str) -> Result<Value> {
    let endpoint = format!("/contracts/{contract_id}/sub-contracts/current-count");
    get(url, &endpoint).await?.context("Empty reply")
}
