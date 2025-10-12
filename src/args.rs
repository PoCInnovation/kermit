use clap::{Parser, Subcommand, ValueHint};

use crate::{
    addresses::AddressesSubcommands, blockflow::BlockflowSubcommands,
    contracts::ContractsSubcommands, infos::InfosSubcommands, miners::MinersSubcommands,
    transactions::TransactionsSubcommands, utils::UtilsSubcommands, wallets::WalletsSubcommands,
};

#[derive(Parser)]
#[command(version)]
pub(crate) struct Kermit {
    #[clap(long, short, env, value_hint = ValueHint::Url,
    default_value = "http://localhost:22973")]
    pub url: String,

    #[clap(subcommand)]
    pub cmd: KermitSubcommand,
}

#[derive(Subcommand)]
pub(crate) enum KermitSubcommand {
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
}
