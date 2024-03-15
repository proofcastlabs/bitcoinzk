mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use sp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};
use std::{
    fs::{read, read_to_string},
    path::Path,
};

const MAX_NUM_BLOCKS: u64 = 10;
const DEFAULT_ELF_PATH: &str = "../program/elf/riscv32im-succinct-zkvm-elf";

fn main() {
    let cli = Cli::parse();

    utils::setup_tracer();

    match cli.commands() {
        Commands::GetSubmissionMaterial {
            start,
            end,
            rpc_endpoint,
        } => {
            assert!(end > start, "end block is not later than start block"); // FIXME
            let diff = end - start;
            assert!(diff <= MAX_NUM_BLOCKS, "MAX_NUM_BLOCKs of exceeded"); // FIXME
            unimplemented!("todo this")
        }
        Commands::GenerateProof {
            hash,
            path,
            elf_path,
        } => {
            let s = read_to_string(path)
                .unwrap_or_else(|_| panic!("could not read file at path: {path}"));

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
            let r = proof.stdout.read::<bool>();
            println!("proof result r: {r}");

            // NOTE: Verify proof.
            SP1Verifier::verify(elf_bytes.as_slice(), &proof).expect("verification failed");

            // NOTE: Save proof.
            proof
                .save("proof-with-io.json")
                .expect("saving proof failed");

            println!("succesfully generated and verified proof for the program!")
        }
    }
}
