use anyhow::{Context, Result, anyhow};
use regex::Regex;
use std::collections::HashMap;

use crate::contracts_funcs::{
    compile_output::{CompileProject, Contract, FieldsMap, FieldsTypesMap, RalphValue, TypeName},
    contract_codec::encode_i32,
    deploy_vm_encode::{
        encode_vmbyte_address, encode_vmbyte_bool, encode_vmbyte_i256, encode_vmbyte_u256,
        encode_vmbyte_vec,
    },
};

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
    let size_buffer = encode_i32(fields.keys().len() as i32);
    let bytecode = fields.iter().try_fold(size_buffer, |mut acc, (name, (value, _))| {
        let encoded_value = match value {
            RalphValue::Bool(b) => encode_vmbyte_bool(*b),
            RalphValue::U256(n) => encode_vmbyte_u256(*n),
            RalphValue::I256(n) => encode_vmbyte_i256(*n),
            RalphValue::ByteVec(bytes) => encode_vmbyte_vec(bytes),
            RalphValue::Address(addr) => encode_vmbyte_address(addr),
            _ => return Err(anyhow!("Unsupported value type for field '{}'", name)),
        }?;
        acc.extend_from_slice(&[name.as_bytes().to_vec(), encoded_value].concat());
        Ok(acc)
    })?;
    Ok(hex::encode(bytecode))
}

pub fn build_bytecode_contract(
    contract: &Contract,
    init_fields: HashMap<String, RalphValue>,
    is_devnet: bool,
) -> Result<String> {
    let main_bytecode = if is_devnet {
        get_debug_bytecode(&contract.bytecode, &contract.bytecode_debug_patch)?
    } else {
        contract.bytecode.clone()
    };

    let fields_types: FieldsTypesMap = contract.fields.clone().try_into()?;
    let mut fields = fields_types
        .iter()
        .zip(init_fields.iter())
        .map(|((name, (ty, is_mutable)), (init_name, value))| {
            if name != init_name {
                return Err(anyhow!(
                    "Field name mismatch: expected '{}', found '{}'",
                    name,
                    init_name
                ));
            }
            Ok((name.clone(), (value.clone(), *is_mutable)))
        })
        .collect::<Result<FieldsMap>>()?;

    let encoded_prefix = get_std_prefix(&contract.std_interface_id);
    if let Some(s) = encoded_prefix {
        fields.insert(
            "__stdInterfaceId".to_string(),
            (RalphValue::ByteVec(s.into()), false),
        );
    }

    let (mutables, immutables): (HashMap<_, _>, HashMap<_, _>) = fields
        .into_iter()
        .partition(|(_, (_, is_mutable))| *is_mutable);

    let imm_bytecode = encode_fields_by_type(immutables)?;
    let mut_bytecode = encode_fields_by_type(mutables)?;

    Ok(main_bytecode + &imm_bytecode + &mut_bytecode)
}
