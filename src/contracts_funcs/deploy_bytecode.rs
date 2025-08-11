use anyhow::{Context, Result, anyhow};
use regex::Regex;

use crate::contracts_funcs::compile_output::{CompileProject, Contract, FieldsMap, TypeName};

fn get_std_prefix(std_interface_id: &str) -> Option<String> {
    const STD_INTERFACE_PREFIX: &str = "414c5048";
    if std_interface_id.is_empty() {
        return None;
    }
    Some(STD_INTERFACE_PREFIX.to_string() + std_interface_id)
}

fn get_debug_bytecode(bytecode: &str, bytecode_patch: &str) -> Result<String> {
    if bytecode_patch.is_empty() {
        return Ok(bytecode.to_string());
    }

    let pattern = Regex::new(r"[=+-][0-9a-f]*").context("Failed to compile regex pattern")?;
    let mut result = String::new();
    let mut index = 0;

    for parts in pattern.find_iter(bytecode_patch) {
        let part = parts.as_str();
        let diff_type = part.chars().next().context("Bytecode size too short")?;

        match diff_type {
            '=' => {
                let length = usize::from_str_radix(&part[1..], 10)
                    .context("Failed to parse length for '=' patch")?;
                result.push_str(&bytecode[index..index + length]);
                index += length;
            },
            '+' => {
                result.push_str(&part[1..]);
            },
            '-' => {
                let length = usize::from_str_radix(&part[1..], 10)
                    .context("Failed to parse length for '-' patch")?;
                index += length;
            },
            _ => return Err(anyhow!("Unknown diff type: {}", diff_type)),
        }
    }

    Ok(result)
}

fn encode_fields_by_type(fields: FieldsMap) -> Result<String> {
    todo!()
}

fn encode_fields(fields: FieldsMap) -> Result<String> {
    todo!()
}

fn build_bytecode_contract(contract: &Contract) -> Result<String> {
    let mut fields: FieldsMap = contract.fields.clone().try_into()?;
    let encoded_prefix = get_std_prefix(&contract.std_interface_id);
    if let Some(z) =  encoded_prefix {
        fields.insert("__stdInterfaceId".to_string(), (TypeName::ByteVec, false));
    }
    todo!()
}

pub fn build_bytecode(compiled_project: CompileProject) -> Result<String> {
    for a in &compiled_project.contracts {
        let bytecode = build_bytecode_contract(a)?;
    }
    todo!()
}
