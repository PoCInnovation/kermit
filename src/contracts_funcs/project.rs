use anyhow::Result;

use crate::{
    contracts::CompilerOptions, contracts_funcs::source_info::CodeInfo, utils::fs::read_file,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub node_version: String,
    pub compiler_options: CompilerOptions,
    pub infos: Vec<CodeInfo>,
}

#[derive(Serialize, Deserialize)]
struct ProjectJson {
    #[serde(rename = "fullNodeVersion")]
    node_version: String,
    #[serde(rename = "compilerOptionsUsed")]
    compiler_options: CompilerOptions,
    infos: HashMap<String, JsonCodeInfo>,
}

#[derive(Serialize, Deserialize)]
struct JsonCodeInfo {
    #[serde(rename = "sourceFile")]
    contract_relative_path: String,
    #[serde(rename = "sourceCodeHash")]
    source_code_hash: String,
    #[serde(rename = "bytecodeDebugPatch")]
    bytecode_debug_patch: Option<String>,
    #[serde(rename = "codeHashDebug")]
    code_hash_debug: Option<String>,
}

impl JsonCodeInfo {
    fn from_json(key_name: &str, json: JsonCodeInfo) -> Result<CodeInfo> {
        Ok(CodeInfo {
            name: key_name.to_string(),
            source_code: read_file(json.contract_relative_path.as_str())?,
            contract_relative_path: json.contract_relative_path,
            source_code_hash: json.source_code_hash,
            bytecode_debug_patch: json.bytecode_debug_patch,
            code_hash_debug: json.code_hash_debug,
        })
    }
}

impl Project {
    pub fn from_json(json_str: &str) -> Result<Project> {
        let pj: ProjectJson = serde_json::from_str(json_str)?;
        Ok(Project {
            node_version: pj.node_version,
            compiler_options: pj.compiler_options,
            infos: pj
                .infos
                .into_iter()
                .map(|(key, value)| JsonCodeInfo::from_json(&key, value))
                .collect::<Result<Vec<_>>>()?,
        })
    }

    pub fn to_json(&self) -> Result<String> {
        let infos: HashMap<String, JsonCodeInfo> = self
            .infos
            .iter()
            .map(|info| {
                (
                    info.name.clone(),
                    JsonCodeInfo {
                        contract_relative_path: info.contract_relative_path.clone(),
                        source_code_hash: info.source_code_hash.clone(),
                        bytecode_debug_patch: info.bytecode_debug_patch.clone(),
                        code_hash_debug: info.code_hash_debug.clone(),
                    },
                )
            })
            .collect();

        let project_json = ProjectJson {
            node_version: self.node_version.clone(),
            compiler_options: self.compiler_options.clone(),
            infos,
        };

        Ok(serde_json::to_string(&project_json)?)
    }
}
