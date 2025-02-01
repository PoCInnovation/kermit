use anyhow::Result;
use clap::Parser;
use serde::Deserialize;

use crate::utils::reqwest::get;

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

#[derive(Clone, Debug, Deserialize)]
struct Item {
    id: String,
    amount: String,
}

#[derive(Clone, Debug, Deserialize)]
struct BalanceResponse {
    balance: String,
    balanceHint: String,
    lockedBalance: String,
    lockedBalanceHint: String,
    transactions: Option<Vec<Item>>,
    unconfirmedTransactions: Option<Vec<Item>>,
    utxoNum: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct UTXORef {
    hint: u32,
    key: String, // 32-byte-hash
}

#[derive(Clone, Debug, Deserialize)]
struct UTXOResponse {
    #[serde(rename = "ref")]
    reference: UTXORef,
    amount: String,
    tokens: Option<Vec<Item>>,
    lockTime: u64,
    additionalData: String, // hex-string
}

#[derive(Clone, Debug, Deserialize)]
struct UTXOResponses {
    utxos: Vec<UTXOResponse>,
}

#[derive(Clone, Debug, Deserialize)]
struct GroupResponse {
    group: u32,
}

async fn get_balance(url: String, address: String, mem_pool: bool) -> Result<()> {
    let method = format!("/addresses/{address}/balance?mempool={mem_pool}");
    let response: BalanceResponse = get(&url, &method).await?;

    println!("Balance: {:#?}", response);

    Ok(())
}

async fn get_utxos(url: String, address: String) -> Result<()> {
    let method = format!("/addresses/{address}/utxos");
    let response: UTXOResponses = get(&url, &method).await?;

    println!("UTXOs: {:#?}", response);
    Ok(())
}

async fn get_group(url: String, address: String) -> Result<()> {
    let method = format!("/addresses/{address}/group");

    let group_response: GroupResponse = get(&url, &method).await?;

    println!("Group: {:#?}", group_response);
    Ok(())
}

impl AddressSubcommands {
    pub async fn run(self, url: String) -> Result<()> {
        match self {
            Self::Balance { address, mem_pool } => {
                get_balance(url, address, mem_pool).await?;
            },
            Self::UTXOS { address } => {
                get_utxos(url, address).await?;
            },
            Self::Group { address } => {
                get_group(url, address).await?;
            },
        }

        Ok(())
    }
}
