use ::futures::future::try_join_all;
use anyhow::{Context, Result};
use serde_json::{Value, json};

use crate::{
    account::address::Address,
    config::config_contracts::Asset,
    contracts_funcs::{
        compile_project::compile_project::{CompiledContract, InputFieldsMap},
        deploy_bytecode::{get_fields, get_fields_bytecode},
        state::{ContractState, state},
    },
    utils::post,
};

async fn get_contract_states(
    url: &str,
    existing_contracts: Vec<&str>,
    contract: &CompiledContract,
) -> Result<Vec<ContractState>> {
    let futures = existing_contracts.iter().map(|id| async move {
        let state = state(url, id).await?;
        serde_json::from_value::<ContractState>(state).context(format!(
            "Failed to deserialize state for contract ID: {}",
            id
        ))
    });
    try_join_all(futures).await
}

pub async fn test_contract(
    url: &str,
    method_name: &str,
    address: &Address,
    contract: &CompiledContract,
    init_fields: InputFieldsMap,
    init_assets: &Vec<Asset>,
    args: InputFieldsMap,
    existing_contracts: Vec<&str>,
) -> Result<Value> {
    let method_index = contract.get_method_index(method_name)?;

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
