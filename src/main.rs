use anyhow::Result;
use args::{Kermit, KermitSubcommand};
use clap::Parser;

mod args;
mod wallet;
mod address;

#[tokio::main]
async fn main() -> Result<()> {
    let kermit = Kermit::parse();

    match kermit.cmd {
        KermitSubcommand::Wallet { command } => command.run().await?,
        KermitSubcommand::Address { command } => command.run().await?,
    }

    Ok(())
}
