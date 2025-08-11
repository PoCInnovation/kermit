use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileProject {
    pub contracts: Vec<Contract>,
    pub scripts: Vec<Script>,
    pub structs: Vec<StructDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub maps: Maps,
    pub std_interface_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructDef {
    pub name: String,
    pub field_names: Vec<String>,
    pub field_types: Vec<TypeName>,
    pub is_mutable: Vec<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub names: Vec<String>,
    pub types: Vec<TypeName>,
    pub is_mutable: Vec<bool>,
}

pub type FieldsMap = HashMap<String, (TypeName, bool)>;

impl Into<FieldsMap> for Fields {
    fn into(self) -> HashMap<String, (TypeName, bool)> {
        self.names
            .into_iter()
            .zip(self.types.into_iter())
            .zip(self.is_mutable.into_iter())
            .map(|((name, ty), is_mut)| (name, (ty, is_mut)))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constant {
    pub name: String,
    pub value: ConstantValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstantValue {
    pub type_name: TypeName,
    pub value: FieldValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumDef {
    pub name: String,
    pub fields: Vec<EnumField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumField {
    pub name: String,
    pub value: EnumFieldValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumFieldValue {
    pub type_name: TypeName,
    pub value: FieldValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub name: String,
    pub field_names: Vec<String>,
    pub field_types: Vec<TypeName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Maps {
    pub names: Vec<String>,
    pub types: Vec<TypeName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldValue {
    pub tyne_name: TypeName,
    pub value: Value, // they can be converted to TypeName...
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
