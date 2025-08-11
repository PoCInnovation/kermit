use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Value, json};

use crate::{
    account::signature::{GLSecp256k1PrivateKey, PrivateKey},
    utils::{HttpResponse, get, post},
};

/// CLI arguments for `kermit transactions`.
#[derive(Parser)]
pub enum TransactionsSubcommands {
    /// Build a new transaction.
    #[command(visible_alias = "b")]
    Build {
        /// Public key of the sender.
        #[arg(env)]
        public_key: String,
        /// Address of the recipient.
        to_addr: String,
        /// Amount to send.
        amount: String,
        /// Gas amount.
        #[arg(long)]
        gas_amount: Option<u64>,
        /// Gas price.
        #[arg(long)]
        gas_price: Option<String>,
    },
    /// Submit a transaction.
    #[command(visible_alias = "sub")]
    Submit {
        /// Transaction ID.
        tx_id: String,
        /// Unsigned transaction.
        unsigned_tx: String,
        /// Private key of the sender.
        #[arg(long, env)]
        private_key: String,
    },
    /// Create a transaction.
    #[command(visible_alias = "c")]
    Create {
        /// Public key of the sender.
        #[arg(env)]
        public_key: String,
        /// Address of the recipient.
        to_addr: String,
        /// Amount to send.
        amount: String,
        /// Gas amount.
        #[arg(long)]
        gas_amount: Option<u64>,
        /// Gas price.
        #[arg(long)]
        gas_price: Option<String>,
        /// Private key of the sender.
        #[arg(long, env)]
        private_key: String,
    },
    /// Decode an unsigned transaction.
    #[command(visible_alias = "d")]
    Decode { unsigned_tx: String },
    #[command(visible_alias = "s")]
    /// Get transaction status
    Status { tx_id: String },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildTransactionResponse {
    tx_id: String,
    unsigned_tx: String,
}

async fn build<T: DeserializeOwned>(
    url: &str,
    public_key: String,
    to_addr: String,
    amount: String,
    gas_amount: Option<u64>,
    gas_price: Option<String>,
) -> Result<HttpResponse<T>> {
    post(
        url,
        "/transactions/build",
        json!({
            "fromPublicKey": public_key,
            "destinations": vec![json!({
                "address": to_addr,
                "attoAlphAmount": amount,
            })],
            "gas_amount": gas_amount,
            "gas_price": gas_price
        }),
    )
    .await
}

async fn submit(url: &str, unsigned_tx: &str, signature: &str) -> Result<HttpResponse<Value>> {
    post(
        url,
        "/transactions/submit",
        json!({
            "unsignedTx": unsigned_tx,
            "signature": signature,
        }),
    )
    .await
}

impl TransactionsSubcommands {
    pub async fn run(self, url: &str) -> Result<()> {
        let value: Value = match self {
            Self::Build {
                public_key,
                to_addr,
                amount,
                gas_amount,
                gas_price,
            } => {
                build(url, public_key, to_addr, amount, gas_amount, gas_price)
                    .await?
                    .data
            },
            Self::Submit {
                tx_id,
                unsigned_tx,
                private_key,
            } => {
                let private_key = GLSecp256k1PrivateKey::new(&private_key)?;
                let signature = private_key.sign(&tx_id)?;
                submit(url, &unsigned_tx, &signature).await?.data
            },
            Self::Create {
                public_key,
                to_addr,
                amount,
                gas_amount,
                gas_price,
                private_key,
            } => {
                let private_key = GLSecp256k1PrivateKey::new(&private_key)?;
                let BuildTransactionResponse { tx_id, unsigned_tx } =
                    build(url, public_key, to_addr, amount, gas_amount, gas_price)
                        .await?
                        .data;

                let signature = private_key.sign(&tx_id)?;
                submit(url, &unsigned_tx, &signature).await?.data
            },
            Self::Decode { unsigned_tx } => {
                post(
                    url,
                    "/transactions/decode-unsigned-tx",
                    json!({"unsignedTx": unsigned_tx}),
                )
                .await?
                .data
            },
            Self::Status { tx_id } => {
                get(url, &format!("/transactions/status?txId={}", tx_id))
                    .await?
                    .data
            },
        };

        serde_json::to_writer_pretty(std::io::stdout(), &value)?;
        println!();

        Ok(())
    }
}
