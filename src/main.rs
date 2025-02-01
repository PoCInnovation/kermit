mod address;
mod args;
mod contracts;
mod wallet;

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
        KermitSubcommand::Wallet { command } => command.run().await?,
        KermitSubcommand::Address { command } => command.run().await?,
        KermitSubcommand::Contracts { command } => command.run().await?,
    }

    Ok(())
}
