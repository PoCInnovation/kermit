use anyhow::{anyhow, Result};
use clap::Parser;
use secp256k1::{Message, Secp256k1, SecretKey};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{json, Value};

use crate::utils::{get, post};

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
) -> Result<T> {
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

fn sign(tx_id: &str, private_key: &str) -> Result<String> {
    let secp = Secp256k1::new();
    let private_key_bytes = hex::decode(private_key)?;
    let secret_key = SecretKey::from_slice(&private_key_bytes)?;

    let tx_id_bytes = hex::decode(tx_id)?;
    let message = Message::from_digest(
        tx_id_bytes
            .try_into()
            .map_err(|_| anyhow!("Invalid hash length"))?,
    );

    let signature = secp.sign_ecdsa(&message, &secret_key);
    let serialized = signature.serialize_compact();
    let signature = hex::encode(serialized);

    Ok(signature)
}

async fn submit(url: &str, unsigned_tx: &str, signature: &str) -> Result<Value> {
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
            } => build(url, public_key, to_addr, amount, gas_amount, gas_price).await?,
            Self::Submit {
                tx_id,
                unsigned_tx,
                private_key,
            } => {
                let signature = sign(&tx_id, &private_key)?;
                submit(url, &unsigned_tx, &signature).await?
            },
            Self::Create {
                public_key,
                to_addr,
                amount,
                gas_amount,
                gas_price,
                private_key,
            } => {
                let BuildTransactionResponse { tx_id, unsigned_tx } =
                    build(url, public_key, to_addr, amount, gas_amount, gas_price).await?;

                let signature = sign(&tx_id, &private_key)?;
                submit(url, &unsigned_tx, &signature).await?
            },
            Self::Decode { unsigned_tx } => {
                post(
                    url,
                    "/transactions/decode-unsigned-tx",
                    json!({"unsignedTx": unsigned_tx}),
                )
                .await?
            },
            Self::Status { tx_id } => {
                get(url, &format!("/transactions/status?txId={}", tx_id)).await?
            },
        };

        serde_json::to_writer_pretty(std::io::stdout(), &value)?;

        Ok(())
    }
}
