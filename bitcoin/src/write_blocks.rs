use crate::{btc_blocks::BtcBlocks, BtcError};
use std::fs::File;
use std::io::prelude::*;

const DEFAULT_BLOCKS_PATH: &str = "blocks.json";

pub fn write_blocks_to_file(blocks: BtcBlocks, maybe_path: Option<String>) -> Result<(), BtcError> {
    let mut file = if let Some(s) = maybe_path {
        File::create(s)
    } else {
        File::create(DEFAULT_BLOCKS_PATH)
    }?;
    let bytes: Vec<u8> = blocks.try_into()?;
    file.write_all(&bytes)?;
    Ok(())
}
