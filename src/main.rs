mod account;
mod addresses;
mod args;
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
            let mut app = Kermit::command();
            let bin_name = app.get_name().to_string();
            let shell = shell.map_or_else(
                || {
                    let detected_shell = std::env::var("SHELL")
                        .ok()
                        .and_then(|p| {
                            std::path::Path::new(&p)
                                .file_name()
                                .and_then(|os| os.to_str())
                                .map(str::to_lowercase)
                        })
                        .or_else(|| {
                            // If SHELL is not set (like on Windows), try to detect the shell by other env vars
                            if std::env::var_os("PSModulePath").is_some() {
                                Some("pwsh".to_string())
                            } else {
                                None
                            }
                        });

                    match detected_shell.as_deref() {
                        Some("bash") => clap_complete::Shell::Bash,
                        Some("zsh") => clap_complete::Shell::Zsh,
                        Some("fish") => clap_complete::Shell::Fish,
                        Some("elvish") => clap_complete::Shell::Elvish,
                        Some("pwsh") | Some("powershell") => clap_complete::Shell::PowerShell,
                        _ => clap_complete::Shell::Bash,
                    }
                },
                |s| s,
            );

            clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
        },
    }

    Ok(())
}
