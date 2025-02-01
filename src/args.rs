use clap::{Parser, Subcommand, ValueHint};

use crate::wallet::WalletsSubcommands;
use crate::infos::InfosSubcommands;
use crate::events::EventsSubcommands;

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
    Wallets {
        #[command(subcommand)]
        command: WalletsSubcommands,
    },
    /// Infos about node and hashrate.
    #[command(visible_alias = "i")]
    Infos {
        #[command(subcommand)]
        command: InfosSubcommands,
    },
    /// Event for contract, block and hash.
    #[command(visible_alias = "e")]
    Events {
        #[command(subcommand)]
        command: EventsSubcommands,
    }
}
