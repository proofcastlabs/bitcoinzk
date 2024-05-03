use std::str::FromStr;

use serde::{Deserialize, Serialize};

use bitcoin::{BlockHash, BtcBlocks};

#[derive(Debug, Serialize, Deserialize)]
pub enum Proof {
    False,
    True(BlockHash, BlockHash),
}

pub fn prove_btc_blocks(blocks: BtcBlocks) -> Proof {
    match blocks.first() {
        Some(first_block) => {
            let mut current_block = first_block.clone();
            if !blocks.iter().enumerate().fold(false, |acc, (i, block)| {
                current_block = block.clone();

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
            }) {
                Proof::True(
                    first_block.header.block_hash(),
                    current_block.header.block_hash(),
                )
            } else {
                Proof::False
            }
        }
        None => Proof::False,
    }
}

pub fn prove_btc_blocks_from_string(blocks: String) -> Proof {
    prove_btc_blocks(BtcBlocks::from_str(&blocks).expect("to unwrap btc blocks"))
}
