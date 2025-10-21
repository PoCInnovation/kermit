use anyhow::Result;
use clap::Parser;
use serde_json::{Value, json};

use crate::common::{check_network, delete, get, post, print_output, put};

/// CLI arguments for `kermit wallets`.
#[derive(Parser)]
pub enum WalletsSubcommands {
    /// List available wallets.
    #[command(visible_alias = "l")]
    List,

    /// Restore a wallet from your mnemonic.
    #[command(visible_alias = "r")]
    Restore {
        wallet_name: String,
        password: String,
        mnemonic: String,
        #[arg(short, long, default_value_t = false)]
        is_miner: bool,
    },

    /// Create a new wallet.
    #[command(visible_alias = "c")]
    Create {
        wallet_name: String,
        password: String,
        #[arg(short, long, default_value_t = false)]
        is_miner: bool,
    },

    /// Get wallet's status.
    #[command(visible_alias = "st")]
    Status { wallet_name: String },

    /// Delete your wallet file (can be recovered with your mnemonic).
    #[command(visible_alias = "d")]
    Delete {
        wallet_name: String,
        password: String,
    },

    /// Lock your wallet.
    #[command(visible_alias = "lk")]
    Lock { wallet_name: String },

    /// Unlock your wallet.
    #[command(visible_alias = "u")]
    Unlock {
        wallet_name: String,
        password: String,
    },

    /// Get your total balance.
    #[command(visible_alias = "b")]
    Balances { wallet_name: String },

    /// Reveal your mnemonic. !!! Use it with caution !!!
    #[command(visible_alias = "re")]
    RevealMnemonic {
        wallet_name: String,
        password: String,
    },

    /// Transfer ALPH from the active address.
    #[command(visible_alias = "t")]
    Transfer {
        wallet_name: String,
        to_address: String,
        amount: String,
    },

    /// Transfer all unlocked ALPH from the active address to another address.
    #[command(visible_alias = "saa")]
    SweepActiveAddress {
        wallet_name: String,
        to_address: String,
    },

    /// Transfer unlocked ALPH from all addresses (including mining addresses)
    /// to another address.
    #[command(visible_alias = "saas")]
    SweepAllAddresses {
        wallet_name: String,
        to_address: String,
    },

    /// Sign the given data and return back the signature.
    #[command(visible_alias = "s")]
    Sign { wallet_name: String, data: String },

    /// List all your wallet's addresses.
    #[command(visible_alias = "a")]
    Addresses { wallet_name: String },

    /// Get address' info.
    #[command(visible_alias = "ai")]
    AddressInfo {
        wallet_name: String,
        address: String,
    },

    /// Derive your next address.
    #[command(visible_alias = "dna")]
    DeriveNextAddress {
        wallet_name: String,
        #[arg(short, long)]
        group: Option<i64>,
    },

    /// Choose the active address.
    #[command(visible_alias = "caa")]
    ChangeActiveAddress {
        wallet_name: String,
        address: String,
    },

    /// List all miner addresses per group.
    #[command(visible_alias = "ma")]
    MinerAddresses { wallet_name: String },

    /// Derive your next miner addresses for each group.
    #[command(visible_alias = "dnma")]
    DeriveNextMinerAddresses { wallet_name: String },
}

impl WalletsSubcommands {
    pub async fn run(self, url: &str) -> Result<()> {
        if !url.contains("localhost") && !url.contains("127.0.0.1") {
            eprintln!("Warning: Wallets commands only work on devnet network.");
        }

        check_network(url).await?;

        let output = match self {
            Self::List => get(url, "/wallets").await?,
            Self::Restore {
                wallet_name,
                password,
                mnemonic,
                is_miner,
            } => {
                put(
                    url,
                    "/wallets",
                    json!(
                        { "walletName": wallet_name,
                        "password": password,
                        "mnemonic": mnemonic,
                        "isMiner": is_miner
                    }),
                )
                .await?
            },
            Self::Create {
                wallet_name,
                password,
                is_miner,
            } => {
                post(
                    url,
                    "/wallets",
                    json!({
                        "password": password,
                        "walletName": wallet_name,
                        "isMiner": is_miner
                    }),
                )
                .await?
            },
            Self::Status { wallet_name } => get(url, &format!("/wallets/{wallet_name}")).await?,
            Self::Delete {
                wallet_name,
                password,
            } => delete(url, &format!("/wallets/{wallet_name}?password={password}")).await?,
            Self::Lock { wallet_name } => {
                post(url, &format!("/wallets/{wallet_name}/lock"), Value::Null).await?
            },
            Self::Unlock {
                wallet_name,
                password,
            } => {
                post(
                    url,
                    &format!("/wallets/{wallet_name}/unlock"),
                    json!({ "password": password }),
                )
                .await?
            },
            Self::Balances { wallet_name } => {
                get(url, &format!("/wallets/{wallet_name}/balances")).await?
            },
            Self::RevealMnemonic {
                wallet_name,
                password,
            } => {
                post(
                    url,
                    &format!("/wallets/{wallet_name}/reveal-mnemonic"),
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
                    &format!("/wallets/{wallet_name}/transfer"),
                    json!({
                        "destinations": [{
                            "address": to_address,
                            "attoAlphAmount": amount
                        }]
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
                    &format!("/wallets/{wallet_name}/sweep-active-address"),
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
                    &format!("/wallets/{wallet_name}/sweep-all-addresses"),
                    json!({ "toAddress": to_address }),
                )
                .await?
            },
            Self::Sign { wallet_name, data } => {
                post(
                    url,
                    &format!("/wallets/{wallet_name}/sign"),
                    json!({ "data": data }),
                )
                .await?
            },
            Self::Addresses { wallet_name } => {
                get(url, &format!("/wallets/{wallet_name}/addresses")).await?
            },
            Self::AddressInfo {
                wallet_name,
                address,
            } => get(url, &format!("/wallets/{wallet_name}/addresses/{address}")).await?,
            Self::DeriveNextAddress { wallet_name, group } => {
                let mut endpoint = format!("/wallets/{wallet_name}/derive-next-address");
                if let Some(group) = group {
                    endpoint.push_str(&format!("?group={group}"));
                }

                post(url, &endpoint, Value::Null).await?
            },
            Self::ChangeActiveAddress {
                wallet_name,
                address,
            } => {
                post(
                    url,
                    &format!("/wallets/{wallet_name}/change-active-address"),
                    json!({ "address": address }),
                )
                .await?
            },
            Self::MinerAddresses { wallet_name } => {
                get(url, &format!("/wallets/{wallet_name}/miner-addresses")).await?
            },
            Self::DeriveNextMinerAddresses { wallet_name } => {
                post(
                    url,
                    &format!("/wallets/{wallet_name}/derive-next-miner-addresses"),
                    Value::Null,
                )
                .await?
            },
        };

        print_output(output)?;

        Ok(())
    }
}
