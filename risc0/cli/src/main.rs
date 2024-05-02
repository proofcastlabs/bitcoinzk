use std::fs::read_to_string;

use bitcoin::{get_blocks, write_blocks_to_file, BtcError, MAX_NUM_BLOCKS};
use clap::Parser;
use risc0_zkvm::{default_prover, ExecutorEnv};

use cli::{Cli, Commands};
use methods::{BITCOINZ_RISC0_LC_PROGRAM_ELF as ELF, BITCOINZ_RISC0_LC_PROGRAM_ID as ID};

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

            let env = ExecutorEnv::builder().write(&s).unwrap().build().unwrap();

            // NOTE: Obtain the default prover.
            let prover = default_prover();

            // NOTE: Produce a receipt by proving the specified ELF binary.
            let receipt = prover.prove(env, ELF).unwrap();

            // NOTE: Read output.
            let r: bool = receipt.journal.decode().unwrap();
            println!("proof result r: {r}");

            // NOTE: Verify proof.
            receipt.verify(ID).expect("verification failed");

            println!("succesfully generated and verified proof for the program!");
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    handle_cli(cli).await.unwrap(); // FIXME
}
