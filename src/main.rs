use anyhow::Result;
use args::{Kermit, KermitSubcommand};
use clap::Parser;

mod address;
mod args;
mod contracts;
mod wallet;

#[tokio::main]
async fn main() -> Result<()> {
    let kermit = Kermit::parse();

    match kermit.cmd {
        KermitSubcommand::Wallet { command } => command.run().await?,
        KermitSubcommand::Address { command } => command.run().await?,
        KermitSubcommand::Contracts { command } => command.run().await?,
    }

    Ok(())
}
