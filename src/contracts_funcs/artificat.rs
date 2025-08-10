use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::{
    contracts_funcs::source_info::{SourceInfo, SourceKind},
    utils::fs::write_file,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Artifact {
    pub node_version: String,
    pub infos: HashMap<String, CodeInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeInfo {
    pub source_file: String,
    pub source_code_hash: String,
    pub bytecode_debug_patch: String,
    pub code_hash_debug: String,
}

fn get_parents_from_source(source_info: &SourceInfo, index: usize) -> Vec<String> {
    let source_code = &source_info.code_info.source_code;
    let from_index = if source_info.kind == SourceKind::Interface {
        source_code[index..]
            .find(&source_info.code_info.name)
            .map(|pos| pos + source_info.code_info.name.len())
    } else {
        source_code[index..].find(')').map(|pos| pos + index)
    };

    if let Some(from_index) = from_index {
        let to_index = source_code[from_index..]
            .find('{')
            .map(|pos| pos + from_index);
        if let Some(to_index) = to_index {
            return source_code[from_index + 1..to_index - 1]
                .replace(|c| c == '(' || c == ')', "")
                .replace("extends", ",")
                .replace("implements", ",")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }
    Vec::new()
}

fn fetch_dependencies(
    source_infos: &Vec<SourceInfo>,
    contract: &SourceInfo,
    parents_per_contract: &HashMap<String, Vec<String>>,
    dependencies: &mut HashMap<String, Vec<String>>,
    cache: &mut HashSet<String>,
) -> Result<()> {
    if cache.contains(&contract.code_info.name) {
        return Err(anyhow::anyhow!(
            "Circular dependency detected for contract: {}",
            contract.code_info.name
        ));
    }
    cache.insert(contract.code_info.name.clone());
    let mut all_parents = HashSet::new();

    if let Some(parents) = parents_per_contract.get(&contract.code_info.name) {
        for parent_id in parents {
            if let Some(parent) = source_infos.iter().find(|s| &s.code_info.name == parent_id) {
                all_parents.insert(parent_id.clone());
                if !dependencies.contains_key(parent_id) {
                    fetch_dependencies(
                        source_infos,
                        parent,
                        parents_per_contract,
                        dependencies,
                        cache,
                    )?;
                }
                if let Some(grand_parents) = dependencies.get(parent_id) {
                    for grand_parent in grand_parents {
                        all_parents.insert(grand_parent.clone());
                    }
                }
            }
        }
    }

    let all_parents_vec: Vec<String> = all_parents.into_iter().collect();
    dependencies.insert(contract.code_info.name.clone(), all_parents_vec.clone());
    Ok(())
}

fn get_dependencies(source_infos: &Vec<SourceInfo>) -> Result<HashMap<String, Vec<String>>> {
    let mut parents_per_contract = HashMap::<String, Vec<String>>::new();
    for source_info in source_infos {
        if let Some(from_index) = source_info.from_index {
            if matches!(
                source_info.kind,
                SourceKind::Contract | SourceKind::AbstractContract | SourceKind::Interface
            ) {
                let contract = &source_info.code_info.name;
                let parents = get_parents_from_source(source_info, from_index);
                parents_per_contract.insert(contract.clone(), parents);
            }
        }
    }

    let mut dependencies = HashMap::<String, Vec<String>>::new();
    let mut cache = HashSet::<String>::new();

    for source in source_infos {
        if matches!(
            source.kind,
            SourceKind::Contract | SourceKind::AbstractContract | SourceKind::Interface
        ) {
            if !dependencies.contains_key(&source.code_info.name) {
                fetch_dependencies(
                    source_infos,
                    source,
                    &parents_per_contract,
                    &mut dependencies,
                    &mut cache,
                )?;
            }
        }
    }

    Ok(dependencies)
}

impl Artifact {
    pub fn save(&self, path: &str) -> Result<()> {
        let path = format!("{}/project.json", path);
        let json = serde_json::to_string_pretty(self)?;
        write_file(&path, &json)
    }

    pub fn load(&self, path: &str) -> Result<Artifact> {
        let path = format!("{}/project.json", path);
        let content = crate::utils::fs::read_file(&path)?;
        let artifact: Artifact = serde_json::from_str(&content)?;
        Ok(artifact)
    }

    // Return the names of the sources who there content has changed, to trigger a recompilation
    pub fn get_changed_sources(&self, source_infos: &Vec<SourceInfo>) -> Result<Vec<String>> {
        let dependencies = get_dependencies(&source_infos)?;
        let mut result = HashSet::new();

        // Get all changed and new sources
        for source_info in source_infos {
            if let Some(info) = self.infos.get(&source_info.code_info.name) {
                if info.source_code_hash != source_info.code_info.source_code_hash {
                    result.insert(source_info.code_info.name.clone());
                    for (key, deps) in &dependencies {
                        if deps.contains(&source_info.code_info.name) {
                            result.insert(key.clone());
                        }
                    }
                }
            } else {
                result.insert(source_info.code_info.name.clone());
                for (key, deps) in &dependencies {
                    if deps.contains(&source_info.code_info.name) {
                        result.insert(key.clone());
                    }
                }
            }
        }

        // Get all removed sources
        for name in self.infos.keys() {
            if !source_infos.iter().any(|s| &s.code_info.name == name) {
                result.insert(name.clone());
                for (key, deps) in &dependencies {
                    if deps.contains(name) {
                        result.insert(key.clone());
                    }
                }
            }
        }

        Ok(result.into_iter().collect())
    }
}
