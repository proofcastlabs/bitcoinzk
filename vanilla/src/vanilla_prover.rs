use std::str::FromStr;

use bitcoin::BtcBlocks;
use lc::{prove_btc_blocks, Proof};

pub(crate) fn prove(btc_blocks: &str) -> Proof {
    let blocks = BtcBlocks::from_str(btc_blocks).expect("to unwrap btc blocks");

    prove_btc_blocks(blocks)
}
