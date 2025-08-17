use std::collections::HashMap;

use serde::de::{self, Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::account::address::Address;
use serde_yaml::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub address: Option<String>,
    pub atto_alph_amount: String,
    pub tokens: Option<Vec<Token>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: String,
    pub amount: String,
}

#[derive(Debug, Clone)]
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
            Value::String(s) => HelperFieldType::String(s),
            Value::Bool(b) => HelperFieldType::String(b.to_string()),
            Value::Number(n) => HelperFieldType::String(n.to_string()),
            Value::Mapping(s) => {
                let map: Result<HashMap<_, _>, _> = s
                    .into_iter()
                    .map(|(k, v)| {
                        let keye = k
                            .as_str()
                            .ok_or_else(|| Error::custom("Key is not a string"))?;
                        let zabi = HelperFieldType::deserialize(v)
                            .map_err(|_| Error::custom("Failed to deserialize value"))?;
                        Ok((keye.to_string(), zabi))
                    })
                    .collect();
                HelperFieldType::Structure(map?)
            },
            Value::Sequence(seq) => {
                let arr: Result<Vec<_>, _> = seq
                    .into_iter()
                    .map(|v| HelperFieldType::deserialize(v).map_err(Error::custom))
                    .collect();
                HelperFieldType::Array(arr?)
            },
            _ => return Err(de::Error::custom("unsupported type for HelperFieldType")),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigContract {
    pub initial_fields: HashMap<String, HelperFieldType>, // can be anything
    pub input_assets: Vec<Asset>,
}
