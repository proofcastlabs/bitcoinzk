use clap::{Parser, Subcommand};
use derive_getters::Getters;

#[derive(Parser, Getters)]
#[command(version, about, long_about = None, rename_all="camelCase")]
pub(crate) struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
#[command(rename_all = "camelCase")]
pub(crate) enum Commands {
    /// Generate proof
    GenerateProof {
        /// Hash of block prior to submission material
        hash: String,

        /// Path to btc submission material
        blocks_path: String,

        /// Optional path to elf
        #[arg(long)]
        elf_path: Option<String>,
    },

    /// Get BTC blocks for ZKP light-client proof generation
    GetBlocks {
        /// BTC rpc endpoint
        rpc_endpoint: String,

        /// Start block number
        start: u64,

        /// Amount of blocks to get
        amount: u64,
    },
}
