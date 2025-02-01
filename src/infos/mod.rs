use anyhow::Result;
use clap::Parser;
use serde::de::DeserializeOwned;
use serde_json::{self, Value};
use std::fmt::Debug;

mod types;

/// CLI arguments for `kermit infos`.
#[derive(Parser)]
pub enum InfosSubcommands {
    /// Get info about that node.
    #[command(visible_alias = "n")]
    Node,

    /// Get version about that node.
    #[command(visible_alias = "v")]
    Version,

    /// Get key params about your blockchain.
    #[command(visible_alias = "cp")]
    ChainParams,

    /// Get info about your own clique.
    #[command(visible_alias = "sc")]
    SelfClique,

    /// Get infos about the inter cliques.
    #[command(visible_alias = "icp")]
    InterCliquePeerInfo,

    /// Get discovered neighbors.
    #[command(visible_alias = "dn")]
    DiscoveredNeighbors,

    /// Get the misbehaviors of peers.
    #[command(visible_alias = "m")]
    Misbehaviors,
}

async fn get_alephium_data(route: String) -> Result<(u16, String)> {
    let url = format!("https://node.mainnet.alephium.org/{route}");
    let response = reqwest::get(&url).await?;
    let status = response.status();
    let body = response.text().await?;

    Ok((status.as_u16(), body))
}

fn return_string_from_data<T: DeserializeOwned + std::fmt::Debug>(
    code: u16,
    str: String,
) -> Result<Option<T>> {
    match code {
        200 => {
            let parsed: T = serde_json::from_str(&str)?;
            Ok(Some(parsed))
        }
        _ => Ok(None),
    }
}

impl InfosSubcommands {
    pub async fn run(self) -> Result<()> {
        let (endpoint, result_type) = match self {
            Self::Node => ("/infos/node", "NodeInfo"),
            Self::Version => ("/infos/version", "VersionInfo"),
            Self::ChainParams => ("/infos/chain-params", "ChainParams"),
            Self::SelfClique => ("/infos/self-clique", "SelfClique"),
            Self::InterCliquePeerInfo => ("/infos/inter-clique-peer-info", "InterCliquePeerInfo"),
            Self::DiscoveredNeighbors => ("/infos/discovered-neighbors", "DiscoveredNeighbors"),
            Self::Misbehaviors => ("/infos/misbehaviors", "Misbehaviors"),
        };

        let data = get_alephium_data(endpoint.to_string()).await?;
        let result: Value = match result_type {
            "NodeInfo" => return_string_from_data::<types::NodeInfo>(data.0, data.1)?,
            "VersionInfo" => return_string_from_data::<types::NodeVersion>(data.0, data.1)?,
            "ChainParams" => return_string_from_data::<types::ChainParams>(data.0, data.1)?,
            "SelfClique" => return_string_from_data::<types::SelfClique>(data.0, data.1)?,
            "InterCliquePeerInfo" => {
                return_string_from_data::<Vec<types::CliqueInfo>>(data.0, data.1)?
            }
            "DiscoveredNeighbors" => {
                return_string_from_data::<Vec<types::DiscoveredNeighbors>>(data.0, data.1)?
            }
            "Misbehaviors" => return_string_from_data::<types::Misbehaviors>(data.0, data.1)?,
            _ => {
                eprintln!("Unknown command.");
                return Ok(());
            }
        };

        

        if let Some(result) = result {
            println!("Ok.\n{:#?}", result);
        }

        Ok(())
    }
}
