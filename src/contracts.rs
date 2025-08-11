use std::collections::HashMap;

use anyhow::{Context, Result};
use clap::{Args, Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::Display;

use crate::{
    account::signature::{GLSecp256k1PrivateKey, PrivateKey},
    contracts_funcs::{
        compile::{compile, get_compiled_project},
        deploy::deploy_contract,
        project::Project,
    },
    utils::fs::read_file,
};

#[derive(Clone, Debug, Display, ValueEnum)]
pub enum CompiledType {
    Contract,
    Script,
}

#[derive(Clone, Copy, Debug, Display, ValueEnum, PartialEq, Eq)]
pub enum NetworkType {
    Main,
    Test,
    Dev,
}

#[derive(Debug, Clone, Args, Serialize, Deserialize)]
pub struct CompilerOptions {
    #[arg(
        long,
        default_value_t = false,
        help = "Ignore external caller warnings"
    )]
    pub ignore_check_external_caller_warnings: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Ignore unused constants warnings"
    )]
    pub ignore_unused_constants_warnings: bool,

    #[arg(long, default_value_t = false, help = "Ignore unused fields warnings")]
    pub ignore_unused_fields_warnings: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Ignore unused function return warnings"
    )]
    pub ignore_unused_function_return_warnings: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Ignore unused private functions warnings"
    )]
    pub ignore_unused_private_functions_warnings: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Ignore unused variables warnings"
    )]
    pub ignore_unused_variables_warnings: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Ignore update fields check warnings"
    )]
    pub ignore_update_fields_check_warnings: bool,

    #[arg(long, default_value_t = false, help = "Skip abstract contract check")]
    pub skip_abstract_contract_check: bool,

    #[arg(long, default_value_t = false, help = "Skip tests")]
    pub skip_tests: bool,
}

#[derive(Parser)]
pub enum ContractsSubcommands {
    #[command(visible_alias = "c")]
    Compile {
        file_path: String,
        #[arg(long, default_value_t = NetworkType::Main)]
        network: NetworkType,
        #[arg(long, value_name = "config", help = "Path to the config YAML file", default_value_t=String::from("./alephium.config.yaml"))]
        config_path: String,
        #[command(flatten)]
        compiler_options: CompilerOptions,
        #[arg(
            long,
            help = "skip generate typescript code by contract artifacts",
            default_value_t = false
        )]
        skip_generate: bool,
        #[arg(
            long,
            help = "show detailed debug information such as error stack traces",
            default_value_t = false
        )]
        debug: bool,
        #[arg(long, help = "enable force recompile", default_value_t = false)]
        force: bool,
    },
    #[command(visible_alias = "d")]
    Deploy {
        #[arg(long, default_value_t = NetworkType::Main)]
        network: NetworkType,
        #[arg(long, default_value_t = CompiledType::Contract)]
        compiled_type: CompiledType,
        #[arg(help = "Index in the list of the wanted compiled object")]
        compiled_index: u32,
        compile_output_path: String,
        project_path: String,
        config_file_path: Option<String>,
        #[arg(long, env)]
        private_key: Option<String>,
    },
}

impl ContractsSubcommands {
    pub async fn run(self, url: &str) -> Result<()> {
        let value: Value = match self {
            Self::Compile {
                file_path,
                network,
                config_path,
                compiler_options,
                skip_generate,
                debug,
                force,
            } => {
                compile(
                    url,
                    &file_path,
                    network,
                    &config_path,
                    compiler_options,
                    skip_generate,
                    debug,
                    force,
                )
                .await?
            },
            Self::Deploy {
                network,
                compiled_type,
                compiled_index,
                compile_output_path,
                project_path,
                config_file_path,
                private_key,
            } => {
                let _project = Project::try_from(read_file(&project_path)?.as_str())?;
                let compiled_project = get_compiled_project(&compile_output_path)?;

                match compiled_type {
                    CompiledType::Contract => {
                        let contract = compiled_project
                            .contracts
                            .get(compiled_index as usize)
                            .context("Invalid compiled index")?;
                        let initial_fields = HashMap::new();

                        let private_key = if let Some(path) = private_key {
                            let private_key: Box<dyn PrivateKey> =
                                Box::new(GLSecp256k1PrivateKey::new(&path)?);
                            Some(private_key)
                        } else {
                            None
                        };

                        deploy_contract(
                            url,
                            private_key,
                            network,
                            &config_file_path
                                .unwrap_or_else(|| String::from("./alephium.config.yaml")),
                            contract.clone(),
                            initial_fields,
                        )
                        .await?
                    },
                    CompiledType::Script => {
                        unimplemented!("Script deployment is not implemented yet");
                    },
                }
            },
        };

        serde_json::to_writer_pretty(std::io::stdout(), &value)?;
        println!();
        Ok(())
    }
}
