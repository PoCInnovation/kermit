use clap::{Parser, Subcommand, ValueHint};

use crate::wallet::WalletSubcommands;

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
    /// Wallet management utilities.
    #[command(visible_alias = "w")]
    Wallet {
        #[command(subcommand)]
        command: WalletSubcommands,
    },
}
