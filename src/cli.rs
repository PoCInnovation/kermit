use clap::{Parser, ValueHint};

use crate::wallet::WalletArgs;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[clap(long, env, value_hint = ValueHint::Url)]
    pub url: String,
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    #[clap(subcommand)]
    Wallet(WalletArgs),
}
