#![cfg(test)]
use crate::btc_blocks::BtcBlocks;
use std::{fs::read_to_string, str::FromStr};

pub(crate) fn get_sample_btc_blocks_1() -> BtcBlocks {
    BtcBlocks::from_str(&read_to_string("src/test_utils/sample_blocks_1.json").unwrap()).unwrap()
}
