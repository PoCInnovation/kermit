use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::utils::fs::read_file;

#[derive(Debug, Deserialize)]
pub struct Network {
    #[serde(rename = "nodeUrl")]
    pub node_url: String,
    #[serde(rename = "privateKeys")]
    pub private_keys: Option<Vec<String>>,
    pub settings: DefaultSettings,
}

#[derive(Debug, Deserialize)]
pub struct Networks {
    pub devnet: Network,
    pub testnet: Network,
    pub mainnet: Network,
}

#[derive(Debug, Deserialize)]
pub struct DefaultSettings {
    #[serde(rename = "issueTokenAmount")]
    pub issue_token_amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub networks: Networks,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "defaultSettings")]
    pub default_settings: DefaultSettings,
    pub configuration: Configuration,
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
