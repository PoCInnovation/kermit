use anyhow::{Context, Result};
use serde_json::{Value, json};

use crate::{
    account::address::Address,
    common::post,
    config::config_contracts::InputAsset,
    contracts_funcs::compile_project::compile_project_structs::{
        CompiledContract, args_to_fields_vec,
    },
};

pub async fn call_contract(
    url: &str,
    method_name: &str,
    contract_info: (&CompiledContract, &str, Vec<String>),
    address: &Address,
    input_assets: &Vec<InputAsset>,
    args: Vec<(String, String)>,
    block_hash: Option<String>,
) -> Result<Value> {
    let (contract, contract_id, interested_contracts) = contract_info;
    let (method, method_index) = contract.get_method(method_name)?;

    let args = args_to_fields_vec(args, &method.params_types)?;

    let body = json!({
        "group": address.group_from_bytes(),
        "address": contract_id,
        "inputAssets": input_assets,
        "methodIndex": method_index,
        "args": args,
        "interestedContracts": interested_contracts,
        "worldStateBlockHash": block_hash
    });

    post(url, "/contracts/call-contract", body)
        .await?
        .context("Empty reply")
}
