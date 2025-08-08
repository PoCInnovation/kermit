use anyhow::{Ok, Result};
use clap::Parser;
use serde_json::{Value, json};

use crate::utils::{get, post};

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
    #[command(visible_alias = "icpi")]
    InterCliquePeerInfo,

    /// Get discovered neighbors.
    #[command(visible_alias = "dn")]
    DiscoveredNeighbors,

    /// Get the misbehaviors of peers.
    #[command(visible_alias = "m")]
    Misbehaviors,

    /// Ban/Unban given peers.
    #[command(visible_alias = "mbu")]
    MisbehaviorsBanUnban { r#type: String, peers: Vec<String> },

    /// Get the unreachable brokers.
    #[command(visible_alias = "ub")]
    UnreachableBrokers,

    /// Set brokers to be unreachable/reachable.
    #[command(visible_alias = "d")]
    Discovery { r#type: String, peers: Vec<String> },

    /// Get history average hashrate on the given time interval.
    #[command(visible_alias = "hh")]
    HistoryHashrate { from_ts: i64, to_ts: Option<i64> },

    /// Get average hashrate from now - timespan(millis) to now.
    #[command(visible_alias = "ch")]
    CurrentHashrate { timespan: Option<i64> },

    /// Get the average difficulty of the latest blocks from all shards.
    #[command(visible_alias = "cd")]
    CurrentDifficulty,
}

impl InfosSubcommands {
    pub async fn run(self, url: &str) -> Result<()> {
        let value: Value = match self {
            Self::Node => get(url, "/infos/node").await?.data,
            Self::Version => get(url, "/infos/version").await?.data,
            Self::ChainParams => get(url, "/infos/chain-params").await?.data,
            Self::SelfClique => get(url, "/infos/self-clique").await?.data,
            Self::InterCliquePeerInfo => get(url, "/infos/inter-clique-peer-info").await?.data,
            Self::DiscoveredNeighbors => get(url, "/infos/discovered-neighbors").await?.data,
            Self::Misbehaviors => get(url, "/infos/misbehaviors").await?.data,
            Self::MisbehaviorsBanUnban { r#type, peers } => {
                post(
                    url,
                    "/infos/misbehaviors",
                    json!({
                        "type": r#type,
                        "peers": peers
                    }),
                )
                .await?.data
            },
            Self::UnreachableBrokers => get(url, "/infos/unreachable").await?.data,
            Self::Discovery { r#type, peers } => {
                post(
                    url,
                    "/infos/misbehaviors",
                    json!({
                        "type": r#type,
                        "peers": peers
                    }),
                )
                .await?.data
            },
            Self::HistoryHashrate { from_ts, to_ts } => {
                let mut endpoint = format!("/infos/history-hashrate?fromTs={}", from_ts);
                if let Some(to_ts) = to_ts {
                    endpoint.push_str(&format!("&toTs={}", to_ts));
                }
                get(url, &endpoint).await?.data
            },
            Self::CurrentHashrate { timespan } => {
                let mut endpoint = "/infos/current-hashrate".to_string();
                if let Some(timespan) = timespan {
                    endpoint.push_str(&format!("?timespan={}", timespan));
                }
                get(url, &endpoint).await?.data
            },
            Self::CurrentDifficulty => get(url, "/infos/current-difficulty").await?.data,
        };

        serde_json::to_writer_pretty(std::io::stdout(), &value)?;
        println!();

        Ok(())
    }
}
