use std::collections::HashMap;

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Value, json};

use crate::{
    account::{
        account::Account,
        address::Address,
        signature::{GLSecp256k1PrivateKey, PrivateKey},
    },
    contracts::NetworkType,
    contracts_funcs::{
        compile_output::{Contract, RalphValue},
        config::Config,
        deploy_bytecode::build_bytecode_contract,
    },
    network::health::is_network_alive,
    transactions::submit,
    utils::{HttpResponse, get, post},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChainParams {
    network_id: u8,
    num_zeros_at_least_in_hash: u32,
    group_num_per_broker: u32,
    groups: u32,
}

async fn validate_chain_params(
    network_id: u8,
    groups: &[u8],
    chain_params: ChainParams,
) -> Result<()> {
    if chain_params.network_id != network_id {
        return Err(anyhow!(
            "The node chain id {} is different from configured chain id {}",
            chain_params.network_id,
            network_id
        ));
    }

    let mut seen = std::collections::HashSet::new();
    if groups.iter().any(|group| !seen.insert(group)) {
        return Err(anyhow!("Found duplicated groups in: {:?}", groups));
    }

    if groups.len() > chain_params.groups as usize {
        return Err(anyhow!(
            "The number of group cannot larger than {}",
            chain_params.groups
        ));
    }

    if groups.iter().any(|&group| group >= chain_params.groups as u8) {
        return Err(anyhow!(
            "Group indexes should be subset of {:?}",
            (0..chain_params.groups).collect::<Vec<_>>()
        ));
    }

    Ok(())
}

////////////////////////////////////////

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildTransactionResponse {
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
) -> Result<HttpResponse<T>> {
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

async fn send_tx(
    url: &str,
    private_key: &Box<dyn PrivateKey>,
    address: &Address,
    bytecode: &str,
    issue_token_amount: u64,
) -> Result<Value> {
    let public_key = private_key.get_public_key()?;
    let BuildTransactionResponse {
        contract_id: _contract_id,
        tx_id,
        unsigned_tx,
        gas_price,
    } = build(url, &public_key, &address.key, bytecode, issue_token_amount)
        .await?
        .data;

    let signature = private_key.sign(&tx_id)?;
    Ok(submit(url, &unsigned_tx, &signature, Some(gas_price))
        .await?
        .data)
}

////////////////////////////////////////

pub async fn deploy_contract(
    url: &str,
    private_key: Option<Box<dyn PrivateKey>>,
    network_id: NetworkType,
    config_path: &str,
    contract: Contract,
    init_fields: HashMap<String, RalphValue>,
) -> Result<Value> {
    let config = Config::new(config_path)?;
    let network = match &network_id {
        NetworkType::Dev => &config.configuration.networks.devnet,
        NetworkType::Test => &config.configuration.networks.testnet,
        NetworkType::Main => &config.configuration.networks.mainnet,
    };

    if !is_network_alive(&network.node_url).await? {
        return Err(anyhow!("Network is not reachable: {}", network.node_url));
    }

    let private_key = if let Some(private_key) = private_key {
        private_key
    } else {
        let private_keys = network
            .private_keys
            .as_ref()
            .ok_or_else(|| anyhow!("No private keys found in devnet configuration"))?;

        let private_key = private_keys
            .get(0)
            .context("No private keys found in devnet configuration")?;

        Box::new(GLSecp256k1PrivateKey::new(private_key)?)
    };

    let account = Account::new(private_key)?;
    let chain_params = get::<ChainParams>(url, "/infos/chain-params").await?.data;

    validate_chain_params(network_id as u8, &vec![account.group], chain_params).await?;

    let bytecode = build_bytecode_contract(&contract, init_fields, network_id == NetworkType::Dev)?;

    println!("Deploying contract with bytecode: {}", bytecode);
    todo!();
    Ok(send_tx(
        url,
        &account.private_key,
        &account.address,
        &bytecode,
        network.settings.issue_token_amount.clone(),
    )
    .await?)
}
