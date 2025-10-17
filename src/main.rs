mod account;
mod addresses;
mod args;
mod blockflow;
mod common;
mod config;
mod contracts;
mod contracts_funcs;
mod infos;
mod miners;
mod network;
mod transactions;
mod utils;
mod wallets;

use anyhow::Result;
use args::{Kermit, KermitSubcommand};
use clap::Parser;

use crate::config::config::Config;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let kermit = Kermit::parse();

    match kermit.cmd {
        KermitSubcommand::Addresses { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Blockflow { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Contracts {
            command,
            config_file_path,
            network,
        } => {
            let config = Config::new(&config_file_path)?;
            command.run(&kermit.url, &config, network).await?;
        },
        KermitSubcommand::Infos { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Miners { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Transactions { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Utils { command } => command.run().await?,
        KermitSubcommand::Wallets { command } => command.run(&kermit.url).await?,
    }

    Ok(())
}
