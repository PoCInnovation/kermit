mod account;
mod address;
mod args;
mod config;
mod contracts;
mod contracts_funcs;
mod events;
mod infos;
mod network;
mod transactions;
mod utils;
mod wallet;

use anyhow::{Result, anyhow};
use args::{Kermit, KermitSubcommand};
use clap::Parser;

use crate::{config::config::Config, contracts::NetworkType, network::health::is_network_alive};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let kermit = Kermit::parse();

    // TODO: Generate a template Config if doesn't exist
    let config = Config::new(&kermit.config_file_path)?;
    let network = match &kermit.network {
        NetworkType::Dev => &config.configuration.networks.devnet,
        NetworkType::Test => &config.configuration.networks.testnet,
        NetworkType::Main => &config.configuration.networks.mainnet,
    };

    if !is_network_alive(&network.node_url).await? {
        return Err(anyhow!("Network is not reachable: {}", network.node_url));
    }

    match kermit.cmd {
        KermitSubcommand::Address { command } => command.run(kermit.url).await?,
        KermitSubcommand::Contracts { command } => {
            command
                .run(&kermit.url, &config, network, kermit.network)
                .await?
        },
        KermitSubcommand::Events { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Infos { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Transactions { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Wallets { command } => command.run(&kermit.url).await?,
    }

    Ok(())
}
