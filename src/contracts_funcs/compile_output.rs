use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileOutput {
    pub contracts: Vec<Contract>,
    pub scripts: Vec<Script>,
    pub structs: Vec<StructDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub version: String,
    pub name: String,
    pub bytecode: String,
    #[serde(rename = "bytecodeDebugPatch")]
    pub bytecode_debug_patch: String,
    #[serde(rename = "codeHash")]
    pub code_hash: String,
    #[serde(rename = "codeHashDebug")]
    pub code_hash_debug: String,
    pub fields: Fields,
    pub functions: Vec<Function>,
    pub constants: Vec<Constant>,
    pub enums: Vec<EnumDef>,
    pub events: Vec<Event>,
    pub warnings: Vec<String>,
    pub maps: Maps,
    #[serde(rename = "stdInterfaceId")]
    pub std_interface_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub version: String,
    pub name: String,
    #[serde(rename = "bytecodeTemplate")]
    pub bytecode_template: String,
    #[serde(rename = "bytecodeDebugPatch")]
    pub bytecode_debug_patch: String,
    pub fields: Fields,
    pub functions: Vec<Function>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDef {
    pub name: String,
    #[serde(rename = "fieldNames")]
    pub field_names: Vec<String>,
    #[serde(rename = "fieldTypes")]
    pub field_types: Vec<TypeName>,
    #[serde(rename = "isMutable")]
    pub is_mutable: Vec<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fields {
    pub names: Vec<String>,
    #[serde(rename = "types")]
    pub types: Vec<TypeName>,
    #[serde(rename = "isMutable")]
    pub is_mutable: Vec<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    #[serde(rename = "usePreapprovedAssets")]
    pub use_preapproved_assets: bool,
    #[serde(rename = "useAssetsInContract")]
    pub use_assets_in_contract: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "paramNames")]
    pub param_names: Vec<String>,
    #[serde(rename = "paramTypes")]
    pub param_types: Vec<TypeName>,
    #[serde(rename = "paramIsMutable")]
    pub param_is_mutable: Vec<bool>,
    #[serde(rename = "returnTypes")]
    pub return_types: Vec<TypeName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    pub value: ConstantValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantValue {
    #[serde(rename = "type")]
    pub type_name: TypeName,
    pub value: FieldValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDef {
    pub name: String,
    pub fields: Vec<EnumField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumField {
    pub name: String,
    pub value: EnumFieldValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumFieldValue {
    #[serde(rename = "type")]
    pub type_name: TypeName,
    pub value: FieldValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    #[serde(rename = "fieldNames")]
    pub field_names: Vec<String>,
    #[serde(rename = "fieldTypes")]
    pub field_types: Vec<TypeName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maps {
    pub names: Vec<String>,
    #[serde(rename = "types")]
    pub types: Vec<TypeName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValue {
    #[serde(rename = "name")]
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
