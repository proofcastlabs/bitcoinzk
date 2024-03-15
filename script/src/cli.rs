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
        path: String,

        /// Optional path to elf
        #[arg(long)]
        elf_path: Option<String>,
    },

    GetSubmissionMaterial {
        /// BTC rpc endpoint
        rpc_endpoint: String,

        /// Start block number
        start: u64,

        /// End block number
        end: u64,
    },
}
