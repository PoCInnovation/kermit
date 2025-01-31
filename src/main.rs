use anyhow::Result;
use args::{Kermit, KermitSubcommand};
use clap::Parser;

mod args;
mod wallet;

#[tokio::main]
async fn main() -> Result<()> {
    let kermit = Kermit::parse();

    match kermit.cmd {
        KermitSubcommand::Wallet { command } => command.run().await?,
    }

    Ok(())
}
