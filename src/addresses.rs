use anyhow::Result;
use clap::Parser;

use crate::{
    common::{get, print_output},
    network::health::check_network,
};

/// CLI arguments for `kermit addresses`.
#[derive(Parser)]
pub(crate) enum AddressesSubcommands {
    /// Get the balance of an address.
    #[command(visible_alias = "b")]
    Balance {
        address: String,
        #[arg(short, long, default_value_t = false)]
        mem_pool: bool,
    },

    /// Get the UTXOs of an address.
    #[command(visible_alias = "u")]
    Utxos {
        address: String,
        #[arg(short, long, default_value_t = false)]
        error_if_exceed_max_utxos: bool,
    },

    /// Get the group of an address.
    #[command(visible_alias = "g")]
    Group { address: String },
}

impl AddressesSubcommands {
    pub(crate) async fn run(self, url: String) -> Result<()> {
        check_network(&url).await?;

        let endpoint = match self {
            Self::Balance { address, mem_pool } => {
                format!("/addresses/{address}/balance?mempool={mem_pool}")
            },
            Self::Utxos {
                address,
                error_if_exceed_max_utxos,
            } => {
                format!(
                    "/addresses/{address}/utxos?\
                    error_if_exceed_max_utxos={error_if_exceed_max_utxos}"
                )
            },
            Self::Group { address } => {
                format!("/addresses/{address}/group")
            },
        };

        let output = get(&url, &endpoint).await?;
        print_output(output)?;

        Ok(())
    }
}
