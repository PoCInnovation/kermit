use crate::utils::{delete, get, put};
use anyhow::Result;
use clap::Parser;
use serde_json::{json, Value};

/// CLI arguments for `kermit mempool`.
#[derive(Parser)]
pub enum MempoolSubcommands {
    /// List mempool transactions.
    #[command(visible_alias = "mt")]
    ListTransactions,

    /// Remove all transactions from the mempool.
    #[command(visible_alias = "rmt")]
    RemoveAllTransactions,

    /// Rebroadcast a mempool transaction to the network.
    #[command(visible_alias = "rbt")]
    RebroadcastTransaction {
        tx_id: String,
    },

    /// Validate all mempool transactions and remove invalid ones.
    #[command(visible_alias = "vmt")]
    ValidateTransactions,
}

impl MempoolSubcommands {
    pub async fn run(self, url: &str) -> Result<()> {
        let value: Value = match self {
            Self::ListTransactions => get(url, "/mempool/transactions").await?,
            Self::RemoveAllTransactions => {
                delete(url, "/mempool/transactions").await?;
                json!("All transactions removed from mempool")
            },
            Self::RebroadcastTransaction { tx_id } => {
                put::<(), Value>(url, &format!("/mempool/transactions/rebroadcast?txId={}", tx_id), Value::Null)
                    .await?;
                json!("Transaction rebroadcasted")
            },
            Self::ValidateTransactions => {
                put::<(), Value>(url, "/mempool/transactions/validate", Value::Null).await?;
                json!("Transactions validated and invalid ones removed")
            },
        };

        serde_json::to_writer_pretty(std::io::stdout(), &value)?;

        Ok(())
    }
}
