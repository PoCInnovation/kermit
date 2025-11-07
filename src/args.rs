use clap::{Parser, Subcommand, ValueHint};
use clap_complete::Shell;

use crate::{
    addresses::AddressesSubcommands,
    blockflow::BlockflowSubcommands,
    contracts::{ContractsSubcommands, NetworkType},
    infos::InfosSubcommands,
    miners::MinersSubcommands,
    transactions::TransactionsSubcommands,
    utils::UtilsSubcommands,
    wallets::WalletsSubcommands,
};

#[derive(Parser)]
#[command(version)]
pub struct Kermit {
    #[clap(long, short, env, value_hint = ValueHint::Url,
    default_value = "http://localhost:22973")]
    pub url: String,

    #[clap(subcommand)]
    pub cmd: KermitSubcommand,
}

#[derive(Subcommand)]
pub enum KermitSubcommand {
    /// Address management utilities.
    #[command(visible_alias = "a")]
    Addresses {
        #[command(subcommand)]
        command: AddressesSubcommands,
    },

    /// Blockflow data retrieval utilities.
    #[command(visible_alias = "b")]
    Blockflow {
        #[command(subcommand)]
        command: BlockflowSubcommands,
    },

    /// Contract management utilities.
    #[command(visible_alias = "c")]
    Contracts {
        #[command(subcommand)]
        command: ContractsSubcommands,

        /// Path to the config YAML file
        #[arg(long, short, default_value = "./alephium.config.yaml")]
        config_file_path: String,

        /// Create the config file for contracts automatically if not set
        #[arg(long, default_value_t = false)]
        auto_create_config_file: bool,

        /// Network type may trigger a different behavior in contract
        /// operations. Choose accordingly
        #[arg(long, short, value_enum, default_value_t = NetworkType::Dev)]
        network: NetworkType,
    },

    /// Infos about node and hashrate.
    #[command(visible_alias = "i")]
    Infos {
        #[command(subcommand)]
        command: InfosSubcommands,
    },

    /// Miners management utilities.
    #[command(visible_alias = "m")]
    Miners {
        #[command(subcommand)]
        command: MinersSubcommands,
    },

    /// Utilities (Address zero, Hash zero, Conversion atto).
    #[command(visible_alias = "u")]
    Utils {
        #[command(subcommand)]
        command: UtilsSubcommands,
    },

    /// Transactions management utilities
    #[command(visible_alias = "tx")]
    Transactions {
        #[command(subcommand)]
        command: TransactionsSubcommands,
    },

    /// Wallet management utilities.
    #[command(visible_alias = "w")]
    Wallets {
        #[command(subcommand)]
        command: WalletsSubcommands,
    },

    /// Generate shell autocompletion script for your current shell
    #[command(visible_alias = "gen-autocpl")]
    GenerateAutocompletion {
        /// Shell to generate completion for. If omitted, the program should try to
        /// detect the current shell at runtime (using env vars).
        #[arg(value_enum)]
        shell: Option<Shell>,
    },
}
