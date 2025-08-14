use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{contracts::NetworkType, contracts_funcs::{config::Config, state::{state, ContractState}}, network::health::is_network_alive};

pub async fn test_contract(
    url: &str,
    config: &Config,
    method_name: &str,
    contract_id: &str,
    existing_contracts: Vec<&str>
) -> Result<Value> {

    let state: ContractState = serde_json::from_value(state(url, contract_id).await?)?;

    todo!()
}
