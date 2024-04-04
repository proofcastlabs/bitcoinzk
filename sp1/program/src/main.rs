#![no_main]
sp1_zkvm::entrypoint!(main);

use std::str::FromStr;

use sp1_zkvm::io::{read, write};

use bitcoin::BtcBlocks;
use lc::prove_btc_blocks;

fn main() {
    let blocks = BtcBlocks::from_str(&read::<String>()).expect("to unwrap btc blocks");

    write(&prove_btc_blocks(blocks))
}
