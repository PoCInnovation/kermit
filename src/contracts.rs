use std::{fs::File, io::Read, path::Path};

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use regex::Regex;
use reqwest::Client;
use serde_json::{Value, json};
use strum::Display;

use crate::contract_encoding::_encode_contract_fields;

#[derive(Clone, Debug, Display, ValueEnum)]
pub enum ContractType {
    Contract,
    Script,
    Project,
}

#[derive(Clone, Debug, Display, ValueEnum)]
pub enum NetworkType {
    Main,
    Test,
    Dev,
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
        public_key: String,
        #[arg(long, default_value_t = NetworkType::Main)]
        network: NetworkType,
        compile_output_path: String,
        #[arg(long, default_value_t = ContractType::Project)]
        contract_type: ContractType,
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
    compiler_options_path: Option<&str>,
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
    let value: Value = response.json().await?;

    serde_json::to_writer_pretty(std::io::stdout(), &value)?;
    println!();

    Ok(())
}

pub async fn compile_project(
    url: &str,
    compiler_options_path: Option<&str>,
    file_path: &str,
) -> Result<()> {
    let re = Regex::new(r#"^import "[^"./]+/[^"]*[a-z][a-z_0-9]*(\.ral)?"#)?;

    let file_path = Path::new(file_path);
    let project_cwd = file_path
        .parent()
        .context("Invalid file path")?
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

            let path = path_buf.to_str().context("Invalid path")?;

            Ok(get_file_buffer(path)?)
        })
        .collect::<Result<Vec<_>>>()?;

    compile_file(
        url,
        compiler_options_path,
        &full_buffer.join("\n"),
        "/compile-project",
    )
    .await
}

pub async fn deploy_contract(
    url: &str,
    public_key: &str,
    network: NetworkType,
    compile_output_path: &str,
) -> Result<()> {
    let client = Client::new();
    let url = format!("{url}/contracts/unsigned-tx/deploy-contract");

    let compile_output: Value = {
        let mut file = File::open(&compile_output_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        serde_json::from_str(&buffer)?
    };

    let contracts = compile_output["contracts"]
        .as_array()
        .context("contracts Not an array")?
        .get(0)
        .context("no contracts in array");

    for contract in contracts.iter() {
        let byte_code = contract["bytecode"]
            .as_str()
            .context("Bytecode not found")?;
        let byte_code_debug = contract["bytecodeDebugPatch"]
            .as_str()
            .context("Bytecode not found")?;

        let fields = &contract["fields"];
        let final_byte_code =
            _encode_contract_fields(byte_code, byte_code_debug, &network, fields)?;

        let body = json!({
            "fromPublicKey": public_key,
            "bytecode": final_byte_code,
        });

        let response = client.post(&url).json(&body).send().await?;
        let json_response = response.json::<Value>().await?;

        println!("Deployment response: {:#?}", json_response);
    }

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
                        compiler_options_path.as_deref(),
                        &get_file_buffer(&file_path)?,
                        "/compile-contract",
                    )
                    .await?
                },
                ContractType::Project => {
                    compile_project(&url, compiler_options_path.as_deref(), &file_path).await?
                },
                ContractType::Script => {
                    compile_file(
                        &url,
                        compiler_options_path.as_deref(),
                        &get_file_buffer(&file_path)?,
                        "/compile-script",
                    )
                    .await?
                },
            },
            Self::Deploy {
                contract_type,
                public_key,
                network,
                compile_output_path,
            } => match contract_type {
                // Problems with devnet bytecode, doesn't deploy
                ContractType::Contract => {
                    deploy_contract(&url, &public_key, network, &compile_output_path).await?
                },
                _ => unimplemented!("Contract type not supported yet"),
            },
        }

        Ok(())
    }
}
