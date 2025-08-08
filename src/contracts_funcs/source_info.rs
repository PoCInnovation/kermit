use sha2::{Digest, Sha256};
use std::path::{MAIN_SEPARATOR, Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum SourceKind {
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

pub struct SourceInfo {
    pub kind: SourceKind,
    pub name: String,
    pub from_index: Option<usize>,
    pub contract_relative_path: String,
    pub source_code: String,
    pub source_code_hash: String,
    pub is_external: bool,
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

    pub fn get_artifact_path(&self, artifact_root_dir: &str) -> String {
        let relative_path = if self.is_external {
            let parts: Vec<&str> = self.contract_relative_path.split(MAIN_SEPARATOR).collect();
            let filtered: Vec<&str> = parts
                .iter()
                .skip_while(|&&p| p == ".." || p == ".")
                .cloned()
                .collect();
            let external_path =
                Path::new(".external").join(filtered.join(&MAIN_SEPARATOR.to_string()));
            Path::new(artifact_root_dir).join(external_path)
        } else {
            Path::new(artifact_root_dir).join(&self.contract_relative_path)
        };

        let dir = relative_path.parent().unwrap_or_else(|| Path::new(""));
        dir.join(format!("{}.ral.json", self.name))
            .to_string_lossy()
            .into_owned()
    }
}
