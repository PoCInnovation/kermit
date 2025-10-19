use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::{
    common::fs::{read_file, write_file},
    config::config_contracts::ConfigContract,
};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub contracts: Option<HashMap<String, ConfigContract>>,
}

impl Config {
    pub fn new(config_path: &str, create_if_not_exist: bool) -> Result<Self> {
        let path = Path::new(config_path);
        let path_str = path.to_str().context("Invalid config file path")?;

        // Create default "alephium.config.yaml" if it doesn't exist
        if create_if_not_exist && !path.exists() {
            let default_config = Self::default();
            let yaml = serde_yaml::to_string(&default_config)
                .map_err(|e| anyhow!("Failed to serialize default config: {e}"))?;

            println!(
                "Contract config file not found at {config_path}. Creating default config file at {path_str} ..."
            );

            write_file(path_str, &yaml)?;

            return Ok(default_config);
        }

        let config_content = read_file(path_str)?;

        let mut config = serde_yaml::from_str::<serde_yaml::Value>(&config_content)
            .map_err(|e| anyhow!("Failed to parse config file: {config_path} : {e}"))?;

        config.apply_merge()?;

        let config = serde_yaml::from_value::<Self>(config)
            .map_err(|e| anyhow!("Failed to parse config file: {config_path} : {e}"))?;

        Ok(config)
    }
}
