use anyhow::{Result, anyhow};
use clap::Parser;
use serde_json::{Value, json};

use crate::utils::{delete, get, post, put};

/// CLI arguments for `kermit wallets`.
#[derive(Parser)]
pub enum WalletsSubcommands {
    /// List available wallets.
    #[command(visible_alias = "lw")]
    List,

    /// Restore a wallet from your mnemonic.
    #[command(visible_alias = "rw")]
    Restore { mnemonic: String },

    /// Create a new wallet.
    #[command(visible_alias = "cw")]
    Create {
        wallet_name: String,
        password: String,
    },

    /// Get wallet's status.
    #[command(visible_alias = "ws")]
    Status { wallet_name: String },

    /// Delete your wallet file (can be recovered with your mnemonic).
    #[command(visible_alias = "dw")]
    Delete {
        wallet_name: String,
        password: String,
    },

    /// Lock your wallet.
    #[command(visible_alias = "wl")]
    Lock { wallet_name: String },

    /// Unlock your wallet.
    #[command(visible_alias = "wu")]
    Unlock {
        wallet_name: String,
        password: String,
    },

    /// Get your total balance.
    #[command(visible_alias = "wb")]
    Balances { wallet_name: String },

    /// Reveal your mnemonic. !!! Use it with caution !!!
    #[command(visible_alias = "rm")]
    RevealMnemonic {
        wallet_name: String,
        password: String,
    },

    /// Transfer ALPH from the active address.
    #[command(visible_alias = "wt")]
    Transfer {
        wallet_name: String,
        to_address: String,
        amount: String,
    },

    /// Transfer all unlocked ALPH from the active address to another address.
    #[command(visible_alias = "wa")]
    SweepActiveAddress {
        wallet_name: String,
        to_address: String,
    },

    /// Transfer unlocked ALPH from all addresses (including mining addresses)
    /// to another address.
    #[command(visible_alias = "waa")]
    SweepAllAddresses {
        wallet_name: String,
        to_address: String,
    },

    /// Sign the given data and return back the signature.
    #[command(visible_alias = "wsign")]
    Sign { wallet_name: String, data: String },

    /// List all your wallet's addresses.
    #[command(visible_alias = "wal")]
    Addresses { wallet_name: String },

    /// Get address' info.
    #[command(visible_alias = "wai")]
    AddressInfo {
        wallet_name: String,
        address: String,
    },

    /// Derive your next address.
    #[command(visible_alias = "wna")]
    DeriveNextAddress {
        wallet_name: String,
        group: Option<i32>,
    },

    /// Choose the active address.
    #[command(visible_alias = "wca")]
    ChangeActiveAddress {
        wallet_name: String,
        address: String,
    },
}

impl WalletsSubcommands {
    pub async fn run(self, url: &str) -> Result<()> {
        if url == "https://node.mainnet.alephium.org" {
            eprintln!("You need to use a devnet adress (-u [ADDRESS]) to use Wallet commands");
            return Err(anyhow!(
                "Invalid URL: Mainnet node is not allowed for wallet operations."
            ));
        }
        let value: Value = match self {
            Self::List => get(url, "/wallets").await?,
            Self::Restore { mnemonic } => {
                put(url, "/wallets", json!({ "mnemonic": mnemonic })).await?
            },
            Self::Create {
                wallet_name,
                password,
            } => {
                post(
                    url,
                    "/wallets",
                    json!({
                        "password": password,
                        "walletName": wallet_name
                    }),
                )
                .await?
            },
            Self::Status { wallet_name } => get(url, &format!("/wallets/{}", wallet_name)).await?,
            Self::Delete {
                wallet_name,
                password,
            } => {
                delete(
                    url,
                    &format!("/wallets/{}?password={}", wallet_name, password),
                )
                .await?;
                json!("wallet remove")
            },
            Self::Lock { wallet_name } => {
                post::<(), Value>(url, &format!("/wallets/{}/lock", wallet_name), Value::Null)
                    .await?;
                json!("wallet lock")
            },
            Self::Unlock {
                wallet_name,
                password,
            } => {
                post(
                    url,
                    &format!("/wallets/{}/unlock", wallet_name),
                    json!({ "password": password }),
                )
                .await?
            },
            Self::Balances { wallet_name } => {
                get(url, &format!("/wallets/{}/balances", wallet_name)).await?
            },
            Self::RevealMnemonic {
                wallet_name,
                password,
            } => {
                post(
                    url,
                    &format!("/wallets/{}/reveal-mnemonic", wallet_name),
                    json!({ "password": password }),
                )
                .await?
            },
            Self::Transfer {
                wallet_name,
                to_address,
                amount,
            } => {
                post(
                    url,
                    &format!("/wallets/{}/transfer", wallet_name),
                    json!({
                        "destinations": [
                            {
                              "address": to_address,
                              "attoAlphAmount": amount
                            }
                          ]

                    }),
                )
                .await?
            },
            Self::SweepActiveAddress {
                wallet_name,
                to_address,
            } => {
                post(
                    url,
                    &format!("/wallets/{}/sweep-active-address", wallet_name),
                    json!({ "toAddress": to_address }),
                )
                .await?
            },
            Self::SweepAllAddresses {
                wallet_name,
                to_address,
            } => {
                post(
                    url,
                    &format!("/wallets/{}/sweep-all-addresses", wallet_name),
                    json!({ "toAddress": to_address }),
                )
                .await?
            },
            Self::Sign { wallet_name, data } => {
                post(
                    url,
                    &format!("/wallets/{}/sign", wallet_name),
                    json!({ "data": data }),
                )
                .await?
            },
            Self::Addresses { wallet_name } => {
                get(url, &format!("/wallets/{}/addresses", wallet_name)).await?
            },
            Self::AddressInfo {
                wallet_name,
                address,
            } => {
                get(
                    url,
                    &format!("/wallets/{}/addresses/{}", wallet_name, address),
                )
                .await?
            },
            Self::DeriveNextAddress { wallet_name, group } => {
                let mut endpoint = format!("/wallets/{}/derive-next-address", wallet_name);
                if let Some(group) = group {
                    endpoint.push_str(&format!("&toTs={}", group));
                }
                post(url, &endpoint, json!({})).await?
            },
            Self::ChangeActiveAddress {
                wallet_name,
                address,
            } => {
                post(
                    url,
                    &format!("/wallets/{}/change-active-address", wallet_name),
                    json!({ "address": address }),
                )
                .await?
            },
        };

        serde_json::to_writer_pretty(std::io::stdout(), &value)?;

        Ok(())
    }
}
