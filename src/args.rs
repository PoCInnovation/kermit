use clap::{Parser, Subcommand, ValueHint};

use crate::{address::AddressSubcommands, contracts::ContractsSubcommands, wallet::WalletSubcommands};

#[derive(Parser)]
#[command(version)]
pub struct Kermit {
    #[clap(long, env, value_hint = ValueHint::Url)]
    pub url: String,

    #[clap(subcommand)]
    pub cmd: KermitSubcommand,
}

#[derive(Subcommand)]
pub enum KermitSubcommand {
    /// Wallet management utilities.
    #[command(visible_alias = "w")]
    Wallet {
        #[command(subcommand)]
        command: WalletSubcommands,
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
    }
}
