use anyhow::Result;
use clap::{Parser, ValueEnum};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug, ValueEnum)]
pub enum ContractType {
    Contract,
    Script,
    Project,
}

impl std::fmt::Display for ContractType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser)]
pub enum ContractsSubcommands {
    #[command(visible_alias = "c")]
    Compile {
        #[arg(short, long, default_value_t = ContractType::Contract)]
        contract_type: ContractType,
        file_path: String,
    },
    #[command(visible_alias = "s")]
    State {
        address: String,
    },
    // #[command(visible_alias = "co")]
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

impl ContractsSubcommands {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Compile {
                contract_type,
                file_path,
            } => {
                println!("Compiling contract of type: {}", contract_type);
                let mut file = File::open(file_path)?;
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;

                let client = Client::new();
                let url = "https://node.mainnet.alephium.org/contracts/compile-contract";

                let body = serde_json::json!({
                    "code": buffer,
                    "compilerOptions": {
                        "ignoreUnusedConstantsWarnings": true
                    }
                });

                let response = client.post(url).json(&body).send().await?;
                let json_response = response.json::<Value>().await?;

                println!("Contract: {:?}", json_response);
            }
            _ => todo!(),
        }

        Ok(())
    }
}
