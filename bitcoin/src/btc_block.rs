use crate::error::Error;
use ::hex::decode;
use bitcoin::{
    blockdata::{
        block::{Block as BtcBlock, Header as BtcBlockHeader, Version},
        transaction::Transaction as BtcTransaction,
    },
    consensus::encode::deserialize as btc_deserialize,
    hash_types::{BlockHash, TxMerkleNode},
    CompactTarget,
};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub(crate) struct BtcBlockJson {
    time: u32,
    nonce: u32,
    height: u64,
    bits: String,
    version: i32,
    tx: Vec<BtcTxJson>,
    merkleroot: String,
    previousblockhash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub(crate) struct BtcTxJson {
    hex: String,
}

impl FromStr for BtcBlockJson {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl BtcBlockJson {
    pub(crate) fn to_btc_block(&self) -> Result<BtcBlock, Error> {
        Ok(BtcBlock {
            /* // FIXME re-instate this!
            txdata: self
                .tx
                .iter()
                .map(|t| Ok(btc_deserialize::<BtcTransaction>(&decode(t.hex())?)?))
                .collect::<Result<Vec<_>, Error>>()?,
            */
            txdata: vec![], // FIXME use the above
            header: BtcBlockHeader {
                time: *self.time(),
                nonce: *self.nonce(),
                version: Version::from_consensus(*self.version()),
                merkle_root: TxMerkleNode::from_str(&self.merkleroot)?,
                prev_blockhash: BlockHash::from_str(&self.previousblockhash)?,
                bits: CompactTarget::from_consensus(u32::from_str_radix(self.bits(), 16)?),
            },
        })
    }
}
