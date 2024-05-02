#![no_main]
risc0_zkvm::guest::entry!(main);

use std::str::FromStr;

use risc0_zkvm::guest::env::{read, commit};

use bitcoin::BtcBlocks;
use lc::prove_btc_blocks;

fn main() {
    let blocks = BtcBlocks::from_str(&read::<String>()).expect("to unwrap btc blocks");

    commit(&prove_btc_blocks(blocks))
}
