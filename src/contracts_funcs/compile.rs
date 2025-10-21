use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::{Context, Result, anyhow, bail};
use regex::{Error, Regex, RegexBuilder};
use serde_json::{Value, json};

use crate::{
    common::{fs::read_file, post},
    contracts::CompilerOptions,
    contracts_funcs::source_info::{SourceInfo, SourceKind},
};

fn load_ral_files(compile_path: &str) -> Result<(Vec<String>, String)> {
    let path = Path::new(compile_path);
    if path.is_file() {
        let dir = path
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .context("Failed to get parent directory")?;
        return Ok((vec![compile_path.to_string()], dir));
    }

    let dir = Path::new(compile_path);

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
    Ok((ral_files_path, compile_path.to_string()))
}

static SOURCE_KIND_REGEX: LazyLock<Result<HashMap<SourceKind, Regex>, Error>> =
    LazyLock::new(|| {
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
    source_code: &str,
    contracts_relative_path: &str,
    is_imported: bool,
) -> Result<Vec<SourceInfo>> {
    let mut source_infos = Vec::new();
    let source_kind_regex = SOURCE_KIND_REGEX
        .as_ref()
        .map_err(|e| anyhow!("Failed to compile regex: {e}"))?;

    for (kind, regex) in source_kind_regex.iter() {
        for cap in regex.captures_iter(source_code) {
            let name = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .context("Invalid match")?;
            let from_index = cap.get(1).map(|m| m.start());
            let source_info = SourceInfo::new(
                *kind,
                name,
                from_index,
                source_code.to_owned(),
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

        source_infos.push(SourceInfo::new(
            SourceKind::Constants,
            name,
            None,
            source_code.to_owned(),
            contracts_relative_path.to_string(),
            false,
        ));
    }
    Ok(source_infos)
}

fn get_import_path(cwd_path: &str, import_path: &str) -> Result<String> {
    let parts: Vec<&str> = import_path.split('/').collect();
    if parts.len() > 1 && parts[0] == "std" {
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        return Ok(Path::new(&current_dir)
            .join(["contracts", import_path].iter().collect::<PathBuf>())
            .to_string_lossy()
            .to_string());
    }

    Ok(format!(
        "{}/{}",
        cwd_path.trim_end_matches('/'),
        import_path.trim_start_matches('/')
    ))
}

static REGEX_GET_IMPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^import\s+"[^"]+"$"#).unwrap());
static REGEX_CHECK_IMPORT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^import ""#).unwrap());

fn load_file(
    path: &str,
    contracts_relative_path: &str,
    import_file_paths_cache: &HashSet<String>,
    is_imported: bool,
) -> Result<(Vec<SourceInfo>, HashSet<String>)> {
    let file_content = read_file(path)?;

    let mut source_content = file_content.clone();
    let mut import_file_paths = vec![];
    let mut import_statements_range = vec![];

    let mut character_index: usize = 0;
    for line in file_content.lines() {
        if let Some(mat) = REGEX_GET_IMPORT.find(line) {
            let import_stmt = mat.as_str();
            let mut import_path = import_stmt[8..import_stmt.len() - 1].to_string();
            if !import_path.ends_with(".ral") {
                import_path.push_str(".ral");
            }
            if !import_file_paths_cache.contains(&import_path) {
                import_file_paths.push(import_path);
            }

            let start = character_index + mat.start();
            let end = character_index + mat.end();
            import_statements_range.push(start..end);
        }
        character_index += line.len() + 1; // +1 for the newline character
    }

    // remove from the end to avoid messing up the indices
    for mat in import_statements_range.iter().rev() {
        source_content.replace_range(mat.to_owned(), "");
    }

    for (i, line) in source_content.lines().enumerate() {
        if REGEX_CHECK_IMPORT.find(line).is_some() {
            bail!(
                "Invalid import statements, source: {} (line {})",
                path,
                i + 1
            );
        }
    }

    let mut new_import_file_paths_cache = import_file_paths_cache.clone();
    let mut imported_source_infos = Vec::new();

    for import_path in &import_file_paths {
        let import_path = get_import_path(contracts_relative_path, import_path)?;
        if new_import_file_paths_cache.contains(&import_path) {
            continue;
        }

        new_import_file_paths_cache.insert(import_path.clone());
        let (imported_source_info, loaded_file_import_cache) = load_file(
            &import_path,
            contracts_relative_path,
            &new_import_file_paths_cache,
            true,
        )?;
        new_import_file_paths_cache.extend(loaded_file_import_cache);
        imported_source_infos.extend(imported_source_info);
    }

    let mut source_infos = get_source_info(&source_content, contracts_relative_path, is_imported)?;
    source_infos.extend(imported_source_infos);

    Ok((source_infos, new_import_file_paths_cache))
}

pub async fn compile(
    url: &str,
    compile_path: &str,
    compiler_options: CompilerOptions,
    _debug: bool, // TODO: handle debug and force compilation
    _force: bool,
) -> Result<Value> {
    let (source_file_paths, compile_path) = load_ral_files(compile_path)?;
    let mut all_source_infos = Vec::new();
    let mut import_file_paths_cache = HashSet::from([compile_path.clone()]);

    for path in &source_file_paths {
        let (source_infos, new_cache) =
            load_file(path, &compile_path, &import_file_paths_cache, false)?;
        all_source_infos.extend(source_infos);
        import_file_paths_cache = new_cache;
    }

    if all_source_infos.is_empty() {
        bail!("No valid source information found in the provided files.");
    }

    // Ensure there is at least one Contract or Script
    if !all_source_infos
        .iter()
        .any(|info| matches!(info.kind, SourceKind::Contract | SourceKind::Script))
    {
        bail!("No Contract or Script found in the provided project files.");
    }

    // Remove duplicate code
    all_source_infos.sort_by_key(|info| info.code_info.source_code_hash.clone());
    all_source_infos.dedup_by_key(|info| info.code_info.source_code_hash.clone());

    all_source_infos.sort_by_key(|info| info.kind);

    let concatenated_code = all_source_infos
        .iter()
        .map(|info| info.code_info.source_code.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    post(
        url,
        "/contracts/compile-project",
        json!({
            "code": concatenated_code,
            "compiler_options": json!(compiler_options)
        }),
    )
    .await?
    .context("Empty reply")
}
