use std::collections::HashMap;

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, Error},
};
use serde_yaml::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub atto_alph_amount: String,
    pub tokens: Option<Vec<Token>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputAsset {
    pub address: String,
    pub asset: Asset,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum HelperFieldType {
    String(String),
    Array(Vec<HelperFieldType>),
    Structure(HashMap<String, HelperFieldType>),
}

impl<'de> Deserialize<'de> for HelperFieldType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(match value {
            Value::String(s) => Self::String(s),
            Value::Bool(b) => Self::String(b.to_string()),
            Value::Number(n) => Self::String(n.to_string()),
            Value::Mapping(s) => {
                let map: Result<HashMap<_, _>, _> = s
                    .into_iter()
                    .map(|(k, v)| {
                        let k = k
                            .as_str()
                            .ok_or_else(|| Error::custom("Key is not a string"))?;
                        let v = Self::deserialize(v)
                            .map_err(|_| Error::custom("Failed to deserialize value"))?;
                        Ok((k.to_string(), v))
                    })
                    .collect();
                Self::Structure(map?)
            },
            Value::Sequence(seq) => {
                let arr: Result<Vec<_>, _> = seq
                    .into_iter()
                    .map(|v| Self::deserialize(v).map_err(Error::custom))
                    .collect();
                Self::Array(arr?)
            },
            _ => return Err(de::Error::custom("unsupported type for HelperFieldType")),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigContract {
    pub initial_fields: HashMap<String, HelperFieldType>, // can be anything
    pub initial_asset: Asset,
    pub input_assets: Vec<InputAsset>,
}
