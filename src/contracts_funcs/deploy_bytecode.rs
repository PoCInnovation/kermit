use anyhow::{Context, Result, anyhow, bail};
use regex::Regex;

use crate::contracts_funcs::{
    compile_project::{
        compile_project::{CompiledContract, FieldsMap, FieldsVec, InputFieldsMap},
        compile_project_values::RalphValue,
    },
    contract_codec::encode_i32,
    deploy_vm_encode::{
        encode_vmbyte_address, encode_vmbyte_bool, encode_vmbyte_i256, encode_vmbyte_u256,
        encode_vmbyte_vec,
    },
};

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
            _ => bail!("Unknown diff type: {}", diff_type),
        }
    }

    Ok(result)
}

fn encode_fields_by_type(fields: &FieldsVec, is_mutable: bool) -> Result<Vec<u8>> {
    let size_buffer = encode_i32(fields.len() as i32);
    let bytecode = fields.iter().try_fold(size_buffer, |mut acc, (value, _)| {
        let encoded_value = match value {
            RalphValue::Bool(b) => encode_vmbyte_bool(*b),
            RalphValue::U256(n) => encode_vmbyte_u256(*n),
            RalphValue::I256(n) => encode_vmbyte_i256(*n),
            RalphValue::ByteVec(bytes) => encode_vmbyte_vec(bytes),
            RalphValue::Address(addr) => encode_vmbyte_address(addr),
            RalphValue::Array(arr) => {
                let mut encoded: Vec<u8> = Vec::new();
                for item in arr {
                    encoded.extend_from_slice(&encode_fields_by_type(
                        &vec![((item.clone(), is_mutable))].into_iter().collect(),
                        is_mutable,
                    )?);
                }
                Ok(encoded)
            },
            // Normally, if the structure is mutable, then at least one of the attributes is
            RalphValue::Structure(fields) => {
                let mut encoded: Vec<u8> = Vec::new();
                for (field_name, field_value) in fields {
                    encoded.extend_from_slice(&encode_fields_by_type(
                        &vec![((field_value.clone(), is_mutable))]
                            .into_iter()
                            .collect(),
                        is_mutable,
                    )?);
                }
                Ok(encoded)
            },
            _ => bail!("Unsupported value type for field '{:?}'", value),
        }?;
        acc.extend_from_slice(&encoded_value);
        Ok(acc)
    })?;
    Ok(bytecode)
}

fn get_contract_prefix(contract_prefix_str: &str) -> Result<RalphValue> {
    let std_bytes = match "ALPH".try_into()? {
        RalphValue::ByteVec(bytes) => bytes,
        _ => bail!("Unsupported RalphValue type for std_value"),
    };

    let contract_prefix_bytes = hex::decode(contract_prefix_str)?;
    Ok(RalphValue::ByteVec(
        [std_bytes, contract_prefix_bytes].concat(),
    ))
}

pub fn get_fields(contract: &CompiledContract, init_fields: InputFieldsMap) -> Result<FieldsMap> {
    let mut fields: FieldsMap = contract
        .fields_types
        .clone()
        .into_iter()
        .map(|(name, (_, is_mutable))| {
            let value = init_fields
                .get(&name)
                .cloned()
                .ok_or_else(|| anyhow!("Missing initial value for field '{}'", name))?;
            Ok((name, (value, is_mutable)))
        })
        .collect::<Result<FieldsMap>>()?;

    let contract_prefix = &contract.std_interface_id;
    if let Some(contract_prefix_str) = contract_prefix {
        let interface_value = get_contract_prefix(contract_prefix_str)?;
        fields.insert("__stdInterfaceId".to_string(), (interface_value, false));
    }

    Ok(fields)
}

pub fn get_fields_vec(contract: &CompiledContract, init_fields: FieldsVec) -> Result<FieldsVec> {
    let mut init_fields = init_fields;
    let contract_prefix = &contract.std_interface_id;
    if let Some(contract_prefix_str) = contract_prefix {
        let interface_value = get_contract_prefix(contract_prefix_str)?;
        init_fields.push((interface_value, false));
    }

    Ok(init_fields)
}

pub fn get_fields_bytecode(fields: FieldsVec) -> Result<(String, FieldsVec, FieldsVec)> {
    let (mutables, immutables): (Vec<_>, Vec<_>) =
        fields.into_iter().partition(|(_, is_mutable)| *is_mutable);

    let imm_bytecode = encode_fields_by_type(&immutables, false)?;
    let mut_bytecode = encode_fields_by_type(&mutables, true)?;

    Ok((
        hex::encode(imm_bytecode) + &hex::encode(mut_bytecode),
        immutables,
        mutables,
    ))
}

pub fn build_bytecode_contract(
    contract: &CompiledContract,
    init_fields: FieldsVec,
    is_devnet: bool,
) -> Result<String> {
    let main_bytecode = if is_devnet {
        get_debug_bytecode(&contract.bytecode, &contract.bytecode_debug_patch)?
    } else {
        contract.bytecode.clone()
    };

    let fields = get_fields_vec(contract, init_fields)?;
    let (fields_bytecode, _, _) = get_fields_bytecode(fields)?;

    Ok(main_bytecode + &fields_bytecode)
}
