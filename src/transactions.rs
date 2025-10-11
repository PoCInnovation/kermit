use anyhow::{Result, anyhow, bail};
use clap::Parser;
use secp256k1::{Message, Secp256k1, SecretKey};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Value, json};

use crate::common::{get, post, print_output};

/// CLI arguments for `kermit transactions`.
#[derive(Parser)]
pub(crate) enum TransactionsSubcommands {
    /// Build a new transaction.
    #[command(visible_alias = "b")]
    Build {
        /// Public key of the sender.
        public_key: String,
        /// Address of the recipient.
        to_addr: String,
        /// Amount to send.
        amount: String,
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
        public_key: String,
        /// Address of the recipient.
        to_addr: String,
        /// Amount to send.
        amount: String,
        /// Private key of the sender.
        #[arg(long, env)]
        private_key: String,
    },

    /// Decode an unsigned transaction.
    #[command(visible_alias = "dec")]
    Decode { unsigned_tx: String },

    /// Get transaction details.
    #[command(visible_alias = "d")]
    Details {
        tx_id: String,
        #[arg(short, long)]
        from_group: Option<i64>,
        #[arg(short, long)]
        to_group: Option<i64>,
    },

    /// Get transaction with enriched input information when node indexes are enabled.
    #[command(visible_alias = "rd")]
    RichDetails {
        tx_id: String,
        #[arg(short, long)]
        from_group: Option<i64>,
        #[arg(short, long)]
        to_group: Option<i64>,
    },

    /// Get raw transaction in hex format.
    #[command(visible_alias = "r")]
    Raw {
        tx_id: String,
        #[arg(short, long)]
        from_group: Option<i64>,
        #[arg(short, long)]
        to_group: Option<i64>,
    },

    /// Get tx status.
    #[command(visible_alias = "s")]
    Status {
        tx_id: String,
        #[arg(short, long)]
        from_group: Option<i64>,
        #[arg(short, long)]
        to_group: Option<i64>,
    },

    /// Get transaction id from transaction output ref.
    #[command(visible_alias = "tifo")]
    TxIdFromOutputref { hint: i64, key: String },
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
) -> Result<Option<T>> {
    post(
        url,
        "/transactions/build",
        json!({
            "fromPublicKey": public_key,
            "destinations": vec![json!({
                "address": to_addr,
                "attoAlphAmount": amount,
            })]
        }),
    )
    .await
}

fn sign(tx_id: &str, private_key: &str) -> Result<String> {
    let secp = Secp256k1::new();
    let private_key_bytes = hex::decode(private_key).map_err(|_| anyhow!("Invalid private key"))?;
    let secret_key =
        SecretKey::from_slice(&private_key_bytes).map_err(|_| anyhow!("Invalid private key"))?;

    let tx_id_bytes = hex::decode(tx_id)?;
    let message = Message::from_digest(
        tx_id_bytes
            .try_into()
            .map_err(|_| anyhow!("Invalid transaction id"))?,
    );

    let signature = secp.sign_ecdsa(&message, &secret_key);
    let serialized = signature.serialize_compact();
    let signature = hex::encode(serialized);

    Ok(signature)
}

async fn submit(url: &str, unsigned_tx: &str, signature: &str) -> Result<Option<Value>> {
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

fn append_groups(endpoint: &mut String, from_group: Option<i64>, to_group: Option<i64>) {
    match (from_group, to_group) {
        (Some(fg), Some(tg)) => {
            endpoint.push_str(&format!("?fromGroup={fg}&toGroup={tg}"));
        },
        (Some(fg), None) => {
            endpoint.push_str(&format!("?fromGroup={fg}"));
        },
        (None, Some(tg)) => {
            endpoint.push_str(&format!("?toGroup={tg}"));
        },
        (None, None) => {},
    }
}

impl TransactionsSubcommands {
    pub(crate) async fn run(self, url: &str) -> Result<()> {
        let output = match self {
            Self::Build {
                public_key,
                to_addr,
                amount,
            } => build(url, public_key, to_addr, amount).await?,
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
                private_key,
            } => {
                let Some(BuildTransactionResponse { tx_id, unsigned_tx }) =
                    build(url, public_key, to_addr, amount).await?
                else {
                    bail!("Failed to build transaction");
                };

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
            Self::Details {
                tx_id,
                from_group,
                to_group,
            } => {
                let mut endpoint = format!("/transactions/details/{tx_id}");
                append_groups(&mut endpoint, from_group, to_group);

                get(url, &endpoint).await?
            },
            Self::RichDetails {
                tx_id,
                from_group,
                to_group,
            } => {
                let mut endpoint = format!("/transactions/rich-details/{tx_id}");
                append_groups(&mut endpoint, from_group, to_group);

                get(url, &endpoint).await?
            },
            Self::Raw {
                tx_id,
                from_group,
                to_group,
            } => {
                let mut endpoint = format!("/transactions/raw/{tx_id}");
                append_groups(&mut endpoint, from_group, to_group);

                get(url, &endpoint).await?
            },
            Self::Status {
                tx_id,
                from_group,
                to_group,
            } => {
                let mut endpoint = format!("/transactions/status?txId={tx_id}");
                if let Some(from_group) = from_group {
                    endpoint.push_str(&format!("&fromGroup={from_group}"));
                }
                if let Some(to_group) = to_group {
                    endpoint.push_str(&format!("&toGroup={to_group}"));
                }

                get(url, &endpoint).await?
            },
            Self::TxIdFromOutputref { hint, key } => {
                get(
                    url,
                    &format!("/transactions/tx-id-from-outputref?hint={hint}&key={key}"),
                )
                .await?
            },
        };

        print_output(output)?;

        Ok(())
    }
}
