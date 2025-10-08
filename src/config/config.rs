use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::{
    config::config_contracts::ConfigContract,
    contracts::NetworkType,
    utils::fs::{read_file, write_file},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub node_url: String,
    pub private_keys: Option<Vec<String>>,
    pub settings: DefaultSettings,
}

impl Network {
    fn default(network_type: NetworkType) -> Self {
        Self {
            node_url: match network_type {
                NetworkType::Dev => "http://127.0.0.1:22973".to_string(),
                NetworkType::Test => "https://node.testnet.alephium.org".to_string(),
                NetworkType::Main => "https://node.mainnet.alephium.org".to_string(),
            },
            private_keys: match network_type {
                NetworkType::Dev => Some(vec![
                    "a642942e67258589cd2b1822c631506632db5a12aabcf413604e785300d762a5".to_string(),
                ]),
                _ => None,
            },
            settings: DefaultSettings::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Networks {
    pub devnet: Network,
    pub testnet: Network,
    pub mainnet: Network,
}

impl Default for Networks {
    fn default() -> Self {
        Self {
            devnet: Network::default(NetworkType::Dev),
            testnet: Network::default(NetworkType::Test),
            mainnet: Network::default(NetworkType::Main),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultSettings {
    pub issue_token_amount: u64,
}

impl Default for DefaultSettings {
    fn default() -> Self {
        Self {
            issue_token_amount: 100,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub networks: Networks,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            networks: Networks::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub configuration: Configuration,
    pub contracts: Option<HashMap<String, ConfigContract>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            configuration: Configuration::default(),
            contracts: None,
        }
    }
}

impl Config {
    pub fn new(config_path: &str) -> Result<Self> {
        let path = Path::new(config_path);
        let path_str = path.to_str().context("Invalid config file path")?;

        // Create default "alephium.config.yaml" if it doesn't exist
        if !path.exists() {
            let default_config = Config::default();
            let yaml = serde_yaml::to_string(&default_config)
                .map_err(|e| anyhow!("Failed to serialize default config: {}", e))?;

            write_file(path_str, &yaml)?;

            return Ok(default_config);
        }

        let config_content = read_file(path_str)?;

        let mut config = serde_yaml::from_str::<serde_yaml::Value>(&config_content)
            .map_err(|e| anyhow!("Failed to parse config file: {} : {}", config_path, e))?;

        config.apply_merge()?;

        let config = serde_yaml::from_value::<Config>(config)
            .map_err(|e| anyhow!("Failed to parse config file: {} : {}", config_path, e))?;

        Ok(config)
    }
}
