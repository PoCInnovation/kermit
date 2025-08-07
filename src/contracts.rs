use std::{fs::File, io::Read, path::Path};

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum, Args};
use regex::Regex;
use reqwest::Client;
use serde_json::{Value, json};
use strum::Display;

use crate::contracts_funcs::compile::compile;

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

#[derive(Debug, Clone, Args)]
pub struct IgnoreWarnings {
    #[arg(long, default_value_t = false, help = "Ignore external caller warnings")]
    pub ignore_check_external_caller_warnings: bool,

    #[arg(long, default_value_t = false, help = "Ignore unused constants warnings")]
    pub ignore_unused_constants_warnings: bool,

    #[arg(long, default_value_t = false, help = "Ignore unused fields warnings")]
    pub ignore_unused_fields_warnings: bool,

    #[arg(long, default_value_t = false, help = "Ignore unused function return warnings")]
    pub ignore_unused_function_return_warnings: bool,

    #[arg(long, default_value_t = false, help = "Ignore unused private functions warnings")]
    pub ignore_unused_private_functions_warnings: bool,

    #[arg(long, default_value_t = false, help = "Ignore unused variables warnings")]
    pub ignore_unused_variables_warnings: bool,

    #[arg(long, default_value_t = false, help = "Ignore update fields check warnings")]
    pub ignore_update_fields_check_warnings: bool,
}

#[derive(Parser)]
pub enum ContractsSubcommands {
    #[command(visible_alias = "c")]
    Compile {
        file_path: String,
        #[arg(long, default_value_t = NetworkType::Main)]
        network: NetworkType,
        #[arg(long, value_name = "config", help = "Path to the config YAML file")]
        config_path: Option<String>,
        #[command(flatten)]
        ignore_warnings: IgnoreWarnings,
        #[arg(long, help = "skip generate typescript code by contract artifacts", default_value_t = false)]
        skip_generate: bool,
        #[arg(long, help = "show detailed debug information such as error stack traces", default_value_t = false)]
        debug: bool,
        #[arg(long, help = "enable force recompile", default_value_t = false)]
        force: bool,
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

impl ContractsSubcommands {
    pub async fn run(self, url: String) -> Result<()> {
        match self {
            Self::Compile {
                file_path,
                network,
                config_path,
                ignore_warnings,
                skip_generate,
                debug,
                force,
            } => {
                return compile(
                    &file_path,
                    network,
                    config_path.as_deref(),
                    skip_generate,
                    debug,
                    force
                ).await;
            },
            Self::Deploy {
                contract_type,
                public_key,
                network,
                compile_output_path,
            } => {
                unimplemented!("Contract type not supported yet")
            },
        }
    }
}
