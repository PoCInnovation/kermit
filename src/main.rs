mod args;
mod utils;
mod wallet;
mod infos;

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
        KermitSubcommand::Infos { command } => command.run().await?,
    }

    Ok(())
}
