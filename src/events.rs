use anyhow::Result;
use clap::Parser;
use serde_json::Value;

use crate::utils::get;

/// CLI arguments for `kermit events`.
#[derive(Parser)]
pub enum EventsSubcommands {
    /// Get events for a contract within a counter range.
    #[command(visible_alias = "ce")]
    ContractEvents {
        contract_address: String,
        start: i32,
        limit: Option<i32>,
        group: Option<i32>,
    },

    /// Get the current value of the events counter for a contract.
    #[command(visible_alias = "ccc")]
    ContractCurrentCount { contract_address: String },

    /// Get contract events for a transaction.
    #[command(visible_alias = "tce")]
    TxContractEvents { tx_id: String, group: Option<i32> },

    /// Get contract events for a block.
    #[command(visible_alias = "bce")]
    BlockContractEvents {
        block_hash: String,
        group: Option<i32>,
    },
}

impl EventsSubcommands {
    pub async fn run(self, url: &str) -> Result<()> {
        let value: Value = match self {
            Self::ContractEvents {
                contract_address,
                start,
                limit,
                group,
            } => {
                let mut endpoint = format!("/events/contract/{}?start={}", contract_address, start);
                if let Some(limit) = limit {
                    endpoint.push_str(&format!("&limit={}", limit));
                }
                if let Some(group) = group {
                    endpoint.push_str(&format!("&group={}", group));
                }
                get(url, &endpoint).await?
            },
            Self::ContractCurrentCount { contract_address } => {
                let endpoint = format!("/events/contract/{}/current-count", contract_address);
                get(url, &endpoint).await?
            },
            Self::TxContractEvents { tx_id, group } => {
                let mut endpoint = format!("/events/tx-id/{}", tx_id);
                if let Some(group) = group {
                    endpoint.push_str(&format!("&group={}", group));
                }
                get(url, &endpoint).await?
            },
            Self::BlockContractEvents { block_hash, group } => {
                let mut endpoint = format!("/events/block-hash/{}", block_hash);
                if let Some(group) = group {
                    endpoint.push_str(&format!("&group={}", group));
                }
                get(url, &endpoint).await?
            },
        };

        serde_json::to_writer_pretty(std::io::stdout(), &value)?;

        Ok(())
    }
}
