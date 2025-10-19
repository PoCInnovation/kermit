use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{MAIN_SEPARATOR, MAIN_SEPARATOR_STR, Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SourceKind {
    Contract,
    Script,
    AbstractContract,
    Interface,
    Struct,
    Constants,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CodeInfo {
    pub name: String,
    pub source_code: String,
    pub source_code_hash: String,
    pub bytecode_debug_patch: Option<String>,
    pub code_hash_debug: Option<String>,
    pub contract_relative_path: String,
}

#[allow(dead_code)]
pub struct SourceInfo {
    pub kind: SourceKind,
    pub from_index: Option<usize>,
    pub is_external: bool,
    pub code_info: CodeInfo,
}

#[allow(dead_code)]
impl SourceInfo {
    pub fn new(
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
            from_index,
            is_external,
            code_info: CodeInfo {
                name,
                source_code,
                source_code_hash,
                bytecode_debug_patch: None,
                code_hash_debug: None,
                contract_relative_path,
            },
        }
    }

    pub fn get_artifact_path(&self, artifact_root_dir: &str) -> String {
        let relative_path = if self.is_external {
            let parts: Vec<&str> = self
                .code_info
                .contract_relative_path
                .split(MAIN_SEPARATOR)
                .collect();
            let filtered: Vec<&str> = parts
                .iter()
                .skip_while(|&&p| p == ".." || p == ".")
                .cloned()
                .collect();
            let external_path = Path::new(".external").join(filtered.join(MAIN_SEPARATOR_STR));
            Path::new(artifact_root_dir).join(external_path)
        } else {
            Path::new(artifact_root_dir).join(&self.code_info.contract_relative_path)
        };

        let dir = relative_path.parent().unwrap_or_else(|| Path::new(""));
        dir.join(format!("{}.ral.json", self.code_info.name))
            .to_string_lossy()
            .into_owned()
    }
}
