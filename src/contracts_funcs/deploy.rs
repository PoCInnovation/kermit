use anyhow::{Context, Result, bail};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Value, json};

use crate::{
    account::{account::Account, address::Address, signature::PrivateKey},
    config::config::Network,
    contracts::NetworkType,
    contracts_funcs::{
        compile_project::compile_project::{CompiledContract, FieldsVec},
        deploy_bytecode::build_bytecode_contract,
    },
    transactions::submit,
    utils::{HttpResponse, get, post},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ChainParams {
    network_id: u8,
    num_zeros_at_least_in_hash: u32,
    group_num_per_broker: u32,
    groups: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DeployedContract {
    pub contract_id: String,
    pub from_group: i32,
    pub to_group: i32,
}

async fn validate_chain_params(
    network_id: u8,
    groups: &[u8],
    chain_params: ChainParams,
) -> Result<()> {
    if chain_params.network_id != network_id {
        bail!(
            "The node chain id {} is different from configured chain id {}",
            chain_params.network_id,
            network_id
        );
    }

    let mut seen = std::collections::HashSet::new();
    if groups.iter().any(|group| !seen.insert(group)) {
        bail!("Found duplicated groups in: {:?}", groups);
    }

    if groups.len() > chain_params.groups as usize {
        bail!(
            "The number of group cannot larger than {}",
            chain_params.groups
        );
    }

    if groups
        .iter()
        .any(|&group| group >= chain_params.groups as u8)
    {
        bail!(
            "Group indexes should be subset of {:?}",
            (0..chain_params.groups).collect::<Vec<_>>()
        );
    }

    Ok(())
}

////////////////////////////////////////

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildTransactionResponse {
    #[serde(rename = "contractAddress")]
    contract_id: String,
    tx_id: String,
    unsigned_tx: String,
    gas_price: String,
}

async fn build<T: DeserializeOwned>(
    url: &str,
    public_key: &str,
    address: &str,
    bytecode: &str,
    issue_token_amount: u64,
) -> Result<Option<HttpResponse<T>>> {
    post(
        url,
        "/contracts/unsigned-tx/deploy-contract",
        json!({
            "fromPublicKey": public_key,
            "fromPublicKeyType": "default",
            "signerAddress": address,
            "signerKeyType": "default",
            "bytecode": bytecode,
            "issueTokenAmount": issue_token_amount
        }),
    )
    .await
}

async fn send_contract_tx(
    url: &str,
    private_key: &Box<dyn PrivateKey>,
    address: &Address,
    bytecode: &str,
    issue_token_amount: u64,
) -> Result<Value> {
    let public_key = private_key.get_public_key()?;
    let build_result = build(url, &public_key, &address.key, bytecode, issue_token_amount).await;
    let BuildTransactionResponse {
        contract_id,
        tx_id,
        unsigned_tx,
        gas_price: _gas_price,
    } = match build_result {
        Ok(response) => response.context("Empty reply")?.data,
        Err(e) => {
            bail!("Error building contract transaction: {:?}", e);
        },
    };

    let signature = private_key.sign(&tx_id)?;
    let mut tx_res = submit(url, &unsigned_tx, &signature)
        .await?
        .context("Empty reply")?
        .data;

    // represents the DeployedContract structure
    if let Some(obj) = tx_res.as_object_mut() {
        obj.insert("contractId".to_string(), Value::String(contract_id));
    }
    Ok(tx_res)
}

////////////////////////////////////////

pub async fn deploy_contract(
    url: &str,
    private_key: Box<dyn PrivateKey>,
    network: &Network,
    network_id: NetworkType,
    contract: &CompiledContract,
    init_fields: FieldsVec,
) -> Result<Value> {
    let account = Account::new(private_key)?;
    let chain_params = get::<ChainParams>(url, "/infos/chain-params")
        .await?
        .context("Empty reply")?
        .data;

    validate_chain_params(network_id as u8, &vec![account.group], chain_params).await?;

    let bytecode = build_bytecode_contract(&contract, init_fields, network_id == NetworkType::Dev)?;

    Ok(send_contract_tx(
        url,
        &account.private_key,
        &account.address,
        &bytecode,
        network.settings.issue_token_amount.clone(),
    )
    .await?)
}
