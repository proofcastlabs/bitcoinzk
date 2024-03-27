#![no_main]
sp1_zkvm::entrypoint!(main);

use bitcoin::BtcBlocks;
use sp1_zkvm::io::{read, write};
use std::str::FromStr;

fn main() {
    let blocks = BtcBlocks::from_str(&read::<String>()).expect("to unwrap btc blocks");
    let are_chained = !blocks.iter().enumerate().fold(false, |acc, (i, block)| {
        if i > 0 {
            let prev_hash = blocks[i - 1].block_hash();
            let expected_prev_hash = block.header.prev_blockhash;
            let is_chained = prev_hash == expected_prev_hash;
            acc || !is_chained
        } else {
            acc
        }
    });
    write(&are_chained)
}
