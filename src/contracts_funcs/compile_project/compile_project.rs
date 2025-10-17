use anyhow::{Context, Result, anyhow, bail};
use indexmap::IndexMap;
use serde_json::{Value, value::RawValue};
use std::collections::HashMap;

use crate::common::{crypto::is_hex_string, fs::read_file};
use crate::contracts_funcs::compile_project::compile_project_deserialize::{
    RawCompileProject, RawContract, RawFunction, StructDef,
};
use crate::contracts_funcs::compile_project::compile_project_values::{RalphValue, TypeName};

pub type FieldsTypesMapMut = IndexMap<String, (TypeName, bool)>;
pub type FieldsVec = Vec<(RalphValue, bool)>;

// Return the position of the parameter for the call with the parsed value
// TODO: new system with json like parsing of structures, since currently arrays of structure is impossible
fn resolve_rec_type(
    name: &str,
    value: String,
    types: &FieldsTypesMapMut,
    override_type: Option<&TypeName>,
) -> Result<(usize, RalphValue)> {
    let mut parts = name.splitn(2, '.');
    let name = parts.next().context("Field name cannot be empty")?;
    let rest = parts.next();

    let (type_name, param_index) = if let Some(typename) = override_type {
        (typename, 0) // 0 magic value since the index is skipped
    } else {
        let (param_index, _, (typename, _)) = types.get_full(name).context(anyhow!(
            "Type for field '{}' not found in provided args types map: {:?}",
            name,
            types
        ))?;
        (typename, param_index)
    };

    match type_name {
        TypeName::Structure((_, struct_types)) => {
            let rest = rest.context(format!(
                "Structure field name cannot be empty for field '{}', type '{:?}'",
                name, type_name
            ))?;
            let mut sub_parts = rest.splitn(2, '.');
            let nested_struct_name = sub_parts.next().context(format!(
                "Structure field name cannot be empty for field '{}', type '{:?}'",
                name, type_name
            ))?;

            if let Some(remaining_dots) = sub_parts.next() {
                // there are nested structure(s)
                if let (sub_index, _, (TypeName::Structure((_, nested_struct_types)), _)) =
                    struct_types.get_full(nested_struct_name).context(format!(
                        "Unknown nested structure name in {name}: {nested_struct_name}"
                    ))?
                {
                    let (sub_nested_index, value) =
                        resolve_rec_type(remaining_dots, value, nested_struct_types, None)?;
                    return Ok((param_index + sub_index + sub_nested_index, value));
                } else {
                    bail!(
                        "Field '{nested_struct_name}' in structure '{type_name:?}' is not a structure",
                    );
                }
            }

            let (sub_index, value) = resolve_rec_type(rest, value, struct_types, None)?;
            Ok((param_index + sub_index, value))
        },
        TypeName::Array((arr_type, arr_size)) => {
            // example: [0, 1]
            // example: [[0,1], [0,2]]

            // Using serde json to parse the array, only the outer array
            let vec_values = serde_json::from_str::<Vec<Box<RawValue>>>(&value)?
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            if vec_values.len() != *arr_size {
                bail!(
                    "Array size mismatch for field '{}': expected {}, found {}",
                    name,
                    arr_size,
                    vec_values.len()
                );
            }

            // We don't care about the index recusively since they are already in order
            let ralph_values = vec_values
                .into_iter()
                .map(|v| {
                    let (_, value) = resolve_rec_type(name, v, types, Some(arr_type))?;
                    Ok(value)
                })
                .collect::<Result<Vec<_>>>()?;

            Ok((param_index, RalphValue::Array(ralph_values)))
        },
        _ => {
            let is_hex_string = is_hex_string(&value);
            let ralph_value = if *type_name == TypeName::ByteVec && !is_hex_string {
                // Human readable  String
                value.as_str().try_into()?
            } else {
                // If it's still a string, but hexified, other types goes here
                let value = if is_hex_string {
                    value.strip_prefix("0x").unwrap_or(&value).to_string()
                } else {
                    value
                };

                RalphValue::from_typename_and_value(type_name, &Value::String(value))?
            };

            Ok((param_index, ralph_value))
        },
    }
}

fn fields_types_map_len(map: &FieldsTypesMapMut) -> usize {
    map.iter()
        .map(|(_, (type_name, _))| match type_name {
            TypeName::Structure((_, struct_fields)) => fields_types_map_len(struct_fields),
            _ => 1,
        })
        .sum()
}

pub fn args_to_fields_vec(
    ralph_vec_input: Vec<(String, String)>,
    types: &FieldsTypesMapMut,
) -> Result<Vec<RalphValue>> {
    if ralph_vec_input.len() != fields_types_map_len(types) {
        bail!(
            "Fields count mismatch with initial fields: expected {}, found {}",
            types.len(),
            ralph_vec_input.len()
        );
    }

    let mut unordered_params_vec = ralph_vec_input
        .into_iter()
        .map(|(name, value)| resolve_rec_type(&name, value, types, None))
        .collect::<Result<Vec<_>>>()?;

    unordered_params_vec.sort_by_key(|(pos, _)| *pos);
    Ok(unordered_params_vec
        .into_iter()
        .map(|(_, val)| val)
        .collect())
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub field_names: Vec<String>,
    pub field_types: Vec<String>,
    pub is_mutable: Vec<bool>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub use_preapproved_assets: bool,
    pub use_assets_in_contract: bool,
    pub is_public: bool,
    pub params_types: FieldsTypesMapMut,
    pub return_types: Vec<TypeName>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CompiledContract {
    pub version: String,
    pub std_interface_id: Option<String>,
    pub name: String,
    pub bytecode: String,
    pub bytecode_debug_patch: String,
    pub code_hash: String,
    pub code_hash_debug: String,
    pub fields_types: FieldsTypesMapMut,
    pub functions: Vec<Function>,
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
                let type_name = TypeName::from_name_and_structures(ty, structs)?;
                Ok((name.clone(), (type_name, *is_mutable)))
            })
            .collect::<Result<IndexMap<_, _>>>()?;

        let functions = contract
            .functions
            .into_iter()
            .map(|raw| Function::from_raw_and_structures(raw, structs))
            .collect::<Result<Vec<_>>>()?;

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

    pub fn get_method(&self, method_name: &str) -> Result<(&Function, usize)> {
        let method_index = self
            .functions
            .iter()
            .position(|f| f.name == method_name)
            .context(format!(
                "Method '{}' not found in contract '{}'",
                method_name, self.name
            ))?;
        let method = &self.functions[method_index];
        Ok((method, method_index))
    }
}

#[allow(dead_code)]
pub struct Script {
    pub version: String,
    pub name: String,
    pub bytecode_template: String,
    pub bytecode_debug_patch: String,
    pub fields: FieldsTypesMapMut,
}

#[allow(dead_code)]
pub struct CompileProject {
    pub contracts: Vec<CompiledContract>,
    pub scripts: Vec<Script>,
}

impl CompileProject {
    pub fn get_contract_by_name(&self, contract_name: &str) -> Result<&CompiledContract> {
        self.contracts
            .iter()
            .find(|c| c.name == contract_name)
            .context(format!(
                "Contract '{contract_name}' not found in compiled project"
            ))
    }
}

impl TryFrom<RawCompileProject> for CompileProject {
    type Error = anyhow::Error;

    fn try_from(raw: RawCompileProject) -> Result<Self, Self::Error> {
        let structs: HashMap<String, Struct> = raw
            .structs
            .unwrap_or_default()
            .into_iter()
            .map(|s: StructDef| {
                let field_names = s.field_names;
                let field_types = s
                    .field_types
                    .iter()
                    .map(|t| {
                        let t = t
                            .as_str()
                            .context(format!("Struct field type is not a string: {t:?}"))?;

                        Ok(t.to_string())
                    })
                    .collect::<Result<Vec<_>>>()?;
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
                    fields: IndexMap::new(), // TODO
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { contracts, scripts })
    }
}

impl Function {
    fn from_raw_and_structures(
        raw: RawFunction,
        structures: &HashMap<String, Struct>,
    ) -> Result<Self> {
        let params_types = raw
            .param_types
            .into_iter()
            .zip(raw.param_is_mutable.iter())
            .zip(raw.param_names.iter())
            .map(|((v, is_mutable), name)| {
                let ty = TypeName::from_name_and_structures(
                    v.as_str().context(format!(
                        "Function parameter should be a correct type. Found {v}"
                    ))?,
                    structures,
                )?;

                Ok((name.clone(), (ty, *is_mutable)))
            })
            .collect::<Result<FieldsTypesMapMut>>()?;

        let return_types = raw
            .return_types
            .into_iter()
            .map(|x| {
                TypeName::from_name_and_structures(
                    x.as_str().context(format!(
                        "Function return type should be a correct type. Found {x}",
                    ))?,
                    structures,
                )
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            name: raw.name,
            use_preapproved_assets: raw.use_preapproved_assets,
            use_assets_in_contract: raw.use_assets_in_contract,
            is_public: raw.is_public,
            params_types,
            return_types,
        })
    }
}

pub fn load_compile_project(path: &str) -> Result<CompileProject> {
    let compiled_project_result = serde_json::from_str::<RawCompileProject>(&read_file(path)?);

    let compiled_project = match compiled_project_result {
        Ok(project) => project,
        Err(e) => {
            bail!("Error parsing compile output: {e}");
        },
    };

    compiled_project.try_into()
}
