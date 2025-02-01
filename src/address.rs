use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use serde::Deserialize;

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

async fn get_balance(address: String, mem_pool: bool) -> Result<()> {
    let client = Client::new();
    let url = format!(
        "https://node.mainnet.alephium.org/addresses/{}/balance?mempool={}",
        address, mem_pool
    );

    let response = client.get(&url).send().await?;
    let balance_response: BalanceResponse = response.json().await?;

    println!("Balance: {:?}", balance_response);

    Ok(())
}

async fn get_utxos(address: String) -> Result<()> {
    let client = Client::new();
    let url = format!(
        "https://node.mainnet.alephium.org/addresses/{}/utxos",
        address
    );

    let response = client.get(&url).send().await?;
    let utxo_responses: UTXOResponses = response.json().await?;

    println!("UTXOs: {:?}", utxo_responses);

    Ok(())
}

async fn get_group(address: String) -> Result<()> {
    let client = Client::new();
    let url = format!(
        "https://node.mainnet.alephium.org/addresses/{}/group",
        address
    );

    let response: reqwest::Response = client.get(&url).send().await?;
    let group_response: GroupResponse = response.json().await?;

    println!("Group: {:?}", group_response);

    Ok(())
}

impl AddressSubcommands {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Balance { address, mem_pool } => {
                get_balance(address, mem_pool).await?;
            }
            Self::UTXOS { address } => {
                get_utxos(address).await?;
            }
            Self::Group { address } => {
                get_group(address).await?;
            }
        }

        Ok(())
    }
}
