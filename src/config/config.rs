use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::{
    config::config_contracts::ConfigContract,
    common::fs::{read_file, write_file},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub contracts: Option<HashMap<String, ConfigContract>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
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

            println!("Contract config file not found at {}. Creating default config file at {} ...", config_path, path_str);

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
