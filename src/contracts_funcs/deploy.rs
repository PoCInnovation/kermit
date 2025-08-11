use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    account::{account::Account, signature::GLSecp256k1PrivateKey},
    contracts::NetworkType,
    contracts_funcs::{config::Config, project::Project},
    network::health::is_network_alive,
    utils::{fs::read_file, get},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChainParams {
    network_id: u8,
    num_zeros_at_least_in_hash: u8,
    group_num_per_broker: u8,
    groups: u8,
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

    if groups.iter().any(|&group| group >= chain_params.groups) {
        let valid_range: Vec<u8> = (0..chain_params.groups).collect();
        return Err(anyhow!(
            "Group indexes should be subset of {:?}",
            valid_range
        ));
    }

    Ok(())
}

pub async fn deploy(
    url: &str,
    network_id: NetworkType,
    config_path: &str,
    project_path: &str,
) -> Result<Value> {
    let config = Config::new(config_path)?;
    let network = match network_id {
        NetworkType::Dev => &config.configuration.networks.devnet,
        NetworkType::Test => &config.configuration.networks.testnet,
        NetworkType::Main => &config.configuration.networks.mainnet,
    };

    if !is_network_alive(&network.node_url).await? {
        return Err(anyhow!("Network is not reachable: {}", network.node_url));
    }

    let private_keys = config
        .configuration
        .networks
        .devnet
        .private_keys
        .ok_or_else(|| anyhow!("No private keys found in devnet configuration"))?;

    let private_key = private_keys
        .get(0)
        .context("No private keys found in devnet configuration")?;

    let private_key = Box::new(GLSecp256k1PrivateKey::new(private_key)?);
    let account = Account::new(private_key)?;

    let project: Project = Project::try_from(read_file(project_path)?.as_str())?;
    let chain_params = get::<ChainParams>(url, "/infos/chain-params").await?.data;

    validate_chain_params(network_id as u8, &vec![account.group], chain_params).await?;
    todo!()
}
