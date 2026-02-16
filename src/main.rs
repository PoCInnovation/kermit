mod account;
mod addresses;
mod args;
mod autocomplete;
mod blockflow;
mod common;
mod config;
mod contracts;
mod contracts_funcs;
mod infos;
mod miners;
mod transactions;
mod utils;
mod wallets;
mod docs;

use anyhow::Result;
use args::{Kermit, KermitSubcommand};
use clap::{CommandFactory, Parser};

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
        KermitSubcommand::Addresses { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Blockflow { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Contracts {
            command,
            config_file_path,
            auto_create_config_file,
            network,
        } => {
            command
                .run(
                    &kermit.url,
                    &config_file_path,
                    network,
                    auto_create_config_file,
                )
                .await?;
        },
        KermitSubcommand::Infos { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Miners { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Transactions { command } => command.run(&kermit.url).await?,
        KermitSubcommand::Utils { command } => command.run().await?,
        KermitSubcommand::Wallets { command } => command.run(&kermit.url).await?,
        KermitSubcommand::GenerateAutocompletion { shell } => {
            autocomplete::generate_autocomplete(&mut Kermit::command(), shell);
        },
        KermitSubcommand::GenerateDocs { command } => {
            command.run().await?;
        },
    }

    Ok(())
}
