use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

use crate::{
    contracts::{CompilerOptions, NetworkType},
    network::health::is_network_alive,
    utils::post,
};
use once_cell::sync::Lazy;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(rename = "defaultSettings")]
    default_settings: DefaultSettings,
    configuration: Configuration,
}

#[derive(Debug, Deserialize)]
struct DefaultSettings {
    #[serde(rename = "issueTokenAmount")]
    issue_token_amount: u64,
    #[serde(rename = "openaiAPIKey")]
    openai_api_key: String,
    ipfs: Ipfs,
}

#[derive(Debug, Deserialize)]
struct Ipfs {
    infura: Infura,
}

#[derive(Debug, Deserialize)]
struct Infura {
    #[serde(rename = "projectId")]
    project_id: String,
    #[serde(rename = "projectSecret")]
    project_secret: String,
}

#[derive(Debug, Deserialize)]
struct Configuration {
    networks: Networks,
}

#[derive(Debug, Deserialize)]
struct Networks {
    devnet: Network,
    testnet: Network,
    mainnet: Network,
}

#[derive(Debug, Deserialize)]
struct Network {
    #[serde(rename = "nodeUrl")]
    node_url: String,
    #[serde(rename = "privateKeys")]
    private_keys: Vec<String>,
    settings: DefaultSettings,
}

fn load_ral_files(file_path: &str) -> Result<Vec<String>> {
    let dir = Path::new(file_path)
        .parent()
        .context("Failed to get parent directory of file_path")?;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
enum SourceKind {
    Contract,
    Script,
    AbstractContract,
    Interface,
    Struct,
    Constants,
}

impl Ord for SourceKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

static SOURCE_KIND_REGEX: Lazy<Result<HashMap<SourceKind, regex::Regex>, regex::Error>> =
    Lazy::new(|| {
        let mut m = HashMap::new();
        m.insert(
            SourceKind::AbstractContract,
            regex::Regex::new(r"^Abstract Contract ([A-Z][a-zA-Z0-9]*)")?,
        );
        m.insert(
            SourceKind::Contract,
            regex::Regex::new(r"^Contract ([A-Z][a-zA-Z0-9]*)")?,
        );
        m.insert(
            SourceKind::Interface,
            regex::Regex::new(r"^Interface ([A-Z][a-zA-Z0-9]*)")?,
        );
        m.insert(
            SourceKind::Script,
            regex::Regex::new(r"^TxScript ([A-Z][a-zA-Z0-9]*)")?,
        );
        m.insert(
            SourceKind::Struct,
            regex::Regex::new(r"struct ([A-Z][a-zA-Z0-9]*)")?,
        );
        Ok(m)
    });

struct SourceInfo {
    kind: SourceKind,
    name: String,
    from_index: Option<usize>,
    contract_relative_path: String,
    source_code: String,
    source_code_hash: String,
    is_external: bool,
}

impl SourceInfo {
    pub fn from(
        kind: SourceKind,
        name: String,
        from_index: Option<usize>,
        source_code: String,
        contract_relative_path: String,
        is_external: bool,
    ) -> Self {
        let source_code_hash = {
            let mut hasher = Sha256::new();
            hasher.update(source_code.as_bytes());
            format!("{:x}", hasher.finalize())
        };
        Self {
            kind,
            name,
            from_index,
            contract_relative_path,
            source_code,
            source_code_hash,
            is_external,
        }
    }
}

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

fn get_import_path(projectRootDir: &str, importPath: &str) -> String {
    importPath.to_string()
    // todo!("test multiple path, since some are relatives")
}

fn load_file(
    path: &str,
    contracts_relative_path: &str,
    import_file_paths_cache: &Vec<String>,
    is_imported: bool,
) -> Result<(Vec<SourceInfo>, Vec<String>)> {
    let file_content = fs::read_to_string(path).context("Failed to read file")?;

    let re = regex::Regex::new(r#"^import "[^"./]+/[^"]*[a-z][a-z_0-9]*(\.ral)?""#)
        .context("Failed to compile regex")?;

    let mut source_content = file_content.clone();
    let mut import_file_paths = Vec::new();

    for mat in re.find_iter(&file_content) {
        let mut import_path = mat.as_str()[8..].to_string(); // get rid of the 'import ...' line
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
    if regex::Regex::new(r#"^import ""#)?
        .find(&source_content)
        .is_some()
    {
        return Err(anyhow::anyhow!(
            "Invalid import statements, source: {}",
            path
        ));
    }

    let mut new_import_file_paths_cache = import_file_paths_cache
        .iter()
        .chain(import_file_paths.iter())
        .cloned()
        .collect::<Vec<String>>();

    let mut imported_source_infos = Vec::new();

    for import_path in &import_file_paths {
        let import_path = get_import_path(path, &import_path);
        if new_import_file_paths_cache.contains(&import_path) {
            continue;
        }

        new_import_file_paths_cache.push(import_path.clone());
        let (imported_source_info, _) = load_file(
            &import_path,
            &import_path,
            &new_import_file_paths_cache,
            true,
        )?;
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
    config_path: Option<&str>,
    compiler_options: CompilerOptions,
    skip_generate: bool,
    debug: bool,
    force: bool,
) -> Result<Value> {
    let config_path = config_path.as_ref().context("Config path is required")?;

    let config_content = std::fs::read_to_string(config_path)
        .context(format!("Failed to read config file: {}", config_path))?;

    let config: Config = serde_yaml::from_str(&config_content)
        .context(format!("Failed to parse YAML config: {}", config_path))?;

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
    let mut import_file_paths_cache = Vec::new();

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
