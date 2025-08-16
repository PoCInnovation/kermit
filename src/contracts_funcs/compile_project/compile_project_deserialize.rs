use serde::{
    Deserialize,
};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCompileProject {
    pub contracts: Vec<RawContract>,
    pub scripts: Vec<RawScript>,
    pub structs: Option<Vec<StructDef>>,
    pub constants: Option<Vec<Constant>>,
    pub enums: Option<Vec<EnumDef>>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawContract {
    pub version: String,
    pub name: String,
    pub bytecode: String,
    pub bytecode_debug_patch: String,
    pub code_hash: String,
    pub code_hash_debug: String,
    pub fields: Fields,
    pub functions: Vec<RawFunction>,
    pub constants: Vec<Constant>,
    pub enums: Vec<EnumDef>,
    pub events: Vec<Event>,
    pub warnings: Vec<String>,
    pub maps: Option<Maps>,
    pub std_interface_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawScript {
    pub version: String,
    pub name: String,
    pub bytecode_template: String,
    pub bytecode_debug_patch: String,
    pub fields: Fields,
    pub functions: Vec<RawFunction>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructDef {
    pub name: String,
    pub field_names: Vec<String>,
    pub field_types: Vec<Value>,
    pub is_mutable: Vec<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub names: Vec<String>,
    pub types: Vec<Value>,
    pub is_mutable: Vec<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawFunction {
    pub name: String,
    pub use_preapproved_assets: bool,
    pub use_assets_in_contract: bool,
    pub is_public: bool,
    pub param_names: Vec<String>,
    pub param_types: Vec<Value>,
    pub param_is_mutable: Vec<bool>,
    pub return_types: Vec<Value>,
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
    pub type_name: Value,
    pub value: FieldValueHelper,
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
    pub value: FieldValueHelper,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub name: String,
    pub field_names: Vec<String>,
    pub field_types: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Maps {
    pub names: Vec<String>,
    pub types: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FieldValueHelper {
    #[serde(rename = "type")]
    pub type_name: String,
    pub value: Value,
}
