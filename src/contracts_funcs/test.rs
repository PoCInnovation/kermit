use anyhow::{Context, Result, anyhow};
use i256::U256;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::task::futures;
use serde::{Deserializer, Serializer};
use ::futures::future::try_join_all;

use crate::{
    account::address::Address,
    contracts::NetworkType,
    contracts_funcs::{
        compile_project::compile_project::{CompiledContract, InputFieldsMap}, config::Config, deploy_bytecode::{get_fields, get_fields_bytecode}, state::{state, Asset, ContractState}
    },
    network::health::is_network_alive, utils::post,
};

async fn get_contract_states(url: &str, existing_contracts: Vec<&str>, contract: &CompiledContract) -> Result<Vec<ContractState>> {
    let futures = existing_contracts
        .iter()
        .map(|id| async move {
            let state = state(url, id).await?;
            serde_json::from_value::<ContractState>(state)
                .context(format!("Failed to deserialize state for contract ID: {}", id))
        });
    try_join_all(futures).await
}

pub async fn test_contract(
    url: &str,
    function_name: &str,
    method_name: &str,
    address: &Address,
    contract: &CompiledContract,
    init_fields: InputFieldsMap,
    init_assets: &Vec<Asset>,
    args: InputFieldsMap,
    existing_contracts: Vec<&str>,

) -> Result<Value> {
    let method_index = contract.get_method_index(function_name, method_name)?;

    let existing_contracts = get_contract_states(url, existing_contracts, contract).await?;

    let fields = get_fields(contract, init_fields)?;
    let (bytecode, immutables, mutables) = get_fields_bytecode(contract, fields)?;
    
    let body = json!({
        "group": address.group_from_bytes(),
        "address": address,
        "bytecode": bytecode,
        "initialImmFields": immutables,
        "initialMutFields": mutables,
        "initialAsset": init_assets,
        "methodIndex": method_index,
        "args": args,
        "existingContracts": existing_contracts
    });

    Ok(post(url, "contracts/test-contract", body).await?.data)
}
