#![no_main]
sp1_zkvm::entrypoint!(main);

use bitcoin::BtcBlocks;
use sp1_zkvm::io::{read, write};
use std::str::FromStr;

fn main() {
    let blocks = BtcBlocks::from_str(&read::<String>()).expect("to unwrap btc blocks");

    let result = !blocks.iter().enumerate().fold(false, |acc, (i, block)| {
        let merkle_root_is_valid = block.check_merkle_root();

        let is_chained = if i > 0 {
            let prev_hash = blocks[i - 1].block_hash();
            let expected_prev_hash = block.header.prev_blockhash;
            prev_hash == expected_prev_hash
        } else {
            // NOTE/TODO: Later iterations of this can take either a block hash or the proof of the past set
            // of blocks and check that this first block is chained to that. For now we just assume the first
            // is chained correctly to whatever came before.
            true
        };

        // NOTE: We're using `false` to start with here and `||` so we can flip the bool once and have it stay
        // that way if anything is amiss at any point.
        acc || !(merkle_root_is_valid && is_chained)
    });


    write(&result)
}
