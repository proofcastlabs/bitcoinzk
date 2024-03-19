mod btc_block;
mod btc_blocks;
mod btc_blocks_wrapper;
mod cli;
mod constants;
mod curl;
mod error;
mod get_block_hashes;
mod get_blocks;
mod json_response;
mod write_blocks;

#[macro_use]
extern crate log;

use self::{curl::curl, json_response::JsonResponse};

pub use self::{
    btc_blocks::BtcBlocks,
    btc_blocks_wrapper::BtcBlocksWrapper,
    cli::{Cli, Commands},
    constants::{DEFAULT_ELF_PATH, MAX_NUM_BLOCKS},
    error::BtcError,
    get_blocks::get_blocks,
    write_blocks::write_blocks_to_file,
};
