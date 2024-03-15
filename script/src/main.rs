//! A simple script to generate and verify the proof of a given program.

use sp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};
use std::{
    fs::{read, read_to_string},
    path::Path,
};

const DEFAULT_ELF_PATH: &str = "../program/elf/riscv32im-succinct-zkvm-elf";

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, rename_all="camelCase")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
#[command(rename_all = "camelCase")]
enum Commands {
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
}

fn main() {
    let cli = Cli::parse();

    utils::setup_tracer();

    match cli.commands {
        Commands::GenerateProof {
            hash,
            path,
            elf_path,
        } => {
            let s = read_to_string(&path)
                .unwrap_or_else(|_| panic!("could not read file at path: {path}"));

            let elf_path = if let Some(path) = elf_path {
                path
            } else {
                DEFAULT_ELF_PATH.to_string()
            };

            // NOTE: Check the elf exists
            if !Path::new(&elf_path).exists() {
                panic!(
                    "elf does not exist at path: {elf_path}, see the readme for how to create it!"
                );
            };

            let elf_bytes = read(&elf_path).expect("this to work because of above check");

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
