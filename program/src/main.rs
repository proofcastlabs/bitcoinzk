//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use ::hex::decode;
use bitcoin::{
    blockdata::{
        block::{Block as BtcBlock, Header as BtcBlockHeader, Version},
        transaction::Transaction as BtcTransaction,
    },
    consensus::encode::deserialize as btc_deserialize,
    hash_types::{BlockHash, TxMerkleNode},
    hex::HexToArrayError,
    CompactTarget,
};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use sp1_zkvm::io::{read, write};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("hex array error: {0}")]
    HexArray(#[from] HexToArrayError),

    #[error("btc consensus encode error: {0}")]
    BtcConsensusEncode(#[from] bitcoin::consensus::encode::Error),

    #[error("from hex error: {0}")]
    FromHex(#[from] ::hex::FromHexError),
}

#[derive(Debug, Clone, Getters)]
struct BtcSubmissionMaterial {
    id: BlockHash,
    block: BtcBlock,
}

impl BtcSubmissionMaterial {
    fn block_hash(&self) -> BlockHash {
        self.block().block_hash()
    }

    fn check_merkle_root(&self) -> bool {
        self.block().check_merkle_root()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
struct BtcSubmissionMaterialJson {
    block: BtcBlockJson,
    transactions: Vec<String>,
}

impl FromStr for BtcSubmissionMaterialJson {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub struct BtcBlockJson {
    bits: u32,
    id: String,
    nonce: u32,
    version: i32,
    height: u64,
    timestamp: u32,
    merkle_root: String,
    previousblockhash: String,
}

impl FromStr for BtcSubmissionMaterial {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let j = BtcSubmissionMaterialJson::from_str(s)?;
        Ok(Self {
            id: BlockHash::from_str(j.block().id()).unwrap_or_else(|_| {
                BlockHash::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000000",
                )
                .unwrap()
            }),
            block: BtcBlock {
                txdata: j
                    .transactions
                    .iter()
                    .map(|t| Ok(btc_deserialize::<BtcTransaction>(&decode(t)?)?))
                    .collect::<Result<Vec<_>, Self::Err>>()?,
                header: BtcBlockHeader {
                    nonce: *j.block().nonce(),
                    time: *j.block().timestamp(),
                    version: Version::from_consensus(*j.block().version()),
                    bits: CompactTarget::from_consensus(*j.block().bits()),
                    merkle_root: TxMerkleNode::from_str(&j.block().merkle_root)?,
                    prev_blockhash: BlockHash::from_str(&j.block().previousblockhash)?,
                },
            },
        })
    }
}

pub fn main() {
    let s = read::<String>();
    let sub_mat = BtcSubmissionMaterial::from_str(&s).expect("could not parse submission material");
    let id = sub_mat.id();
    let hash = sub_mat.block_hash();
    println!("  hash in header: {id}");
    println!(" calculated hash: {hash}");
    let header_hash_result = &hash == id;
    let tx_merkle_root_result = sub_mat.check_merkle_root();
    write(&(header_hash_result && tx_merkle_root_result));
}
