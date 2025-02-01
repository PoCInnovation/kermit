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

pub async fn compile_contract(
    url: String,
    compiler_options_path: Option<String>,
    file_path: String,
) -> Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let client = Client::new();
    let url = format!("{url}/contracts/compile-contract");

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
        "code": buffer,
        "compilerOptions": compiler_options
    });

    let response = client.post(url).json(&body).send().await?;
    let json_response = response.json::<Value>().await?;

    println!("Contract: {:#?}", json_response);
    Ok(())
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
                    compile_contract(url, compiler_options_path, file_path).await?
                },
                _ => todo!("Contract type not yet implemented"),
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
