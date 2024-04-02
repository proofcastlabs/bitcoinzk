use crate::constants::MAX_NUM_BLOCKS;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BtcError {
    #[error("cannot get block with hash: {0}")]
    NoBlock(bitcoin::BlockHash),

    #[error("hex array error: {0}")]
    HexArray(#[from] bitcoin::hex::HexToArrayError),

    #[error("http json response error: {0}")]
    HttpJsonResponse(String),

    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("utf8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("timed out error: {0}")]
    TimedOut(String),

    #[error("btc consensus encode error: {0}")]
    BtcConsensusEncode(#[from] bitcoin::consensus::encode::Error),

    #[error("from hex error: {0}")]
    FromHex(#[from] ::hex::FromHexError),

    #[error("from slice error: {0}")]
    FromSlice(#[from] bitcoin::hashes::FromSliceError),

    #[error("parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("{0} is > than MAX_NUM_BLOCK of {MAX_NUM_BLOCKS}")]
    TooManyBlocks(u64),

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}
