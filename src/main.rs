use anyhow::Result;
use args::{Kermit, KermitSubcommand};
use clap::Parser;

mod args;
mod wallet;
mod infos;

#[tokio::main]
async fn main() -> Result<()> {
    let kermit = Kermit::parse();

    match kermit.cmd {
        KermitSubcommand::Wallet { command } => command.run().await?,
        KermitSubcommand::Infos { command } => command.run().await?,
    }

    Ok(())
}
