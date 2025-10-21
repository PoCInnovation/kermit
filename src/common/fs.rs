use std::{
    fs,
    io::{ErrorKind, Write},
};

use anyhow::{Context, Result};

pub fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path).context(match fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => {
            format!("Path points to a directory, not a file: {path}")
        },
        Err(e) if e.kind() == ErrorKind::NotFound => {
            format!("File does not exist at path: {path}")
        },
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            format!("No permission to read the file at path: {path}")
        },
        Ok(_) => "".to_string(),
        Err(_) => format!("Failed to access the file at path: {path}"),
    })
}

pub fn write_file(path: &str, contents: &str) -> Result<()> {
    let mut file = fs::File::create(path).context(match fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => {
            format!("Path points to a directory, not a file: {path}")
        },
        Err(e) if e.kind() == ErrorKind::NotFound => {
            format!("File does not exist at path: {path}")
        },
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            format!("No permission to write to the file at path: {path}")
        },
        Ok(_) => "".to_string(),
        Err(_) => format!("Failed to access the file at path: {path}"),
    })?;
    file.write_all(contents.as_bytes())
        .context(format!("Failed to write to file at path: {path}"))
}
