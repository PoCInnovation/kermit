use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    config::config_contracts::Asset,
    contracts_funcs::compile_project::compile_project_values::RalphValue, utils::get,
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
    Ok(get::<Value>(url, endpoint.as_str()).await?.data)
}
