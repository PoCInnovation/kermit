mod address;
mod args;
mod contracts;
mod events;
mod infos;
mod transactions;
mod utils;
mod wallet;

mod contract_encoding;

use anyhow::Result;
use args::{Kermit, KermitSubcommand};
use clap::Parser;

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
        KermitSubcommand::Address { command } => command.run(kermit.url).await?,
        KermitSubcommand::Contracts { command } => command.run(kermit.url).await?,
        KermitSubcommand::Events { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Infos { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Transactions { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Wallets { command } => command.run(&kermit.url).await?,
    }

    Ok(())
}
