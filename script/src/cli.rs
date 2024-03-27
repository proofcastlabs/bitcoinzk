use clap::{Parser, Subcommand};
use derive_getters::Getters;

#[derive(Parser, Getters)]
#[command(version, about, long_about = None, rename_all="camelCase")]
pub struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
#[command(rename_all = "camelCase")]
pub enum Commands {
    /// Generate proof
    GenerateProof {
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

        /// Optional path to write blocks file to
        #[arg(long)]
        output: Option<String>,
    },
}
