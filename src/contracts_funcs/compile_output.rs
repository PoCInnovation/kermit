use anyhow::{Context, Error, Result, anyhow};
use i256::{I256, U256};
use serde::{
    Deserialize,
    de::{self, Deserializer},
};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileProject {
    pub contracts: Vec<Contract>,
    pub scripts: Vec<Script>,
    pub structs: Option<Vec<StructDef>>,
    pub constants: Option<Vec<Constant>>,
    pub enums: Option<Vec<EnumDef>>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub version: String,
    pub name: String,
    pub bytecode: String,
    pub bytecode_debug_patch: String,
    pub code_hash: String,
    pub code_hash_debug: String,
    pub fields: Fields,
    pub functions: Vec<Function>,
    pub constants: Vec<Constant>,
    pub enums: Vec<EnumDef>,
    pub events: Vec<Event>,
    pub warnings: Vec<String>,
    pub maps: Option<Maps>,
    pub std_interface_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    pub version: String,
    pub name: String,
    pub bytecode_template: String,
    pub bytecode_debug_patch: String,
    pub fields: Fields,
    pub functions: Vec<Function>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructDef {
    pub name: String,
    pub field_names: Vec<String>,
    pub field_types: Vec<TypeName>,
    pub is_mutable: Vec<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub names: Vec<String>,
    pub types: Vec<TypeName>,
    pub is_mutable: Vec<bool>,
}

pub type FieldsTypesMap = HashMap<String, (TypeName, bool)>;
pub type FieldsMap = HashMap<String, (RalphValue, bool)>;
pub type InputFieldsMap = HashMap<String, RalphValue>;
pub type InputFieldsTypesMap = HashMap<String, TypeName>;

impl Into<FieldsTypesMap> for Fields {
    fn into(self) -> FieldsTypesMap {
        self.names
            .into_iter()
            .zip(self.types.into_iter())
            .zip(self.is_mutable.into_iter())
            .filter(|((name, _), _)| !name.starts_with("__"))
            .map(|((name, ty), is_mut)| (name, (ty, is_mut)))
            .collect()
    }
}

impl Into<InputFieldsTypesMap> for Fields {
    fn into(self) -> InputFieldsTypesMap {
        self.names
            .into_iter()
            .zip(self.types.into_iter())
            .filter(|(name, _)| !name.starts_with("__"))
            .map(|(name, ty)| (name, ty))
            .collect()
    }
}

pub fn fields_vec_to_fields_map(
    v: Vec<(String, String)>,
    types: &InputFieldsTypesMap,
) -> Result<InputFieldsMap> {
    let mut result = HashMap::new();

    if v.len() != types.len() {
        return Err(anyhow!(
            "Fields count mismatch with initial fields: expected {}, found {}",
            types.len(),
            v.len()
        ));
    }

    for (name, value) in v {
        let type_name = types.get(&name).context(anyhow!(
            "Type for field '{}' not found in provided types map",
            name
        ))?;
        let ralph_value = RalphValue::from_typename_and_value(type_name, &Value::String(value))?;
        result.insert(name, ralph_value);
    }
    Ok(result)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    pub name: String,
    pub use_preapproved_assets: bool,
    pub use_assets_in_contract: bool,
    pub is_public: bool,
    pub param_names: Vec<String>,
    pub param_types: Vec<TypeName>,
    pub param_is_mutable: Vec<bool>,
    pub return_types: Vec<TypeName>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constant {
    pub name: String,
    pub value: ConstantValue,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstantValue {
    pub type_name: TypeName,
    pub value: FieldValue,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumDef {
    pub name: String,
    pub fields: Vec<EnumField>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumField {
    pub name: String,
    pub value: FieldValue,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub name: String,
    pub field_names: Vec<String>,
    pub field_types: Vec<TypeName>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Maps {
    pub names: Vec<String>,
    pub types: Vec<TypeName>,
}

#[derive(Debug, Clone)]
pub struct FieldValue {
    pub type_name: TypeName,
    pub value: RalphValue,
}

impl<'de> Deserialize<'de> for FieldValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct Helper {
            #[serde(rename = "type")]
            type_name: String,
            value: serde_json::Value,
        }

        let helper = Helper::deserialize(deserializer)?;
        let type_name = TypeName::try_from(helper.type_name.as_str()).map_err(de::Error::custom)?;
        let value = RalphValue::from_typename_and_value(&type_name, &helper.value)
            .map_err(de::Error::custom)?;
        Ok(FieldValue { type_name, value })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum TypeName {
    Bool,
    U256,
    I256,
    ByteVec,
    Address,
    Map(Box<TypeName>, Box<TypeName>),
}

impl TryFrom<&str> for TypeName {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "Bool" => Ok(TypeName::Bool),
            "U256" => Ok(TypeName::U256),
            "I256" => Ok(TypeName::I256),
            "ByteVec" => Ok(TypeName::ByteVec),
            "Address" => Ok(TypeName::Address),
            s if s.starts_with("Map[") && s.ends_with(']') => {
                // Example: Map[U256,U256]
                let inner = &s[4..s.len() - 1];
                let mut parts = inner.split(',');
                let key = parts.next().context("Missing key type in Map")?;
                let value = parts.next().context("Missing value type in Map")?;
                Ok(TypeName::Map(
                    Box::new(TypeName::try_from(key)?),
                    Box::new(TypeName::try_from(value)?),
                ))
            },
            _ => Err(anyhow!("Unknown type name: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RalphValue {
    Bool(bool),
    U256(U256),
    I256(I256),
    ByteVec(Vec<u8>),
    Address(String),
    Map(HashMap<RalphValue, RalphValue>),
}

impl Hash for RalphValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the variant discriminant first so different variants
        // with the same inner value don't collide
        std::mem::discriminant(self).hash(state);
        match self {
            RalphValue::Bool(b) => b.hash(state),
            RalphValue::U256(u) => u.hash(state),
            RalphValue::I256(i) => i.hash(state),
            RalphValue::ByteVec(bytes) => bytes.hash(state),
            RalphValue::Address(addr) => addr.hash(state),
            RalphValue::Map(map) => {
                for (key, value) in map {
                    key.hash(state);
                    value.hash(state);
                }
            },
        }
    }
}

impl TryFrom<&str> for RalphValue {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
        let hex_str = hex::encode(s.as_bytes());
        let decoded = hex::decode(hex_str)?;
        Ok(RalphValue::ByteVec(decoded))
    }
}

impl RalphValue {
    pub fn from_typename_and_value(ty: &TypeName, value: &Value) -> Result<Self> {
        match ty {
            TypeName::Bool => {
                if let Some(b) = value.as_bool() {
                    Ok(RalphValue::Bool(b))
                } else if let Some(s) = value.as_str() {
                    match s {
                        "true" => Ok(RalphValue::Bool(true)),
                        "false" => Ok(RalphValue::Bool(false)),
                        _ => Err(anyhow!("Expected 'true' or 'false' string for Bool")),
                    }
                } else {
                    Err(anyhow!("Expected bool value or 'true'/'false' string"))
                }
            },
            TypeName::U256 => {
                let n = value
                    .as_str()
                    .context("Expected U256 as string or number")?;
                let parsed = n.parse::<U256>().map_err(Error::msg)?;
                Ok(RalphValue::U256(parsed))
            },
            TypeName::I256 => {
                let n = value
                    .as_str()
                    .context("Expected I256 as string or number")?;
                let parsed = n.parse::<I256>().map_err(Error::msg)?;
                Ok(RalphValue::I256(parsed))
            },
            TypeName::ByteVec => {
                if let Some(arr) = value.as_array() {
                    let bytes = arr
                        .iter()
                        .map(|v| {
                            v.as_u64()
                                .map(|b| b as u8)
                                .context("Expected u8 in ByteVec array")
                        })
                        .collect::<Result<Vec<u8>>>();
                    Ok(RalphValue::ByteVec(bytes?))
                } else if let Some(s) = value.as_str() {
                    Ok(s.try_into()?)
                } else {
                    Err(anyhow!("Expected ByteVec as array or string"))
                }
            },
            TypeName::Address => {
                let addr = value.as_str().context("Expected Address as string")?;
                Ok(RalphValue::Address(addr.to_string()))
            },
            TypeName::Map(key_ty, val_ty) => {
                let obj = value.as_object().context("Expected Map as object")?;
                let mut map = HashMap::new();
                for (k, v) in obj {
                    let key =
                        RalphValue::from_typename_and_value(key_ty, &Value::String(k.clone()))?;
                    let val = RalphValue::from_typename_and_value(val_ty, v)?;
                    map.insert(key, val);
                }
                Ok(RalphValue::Map(map))
            },
        }
    }
}
