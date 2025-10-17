use anyhow::Result;
use clap::Parser;
use serde_json::{Value, json};

use crate::{
    common::{get, post, print_output, put},
    network::health::check_network,
};

/// CLI arguments for `kermit miners`.
#[derive(Parser)]
pub enum MinersSubcommands {
    /// Execute an action on CPU miner. !!! for test only !!!
    #[command(visible_alias = "cm")]
    CpuMining { action: String },

    /// Mine a block on CPU miner. !!! for test only !!!
    #[command(visible_alias = "mob")]
    MineOneBlock { from_group: i64, to_group: i64 },

    /// List miner's addresses.
    #[command(visible_alias = "a")]
    Addresses,

    /// Update miner's addresses, but better to use user.conf instead.
    #[command(visible_alias = "ua")]
    UpdateAddresses { new_addresses: Vec<String> },
}

impl MinersSubcommands {
    pub async fn run(self, url: &str) -> Result<()> {
        check_network(&url).await?;

        let output = match self {
            Self::CpuMining { action } => {
                if !url.contains("localhost") && !url.contains("127.0.0.1") {
                    eprintln!("Warning: CpuMining command only works on devnet network.");
                }

                post(
                    url,
                    &format!("/miners/cpu-mining?action={action}"),
                    Value::Null,
                )
                .await?
            },
            Self::MineOneBlock {
                from_group,
                to_group,
            } => {
                if !url.contains("localhost") && !url.contains("127.0.0.1") {
                    eprintln!("Warning: MineOneBlock command only works on devnet network.");
                }

                post(
                    url,
                    &format!(
                        "/miners/cpu-mining/mine-one-block?fromGroup={from_group}&\
                        toGroup={to_group}",
                    ),
                    Value::Null,
                )
                .await?
            },
            Self::Addresses => get(url, "/miners/addresses").await?,
            Self::UpdateAddresses { new_addresses } => {
                put(
                    url,
                    "/miners/addresses",
                    json!({
                        "addresses": new_addresses
                    }),
                )
                .await?
            },
        };

        print_output(output)?;

        Ok(())
    }
}
