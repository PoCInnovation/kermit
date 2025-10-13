use anyhow::Result;
use clap::Parser;

use crate::{
    common::{get, print_output},
    network::health::check_network,
};

/// CLI arguments for `kermit blockflow`.
#[derive(Parser)]
pub(crate) enum BlockflowSubcommands {
    /// List blocks on the given time interval.
    #[command(visible_alias = "bs")]
    Blocks { from_ts: i64, to_ts: Option<i64> },

    /// List blocks with events on the given time interval.
    #[command(visible_alias = "bswe")]
    BlocksWithEvents { from_ts: i64, to_ts: Option<i64> },

    /// Given a time interval, list blocks containing events and transactions
    /// with enriched input information when node indexes are enabled.
    #[command(visible_alias = "rbs")]
    RichBlocks { from_ts: i64, to_ts: Option<i64> },

    /// Get a block with hash?
    #[command(visible_alias = "b")]
    Block { block_hash: String },

    /// Get a mainchain block by ghost uncle hash.
    #[command(visible_alias = "mcbbgu")]
    MainChainBlockByGhostUncle { ghost_uncle_hash: String },

    /// Get a block and events with hash.
    #[command(visible_alias = "bwe")]
    BlockWithEvents { block_hash: String },

    /// Get a block containing events and transactions with enriched input
    /// information when node indexes are enabled.
    #[command(visible_alias = "rb")]
    RichBlock { block_hash: String },

    /// Check if the block is in main chain.
    #[command(visible_alias = "ibimc")]
    IsBlockInMainChain { block_hash: String },

    /// Get all block's hashes at given height for given groups.
    #[command(visible_alias = "hs")]
    Hashes {
        from_group: i64,
        to_group: i64,
        height: i64,
    },

    /// Get infos about the chain from the given groups.
    #[command(visible_alias = "ci")]
    ChainInfo { from_group: i64, to_group: i64 },

    /// Get infos about the chain from the given groups.
    #[command(visible_alias = "h")]
    Header { block_hash: String },

    /// Get raw block in hex format.
    #[command(visible_alias = "rawb")]
    RawBlock { block_hash: String },
}

impl BlockflowSubcommands {
    pub(crate) async fn run(self, url: &str) -> Result<()> {
        check_network(&url).await?;

        let endpoint = match self {
            Self::Blocks { from_ts, to_ts } => {
                let mut endpoint = format!("/blockflow/blocks?fromTs={from_ts}");
                if let Some(to_ts) = to_ts {
                    endpoint.push_str(&format!("&toTs={to_ts}"));
                }

                endpoint
            },
            Self::BlocksWithEvents { from_ts, to_ts } => {
                let mut endpoint = format!("/blockflow/blocks-with-events?fromTs={from_ts}");
                if let Some(to_ts) = to_ts {
                    endpoint.push_str(&format!("&toTs={to_ts}"));
                }

                endpoint
            },
            Self::RichBlocks { from_ts, to_ts } => {
                let mut endpoint = format!("/blockflow/rich-blocks?fromTs={from_ts}");
                if let Some(to_ts) = to_ts {
                    endpoint.push_str(&format!("&toTs={to_ts}"));
                }

                endpoint
            },
            Self::Block { block_hash } => {
                format!("/blockflow/blocks/{block_hash}")
            },
            Self::MainChainBlockByGhostUncle { ghost_uncle_hash } => {
                format!("/blockflow/main-chain-block-by-ghost-uncle/{ghost_uncle_hash}")
            },
            Self::BlockWithEvents { block_hash } => {
                format!("/blockflow/blocks-with-events/{block_hash}")
            },
            Self::RichBlock { block_hash } => {
                format!("/blockflow/rich-blocks/{block_hash}")
            },
            Self::IsBlockInMainChain { block_hash } => {
                format!("/blockflow/is-block-in-main-chain?blockHash={block_hash}")
            },
            Self::Hashes {
                from_group,
                to_group,
                height,
            } => {
                format!(
                    "/blockflow/hashes?fromGroup={from_group}&toGroup={to_group}&height={height}",
                )
            },
            Self::ChainInfo {
                from_group,
                to_group,
            } => {
                format!("/blockflow/chain-info?fromGroup={from_group}&toGroup={to_group}")
            },
            Self::Header { block_hash } => {
                format!("/blockflow/headers/{block_hash}")
            },
            Self::RawBlock { block_hash } => {
                format!("/blockflow/raw-blocks/{block_hash}")
            },
        };

        let output = get(url, &endpoint).await?;
        print_output(output)?;

        Ok(())
    }
}
