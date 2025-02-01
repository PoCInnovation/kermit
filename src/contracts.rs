use anyhow::anyhow;
use anyhow::Result;
use clap::{Parser, ValueEnum};
use regex::Regex;
use reqwest::Client;
use serde_json::{json, Value};
use strum::Display;

use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug, Display, ValueEnum)]
pub enum ContractType {
    Contract,
    Script,
    Project,
}

#[derive(Parser)]
pub enum ContractsSubcommands {
    #[command(visible_alias = "c")]
    Compile {
        file_path: String,
        #[arg(long, default_value_t = ContractType::Project)]
        contract_type: ContractType,
        #[arg(long)]
        compiler_options_path: Option<String>,
    },
    #[command(visible_alias = "d")]
    Deploy {
        #[arg(short, long, default_value_t = ContractType::Project)]
        contract_type: ContractType,
        public_key: String, // todo: regex if this is a public key
        #[arg(short, long)]
        byte_code: String,
    },
    #[command(visible_alias = "s")]
    State {
        address: String,
    },
    Code {
        address: String,
    },
    Test {
        // body
    },
    Call {
        // body
        #[arg(short, long)]
        multiple: Option<bool>,
    },
    Parent {
        address: String,
    },
    SubContracts {
        address: String,
        #[arg(short, long)]
        current_count: Option<bool>,
    },
    CallTxScript {
        // body
    },
}

fn get_file_buffer(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub async fn compile_file(
    url: &str,
    compiler_options_path: Option<String>,
    file_buffer: &str,
    end_point: &str,
) -> Result<()> {
    let client = Client::new();
    let url = format!("{url}/contracts{end_point}");

    let compiler_options = match compiler_options_path {
        Some(path) => {
            let mut option_file = File::open(&path)?;
            let mut buffer = String::new();
            option_file.read_to_string(&mut buffer)?;
            buffer.into()
        },
        None => {
            json!({
                "ignoreUnusedConstantsWarnings": true
            })
        },
    };

    let body = json!({
        "code": file_buffer,
        "compilerOptions": compiler_options
    });

    let response = client.post(url).json(&body).send().await?;
    let json_response = response.json::<Value>().await?;

    println!("Contract: {:#?}", json_response);
    Ok(())
}

pub async fn compile_project(
    url: &str,
    compiler_options_path: Option<String>,
    file_path: &str,
) -> Result<()> {
    let re = Regex::new(r#"^import "[^"./]+/[^"]*[a-z][a-z_0-9]*(\.ral)?"#)?;

    let file_path: &std::path::Path = std::path::Path::new(file_path);
    let project_cwd = file_path
        .parent()
        .ok_or_else(|| anyhow!("Invalid file path"))?
        .canonicalize()?;

    let mut buffer = String::new();
    File::open(file_path)?.read_to_string(&mut buffer)?;

    let full_buffer = buffer
        .lines()
        .into_iter()
        .map(|line| {
            if !re.is_match(line) {
                return Ok(line.to_string());
            }
            let trimmed = line.trim();
            let line = trimmed.split_whitespace().collect::<Vec<&str>>();

            let import_file = if let Some(second) = line.get(1) {
                second.to_string().trim_matches('"').to_string()
            } else {
                String::new()
            };

            // handle with missing .ral, concat it

            let path_buf = if import_file.starts_with("std") {
                std::env::current_dir()?.join("contracts").join(import_file)
            } else {
                project_cwd.join(import_file).into()
            };

            let path = path_buf.to_str().ok_or_else(|| anyhow!("Invalid path"))?;

            Ok(get_file_buffer(path)?)
        })
        .collect::<Result<Vec<String>, anyhow::Error>>()?;

    compile_file(
        url,
        compiler_options_path,
        &full_buffer.join("\n"),
        "/compile-project",
    )
    .await
}

pub async fn deploy_contract(url: String, public_key: String, byte_code: String) -> Result<()> {
    let public_key_regex = Regex::new(r"^[a-f0-9]{64}$")?;
    if !public_key_regex.is_match(&public_key) {
        return Err(anyhow::anyhow!("Invalid public key format"));
    }

    let client = Client::new();
    let url = format!("{url}/contracts/unigned-tx/deploy-contract");

    let byte_code = if let Ok(mut file) = File::open(&byte_code) {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        buffer
    } else {
        byte_code
    };

    let body = json!({
        "publicKey": public_key,
        "byteCode": byte_code,
    });

    let response = client.post(url).json(&body).send().await?;
    let json_response = response.json::<Value>().await?;

    println!("Deployment response: {:#?}", json_response);
    Ok(())
}

impl ContractsSubcommands {
    pub async fn run(self, url: String) -> Result<()> {
        match self {
            Self::Compile {
                contract_type,
                compiler_options_path,
                file_path,
            } => match contract_type {
                ContractType::Contract => {
                    compile_file(
                        &url,
                        compiler_options_path,
                        &get_file_buffer(&file_path)?,
                        "/compile-contract",
                    )
                    .await?
                },
                ContractType::Project => {
                    compile_project(&url, compiler_options_path, &file_path).await?
                },
                ContractType::Script => {
                    compile_file(
                        &url,
                        compiler_options_path,
                        &get_file_buffer(&file_path)?,
                        "/compile-script",
                    )
                    .await?
                },
            },
            Self::Deploy {
                contract_type,
                public_key,
                byte_code,
            } => match contract_type {
                ContractType::Contract => deploy_contract(url, public_key, byte_code).await?,
                _ => todo!("Contract type not yet deployable"),
            },
            _ => todo!(),
        }

        Ok(())
    }
}
