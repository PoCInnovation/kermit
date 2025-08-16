use anyhow::{Context, Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;

use crate::contracts_funcs::compile_project::compile_project_deserialize::{
    RawCompileProject, RawContract, RawFunction,
};
use crate::contracts_funcs::compile_project::compile_project_values::{
    FieldValue, RalphValue, TypeName,
};
use crate::utils::crypto::is_hex_string;
use crate::utils::fs::read_file;

pub type FieldsTypesMap = HashMap<String, TypeName>;
pub type FieldsTypesMapMut = HashMap<String, (TypeName, bool)>;
pub type FieldsMap = HashMap<String, (RalphValue, bool)>;
pub type InputFieldsMap = HashMap<String, RalphValue>;

fn resolve_rec_type(
    name: &str,
    value: String,
    types: &FieldsTypesMap,
    override_type: Option<&TypeName>,
) -> Result<RalphValue> {
    let mut parts = name.splitn(2, '.');
    let name = parts.next().context("Field name cannot be empty")?;
    let rest = parts.next();

    let type_name = if let Some(a) = override_type {
        a
    } else {
        types.get(name).context(anyhow!(
            "Type for field '{}' not found in provided types map",
            name
        ))?
    };

    match type_name {
        TypeName::Array(array_type) => {
            if !value.starts_with('[') || !value.ends_with(']') {
                return Err(anyhow!(
                    "Array value for field '{}' must begin and end with []",
                    name
                ));
            }

            let array_values = value
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(|v| resolve_rec_type("", v.to_string(), types, Some(&*array_type)))
                .collect::<Result<Vec<_>>>()?;
            Ok(RalphValue::Array(array_values))
        },
        TypeName::Structure(struct_content) => {
            let rest = rest.context(format!(
                "Structure field name cannot be empty for field '{}', type '{:?}'",
                name, type_name
            ))?;
            let mut sub_parts = rest.splitn(2, '.');

            if let Some(remaining_dots) = sub_parts.next() {
                // there are nested structure(s)
                return resolve_rec_type(remaining_dots, value, struct_content, None);
            }

            resolve_rec_type(rest, value, struct_content, None)
        },
        _ => {
            let ralph_value = if type_name.clone() == TypeName::ByteVec && !is_hex_string(&value) {
                value.as_str().try_into()?
            } else {
                RalphValue::from_typename_and_value(type_name, &Value::String(value))?
            };

            Ok(ralph_value)
        },
    }
}

fn fields_types_map_len(map: &FieldsTypesMap) -> usize {
    map.iter()
        .map(|(_, type_name)| match type_name {
            TypeName::Structure(struct_fields) => fields_types_map_len(struct_fields),
            _ => 1,
        })
        .sum()
}

pub fn fields_vec_to_fields_map(
    ralph_input: Vec<(String, String)>,
    types: &FieldsTypesMap,
) -> Result<InputFieldsMap> {
    if ralph_input.len() != fields_types_map_len(types) {
        return Err(anyhow!(
            "Fields count mismatch with initial fields: expected {}, found {}",
            types.len(),
            ralph_input.len()
        ));
    }

    let result = ralph_input
        .into_iter()
        .map(|(name, value)| Ok((name.clone(), resolve_rec_type(&name, value, types, None)?)))
        .collect::<Result<InputFieldsMap>>()?;
    Ok(result)
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub field_names: Vec<String>,
    pub field_types: Vec<String>,
    pub is_mutable: Vec<bool>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub use_preapproved_assets: bool,
    pub use_assets_in_contract: bool,
    pub is_public: bool,
    pub param_names: Vec<String>,
    pub param_types: Vec<FieldValue>,
    pub param_is_mutable: Vec<bool>,
    pub return_types: Vec<FieldValue>,
}

#[derive(Debug, Clone)]
pub struct CompiledContract {
    pub version: String,
    pub std_interface_id: Option<String>,
    pub name: String,
    pub bytecode: String,
    pub bytecode_debug_patch: String,
    pub code_hash: String,
    pub code_hash_debug: String,
    pub fields_types: FieldsTypesMapMut,
    pub functions: HashMap<String, Function>,
}

impl CompiledContract {
    pub fn try_from(contract: RawContract, structs: &HashMap<String, Struct>) -> Result<Self> {
        let fields_iter = contract.fields.names.iter().zip(
            contract
                .fields
                .types
                .iter()
                .zip(contract.fields.is_mutable.iter()),
        );

        let fields_types = fields_iter
            .map(|(name, (ty, is_mutable))| {
                let ty = ty.as_str().context("Expected field type as string")?;
                let type_name = TypeName::from_name_and_structures(ty, &structs)?;
                Ok((name.clone(), (type_name, *is_mutable)))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        let functions = contract
            .functions
            .into_iter()
            .map(|f| {
                let name = f.name.clone();
                let function = f.try_into()?;
                Ok((name, function))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Self {
            version: contract.version,
            std_interface_id: contract.std_interface_id,
            name: contract.name,
            bytecode: contract.bytecode,
            bytecode_debug_patch: contract.bytecode_debug_patch,
            code_hash: contract.code_hash,
            code_hash_debug: contract.code_hash_debug,
            fields_types,
            functions,
        })
    }

    pub fn get_method_index(&self, function_name: &str, method_name: &str) -> Result<usize> {
        let function = self.functions.get(function_name).context(format!(
            "Function '{}' not found in contract '{}'",
            function_name, self.name
        ))?;

        function
            .param_names
            .iter()
            .position(|name| name == method_name)
            .context(format!(
                "Method '{}' not found in function '{}'",
                method_name, function_name
            ))
    }
}

pub struct Script {
    pub version: String,
    pub name: String,
    pub bytecode_template: String,
    pub bytecode_debug_patch: String,
    pub fields: FieldsTypesMapMut,
}

pub struct CompileProject {
    pub contracts: Vec<CompiledContract>,
    pub scripts: Vec<Script>,
}

impl TryFrom<RawCompileProject> for CompileProject {
    type Error = anyhow::Error;

    fn try_from(raw: RawCompileProject) -> Result<Self, Self::Error> {
        let structs: HashMap<String, Struct> = raw
            .structs
            .unwrap_or_default()
            .into_iter()
            .map(|s| {
                let field_names = s.field_names;
                let field_types = s.field_types.iter().map(|t| t.to_string()).collect();
                let is_mutable = s.is_mutable;
                Ok((
                    s.name,
                    Struct {
                        field_names,
                        field_types,
                        is_mutable,
                    },
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        let contracts = raw
            .contracts
            .into_iter()
            .map(|contract| CompiledContract::try_from(contract, &structs))
            .collect::<Result<Vec<_>>>()?;

        let scripts = raw
            .scripts
            .into_iter()
            .map(|raw_script| {
                Ok(Script {
                    version: raw_script.version,
                    name: raw_script.name,
                    bytecode_template: raw_script.bytecode_template,
                    bytecode_debug_patch: raw_script.bytecode_debug_patch,
                    fields: HashMap::new(), // TODO
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(CompileProject { contracts, scripts })
    }
}

impl TryFrom<RawFunction> for Function {
    type Error = anyhow::Error;

    fn try_from(raw: RawFunction) -> Result<Self, Self::Error> {
        let param_types = raw
            .param_types
            .into_iter()
            .map(FieldValue::try_from)
            .collect::<Result<Vec<_>>>()?;

        let return_types = raw
            .return_types
            .into_iter()
            .map(FieldValue::try_from)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            use_preapproved_assets: raw.use_preapproved_assets,
            use_assets_in_contract: raw.use_assets_in_contract,
            is_public: raw.is_public,
            param_names: raw.param_names,
            param_types,
            param_is_mutable: raw.param_is_mutable,
            return_types,
        })
    }
}

pub fn load_compile_project(path: &str) -> Result<CompileProject> {
    let compiled_project_result = serde_json::from_str::<RawCompileProject>(&read_file(&path)?);

    let compiled_project = match compiled_project_result {
        Ok(project) => project,
        Err(e) => {
            return Err(anyhow!("Error parsing compile output: {}", e));
        },
    };

    compiled_project.try_into()
}
