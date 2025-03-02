use anyhow::Result;
use clap::Parser;
use serde_json::Value;

use crate::utils::get;

#[derive(Parser)]
pub enum AddressSubcommands {
    #[command(visible_alias = "b")]
    Balance {
        address: String,
        #[arg(short, long, default_value_t = false)]
        mem_pool: bool,
    },
    #[command(visible_alias = "utxo")]
    UTXOS { address: String },
    #[command(visible_alias = "g")]
    Group { address: String },
}

impl AddressSubcommands {
    pub async fn run(self, url: String) -> Result<()> {
        let endpoint = match self {
            Self::Balance { address, mem_pool } => {
                format!("/addresses/{address}/balance?mempool={mem_pool}")
            },
            Self::UTXOS { address } => {
                format!("/addresses/{address}/utxos")
            },
            Self::Group { address } => {
                format!("/addresses/{address}/group")
            },
        };

        let value: Value = get(&url, &endpoint).await?;
        serde_json::to_writer_pretty(std::io::stdout(), &value)?;
        println!();

        Ok(())
    }
}
