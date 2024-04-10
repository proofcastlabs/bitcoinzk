#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use std::str::FromStr;

use bitcoin::BtcBlocks;
use lc::prove_btc_blocks;

#[jolt::provable]
fn check_blocks(blocks: &str) -> bool {
    let blocks = BtcBlocks::from_str(blocks).expect("to unwrap btc blocks");

    prove_btc_blocks(blocks)
}
