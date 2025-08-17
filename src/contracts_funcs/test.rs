use ::futures::future::try_join_all;
use anyhow::{Context, Result};
use serde_json::{Value, json};

use crate::{
    config::config_contracts::{Asset, InputAsset},
    contracts_funcs::{
        compile_project::compile_project::{CompiledContract, FieldsVec, args_to_fields_vec},
        deploy_bytecode::{get_fields_bytecode, get_fields_vec},
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
    contract_id: &str,
    contract: &CompiledContract,
    init_fields: FieldsVec,
    init_assets: &Asset,
    input_assets: &Vec<InputAsset>,
    args: Vec<(String, String)>,
    existing_contracts: Vec<&str>,
) -> Result<Value> {
    let (method, method_index) = contract.get_method(method_name)?;

    let args = args_to_fields_vec(args, &method.params_types)?;

    let existing_contracts = get_contract_states(url, existing_contracts, contract).await?;

    let fields = get_fields_vec(contract, init_fields)?;
    let (_, immutables, mutables) = get_fields_bytecode(contract, fields)?;

    let body = json!({
        "address": contract_id,
        "bytecode": contract.bytecode,
        "initialImmFields": immutables.iter().map(|(v, _)| v).collect::<Vec<_>>(),
        "initialMutFields": mutables.iter().map(|(v, _)| v).collect::<Vec<_>>(),
        "initialAsset": init_assets,
        "inputAssets": input_assets,
        "methodIndex": method_index,
        "args": args,
        "existingContracts": existing_contracts
    });

    Ok(post(url, "/contracts/test-contract", body).await?.data)
}
