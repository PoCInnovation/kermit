use clap::{Parser, Subcommand, ValueHint};

use crate::{
    address::AddressSubcommands, contracts::ContractsSubcommands, wallet::WalletsSubcommands,
};
use crate::events::EventsSubcommands;
use crate::infos::InfosSubcommands;
use crate::transactions::TransactionsSubcommands;

#[derive(Parser)]
#[command(version)]
pub struct Kermit {
    #[clap(long, short, env, value_hint = ValueHint::Url,
    default_value = "https://node.mainnet.alephium.org")]
    pub url: String,

    #[clap(subcommand)]
    pub cmd: KermitSubcommand,
}

#[derive(Subcommand)]
pub enum KermitSubcommand {
    /// Event for contract, block and hash.
    #[command(visible_alias = "e")]
    Events {
        #[command(subcommand)]
        command: EventsSubcommands,
    },

    /// Infos about node and hashrate.
    #[command(visible_alias = "i")]
    Infos {
        #[command(subcommand)]
        command: InfosSubcommands,
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
    #[command(visible_alias = "a")]
    Address {
        #[command(subcommand)]
        command: AddressSubcommands,
    },
    #[command(visible_alias = "c")]
    Contracts {
        #[command(subcommand)]
        command: ContractsSubcommands,
    },
}
