//! A simple script to generate and verify the proof of a given program.

use sp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};
use std::fs::read_to_string;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate proof
    GenerateProof {
        /// Hash of block prior to submission material
        hash: String,

        /// Path to btc submission material
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    utils::setup_tracer();

    match cli.commands {
        Commands::GenerateProof { hash, path } => {
            let s = read_to_string(&path)
                .unwrap_or_else(|_| panic!("could not read file at path: {path}"));

            // Generate proof.
            let mut stdin = SP1Stdin::new();
            stdin.write(&s);

            let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

            // Read output.
            let r = proof.stdout.read::<bool>();
            println!("proof result r: {r}");

            // Verify proof.
            SP1Verifier::verify(ELF, &proof).expect("verification failed");

            // Save proof.
            proof
                .save("proof-with-io.json")
                .expect("saving proof failed");

            println!("succesfully generated and verified proof for the program!")
        }
    }
}
