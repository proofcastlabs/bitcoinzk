use std::fs::read_to_string;

use bitcoin::{get_blocks, write_blocks_to_file, BtcError, MAX_NUM_BLOCKS};
use clap::Parser;

use cli::{Cli, Commands};

async fn handle_cli(cli: Cli) -> Result<(), BtcError> {
    match cli.commands() {
        Commands::GetBlocks {
            start,
            amount,
            output,
            rpc_endpoint,
        } => {
            if *amount > MAX_NUM_BLOCKS {
                return Err(BtcError::TooManyBlocks(*amount));
            };
            let blocks = get_blocks(rpc_endpoint, *start, *amount).await?;
            write_blocks_to_file(blocks, output.clone())?;

            Ok(())
        }

        Commands::GenerateProof { blocks_path, .. } => {
            let s = read_to_string(blocks_path)
                .unwrap_or_else(|_| panic!("could not read file at path: {blocks_path}"));

            // NOTE: Retrieve prover and verifier.
            let (prove_blocks, verify_blocks) = guest::build_check_blocks();

            // NOTE: Produce a receipt.
            let (_output, proof) = prove_blocks(&s);

            // NOTE: Verify proof.
            if verify_blocks(proof) {
                println!("successfully generated and verified proof for the blocks!");
            } else {
                println!("error on verifying proof for the blocks!");
            }

            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    handle_cli(cli).await.unwrap(); // FIXME
}
