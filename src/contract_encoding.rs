use anyhow;
use serde_json::Value;

use crate::contracts::NetworkType;

fn _build_debug_bytecode(bytecode: &str, bytecode_patch: &str) -> Result<String, anyhow::Error> {
    if bytecode_patch.is_empty() {
        return Ok(bytecode.to_string());
    }

    let pattern = regex::Regex::new(r"[=+-][0-9a-f]*")?;
    let mut result = String::new();
    let mut index = 0;

    for parts in pattern.find_iter(bytecode_patch) {
        let part = parts.as_str();
        let diff_type = part.chars().next().unwrap();

        match diff_type {
            '=' => {
                let length = part[1..].parse::<usize>().unwrap();
                result.push_str(&bytecode[index..index + length]);
                index += length;
            },
            '+' => {
                result.push_str(&part[1..]);
            },
            '-' => {
                let length = part[1..].parse::<usize>().unwrap();
                index += length;
            },
            _ => {},
        }
    }

    Ok(result)
}

pub fn _encode_contract_fields(byte_code: &str, byte_code_debug: &str, network: &NetworkType, _fields: &Value) -> Result<String, anyhow::Error> {
    /* If devnet, use build_debug_bytecode to patch the compiled bytecode with bytecodeDebugPatch */
    /* Encode the values of the fields and concat it */
    let _byte_code = match network {
        NetworkType::Dev => { _build_debug_bytecode(byte_code, byte_code_debug)? },
        _ => byte_code.to_string()
    };

    todo!()
}
