use std::{
    fs::{read, read_to_string},
    path::Path,
};

use clap::Parser;
use sp1_core::{utils as sp1_utils, SP1Prover, SP1Stdin, SP1Verifier};

use bitcoin::{get_blocks, write_blocks_to_file, BtcError, DEFAULT_ELF_PATH, MAX_NUM_BLOCKS};
use cli::{Cli, Commands};
use lc::Proof as LCProof;

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
        Commands::GenerateProof {
            blocks_path,
            elf_path,
        } => {
            let s = read_to_string(blocks_path)
                .unwrap_or_else(|_| panic!("could not read file at path: {blocks_path}"));

            let elf_path = if let Some(path) = elf_path {
                path
            } else {
                DEFAULT_ELF_PATH
            };

            // NOTE: Check the elf exists
            if !Path::new(elf_path).exists() {
                panic!(
                    "elf does not exist at path: {elf_path}, see the readme for how to create it!"
                );
            };

            let elf_bytes = read(elf_path).expect("this to work because of above check");

            // NOTE:  Generate proof.
            let mut stdin = SP1Stdin::new();
            stdin.write(&s);

            let mut proof = SP1Prover::prove(elf_bytes.as_slice(), stdin).expect("proving failed");

            // NOTE: Read output.
            let r = proof.stdout.read::<LCProof>();
            println!("proof result: {r:?}");

            // NOTE: Verify proof.
            SP1Verifier::verify(elf_bytes.as_slice(), &proof).expect("verification failed");

            // NOTE: Save proof.
            proof
                .save("proof-with-io.json")
                .expect("saving proof failed");

            println!("succesfully generated and verified proof for the program!");
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    sp1_utils::setup_tracer();
    handle_cli(cli).await.unwrap(); // FIXME
}
