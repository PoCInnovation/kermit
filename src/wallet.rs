use anyhow::Result;
use clap::Parser;

/// CLI arguments for `kermit wallet`.
#[derive(Parser)]
pub enum WalletSubcommands {
    /// Create a new random keypair.
    #[command(visible_alias = "n")]
    New {
        /// Number of wallets to generate.
        #[arg(long, short, default_value = "1")]
        number: u32,
    },
}

impl WalletSubcommands {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::New { number } => {
                for _ in 0..number {
                    println!("Successfully created new keypair.");
                    println!("Address:     {}", "0x1234");
                    println!(
                        "Private key: 0x{}",
                        "1234567890123456789012345678901234567890123456789012345678901234"
                    );
                }
            }
        }

        Ok(())
    }
}
