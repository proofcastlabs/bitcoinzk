use crate::{btc_blocks::BtcBlocks, btc_blocks_wrapper::BtcBlocksWrapper, BtcError};
use std::fs::File;
use std::io::prelude::*;

const DEFAULT_BLOCKS_PATH: &str = "blocks.json";

pub fn write_blocks_to_file(blocks: BtcBlocks, maybe_path: Option<String>) -> Result<(), BtcError> {
    let mut file = if let Some(s) = maybe_path {
        debug!("writing output to {s}");
        File::create(s)
    } else {
        debug!("writing output to {DEFAULT_BLOCKS_PATH}");
        File::create(DEFAULT_BLOCKS_PATH)
    }?;
    let bytes: Vec<u8> = BtcBlocksWrapper::new(blocks.iter().cloned().collect()).try_into()?;
    file.write_all(&bytes)?;
    debug!("blocks written successfully");
    Ok(())
}
