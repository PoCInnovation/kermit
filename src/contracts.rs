use anyhow::{Context, Result};
use clap::{Args, Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::Display;

use crate::{
    account::{
        address::Address,
        signature::{GLSecp256k1PrivateKey, PrivateKey},
    },
    config::config_struct::Config,
    contracts_funcs::{
        call::call_contract,
        compile::compile,
        compile_project::{
            compile_project_structs::{FieldsTypesMapMut, FieldsVec, load_compile_project},
            compile_project_values::config_fields_to_vec,
        },
        deploy::deploy_contract,
        state::{code, parent, state, sub_contracts, sub_contracts_current_count},
        test::test_contract,
    },
    network::health::check_network,
};

#[derive(Clone, Debug, Display, ValueEnum)]
pub enum CompiledType {
    Contract,
    Script,
}

#[derive(Clone, Copy, Debug, Display, ValueEnum, PartialEq, Eq)]
pub enum NetworkType {
    Main = 0,
    Test = 1,
    Dev = 4,
}

#[derive(Debug, Clone, Args, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    /// Ignore warnings related to external caller checks.
    #[arg(long, default_value_t = false)]
    pub ignore_check_external_caller_warnings: bool,

    /// Ignore warnings about unused constants in the code.
    #[arg(long, default_value_t = false)]
    pub ignore_unused_constants_warnings: bool,

    /// Ignore warnings about unused fields in the code.
    #[arg(long, default_value_t = false)]
    pub ignore_unused_fields_warnings: bool,

    /// Ignore warnings about unused function return values.
    #[arg(long, default_value_t = false)]
    pub ignore_unused_function_return_warnings: bool,

    /// Ignore warnings about unused private functions in the code.
    #[arg(long, default_value_t = false)]
    pub ignore_unused_private_functions_warnings: bool,

    /// Ignore warnings about unused variables in the code.
    #[arg(long, default_value_t = false)]
    pub ignore_unused_variables_warnings: bool,

    /// Ignore warnings related to update fields checks.
    #[arg(long, default_value_t = false)]
    pub ignore_update_fields_check_warnings: bool,

    /// Skip checks for abstract contracts.
    #[arg(long, default_value_t = false)]
    pub skip_abstract_contract_check: bool,

    /// Skip running tests during the compilation process.
    #[arg(long, default_value_t = false)]
    pub skip_tests: bool,
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid KEY=VALUE: '{s}'"));
    }
    if let (Some(key), Some(value)) = (parts.first(), parts.get(1)) {
        Ok((key.to_string(), value.to_string()))
    } else {
        Err("Failed to parse key-value pair".to_string())
    }
}

#[derive(Parser)]
pub enum ContractsSubcommands {
    #[command(visible_alias = "comp")]
    Compile {
        file_path: String,

        #[command(flatten)]
        compiler_options: CompilerOptions,

        /// Show detailed debug information such as error stack traces.
        #[arg(long, default_value_t = false)]
        debug: bool,

        /// Enable force recompile.
        #[arg(long, default_value_t = false)]
        force: bool,
    },
    #[command(visible_alias = "d")]
    Deploy {
        #[arg(long, value_enum, default_value_t = CompiledType::Contract)]
        compiled_type: CompiledType,
        contract_name: String,
        compile_output_path: String,
        #[arg(long, env)]
        private_key: String,

        /// "Amount of tokens to issue"
        #[arg(long, default_value_t = 100)]
        issue_token_amount: u64,
    },
    #[command(visible_alias = "s")]
    State {
        contract_id: String,
    },
    #[command(visible_alias = "t")]
    Test {
        contract_name: String,
        contract_id: String,
        compile_output_path: String,
        method_name: String,

        #[arg(long = "args", value_parser = parse_key_val, num_args = 1..)]
        args: Vec<(String, String)>,

        /// List of existing contracts to include in the test
        #[arg(long = "existing-contracts", value_name = "CONTRACT_ID", num_args = 0..)]
        exiting_contracts: Vec<String>,
    },
    #[command(visible_alias = "c")]
    Call {
        contract_name: String,
        contract_id: String,
        compile_output_path: String, // It assumes the user has the source code
        method_name: String,

        #[arg(long = "args", value_parser = parse_key_val, num_args = 1..)]
        args: Vec<(String, String)>,

        #[arg(long, env)]
        private_key: String,

        /// List of existing contracts to include in the test
        #[arg(value_name = "CONTRACT_ADDRESS", num_args = 0..)]
        interested_contracts: Vec<String>,

        /// Block hash to use for the call
        #[arg(long)]
        block_hash: Option<String>,
    },
    Code {
        code_hash: String,
    },
    Parent {
        address: String,
    },
    SubContracts {
        address: String,

        /// Start index for pagination
        #[arg(long, default_value_t = 0)]
        start: i32,

        /// Number of results to return
        #[arg(long)]
        limit: Option<i32>,
    },
    SubContractsCurrentCount {
        address: String,
    },
}

fn get_contract_initial_fields(
    config: &Config,
    contract_name: &str,
    contract_fields_types: &FieldsTypesMapMut,
) -> Result<FieldsVec> {
    let contracts_map = config
        .contracts
        .to_owned()
        .context("No 'contracts' field in config")?;
    let config_contract = contracts_map
        .get(contract_name)
        .context(format!("Contract '{contract_name}' not found in config"))?
        .to_owned();

    config_fields_to_vec(&config_contract.initial_fields, contract_fields_types)
}

impl ContractsSubcommands {
    pub async fn run(
        self,
        url: &str,
        config_file_path: &str,
        network_id: NetworkType,
        auto_create_config_file: bool,
    ) -> Result<()> {
        check_network(url).await?;

        let value: Value = match self {
            Self::Compile {
                file_path,
                compiler_options,
                debug,
                force,
            } => compile(url, &file_path, compiler_options, debug, force).await?,
            Self::Deploy {
                compiled_type,
                contract_name,
                compile_output_path,
                private_key,
                issue_token_amount,
            } => {
                let compiled_project = load_compile_project(&compile_output_path)?;
                let config = Config::new(config_file_path, auto_create_config_file)?;

                match compiled_type {
                    CompiledType::Contract => {
                        let contract = compiled_project.get_contract_by_name(&contract_name)?;

                        let initial_fields = get_contract_initial_fields(
                            &config,
                            &contract_name,
                            &contract.fields_types,
                        )?;

                        let private_key: Box<dyn PrivateKey> =
                            Box::new(GLSecp256k1PrivateKey::new(&private_key)?);

                        deploy_contract(
                            url,
                            private_key,
                            network_id,
                            contract,
                            initial_fields,
                            issue_token_amount,
                        )
                        .await?
                    },
                    CompiledType::Script => {
                        unimplemented!("Script deployment is not implemented yet");
                    },
                }
            },
            Self::State { contract_id } => state(url, &contract_id).await?,
            Self::Test {
                method_name,
                contract_name,
                contract_id,
                compile_output_path,
                args,
                exiting_contracts: existing_contracts,
            } => {
                let compiled_project = load_compile_project(&compile_output_path)?;
                let contract = compiled_project.get_contract_by_name(&contract_name)?;
                let config = Config::new(config_file_path, auto_create_config_file)?;

                let contracts_map = config
                    .contracts
                    .to_owned()
                    .context("No 'contracts' field in config")?;
                let config_contract = contracts_map
                    .get(&contract.name)
                    .context(format!("Contract '{contract_name}' not found in config"))?
                    .to_owned();

                let initial_fields =
                    config_fields_to_vec(&config_contract.initial_fields, &contract.fields_types)?;

                test_contract(
                    url,
                    &method_name,
                    (contract, &contract_id, existing_contracts),
                    initial_fields,
                    &config_contract.initial_asset,
                    &config_contract.input_assets,
                    args,
                )
                .await?
            },
            Self::Call {
                contract_name,
                contract_id,
                compile_output_path,
                method_name,
                args,
                private_key,
                interested_contracts,
                block_hash,
            } => {
                let compiled_project = load_compile_project(&compile_output_path)?;
                let contract = compiled_project.get_contract_by_name(&contract_name)?;
                let config = Config::new(config_file_path, auto_create_config_file)?;

                let contracts_map = config
                    .contracts
                    .to_owned()
                    .context("No 'contracts' field in config")?;
                let config_contract = contracts_map
                    .get(&contract.name)
                    .context(format!("Contract '{contract_name}' not found in config"))?
                    .to_owned();

                let private_key: Box<dyn PrivateKey> =
                    Box::new(GLSecp256k1PrivateKey::new(&private_key)?);
                let address = Address::new(&private_key.get_public_key()?)?;

                call_contract(
                    url,
                    &method_name,
                    (contract, &contract_id, interested_contracts),
                    &address,
                    &config_contract.input_assets,
                    args,
                    block_hash,
                )
                .await?
            },
            Self::Code { code_hash } => code(url, &code_hash).await?,
            Self::Parent { address } => parent(url, &address).await?,
            Self::SubContracts {
                address,
                start,
                limit,
            } => sub_contracts(url, &address, start, limit).await?,
            Self::SubContractsCurrentCount { address } => {
                sub_contracts_current_count(url, &address).await?
            },
        };

        serde_json::to_writer_pretty(std::io::stdout(), &value)?;
        println!();
        Ok(())
    }
}
