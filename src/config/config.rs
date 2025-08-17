use std::collections::HashMap;

use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::{config::config_contracts::ConfigContract, utils::fs::read_file};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub node_url: String,
    pub private_keys: Option<Vec<String>>,
    pub settings: DefaultSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Networks {
    pub devnet: Network,
    pub testnet: Network,
    pub mainnet: Network,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultSettings {
    pub issue_token_amount: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub networks: Networks,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub configuration: Configuration,
    pub contracts: Vec<HashMap<String, ConfigContract>>, 
}

impl Config {
    pub fn new(config_path: &str) -> Result<Self> {
        let config_content = read_file(config_path)?;

        let config = serde_yaml::from_str::<serde_yaml::Value>(&config_content);
        if config.is_err() {
            return Err(anyhow!(
                "Failed to parse config file: {} : {}",
                config_path,
                config.unwrap_err()
            ));
        }

        let mut config = config?;
        config.apply_merge()?;

        let config = serde_yaml::from_value::<Config>(config);
        if config.is_err() {
            return Err(anyhow!(
                "Failed to parse config file: {} : {}",
                config_path,
                config.as_ref().unwrap_err()
            ));
        }

        Ok(config?)
    }
}
