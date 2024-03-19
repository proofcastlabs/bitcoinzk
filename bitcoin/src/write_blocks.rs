use crate::{btc_blocks::BtcBlocks, btc_blocks_wrapper::BtcBlocksWrapper, BtcError};
use std::fs::File;
use std::io::prelude::*;

const BLOCKS_PATH: &str = "blocks.json";

pub fn write_blocks_to_file(blocks: BtcBlocks) -> Result<(), BtcError> {
    let mut file = File::create(BLOCKS_PATH)?;
    let bytes: Vec<u8> = BtcBlocksWrapper::new(blocks.iter().cloned().collect()).try_into()?;
    file.write_all(&bytes)?;
    info!("blocks written to path: {BLOCKS_PATH}");
    Ok(())
}
