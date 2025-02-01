mod args;
mod events;
mod infos;
mod transactions;
mod utils;
mod wallet;
mod memepool;

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
        KermitSubcommand::Events { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Infos { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Transactions { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Wallets { command } => command.run(&kermit.url).await?,
    }

    Ok(())
}
