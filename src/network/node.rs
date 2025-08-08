use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DefaultSettings {
    #[serde(rename = "issueTokenAmount")]
    pub issue_token_amount: u64
}

#[derive(Debug, Deserialize)]
pub struct Network {
    #[serde(rename = "nodeUrl")]
    pub node_url: String,
    #[serde(rename = "privateKeys")]
    pub private_keys: Option<Vec<String>>,
    pub settings: DefaultSettings,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "defaultSettings")]
    pub default_settings: DefaultSettings,
    pub configuration: Configuration,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub networks: Networks,
}

#[derive(Debug, Deserialize)]
pub struct Networks {
    pub devnet: Network,
    pub testnet: Network,
    pub mainnet: Network,
}

