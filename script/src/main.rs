//! A simple script to generate and verify the proof of a given program.

use std::fs::read_to_string;
use sp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_tracer();

    let mut args = std::env::args();
    let path = args.nth(1).expect("please supply an argument");
    let s = read_to_string(&path).expect(&format!("could not read file at path: '{path}'"));

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
