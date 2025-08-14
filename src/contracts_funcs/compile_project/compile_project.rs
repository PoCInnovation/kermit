use anyhow::{Context, Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;

use crate::contracts_funcs::compile_project::compile_project_deserialize::{
    RawCompileProject, RawContract,
};
use crate::contracts_funcs::compile_project::compile_project_values::{RalphValue, TypeName};
use crate::utils::crypto::is_hex_string;
use crate::utils::fs::read_file;

pub type FieldsTypesMap = HashMap<String, (TypeName, bool)>;
pub type FieldsMap = HashMap<String, (RalphValue, bool)>;
pub type InputFieldsMap = HashMap<String, RalphValue>;

pub fn fields_vec_to_fields_map(
    ralph_input: Vec<(String, String)>,
    types: &FieldsTypesMap,
) -> Result<InputFieldsMap> {
    // TODO: handle structure  and array types of inputs 

    if ralph_input.len() != types.len() {
        return Err(anyhow!(
            "Fields count mismatch with initial fields: expected {}, found {}",
            types.len(),
            ralph_input.len()
        ));
    }

    let result = ralph_input.into_iter().map(|(name, value)| {
        let (type_name, _) = types.get(&name).context(anyhow!(
            "Type for field '{}' not found in provided types map",
            name
        ))?;

        let ralph_value = if type_name.clone() == TypeName::ByteVec && !is_hex_string(&value) {
            value.as_str().try_into()?
        } else {
            RalphValue::from_typename_and_value(type_name, &Value::String(value))?
        };

        Ok((name, ralph_value))
    }).collect::<Result<HashMap<_, _>>>()?;
    Ok(result)
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub field_names: Vec<String>,
    pub field_types: Vec<String>,
    pub is_mutable: Vec<bool>,
}

#[derive(Debug, Clone)]
pub struct Contract {
    pub version: String,
    pub std_interface_id: Option<String>,
    pub name: String,
    pub bytecode: String,
    pub bytecode_debug_patch: String,
    pub code_hash: String,
    pub code_hash_debug: String,
    pub fields_types: FieldsTypesMap,
}

impl Contract {
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

        Ok(Self {
            version: contract.version,
            std_interface_id: contract.std_interface_id,
            name: contract.name,
            bytecode: contract.bytecode,
            bytecode_debug_patch: contract.bytecode_debug_patch,
            code_hash: contract.code_hash,
            code_hash_debug: contract.code_hash_debug,
            fields_types,
        })
    }
}

pub struct Script {
    pub version: String,
    pub name: String,
    pub bytecode_template: String,
    pub bytecode_debug_patch: String,
    pub fields: FieldsTypesMap,
}

pub struct CompileProject {
    pub contracts: Vec<Contract>,
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
            .map(|contract| Contract::try_from(contract, &structs))
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
