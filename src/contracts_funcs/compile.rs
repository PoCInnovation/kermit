use anyhow::{Context, Result};
use regex::{Error, Regex, RegexBuilder};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

use crate::{
    contracts::{CompilerOptions, NetworkType},
    contracts_funcs::source_info::{SourceInfo, SourceKind},
    network::{health::is_network_alive, node::Config},
    utils::{fs::read_file, post},
};
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

fn load_ral_files(file_path: &str) -> Result<Vec<String>> {
    let dir = Path::new(file_path);

    let mut ral_files_path = Vec::new();
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_name = entry.file_name().to_string_lossy();
        if file_name.contains(".ral") {
            ral_files_path.push(entry.path().to_string_lossy().to_string());
        }
    }
    Ok(ral_files_path)
}

static SOURCE_KIND_REGEX: Lazy<Result<HashMap<SourceKind, Regex>, Error>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        SourceKind::AbstractContract,
        RegexBuilder::new(r"^Abstract Contract ([A-Z][a-zA-Z0-9]*)")
            .multi_line(true)
            .build()?,
    );
    m.insert(
        SourceKind::Contract,
        RegexBuilder::new(r"^Contract ([A-Z][a-zA-Z0-9]*)")
            .multi_line(true)
            .build()?,
    );
    m.insert(
        SourceKind::Interface,
        RegexBuilder::new(r"^Interface ([A-Z][a-zA-Z0-9]*)")
            .multi_line(true)
            .build()?,
    );
    m.insert(
        SourceKind::Script,
        RegexBuilder::new(r"^TxScript ([A-Z][a-zA-Z0-9]*)")
            .multi_line(true)
            .build()?,
    );
    m.insert(
        SourceKind::Struct,
        RegexBuilder::new(r"struct ([A-Z][a-zA-Z0-9]*)")
            .multi_line(true)
            .build()?,
    );
    Ok(m)
});

fn get_source_info(
    source_code: String,
    contracts_relative_path: &str,
    is_imported: bool,
) -> Result<Vec<SourceInfo>> {
    let mut source_infos = Vec::new();
    let source_kind_regex = SOURCE_KIND_REGEX
        .as_ref()
        .map_err(|e| anyhow::anyhow!("Failed to compile regex: {}", e))?;

    for (kind, regex) in source_kind_regex.iter() {
        for cap in regex.captures_iter(&source_code) {
            let name = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .context("Invalid match")?;
            let from_index = cap.get(1).map(|m| m.start());
            let source_info = SourceInfo::from(
                *kind,
                name,
                from_index,
                source_code.clone(),
                contracts_relative_path.to_string(), // contract_relative_path, adjust as needed
                is_imported,
            );
            source_infos.push(source_info);
        }
    }

    if source_infos.is_empty() {
        let name = Path::new(contracts_relative_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Failed to extract file stem for constants")?
            .to_string();

        source_infos.push(SourceInfo::from(
            SourceKind::Constants,
            name,
            None,
            source_code.clone(),
            contracts_relative_path.to_string(),
            false,
        ));
    }
    Ok(source_infos)
}

fn get_import_path(import_path: &str) -> Result<String> {
    let parts: Vec<&str> = import_path.split('/').collect();
    if parts.len() > 1 && parts[0] == "std" {
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        return Ok(Path::new(&current_dir)
            .join(["contracts", import_path].iter().collect::<PathBuf>())
            .to_string_lossy()
            .to_string());
    }

    Ok(import_path.to_string())
}

fn load_file(
    path: &str,
    contracts_relative_path: &str,
    import_file_paths_cache: &HashSet<String>,
    is_imported: bool,
) -> Result<(Vec<SourceInfo>, HashSet<String>)> {
    let file_content = read_file(path)?;

    let re = Regex::new(r#"^import "[^"./]+/[^"]*[a-z][a-z_0-9]*(\.ral)?""#)
        .context("Failed to compile regex")?;

    let mut source_content = file_content.clone();
    let mut import_file_paths = Vec::new();

    for mat in re.find_iter(&file_content) {
        let mut import_path = mat.as_str()[8..mat.as_str().len() - 1].to_string(); // get rid of the "import ..." chunk, as well as the final quote
        if !import_path.ends_with(".ral") {
            import_path.push_str(".ral");
        }
        if !import_file_paths_cache.contains(&import_path) {
            import_file_paths.push(import_path);
        }
        // remove the import string
        source_content.replace_range(mat.start()..mat.end(), "");
    }

    // these are incomplete import statements
    if Regex::new(r#"^import ""#)?.find(&source_content).is_some() {
        return Err(anyhow::anyhow!(
            "Invalid import statements, source: {}",
            path
        ));
    }

    let mut new_import_file_paths_cache = import_file_paths_cache.clone();
    let mut imported_source_infos = Vec::new();

    for import_path in &import_file_paths {
        let import_path = get_import_path(&import_path)?;
        if new_import_file_paths_cache.contains(&import_path) {
            continue;
        }

        new_import_file_paths_cache.insert(import_path.clone());
        let (imported_source_info, loaded_file_import_cache) = load_file(
            &import_path,
            &import_path,
            &new_import_file_paths_cache,
            true,
        )?;
        new_import_file_paths_cache.extend(loaded_file_import_cache);
        imported_source_infos.extend(imported_source_info);
    }

    let mut source_infos =
        get_source_info(source_content.clone(), contracts_relative_path, is_imported)?;
    source_infos.extend(imported_source_infos);

    Ok((source_infos, new_import_file_paths_cache))
}

pub async fn compile(
    url: &str,
    file_path: &str,
    network: NetworkType,
    config_path: &str,
    compiler_options: CompilerOptions,
    skip_generate: bool,
    debug: bool,
    force: bool,
) -> Result<Value> {
    let config_content = read_file(config_path)?;

    let config = serde_yaml::from_str::<serde_yaml::Value>(&config_content);
    if config.is_err() {
        return Err(anyhow::anyhow!(
            "Failed to parse config file: {} : {}",
            config_path,
            config.unwrap_err()
        ));
    }

    let mut config = config?;
    config.apply_merge()?;
    let config = serde_yaml::from_value::<Config>(config);
    if config.is_err() {
        return Err(anyhow::anyhow!(
            "Failed to parse config file: {} : {}",
            config_path,
            config.unwrap_err()
        ));
    }

    let config = config?;
    let network_url = match network {
        NetworkType::Dev => &config.configuration.networks.devnet.node_url,
        NetworkType::Test => &config.configuration.networks.testnet.node_url,
        NetworkType::Main => &config.configuration.networks.mainnet.node_url,
    };

    if !is_network_alive(&network_url).await? {
        return Err(anyhow::anyhow!("Network is not reachable: {}", network_url));
    }

    let source_file_paths = load_ral_files(file_path)?;
    let mut all_source_infos = Vec::new();
    let mut import_file_paths_cache = HashSet::new();

    for path in &source_file_paths {
        let (source_infos, new_cache) = load_file(path, path, &import_file_paths_cache, false)?;
        all_source_infos.extend(source_infos);
        import_file_paths_cache = new_cache;
    }

    if all_source_infos.is_empty() {
        return Err(anyhow::anyhow!(
            "No valid source information found in the provided files."
        ));
    }

    // Ensure there is at least one Contract or Script
    if !all_source_infos
        .iter()
        .any(|info| matches!(info.kind, SourceKind::Contract | SourceKind::Script))
    {
        return Err(anyhow::anyhow!(
            "No Contract or Script found in the provided project files."
        ));
    }

    // Remove duplicate code
    all_source_infos.sort_by_key(|info| info.source_code_hash.clone());
    all_source_infos.dedup_by_key(|info| info.source_code_hash.clone());

    // The sorting will be used to concat the sources to compile the contract
    all_source_infos.sort_by_key(|info| info.kind);

    let concatenated_code = all_source_infos
        .iter()
        .map(|info| info.source_code.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(post(
        url,
        "/contracts/compile-project",
        json!({
            "code": concatenated_code,
            "compiler_options": json!(compiler_options)
        }),
    )
    .await?
    .data)
}
