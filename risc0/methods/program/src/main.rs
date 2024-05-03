#![no_main]
risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env::{read, commit};

use lc::prove_btc_blocks_from_string;

fn main() {
    let blocks = read::<String>();

    commit(&prove_btc_blocks_from_string(blocks))
}
