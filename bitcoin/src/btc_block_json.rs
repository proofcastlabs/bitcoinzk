use crate::{btc_blocks::BtcBlocks, BtcError};
use bitcoin::{
    blockdata::{
        block::{Block as BtcBlock, Header as BtcBlockHeader, Version},
        transaction::Transaction as BtcTransaction,
    },
    consensus::encode::{deserialize as btc_deserialize, serialize as btc_serialize},
    hash_types::{BlockHash, TxMerkleNode},
    CompactTarget,
};
use derive_getters::Getters;
use derive_more::{Constructor, Deref};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, Constructor, Deref)]
pub(crate) struct BtcBlockJsons(Vec<BtcBlockJson>);

impl TryFrom<BtcBlocks> for BtcBlockJsons {
    type Error = BtcError;

    fn try_from(x: BtcBlocks) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl TryFrom<&BtcBlocks> for BtcBlockJsons {
    type Error = BtcError;

    fn try_from(x: &BtcBlocks) -> Result<Self, Self::Error> {
        Ok(Self::new(
            x.iter()
                .map(|b| BtcBlockJson::try_from(b.clone()))
                .collect::<Result<Vec<_>, Self::Error>>()?,
        ))
    }
}

impl From<Vec<BtcBlockJson>> for BtcBlockJsons {
    fn from(x: Vec<BtcBlockJson>) -> Self {
        Self::new(x)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub(crate) struct BtcBlockJson {
    time: u32,
    nonce: u32,
    bits: String,
    version: i32,
    tx: Vec<BtcTxJson>,
    merkleroot: String,
    height: Option<u64>,
    previousblockhash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Constructor)]
pub(crate) struct BtcTxJson {
    hex: String,
}

impl From<&BtcTransaction> for BtcTxJson {
    fn from(tx: &BtcTransaction) -> Self {
        Self::new(hex::encode(btc_serialize(tx)))
    }
}

impl FromStr for BtcBlockJson {
    type Err = BtcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl TryFrom<BtcBlock> for BtcBlockJson {
    type Error = BtcError;

    fn try_from(block: BtcBlock) -> Result<Self, Self::Error> {
        Ok(Self {
            height: None, // NOTE: Block number not used for anything to do with consensus
            time: block.header.time,
            nonce: block.header.nonce,
            version: block.header.version.to_consensus(),
            merkleroot: block.header.merkle_root.to_string(),
            bits: format!("{:x}", block.header.bits.to_consensus()),
            previousblockhash: block.header.prev_blockhash.to_string(),
            tx: block
                .txdata
                .iter()
                .map(BtcTxJson::from)
                .collect::<Vec<BtcTxJson>>(),
        })
    }
}

impl BtcBlockJson {
    pub(crate) fn to_btc_block(&self) -> Result<BtcBlock, BtcError> {
        Ok(BtcBlock {
            txdata: self
                .tx
                .iter()
                .map(|t| Ok(btc_deserialize::<BtcTransaction>(&hex::decode(t.hex())?)?))
                .collect::<Result<Vec<BtcTransaction>, BtcError>>()?,
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
