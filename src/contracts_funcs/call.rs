use anyhow::{Result, Context};
use serde_json::{Value, json};

use crate::{
    account::address::Address,
    config::config_contracts::InputAsset,
    contracts_funcs::compile_project::compile_project::{CompiledContract, args_to_fields_vec},
    utils::post,
};

pub async fn call_contract(
    url: &str,
    method_name: &str,
    contract_id: &str,
    contract: &CompiledContract,
    address: &Address,
    input_assets: &Vec<InputAsset>,
    args: Vec<(String, String)>,
    existing_contracts_addresses: Vec<String>,
    block_hash: Option<String>,
) -> Result<Value> {
    let (method, method_index) = contract.get_method(method_name)?;

    let args = args_to_fields_vec(args, &method.params_types)?;

    let body = json!({
        "group": address.group_from_bytes(),
        "address": contract_id,
        "inputAssets": input_assets,
        "methodIndex": method_index,
        "args": args,
        "interestedContracts": existing_contracts_addresses,
        "worldStateBlockHash": block_hash
    });

    Ok(post(url, "/contracts/call-contract", body)
        .await?
        .context("Empty reply")?
        .data)
}
