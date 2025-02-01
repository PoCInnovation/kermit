use anyhow::Result;
use clap::{Parser, ValueEnum};
use reqwest::Client;
use serde::Deserialize;


#[derive(Clone, Debug, ValueEnum)]
pub enum ContractType {
    Contract,
    Script,
    Project,
}

#[derive(Parser)]
pub enum ContractsSubcommands {
    #[command(visible_alias = "c")]
    Compile { // le code est dans un string, l'importer à partir d'un path
        contract_type: Option<ContractType>,
    },
    #[command(visible_alias = "s")]
    State {
        address: String,
    },
    #[command(visible_alias = "code")]
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
    }
}

impl ContractsSubcommands {
    pub async fn run(self) -> Result<()> {
        match self {
            _ => todo!()
        }

        Ok(())
    }
}