use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    contracts::NetworkType,
    contracts_funcs::{config::Config, project::Project},
    network::health::is_network_alive,
    utils::fs::read_file,
};

#[derive(Debug, Deserialize)]
struct ChainParams {
    network_id: u8,
    num_zeros_at_least_in_hash: u8,
    group_num_per_broker: u8,
    groups: u8,
}

pub async fn deploy(url: &str, network: NetworkType, config_path: &str, project_path: &str) -> Result<Value> {
    let config = Config::new(config_path)?;
    let network_url = match network {
        NetworkType::Dev => &config.configuration.networks.devnet.node_url,
        NetworkType::Test => &config.configuration.networks.testnet.node_url,
        NetworkType::Main => &config.configuration.networks.mainnet.node_url,
    };

    if !is_network_alive(&network_url).await? {
        return Err(anyhow::anyhow!("Network is not reachable: {}", network_url));
    }

    let project = Project::from_json(&read_file(project_path)?)?;

    todo!()
}
