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

use anyhow::{Result, bail};
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

    let node_url = kermit.url.unwrap_or(network.node_url.to_owned());
    if !is_network_alive(&node_url).await? {
        bail!("Network is not reachable: {}", node_url);
    }

    match kermit.cmd {
        KermitSubcommand::Address { command } => command.run(&node_url).await?,
        KermitSubcommand::Contracts { command } => {
            command
                .run(&node_url, &config, network, kermit.network)
                .await?
        },
        KermitSubcommand::Events { command } => command.run(&node_url).await?,
        KermitSubcommand::Infos { command } => command.run(&node_url).await?,
        KermitSubcommand::Transactions { command } => command.run(&node_url).await?,
        KermitSubcommand::Wallets { command } => command.run(&node_url).await?,
    }

    Ok(())
}
